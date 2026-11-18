//! Recursive-descent parser: `[program header;] begin statements end .`
use crate::ast::{Ast, Id, Kind};
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
    if p.kw(b"PROGRAM") {
        p.ident()?;
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
    if !p.kw(b"BEGIN") {
        return Err(p.err(36));
    }
    let root = p.compound()?;
    p.expect_sym(b".", 94)?;
    Ok((p.ast, root))
}

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

    fn statement(&mut self) -> R<Option<Id>> {
        let t = self.tok();
        let (line, col) = (t.line, t.col);
        if t.kind == Tk::Ident {
            let name = self.ident()?;
            let mut args = Vec::new();
            if self.sym(b"(") {
                loop {
                    args.push(self.expr()?);
                    if !self.sym(b",") {
                        break;
                    }
                }
                self.expect_sym(b")", 89)?;
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

    fn expr(&mut self) -> R<Id> {
        let t = self.tok();
        let (line, col) = (t.line, t.col);
        let kind = match t.kind {
            Tk::Ident => Kind::Name(self.ident()?),
            Tk::StrLit | Tk::Ctrl => {
                let t = &self.toks[self.at];
                let bytes = if t.kind == Tk::Ctrl {
                    vec![t.text[0].to_ascii_uppercase() ^ 64]
                } else {
                    t.text.clone()
                };
                self.at += 1;
                self.ast.strs.push(bytes);
                Kind::StrLit(self.ast.strs.len() as u32 - 1)
            }
            _ => return Err(self.err(42)),
        };
        Ok(self.ast.add(kind, line, col))
    }
}
