#![allow(dead_code, unused_imports)]
use tp5c::lex::Tk as R;
pub use tp5c::lex::{lex, Tok};

/// Lexes `src`, requires success and a trailing Eof, returns the tokens without it.
pub fn toks(src: &str) -> Vec<Tok> {
    let mut v = lex(src.as_bytes()).unwrap_or_else(|e| panic!("{src:?}: lex error {e:?}"));
    let last = v.pop().unwrap_or_else(|| panic!("{src:?}: no Eof token"));
    assert_eq!(last.kind, R::Eof, "{src:?}: last token");
    v
}

pub fn kinds(src: &str) -> Vec<Tk> {
    toks(src).iter().map(k).collect()
}

/// Lexes a single-token source and returns that token.
pub fn one(src: &str) -> Tok {
    let v = toks(src);
    assert_eq!(v.len(), 1, "{src:?}: expected one token, got {v:?}");
    v.into_iter().next().unwrap()
}

pub fn int(src: &str) -> i64 {
    let t = one(src);
    assert_eq!(t.kind, R::IntLit, "{src:?}");
    t.int
}

pub fn real(src: &str) -> f64 {
    let t = one(src);
    assert_eq!(t.kind, R::RealLit, "{src:?}");
    t.real
}

pub fn string(src: &str) -> Vec<u8> {
    let t = one(src);
    assert_eq!(t.kind, R::StrLit, "{src:?}");
    t.text
}

pub fn err(src: &str) -> (u16, u32, u32) {
    match lex(src.as_bytes()) {
        Ok(v) => panic!("{src:?}: expected error, got {v:?}"),
        Err(e) => (e.code, e.line, e.col),
    }
}

/// Test-side view of a token: keywords and symbols named by variant, other kinds as-is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tk {
    Ident,
    IntLit,
    RealLit,
    StrLit,
    Directive,
    Ctrl,
    Eof,
    And,
    Array,
    Asm,
    Begin,
    Case,
    Const,
    Div,
    Do,
    Downto,
    Else,
    End,
    File,
    For,
    Function,
    Goto,
    If,
    Implementation,
    In,
    Inline,
    Interface,
    Label,
    Mod,
    Nil,
    Not,
    Of,
    Or,
    Packed,
    Procedure,
    Program,
    Record,
    Repeat,
    Set,
    Shl,
    Shr,
    String,
    Then,
    To,
    Type,
    Unit,
    Until,
    Uses,
    Var,
    While,
    With,
    Xor,
    Absolute,
    External,
    Forward,
    Interrupt,
    Plus,
    Minus,
    Star,
    Slash,
    Eq,
    Lt,
    Gt,
    Le,
    Ge,
    Ne,
    Assign,
    LParen,
    RParen,
    LBrack,
    RBrack,
    Dot,
    DotDot,
    Comma,
    Semi,
    Colon,
    Caret,
    At,
}

const KWS: &[Tk] = &[
    Tk::And,
    Tk::Array,
    Tk::Asm,
    Tk::Begin,
    Tk::Case,
    Tk::Const,
    Tk::Div,
    Tk::Do,
    Tk::Downto,
    Tk::Else,
    Tk::End,
    Tk::File,
    Tk::For,
    Tk::Function,
    Tk::Goto,
    Tk::If,
    Tk::Implementation,
    Tk::In,
    Tk::Inline,
    Tk::Interface,
    Tk::Label,
    Tk::Mod,
    Tk::Nil,
    Tk::Not,
    Tk::Of,
    Tk::Or,
    Tk::Packed,
    Tk::Procedure,
    Tk::Program,
    Tk::Record,
    Tk::Repeat,
    Tk::Set,
    Tk::Shl,
    Tk::Shr,
    Tk::String,
    Tk::Then,
    Tk::To,
    Tk::Type,
    Tk::Unit,
    Tk::Until,
    Tk::Uses,
    Tk::Var,
    Tk::While,
    Tk::With,
    Tk::Xor,
    Tk::Absolute,
    Tk::External,
    Tk::Forward,
    Tk::Interrupt,
];

const SYMS: [(&str, Tk); 22] = [
    ("+", Tk::Plus),
    ("-", Tk::Minus),
    ("*", Tk::Star),
    ("/", Tk::Slash),
    ("=", Tk::Eq),
    ("<", Tk::Lt),
    (">", Tk::Gt),
    ("<=", Tk::Le),
    (">=", Tk::Ge),
    ("<>", Tk::Ne),
    (":=", Tk::Assign),
    ("(", Tk::LParen),
    (")", Tk::RParen),
    ("[", Tk::LBrack),
    ("]", Tk::RBrack),
    (".", Tk::Dot),
    ("..", Tk::DotDot),
    (",", Tk::Comma),
    (";", Tk::Semi),
    (":", Tk::Colon),
    ("^", Tk::Caret),
    ("@", Tk::At),
];

pub fn k(t: &Tok) -> Tk {
    match t.kind {
        R::Ident => Tk::Ident,
        R::IntLit => Tk::IntLit,
        R::RealLit => Tk::RealLit,
        R::StrLit => Tk::StrLit,
        R::Directive => Tk::Directive,
        R::Ctrl => Tk::Ctrl,
        R::Eof => Tk::Eof,
        R::Error => panic!("the lexer never makes error tokens"),
        R::Kw => *KWS
            .iter()
            .find(|v| format!("{v:?}").to_uppercase().as_bytes() == t.text)
            .unwrap_or_else(|| panic!("unknown keyword text {:?}", t.text)),
        R::Sym => {
            SYMS.iter()
                .find(|(s, _)| s.as_bytes() == t.text)
                .unwrap_or_else(|| panic!("unknown symbol text {:?}", t.text))
                .1
        }
    }
}

pub fn raw_kind(src: &str) -> (tp5c::lex::Tk, Vec<u8>) {
    let t = one(src);
    (t.kind, t.text)
}
