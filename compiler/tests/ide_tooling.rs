//! Tooling for the IDE (decisions 40 to 44): the TP 5.0 limit probes (tests/ide/limits) against
//! this compiler. The tp_only probes are not run here: they record
//! what only the real Turbo Pascal does.
mod common;
use common::OutputGuarded as _;
use std::path::PathBuf;
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn node(args: &[&str]) {
    let o = Command::new("node")
        .args(args)
        .current_dir(root())
        .output_guarded()
        .expect("node must be installed");
    assert!(
        o.status.success(),
        "node {args:?}: {}{}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    );
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
            .output_guarded()
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
