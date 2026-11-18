mod common;
use common::*;
use std::process::Command;
use tp5c::wasm::*;

/// Phase 0 exit: the emitter builds a module that imports `host.write` and prints "OK";
/// it validates and runs under Node through the test runner against the expected file.
#[test]
fn emitted_ok_module_validates_and_passes_through_the_runner() {
    let mut m = Module::new();
    let write = m.ty(&[I32, I32, I32], &[]);
    let main = m.ty(&[], &[]);
    m.import_func("host", "write", write);
    m.memory(1);
    m.data(256, b"OK\n");
    let mut c = Code::new();
    c.i32_const(1).i32_const(256).i32_const(3).idx(CALL, 0);
    let f = m.func(main, &[], c);
    m.export("memory", MEM, 0);
    m.export("main", FUNC, f);
    let w = m.finish();
    assert!(validates(&w));

    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let dir = root.join("tmp");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("phase0_ok.wasm");
    std::fs::write(&path, &w).unwrap();
    let out = Command::new("node")
        .arg(root.join("tests/run.mjs"))
        .arg(&path)
        .arg(root.join("tests/runner/ok.out"))
        .output()
        .unwrap();
    assert_eq!(
        out.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let out = Command::new("node")
        .arg(root.join("tests/run.mjs"))
        .arg(&path)
        .output()
        .unwrap();
    assert_eq!(out.stdout, b"OK\n");
}
