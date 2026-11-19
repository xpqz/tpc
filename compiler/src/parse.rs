//! Recursive-descent parser: `[program header;] declarations begin statements end .`
//!
//! One method per grammar rule, reading tokens through the lookahead `self.tok()`. Operator
//! precedence is the chain `expr` (relations), `simple` (add, or), `term` (multiply, and),
//! `factor`. Every node is stamped with the position where Turbo Pascal reports its errors,
//! which is not always where the node starts (see `ast::Kind`): operators, assignments and
//! `:` are stamped with the token after their last operand. A node stamped with a token not
//! yet read is reserved first (`reserve_node`) and filled in (`fill_node`) when its children
//! are parsed.
//!
//! One parser run reads one source file: System's declaration part, a unit, or the program.
//! All runs add to the same `Ast`, so names and nodes are shared between files.
//!
//! A unit name (System, the unit itself, each name in a `uses` clause) followed by `.` and an
//! identifier is a qualified name. The parser replaces the pair by one interned name `U.X`
//! and records it in `Ast::qualified`; the symbol table resolves it in the unit's exports.
use crate::ast::{Ast, FollowLine, Id, Kind, List, Op, Qualified, NO_NAME};
use crate::diag::{code, Diag};
use crate::lex::{Tk, Tok};

/// Keywords that may follow an empty statement in the enclosing construct.
#[derive(Clone, Copy, Default)]
struct StopWords {
    end: bool,
    until: bool,
    else_: bool,
}

impl StopWords {
    const END: StopWords = StopWords {
        end: true,
        until: false,
        else_: false,
    };
    const UNTIL: StopWords = StopWords {
        end: false,
        until: true,
        else_: false,
    };

    fn with_else(self) -> StopWords {
        StopWords {
            else_: true,
            ..self
        }
    }

    fn contains(self, keyword: &[u8]) -> bool {
        match keyword {
            b"END" => self.end,
            b"UNTIL" => self.until,
            b"ELSE" => self.else_,
            _ => false,
        }
    }
}

/// An identifier with the position of its first character.
struct IdentAt {
    name: u32,
    line: u32,
    col: u32,
}

struct Parser<'a> {
    tokens: Vec<Tok>,
    pos: usize,
    ast: &'a mut Ast,
    stops: StopWords,
    /// Parsing an embedded runtime source, the only place `external` is allowed.
    in_runtime: bool,
    /// Parsing a unit's interface part: a routine heading stands alone, as a forward.
    in_interface: bool,
    /// Names of the units this file may qualify names with: System, the unit itself, and
    /// every unit named in a `uses` clause so far. A unit name is declared once per file.
    unit_names: Vec<u32>,
}

type R<T> = Result<T, Diag>;

impl Parser<'_> {
    /// `[program Name [(files)];]`; returns the name, `NO_NAME` when there is no header.
    fn program_header(&mut self) -> R<u32> {
        if !self.kw(b"PROGRAM") {
            return Ok(NO_NAME);
        }
        let name = self.ident()?;
        if self.sym(b"(") {
            self.identifier_list()?;
            self.expect_sym(b")", code::RIGHT_PAREN_EXPECTED)?;
        }
        self.expect_sym(b";", code::SEMICOLON_EXPECTED)?;
        Ok(name)
    }

    /// `[uses A, B;]`: the unit names as `Name` nodes. A unit name may be declared only once
    /// in a file, so a repeated one, `System` or the unit's own name is error 4.
    fn uses_clause(&mut self) -> R<List> {
        let mut entries = Vec::new();
        if self.kw(b"USES") {
            loop {
                let (line, col) = self.pos();
                // Built before the name is read, so it is positioned at the name.
                let duplicate = self.err(code::DUPLICATE_IDENTIFIER);
                let name = self.ident()?;
                if self.unit_names.contains(&name) {
                    return Err(duplicate);
                }
                self.unit_names.push(name);
                entries.push(self.ast.add(Kind::Name(name), line, col));
                if !self.sym(b",") {
                    break;
                }
            }
            self.expect_sym(b";", code::SEMICOLON_EXPECTED)?;
        }
        Ok(self.ast.list(&entries))
    }

    /// `unit Name; interface [uses] declarations implementation [uses] declarations
    /// [begin statements] end .` A file that starts with `program` is not a unit.
    fn unit(&mut self) -> R<Id> {
        self.expect_kw(b"UNIT", code::UNIT_EXPECTED)?;
        let name = self.ident()?;
        self.unit_names.push(name);
        self.expect_sym(b";", code::SEMICOLON_EXPECTED)?;
        self.expect_kw(b"INTERFACE", code::INTERFACE_EXPECTED)?;
        let interface_uses = self.uses_clause()?;
        self.in_interface = true;
        let interface = self.declarations()?;
        self.in_interface = false;
        self.expect_kw(b"IMPLEMENTATION", code::IMPLEMENTATION_EXPECTED)?;
        let implementation_uses = self.uses_clause()?;
        let implementation = self.declarations()?;
        let (line, col) = self.pos();
        let body = if self.kw(b"BEGIN") {
            self.compound()?
        } else {
            self.expect_kw(b"END", code::END_EXPECTED)?;
            let nothing = self.ast.list(&[]);
            self.ast.add(Kind::Compound(nothing), line, col)
        };
        self.expect_sym(b".", code::PERIOD_EXPECTED)?;
        let unit = Kind::Unit {
            name,
            interface_uses,
            interface: self.ast.list(&interface),
            implementation_uses,
            implementation: self.ast.list(&implementation),
            body,
        };
        Ok(self.ast.add(unit, line, col))
    }
}

/// A parser over `toks` that adds to `ast`.
fn parser(ast: &mut Ast, tokens: Vec<Tok>) -> Parser<'_> {
    let system = ast.intern(b"SYSTEM");
    ast.range_checks = false;
    ast.io_checks = true;
    Parser {
        tokens,
        pos: 0,
        ast,
        stops: StopWords::default(),
        in_runtime: false,
        in_interface: false,
        unit_names: vec![system],
    }
}

/// System's declaration part, as a unit whose declarations are all in its interface.
pub fn parse_system(ast: &mut Ast, toks: Vec<Tok>) -> R<Id> {
    let mut parser = parser(ast, toks);
    parser.in_runtime = true;
    let declarations = parser.declarations()?;
    if parser.tok().kind != Tk::Eof {
        return Err(parser.err(code::BEGIN_EXPECTED));
    }
    let nothing = parser.ast.list(&[]);
    let body = parser.ast.add(Kind::Compound(nothing), 0, 0);
    let unit = Kind::Unit {
        name: parser.unit_names[0],
        interface_uses: nothing,
        interface: parser.ast.list(&declarations),
        implementation_uses: nothing,
        implementation: nothing,
        body,
    };
    Ok(parser.ast.add(unit, 0, 0))
}

/// The unit in `toks`.
pub fn parse_unit(ast: &mut Ast, toks: Vec<Tok>) -> R<Id> {
    parser(ast, toks).unit()
}

/// A unit of the embedded runtime: like `parse_unit`, but it may declare `external` routines.
pub fn parse_runtime_unit(ast: &mut Ast, toks: Vec<Tok>) -> R<Id> {
    let mut parser = parser(ast, toks);
    parser.in_runtime = true;
    parser.unit()
}

/// The program in `toks`.
pub fn parse_program(ast: &mut Ast, toks: Vec<Tok>) -> R<Id> {
    let mut parser = parser(ast, toks);
    let name = parser.program_header()?;
    let uses = parser.uses_clause()?;
    let declarations = parser.declarations()?;
    let (line, col) = parser.pos();
    let body = parser.begin()?;
    parser.expect_sym(b".", code::PERIOD_EXPECTED)?;
    let declarations = parser.ast.list(&declarations);
    let program = Kind::Program {
        name,
        uses,
        declarations,
        body,
    };
    Ok(parser.ast.add(program, line, col))
}

const ADD: &[(&[u8], Op)] = &[
    (b"+", Op::Add),
    (b"-", Op::Sub),
    (b"OR", Op::Or),
    (b"XOR", Op::Xor),
];
const MUL: &[(&[u8], Op)] = &[
    (b"*", Op::Mul),
    (b"/", Op::Slash),
    (b"DIV", Op::Div),
    (b"MOD", Op::Mod),
    (b"AND", Op::And),
    (b"SHL", Op::Shl),
    (b"SHR", Op::Shr),
];
const REL: &[(&[u8], Op)] = &[
    (b"=", Op::Eq),
    (b"<>", Op::Ne),
    (b"<", Op::Lt),
    (b">", Op::Gt),
    (b"<=", Op::Le),
    (b">=", Op::Ge),
    (b"IN", Op::In),
];

impl Parser<'_> {
    fn tok(&self) -> &Tok {
        &self.tokens[self.pos]
    }

    /// The token `ahead` places after the lookahead. The final Eof token is never passed, so
    /// callers only look ahead after seeing a token that is not Eof.
    fn peek(&self, ahead: usize) -> &Tok {
        &self.tokens[self.pos + ahead]
    }

    /// The token just consumed.
    fn previous(&self) -> &Tok {
        &self.tokens[self.pos - 1]
    }

    /// Consumes the lookahead. New nodes belong to the file and `{$R}`/`{$I}` state of the last
    /// token consumed.
    fn advance(&mut self) {
        self.ast.file = self.tokens[self.pos].file;
        self.ast.range_checks = self.tokens[self.pos].range_checks;
        self.ast.io_checks = self.tokens[self.pos].io_checks;
        self.pos += 1;
    }

    fn pos(&self) -> (u32, u32) {
        (self.tok().line, self.tok().col)
    }

    /// An error at the lookahead token; at end of file it is always 10, at a scanning error it is
    /// that error.
    fn err(&self, code: u16) -> Diag {
        let t = self.tok();
        let code = match t.kind {
            Tk::Eof => code::UNEXPECTED_END_OF_FILE,
            Tk::Error => t.int as u16,
            _ => code,
        };
        Diag {
            code,
            line: t.line,
            col: t.col,
            file: t.file,
        }
    }

    fn is(&self, kind: Tk, text: &[u8]) -> bool {
        self.tok().kind == kind && self.tok().text == text
    }

    /// Consumes the lookahead if it is the keyword `text`.
    fn kw(&mut self, text: &[u8]) -> bool {
        let hit = self.is(Tk::Kw, text);
        if hit {
            self.advance();
        }
        hit
    }

    /// Consumes the lookahead if it is the symbol `text`.
    fn sym(&mut self, text: &[u8]) -> bool {
        let hit = self.is(Tk::Sym, text);
        if hit {
            self.advance();
        }
        hit
    }

    fn expect_sym(&mut self, text: &[u8], code: u16) -> R<()> {
        if self.sym(text) {
            Ok(())
        } else {
            Err(self.err(code))
        }
    }

    fn expect_kw(&mut self, text: &[u8], code: u16) -> R<()> {
        if self.kw(text) {
            Ok(())
        } else {
            Err(self.err(code))
        }
    }

    fn ident(&mut self) -> R<u32> {
        if self.tok().kind != Tk::Ident {
            return Err(self.err(code::IDENTIFIER_EXPECTED));
        }
        let name = self.ast.intern(&self.tokens[self.pos].text);
        self.advance();
        Ok(name)
    }

    /// An identifier that names something declared elsewhere. A unit name followed by `.` and an
    /// identifier is the qualified name of that unit's member.
    fn name_reference(&mut self) -> R<u32> {
        let name = self.ident()?;
        let qualifies = self.unit_names.contains(&name)
            && self.is(Tk::Sym, b".")
            && self.peek(1).kind == Tk::Ident;
        if !qualifies {
            return Ok(name);
        }
        self.advance();
        let member = self.ident()?;
        let mut joined = self.ast.names[name as usize].clone();
        joined.push(b'.');
        joined.extend_from_slice(&self.ast.names[member as usize]);
        let joined = self.ast.intern(&joined);
        let known = self.ast.qualified.iter().any(|q| q.joined == joined);
        if !known {
            let qualified = Qualified {
                joined,
                unit: name,
                member,
            };
            self.ast.qualified.push(qualified);
        }
        Ok(joined)
    }

    /// `ident {, ident}` with the position of each.
    fn identifier_list(&mut self) -> R<Vec<IdentAt>> {
        let mut identifiers = Vec::new();
        loop {
            let (line, col) = self.pos();
            let name = self.ident()?;
            identifiers.push(IdentAt { name, line, col });
            if !self.sym(b",") {
                return Ok(identifiers);
            }
        }
    }

    /// A node stamped with the lookahead position (see `Kind`).
    fn after(&mut self, kind: Kind) -> Id {
        let (line, col) = self.pos();
        self.ast.add(kind, line, col)
    }

    /// Takes the handle of a node that is stamped with the lookahead now but whose children are
    /// parsed later; `fill_node` gives it its real kind.
    fn reserve_node(&mut self) -> Id {
        let nothing = self.ast.list(&[]);
        self.after(Kind::Compound(nothing))
    }

    fn fill_node(&mut self, id: Id, kind: Kind) {
        self.ast.set_kind(id, kind);
    }

    /// Error 200 at the lookahead if it is one of the omitted keywords `words`.
    fn unsupported(&self, words: &[&[u8]]) -> R<()> {
        let t = self.tok();
        if t.kind == Tk::Kw && words.contains(&&t.text[..]) {
            return Err(self.err(code::NOT_SUPPORTED));
        }
        Ok(())
    }

    // ---- Statements ----

    /// Statements separated by `;`, up to (not including) the keyword `closer`.
    fn statements(&mut self, stops: StopWords, closer: &[u8]) -> R<Vec<Id>> {
        let outer = std::mem::replace(&mut self.stops, stops);
        let mut items = Vec::new();
        loop {
            items.extend(self.statement()?);
            if !self.sym(b";") {
                break;
            }
        }
        self.stops = outer;
        if !self.is(Tk::Kw, closer) {
            return Err(self.err(code::SEMICOLON_EXPECTED));
        }
        Ok(items)
    }

    /// `BEGIN statements END`.
    fn begin(&mut self) -> R<Id> {
        self.expect_kw(b"BEGIN", code::BEGIN_EXPECTED)?;
        self.compound()
    }

    /// After `BEGIN`; the node is stamped at the `BEGIN`.
    fn compound(&mut self) -> R<Id> {
        let (line, col) = (self.previous().line, self.previous().col);
        let items = self.statements(StopWords::END, b"END")?;
        self.advance();
        let list = self.ast.list(&items);
        Ok(self.ast.add(Kind::Compound(list), line, col))
    }

    /// A statement used as a branch or body; an empty one becomes an empty compound.
    fn sub(&mut self, stops: StopWords) -> R<Id> {
        let outer = std::mem::replace(&mut self.stops, stops);
        let statement = self.statement()?;
        self.stops = outer;
        match statement {
            Some(statement) => Ok(statement),
            None => Ok(self.reserve_node()),
        }
    }

    /// A statement, or None for an empty one (before `;`, `end`, `until` or `else`).
    fn statement(&mut self) -> R<Option<Id>> {
        if self.tok().kind == Tk::Ident {
            return self.assignment_or_call().map(Some);
        }
        if self.kw(b"BEGIN") {
            return self.compound().map(Some);
        }
        if let Some(statement) = self.control_statement()? {
            return Ok(Some(statement));
        }
        let t = self.tok();
        let stops_here = t.kind == Tk::Kw && self.stops.contains(&t.text);
        if stops_here || self.is(Tk::Sym, b";") {
            return Ok(None);
        }
        self.unsupported(&[b"GOTO", b"ASM", b"INLINE"])?;
        Err(self.err(code::ERROR_IN_STATEMENT))
    }

    /// A statement that starts with an identifier: an assignment `v[i].f := e`, a procedure
    /// call `p(args)`, or an assignment to a typecast `T(x)^ := e`.
    fn assignment_or_call(&mut self) -> R<Id> {
        let target = self.designator()?;
        if self.sym(b":=") {
            let value = self.expr()?;
            return Ok(self.after(Kind::Assign(target, value)));
        }
        let (mut args, mut call_at) = (Vec::new(), self.pos());
        if self.sym(b"(") {
            (args, call_at) = self.arguments()?;
        } else if self.is(Tk::Sym, b"=") {
            // TP takes the identifier for a variable and wants `:=`.
            return Err(self.err(code::ASSIGN_EXPECTED));
        }
        let list = self.ast.list(&args);
        let call = self.ast.add(Kind::Call(target, list), call_at.0, call_at.1);
        if self.is(Tk::Sym, b"^") {
            // A typecast designator: `T(x)^ := v`.
            let target = self.selectors(call)?;
            self.expect_sym(b":=", code::ERROR_IN_STATEMENT)?;
            let value = self.expr()?;
            return Ok(self.after(Kind::Assign(target, value)));
        }
        Ok(call)
    }

    /// `if`, `while`, `repeat`, `case`, `with` and `for` statements; None if the lookahead
    /// starts none of them.
    fn control_statement(&mut self) -> R<Option<Id>> {
        let statement = if self.kw(b"IF") {
            self.if_statement()?
        } else if self.kw(b"WHILE") {
            self.while_statement()?
        } else if self.kw(b"REPEAT") {
            self.repeat_statement()?
        } else if self.kw(b"CASE") {
            self.case_statement()?
        } else if self.kw(b"WITH") {
            self.with_statement()?
        } else if self.kw(b"FOR") {
            self.for_statement()?
        } else {
            return Ok(None);
        };
        Ok(Some(statement))
    }

    fn if_statement(&mut self) -> R<Id> {
        let condition = self.expr()?;
        let id = self.reserve_node();
        self.expect_kw(b"THEN", code::THEN_EXPECTED)?;
        let then_branch = self.sub(self.stops.with_else())?;
        let else_branch = if self.kw(b"ELSE") {
            Some(self.sub(self.stops)?)
        } else {
            None
        };
        let kind = Kind::If {
            condition,
            then_branch,
            else_branch,
        };
        self.fill_node(id, kind);
        Ok(id)
    }

    fn while_statement(&mut self) -> R<Id> {
        let condition = self.expr()?;
        let id = self.reserve_node();
        self.expect_kw(b"DO", code::DO_EXPECTED)?;
        let body = self.sub(self.stops)?;
        self.fill_node(id, Kind::While(condition, body));
        Ok(id)
    }

    fn repeat_statement(&mut self) -> R<Id> {
        let items = self.statements(StopWords::UNTIL, b"UNTIL")?;
        self.advance();
        let condition = self.expr()?;
        let body = self.ast.list(&items);
        Ok(self.after(Kind::Repeat(body, condition)))
    }

    /// After `CASE`: `expr of arm {; arm} [; else statements] [;] end`.
    fn case_statement(&mut self) -> R<Id> {
        let selector = self.expr()?;
        let id = self.reserve_node();
        self.expect_kw(b"OF", code::OF_EXPECTED)?;
        let mut arms = Vec::new();
        loop {
            arms.push(self.case_arm()?);
            let more = self.sym(b";") && !self.is(Tk::Kw, b"ELSE") && !self.is(Tk::Kw, b"END");
            if !more {
                break;
            }
        }
        let else_statements = if self.kw(b"ELSE") {
            self.statements(StopWords::END, b"END")?
        } else {
            Vec::new()
        };
        self.expect_kw(b"END", code::SEMICOLON_EXPECTED)?;
        let kind = Kind::Case {
            selector,
            arms: self.ast.list(&arms),
            else_body: self.ast.list(&else_statements),
        };
        self.fill_node(id, kind);
        Ok(id)
    }

    /// `label {, label}: statement`, a label being a constant or a range `lo..hi`.
    fn case_arm(&mut self) -> R<Id> {
        let (line, col) = self.pos();
        let mut labels = Vec::new();
        loop {
            let (lo_line, lo_col) = self.pos();
            let lo = self.expr()?;
            let hi = if self.sym(b"..") { self.expr()? } else { lo };
            labels.push(self.ast.add(Kind::Range(lo, hi), lo_line, lo_col));
            if !self.sym(b",") {
                break;
            }
        }
        self.expect_sym(b":", code::COLON_EXPECTED)?;
        let body = self.sub(StopWords::END.with_else())?;
        let labels = self.ast.list(&labels);
        Ok(self.ast.add(Kind::Arm(labels, body), line, col))
    }

    fn with_statement(&mut self) -> R<Id> {
        let mut records = Vec::new();
        loop {
            records.push(self.designator()?);
            if !self.sym(b",") {
                break;
            }
        }
        let id = self.reserve_node();
        self.expect_kw(b"DO", code::DO_EXPECTED)?;
        let body = self.sub(self.stops)?;
        let records = self.ast.list(&records);
        self.fill_node(id, Kind::With(records, body));
        Ok(id)
    }

    fn for_statement(&mut self) -> R<Id> {
        let (line, col) = self.pos();
        if self.tok().kind != Tk::Ident {
            return Err(self.err(code::VARIABLE_EXPECTED));
        }
        let name = self.ident()?;
        let variable = self.ast.add(Kind::Name(name), line, col);
        self.expect_sym(b":=", code::ASSIGN_EXPECTED)?;
        let first = self.expr()?;
        let downto = self.is(Tk::Kw, b"DOWNTO");
        if !downto && !self.is(Tk::Kw, b"TO") {
            return Err(self.err(code::TO_OR_DOWNTO_EXPECTED));
        }
        self.advance();
        let last = self.expr()?;
        let id = self.reserve_node();
        self.expect_kw(b"DO", code::DO_EXPECTED)?;
        let body = self.sub(self.stops)?;
        let kind = Kind::For {
            variable,
            first,
            last,
            body,
            downto,
        };
        self.fill_node(id, kind);
        Ok(id)
    }

    // ---- Types ----

    /// A type name; `String` is the keyword.
    fn type_name(&mut self) -> R<u32> {
        self.unsupported(&[b"FILE"])?;
        if self.kw(b"STRING") {
            return Ok(self.ast.intern(b"STRING"));
        }
        if self.is(Tk::Kw, b"ARRAY") || self.is(Tk::Kw, b"RECORD") || self.is(Tk::Kw, b"PACKED") {
            return Err(self.err(code::TYPE_IDENTIFIER_EXPECTED));
        }
        self.name_reference()
    }

    /// An optional parenthesised parameter list, as `Param` nodes.
    fn params(&mut self) -> R<Option<List>> {
        if !self.sym(b"(") {
            return Ok(None);
        }
        let mut list = Vec::new();
        loop {
            let by_ref = self.kw(b"VAR");
            let names = self.identifier_list()?;
            if by_ref && !self.is(Tk::Sym, b":") {
                return Err(self.err(code::NOT_SUPPORTED));
            }
            self.expect_sym(b":", code::COLON_EXPECTED)?;
            let type_name = self.type_name()?;
            for IdentAt { name, line, col } in names {
                let param = Kind::Param {
                    name,
                    type_name,
                    by_ref,
                };
                list.push(self.ast.add(param, line, col));
            }
            if !self.sym(b";") {
                break;
            }
        }
        self.expect_sym(b")", code::RIGHT_PAREN_EXPECTED)?;
        Ok(Some(self.ast.list(&list)))
    }

    /// A type: a type name, a pointer `^T`, an enumeration `(a, b)`, a set, array, record or
    /// string type, a subrange `lo..hi` of constants, or `procedure [(params)]` /
    /// `function [(params)]: name`.
    fn type_spec(&mut self) -> R<Id> {
        let (line, col) = self.pos();
        // `^T` with a one-letter name is lexed as a control character.
        let control_letter =
            self.tok().kind == Tk::Ctrl && self.tok().text[0].is_ascii_alphabetic();
        if control_letter {
            let name = self.ast.intern(&self.tokens[self.pos].text);
            self.advance();
            return Ok(self.ast.add(Kind::PtrType(name), line, col));
        }
        if self.sym(b"^") {
            let name = self.type_name()?;
            return Ok(self.ast.add(Kind::PtrType(name), line, col));
        }
        if self.sym(b"(") {
            return self.enumeration(line, col);
        }
        if self.kw(b"SET") {
            self.expect_kw(b"OF", code::OF_EXPECTED)?;
            let base = self.type_spec()?;
            return Ok(self.ast.add(Kind::SetType(base), line, col));
        }
        // `packed` is accepted only in front of `array` or `record`, and then ignored.
        if self.kw(b"PACKED") && !self.is(Tk::Kw, b"ARRAY") && !self.is(Tk::Kw, b"RECORD") {
            return Err(self.err(code::SYNTAX_ERROR));
        }
        if self.kw(b"ARRAY") {
            return self.array_type(line, col);
        }
        if self.kw(b"RECORD") {
            return self.record_type(line, col);
        }
        if self.is(Tk::Kw, b"STRING") && self.peek(1).text == b"[" {
            self.pos += 2;
            let size = self.expr()?;
            self.expect_sym(b"]", code::RIGHT_BRACKET_EXPECTED)?;
            return Ok(self.ast.add(Kind::StrType(size), line, col));
        }
        let is_function = self.is(Tk::Kw, b"FUNCTION");
        if self.kw(b"PROCEDURE") || self.kw(b"FUNCTION") {
            return self.procedure_type(is_function, line, col);
        }
        self.subrange_or_type_name(line, col)
    }

    /// After `(`: the identifiers of an enumeration.
    fn enumeration(&mut self, line: u32, col: u32) -> R<Id> {
        let mut names = Vec::new();
        for IdentAt { name, line, col } in self.identifier_list()? {
            names.push(self.ast.add(Kind::Name(name), line, col));
        }
        self.expect_sym(b")", code::RIGHT_PAREN_EXPECTED)?;
        let list = self.ast.list(&names);
        Ok(self.ast.add(Kind::Enum(list), line, col))
    }

    /// After `array`: `[index types] of element type`.
    fn array_type(&mut self, line: u32, col: u32) -> R<Id> {
        self.expect_sym(b"[", code::RIGHT_BRACKET_EXPECTED)?;
        let mut index = vec![self.type_spec()?];
        while self.sym(b",") {
            index.push(self.type_spec()?);
        }
        self.expect_sym(b"]", code::RIGHT_BRACKET_EXPECTED)?;
        self.expect_kw(b"OF", code::OF_EXPECTED)?;
        let (index, element) = (self.ast.list(&index), self.type_spec()?);
        Ok(self.ast.add(Kind::Array(index, element), line, col))
    }

    /// After `record`: the fields up to `end`.
    fn record_type(&mut self, line: u32, col: u32) -> R<Id> {
        let fields = self.fields()?;
        if !self.kw(b"END") {
            return Err(self.err(code::IDENTIFIER_EXPECTED));
        }
        let list = self.ast.list(&fields);
        Ok(self.ast.add(Kind::Record(list), line, col))
    }

    /// After `procedure` or `function`: the parameters and, for a function, `: result`.
    fn procedure_type(&mut self, is_function: bool, line: u32, col: u32) -> R<Id> {
        let params = self.params()?;
        let mut result = NO_NAME;
        if is_function {
            self.expect_sym(b":", code::COLON_EXPECTED)?;
            result = self.type_name()?;
        }
        let kind = Kind::ProcType {
            params,
            result,
            is_function,
        };
        Ok(self.ast.add(kind, line, col))
    }

    /// A type name (including `String`) or a subrange `lo..hi` whose bounds are constants.
    fn subrange_or_type_name(&mut self, line: u32, col: u32) -> R<Id> {
        self.unsupported(&[b"FILE"])?;
        if self.is(Tk::Kw, b"STRING") {
            let name = self.type_name()?;
            return Ok(self.ast.add(Kind::Name(name), line, col));
        }
        let t = self.tok();
        let starts_constant = matches!(t.kind, Tk::Ident | Tk::IntLit | Tk::StrLit | Tk::Ctrl)
            || self.is(Tk::Sym, b"-")
            || self.is(Tk::Sym, b"+");
        if !starts_constant {
            return Err(self.err(code::IDENTIFIER_EXPECTED));
        }
        // A simple expression: a relational operator after a type name (`of Integer = (1, 2)`)
        // belongs to the declaration, not to the type.
        let lo = self.simple()?;
        if !self.sym(b"..") {
            // Not a range: the expression must have been a lone type name.
            return match self.ast.kind(lo) {
                Kind::Name(_) => Ok(lo),
                _ => Err(self.err(code::SYNTAX_ERROR)),
            };
        }
        let hi = self.simple()?;
        Ok(self.ast.add(Kind::Range(lo, hi), line, col))
    }

    /// A record's field list up to (not including) `END` or `)`: fixed fields, then an optional
    /// variant part `case [tag:] type of labels: (fields); ...`.
    fn fields(&mut self) -> R<Vec<Id>> {
        let mut out = Vec::new();
        while self.tok().kind == Tk::Ident {
            let names = self.identifier_list()?;
            self.expect_sym(b":", code::COLON_EXPECTED)?;
            let spec = self.type_spec()?;
            for IdentAt { name, line, col } in names {
                out.push(self.ast.add(Kind::FieldDecl(name, spec), line, col));
            }
            if !self.sym(b";") {
                return Ok(out);
            }
        }
        if self.is(Tk::Kw, b"CASE") {
            self.variant_part(&mut out)?;
        }
        Ok(out)
    }

    /// `case [tag:] type of labels: (fields); ...`, appended to `out` as the selector's
    /// `FieldDecl` and a `Variant` node holding one `Record` per arm.
    fn variant_part(&mut self, out: &mut Vec<Id>) -> R<()> {
        let (line, col) = self.pos();
        self.advance();
        let has_tag = self.peek(1).text == b":";
        let tag_name = if has_tag { self.ident()? } else { NO_NAME };
        if has_tag {
            self.advance();
        }
        let selector_type = self.type_spec()?;
        out.push(
            self.ast
                .add(Kind::FieldDecl(tag_name, selector_type), line, col),
        );
        self.expect_kw(b"OF", code::OF_EXPECTED)?;
        let mut arms = Vec::new();
        loop {
            let (arm_line, arm_col) = self.pos();
            // The labels are parsed and dropped: arms are told apart only by position.
            loop {
                self.expr()?;
                if !self.sym(b",") {
                    break;
                }
            }
            self.expect_sym(b":", code::COLON_EXPECTED)?;
            self.expect_sym(b"(", code::LEFT_PAREN_EXPECTED)?;
            let inner = self.fields()?;
            self.expect_sym(b")", code::RIGHT_PAREN_EXPECTED)?;
            let list = self.ast.list(&inner);
            arms.push(self.ast.add(Kind::Record(list), arm_line, arm_col));
            let more = self.sym(b";") && !self.is(Tk::Kw, b"END") && !self.is(Tk::Sym, b")");
            if !more {
                break;
            }
        }
        let list = self.ast.list(&arms);
        out.push(self.ast.add(Kind::Variant(list), line, col));
        Ok(())
    }

    // ---- Declarations ----

    /// After `PROCEDURE` or `FUNCTION`.
    fn routine(&mut self, is_function: bool) -> R<Id> {
        let name = self.ident()?;
        let params = self.params()?;
        let mut result = NO_NAME;
        if is_function && self.sym(b":") {
            result = self.type_name()?;
        } else if is_function && !self.is(Tk::Sym, b";") {
            return Err(self.err(code::COLON_EXPECTED));
        }
        let id = self.reserve_node();
        self.expect_sym(b";", code::SEMICOLON_EXPECTED)?;
        self.unsupported(&[b"INLINE", b"INTERRUPT"])?;
        let external = self.external_directive(name)?;
        // An interface heading is complete at its `;`: the definition comes later.
        let heading_only = self.in_interface;
        // `kw` consumes `forward`, so it is the last test.
        let has_no_body = heading_only || external.is_some() || self.kw(b"FORWARD");
        let block = if has_no_body {
            None
        } else {
            let (line, col) = self.pos();
            let declarations = self.declarations()?;
            let body = self.begin()?;
            let declarations = self.ast.list(&declarations);
            Some(self.ast.add(Kind::Block(declarations, body), line, col))
        };
        if !heading_only {
            self.expect_sym(b";", code::SEMICOLON_EXPECTED)?;
        }
        let kind = Kind::Routine {
            name,
            params,
            result,
            is_function,
            block,
            external,
        };
        self.fill_node(id, kind);
        Ok(id)
    }

    /// `external ['import name']` after a routine header: the routine is a WASM import of the
    /// host. Only the runtime source may use it (TP5 links an .OBJ file instead; error 200).
    /// Returns the import's name, which is the routine's own `name` unless a string gives it.
    fn external_directive(&mut self, name: u32) -> R<Option<u32>> {
        if !self.in_runtime {
            self.unsupported(&[b"EXTERNAL"])?;
            return Ok(None);
        }
        if !self.kw(b"EXTERNAL") {
            return Ok(None);
        }
        if self.tok().kind != Tk::StrLit {
            return Ok(Some(name));
        }
        let import_name = self.ast.intern(&self.tokens[self.pos].text);
        self.advance();
        Ok(Some(import_name))
    }

    /// `CONST`, `TYPE`, `VAR`, `PROCEDURE` and `FUNCTION` sections, in any order and number.
    fn declarations(&mut self) -> R<Vec<Id>> {
        let mut out = Vec::new();
        loop {
            if self.kw(b"CONST") {
                self.const_section(&mut out)?;
            } else if self.kw(b"TYPE") {
                self.type_section(&mut out)?;
            } else if self.kw(b"VAR") {
                self.var_section(&mut out)?;
            } else if self.kw(b"PROCEDURE") || self.kw(b"FUNCTION") {
                let is_function = self.previous().text == b"FUNCTION";
                out.push(self.routine(is_function)?);
            } else {
                self.unsupported(&[b"LABEL"])?;
                return Ok(out);
            }
        }
    }

    /// After `CONST`: `name = expr;` or the typed form `name: type = initialiser;` while an
    /// identifier follows.
    fn const_section(&mut self, out: &mut Vec<Id>) -> R<()> {
        loop {
            let (line, col) = self.pos();
            let name = self.ident()?;
            let kind = if self.sym(b":") {
                let type_name = self.declared_type(b"=", out)?;
                self.expect_sym(b"=", code::EQUALS_EXPECTED)?;
                Kind::TypedConst {
                    name,
                    type_name,
                    init: self.initialiser()?,
                }
            } else {
                self.expect_sym(b"=", code::EQUALS_EXPECTED)?;
                Kind::ConstDecl(name, self.expr()?)
            };
            self.expect_sym(b";", code::SEMICOLON_EXPECTED)?;
            out.push(self.ast.add(kind, line, col));
            if self.tok().kind != Tk::Ident {
                return Ok(());
            }
        }
    }

    /// The type after `name:` of a variable or typed constant: the handle of a type name, or
    /// of an anonymous type declared by a `TypeDecl` pushed to `out`. `end` is the symbol
    /// that follows the type.
    fn declared_type(&mut self, end: &[u8], out: &mut Vec<Id>) -> R<u32> {
        let names_a_type = self.tok().kind == Tk::Ident
            && self.peek(1).kind == Tk::Sym
            && self.peek(1).text == end;
        if names_a_type {
            return self.type_name();
        }
        let name = self.ast.anonymous_name();
        let (line, col) = self.pos();
        let spec = self.type_spec()?;
        out.push(self.ast.add(Kind::TypeDecl(name, spec), line, col));
        Ok(name)
    }

    /// The value of a typed constant: an expression, or a parenthesised list. A `(` may also
    /// start an expression such as `(1 + 2) * 3`, so the list is tried first and the parser
    /// goes back to the `(` when what follows the `)` does not end the initialiser. The nodes and
    /// `ast.follow` entries of the abandoned attempt stay in the arena, unused.
    fn initialiser(&mut self) -> R<Id> {
        if self.is(Tk::Sym, b"(") {
            let start = self.pos;
            if let Ok(list) = self.initialiser_list() {
                let ends = [b";".as_slice(), b")", b","];
                if ends.iter().any(|end| self.is(Tk::Sym, end)) {
                    return Ok(list);
                }
            }
            self.pos = start;
        }
        self.expr()
    }

    /// `(a, b, c)` is an `ArrayInit`; `(f: a; g: b)` (an identifier and a colon first) is a
    /// `RecordInit`.
    fn initialiser_list(&mut self) -> R<Id> {
        self.advance();
        let is_record = self.tok().kind == Tk::Ident && self.peek(1).text == b":";
        let separator: &[u8] = if is_record { b";" } else { b"," };
        let mut items = Vec::new();
        loop {
            items.push(if is_record {
                self.field_initialiser()?
            } else {
                self.initialiser()?
            });
            if !self.sym(separator) {
                break;
            }
        }
        let items = self.ast.list(&items);
        let kind = if is_record {
            Kind::RecordInit(items)
        } else {
            Kind::ArrayInit(items)
        };
        let (line, col) = self.pos();
        self.expect_sym(b")", code::RIGHT_PAREN_EXPECTED)?;
        Ok(self.ast.add(kind, line, col))
    }

    /// `field: initialiser`.
    fn field_initialiser(&mut self) -> R<Id> {
        let (line, col) = self.pos();
        let name = self.ident()?;
        self.expect_sym(b":", code::COLON_EXPECTED)?;
        let value = self.initialiser()?;
        Ok(self.ast.add(Kind::FieldInit(name, value), line, col))
    }

    /// After `TYPE`: `name = type;` entries while an identifier follows.
    fn type_section(&mut self, out: &mut Vec<Id>) -> R<()> {
        loop {
            let (line, col) = self.pos();
            let name = self.ident()?;
            self.expect_sym(b"=", code::EQUALS_EXPECTED)?;
            let spec = self.type_spec()?;
            self.expect_sym(b";", code::SEMICOLON_EXPECTED)?;
            out.push(self.ast.add(Kind::TypeDecl(name, spec), line, col));
            if self.tok().kind != Tk::Ident {
                return Ok(());
            }
        }
    }

    /// After `VAR`: `names: type;` entries while an identifier follows. A type that is not a
    /// plain name is declared first under an anonymous name.
    fn var_section(&mut self, out: &mut Vec<Id>) -> R<()> {
        loop {
            let names = self.identifier_list()?;
            self.expect_sym(b":", code::COLON_EXPECTED)?;
            let type_name = self.declared_type(b";", out)?;
            self.unsupported(&[b"ABSOLUTE"])?;
            self.expect_sym(b";", code::SEMICOLON_EXPECTED)?;
            for IdentAt { name, line, col } in names {
                out.push(self.ast.add(Kind::VarDecl(name, type_name), line, col));
            }
            if self.tok().kind != Tk::Ident {
                return Ok(());
            }
        }
    }

    // ---- Expressions ----

    /// Arguments after `(`, up to and including `)`; also the position of the `)`, where TP
    /// reports argument errors.
    fn arguments(&mut self) -> R<(Vec<Id>, (u32, u32))> {
        let mut args = Vec::new();
        loop {
            args.push(self.argument()?);
            if !self.sym(b",") {
                break;
            }
        }
        let close = self.pos();
        self.expect_sym(b")", code::RIGHT_PAREN_EXPECTED)?;
        Ok((args, close))
    }

    /// `expr [: width [: decimals]]`, the form of a write argument.
    fn argument(&mut self) -> R<Id> {
        let value = self.expr()?;
        if !self.sym(b":") {
            return Ok(value);
        }
        let width = self.expr()?;
        let decimals = self.sym(b":").then(|| self.expr()).transpose()?;
        Ok(self.after(Kind::Fmt {
            value,
            width,
            decimals,
        }))
    }

    /// Index and field selectors after the designator `e`, all stamped with `e`'s position.
    fn selectors(&mut self, mut e: Id) -> R<Id> {
        let (line, col) = self.ast.position(e);
        loop {
            if self.sym(b"[") {
                let mut continues = false;
                loop {
                    let index = self.expr()?;
                    let node = Kind::Index {
                        base: e,
                        index,
                        continues,
                    };
                    e = self.ast.add(node, line, col);
                    continues = true;
                    if !self.sym(b",") {
                        break;
                    }
                }
                self.expect_sym(b"]", code::RIGHT_BRACKET_EXPECTED)?;
            } else if self.sym(b"^") {
                e = self.ast.add(Kind::Deref(e), line, col);
            } else if self.sym(b".") {
                if self.tok().kind != Tk::Ident {
                    return Err(self.err(code::FIELD_IDENTIFIER_EXPECTED));
                }
                let name = self.ident()?;
                e = self.ast.add(Kind::Field(e, name), line, col);
            } else {
                return Ok(e);
            }
        }
    }

    /// A variable with selectors.
    fn designator(&mut self) -> R<Id> {
        let (line, col) = self.pos();
        let name = self.name_reference()?;
        let base = self.ast.add(Kind::Name(name), line, col);
        self.selectors(base)
    }

    /// Consumes the lookahead if it is a keyword or symbol listed in `ops`.
    fn operator(&mut self, ops: &[(&[u8], Op)]) -> Option<Op> {
        let t = self.tok();
        if !matches!(t.kind, Tk::Kw | Tk::Sym) {
            return None;
        }
        let (_, op) = ops.iter().find(|(text, _)| *text == t.text)?;
        let op = *op;
        self.advance();
        Some(op)
    }

    /// `simple [relation simple]`.
    fn expr(&mut self) -> R<Id> {
        let left = self.simple()?;
        let Some(op) = self.operator(REL) else {
            return Ok(left);
        };
        let right = self.simple()?;
        Ok(self.after(Kind::Binary { op, left, right }))
    }

    /// `[sign] term {addop term}`; a leading sign applies to the whole first term (`-a * b`
    /// is `-(a * b)`). `factor` accepts a sign in front of any later operand.
    fn simple(&mut self) -> R<Id> {
        let negate = self.sym(b"-");
        self.sym(b"+");
        let mut left = self.term()?;
        if negate {
            left = self.after(Kind::Unary(Op::Neg, left));
        }
        while let Some(op) = self.operator(ADD) {
            let right = self.term()?;
            left = self.after(Kind::Binary { op, left, right });
        }
        Ok(left)
    }

    /// `factor {mulop factor}`.
    fn term(&mut self) -> R<Id> {
        let mut left = self.factor()?;
        while let Some(op) = self.operator(MUL) {
            let right = self.factor()?;
            left = self.after(Kind::Binary { op, left, right });
        }
        Ok(left)
    }

    fn factor(&mut self) -> R<Id> {
        // TP accepts a sign before any operand, not only the first term.
        let negate = self.sym(b"-");
        self.sym(b"+");
        if negate {
            let operand = self.factor()?;
            return Ok(self.after(Kind::Unary(Op::Neg, operand)));
        }
        let (line, col) = self.pos();
        let t = self.tok();
        match t.kind {
            Tk::Ident => self.identifier_factor(line, col),
            Tk::IntLit => {
                // Only a hexadecimal literal can exceed LongInt (the lexer caps decimal ones):
                // `$80000000`..`$FFFFFFFF` is the 32-bit pattern of a negative LongInt.
                let value = t.int as u32 as i32 as i64;
                self.advance();
                Ok(self.ast.add(Kind::IntLit(value), line, col))
            }
            Tk::RealLit => {
                let value = t.real;
                self.advance();
                Ok(self.ast.add(Kind::RealLit(value), line, col))
            }
            Tk::StrLit | Tk::Ctrl => {
                let text = if t.kind == Tk::Ctrl {
                    vec![t.text[0].to_ascii_uppercase() - b'@']
                } else {
                    t.text.clone()
                };
                self.advance();
                self.ast.strs.push(text);
                let index = self.ast.strs.len() as u32 - 1;
                Ok(self.ast.add(Kind::StrLit(index), line, col))
            }
            Tk::Sym if t.text == b"[" => self.set_constructor(line, col),
            Tk::Sym if t.text == b"(" => self.parenthesised(line, col),
            Tk::Sym if t.text == b"@" => {
                self.advance();
                let operand = self.factor()?;
                Ok(self.ast.add(Kind::AddrOf(operand), line, col))
            }
            Tk::Kw if t.text == b"STRING" => {
                let name = self.type_name()?;
                Ok(self.ast.add(Kind::Name(name), line, col))
            }
            Tk::Kw if t.text == b"NIL" => {
                self.advance();
                let nil = self.ast.intern(b"NIL");
                let e = self.ast.add(Kind::Name(nil), line, col);
                self.selectors(e)
            }
            Tk::Kw if t.text == b"NOT" => {
                self.advance();
                let operand = self.factor()?;
                Ok(self.after(Kind::Unary(Op::Not, operand)))
            }
            _ => Err(self.err(code::ERROR_IN_EXPRESSION)),
        }
    }

    /// A variable, a constant, or a call `f(args)`, with selectors.
    fn identifier_factor(&mut self, line: u32, col: u32) -> R<Id> {
        let name = self.name_reference()?;
        let callee = self.ast.add(Kind::Name(name), line, col);
        if !self.sym(b"(") {
            let follow = FollowLine {
                node: callee,
                line: self.tok().line,
            };
            self.ast.follow.push(follow);
            let designator = self.selectors(callee)?;
            return self.call_of_designator(designator);
        }
        let (args, call_at) = self.arguments()?;
        let list = self.ast.list(&args);
        let call = self.ast.add(Kind::Call(callee, list), call_at.0, call_at.1);
        // `f(x)^` is a dereference; `f(x)[i]` and `f(x).a` parse as selectors so that the
        // checker, not the grammar, rejects a call that is not a variable (error 121).
        if self.is(Tk::Sym, b"^") || self.is(Tk::Sym, b"[") {
            self.selectors(call)
        } else {
            Ok(call)
        }
    }

    /// `r.f(args)` or `a[i](args)`: after the selectors, a call through the procedural value
    /// that the variable holds.
    fn call_of_designator(&mut self, designator: Id) -> R<Id> {
        if !self.sym(b"(") {
            return Ok(designator);
        }
        let (args, call_at) = self.arguments()?;
        let list = self.ast.list(&args);
        Ok(self
            .ast
            .add(Kind::Call(designator, list), call_at.0, call_at.1))
    }

    /// `[a, b..c]`: a set constructor.
    fn set_constructor(&mut self, line: u32, col: u32) -> R<Id> {
        self.advance();
        let mut items = Vec::new();
        while !self.is(Tk::Sym, b"]") {
            let lo = self.expr()?;
            if self.sym(b"..") {
                let hi = self.expr()?;
                items.push(self.ast.add(Kind::Range(lo, hi), line, col));
            } else {
                items.push(lo);
            }
            if !self.sym(b",") {
                break;
            }
        }
        self.expect_sym(b"]", code::RIGHT_BRACKET_EXPECTED)?;
        let list = self.ast.list(&items);
        Ok(self.ast.add(Kind::SetLit(list), line, col))
    }

    /// `( expr )`; a parenthesised name is marked so that it is not taken for a variable
    /// reference when passed as a `var` argument.
    fn parenthesised(&mut self, line: u32, col: u32) -> R<Id> {
        self.advance();
        let e = self.expr()?;
        self.expect_sym(b")", code::RIGHT_PAREN_EXPECTED)?;
        if matches!(self.ast.kind(e), Kind::Name(_)) {
            return Ok(self.ast.add(Kind::Paren(e), line, col));
        }
        Ok(e)
    }
}
