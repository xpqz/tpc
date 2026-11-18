#![allow(dead_code, unused_imports)]
pub use tp5c::lex::{lex, LexErr, Tk, Tok};

/// Lexes `src`, requires success and a trailing Eof, returns the tokens without it.
pub fn toks(src: &str) -> Vec<Tok> {
    let mut v = lex(src.as_bytes()).unwrap_or_else(|e| panic!("{src:?}: lex error {e:?}"));
    let last = v.pop().unwrap_or_else(|| panic!("{src:?}: no Eof token"));
    assert_eq!(last.kind, Tk::Eof, "{src:?}: last token");
    v
}

pub fn kinds(src: &str) -> Vec<Tk> {
    toks(src).iter().map(|t| t.kind).collect()
}

/// Lexes a single-token source and returns that token.
pub fn one(src: &str) -> Tok {
    let v = toks(src);
    assert_eq!(v.len(), 1, "{src:?}: expected one token, got {v:?}");
    v.into_iter().next().unwrap()
}

pub fn int(src: &str) -> i64 {
    let t = one(src);
    assert_eq!(t.kind, Tk::IntLit, "{src:?}");
    t.int
}

pub fn real(src: &str) -> f64 {
    let t = one(src);
    assert_eq!(t.kind, Tk::RealLit, "{src:?}");
    t.real
}

pub fn string(src: &str) -> Vec<u8> {
    let t = one(src);
    assert_eq!(t.kind, Tk::StrLit, "{src:?}");
    t.text
}

pub fn err(src: &str) -> (u16, u32, u32) {
    match lex(src.as_bytes()) {
        Ok(v) => panic!("{src:?}: expected error, got {v:?}"),
        Err(e) => (e.num, e.line, e.col),
    }
}
