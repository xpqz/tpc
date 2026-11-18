#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Diag {
    pub code: u16,
    pub line: u32,
    pub col: u32,
}

/// Front end not present yet: every program is reported as unsupported.
pub fn compile(_src: &[u8]) -> Result<Vec<u8>, Diag> {
    Err(Diag {
        code: 200,
        line: 1,
        col: 1,
    })
}

/// `tp5c in.pas -o out.wasm`; returns the process exit code.
pub fn run(_args: &[String]) -> i32 {
    0
}
