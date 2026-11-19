//! The browser-side host code that runs under Node: `host/keys.js` (key ring, key mapping),
//! `host/terminal.js` (canvas renderer on a recording context) and `host/worker.js` (a program in
//! a worker thread: blocking readkey, sleep, stop). Browser-level checks are `tests/host/*`'s
//! successor in the Playwright suite.
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn node(script: &str) {
    let o = std::process::Command::new("node")
        .arg(root().join("tests/host").join(script))
        .output()
        .expect("node must be installed");
    let out = String::from_utf8_lossy(&o.stdout);
    let err = String::from_utf8_lossy(&o.stderr);
    assert!(o.status.success(), "{script}: {out}{err}");
}

#[test]
fn key_queue_and_key_mapping() {
    node("keys.mjs");
}

#[test]
fn terminal_renders_the_grid() {
    node("terminal.mjs");
}

#[test]
fn worker_runs_and_stops_programs() {
    node("worker.mjs");
}

#[test]
fn browser_key_mapping_agrees_with_the_key_table() {
    node("key_bytes.mjs");
}

#[test]
fn persistence_store_and_write_hook() {
    node("persistence.mjs");
}

#[test]
fn console_readln_without_crt_reads_typed_lines() {
    let o = std::process::Command::new("node")
        .arg(root().join("tests/host/console_readln.mjs"))
        .env("TP5C", env!("CARGO_BIN_EXE_tp5c"))
        .output()
        .expect("node must be installed");
    let out = String::from_utf8_lossy(&o.stdout);
    let err = String::from_utf8_lossy(&o.stderr);
    assert!(o.status.success(), "console_readln.mjs: {out}{err}");
}
