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
pub fn run(args: &[String]) -> i32 {
    let [input, flag, output] = args else {
        return fail("usage: tp5c in.pas -o out.wasm", 2);
    };
    if flag != "-o" {
        return fail("usage: tp5c in.pas -o out.wasm", 2);
    }
    let Ok(src) = std::fs::read(input) else {
        return fail(&["tp5c: cannot read ", input].concat(), 2);
    };
    match compile(&src) {
        Ok(wasm) => match std::fs::write(output, wasm) {
            Ok(()) => 0,
            Err(_) => fail(&["tp5c: cannot write ", output].concat(), 2),
        },
        Err(d) => fail(&crate::diag::render(input, &d), 1),
    }
}

fn fail(msg: &str, code: i32) -> i32 {
    use std::io::Write;
    let mut e = std::io::stderr();
    let _ = e.write_all(msg.as_bytes());
    let _ = e.write_all(b"\n");
    code
}
