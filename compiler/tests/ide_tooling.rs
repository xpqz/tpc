//! Tooling for the IDE (decisions 40 to 44): the draft host imports against the real compiler.wasm,
//! the draft browser key mapping against tools/crt/KEYS.TBL, and the TP 5.0 limit probes
//! (tests/ide/limits) against this compiler. The tp_only probes are not run here: they record
//! what only the real Turbo Pascal does.
use std::path::PathBuf;
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn node(args: &[&str]) {
    let o = Command::new("node")
        .args(args)
        .current_dir(root())
        .output()
        .expect("node must be installed");
    assert!(
        o.status.success(),
        "node {args:?}: {}{}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn draft_browser_key_mapping_agrees_with_the_key_table() {
    node(&["tests/ide/draft/key_bytes_test.mjs"]);
}

#[test]
fn draft_ide_imports_compile_and_run_with_the_real_compiler() {
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
    node(&["tests/ide/draft/draft_test.mjs", wasm.to_str().unwrap()]);
}

#[test]
fn limit_probes_give_the_same_output_as_real_turbo_pascal() {
    let scratch = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ide_limits");
    std::fs::create_dir_all(&scratch).unwrap();
    for name in [
        "getmem_largest_block",
        "index_arithmetic_above_32767",
        "longint_index_above_32767",
    ] {
        let src = root().join(format!("tests/ide/limits/{name}.pas"));
        let wasm = scratch.join(format!("{name}.wasm"));
        let compile = Command::new(env!("CARGO_BIN_EXE_tp5c"))
            .arg(&src)
            .arg("-o")
            .arg(&wasm)
            .output()
            .unwrap();
        assert!(
            compile.status.success(),
            "{name}: {}",
            String::from_utf8_lossy(&compile.stderr)
        );
        node(&[
            "tests/run.mjs",
            wasm.to_str().unwrap(),
            src.with_extension("out").to_str().unwrap(),
        ]);
    }
}
