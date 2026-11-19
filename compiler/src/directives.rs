//! Directives: the front end between lexing and parsing. It turns the token stream of one source
//! file into the stream the parser reads: conditional compilation applied, include files spliced
//! in, `{$R}` resolved, and no directive tokens left.
//!
//! Every token leaves with the number of the file it came from and the `{$R+}` state in force
//! at it, so the parser and later stages never see where a token was read.
//!
//! - Conditionals: a stack of frames, one per open `$IFDEF` or `$IFNDEF`. While the top frame is
//!   inactive the lexer runs in skipping mode, where only strings, comments and conditional
//!   directives are recognised (Turbo Pascal does the same, so skipped text may be invalid
//!   Pascal). A conditional must close in the file that opened it.
//! - Symbols are local to a source file: an include shares the symbols of the file that
//!   includes it, and each unit or program starts with the predefined ones.
//! - Includes are read from the registered file table. TP allows them in declaration parts only,
//!   so the expander follows the nesting of `begin`/`end` to know when it is inside a
//!   statement part (error 118).
//! - `{$R+}` and `{$R-}` are state that flows with the text, also out of an include. Other
//!   switches are not implemented (error 200); a malformed directive is error 17.
use crate::diag::{code, Diag};
use crate::lex::{Lexer, Tk, Tok};
use crate::units::{find_file, SourceFile};

/// Includes nest this deep, counting the include itself but not the including program or unit.
const MAX_INCLUDE_DEPTH: u32 = 14;

const PREDEFINED_SYMBOLS: [&[u8]; 4] = [b"VER50", b"MSDOS", b"CPU86", b"CPU87"];

/// What one directive asks for. Symbols are upper case.
enum Directive {
    Include(Vec<u8>),
    Define(Vec<u8>),
    Undef(Vec<u8>),
    IfDef(Vec<u8>),
    IfNotDef(Vec<u8>),
    Else,
    EndIf,
    /// `{$R+}`, `{$R-}` and the switches that are accepted and ignored.
    RangeChecks(bool),
    Ignored,
}

/// One open conditional.
struct Frame {
    /// The text now being read belongs to a taken branch.
    active: bool,
    /// The text around the conditional is being read; inside skipped text nothing is taken.
    enclosing_active: bool,
}

/// TP reports end of file on the last line of the file, not after its final newline.
fn last_line(text: &[u8]) -> u32 {
    let newlines = text.iter().filter(|&&c| c == b'\n').count() as u32;
    let unterminated = !text.ends_with(b"\n") as u32;
    (newlines + unterminated).max(1)
}

/// A diagnostic of file number `file` whose text is `text`. System's own diagnostics stay at
/// line 0, as its tokens are.
pub fn in_file(d: Diag, file: u32, text: &[u8]) -> Diag {
    let end_of_file = d.code == code::UNEXPECTED_END_OF_FILE && d.line != 0;
    let line = if end_of_file { last_line(text) } else { d.line };
    Diag { line, file, ..d }
}

/// Follows the `begin`/`end` nesting of the token stream to tell statement parts, where include
/// files are not allowed, from declaration parts.
#[derive(Default)]
struct StatementPart {
    /// Open `begin`/`case` (inside a statement part) or `record` (outside) blocks.
    depth: u32,
    inside: bool,
    /// The final `end` has been seen; the statement part ends after the next token (`;` or `.`).
    closing: bool,
}

impl StatementPart {
    fn after(&mut self, token: &Tok) {
        if self.closing {
            self.inside = false;
            self.closing = false;
        }
        if token.kind != Tk::Kw {
            return;
        }
        match (self.inside, token.text.as_slice()) {
            (false, b"BEGIN") => {
                self.inside = true;
                self.depth = 1;
            }
            (false, b"RECORD") => self.depth += 1,
            (false, b"END") => self.depth = self.depth.saturating_sub(1),
            (true, b"BEGIN" | b"CASE") => self.depth += 1,
            (true, b"END") => {
                self.depth -= 1;
                self.closing = self.depth == 0;
            }
            _ => {}
        }
    }
}

/// The first blank separated word of `text`.
fn first_word(text: &[u8]) -> &[u8] {
    let start = text.iter().position(|c| !c.is_ascii_whitespace());
    let rest = &text[start.unwrap_or(text.len())..];
    let end = rest.iter().position(|c| c.is_ascii_whitespace());
    &rest[..end.unwrap_or(rest.len())]
}

/// The conditional symbol after a directive word: an identifier, error 2 otherwise. Text after
/// the identifier is ignored.
fn symbol(text: &[u8]) -> Result<Vec<u8>, u16> {
    let word = first_word(text);
    let is_identifier = word
        .first()
        .is_some_and(|c| c.is_ascii_alphabetic() || *c == b'_');
    if !is_identifier {
        return Err(code::IDENTIFIER_EXPECTED);
    }
    let length = word
        .iter()
        .position(|c| !(c.is_ascii_alphanumeric() || *c == b'_'))
        .unwrap_or(word.len());
    Ok(word[..length].to_ascii_uppercase())
}

/// A comma separated list of switches `R+`, `F-`, ... Anything else is error 17, and a
/// well-formed switch this compiler does not know is error 200. `I+` and `I-` are such a
/// switch; `I` followed by a blank is an include.
fn switches(text: &[u8]) -> Result<Directive, u16> {
    let mut range_checks = None;
    for item in text.split(|&c| c == b',') {
        match item.to_ascii_uppercase().as_slice() {
            b"F+" | b"F-" => {}
            b"R+" | b"R-" => range_checks = Some(item[1] == b'+'),
            [letter, b'+' | b'-'] if letter.is_ascii_alphabetic() => {
                return Err(code::NOT_SUPPORTED)
            }
            _ => return Err(code::INVALID_DIRECTIVE),
        }
    }
    Ok(range_checks.map_or(Directive::Ignored, Directive::RangeChecks))
}

/// Reads the directive whose text is `text` (the comment body after the `$`).
fn parse_directive(text: &[u8]) -> Result<Directive, u16> {
    let word_length = text.iter().take_while(|c| c.is_ascii_alphabetic()).count();
    let (word, rest) = text.split_at(word_length);
    match word.to_ascii_uppercase().as_slice() {
        b"IFDEF" => Ok(Directive::IfDef(symbol(rest)?)),
        b"IFNDEF" => Ok(Directive::IfNotDef(symbol(rest)?)),
        b"DEFINE" => Ok(Directive::Define(symbol(rest)?)),
        b"UNDEF" => Ok(Directive::Undef(symbol(rest)?)),
        b"ELSE" => Ok(Directive::Else),
        b"ENDIF" => Ok(Directive::EndIf),
        b"I" if rest.first().is_none_or(|c| c.is_ascii_whitespace()) => {
            Ok(Directive::Include(first_word(rest).to_vec()))
        }
        _ => switches(text),
    }
}

struct Expander<'a> {
    files: &'a [SourceFile],
    symbols: Vec<Vec<u8>>,
    range_checks: bool,
    statement_part: StatementPart,
    tokens: Vec<Tok>,
}

/// The tokens of the source file number `file` with text `text`, includes spliced in, ending
/// with `Eof`. `files` is the registered file table that includes are read from.
pub fn expand(file: u32, text: &[u8], files: &[SourceFile]) -> Result<Vec<Tok>, Diag> {
    let mut expander = Expander {
        files,
        symbols: PREDEFINED_SYMBOLS.iter().map(|s| s.to_vec()).collect(),
        range_checks: false,
        statement_part: StatementPart::default(),
        tokens: Vec::new(),
    };
    let end = expander.read(file, text, 0)?;
    expander.push(end, file);
    Ok(expander.tokens)
}

impl Expander<'_> {
    fn push(&mut self, mut token: Tok, file: u32) {
        token.file = file;
        token.range_checks = self.range_checks;
        self.statement_part.after(&token);
        self.tokens.push(token);
    }

    /// Reads one file at include depth `depth`, adding its tokens; returns its `Eof` token.
    fn read(&mut self, file: u32, text: &[u8], depth: u32) -> Result<Tok, Diag> {
        let mut lexer = Lexer::new(text);
        let mut frames: Vec<Frame> = Vec::new();
        loop {
            let skipping = frames.last().is_some_and(|f| !f.active);
            let token = if skipping {
                lexer.next_skipped()
            } else {
                lexer.next_token().map_err(|d| in_file(d, file, text))?
            };
            match token.kind {
                Tk::Directive => {
                    let at = Diag {
                        line: token.line,
                        col: token.col,
                        file,
                        code: 0,
                    };
                    self.directive(&token.text, at, depth, &mut frames)?
                }
                Tk::Eof => {
                    let at = Diag {
                        line: last_line(text),
                        col: 1,
                        file,
                        code: 0,
                    };
                    return self.end_of_file(token, at, depth, &frames);
                }
                _ => self.push(token, file),
            }
        }
    }

    /// The text of a file ended: a conditional must have closed in the file that opened it, and
    /// an include may not end inside a statement part. A program or unit stops at its final
    /// `end.`, so a conditional still open there is fine unless the text ran out while skipping.
    fn end_of_file(&self, eof: Tok, at: Diag, depth: u32, frames: &[Frame]) -> Result<Tok, Diag> {
        let skipping = frames.last().is_some_and(|f| !f.active);
        if !frames.is_empty() && (skipping || depth > 0) {
            return Err(Diag {
                code: code::ENDIF_MISSING,
                ..at
            });
        }
        if depth > 0 && self.statement_part.inside {
            return Err(Diag {
                code: code::UNEXPECTED_END_OF_FILE,
                ..at
            });
        }
        Ok(eof)
    }

    /// Acts on one directive. `at` is its position and file. In skipped text only conditional
    /// directives count; a malformed directive there is ignored.
    fn directive(
        &mut self,
        text: &[u8],
        at: Diag,
        depth: u32,
        frames: &mut Vec<Frame>,
    ) -> Result<(), Diag> {
        let skipping = frames.last().is_some_and(|f| !f.active);
        let directive = match parse_directive(text) {
            Ok(directive) => directive,
            Err(_) if skipping => Directive::Ignored,
            Err(code) => return Err(Diag { code, ..at }),
        };
        match directive {
            Directive::IfDef(name) => open_conditional(frames, self.symbols.contains(&name)),
            Directive::IfNotDef(name) => open_conditional(frames, !self.symbols.contains(&name)),
            Directive::Else => match frames.last_mut() {
                Some(frame) => frame.active = frame.enclosing_active && !frame.active,
                None => {
                    return Err(Diag {
                        code: code::MISPLACED_CONDITIONAL,
                        ..at
                    })
                }
            },
            Directive::EndIf => {
                if frames.pop().is_none() {
                    return Err(Diag {
                        code: code::MISPLACED_CONDITIONAL,
                        ..at
                    });
                }
            }
            _ if skipping => {}
            Directive::Define(name) => {
                if !self.symbols.contains(&name) {
                    self.symbols.push(name);
                }
            }
            Directive::Undef(name) => self.symbols.retain(|s| *s != name),
            Directive::RangeChecks(on) => self.range_checks = on,
            Directive::Include(name) => self.include(&name, at, depth)?,
            Directive::Ignored => {}
        }
        Ok(())
    }

    /// `{$I name}`: the file `name`, or `name.PAS` when it has no extension, spliced in here.
    fn include(&mut self, name: &[u8], at: Diag, depth: u32) -> Result<(), Diag> {
        if self.statement_part.inside {
            return Err(Diag {
                code: code::INCLUDE_NOT_ALLOWED,
                ..at
            });
        }
        if depth >= MAX_INCLUDE_DEPTH {
            return Err(Diag {
                code: code::TOO_MANY_NESTED_FILES,
                ..at
            });
        }
        let mut wanted = name.to_vec();
        if !name.contains(&b'.') {
            wanted.extend_from_slice(b".PAS");
        }
        let Some(position) = find_file(self.files, &wanted) else {
            return Err(Diag {
                code: code::FILE_NOT_FOUND,
                ..at
            });
        };
        // The file table numbers files from 1; 0 is the main program.
        let file = position as u32 + 1;
        let text = &self.files[position].text;
        self.read(file, text, depth + 1)?;
        Ok(())
    }
}

/// Opens a conditional whose condition is `taken`; inside skipped text it is never taken.
fn open_conditional(frames: &mut Vec<Frame>, taken: bool) {
    let enclosing_active = frames.last().is_none_or(|f| f.active);
    frames.push(Frame {
        active: enclosing_active && taken,
        enclosing_active,
    });
}
