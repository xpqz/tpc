//! Lexer: bytes to tokens in one left-to-right pass, no backtracking.
//!
//! Identifiers and keywords are folded to upper case. `{...}` and `(*...*)` comments are
//! skipped, except that a comment starting with `$` is returned as a `Directive` token for the
//! parser. `(.` and `.)` are alternative spellings of `[` and `]`. Scanning stops after the
//! program's final `END .`.
//!
//! The one ambiguity is `^`: after an operand (`p^`, `a[i]^`, `f(x)^`) it dereferences, anywhere
//! else `^M` is a control character, which may be part of a string (`'a'^M'b'`). The scanner
//! decides by looking at the previous token only.
use crate::diag::{code, Diag};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tk {
    Ident,
    IntLit,
    RealLit,
    StrLit,
    Directive,
    /// A lone control character `^X`; `text` holds the letter `X`.
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

const KEYWORDS: &str = "\
    AND ARRAY ASM BEGIN CASE CONST DIV DO DOWNTO ELSE END FILE FOR FUNCTION GOTO IF \
    IMPLEMENTATION IN INLINE INTERFACE LABEL MOD NIL NOT OF OR PACKED PROCEDURE PROGRAM RECORD \
    REPEAT SET SHL SHR STRING THEN TO TYPE UNIT UNTIL USES VAR WHILE WITH XOR ABSOLUTE EXTERNAL \
    FORWARD INTERRUPT";
const TWO_CHAR_SYMBOLS: &[&str] = &["<=", ">=", "<>", ":=", "..", "(.", ".)"];
const ONE_CHAR_SYMBOLS: &[u8] = b"+-*/=<>()[],.;:^@";

/// Digit values stop growing here, so an overlong literal cannot overflow while it is read.
const SATURATE_AT: i64 = 1 << 40;
const MAX_DECIMAL: i64 = 0x7FFF_FFFF;
const MAX_HEX: i64 = 0xFFFF_FFFF;

struct Scanner<'a> {
    src: &'a [u8],
    pos: usize,
    line: u32,
    /// Offset of the first byte of the current line, for column numbers.
    line_start: usize,
    /// Where the token being scanned began.
    token_line: u32,
    token_col: u32,
}

impl Scanner<'_> {
    /// The byte `ahead` places after the cursor, or 0 past the end.
    fn peek(&self, ahead: usize) -> u8 {
        *self.src.get(self.pos + ahead).unwrap_or(&0)
    }

    fn error(&self, code: u16) -> Diag {
        Diag {
            code,
            line: self.token_line,
            col: self.token_col,
        }
    }

    fn token(&self, kind: Tk, text: Vec<u8>) -> Tok {
        Tok {
            kind,
            line: self.token_line,
            col: self.token_col,
            text,
            int: 0,
            real: 0.0,
        }
    }

    fn mark_token_start(&mut self) {
        self.token_line = self.line;
        self.token_col = (self.pos - self.line_start + 1) as u32;
    }

    /// Records that the byte at `newline_at` ends a line.
    fn newline(&mut self, newline_at: usize) {
        self.line += 1;
        self.line_start = newline_at + 1;
    }

    /// Reads digits in `base`; the value saturates at `SATURATE_AT`. None when there is no
    /// digit.
    fn digits_value(&mut self, base: u32) -> Option<i64> {
        let mut value: Option<i64> = None;
        while let Some(digit) = (self.peek(0) as char).to_digit(base) {
            let next = value.unwrap_or(0) * base as i64 + digit as i64;
            value = Some(next.min(SATURATE_AT));
            self.pos += 1;
        }
        value
    }

    fn skip_decimal_digits(&mut self) {
        while self.peek(0).is_ascii_digit() {
            self.pos += 1;
        }
    }

    /// `.digits`; a `.` not followed by a digit is left alone (`1..5` is a range).
    fn skip_fraction(&mut self) -> bool {
        if self.peek(0) != b'.' || !self.peek(1).is_ascii_digit() {
            return false;
        }
        self.pos += 1;
        self.skip_decimal_digits();
        true
    }

    /// `E[+-]digits`; at least one digit is required.
    fn skip_exponent(&mut self) -> Result<bool, Diag> {
        if !self.peek(0).eq_ignore_ascii_case(&b'e') {
            return Ok(false);
        }
        self.pos += 1;
        if self.peek(0) == b'+' || self.peek(0) == b'-' {
            self.pos += 1;
        }
        let digits_start = self.pos;
        self.skip_decimal_digits();
        if self.pos == digits_start {
            return Err(self.error(code::REAL_CONSTANT_ERROR));
        }
        Ok(true)
    }

    /// An integer, hexadecimal (`$FF`) or real literal.
    fn number(&mut self) -> Result<Tok, Diag> {
        let start = self.pos;
        let is_hex = self.peek(0) == b'$';
        if is_hex {
            self.pos += 1;
            let value = self.digits_value(16);
            return self.integer_token(value, MAX_HEX);
        }
        let value = self.digits_value(10);
        let has_fraction = self.skip_fraction();
        let has_exponent = self.skip_exponent()?;
        if has_fraction || has_exponent {
            return self.real_token(start);
        }
        self.integer_token(value, MAX_DECIMAL)
    }

    fn integer_token(&self, value: Option<i64>, max: i64) -> Result<Tok, Diag> {
        let value = value.filter(|&v| v <= max);
        let mut token = self.token(Tk::IntLit, Vec::new());
        token.int = value.ok_or(self.error(code::INTEGER_CONSTANT_ERROR))?;
        Ok(token)
    }

    fn real_token(&self, start: usize) -> Result<Tok, Diag> {
        let text = std::str::from_utf8(&self.src[start..self.pos]).unwrap_or("");
        let value: f64 = text.parse().unwrap_or(f64::INFINITY);
        if !value.is_finite() {
            return Err(self.error(code::REAL_CONSTANT_ERROR));
        }
        let mut token = self.token(Tk::RealLit, Vec::new());
        token.real = value;
        Ok(token)
    }

    /// The control character `^X` at `ahead`: `^@`..`^_` are the codes 0..31. A `^` before an
    /// identifier character (`^Byte`) is a pointer type instead.
    fn control_char(&self, ahead: usize) -> Option<u8> {
        let letter = self.peek(ahead + 1).to_ascii_uppercase();
        let next = self.peek(ahead + 2);
        let next_is_identifier = next.is_ascii_alphanumeric() || next == b'_';
        let is_control =
            self.peek(ahead) == b'^' && (b'@'..=b'_').contains(&letter) && !next_is_identifier;
        is_control.then(|| letter - b'@')
    }

    /// Pieces of one string constant, concatenated: `'text'`, `#65`, `#$41` and `^M`.
    fn string(&mut self) -> Result<Tok, Diag> {
        let mut text = Vec::new();
        loop {
            match self.peek(0) {
                b'\'' => self.quoted_piece(&mut text)?,
                b'#' => self.char_code_piece(&mut text)?,
                b'^' if self.control_char(0).is_some() => {
                    text.extend(self.control_char(0));
                    self.pos += 2;
                }
                _ => return Ok(self.token(Tk::StrLit, text)),
            }
        }
    }

    /// A single control character that does not continue into a string is a `Ctrl` token.
    fn control_char_token_or_string(&mut self) -> Result<Tok, Diag> {
        let continues = matches!(self.peek(2), b'\'' | b'#') || self.control_char(2).is_some();
        if continues {
            return self.string();
        }
        let letter = self.peek(1);
        self.pos += 2;
        Ok(self.token(Tk::Ctrl, vec![letter]))
    }

    /// `'...'` with `''` for a quote; it may not run past the end of the line.
    fn quoted_piece(&mut self, text: &mut Vec<u8>) -> Result<(), Diag> {
        self.pos += 1;
        loop {
            let c = self.peek(0);
            if self.pos >= self.src.len() || c == b'\n' {
                return Err(self.error(code::STRING_EXCEEDS_LINE));
            }
            self.pos += 1;
            if c != b'\'' {
                text.push(c);
            } else if self.peek(0) == b'\'' {
                text.push(c);
                self.pos += 1;
            } else {
                return Ok(());
            }
        }
    }

    /// `#nn` or `#$hh`: one character by its code, 0..255.
    fn char_code_piece(&mut self, text: &mut Vec<u8>) -> Result<(), Diag> {
        self.pos += 1;
        let is_hex = self.peek(0) == b'$';
        if is_hex {
            self.pos += 1;
        }
        match self.digits_value(if is_hex { 16 } else { 10 }) {
            None => Err(self.error(code::INTEGER_CONSTANT_ERROR)),
            Some(v) if v > 255 => Err(self.error(code::CONSTANT_OUT_OF_RANGE)),
            Some(v) => {
                text.push(v as u8);
                Ok(())
            }
        }
    }

    /// Skips a comment that starts `open` bytes after the cursor and ends with `close`; a `$`
    /// body is returned as a directive token.
    fn comment(&mut self, open: usize, close: &[u8]) -> Result<Option<Tok>, Diag> {
        let body_start = self.pos + open;
        let mut end = body_start;
        while !self.src[end.min(self.src.len())..].starts_with(close) {
            if end >= self.src.len() {
                return Err(self.error(code::UNEXPECTED_END_OF_FILE));
            }
            if self.src[end] == b'\n' {
                self.newline(end);
            }
            end += 1;
        }
        self.pos = end + close.len();
        let is_directive = self.src.get(body_start) == Some(&b'$');
        Ok(is_directive.then(|| self.token(Tk::Directive, self.src[body_start + 1..end].to_vec())))
    }

    fn word(&mut self) -> Tok {
        let start = self.pos;
        while self.peek(0).is_ascii_alphanumeric() || self.peek(0) == b'_' {
            self.pos += 1;
        }
        let text = self.src[start..self.pos].to_ascii_uppercase();
        let is_keyword = KEYWORDS.split(' ').any(|k| k.as_bytes() == text);
        self.token(if is_keyword { Tk::Kw } else { Tk::Ident }, text)
    }

    fn symbol(&mut self) -> Result<Tok, Diag> {
        let two = self.src.get(self.pos..self.pos + 2).unwrap_or(&[]);
        let length = if TWO_CHAR_SYMBOLS.iter().any(|s| s.as_bytes() == two) {
            2
        } else if ONE_CHAR_SYMBOLS.contains(&self.peek(0)) {
            1
        } else {
            return Err(self.error(code::SYNTAX_ERROR));
        };
        let text: &[u8] = match two {
            b"(." => b"[",
            b".)" => b"]",
            _ => &self.src[self.pos..self.pos + length],
        };
        let token = self.token(Tk::Sym, text.to_vec());
        self.pos += length;
        Ok(token)
    }
}

/// True when `previous` ends an operand, so that a following `^` is a dereference.
fn ends_operand(previous: Option<&Tok>) -> bool {
    let Some(token) = previous else { return false };
    match token.kind {
        Tk::Ident => true,
        Tk::Sym => [&b")"[..], b"]", b"^"].contains(&&token.text[..]),
        _ => false,
    }
}

/// True when the last two tokens are the keyword `END` and a period.
fn ends_program(tokens: &[Tok]) -> bool {
    let [.., end, period] = tokens else {
        return false;
    };
    end.kind == Tk::Kw && end.text == b"END" && period.kind == Tk::Sym && period.text == b"."
}

pub fn lex(src: &[u8]) -> Result<Vec<Tok>, Diag> {
    let mut scanner = Scanner {
        src,
        pos: 0,
        line: 1,
        line_start: 0,
        token_line: 1,
        token_col: 1,
    };
    let mut tokens: Vec<Tok> = Vec::new();
    while scanner.pos < src.len() {
        let c = src[scanner.pos];
        if c == b'\n' {
            scanner.newline(scanner.pos);
        }
        if c <= b' ' {
            scanner.pos += 1;
            continue;
        }
        scanner.mark_token_start();
        let is_dereference = ends_operand(tokens.last());
        let token = match c {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => Some(scanner.word()),
            b'0'..=b'9' | b'$' => Some(scanner.number()?),
            b'\'' | b'#' => Some(scanner.string()?),
            b'^' if !is_dereference && scanner.control_char(0).is_some() => {
                Some(scanner.control_char_token_or_string()?)
            }
            b'{' => scanner.comment(1, b"}")?,
            b'(' if scanner.peek(1) == b'*' => scanner.comment(2, b"*)")?,
            _ => Some(scanner.symbol()?),
        };
        tokens.extend(token);
        if ends_program(&tokens) {
            break;
        }
    }
    scanner.mark_token_start();
    tokens.push(scanner.token(Tk::Eof, Vec::new()));
    Ok(tokens)
}
