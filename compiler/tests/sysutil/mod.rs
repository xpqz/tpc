//! Shared helpers for the System-unit and dead-code tests.
#![allow(dead_code)]
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// The committed runtime component that the compiler embeds.
pub fn system_pas() -> Vec<u8> {
    let path = root().join("runtime/system.pas");
    std::fs::read(&path).unwrap_or_else(|e| panic!("{} is missing: {e}", path.display()))
}

pub fn compile(src: &str) -> Vec<u8> {
    tp5c::compile(src.as_bytes()).unwrap_or_else(|d| panic!("compile failed: {d:?}\n{src}"))
}

pub fn compile_with(system: &[u8], src: &str) -> Result<Vec<u8>, tp5c::Diag> {
    tp5c::compile_with(system, src.as_bytes())
}

/// Runs a module under tests/run.mjs and returns its stdout; the exit code must be 0.
pub fn run(wasm: &[u8]) -> String {
    static N: AtomicU32 = AtomicU32::new(0);
    let dir = root().join("tmp/sysutil");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(format!(
        "{}-{}.wasm",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst)
    ));
    std::fs::write(&path, wasm).unwrap();
    let out = Command::new("node")
        .arg(root().join("tests/run.mjs"))
        .arg(&path)
        .output()
        .expect("node must be installed");
    assert!(
        out.status.success(),
        "run failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}

pub fn contains(hay: &[u8], needle: &str) -> bool {
    hay.windows(needle.len()).any(|w| w == needle.as_bytes())
}

/// True when the engine accepts the module.
pub fn run_validate(wasm: &[u8]) -> bool {
    let hex: String = wasm.iter().map(|b| format!("{b:02x}")).collect();
    let js = "console.log(WebAssembly.validate(Buffer.from(process.argv[1], 'hex')))";
    let out = Command::new("node")
        .args(["-e", js, &hex])
        .output()
        .expect("node must be installed");
    String::from_utf8_lossy(&out.stdout).trim() == "true"
}
