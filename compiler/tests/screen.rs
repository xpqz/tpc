//! Runs the Node checks of the screen grid, key scripts and virtual clock (tests/host/screen.mjs).
use std::path::PathBuf;
use std::process::Command;

#[test]
fn screen_model_follows_the_contract() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let o = Command::new("node")
        .arg(root.join("tests/host/screen.mjs"))
        .output()
        .expect("node must be installed");
    let out = String::from_utf8_lossy(&o.stdout);
    assert!(
        o.status.success(),
        "{out}{}",
        String::from_utf8_lossy(&o.stderr)
    );
    assert!(
        out.lines().filter(|l| l.starts_with("ok ")).count() >= 20,
        "{out}"
    );
}
