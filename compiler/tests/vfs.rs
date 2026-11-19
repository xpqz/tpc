//! Runs the Node checks of the test runner's file system and console input (tests/host/vfs.mjs).
use std::path::PathBuf;
use std::process::Command;

#[test]
fn runner_file_system_follows_the_contract() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let o = Command::new("node")
        .arg(root.join("tests/host/vfs.mjs"))
        .output()
        .expect("node must be installed");
    let out = String::from_utf8_lossy(&o.stdout);
    assert!(
        o.status.success(),
        "{out}{}",
        String::from_utf8_lossy(&o.stderr)
    );
    assert!(
        out.lines().filter(|l| l.starts_with("ok ")).count() >= 25,
        "{out}"
    );
}
