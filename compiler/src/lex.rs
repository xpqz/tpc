#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tk {
    Ident,
    IntLit,
    RealLit,
    StrLit,
    Directive,
    Kw,
    Sym,
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tok {
    pub kind: Tk,
    pub line: u32,
    pub col: u32,
    pub text: Vec<u8>,
    pub int: i64,
    pub real: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LexErr {
    pub num: u16,
    pub line: u32,
    pub col: u32,
}

pub fn lex(_src: &[u8]) -> Result<Vec<Tok>, LexErr> {
    Ok(Vec::new())
}
