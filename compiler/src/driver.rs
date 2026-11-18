use crate::check::check;
use crate::codegen::gen;
use crate::lex::lex;
use crate::parse::parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Diag {
    pub code: u16,
    pub line: u32,
    pub col: u32,
}

/// lex, parse, check, codegen.
pub fn compile(src: &[u8]) -> Result<Vec<u8>, Diag> {
    compile_with(&[], src)
}

/// `compile` with `system` as the source of the System declaration part.
pub fn compile_with(_system: &[u8], src: &[u8]) -> Result<Vec<u8>, Diag> {
    // TP reports end of file on the last line of the file, not after its final newline.
    let last_line =
        src.iter().filter(|&&c| c == b'\n').count() as u32 + !src.ends_with(b"\n") as u32;
    let diag = |d: Diag| Diag {
        line: if d.code == 10 {
            last_line.max(1)
        } else {
            d.line
        },
        ..d
    };
    let toks = lex(src).map_err(|e| {
        diag(Diag {
            code: e.num,
            line: e.line,
            col: e.col,
        })
    })?;
    let (mut ast, root) = parse(toks).map_err(diag)?;
    let info = check(&mut ast, root)?;
    Ok(gen(&ast, &info, root))
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
