//! Host imports declared as `external` routines in an embedded runtime source (decision 29):
//! the routine becomes an import from module `host`, named like the routine in lower case,
//! emitted only when a live body calls it, numbered before every defined function in the
//! order the compiler first reaches it. Programs for a synthetic System (the `compile_with`
//! hook) have no TP 5.0 counterpart; the corpus programs that pin the migrated imports are in
//! tests/lang and tests/rt, and `external` in a user program is tests/neg/unsupported_external_*.
mod common;
mod sysutil;
use common::{assert_hello_fits, node};
use sysutil::{compile_with, contains, system_pas};

const HELLO: &str = "program Hello;\nbegin\n  WriteLn('hello')\nend.";

/// The size of hello-world with `Write` going through `TextOut` (was 429 before text files);
/// it may not grow.
const HELLO_BYTES: usize = 676;

/// The real System with `extra` declarations appended.
fn system_with(extra: &str) -> Vec<u8> {
    let mut system = system_pas();
    system.extend_from_slice(b"\n");
    system.extend_from_slice(extra.as_bytes());
    system
}

fn compiled(system: &[u8], program: &str) -> Vec<u8> {
    compile_with(system, program).unwrap_or_else(|d| panic!("{d:?}\n{program}"))
}

/// The imports of a module as `module.name` strings, in function-number order.
fn imports(wasm: &[u8]) -> Vec<String> {
    let js = "for (const i of WebAssembly.Module.imports(new WebAssembly.Module(b))) \
              console.log(i.kind + ' ' + i.module + '.' + i.name);";
    node(js, wasm)
        .lines()
        .map(|line| line.strip_prefix("function ").expect(line).to_string())
        .collect()
}

/// Runs `main` with every import recording its call as `name(arguments)`; `twice` returns twice
/// its argument, every other import returns 0. Returns the calls, space separated.
fn calls(wasm: &[u8]) -> String {
    let js = "const m = new WebAssembly.Module(b); const log = []; const host = {}; \
        for (const i of WebAssembly.Module.imports(m)) { \
          host[i.name] = (...a) => { log.push(i.name + '(' + a.join(',') + ')'); \
                                     return i.name === 'twice' ? a[0] * 2 : 0; }; } \
        new WebAssembly.Instance(m, { host }).exports.main(); \
        console.log(log.join(' '));";
    node(js, wasm).trim().to_string()
}

#[test]
fn an_external_routine_is_imported_only_when_called() {
    let system = system_with("procedure Beep(n: LongInt); external;");
    let unused = compiled(&system, HELLO);
    let names = imports(&unused);
    // The positive half: `write` is imported, so an empty list cannot pass.
    assert!(names.contains(&"host.write".to_string()), "{names:?}");
    assert!(!names.contains(&"host.beep".to_string()), "{names:?}");
    assert_eq!(
        unused,
        compiled(&system_pas(), HELLO),
        "an unused external changed hello"
    );

    let called = "program P;\nbegin\n  Beep(3)\nend.";
    let names = imports(&compiled(&system, called));
    assert!(names.contains(&"host.write".to_string()), "{names:?}");
    assert_eq!(
        names.iter().filter(|n| *n == "host.beep").count(),
        1,
        "{names:?}"
    );
}

#[test]
fn an_external_called_twice_is_imported_once() {
    let system = system_with("procedure Beep(n: LongInt); external;");
    let program = "program P;\nbegin\n  Beep(1);\n  Beep(2)\nend.";
    let wasm = compiled(&system, program);
    assert_eq!(
        imports(&wasm).iter().filter(|n| *n == "host.beep").count(),
        1
    );
    assert_eq!(calls(&wasm), "beep(1) beep(2)");
}

#[test]
fn an_external_only_another_unused_routine_calls_is_not_imported() {
    let system = system_with(
        "procedure Beep(n: LongInt); external;\n\
         procedure BeepTwice(n: LongInt);\nbegin\n  Beep(n);\n  Beep(n)\nend;",
    );
    let names = imports(&compiled(&system, HELLO));
    assert!(names.contains(&"host.write".to_string()), "{names:?}");
    assert!(!names.contains(&"host.beep".to_string()), "{names:?}");
    // Called through a System routine, it is reached.
    let program = "program P;\nbegin\n  BeepTwice(5)\nend.";
    assert_eq!(calls(&compiled(&system, program)), "beep(5) beep(5)");
}

#[test]
fn import_numbers_follow_first_use_not_declaration_order() {
    let alpha_first = system_with(
        "procedure Alpha(n: LongInt); external;\nprocedure Beta(n: LongInt); external;",
    );
    let beta_first = system_with(
        "procedure Beta(n: LongInt); external;\nprocedure Alpha(n: LongInt); external;",
    );
    let beta_then_alpha = "program P;\nbegin\n  Beta(1);\n  Alpha(2);\n  Beta(3)\nend.";
    let alpha_then_beta = "program P;\nbegin\n  Alpha(1);\n  Beta(2);\n  Alpha(3)\nend.";
    for (program, order, log) in [
        (
            beta_then_alpha,
            ["host.beta", "host.alpha"],
            "beta(1) alpha(2) beta(3)",
        ),
        (
            alpha_then_beta,
            ["host.alpha", "host.beta"],
            "alpha(1) beta(2) alpha(3)",
        ),
    ] {
        let a = compiled(&alpha_first, program);
        let b = compiled(&beta_first, program);
        assert_eq!(a, b, "declaration order changed the module");
        let ours: Vec<String> = imports(&a)
            .into_iter()
            .filter(|n| n.ends_with(".alpha") || n.ends_with(".beta"))
            .collect();
        assert_eq!(ours, order);
        // Each call reaches the host function of its own name: the numbering is consistent.
        assert_eq!(calls(&a), log);
    }
}

#[test]
fn imports_come_before_every_defined_function() {
    // A defined routine that calls an external, reached before a second external: if imports
    // were numbered after defined functions the engine would reject the module or call the
    // wrong function.
    let system = system_with(
        "procedure Alpha(n: LongInt); external;\nprocedure Beta(n: LongInt); external;",
    );
    let program = "program P;\nprocedure Mine(n: LongInt);\nbegin\n  Alpha(n)\nend;\n\
                   begin\n  Mine(4);\n  Beta(5);\n  Mine(6)\nend.";
    assert_eq!(
        calls(&compiled(&system, program)),
        "alpha(4) beta(5) alpha(6)"
    );
}

#[test]
fn scalar_arguments_arrive_by_value_in_declared_order() {
    let system = system_with(
        "procedure Probe(i: Integer; x: Real; l: LongInt; b: Boolean; c: Char); external;\n\
         function Twice(n: LongInt): LongInt; external;",
    );
    let program =
        "program P;\nvar r: Real;\nbegin\n  r := 2.5;\n  Probe(-3, r, 70000, True, 'A');\n\
        Probe(7, r + 1, 5, False, 'z');\n  Probe(1, 3, 2, True, #0);\n\
        Probe(Twice(21), 0.125, 0, False, 'B')\nend.";
    assert_eq!(
        calls(&compiled(&system, program)),
        "probe(-3,2.5,70000,1,65) probe(7,3.5,5,0,122) probe(1,3,2,1,0) \
         twice(21) probe(42,0.125,0,0,66)"
    );
}

#[test]
fn an_external_function_result_is_used_like_any_function_result() {
    let system = system_with("function Twice(n: LongInt): LongInt; external;");
    let program = "program P;\nvar a: LongInt;\nbegin\n  a := Twice(4) + Twice(Twice(1));\n  \
                   if Twice(a) = 24 then a := Twice(0)\nend.";
    assert_eq!(
        calls(&compiled(&system, program)),
        "twice(4) twice(1) twice(2) twice(12) twice(0)"
    );
}

#[test]
fn the_migrated_imports_are_reached_by_the_routines_that_need_them() {
    // `fmt_real` only when a Real is formatted, `rterror` and `write` in every module.
    let plain = imports(&compiled(&system_pas(), HELLO));
    assert!(plain.contains(&"host.write".to_string()), "{plain:?}");
    assert!(plain.contains(&"host.rterror".to_string()), "{plain:?}");
    assert!(!plain.contains(&"host.fmt_real".to_string()), "{plain:?}");
    let real = "program P;\nvar x: Real;\nbegin\n  x := 1.5;\n  WriteLn(x:0:2)\nend.";
    let names = imports(&compiled(&system_pas(), real));
    assert!(names.contains(&"host.fmt_real".to_string()), "{names:?}");
    assert_eq!(
        names.iter().filter(|n| n.as_str() == "host.write").count(),
        1
    );
}

#[test]
fn hello_world_does_not_grow() {
    let wasm = compiled(&system_pas(), HELLO);
    assert_hello_fits(&wasm);
    assert!(
        wasm.len() <= HELLO_BYTES,
        "hello-world is {} bytes, was {HELLO_BYTES}",
        wasm.len()
    );
    assert!(contains(&wasm, "hello"));
}

/// An `external` declaration the compiler cannot pass to the host, with System's line 0.
#[test]
fn unsupported_external_signatures_are_error_200_in_system() {
    let bad = [
        ("var parameter", "procedure Bad(var n: LongInt); external;"),
        ("String parameter", "procedure Bad(s: String); external;"),
        (
            "array parameter",
            "type Buf = array[0..3] of Byte;\nprocedure Bad(b: Buf); external;",
        ),
        (
            "String result",
            "function Bad(n: LongInt): String; external;",
        ),
        ("Real result", "function Bad(n: LongInt): Real; external;"),
        (
            "nested in a System routine",
            "procedure Outer;\n  procedure Inner; external;\nbegin\nend;",
        ),
    ];
    for (what, declaration) in bad {
        let d = compile_with(&system_with(declaration), HELLO).expect_err(what);
        assert_eq!((d.code, d.line), (200, 0), "{what}");
    }
}

#[test]
fn a_well_formed_external_is_not_an_error_in_system() {
    let system = system_with("procedure Fine(a: LongInt; x: Real; b: Boolean); external;");
    assert!(compile_with(&system, HELLO).is_ok());
}
