//! The IdeHost unit (runtime/idehost.pas, decision 42): an embedded unit whose two routines
//! call the host imports `ide_compile` and `ide_run`, only when reachable. The behaviour behind
//! the imports is tested with the real compiler.wasm in tests/ide/host_test.mjs.
//! The corpus programs `ide_host_*` (tests/lang, tests/neg) have their expectations from the
//! real TP 5.0, which compiles them with the stand-in tests/ide/standin/IDEHOST.PAS.
mod common;
use common::node;
use std::path::PathBuf;
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn imports(wasm: &[u8]) -> Vec<String> {
    let js = "for (const i of WebAssembly.Module.imports(new WebAssembly.Module(b))) \
              console.log(i.module + '.' + i.name);";
    node(js, wasm).lines().map(String::from).collect()
}

fn count_of(names: &[String], import: &str) -> usize {
    names.iter().filter(|n| *n == import).count()
}

const BOTH: &str = "program P;\nuses IdeHost;\nvar R: CompileResult;\nbegin\n  \
    Compile('P.PAS', nil, 0, R);\n  WriteLn(Run);\nend.\n";

#[test]
fn compile_and_run_import_their_host_functions_once() {
    let names = imports(&tp5c::compile(BOTH.as_bytes()).unwrap());
    assert_eq!(count_of(&names, "host.ide_compile"), 1, "{names:?}");
    assert_eq!(count_of(&names, "host.ide_run"), 1, "{names:?}");
}

#[test]
fn only_the_routines_a_program_calls_are_imported() {
    let run_only = "program P;\nuses IdeHost;\nbegin\n  WriteLn(Run);\nend.\n";
    let names = imports(&tp5c::compile(run_only.as_bytes()).unwrap());
    assert_eq!(count_of(&names, "host.ide_run"), 1, "{names:?}");
    assert_eq!(count_of(&names, "host.ide_compile"), 0, "{names:?}");

    let neither = "program P;\nuses IdeHost;\nvar R: CompileResult;\nbegin\n  R.Code := 1;\nend.\n";
    let names = imports(&tp5c::compile(neither.as_bytes()).unwrap());
    assert_eq!(count_of(&names, "host.ide_run"), 0, "{names:?}");
    assert_eq!(count_of(&names, "host.ide_compile"), 0, "{names:?}");
}

#[test]
fn a_program_without_the_unit_imports_nothing_of_it() {
    let names = imports(&tp5c::compile(b"program P;\nbegin\n  WriteLn('hi');\nend.\n").unwrap());
    assert!(names.iter().all(|n| !n.contains("ide_")), "{names:?}");
}

#[test]
fn a_registered_file_called_idehost_does_not_replace_the_unit() {
    // The oracle stand-in has the same name; the embedded unit is found first, as for Crt.
    let stand_in = std::fs::read(root().join("tests/ide/standin/IDEHOST.PAS")).unwrap();
    let files = [tp5c::SourceFile {
        name: "IDEHOST.PAS".into(),
        text: stand_in,
    }];
    let wasm = tp5c::compile_files(BOTH.as_bytes(), &files).unwrap();
    let names = imports(&wasm);
    assert_eq!(count_of(&names, "host.ide_compile"), 1, "{names:?}");
}

/// The IDE host against the real compiler.wasm: imports called directly, and a toy IDE program
/// driven by keys (tests/ide/host_test.mjs).
#[test]
fn ide_host_with_the_real_compiler_wasm() {
    let target = Command::new("sh")
        .arg(root().join("scripts/cargo-target.sh"))
        .output()
        .unwrap();
    let target = String::from_utf8(target.stdout).unwrap();
    let target = target.trim();
    let build = Command::new("cargo")
        .args([
            "build",
            "--release",
            "--lib",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .current_dir(root())
        .env("CARGO_TARGET_DIR", target)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "{}",
        String::from_utf8_lossy(&build.stderr)
    );
    let wasm = PathBuf::from(target).join("wasm32-unknown-unknown/release/tp5c.wasm");
    let o = Command::new("node")
        .arg(root().join("tests/ide/host_test.mjs"))
        .arg(wasm)
        .current_dir(root())
        .output()
        .expect("node must be installed");
    assert!(
        o.status.success(),
        "{}{}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn the_runner_refuses_an_ide_program_without_a_compiler_and_a_missing_compiler_file() {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ide_host");
    std::fs::create_dir_all(&dir).unwrap();
    let wasm = dir.join("both.wasm");
    std::fs::write(&wasm, tp5c::compile(BOTH.as_bytes()).unwrap()).unwrap();
    let run = |args: &[&str]| {
        let o = Command::new("node")
            .arg(root().join("tests/run.mjs"))
            .args(args)
            .arg(&wasm)
            .output()
            .unwrap();
        (
            o.status.code(),
            String::from_utf8_lossy(&o.stderr).into_owned(),
        )
    };
    let (code, stderr) = run(&[]);
    assert_eq!(code, Some(2), "{stderr}");
    assert!(
        stderr.contains("unknown import host.ide_compile"),
        "{stderr}"
    );
    let (code, stderr) = run(&["--compiler", "/nonexistent/tp5c.wasm"]);
    assert_eq!(code, Some(2), "{stderr}");
    assert!(
        stderr.contains("cannot read /nonexistent/tp5c.wasm"),
        "{stderr}"
    );
}
