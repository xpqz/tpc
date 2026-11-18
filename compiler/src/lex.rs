#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tk {
    Ident,
    IntLit,
    RealLit,
    StrLit,
    Directive,
    Ctrl,
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

const KEYWORDS: &[&str] = &[
    "AND",
    "ARRAY",
    "ASM",
    "BEGIN",
    "CASE",
    "CONST",
    "DIV",
    "DO",
    "DOWNTO",
    "ELSE",
    "END",
    "FILE",
    "FOR",
    "FUNCTION",
    "GOTO",
    "IF",
    "IMPLEMENTATION",
    "IN",
    "INLINE",
    "INTERFACE",
    "LABEL",
    "MOD",
    "NIL",
    "NOT",
    "OF",
    "OR",
    "PACKED",
    "PROCEDURE",
    "PROGRAM",
    "RECORD",
    "REPEAT",
    "SET",
    "SHL",
    "SHR",
    "STRING",
    "THEN",
    "TO",
    "TYPE",
    "UNIT",
    "UNTIL",
    "USES",
    "VAR",
    "WHILE",
    "WITH",
    "XOR",
    "ABSOLUTE",
    "EXTERNAL",
    "FORWARD",
    "INTERRUPT",
];
const SYMS2: &[&str] = &["<=", ">=", "<>", ":=", "..", "(.", ".)"];
const SYMS1: &[u8] = b"+-*/=<>()[],.;:^@";

type R<T> = Result<T, LexErr>;

struct Lx<'a> {
    s: &'a [u8],
    i: usize,
    line: u32,
    ls: usize,
    tl: u32,
    tc: u32,
}

impl Lx<'_> {
    fn at(&self, k: usize) -> u8 {
        *self.s.get(self.i + k).unwrap_or(&0)
    }

    fn err(&self, num: u16) -> LexErr {
        LexErr {
            num,
            line: self.tl,
            col: self.tc,
        }
    }

    fn tok(&self, kind: Tk, text: Vec<u8>) -> Tok {
        Tok {
            kind,
            line: self.tl,
            col: self.tc,
            text,
            int: 0,
            real: 0.0,
        }
    }

    fn newline(&mut self, at: usize) {
        self.line += 1;
        self.ls = at + 1;
    }

    /// Reads digits in `base`; the value saturates at 2^40. None when there is no digit.
    fn num(&mut self, base: u32) -> Option<i64> {
        let mut v: Option<i64> = None;
        while let Some(d) = (self.at(0) as char).to_digit(base) {
            v = Some((v.unwrap_or(0) * base as i64 + d as i64).min(1 << 40));
            self.i += 1;
        }
        v
    }

    fn digits(&mut self) {
        while self.at(0).is_ascii_digit() {
            self.i += 1;
        }
    }

    fn number(&mut self) -> R<Tok> {
        let st = self.i;
        let hex = self.at(0) == b'$';
        self.i += hex as usize;
        let v = self.num(if hex { 16 } else { 10 });
        let (limit, v) = (
            if hex { 0xFFFF_FFFF } else { 0x7FFF_FFFF },
            v.ok_or(self.err(7)),
        );
        let mut real = false;
        if !hex {
            if self.at(0) == b'.' && self.at(1).is_ascii_digit() {
                self.i += 1;
                self.digits();
                real = true;
            }
            if self.at(0) | 32 == b'e' {
                self.i += 1;
                self.i += (self.at(0) == b'+' || self.at(0) == b'-') as usize;
                let cur = self.i;
                self.digits();
                if self.i == cur {
                    return Err(self.err(6));
                }
                real = true;
            }
        }
        let mut t = self.tok(Tk::IntLit, Vec::new());
        if real {
            let text = std::str::from_utf8(&self.s[st..self.i]).unwrap_or("");
            t.kind = Tk::RealLit;
            t.real = text.parse().unwrap_or(f64::INFINITY);
            if !t.real.is_finite() {
                return Err(self.err(6));
            }
        } else {
            t.int = v.ok().filter(|&v| v <= limit).ok_or(self.err(7))?;
        }
        Ok(t)
    }

    /// The control character `^X` at the cursor; `^` before an identifier character is a pointer.
    fn control(&self) -> Option<u8> {
        let c = self.at(1).to_ascii_uppercase();
        let ident = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
        (self.at(0) == b'^' && (64..96).contains(&c) && !ident(self.at(2))).then_some(c ^ 64)
    }

    fn string(&mut self) -> R<Tok> {
        let mut t = Vec::new();
        loop {
            match self.at(0) {
                b'\'' => {
                    self.i += 1;
                    loop {
                        let c = self.at(0);
                        if self.i >= self.s.len() || c == b'\n' {
                            return Err(self.err(8));
                        }
                        self.i += 1;
                        if c != b'\'' {
                            t.push(c);
                        } else if self.at(0) == b'\'' {
                            t.push(c);
                            self.i += 1;
                        } else {
                            break;
                        }
                    }
                }
                b'#' => {
                    self.i += 1;
                    let hex = self.at(0) == b'$';
                    self.i += hex as usize;
                    match self.num(if hex { 16 } else { 10 }) {
                        None => return Err(self.err(7)),
                        Some(v) if v > 255 => return Err(self.err(76)),
                        Some(v) => t.push(v as u8),
                    }
                }
                b'^' if self.control().is_some() => {
                    t.extend(self.control());
                    self.i += 2;
                }
                _ => return Ok(self.tok(Tk::StrLit, t)),
            }
        }
    }

    /// Skips a comment closed by `close`; a `$` body is returned as a directive token.
    fn comment(&mut self, open: usize, close: &[u8]) -> R<Option<Tok>> {
        let start = self.i + open;
        let mut j = start;
        while !self.s[j.min(self.s.len())..].starts_with(close) {
            if j >= self.s.len() {
                return Err(self.err(10));
            }
            if self.s[j] == b'\n' {
                self.newline(j);
            }
            j += 1;
        }
        self.i = j + close.len();
        let dir = self.s.get(start) == Some(&b'$');
        Ok(dir.then(|| self.tok(Tk::Directive, self.s[start + 1..j].to_vec())))
    }

    fn word(&mut self) -> Tok {
        let st = self.i;
        while self.at(0).is_ascii_alphanumeric() || self.at(0) == b'_' {
            self.i += 1;
        }
        let text = self.s[st..self.i].to_ascii_uppercase();
        let kw = KEYWORDS.iter().any(|k| k.as_bytes() == text);
        self.tok(if kw { Tk::Kw } else { Tk::Ident }, text)
    }

    fn symbol(&mut self) -> R<Tok> {
        let two = self.s.get(self.i..self.i + 2).unwrap_or(&[]);
        let n = match two {
            _ if SYMS2.iter().any(|s| s.as_bytes() == two) => 2,
            _ if SYMS1.contains(&self.at(0)) => 1,
            _ => return Err(self.err(5)),
        };
        let text = match two {
            b"(." => b"[",
            b".)" => b"]",
            _ => &self.s[self.i..self.i + n],
        };
        let t = self.tok(Tk::Sym, text.to_vec());
        self.i += n;
        Ok(t)
    }
}

pub fn lex(src: &[u8]) -> Result<Vec<Tok>, LexErr> {
    let mut lx = Lx {
        s: src,
        i: 0,
        line: 1,
        ls: 0,
        tl: 1,
        tc: 1,
    };
    let mut out: Vec<Tok> = Vec::new();
    while lx.i < src.len() {
        let c = src[lx.i];
        if c == b'\n' {
            lx.newline(lx.i);
        }
        if c <= b' ' {
            lx.i += 1;
            continue;
        }
        (lx.tl, lx.tc) = (lx.line, (lx.i - lx.ls + 1) as u32);
        let deref = out.last().is_some_and(|t| {
            t.kind == Tk::Ident
                || (t.kind == Tk::Sym && [&b")"[..], b"]", b"^"].contains(&&t.text[..]))
        });
        let tok = match c {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => Some(lx.word()),
            b'0'..=b'9' | b'$' => Some(lx.number()?),
            b'\'' | b'#' => Some(lx.string()?),
            b'^' if !deref && lx.control().is_some() => Some(lx.string()?),
            b'{' => lx.comment(1, b"}")?,
            b'(' if lx.at(1) == b'*' => lx.comment(2, b"*)")?,
            _ => Some(lx.symbol()?),
        };
        out.extend(tok);
    }
    (lx.tl, lx.tc) = (lx.line, (lx.i - lx.ls + 1) as u32);
    out.push(lx.tok(Tk::Eof, Vec::new()));
    Ok(out)
}
