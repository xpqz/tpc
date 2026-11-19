#![allow(dead_code)]
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};

pub const HEADER: [u8; 8] = [0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];

pub const UNREACHABLE: u8 = 0x00;
pub const NOP: u8 = 0x01;
pub const RETURN: u8 = 0x0F;
pub const CALL: u8 = 0x10;
pub const LOCAL_GET: u8 = 0x20;
pub const LOCAL_SET: u8 = 0x21;
pub const LOCAL_TEE: u8 = 0x22;
pub const GLOBAL_GET: u8 = 0x23;
pub const GLOBAL_SET: u8 = 0x24;
pub const I32_LOAD: u8 = 0x28;
pub const F64_LOAD: u8 = 0x2B;
pub const I32_STORE: u8 = 0x36;
pub const F64_STORE: u8 = 0x39;
pub const I32_EQZ: u8 = 0x45;
pub const I32_LT_S: u8 = 0x48;
pub const I32_GT_S: u8 = 0x4A;
pub const F64_LT: u8 = 0x63;
pub const I32_ADD: u8 = 0x6A;
pub const I32_SUB: u8 = 0x6B;
pub const I32_MUL: u8 = 0x6C;
pub const F64_ADD: u8 = 0xA0;
pub const F64_MUL: u8 = 0xA2;
pub const F64_CONVERT_I32_S: u8 = 0xB7;

/// Size target for the hello-world module (docs/size.md); every size-guard test uses it.
pub const HELLO_SIZE_LIMIT: usize = 2048;

pub fn assert_hello_fits(wasm: &[u8]) {
    assert!(
        wasm.len() <= HELLO_SIZE_LIMIT,
        "hello-world is {} bytes, limit {HELLO_SIZE_LIMIT}",
        wasm.len()
    );
}

/// Section with id and exact content bytes (content length < 128).
pub fn section(id: u8, content: &[u8]) -> Vec<u8> {
    assert!(content.len() < 128);
    let mut v = vec![id, content.len() as u8];
    v.extend_from_slice(content);
    v
}

pub fn concat(parts: &[&[u8]]) -> Vec<u8> {
    parts.concat()
}

/// Decodes one LEB128 value; returns (value as i128, bytes consumed).
pub fn decode_leb(bytes: &[u8], signed: bool) -> (i128, usize) {
    assert!(!bytes.is_empty(), "no bytes to decode");
    let mut v: i128 = 0;
    for (i, &b) in bytes.iter().enumerate() {
        v |= ((b & 0x7F) as i128) << (7 * i);
        if b & 0x80 == 0 {
            let bits = 7 * (i + 1);
            if signed && b & 0x40 != 0 {
                v -= 1i128 << bits;
            }
            return (v, i + 1);
        }
    }
    panic!("unterminated LEB128");
}

/// Runs `js` under Node with `b` (a Buffer holding the module) in scope; returns stdout.
/// The module bytes arrive as hex on stdin.
pub fn node(js: &str, wasm: &[u8]) -> String {
    let hex: String = wasm.iter().map(|b| format!("{b:02x}")).collect();
    let prelude = "const b = Buffer.from(require('fs').readFileSync(0, 'utf8').trim(), 'hex');";
    let mut child = Command::new("node")
        .arg("-e")
        .arg(format!("{prelude}\n{js}"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("node must be installed");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(hex.as_bytes())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(
        out.status.success(),
        "node failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}

pub fn validates(wasm: &[u8]) -> bool {
    node("console.log(WebAssembly.validate(b))", wasm).trim() == "true"
}

/// A new, empty directory `tmp/KIND/PID-COUNTER` at the repository root, used by exactly one
/// caller. Process ids are reused, so a directory left by an earlier process can carry the
/// same name; `create_dir` fails on an existing directory and the counter moves on, so no
/// directory is ever shared and none is ever cleared.
pub fn fresh_dir(kind: &str) -> PathBuf {
    static NEXT: AtomicU32 = AtomicU32::new(0);
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("tmp")
        .join(kind);
    std::fs::create_dir_all(&base).unwrap();
    loop {
        let n = NEXT.fetch_add(1, Ordering::SeqCst);
        let dir = base.join(format!("{}-{n}", std::process::id()));
        match std::fs::create_dir(&dir) {
            Ok(()) => return dir,
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => panic!("{}: {e}", dir.display()),
        }
    }
}
