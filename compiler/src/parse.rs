//! Recursive-descent parser: `[program header;] begin statements end .`
use crate::ast::{Ast, Id, Kind, Op, NO_NAME};
use crate::driver::Diag;
use crate::lex::{Tk, Tok};

struct Parser {
    toks: Vec<Tok>,
    at: usize,
    ast: Ast,
}

type R<T> = Result<T, Diag>;

pub fn parse(toks: Vec<Tok>) -> R<(Ast, Id)> {
    let mut p = Parser {
        toks,
        at: 0,
        ast: Ast::default(),
    };
    let (line, col) = (p.tok().line, p.tok().col);
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
    let decls = p.declarations()?;
    if !p.kw(b"BEGIN") {
        return Err(p.err(36));
    }
    let body = p.compound()?;
    p.expect_sym(b".", 94)?;
    let decls = p.ast.list(&decls);
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

    /// After `BEGIN`: statements separated by `;`, then `END`.
    fn compound(&mut self) -> R<Id> {
        let (line, col) = (self.tok().line, self.tok().col);
        let mut items = Vec::new();
        loop {
            items.extend(self.statement()?);
            if !self.sym(b";") {
                break;
            }
        }
        if !self.kw(b"END") {
            return Err(self.err(85));
        }
        let list = self.ast.list(&items);
        Ok(self.ast.add(Kind::Compound(list), line, col))
    }

    fn pos(&self) -> (u32, u32) {
        (self.tok().line, self.tok().col)
    }

    /// A node stamped with the lookahead position (see `Kind`).
    fn after(&mut self, kind: Kind) -> Id {
        let (line, col) = self.pos();
        self.ast.add(kind, line, col)
    }

    /// `CONST` and `VAR` sections, in any order and number.
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
                    let ty = self.ident()?;
                    self.expect_sym(b";", 85)?;
                    for (name, line, col) in names {
                        out.push(self.ast.add(Kind::VarDecl(name, ty), line, col));
                    }
                    if self.tok().kind != Tk::Ident {
                        break;
                    }
                }
            } else {
                return Ok(out);
            }
        }
    }

    fn statement(&mut self) -> R<Option<Id>> {
        let (line, col) = self.pos();
        if self.tok().kind == Tk::Ident {
            let name = self.ident()?;
            if self.sym(b":=") {
                let target = self.ast.add(Kind::Name(name), line, col);
                let value = self.expr()?;
                return Ok(Some(self.after(Kind::Assign(target, value))));
            }
            let mut args = Vec::new();
            if self.sym(b"(") {
                loop {
                    args.push(self.argument()?);
                    if !self.sym(b",") {
                        break;
                    }
                }
                self.expect_sym(b")", 89)?;
            } else if self.is(Tk::Sym, b"=") {
                // TP takes the identifier for a variable and wants `:=`.
                return Err(self.err(91));
            }
            let list = self.ast.list(&args);
            return Ok(Some(self.ast.add(Kind::Call(name, list), line, col)));
        }
        if self.kw(b"BEGIN") {
            return self.compound().map(Some);
        }
        if self.is(Tk::Sym, b";") || self.is(Tk::Kw, b"END") {
            return Ok(None);
        }
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
            Tk::Ident => Kind::Name(self.ident()?),
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
            Tk::Sym if t.text == b"(" => {
                self.at += 1;
                let e = self.expr()?;
                self.expect_sym(b")", 89)?;
                return Ok(e);
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
