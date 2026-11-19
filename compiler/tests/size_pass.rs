//! What a compiled program carries (docs/size.md): no custom sections, no WASM type nobody
//! uses, and the built-in helpers (`write_integer`, `quotient`) only in programs that need them.
//! The corpus programs `builtins_*` and the `rt` programs `*_without_write_integer` check that
//! the helpers still work alone; their expected output comes from the real TP 5.0.
mod sysutil;
use sysutil::*;

const HELLO: &str = "begin\n  WriteLn('Hello, world')\nend.\n";

/// One section of a module: its id and its content bytes.
fn sections(wasm: &[u8]) -> Vec<(u8, &[u8])> {
    let mut found = Vec::new();
    let mut at = 8;
    while at < wasm.len() {
        let id = wasm[at];
        let (size, used) = leb(&wasm[at + 1..]);
        let start = at + 1 + used;
        found.push((id, &wasm[start..start + size]));
        at = start + size;
    }
    found
}

fn leb(bytes: &[u8]) -> (usize, usize) {
    let mut value = 0;
    for (i, &b) in bytes.iter().enumerate() {
        value |= ((b & 0x7F) as usize) << (7 * i);
        if b & 0x80 == 0 {
            return (value, i + 1);
        }
    }
    panic!("unterminated LEB128");
}

fn section_content(wasm: &[u8], id: u8) -> Option<&[u8]> {
    sections(wasm).into_iter().find(|s| s.0 == id).map(|s| s.1)
}

/// The number of entries of a section, 0 when the module has no such section.
fn entry_count(wasm: &[u8], id: u8) -> usize {
    section_content(wasm, id).map_or(0, |content| leb(content).0)
}

const TYPE: u8 = 1;
const IMPORT: u8 = 2;
const FUNCTION: u8 = 3;

/// The (parameter count, result count) of every function type; all values are i32 or f64.
fn type_shapes(wasm: &[u8]) -> Vec<(usize, usize)> {
    let content = section_content(wasm, TYPE).expect("a type section");
    let (count, mut at) = leb(content);
    let mut shapes = Vec::new();
    for _ in 0..count {
        assert_eq!(content[at], 0x60, "a function type");
        let (params, used) = leb(&content[at + 1..]);
        at += 1 + used + params;
        let (results, used) = leb(&content[at..]);
        at += used + results;
        shapes.push((params, results));
    }
    shapes
}

/// The names of the imports, in order.
fn import_names(wasm: &[u8]) -> Vec<String> {
    let content = section_content(wasm, IMPORT).expect("an import section");
    let (count, mut at) = leb(content);
    let mut names = Vec::new();
    for _ in 0..count {
        for part in 0..2 {
            let (len, used) = leb(&content[at..]);
            at += used;
            if part == 1 {
                names.push(String::from_utf8(content[at..at + len].to_vec()).unwrap());
            }
            at += len;
        }
        assert_eq!(content[at], 0, "an imported function");
        let (_, used) = leb(&content[at + 1..]);
        at += 1 + used;
    }
    names
}

fn function_count(wasm: &[u8]) -> usize {
    entry_count(wasm, FUNCTION)
}

#[test]
fn hello_world_has_no_custom_sections() {
    let wasm = compile(HELLO);
    let sections = sections(&wasm);
    assert!(sections.len() > 5, "the module was not parsed");
    assert!(
        sections.iter().all(|s| s.0 != 0),
        "a custom section is present"
    );
}

#[test]
fn an_unused_procedural_type_has_no_wasm_type() {
    let with_type = format!("type Five = function(a, b, c, d, e: LongInt): LongInt;\n{HELLO}");
    let shapes = type_shapes(&compile(&with_type));
    assert!(!shapes.contains(&(5, 1)), "{shapes:?}");
}

#[test]
fn a_used_procedural_type_has_its_wasm_type_and_runs() {
    let src = "type Five = function(a, b, c, d, e: LongInt): LongInt;\n\
        function Sum(a, b, c, d, e: LongInt): LongInt;\nbegin\n  Sum := a + b + c + d + e\nend;\n\
        var f: Five;\nbegin\n  f := Sum;\n  WriteLn(f(1, 2, 3, 4, 5))\nend.\n";
    let wasm = compile(src);
    assert!(type_shapes(&wasm).contains(&(5, 1)));
    assert_eq!(run(&wasm), "15\n");
}

#[test]
fn the_runtime_type_of_the_console_line_reader_is_absent_when_no_line_is_read() {
    // `ConsoleLineReader` of runtime/system.pas is a function type of two parameters.
    let shapes = type_shapes(&compile(HELLO));
    assert!(!shapes.contains(&(2, 1)), "{shapes:?}");
}

#[test]
fn hello_world_defines_only_what_it_calls() {
    // write_string, TextOut and what they reach, the main program and the trap stub.
    let functions = function_count(&compile(HELLO));
    assert!(functions <= 8, "hello-world defines {functions} functions");
}

#[test]
fn hello_world_does_not_import_rterror() {
    let names = import_names(&compile(HELLO));
    assert!(!names.is_empty());
    assert!(!names.contains(&"rterror".to_string()), "{names:?}");
}

#[test]
fn integer_output_adds_the_integer_writer() {
    let with_integer = "var n: LongInt;\nbegin\n  n := 5;\n  WriteLn('Hello, world', n)\nend.\n";
    let base = function_count(&compile(HELLO));
    assert!(function_count(&compile(with_integer)) > base);
}

#[test]
fn division_adds_the_quotient_helper_and_the_rterror_import() {
    let dividing =
        "var n: LongInt;\nbegin\n  n := 5;\n  WriteLn('Hello, world', n div 2 = 2)\nend.\n";
    let wasm = compile(dividing);
    assert!(function_count(&wasm) > function_count(&compile(HELLO)));
    assert!(import_names(&wasm).contains(&"rterror".to_string()));
}

#[test]
fn real_division_adds_its_helper_and_the_rterror_import() {
    let dividing =
        "var x: Real;\nbegin\n  x := 5;\n  WriteLn('Hello, world', x / 2.0 > 2.0)\nend.\n";
    let wasm = compile(dividing);
    assert!(function_count(&wasm) > function_count(&compile(HELLO)));
    assert!(import_names(&wasm).contains(&"rterror".to_string()));
}

#[test]
fn hello_world_stays_under_the_size_after_the_size_pass() {
    // Before the pass it was 875 bytes.
    let module = compile(HELLO);
    assert!(module.len() <= 760, "hello-world is {} bytes", module.len());
}

#[test]
fn imported_functions_are_called_by_a_module_that_still_validates() {
    for src in [
        HELLO,
        "var n: LongInt;\nbegin\n  n := 3;\n  WriteLn(n, n div 2, n mod 2)\nend.\n",
    ] {
        assert!(run_validate(&compile(src)), "{src}");
    }
}
