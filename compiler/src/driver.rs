//! The compiler pipeline (lex, parse, check, codegen) and the native command line.
use crate::check::check;
use crate::codegen::gen;
use crate::diag::{code, Diag};
use crate::lex::lex;
use crate::parse::parse;

/// lex, parse, check, codegen.
pub fn compile(src: &[u8]) -> Result<Vec<u8>, Diag> {
    compile_with(include_bytes!("../../runtime/system.pas"), src)
}

/// TP reports end of file on the last line of the file, not after its final newline.
fn last_line(src: &[u8]) -> u32 {
    let newlines = src.iter().filter(|&&c| c == b'\n').count() as u32;
    let unterminated = !src.ends_with(b"\n") as u32;
    (newlines + unterminated).max(1)
}

/// `compile` with `system` as the source of the System declaration part.
pub fn compile_with(system: &[u8], src: &[u8]) -> Result<Vec<u8>, Diag> {
    let last_line = last_line(src);
    let at_last_line = |d: Diag| {
        let eof_in_user_source = d.code == code::UNEXPECTED_END_OF_FILE && d.line != 0;
        let line = if eof_in_user_source {
            last_line
        } else {
            d.line
        };
        Diag { line, ..d }
    };
    // Every token of System is at line 0, so its diagnostics never carry a user line.
    let mut system_tokens = lex(system).map_err(|d| Diag { line: 0, ..d })?;
    system_tokens.iter_mut().for_each(|t| t.line = 0);
    let user_tokens = lex(src).map_err(at_last_line)?;
    let (mut ast, root) = parse(system_tokens, user_tokens).map_err(at_last_line)?;
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
