mod common;
use common::*;
use std::path::PathBuf;
use std::process::{Command, Output};
use tp5c::wasm::*;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn scratch(name: &str) -> PathBuf {
    let dir = root().join("tmp/runner");
    std::fs::create_dir_all(&dir).unwrap();
    dir.join(name)
}

fn runner(args: &[&std::ffi::OsStr]) -> Output {
    Command::new("node")
        .arg(root().join("tests/run.mjs"))
        .args(args)
        .output()
        .expect("node must be installed")
}

/// Writes `wasm` (and optionally the expected output) to tmp/ and runs the runner on it.
fn run_with(name: &str, wasm: &[u8], expected: Option<&[u8]>) -> Output {
    let w = scratch(&format!("{name}.wasm"));
    std::fs::write(&w, wasm).unwrap();
    let mut args = vec![w.as_os_str().to_owned()];
    if let Some(e) = expected {
        let p = scratch(&format!("{name}.out"));
        std::fs::write(&p, e).unwrap();
        args.push(p.into_os_string());
    }
    let args: Vec<&std::ffi::OsStr> = args.iter().map(|a| a.as_os_str()).collect();
    runner(&args)
}

fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}

/// Host imports in the order of `docs/abi.md`, with their value signatures.
const HOST: &[(&str, &[u8], &[u8])] = &[
    ("write", &[I32, I32, I32], &[]),
    ("readkey", &[], &[I32]),
    ("keypressed", &[], &[I32]),
    ("readline", &[I32, I32], &[I32]),
    ("screen", &[I32, I32, I32, I32], &[]),
    ("fs_open", &[I32, I32, I32], &[I32]),
    ("fs_read", &[I32, I32, I32], &[I32]),
    ("fs_write", &[I32, I32, I32], &[I32]),
    ("fs_close", &[I32], &[I32]),
    ("fmt_real", &[F64, I32, I32, I32], &[I32]),
    ("halt", &[I32], &[]),
    ("rterror", &[I32], &[]),
    ("time", &[], &[I32]),
    ("sleep", &[I32], &[]),
    ("compile", &[], &[]),
];

enum Step<'a> {
    Write(i32, &'a [u8]),
    WriteRaw(i32, i32, i32),
    Halt(i32),
    RtError(i32),
    Call(&'a str),
    Trap,
}

/// Module importing the named host functions (in `imports` order), with `data` at 256 and
/// an exported `main` running `steps`. `memory` and `main` are exported unless suppressed.
struct Build<'a> {
    imports: Vec<(&'a str, &'a str)>,
    steps: Vec<Step<'a>>,
    export_memory: bool,
    export_main: bool,
}

fn build(imports: &[&str], steps: Vec<Step>) -> Vec<u8> {
    let b = Build {
        imports: imports.iter().map(|n| ("host", *n)).collect(),
        steps,
        export_memory: true,
        export_main: true,
    };
    emit(&b)
}

fn emit(b: &Build) -> Vec<u8> {
    let mut m = Module::new();
    let mut index = std::collections::HashMap::new();
    for (module, name) in &b.imports {
        let (p, r) = HOST
            .iter()
            .find(|h| h.0 == *name)
            .map_or((&[][..], &[][..]), |h| (h.1, h.2));
        let ty = m.ty(p, r);
        index.insert(*name, m.import_func(module, name, ty));
    }
    let main_ty = m.ty(&[], &[]);
    m.memory(1);
    let mut data = Vec::new();
    let mut c = Code::new();
    for s in &b.steps {
        match s {
            Step::Write(fd, bytes) => {
                let at = 256 + data.len() as i32;
                data.extend_from_slice(bytes);
                c.i32_const(*fd).i32_const(at).i32_const(bytes.len() as i32);
                c.idx(CALL, index["write"]);
            }
            Step::WriteRaw(fd, ptr, len) => {
                c.i32_const(*fd).i32_const(*ptr).i32_const(*len);
                c.idx(CALL, index["write"]);
            }
            Step::Halt(code) => {
                c.i32_const(*code).idx(CALL, index["halt"]);
            }
            Step::RtError(code) => {
                c.i32_const(*code).idx(CALL, index["rterror"]);
            }
            Step::Call(name) => {
                c.idx(CALL, index[name]);
                if !HOST.iter().find(|h| h.0 == *name).unwrap().2.is_empty() {
                    c.op(0x1A); // drop
                }
            }
            Step::Trap => {
                c.op(UNREACHABLE);
            }
        }
    }
    let f = m.func(main_ty, &[], c);
    m.data(256, &data);
    if b.export_memory {
        m.export("memory", MEM, 0);
    }
    if b.export_main {
        m.export("main", FUNC, f);
    }
    let w = m.finish();
    assert!(validates(&w), "fixture module must be valid wasm");
    w
}

fn ok_module() -> Vec<u8> {
    build(&["write"], vec![Step::Write(1, b"OK\n")])
}

#[test]
fn matching_output_passes_silently_with_exit_0() {
    let o = run_with("match", &ok_module(), Some(b"OK\n"));
    assert_eq!(o.status.code(), Some(0), "stderr: {}", stderr(&o));
    assert!(o.stdout.is_empty());
    assert!(o.stderr.is_empty());
}

#[test]
fn committed_fixture_matches_fpc_output() {
    let want = std::fs::read(root().join("tests/runner/ok.out")).unwrap();
    assert_eq!(want, b"OK\n");
    let o = run_with("fixture", &ok_module(), Some(&want));
    assert_eq!(o.status.code(), Some(0), "stderr: {}", stderr(&o));
}

#[test]
fn mismatch_fails_with_diff_and_exit_1() {
    let w = build(&["write"], vec![Step::Write(1, b"one\ntwo\nthree\n")]);
    let o = run_with("mismatch", &w, Some(b"one\nTWO\nthree\n"));
    assert_eq!(o.status.code(), Some(1), "stderr: {}", stderr(&o));
    let e = stderr(&o);
    assert!(e.contains("FAIL"), "{e}");
    assert!(
        e.contains("-TWO"),
        "expected-only line marked with '-': {e}"
    );
    assert!(e.contains("+two"), "actual-only line marked with '+': {e}");
    assert!(
        e.contains(" one"),
        "common line shown with a leading space: {e}"
    );
    assert!(e.contains(" three"), "{e}");
}

#[test]
fn comparison_is_byte_exact_trailing_newline_and_crlf() {
    for (name, got, want) in [
        ("nl_missing", &b"OK"[..], &b"OK\n"[..]),
        ("nl_extra", b"OK\n", b"OK"),
        ("crlf", b"OK\n", b"OK\r\n"),
        ("empty_vs_text", b"", b"OK\n"),
    ] {
        let w = if got.is_empty() {
            build(&["write"], vec![Step::Write(1, b"")])
        } else {
            build(&["write"], vec![Step::Write(1, got)])
        };
        let o = run_with(name, &w, Some(want));
        assert_eq!(o.status.code(), Some(1), "{name}: {}", stderr(&o));
        assert!(stderr(&o).contains("FAIL"), "{name}");
    }
}

#[test]
fn empty_output_matches_empty_expected_file() {
    let w = build(&["write"], vec![Step::Write(1, b"")]);
    let o = run_with("empty_ok", &w, Some(b""));
    assert_eq!(o.status.code(), Some(0), "stderr: {}", stderr(&o));
}

#[test]
fn high_bit_mismatch_reports_first_differing_byte_offset() {
    let w = build(&["write"], vec![Step::Write(1, &[0x41, 0x80, 0x43])]);
    let o = run_with("hi_mismatch", &w, Some(&[0x41, 0x81, 0x43]));
    assert_eq!(o.status.code(), Some(1));
    assert!(
        stderr(&o).contains("first difference at byte 1"),
        "{}",
        stderr(&o)
    );
}

#[test]
fn without_expected_file_output_is_printed_and_exit_0() {
    let o = run_with("print", &ok_module(), None);
    assert_eq!(o.status.code(), Some(0), "stderr: {}", stderr(&o));
    assert_eq!(o.stdout, b"OK\n");
}

#[test]
fn halt_keeps_earlier_output_stops_execution_and_sets_exit_code() {
    let w = build(
        &["write", "halt"],
        vec![
            Step::Write(1, b"before\n"),
            Step::Halt(3),
            Step::Write(1, b"after\n"),
        ],
    );
    let o = run_with("halt3", &w, None);
    assert_eq!(o.status.code(), Some(3), "stderr: {}", stderr(&o));
    assert_eq!(o.stdout, b"before\n");
}

#[test]
fn halt_0_is_success_and_matching_expected_keeps_program_exit_code() {
    let w = build(
        &["write", "halt"],
        vec![Step::Write(1, b"x"), Step::Halt(0), Step::Trap],
    );
    let o = run_with("halt0", &w, Some(b"x"));
    assert_eq!(o.status.code(), Some(0), "stderr: {}", stderr(&o));
    let w = build(
        &["write", "halt"],
        vec![Step::Write(1, b"x"), Step::Halt(5)],
    );
    let o = run_with("halt5_match", &w, Some(b"x"));
    assert_eq!(o.status.code(), Some(5), "stderr: {}", stderr(&o));
    let o = run_with("halt5_mismatch", &w, Some(b"y"));
    assert_eq!(
        o.status.code(),
        Some(1),
        "output mismatch wins: {}",
        stderr(&o)
    );
}

#[test]
fn rterror_prints_line_to_stdout_and_exits_with_the_code() {
    let w = build(
        &["write", "rterror"],
        vec![
            Step::Write(1, b"partial"),
            Step::RtError(200),
            Step::Write(1, b"after"),
        ],
    );
    let o = run_with("rt200", &w, None);
    assert_eq!(o.status.code(), Some(200), "stderr: {}", stderr(&o));
    assert_eq!(o.stdout, b"partialRuntime error 200\n");
}

#[test]
fn rterror_line_participates_in_expected_comparison() {
    let w = build(&["rterror"], vec![Step::RtError(201)]);
    let o = run_with("rt201", &w, Some(b"Runtime error 201\n"));
    assert_eq!(o.status.code(), Some(201), "stderr: {}", stderr(&o));
    assert!(o.stderr.is_empty());
}

#[test]
fn high_bit_bytes_pass_through_byte_exactly() {
    let all: Vec<u8> = (0..=255u8).collect();
    let w = build(&["write"], vec![Step::Write(1, &all)]);
    let o = run_with("bytes_print", &w, None);
    assert_eq!(o.status.code(), Some(0), "stderr: {}", stderr(&o));
    assert_eq!(o.stdout, all);
    let o = run_with("bytes_cmp", &w, Some(&all));
    assert_eq!(o.status.code(), Some(0), "stderr: {}", stderr(&o));
}

#[test]
fn zero_length_write_is_a_no_op_even_with_wild_pointer() {
    let w = build(
        &["write"],
        vec![
            Step::WriteRaw(1, 256, 0),
            Step::WriteRaw(1, 0x7FFF_FFF0, 0),
            Step::Write(1, b"x"),
        ],
    );
    let o = run_with("zero_len", &w, None);
    assert_eq!(o.status.code(), Some(0), "stderr: {}", stderr(&o));
    assert_eq!(o.stdout, b"x");
}

#[test]
fn writes_accumulate_in_order() {
    let w = build(
        &["write"],
        vec![
            Step::Write(1, b"a"),
            Step::Write(1, b"bc"),
            Step::Write(1, b"\n"),
        ],
    );
    assert_eq!(run_with("order", &w, None).stdout, b"abc\n");
}

#[test]
fn fd_2_goes_to_stderr_not_stdout() {
    let w = build(
        &["write"],
        vec![Step::Write(2, b"err"), Step::Write(1, b"out")],
    );
    let o = run_with("fd2", &w, None);
    assert_eq!(o.status.code(), Some(0));
    assert_eq!(o.stdout, b"out");
    assert!(stderr(&o).contains("err"));
}

#[test]
fn write_to_unknown_fd_is_a_runner_error() {
    let w = build(&["write"], vec![Step::Write(7, b"x")]);
    let o = run_with("fd7", &w, None);
    assert_eq!(o.status.code(), Some(2));
    assert!(stderr(&o).contains("bad fd 7"), "{}", stderr(&o));
}

#[test]
fn write_outside_memory_is_a_runner_error() {
    let w = build(&["write"], vec![Step::WriteRaw(1, 65530, 100)]);
    let o = run_with("oob", &w, None);
    assert_eq!(o.status.code(), Some(2));
    assert!(stderr(&o).contains("out of bounds"), "{}", stderr(&o));
}

#[test]
fn invalid_wasm_exits_2_with_message() {
    for (name, bytes) in [
        ("garbage", b"not a wasm module".to_vec()),
        ("empty", Vec::new()),
        ("truncated", {
            let m = ok_module();
            m[..m.len() - 3].to_vec()
        }),
    ] {
        let o = run_with(name, &bytes, None);
        assert_eq!(o.status.code(), Some(2), "{name}: {}", stderr(&o));
        assert!(
            stderr(&o).contains("invalid wasm"),
            "{name}: {}",
            stderr(&o)
        );
        assert!(o.stdout.is_empty());
    }
}

#[test]
fn missing_files_and_bad_usage_exit_2() {
    let w = scratch("present.wasm");
    std::fs::write(&w, ok_module()).unwrap();
    let missing = scratch("does_not_exist");
    let _ = std::fs::remove_file(&missing);

    let o = runner(&[missing.as_os_str()]);
    assert_eq!(o.status.code(), Some(2));
    assert!(stderr(&o).contains("does_not_exist"), "{}", stderr(&o));

    let o = runner(&[w.as_os_str(), missing.as_os_str()]);
    assert_eq!(o.status.code(), Some(2));
    assert!(stderr(&o).contains("does_not_exist"), "{}", stderr(&o));
    assert!(
        o.stdout.is_empty(),
        "module must not run when expected file is missing"
    );

    let o = runner(&[]);
    assert_eq!(o.status.code(), Some(2));
    assert!(stderr(&o).contains("usage"), "{}", stderr(&o));
}

#[test]
fn stubbed_import_instantiates_and_reports_message_when_called() {
    let w = build(
        &["write", "compile"],
        vec![Step::Write(1, b"hi"), Step::Call("compile")],
    );
    let o = run_with("compile_stub", &w, None);
    assert_eq!(o.status.code(), Some(2));
    assert!(
        stderr(&o).contains("host import compile not available in test runner"),
        "{}",
        stderr(&o)
    );
    assert_eq!(o.stdout, b"hi", "output before the stub call is kept");
}

#[test]
fn module_importing_every_host_function_instantiates() {
    let names: Vec<&str> = HOST.iter().map(|h| h.0).collect();
    let w = build(&names, vec![Step::Write(1, b"OK\n")]);
    // A program that imports `screen` ends with the dump of its (blank) screen.
    let want = format!("OK\n{}cursor 1 1\nexit 0\n", "\n".repeat(25));
    let o = run_with("all_imports", &w, Some(want.as_bytes()));
    assert_eq!(o.status.code(), Some(0), "stderr: {}", stderr(&o));
}

#[test]
fn unknown_imports_fail_clearly() {
    for (name, module, func) in [("unk_func", "host", "bogus"), ("unk_mod", "env", "write")] {
        let b = Build {
            imports: vec![(module, func)],
            steps: vec![],
            export_memory: true,
            export_main: true,
        };
        let o = run_with(name, &emit(&b), None);
        assert_eq!(o.status.code(), Some(2), "{name}: {}", stderr(&o));
        let want = format!("unknown import {module}.{func}");
        assert!(stderr(&o).contains(&want), "{name}: {}", stderr(&o));
    }
}

#[test]
fn missing_main_or_memory_export_is_a_runner_error() {
    let b = Build {
        imports: vec![("host", "write")],
        steps: vec![],
        export_memory: true,
        export_main: false,
    };
    let o = run_with("no_main", &emit(&b), None);
    assert_eq!(o.status.code(), Some(2));
    assert!(stderr(&o).contains("export main"), "{}", stderr(&o));

    let b = Build {
        imports: vec![("host", "write")],
        steps: vec![],
        export_memory: false,
        export_main: true,
    };
    let o = run_with("no_memory", &emit(&b), None);
    assert_eq!(o.status.code(), Some(2));
    assert!(stderr(&o).contains("export memory"), "{}", stderr(&o));
}

#[test]
fn wasm_trap_is_reported_with_exit_2_and_output_kept() {
    let w = build(&["write"], vec![Step::Write(1, b"x"), Step::Trap]);
    let o = run_with("trap", &w, None);
    assert_eq!(o.status.code(), Some(2));
    assert!(stderr(&o).contains("trap"), "{}", stderr(&o));
    assert_eq!(o.stdout, b"x");
}

#[test]
fn js_api_run_returns_stdout_and_code() {
    let w = build(
        &["write", "halt"],
        vec![Step::Write(1, &[0xFF, 0x0A]), Step::Halt(4)],
    );
    let p = scratch("api.wasm");
    std::fs::write(&p, &w).unwrap();
    let script = format!(
        "import('{}').then(async m => {{ \
         const r = await m.run(require('fs').readFileSync('{}')); \
         console.log(JSON.stringify([Buffer.from(r.stdout).toString('hex'), r.code])); }})",
        root().join("tests/run.mjs").display(),
        p.display()
    );
    let o = Command::new("node").arg("-e").arg(script).output().unwrap();
    assert!(o.status.success(), "{}", stderr(&o));
    assert_eq!(String::from_utf8_lossy(&o.stdout).trim(), r#"["ff0a",4]"#);
}

#[test]
fn importing_the_module_does_not_run_the_cli() {
    let script = format!(
        "import('{}').then(() => console.log('imported'))",
        root().join("tests/run.mjs").display()
    );
    let o = Command::new("node").arg("-e").arg(script).output().unwrap();
    assert!(o.status.success(), "{}", stderr(&o));
    assert_eq!(String::from_utf8_lossy(&o.stdout).trim(), "imported");
}

/// Module exporting mutable global `sp` (initial 65536) when `export_sp`. `main` adds
/// `delta` to it, writes "done", then calls `halt(code)` if given.
fn sp_module(export_sp: bool, delta: i32, halt: Option<i32>) -> Vec<u8> {
    let mut m = Module::new();
    let write = m.ty(&[I32, I32, I32], &[]);
    let halt_ty = m.ty(&[I32], &[]);
    let main_ty = m.ty(&[], &[]);
    let write_f = m.import_func("host", "write", write);
    let halt_f = m.import_func("host", "halt", halt_ty);
    m.memory(1);
    m.data(256, b"done\n");
    let sp = m.global_i32(true, 65536);
    let mut c = Code::new();
    c.idx(GLOBAL_GET, sp)
        .i32_const(delta)
        .op(I32_ADD)
        .idx(GLOBAL_SET, sp);
    c.i32_const(1)
        .i32_const(256)
        .i32_const(5)
        .idx(CALL, write_f);
    if let Some(code) = halt {
        c.i32_const(code).idx(CALL, halt_f);
    }
    let f = m.func(main_ty, &[], c);
    m.export("memory", MEM, 0);
    m.export("main", FUNC, f);
    if export_sp {
        m.export("sp", GLOBAL, sp);
    }
    let w = m.finish();
    assert!(validates(&w), "fixture module must be valid wasm");
    w
}

#[test]
fn sp_restored_by_main_passes() {
    let w = sp_module(true, 0, None);
    let o = run_with("sp_same", &w, Some(b"done\n"));
    assert_eq!(o.status.code(), Some(0), "stderr: {}", stderr(&o));
    assert!(o.stderr.is_empty());
}

#[test]
fn sp_not_restored_is_a_runner_error() {
    for (name, delta) in [("sp_down", -16), ("sp_up", 4)] {
        let w = sp_module(true, delta, None);
        let o = run_with(name, &w, Some(b"done\n"));
        assert_eq!(o.status.code(), Some(2), "{name}: {}", stderr(&o));
        assert!(
            stderr(&o).contains("run.mjs: sp not restored"),
            "{name}: {}",
            stderr(&o)
        );
        assert_eq!(
            o.stdout, b"done\n",
            "{name}: output before the error is kept"
        );
    }
}

#[test]
fn sp_not_restored_is_an_error_in_print_mode_too() {
    let o = run_with("sp_print", &sp_module(true, -8, None), None);
    assert_eq!(o.status.code(), Some(2), "stderr: {}", stderr(&o));
    assert!(stderr(&o).contains("sp not restored"), "{}", stderr(&o));
}

#[test]
fn sp_is_not_checked_after_halt() {
    let o = run_with("sp_halt", &sp_module(true, -16, Some(3)), Some(b"done\n"));
    assert_eq!(o.status.code(), Some(3), "stderr: {}", stderr(&o));
    assert!(o.stderr.is_empty(), "{}", stderr(&o));
}

#[test]
fn module_without_sp_export_is_not_checked() {
    let o = run_with("sp_absent", &sp_module(false, -16, None), Some(b"done\n"));
    assert_eq!(o.status.code(), Some(0), "stderr: {}", stderr(&o));
}
