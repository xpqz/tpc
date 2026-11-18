//! The wasm32 release build exports the compiler API (plan section 3).
use std::path::PathBuf;
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

#[test]
fn wasm_exports_compile_and_report_errors() {
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

    let ok = root().join("tests/lang/hello_string.pas");
    let err = root().join("tests/neg/assign_to_constant.pas");
    let d = tp5c::compile(&std::fs::read(&err).unwrap()).unwrap_err();
    assert_ne!(d.code, 0);
    let out = Command::new("node")
        .arg(root().join("tests/wasm_api.mjs"))
        .arg(&wasm)
        .arg(&ok)
        .arg(ok.with_extension("out"))
        .arg(&err)
        .args([d.code.to_string(), d.line.to_string(), d.col.to_string()])
        .output()
        .expect("node must be installed");
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}
