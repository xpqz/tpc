//! Recursive-descent parser: `[program header;] begin statements end .`
use crate::ast::{Ast, Id, Kind, List, Op, NO_NAME};
use crate::driver::Diag;
use crate::lex::{Tk, Tok};

/// Keywords that may follow an empty statement in the enclosing construct.
const END: u8 = 1;
const UNTIL: u8 = 2;
const ELSE: u8 = 4;

struct Parser {
    toks: Vec<Tok>,
    at: usize,
    ast: Ast,
    stops: u8,
}

type R<T> = Result<T, Diag>;

/// Only the `{$F+}` and `{$F-}` switches are accepted; they have no effect.
fn drop_directives(toks: Vec<Tok>) -> R<Vec<Tok>> {
    for t in toks.iter().filter(|t| t.kind == Tk::Directive) {
        if !t.text.eq_ignore_ascii_case(b"F+") && !t.text.eq_ignore_ascii_case(b"F-") {
            return Err(Diag {
                code: 200,
                line: t.line,
                col: t.col,
            });
        }
    }
    Ok(toks
        .into_iter()
        .filter(|t| t.kind != Tk::Directive)
        .collect())
}

/// The System declaration part `system`, then the program `toks`; the program's declaration list
/// starts with System's (`Ast::system` of them).
pub fn parse(system: Vec<Tok>, toks: Vec<Tok>) -> R<(Ast, Id)> {
    let mut p = Parser {
        toks: drop_directives(system)?,
        at: 0,
        ast: Ast::default(),
        stops: 0,
    };
    let mut sys = p.declarations()?;
    if p.tok().kind != Tk::Eof {
        return Err(p.err(36));
    }
    p.ast.system = sys.len();
    (p.toks, p.at) = (drop_directives(toks)?, 0);
    let mut name = NO_NAME;
    if p.kw(b"PROGRAM") {
        name = p.ident()?;
        if p.sym(b"(") {
            loop {
                p.ident()?;
                if !p.sym(b",") {
                    break;
                }
            }
            p.expect_sym(b")", 89)?;
        }
        p.expect_sym(b";", 85)?;
    }
    sys.extend(p.declarations()?);
    let (line, col) = p.pos();
    let body = p.begin()?;
    p.expect_sym(b".", 94)?;
    let decls = p.ast.list(&sys);
    let root = p.ast.add(Kind::Program(name, decls, body), line, col);
    Ok((p.ast, root))
}

const ADD: &[(&[u8], Op)] = &[
    (b"+", Op::Add),
    (b"-", Op::Sub),
    (b"OR", Op::Or),
    (b"XOR", Op::Xor),
];
const MUL: &[(&[u8], Op)] = &[
    (b"*", Op::Mul),
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

impl Parser {
    fn tok(&self) -> &Tok {
        &self.toks[self.at]
    }

    /// An error at the lookahead token; at end of file it is always 10.
    fn err(&self, code: u16) -> Diag {
        let t = self.tok();
        let code = if t.kind == Tk::Eof { 10 } else { code };
        Diag {
            code,
            line: t.line,
            col: t.col,
        }
    }

    fn is(&self, kind: Tk, text: &[u8]) -> bool {
        self.tok().kind == kind && self.tok().text == text
    }

    fn kw(&mut self, text: &[u8]) -> bool {
        let hit = self.is(Tk::Kw, text);
        self.at += hit as usize;
        hit
    }

    fn sym(&mut self, text: &[u8]) -> bool {
        let hit = self.is(Tk::Sym, text);
        self.at += hit as usize;
        hit
    }

    fn expect_sym(&mut self, text: &[u8], code: u16) -> R<()> {
        if self.sym(text) {
            Ok(())
        } else {
            Err(self.err(code))
        }
    }

    fn ident(&mut self) -> R<u32> {
        if self.tok().kind != Tk::Ident {
            return Err(self.err(2));
        }
        let name = self.ast.intern(&self.toks[self.at].text);
        self.at += 1;
        Ok(name)
    }

    /// Statements separated by `;`, up to (not including) the keyword `closer`.
    fn statements(&mut self, stops: u8, closer: &[u8]) -> R<Vec<Id>> {
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
            return Err(self.err(85));
        }
        Ok(items)
    }

    /// `BEGIN statements END`.
    fn begin(&mut self) -> R<Id> {
        self.expect_kw(b"BEGIN", 36)?;
        self.compound()
    }

    /// After `BEGIN`; the node is stamped at the `BEGIN`.
    fn compound(&mut self) -> R<Id> {
        let (line, col) = (self.toks[self.at - 1].line, self.toks[self.at - 1].col);
        let items = self.statements(END, b"END")?;
        self.at += 1;
        let list = self.ast.list(&items);
        Ok(self.ast.add(Kind::Compound(list), line, col))
    }

    /// A statement used as a branch or body; an empty one becomes an empty compound.
    fn sub(&mut self, stops: u8) -> R<Id> {
        let outer = std::mem::replace(&mut self.stops, stops);
        let s = self.statement()?;
        self.stops = outer;
        Ok(match s {
            Some(s) => s,
            None => {
                let list = self.ast.list(&[]);
                self.after(Kind::Compound(list))
            }
        })
    }

    fn expect_kw(&mut self, text: &[u8], code: u16) -> R<()> {
        if self.kw(text) {
            Ok(())
        } else {
            Err(self.err(code))
        }
    }

    /// After `CASE`: `expr of arm {; arm} [; else statements] [;] end`.
    fn case(&mut self) -> R<Id> {
        let selector = self.expr()?;
        let placeholder = Kind::Compound(self.ast.list(&[]));
        let id = self.after(placeholder);
        self.expect_kw(b"OF", 54)?;
        let mut arms = Vec::new();
        loop {
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
            self.expect_sym(b":", 86)?;
            let body = self.sub(END | ELSE)?;
            let labels = self.ast.list(&labels);
            arms.push(self.ast.add(Kind::Arm(labels, body), line, col));
            if !self.sym(b";") || self.is(Tk::Kw, b"ELSE") || self.is(Tk::Kw, b"END") {
                break;
            }
        }
        let other = if self.kw(b"ELSE") {
            self.statements(END, b"END")?
        } else {
            Vec::new()
        };
        self.expect_kw(b"END", 85)?;
        let (arms, other) = (self.ast.list(&arms), self.ast.list(&other));
        self.ast.nodes[id as usize].kind = Kind::Case(selector, arms, other);
        Ok(id)
    }

    fn control(&mut self) -> R<Option<Id>> {
        let placeholder = Kind::Compound(self.ast.list(&[]));
        if self.kw(b"IF") {
            let cond = self.expr()?;
            let id = self.after(placeholder);
            self.expect_kw(b"THEN", 57)?;
            let then = self.sub(self.stops | ELSE)?;
            let other = if self.kw(b"ELSE") {
                Some(self.sub(self.stops)?)
            } else {
                None
            };
            self.ast.nodes[id as usize].kind = Kind::If(cond, then, other);
            return Ok(Some(id));
        }
        if self.kw(b"WHILE") {
            let cond = self.expr()?;
            let id = self.after(placeholder);
            self.expect_kw(b"DO", 50)?;
            let body = self.sub(self.stops)?;
            self.ast.nodes[id as usize].kind = Kind::While(cond, body);
            return Ok(Some(id));
        }
        if self.kw(b"REPEAT") {
            let items = self.statements(UNTIL, b"UNTIL")?;
            self.at += 1;
            let cond = self.expr()?;
            let list = self.ast.list(&items);
            return Ok(Some(self.after(Kind::Repeat(list, cond))));
        }
        if self.kw(b"CASE") {
            return self.case().map(Some);
        }
        if self.kw(b"WITH") {
            let mut records = Vec::new();
            loop {
                records.push(self.designator()?);
                if !self.sym(b",") {
                    break;
                }
            }
            let id = self.after(placeholder);
            self.expect_kw(b"DO", 50)?;
            let body = self.sub(self.stops)?;
            let list = self.ast.list(&records);
            self.ast.nodes[id as usize].kind = Kind::With(list, body);
            return Ok(Some(id));
        }
        if self.kw(b"FOR") {
            let (line, col) = self.pos();
            if self.tok().kind != Tk::Ident {
                return Err(self.err(20));
            }
            let name = self.ident()?;
            let var = self.ast.add(Kind::Name(name), line, col);
            self.expect_sym(b":=", 91)?;
            let first = self.expr()?;
            let down = self.is(Tk::Kw, b"DOWNTO");
            if !down && !self.is(Tk::Kw, b"TO") {
                return Err(self.err(58));
            }
            self.at += 1;
            let last = self.expr()?;
            let id = self.after(placeholder);
            self.expect_kw(b"DO", 50)?;
            let body = self.sub(self.stops)?;
            self.ast.nodes[id as usize].kind = Kind::For(var, first, last, body, down);
            return Ok(Some(id));
        }
        Ok(None)
    }

    fn pos(&self) -> (u32, u32) {
        (self.tok().line, self.tok().col)
    }

    /// A node stamped with the lookahead position (see `Kind`).
    fn after(&mut self, kind: Kind) -> Id {
        let (line, col) = self.pos();
        self.ast.add(kind, line, col)
    }

    /// Error 200 at the lookahead if it is one of the omitted keywords `words`.
    fn unsupported(&self, words: &[&[u8]]) -> R<()> {
        let t = self.tok();
        if t.kind == Tk::Kw && words.contains(&&t.text[..]) {
            return Err(self.err(200));
        }
        Ok(())
    }

    /// A type name; `String` is the keyword.
    fn type_name(&mut self) -> R<u32> {
        self.unsupported(&[b"FILE"])?;
        if self.kw(b"STRING") {
            return Ok(self.ast.intern(b"STRING"));
        }
        if self.is(Tk::Kw, b"ARRAY") || self.is(Tk::Kw, b"RECORD") || self.is(Tk::Kw, b"PACKED") {
            return Err(self.err(12));
        }
        self.ident()
    }

    /// An optional parenthesised parameter list, as `Param` nodes.
    fn params(&mut self) -> R<Option<List>> {
        if !self.sym(b"(") {
            return Ok(None);
        }
        let mut list = Vec::new();
        loop {
            let byref = self.kw(b"VAR");
            let mut names = Vec::new();
            loop {
                let (line, col) = self.pos();
                names.push((self.ident()?, line, col));
                if !self.sym(b",") {
                    break;
                }
            }
            if byref && !self.is(Tk::Sym, b":") {
                return Err(self.err(200));
            }
            self.expect_sym(b":", 86)?;
            let ty = self.type_name()?;
            for (name, line, col) in names {
                list.push(self.ast.add(Kind::Param(name, ty, byref), line, col));
            }
            if !self.sym(b";") {
                break;
            }
        }
        self.expect_sym(b")", 89)?;
        Ok(Some(self.ast.list(&list)))
    }

    /// A type: a type name, an enumeration `(a, b)`, a subrange `lo..hi` of constants, or
    /// `procedure [(params)]` / `function [(params)]: name`.
    fn type_spec(&mut self) -> R<Id> {
        let (line, col) = self.pos();
        let func = self.is(Tk::Kw, b"FUNCTION");
        // `^T` with a one-letter name is lexed as a control character.
        let ctrl = self.tok().kind == Tk::Ctrl && self.tok().text[0].is_ascii_alphabetic();
        if ctrl || self.sym(b"^") {
            let name = match ctrl {
                true => self.ast.intern(&self.toks[self.at].text),
                false => self.type_name()?,
            };
            self.at += ctrl as usize;
            return Ok(self.ast.add(Kind::PtrType(name), line, col));
        }
        if self.sym(b"(") {
            let mut names = Vec::new();
            loop {
                let (line, col) = self.pos();
                let name = self.ident()?;
                names.push(self.ast.add(Kind::Name(name), line, col));
                if !self.sym(b",") {
                    break;
                }
            }
            self.expect_sym(b")", 89)?;
            let list = self.ast.list(&names);
            return Ok(self.ast.add(Kind::Enum(list), line, col));
        }
        if self.kw(b"SET") {
            self.expect_kw(b"OF", 54)?;
            let base = self.type_spec()?;
            return Ok(self.ast.add(Kind::SetType(base), line, col));
        }
        if self.kw(b"PACKED") && !self.is(Tk::Kw, b"ARRAY") && !self.is(Tk::Kw, b"RECORD") {
            return Err(self.err(5));
        }
        if self.kw(b"ARRAY") {
            self.expect_sym(b"[", 93)?;
            let mut index = vec![self.type_spec()?];
            while self.sym(b",") {
                index.push(self.type_spec()?);
            }
            self.expect_sym(b"]", 93)?;
            self.expect_kw(b"OF", 54)?;
            let (index, elem) = (self.ast.list(&index), self.type_spec()?);
            return Ok(self.ast.add(Kind::Array(index, elem), line, col));
        }
        if self.kw(b"RECORD") {
            let fields = self.fields()?;
            if !self.kw(b"END") {
                return Err(self.err(2));
            }
            let list = self.ast.list(&fields);
            return Ok(self.ast.add(Kind::Record(list), line, col));
        }
        if self.is(Tk::Kw, b"STRING") && self.toks[self.at + 1].text == b"[" {
            self.at += 2;
            let size = self.expr()?;
            self.expect_sym(b"]", 93)?;
            return Ok(self.ast.add(Kind::StrType(size), line, col));
        }
        if !self.kw(b"PROCEDURE") && !self.kw(b"FUNCTION") {
            self.unsupported(&[b"FILE"])?;
            if self.is(Tk::Kw, b"STRING") {
                let name = self.type_name()?;
                return Ok(self.ast.add(Kind::Name(name), line, col));
            }
            let t = self.tok();
            let starts = matches!(t.kind, Tk::Ident | Tk::IntLit | Tk::StrLit | Tk::Ctrl)
                || self.is(Tk::Sym, b"-")
                || self.is(Tk::Sym, b"+");
            if !starts {
                return Err(self.err(2));
            }
            let lo = self.expr()?;
            if !self.sym(b"..") {
                return match self.ast.nodes[lo as usize].kind {
                    Kind::Name(_) => Ok(lo),
                    _ => Err(self.err(5)),
                };
            }
            let hi = self.expr()?;
            return Ok(self.ast.add(Kind::Range(lo, hi), line, col));
        }
        let params = self.params()?;
        let mut result = NO_NAME;
        if func {
            self.expect_sym(b":", 86)?;
            result = self.type_name()?;
        }
        Ok(self
            .ast
            .add(Kind::ProcType(params, result, func), line, col))
    }

    /// A record's field list up to (not including) `END` or `)`: fixed fields, then an optional
    /// variant part `case [tag:] type of labels: (fields); ...`.
    fn fields(&mut self) -> R<Vec<Id>> {
        let mut out = Vec::new();
        while self.tok().kind == Tk::Ident {
            let mut names = Vec::new();
            loop {
                let (line, col) = self.pos();
                names.push((self.ident()?, line, col));
                if !self.sym(b",") {
                    break;
                }
            }
            self.expect_sym(b":", 86)?;
            let spec = self.type_spec()?;
            for (name, line, col) in names {
                out.push(self.ast.add(Kind::FieldDecl(name, spec), line, col));
            }
            if !self.sym(b";") {
                return Ok(out);
            }
        }
        if !self.is(Tk::Kw, b"CASE") {
            return Ok(out);
        }
        let (line, col) = self.pos();
        self.at += 1;
        let tagged = self.toks[self.at + 1].text == b":";
        let name = if tagged { self.ident()? } else { NO_NAME };
        if tagged {
            self.at += 1;
        }
        let spec = self.type_spec()?;
        out.push(self.ast.add(Kind::FieldDecl(name, spec), line, col));
        self.expect_kw(b"OF", 54)?;
        let mut arms = Vec::new();
        loop {
            let (line, col) = self.pos();
            while {
                self.expr()?;
                self.sym(b",")
            } {}
            self.expect_sym(b":", 86)?;
            self.expect_sym(b"(", 88)?;
            let inner = self.fields()?;
            self.expect_sym(b")", 89)?;
            let list = self.ast.list(&inner);
            arms.push(self.ast.add(Kind::Record(list), line, col));
            if !self.sym(b";") || self.is(Tk::Kw, b"END") || self.is(Tk::Sym, b")") {
                break;
            }
        }
        let list = self.ast.list(&arms);
        out.push(self.ast.add(Kind::Variant(list), line, col));
        Ok(out)
    }

    /// After `PROCEDURE` or `FUNCTION`.
    fn routine(&mut self, func: bool) -> R<Id> {
        let name = self.ident()?;
        let params = self.params()?;
        let mut result = NO_NAME;
        if func && self.sym(b":") {
            result = self.type_name()?;
        } else if func && !self.is(Tk::Sym, b";") {
            return Err(self.err(86));
        }
        let placeholder = Kind::Compound(self.ast.list(&[]));
        let id = self.after(placeholder);
        self.expect_sym(b";", 85)?;
        self.unsupported(&[b"INLINE", b"INTERRUPT"])?;
        let block = if self.kw(b"FORWARD") {
            None
        } else {
            let (line, col) = self.pos();
            let decls = self.declarations()?;
            let body = self.begin()?;
            let decls = self.ast.list(&decls);
            Some(self.ast.add(Kind::Block(decls, body), line, col))
        };
        self.expect_sym(b";", 85)?;
        self.ast.nodes[id as usize].kind = Kind::Routine(name, params, result, func, block);
        Ok(id)
    }

    /// `CONST`, `VAR`, `PROCEDURE` and `FUNCTION` sections, in any order and number.
    fn declarations(&mut self) -> R<Vec<Id>> {
        let mut out = Vec::new();
        loop {
            if self.kw(b"CONST") {
                loop {
                    let (line, col) = self.pos();
                    let name = self.ident()?;
                    self.expect_sym(b"=", 90)?;
                    let e = self.expr()?;
                    self.expect_sym(b";", 85)?;
                    out.push(self.ast.add(Kind::ConstDecl(name, e), line, col));
                    if self.tok().kind != Tk::Ident {
                        break;
                    }
                }
            } else if self.kw(b"TYPE") {
                loop {
                    let (line, col) = self.pos();
                    let name = self.ident()?;
                    self.expect_sym(b"=", 90)?;
                    let spec = self.type_spec()?;
                    self.expect_sym(b";", 85)?;
                    out.push(self.ast.add(Kind::TypeDecl(name, spec), line, col));
                    if self.tok().kind != Tk::Ident {
                        break;
                    }
                }
            } else if self.kw(b"VAR") {
                loop {
                    let mut names = Vec::new();
                    loop {
                        let (line, col) = self.pos();
                        names.push((self.ident()?, line, col));
                        if !self.sym(b",") {
                            break;
                        }
                    }
                    self.expect_sym(b":", 86)?;
                    let named = self.tok().kind == Tk::Ident
                        && self.toks[self.at + 1].kind == Tk::Sym
                        && self.toks[self.at + 1].text == b";";
                    let ty = if !named {
                        // A hidden type name no identifier can have.
                        let n = self.ast.names.len() as u32;
                        let key = [b' ', 0x80 | (n & 0x7F) as u8, 0x80 | (n >> 7) as u8];
                        let (line, col) = self.pos();
                        let name = self.ast.intern(&key);
                        let spec = self.type_spec()?;
                        out.push(self.ast.add(Kind::TypeDecl(name, spec), line, col));
                        name
                    } else {
                        self.type_name()?
                    };
                    self.unsupported(&[b"ABSOLUTE"])?;
                    self.expect_sym(b";", 85)?;
                    for (name, line, col) in names {
                        out.push(self.ast.add(Kind::VarDecl(name, ty), line, col));
                    }
                    if self.tok().kind != Tk::Ident {
                        break;
                    }
                }
            } else if self.kw(b"PROCEDURE") || self.kw(b"FUNCTION") {
                let func = self.toks[self.at - 1].text == b"FUNCTION";
                out.push(self.routine(func)?);
            } else {
                self.unsupported(&[b"LABEL"])?;
                return Ok(out);
            }
        }
    }

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
        self.expect_sym(b")", 89)?;
        Ok((args, close))
    }

    /// Index and field selectors after the designator `e` that starts at `(line, col)`.
    fn selectors(&mut self, mut e: Id) -> R<Id> {
        let (line, col) = (
            self.ast.nodes[e as usize].line,
            self.ast.nodes[e as usize].col,
        );
        loop {
            if self.sym(b"[") {
                let mut more = false;
                loop {
                    let index = self.expr()?;
                    e = self.ast.add(Kind::Index(e, index, more), line, col);
                    more = true;
                    if !self.sym(b",") {
                        break;
                    }
                }
                self.expect_sym(b"]", 93)?;
            } else if self.sym(b"^") {
                e = self.ast.add(Kind::Deref(e), line, col);
            } else if self.sym(b".") {
                if self.tok().kind != Tk::Ident {
                    return Err(self.err(44));
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
        let name = self.ident()?;
        let base = self.ast.add(Kind::Name(name), line, col);
        self.selectors(base)
    }

    fn statement(&mut self) -> R<Option<Id>> {
        let (line, col) = self.pos();
        if self.tok().kind == Tk::Ident {
            let name = self.ident()?;
            let mut target = self.ast.add(Kind::Name(name), line, col);
            target = self.selectors(target)?;
            if self.sym(b":=") {
                let value = self.expr()?;
                return Ok(Some(self.after(Kind::Assign(target, value))));
            }
            if !matches!(self.ast.nodes[target as usize].kind, Kind::Name(_)) {
                return Err(self.err(113));
            }
            let (mut args, mut at) = (Vec::new(), self.pos());
            if self.sym(b"(") {
                (args, at) = self.arguments()?;
            } else if self.is(Tk::Sym, b"=") {
                // TP takes the identifier for a variable and wants `:=`.
                return Err(self.err(91));
            }
            let list = self.ast.list(&args);
            let call = self.ast.add(Kind::Call(target, list), at.0, at.1);
            if self.is(Tk::Sym, b"^") {
                // A typecast designator: `T(x)^ := v`.
                let target = self.selectors(call)?;
                self.expect_sym(b":=", 113)?;
                let value = self.expr()?;
                return Ok(Some(self.after(Kind::Assign(target, value))));
            }
            return Ok(Some(call));
        }
        if self.kw(b"BEGIN") {
            return self.compound().map(Some);
        }
        if let Some(s) = self.control()? {
            return Ok(Some(s));
        }
        let t = self.tok();
        let stop = [(END, &b"END"[..]), (UNTIL, b"UNTIL"), (ELSE, b"ELSE")];
        let stops = stop
            .iter()
            .any(|&(bit, k)| self.stops & bit != 0 && t.text == k);
        if t.kind == Tk::Kw && stops || self.is(Tk::Sym, b";") {
            return Ok(None);
        }
        self.unsupported(&[b"GOTO", b"ASM", b"INLINE"])?;
        Err(self.err(113))
    }

    /// `expr [: width]`.
    fn argument(&mut self) -> R<Id> {
        let e = self.expr()?;
        if !self.sym(b":") {
            return Ok(e);
        }
        let w = self.expr()?;
        Ok(self.after(Kind::Fmt(e, w)))
    }

    /// Consumes the lookahead if it is a keyword or symbol listed in `ops`.
    fn operator(&mut self, ops: &[(&[u8], Op)]) -> Option<Op> {
        let t = self.tok();
        let op = ops
            .iter()
            .find(|o| o.0 == t.text && matches!(t.kind, Tk::Kw | Tk::Sym));
        let op = op.map(|o| o.1);
        self.at += op.is_some() as usize;
        op
    }

    fn expr(&mut self) -> R<Id> {
        let left = self.simple()?;
        let Some(op) = self.operator(REL) else {
            return Ok(left);
        };
        let right = self.simple()?;
        Ok(self.after(Kind::Binary(op, left, right)))
    }

    /// `[sign] term {addop term}`; the sign applies to the first term.
    fn simple(&mut self) -> R<Id> {
        let neg = self.sym(b"-");
        self.sym(b"+");
        let mut left = self.term()?;
        if neg {
            left = self.after(Kind::Unary(Op::Neg, left));
        }
        while let Some(op) = self.operator(ADD) {
            let right = self.term()?;
            left = self.after(Kind::Binary(op, left, right));
        }
        Ok(left)
    }

    fn term(&mut self) -> R<Id> {
        let mut left = self.factor()?;
        while let Some(op) = self.operator(MUL) {
            let right = self.factor()?;
            left = self.after(Kind::Binary(op, left, right));
        }
        Ok(left)
    }

    fn factor(&mut self) -> R<Id> {
        // TP accepts a sign before any operand, not only the first term.
        let neg = self.sym(b"-");
        self.sym(b"+");
        if neg {
            let e = self.factor()?;
            return Ok(self.after(Kind::Unary(Op::Neg, e)));
        }
        let (line, col) = self.pos();
        let t = self.tok();
        let kind = match t.kind {
            Tk::Ident => {
                let name = self.ident()?;
                let callee = self.ast.add(Kind::Name(name), line, col);
                if !self.sym(b"(") {
                    self.ast.follow.push((callee, self.tok().line));
                    return self.selectors(callee);
                }
                let (args, at) = self.arguments()?;
                let list = self.ast.list(&args);
                let call = self.ast.add(Kind::Call(callee, list), at.0, at.1);
                return if self.is(Tk::Sym, b"^") {
                    self.selectors(call)
                } else {
                    Ok(call)
                };
            }
            Tk::IntLit if t.int > i32::MAX as i64 => return Err(self.err(7)),
            Tk::IntLit => {
                let v = t.int;
                self.at += 1;
                Kind::IntLit(v)
            }
            Tk::StrLit | Tk::Ctrl => {
                let bytes = if t.kind == Tk::Ctrl {
                    vec![t.text[0].to_ascii_uppercase() ^ 64]
                } else {
                    t.text.clone()
                };
                self.at += 1;
                self.ast.strs.push(bytes);
                Kind::StrLit(self.ast.strs.len() as u32 - 1)
            }
            Tk::Sym if t.text == b"[" => {
                self.at += 1;
                let mut items = Vec::new();
                while !self.is(Tk::Sym, b"]") {
                    let lo = self.expr()?;
                    items.push(match self.sym(b"..") {
                        true => {
                            let hi = self.expr()?;
                            self.ast.add(Kind::Range(lo, hi), line, col)
                        }
                        false => lo,
                    });
                    if !self.sym(b",") {
                        break;
                    }
                }
                self.expect_sym(b"]", 93)?;
                let list = self.ast.list(&items);
                return Ok(self.ast.add(Kind::SetLit(list), line, col));
            }
            Tk::Sym if t.text == b"(" => {
                self.at += 1;
                let e = self.expr()?;
                self.expect_sym(b")", 89)?;
                if matches!(self.ast.nodes[e as usize].kind, Kind::Name(_)) {
                    return Ok(self.ast.add(Kind::Paren(e), line, col));
                }
                return Ok(e);
            }
            Tk::Kw if t.text == b"STRING" => {
                let name = self.type_name()?;
                return Ok(self.ast.add(Kind::Name(name), line, col));
            }
            Tk::Sym if t.text == b"@" => {
                self.at += 1;
                let e = self.factor()?;
                return Ok(self.ast.add(Kind::AddrOf(e), line, col));
            }
            Tk::Kw if t.text == b"NIL" => {
                self.at += 1;
                let nil = self.ast.intern(b"NIL");
                let e = self.ast.add(Kind::Name(nil), line, col);
                return self.selectors(e);
            }
            Tk::Kw if t.text == b"NOT" => {
                self.at += 1;
                let e = self.factor()?;
                return Ok(self.after(Kind::Unary(Op::Not, e)));
            }
            Tk::RealLit => return Err(self.err(200)),
            _ => return Err(self.err(42)),
        };
        Ok(self.ast.add(kind, line, col))
    }
}
