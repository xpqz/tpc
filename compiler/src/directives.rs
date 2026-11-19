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
//! - `{$R+/-}` and `{$I+/-}` are state that flows with the text, also out of an include. `{$I}`
//!   is on by default. `{$A B D F L O S V}` and `{$M stack,heapmin,heapmax}` are checked and
//!   then ignored: there is no code generator to tune, no overlay, no 8087 and no DOS memory
//!   layout. `{$N+}` and `{$E+}` are error 200 (8087 code is omitted), their `-` forms change
//!   nothing; any other letter and a malformed directive are error 17, as in TP.
use crate::diag::{code, Diag};
use crate::lex::{is_ident_byte, is_ident_start, Lexer, Tk, Tok};
use crate::units::{file_number, find_file, SourceFile};

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
    /// The new `{$R}` and `{$I}` states, each `None` when the directive does not set it.
    Switches(Option<bool>, Option<bool>),
    /// A directive with no effect: `{$M}`, or a malformed directive in skipped text, where it is
    /// not an error.
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
    let unterminated_last_line = if text.ends_with(b"\n") { 0 } else { 1 };
    // An empty file still has a line 1.
    (newlines + unterminated_last_line).max(1)
}

/// A diagnostic of file number `file` whose text is `text`. System's own diagnostics stay at
/// line 0, as its tokens are.
pub fn locate_in_file(d: Diag, file: u32, text: &[u8]) -> Diag {
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
    fn observe(&mut self, token: &Tok) {
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
            // Outside a statement part `end` closes a record (or the unit's own `end`).
            (false, b"RECORD") => self.depth += 1,
            (false, b"END") => self.depth = self.depth.saturating_sub(1),
            (true, b"BEGIN" | b"CASE") => self.depth += 1,
            // The `end` that brings the depth to 0 closes the statement part.
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
    let is_identifier = word.first().is_some_and(|&c| is_ident_start(c));
    if !is_identifier {
        return Err(code::IDENTIFIER_EXPECTED);
    }
    let length = word
        .iter()
        .position(|&c| !is_ident_byte(c))
        .unwrap_or(word.len());
    Ok(word[..length].to_ascii_uppercase())
}

/// Switch letters TP accepts that change nothing here.
const IGNORED_SWITCHES: &[u8] = b"ABDFLOSV";

/// Switch letters TP accepts that need an 8087 or its emulator, which are omitted.
const NUMERIC_SWITCHES: &[u8] = b"NE";

/// A comma separated list of switches `R+`, `F-`, ... Anything else is error 17, and a
/// switch that needs the omitted 8087 (`N+`, `E+`) is error 200. The whole list is read first
/// because an invalid item is 17 wherever it stands: TP has no 8087 mode to leave out, so it
/// reads `E+` and goes on to the next item. `I` followed by a blank is an include, not a switch.
fn switches(text: &[u8]) -> Result<Directive, u16> {
    let mut range_checks = None;
    let mut io_checks = None;
    let mut needs_8087 = false;
    for item in text.split(|&c| c == b',') {
        let item = item.to_ascii_uppercase();
        match item.as_slice() {
            b"R+" | b"R-" => range_checks = Some(item[1] == b'+'),
            b"I+" | b"I-" => io_checks = Some(item[1] == b'+'),
            [letter, b'+' | b'-'] if IGNORED_SWITCHES.contains(letter) => {}
            [letter, b'-'] if NUMERIC_SWITCHES.contains(letter) => {}
            [letter, b'+'] if NUMERIC_SWITCHES.contains(letter) => needs_8087 = true,
            _ => return Err(code::INVALID_DIRECTIVE),
        }
    }
    if needs_8087 {
        return Err(code::NOT_SUPPORTED);
    }
    Ok(Directive::Switches(range_checks, io_checks))
}

/// The value of a decimal or `$` hexadecimal number at the start of `text`, after blanks.
/// None when there is no digit; a value too large for the limits below saturates.
fn memory_number(text: &[u8]) -> Option<u32> {
    let text = text.trim_ascii_start();
    let (base, digits) = match text.strip_prefix(b"$") {
        Some(hex) => (16, hex),
        None => (10, text),
    };
    let mut value: Option<u32> = None;
    for &c in digits {
        let Some(digit) = (c as char).to_digit(base) else {
            break;
        };
        value = Some(
            value
                .unwrap_or(0)
                .saturating_mul(base)
                .saturating_add(digit),
        );
    }
    value
}

/// `{$M stack,heapmin,heapmax}`: three numbers, the stack size in 1024 to 65520 and the heap
/// sizes in 0 to 655360 (the one megabyte DOS can address less its upper 384 KB). It is checked
/// and ignored: memory is the host's. Text after the third number is ignored, as in TP.
fn memory_sizes(rest: &[u8]) -> Result<Directive, u16> {
    const STACK: std::ops::RangeInclusive<u32> = 1024..=65520;
    const HEAP: std::ops::RangeInclusive<u32> = 0..=655360;
    let mut fields = rest.split(|&c| c == b',');
    let mut next_field = |limits: std::ops::RangeInclusive<u32>| {
        let value = memory_number(fields.next().unwrap_or_default());
        value
            .filter(|v| limits.contains(v))
            .ok_or(code::INVALID_DIRECTIVE)
    };
    next_field(STACK)?;
    next_field(HEAP)?;
    next_field(HEAP)?;
    Ok(Directive::Ignored)
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
        b"M" if rest.first().is_some_and(|c| c.is_ascii_whitespace()) => memory_sizes(rest),
        _ => switches(text),
    }
}

struct Expander<'a> {
    files: &'a [SourceFile],
    symbols: Vec<Vec<u8>>,
    range_checks: bool,
    io_checks: bool,
    statement_part: StatementPart,
    tokens: Vec<Tok>,
}

/// The tokens of the source file number `file` with text `text`, includes spliced in, ending
/// with `Eof`. `files` is the registered file table that includes are read from. The first
/// error of the text ends the stream with an `Error` token before the `Eof`: the parser reports
/// it only if it gets that far, as TP, which scans one token at a time, does.
pub fn expand(file: u32, text: &[u8], files: &[SourceFile]) -> Vec<Tok> {
    let mut expander = Expander {
        files,
        symbols: PREDEFINED_SYMBOLS.iter().map(|s| s.to_vec()).collect(),
        range_checks: false,
        io_checks: true,
        statement_part: StatementPart::default(),
        tokens: Vec::new(),
    };
    match expander.read(file, text, 0) {
        Ok(end) => expander.push(end, file),
        Err(d) => {
            let error = Tok::error(d);
            let eof = Tok {
                kind: Tk::Eof,
                ..error.clone()
            };
            expander.tokens.push(error);
            expander.tokens.push(eof);
        }
    }
    expander.tokens
}

impl Expander<'_> {
    fn push(&mut self, mut token: Tok, file: u32) {
        token.file = file;
        token.range_checks = self.range_checks;
        token.io_checks = self.io_checks;
        self.statement_part.observe(&token);
        self.tokens.push(token);
    }

    /// Reads one file at include depth `depth`, adding its tokens; returns its `Eof` token.
    fn read(&mut self, file: u32, text: &[u8], depth: u32) -> Result<Tok, Diag> {
        let mut lexer = Lexer::new(text);
        let mut frames: Vec<Frame> = Vec::new();
        loop {
            let skipping = is_skipping(&frames);
            let token = if skipping {
                lexer.next_skipped()
            } else {
                lexer.next_token()
            };
            let token = token.map_err(|d| locate_in_file(d, file, text))?;
            match token.kind {
                Tk::Directive => {
                    let position = Diag {
                        line: token.line,
                        col: token.col,
                        file,
                        code: 0,
                    };
                    self.directive(&token.text, position, depth, &mut frames)?
                }
                Tk::Eof => {
                    let position = Diag {
                        line: last_line(text),
                        col: 1,
                        file,
                        code: 0,
                    };
                    return self.end_of_file(token, position, depth, &frames);
                }
                _ => self.push(token, file),
            }
        }
    }

    /// The text of a file ended: a conditional must have closed in the file that opened it, and
    /// an include may not end inside a statement part. A program or unit stops at its final
    /// `end.`, so a conditional still open there is fine unless the text ran out while skipping.
    fn end_of_file(
        &self,
        eof: Tok,
        position: Diag,
        depth: u32,
        frames: &[Frame],
    ) -> Result<Tok, Diag> {
        let skipping = is_skipping(frames);
        if !frames.is_empty() && (skipping || depth > 0) {
            return Err(position.with_code(code::ENDIF_MISSING));
        }
        if depth > 0 && self.statement_part.inside {
            return Err(position.with_code(code::UNEXPECTED_END_OF_FILE));
        }
        Ok(eof)
    }

    /// Acts on one directive. `position` is a diagnostic with code 0 that holds the directive's
    /// line, column and file; an error adds its code. In skipped text only conditional
    /// directives count; a malformed directive there is ignored.
    fn directive(
        &mut self,
        text: &[u8],
        position: Diag,
        depth: u32,
        frames: &mut Vec<Frame>,
    ) -> Result<(), Diag> {
        let skipping = is_skipping(frames);
        let directive = match parse_directive(text) {
            Ok(directive) => directive,
            Err(_) if skipping => Directive::Ignored,
            Err(code) => return Err(position.with_code(code)),
        };
        // Conditional directives act even in skipped text; every other directive is ignored
        // while skipping, which the `_ if skipping` arm does.
        match directive {
            Directive::IfDef(name) => open_conditional(frames, self.symbols.contains(&name)),
            Directive::IfNotDef(name) => open_conditional(frames, !self.symbols.contains(&name)),
            Directive::Else => match frames.last_mut() {
                Some(frame) => frame.active = frame.enclosing_active && !frame.active,
                None => return Err(position.with_code(code::MISPLACED_CONDITIONAL)),
            },
            Directive::EndIf => {
                if frames.pop().is_none() {
                    return Err(position.with_code(code::MISPLACED_CONDITIONAL));
                }
            }
            _ if skipping => {}
            Directive::Define(name) => {
                if !self.symbols.contains(&name) {
                    self.symbols.push(name);
                }
            }
            Directive::Undef(name) => self.symbols.retain(|s| *s != name),
            Directive::Switches(range, io) => {
                self.range_checks = range.unwrap_or(self.range_checks);
                self.io_checks = io.unwrap_or(self.io_checks);
            }
            Directive::Include(name) => self.include(&name, position, depth)?,
            Directive::Ignored => {}
        }
        Ok(())
    }

    /// `{$I name}`: the file `name`, or `name.PAS` when it has no extension, spliced in here.
    fn include(&mut self, name: &[u8], position: Diag, depth: u32) -> Result<(), Diag> {
        if self.statement_part.inside {
            return Err(position.with_code(code::INCLUDE_NOT_ALLOWED));
        }
        if depth >= MAX_INCLUDE_DEPTH {
            return Err(position.with_code(code::TOO_MANY_NESTED_FILES));
        }
        let mut wanted = name.to_vec();
        if !name.contains(&b'.') {
            wanted.extend_from_slice(b".PAS");
        }
        let Some(position) = find_file(self.files, &wanted) else {
            return Err(position.with_code(missing_include_code(&wanted)));
        };
        let file = file_number(position);
        let text = &self.files[position].text;
        self.read(file, text, depth + 1)?;
        Ok(())
    }
}

/// TP reports a name whose directory does not exist as an invalid file name (14) and a missing
/// file in an existing directory as 15. The file table has no directories, so a name that
/// has a directory part is taken to name a directory that is not there.
fn missing_include_code(name: &[u8]) -> u16 {
    let has_directory = name.iter().any(|c| matches!(c, b'\\' | b'/' | b':'));
    match has_directory {
        true => code::INVALID_FILENAME,
        false => code::FILE_NOT_FOUND,
    }
}

/// True while the innermost open conditional is in a branch that is not taken.
fn is_skipping(frames: &[Frame]) -> bool {
    frames.last().is_some_and(|f| !f.active)
}

/// Opens a conditional whose condition is `taken`; inside skipped text it is never taken.
fn open_conditional(frames: &mut Vec<Frame>, taken: bool) {
    let enclosing_active = frames.last().is_none_or(|f| f.active);
    frames.push(Frame {
        active: enclosing_active && taken,
        enclosing_active,
    });
}
