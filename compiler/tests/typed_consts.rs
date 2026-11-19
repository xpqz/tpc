//! Typed constants (decision 14 for reachability): `const X: T = value;` of scalars, strings,
//! arrays, records, sets and pointers are initialised static data, mutable at run time, and
//! static even when declared inside a routine. Every `.out` and `.err` (`N LINE`) comes from the
//! real TP 5.0 (`tools/tp5.sh`). TP quirks the tests pin: a record initialiser lists a prefix of
//! the fields in declaration order (101 otherwise), a character array takes a string of exactly
//! its length (100), a list of the wrong length is 87 at the closing parenthesis or 89 at the
//! first extra element, a procedural type is 99. One deviation, not tested: for a list with too
//! many elements TP reports the line of the comma before the first extra element, this compiler
//! the line of the element.
mod common;
mod driver;
use driver::*;
use std::path::{Path, PathBuf};

fn fx(sub: &str) -> PathBuf {
    root()
        .join("compiler/tests/fixtures/typed_consts")
        .join(sub)
}

fn assert_clean(failures: Vec<String>) {
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn stems(dir: &Path, ext: &str) -> Vec<String> {
    let mut v: Vec<String> = std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.unwrap().file_name().into_string().ok())
        .filter_map(|n| n.strip_suffix(ext).map(String::from))
        .collect();
    v.sort();
    v
}

#[test]
fn own_lang_programs_print_what_tp5_prints() {
    assert!(stems(&fx("lang"), ".pas").len() >= 10);
    assert_eq!(stems(&fx("lang"), ".pas"), stems(&fx("lang"), ".out"));
    for s in stems(&fx("lang"), ".out") {
        let out = std::fs::read(fx("lang").join(format!("{s}.out"))).unwrap();
        assert!(!out.is_empty(), "{s}: empty expected output");
    }
    assert_clean(check_lang(&fx("lang"), &real_compile));
}

#[test]
fn own_invalid_programs_report_tp5_number_and_line() {
    assert!(stems(&fx("neg"), ".pas").len() >= 19);
    assert_eq!(stems(&fx("neg"), ".pas"), stems(&fx("neg"), ".err"));
    assert_clean(check_neg(&fx("neg"), &real_compile));
}

/// Corpus programs (tests/lang, tests/neg) that this slice must make pass. Typed constants in
/// units belong to the units slice.
const LANG: &[&str] = &[
    "typedconst_array_indexed_by_enum",
    "typedconst_array_of_records",
    "typedconst_array_of_strings_lookup",
    "typedconst_array_one_dimension",
    "typedconst_array_two_dimensions",
    "typedconst_char_arrays_from_string_and_list",
    "typedconst_constant_expressions",
    "typedconst_in_procedure_keeps_value_between_calls",
    "typedconst_in_recursive_procedure_is_shared",
    "typedconst_passed_as_var_parameter",
    "typedconst_pointer_nil",
    "typedconst_record",
    "typedconst_record_nested_and_with_array",
    "typedconst_scalars_are_mutable",
    "typedconst_scalars_initial_values",
    "typedconst_sets",
    "typedconst_string_longer_than_declared_is_cut",
    "typedconst_string_shorter_than_declared_size",
    "typedconst_strings",
];

const NEG: &[&str] = &[
    "typedconst_initializer_uses_variable",
    "typedconst_not_allowed_as_array_bound",
    "typedconst_too_few_elements",
    "typedconst_too_many_elements",
    "typedconst_unknown_record_field",
    "typedconst_value_of_wrong_type",
];

/// Copies `names` with the given extensions into a fresh directory (no PENDING file).
fn subset(kind: &str, names: &[&str], exts: &[&str]) -> PathBuf {
    let dir = scratch("x").parent().unwrap().to_path_buf();
    for n in names {
        for e in exts {
            let f = format!("{n}.{e}");
            std::fs::copy(root().join("tests").join(kind).join(&f), dir.join(&f)).unwrap();
        }
    }
    dir
}

#[test]
fn corpus_lang_programs_match_tp5_output() {
    assert_clean(check_lang(
        &subset("lang", LANG, &["pas", "out"]),
        &real_compile,
    ));
}

#[test]
fn corpus_neg_programs_report_tp5_number_and_line() {
    assert_clean(check_neg(
        &subset("neg", NEG, &["pas", "err"]),
        &real_compile,
    ));
}

#[test]
fn corpus_lists_are_sorted_name_existing_programs_and_leave_pending() {
    for (kind, names, ext) in [("lang", LANG, "out"), ("neg", NEG, "err")] {
        let mut sorted = names.to_vec();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted, names, "{kind} list must be sorted and unique");
        let pending =
            std::fs::read_to_string(root().join(format!("tests/{kind}/PENDING"))).unwrap();
        for n in names {
            for e in ["pas", ext] {
                let p = root().join(format!("tests/{kind}/{n}.{e}"));
                assert!(p.exists(), "{}", p.display());
            }
            assert!(
                !pending.lines().any(|l| l.trim() == *n),
                "{kind}/{n} is still in PENDING"
            );
        }
    }
    assert_eq!((LANG.len(), NEG.len()), (19, 6));
}

fn compile(src: &str) -> Vec<u8> {
    tp5c::compile(src.as_bytes())
        .unwrap_or_else(|d| panic!("rejected with {} at {}:\n{src}", d.code, d.line))
}

fn occurrences(module: &[u8], pattern: &[u8]) -> usize {
    module
        .windows(pattern.len())
        .filter(|w| *w == pattern)
        .count()
}

fn program(decls: &str, body: &str) -> String {
    format!("program T;\n{decls}\nbegin\n  {body}\nend.")
}

/// The initial memory image of each kind of constant is in the module exactly once, laid out
/// as a variable of that type is (little-endian, no padding, strings with their length byte
/// padded to the capacity, sets as 32 bytes with bit i for element i, Reals as f64).
#[test]
fn initial_values_are_laid_out_like_variables() {
    let real = 1234.5678f64.to_le_bytes();
    let mut set = vec![0u8; 32];
    set[0] = 0b0000_1010;
    set[25] = 1;
    let cases: Vec<(&str, &str, Vec<u8>)> = vec![
        (
            "a: array[1..3] of Integer = (-2, $1234, -30000);",
            "a[1] := 0",
            vec![0xFE, 0xFF, 0x34, 0x12, 0xD0, 0x8A],
        ),
        (
            "a: array[1..2] of LongInt = (-2, $12345678);",
            "a[1] := 0",
            vec![0xFE, 0xFF, 0xFF, 0xFF, 0x78, 0x56, 0x34, 0x12],
        ),
        (
            "a: array[1..5] of ShortInt = (-3, 100, -128, 57, -99);",
            "a[1] := 0",
            vec![0xFD, 100, 0x80, 57, 0x9D],
        ),
        (
            "a: array[1..6] of Boolean = (True, False, True, True, False, False);",
            "a[1] := False",
            vec![1, 0, 1, 1, 0, 0],
        ),
        (
            "s: string[5] = 'ab';",
            "s := ''",
            vec![2, b'a', b'b', 0, 0, 0],
        ),
        ("s: string[3] = 'toolong';", "s := ''", vec![3, b't', b'o', b'o']),
        (
            "a: array[1..3] of Char = 'QRS';",
            "a[1] := 'a'",
            b"QRS".to_vec(),
        ),
        ("r: Real = 1234.5678;", "r := 0", real.to_vec()),
        ("r: Real = 2;", "r := 0", 2.0f64.to_le_bytes().to_vec()),
        (
            "t: set of Byte = [1, 3, 200];",
            "t := []",
            set,
        ),
        (
            "type R = record a: Integer; b: Char; c: string[2]; end;\nconst r: R = (a: 258; b: 'Z'; c: 'q');",
            "r.a := 0",
            vec![2, 1, b'Z', 1, b'q', 0],
        ),
    ];
    for (decl, body, image) in cases {
        let decl = if decl.starts_with("type") {
            decl.to_string()
        } else {
            format!("const {decl}")
        };
        let module = compile(&program(&decl, body));
        assert_eq!(occurrences(&module, &image), 1, "{decl}: {image:?}");
    }
}

/// A typed constant is one data segment, emitted only when a live body names it.
#[test]
fn unreferenced_typed_constants_are_not_in_the_module() {
    let decls = "const\n  dead: array[1..2] of LongInt = ($11223344, $55667788);\n  live: array[1..2] of LongInt = ($0A0B0C0D, $01020304);\nprocedure Unused;\nbegin\n  dead[1] := 0\nend;\n";
    let module = compile(&program(decls, "live[1] := live[2]"));
    let dead = [0x44, 0x33, 0x22, 0x11, 0x88, 0x77, 0x66, 0x55];
    let live = [0x0D, 0x0C, 0x0B, 0x0A, 0x04, 0x03, 0x02, 0x01];
    assert_eq!(occurrences(&module, &dead), 0, "constant of dead code kept");
    assert_eq!(occurrences(&module, &live), 1);
}

#[test]
fn a_constant_used_from_several_routines_is_one_segment() {
    let decls = "const\n  shared: array[1..2] of LongInt = ($11223344, $55667788);\nprocedure A;\nbegin\n  shared[1] := 1\nend;\nprocedure B;\nbegin\n  shared[2] := shared[1]\nend;\n";
    let module = compile(&program(decls, "A; B; shared[1] := shared[2]"));
    let image = [0x44, 0x33, 0x22, 0x11, 0x88, 0x77, 0x66, 0x55];
    assert_eq!(occurrences(&module, &image), 1);
}

#[test]
fn a_constant_used_only_by_a_reached_routine_is_kept() {
    let decls = "procedure Used;\nconst\n  k: array[1..2] of LongInt = ($11223344, $55667788);\nbegin\n  k[1] := k[2]\nend;\n";
    let module = compile(&program(decls, "Used"));
    let image = [0x44, 0x33, 0x22, 0x11, 0x88, 0x77, 0x66, 0x55];
    assert_eq!(occurrences(&module, &image), 1);
    assert!(common::validates(&module));
}

/// A typed constant shares nothing with a variable of the same name or with a constant of
/// the same name in another routine.
#[test]
fn constants_in_different_routines_do_not_overlap() {
    let decls = "procedure A;\nconst n: array[1..2] of Integer = (1101, 1102);\nbegin\n  n[1] := n[2]\nend;\nprocedure B;\nconst n: array[1..2] of Integer = (2201, 2202);\nbegin\n  n[1] := n[2]\nend;\nvar g: array[1..2] of Integer;\n";
    let module = compile(&program(decls, "A; B; g[1] := 0"));
    assert_eq!(occurrences(&module, &[0x4D, 4, 0x4E, 4]), 1);
    assert_eq!(occurrences(&module, &[0x99, 8, 0x9A, 8]), 1);
}

#[test]
fn typed_constant_rules_in_the_checker() {
    let ok = [
        "const a: Integer = 5;",
        "const a: Integer = (5);",
        "const a: string[3] = 'x';",
        "const a: Char = 'x';",
        "const a: array[1..2] of Char = 'xy';",
        "const a: array[1..2] of Char = ('x', 'y');",
        "const a: array[1..2, 1..2] of Byte = ((1, 2), (3, 4));",
        "const a: set of Byte = [];",
        "type P = ^Integer; const a: P = nil;",
        "type R = record x, y: Integer; end; const a: R = (x: 1);",
        "type R = record x, y: Integer; end; const a: R = (x: 1; y: 2);",
        "const n = 3; a: array[1..n] of Integer = (1, 2, 3);",
        "const n = 3; a: Integer = n * 2 + 1; b: Boolean = n > 2;",
        "const a: Integer = 1; b: Integer = 2;",
        "const a: Integer = 1;\nvar b: Integer;\nconst c: Integer = 3;",
    ];
    for decls in ok {
        compile(&program(decls, ""));
    }
    let bad: [(&str, (u16, u32)); 10] = [
        ("const a: Integer = 40000;", (76, 2)),
        ("const a: Char = 'xy';", (26, 2)),
        ("const a: Boolean = 1;", (26, 2)),
        ("const a: Integer = 1.5;", (26, 2)),
        ("var v: Integer;\nconst a: Integer = v + 1;", (133, 3)),
        ("const a: array[1..2] of Integer = (1, 2, 3);", (89, 2)),
        ("const a: array[1..2] of Integer = (1);", (87, 2)),
        (
            "type R = record x: Integer; end; const a: R = (y: 1);",
            (44, 2),
        ),
        (
            "type R = record x: Integer; end; const a: R = (x: 1; x: 2);",
            (101, 2),
        ),
        (
            "type T = procedure; procedure P; begin end; const a: T = P;",
            (99, 2),
        ),
    ];
    for (decls, want) in bad {
        let src = program(decls, "");
        match tp5c::compile(src.as_bytes()) {
            Ok(_) => panic!("accepted:\n{src}"),
            Err(d) => assert_eq!((d.code, d.line), want, "{src}"),
        }
    }
}
