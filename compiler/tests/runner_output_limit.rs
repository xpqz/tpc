//! A program that loops forever writing output (a retry loop whose `ReadLn` fails at the end of
//! the input, as in two SWAG programs) must not make the test runner collect output until node
//! dies: the runner stops it at an output limit with a diagnostic. The program's heap is not
//! involved (it is fixed and never grows); the runner's collected stdout was the unbounded part.
mod common;
use common::OutputGuarded as _;
use common::*;
use std::process::Command;

const RETRY_FOREVER: &str = "program Prog;
var s: string; n, code: Integer;
begin
  repeat
    Write('number? ');
    ReadLn(s);
    Val(s, n, code)
  until code = 0;
  WriteLn(n)
end.
";

fn run_with_limit(limit: &str) -> (i32, usize, String) {
    let wasm = tp5c::compile(RETRY_FOREVER.as_bytes()).expect("compiles");
    let path = fresh_dir("outlimit").join("prog.wasm");
    std::fs::write(&path, wasm).unwrap();
    let empty = path.with_file_name("empty.in");
    std::fs::write(&empty, b"").unwrap();
    let o = Command::new("node")
        .arg(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../tests/run.mjs"))
        .env("TP5_OUTPUT_LIMIT", limit)
        .arg("--in")
        .arg(&empty)
        .arg(&path)
        .output_guarded()
        .expect("node must be installed");
    (
        o.status.code().unwrap_or(-1),
        o.stdout.len(),
        String::from_utf8_lossy(&o.stderr).into_owned(),
    )
}

use std::path::PathBuf;

#[test]
fn endless_output_stops_at_the_limit() {
    let (code, out_len, err) = run_with_limit("100000");
    assert_eq!(code, 2, "stderr: {err}");
    assert!(err.contains("output limit"), "stderr: {err}");
    assert!(
        out_len > 0 && out_len <= 100_000 + 64,
        "stdout {out_len} bytes"
    );
}

#[test]
fn a_program_within_the_limit_is_unaffected() {
    let src = "program Prog;\nbegin\n  WriteLn('hello')\nend.\n";
    let wasm = tp5c::compile(src.as_bytes()).expect("compiles");
    let path = fresh_dir("outlimit").join("hello.wasm");
    std::fs::write(&path, wasm).unwrap();
    let o = Command::new("node")
        .arg(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../tests/run.mjs"))
        .env("TP5_OUTPUT_LIMIT", "100")
        .arg(&path)
        .output_guarded()
        .unwrap();
    assert_eq!(String::from_utf8_lossy(&o.stdout), "hello\n");
    assert_eq!(o.status.code(), Some(0));
}
