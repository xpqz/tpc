//! Whole-program checker: declarations, name resolution, typing and constant folding.
use crate::ast::{Ast, Id, Kind, Op, NO_NAME};
use crate::driver::Diag;
use crate::types::{Sym, SymKind, Symbol, Tag, Ty, Types};

/// Static data: globals start here, after the codegen's fixed header.
pub const GLOBALS: u32 = 280;

pub struct Info {
    pub types: Types,
    pub syms: Vec<Symbol>,
    /// First address after the globals.
    pub data_end: u32,
}

type R<T> = Result<T, Diag>;

const PREDEFINED: &[(&[u8], SymKind, Ty, i64)] = &[
    (b"BYTE", SymKind::Type, Ty::BYTE, 0),
    (b"SHORTINT", SymKind::Type, Ty::SHORTINT, 0),
    (b"INTEGER", SymKind::Type, Ty::INTEGER, 0),
    (b"WORD", SymKind::Type, Ty::WORD, 0),
    (b"LONGINT", SymKind::Type, Ty::LONGINT, 0),
    (b"BOOLEAN", SymKind::Type, Ty::BOOLEAN, 0),
    (b"CHAR", SymKind::Type, Ty::CHAR, 0),
    (b"REAL", SymKind::Omitted, Ty::NONE, 0),
    (b"SINGLE", SymKind::Omitted, Ty::NONE, 0),
    (b"DOUBLE", SymKind::Omitted, Ty::NONE, 0),
    (b"EXTENDED", SymKind::Omitted, Ty::NONE, 0),
    (b"COMP", SymKind::Omitted, Ty::NONE, 0),
    (b"FALSE", SymKind::Const, Ty::BOOLEAN, 0),
    (b"TRUE", SymKind::Const, Ty::BOOLEAN, 1),
    (b"WRITE", SymKind::Intrinsic, Ty::NONE, 0),
    (b"WRITELN", SymKind::Intrinsic, Ty::NONE, 1),
];

struct Cx<'a> {
    ast: &'a mut Ast,
    types: Types,
    syms: Vec<Symbol>,
    scope: Vec<(u32, Sym)>,
    /// Scope entries below this index are predefined and may be redeclared.
    user: usize,
    end: u32,
}

pub fn check(ast: &mut Ast, root: Id) -> R<Info> {
    let mut cx = Cx {
        ast,
        types: Types::new(),
        syms: Vec::new(),
        scope: Vec::new(),
        user: 0,
        end: GLOBALS,
    };
    for &(name, kind, ty, val) in PREDEFINED {
        let name = cx.ast.intern(name);
        cx.push(name, Symbol { kind, ty, val });
    }
    cx.user = cx.scope.len();
    let Kind::Program(name, decls, body) = cx.ast.nodes[root as usize].kind else {
        unreachable!("the parser's root is a program")
    };
    if name != NO_NAME {
        cx.declare(
            root,
            name,
            Symbol {
                kind: SymKind::Prog,
                ty: Ty::NONE,
                val: 0,
            },
        )?;
    }
    for d in cx.ast.items(decls).to_vec() {
        cx.decl(d)?;
    }
    cx.stmt(body)?;
    Ok(Info {
        types: cx.types,
        syms: cx.syms,
        data_end: cx.end,
    })
}

/// Value of a constant operator, computed at LongInt precision.
fn fold(op: Op, a: i64, b: i64) -> i64 {
    let (x, y) = (a as i32, b as i32);
    let v = match op {
        Op::Add => x.wrapping_add(y),
        Op::Sub => x.wrapping_sub(y),
        Op::Mul => x.wrapping_mul(y),
        Op::Div => x.wrapping_div(y),
        Op::Mod => x.wrapping_rem(y),
        Op::And => x & y,
        Op::Or => x | y,
        Op::Xor => x ^ y,
        Op::Shl => x.wrapping_shl(y as u32),
        Op::Shr => (x as u32).wrapping_shr(y as u32) as i32,
        Op::Eq => (x == y) as i32,
        Op::Ne => (x != y) as i32,
        Op::Lt => (x < y) as i32,
        Op::Gt => (x > y) as i32,
        Op::Le => (x <= y) as i32,
        Op::Ge => (x >= y) as i32,
        Op::Neg => x.wrapping_neg(),
        Op::Not => !x,
    };
    v as i64
}

impl Cx<'_> {
    fn push(&mut self, name: u32, s: Symbol) -> Sym {
        self.syms.push(s);
        let sym = Sym(self.syms.len() as u32 - 1);
        self.scope.push((name, sym));
        sym
    }

    fn lookup(&self, name: u32) -> Option<Sym> {
        self.scope.iter().rev().find(|e| e.0 == name).map(|e| e.1)
    }

    fn err(&self, id: Id, code: u16) -> Diag {
        let n = self.ast.nodes[id as usize];
        Diag {
            code,
            line: n.line,
            col: n.col,
        }
    }

    fn declare(&mut self, at: Id, name: u32, s: Symbol) -> R<Sym> {
        if self.scope[self.user..].iter().any(|e| e.0 == name) {
            return Err(self.err(at, 4));
        }
        Ok(self.push(name, s))
    }

    fn decl(&mut self, d: Id) -> R<()> {
        match self.ast.nodes[d as usize].kind {
            Kind::ConstDecl(name, e) => {
                self.expr(e)?;
                let n = self.ast.nodes[e as usize];
                let (val, ty) = match n.kind {
                    Kind::IntLit(v) => (v, n.ty),
                    Kind::StrLit(i) => (i as i64, Ty::STRING),
                    _ => return Err(self.err(e, 133)),
                };
                self.declare(
                    d,
                    name,
                    Symbol {
                        kind: SymKind::Const,
                        ty,
                        val,
                    },
                )?;
            }
            Kind::VarDecl(name, type_name) => {
                let ty = match self.lookup(type_name).map(|s| self.syms[s.0 as usize]) {
                    None => return Err(self.err(d, 3)),
                    Some(s) if s.kind == SymKind::Type => s.ty,
                    Some(s) if s.kind == SymKind::Omitted => return Err(self.err(d, 200)),
                    Some(_) => return Err(self.err(d, 12)),
                };
                let val = self.end as i64;
                self.declare(
                    d,
                    name,
                    Symbol {
                        kind: SymKind::Var,
                        ty,
                        val,
                    },
                )?;
                self.end += self.types.ord(ty).2 as u32;
            }
            _ => unreachable!("declarations only"),
        }
        Ok(())
    }

    fn resolve(&self, id: Id, name: u32) -> R<(Sym, Symbol)> {
        let s = self.lookup(name).ok_or(self.err(id, 3))?;
        Ok((s, self.syms[s.0 as usize]))
    }

    fn stmt(&mut self, id: Id) -> R<()> {
        match self.ast.nodes[id as usize].kind {
            Kind::Compound(l) => {
                for s in self.ast.items(l).to_vec() {
                    self.stmt(s)?;
                }
            }
            Kind::Assign(target, value) => {
                let Kind::Name(name) = self.ast.nodes[target as usize].kind else {
                    unreachable!("the parser assigns to names")
                };
                let (sym, s) = self.resolve(target, name)?;
                if s.kind != SymKind::Var {
                    return Err(self.err(target, 113));
                }
                self.ast.nodes[target as usize].kind = Kind::Var(sym);
                self.ast.nodes[target as usize].ty = s.ty;
                self.expr(value)?;
                self.assignable(id, s.ty, value)?;
            }
            Kind::Call(name, args) => {
                let (_, s) = self.resolve(id, name)?;
                if s.kind != SymKind::Intrinsic {
                    return Err(self.err(id, 91));
                }
                for a in self.ast.items(args).to_vec() {
                    self.write_arg(a)?;
                }
                self.ast.nodes[id as usize].kind = Kind::Write(args, s.val == 1);
            }
            _ => unreachable!("statements only"),
        }
        Ok(())
    }

    /// `at` is the assignment; a constant must also fit the destination's range.
    fn assignable(&self, at: Id, dest: Ty, value: Id) -> R<()> {
        let v = self.ast.nodes[value as usize];
        match (self.types.tag(dest), self.types.tag(v.ty)) {
            (Some(a), Some(b)) if a == b => {}
            _ => return Err(self.err(at, 26)),
        }
        match v.kind {
            Kind::IntLit(c) if self.types.tag(dest) == Some(Tag::Int) => {
                let (lo, hi, ..) = self.types.ord(dest);
                if c < lo || c > hi {
                    return Err(self.err(at, 76));
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn write_arg(&mut self, a: Id) -> R<()> {
        let mut value = a;
        if let Kind::Fmt(v, w) = self.ast.nodes[a as usize].kind {
            value = v;
            self.expr(w)?;
            if self.types.tag(self.ast.nodes[w as usize].ty) != Some(Tag::Int) {
                return Err(self.err(a, 38));
            }
        }
        self.expr(value)?;
        let n = self.ast.nodes[value as usize];
        if n.ty == Ty::STRING && !matches!(n.kind, Kind::StrLit(_)) {
            return Err(self.err(value, 200));
        }
        Ok(())
    }

    fn expr(&mut self, e: Id) -> R<()> {
        let n = self.ast.nodes[e as usize];
        let (kind, ty) = match n.kind {
            Kind::IntLit(v) => (n.kind, self.types.const_ty(v)),
            Kind::StrLit(i) => match self.ast.strs[i as usize][..] {
                [c] => (Kind::IntLit(c as i64), Ty::CHAR),
                _ => (n.kind, Ty::STRING),
            },
            Kind::Name(name) => {
                let (sym, s) = self.resolve(e, name)?;
                match s.kind {
                    SymKind::Const if s.ty == Ty::STRING => (Kind::StrLit(s.val as u32), s.ty),
                    SymKind::Const => (Kind::IntLit(s.val), s.ty),
                    SymKind::Var => (Kind::Var(sym), s.ty),
                    _ => return Err(self.err(e, 42)),
                }
            }
            Kind::Unary(op, a) => {
                self.expr(a)?;
                let ty = self.types.unary(op, self.ast.nodes[a as usize].ty);
                let ty = ty.map_err(|c| self.err(e, c))?;
                match self.ast.nodes[a as usize].kind {
                    Kind::IntLit(v) if ty == Ty::BOOLEAN => self.constant(v ^ 1, ty),
                    Kind::IntLit(v) => self.constant(fold(op, v, 0), ty),
                    _ => (n.kind, ty),
                }
            }
            Kind::Binary(op, a, b) => {
                self.expr(a)?;
                self.expr(b)?;
                let (na, nb) = (self.ast.nodes[a as usize], self.ast.nodes[b as usize]);
                let ty = self
                    .types
                    .binary(op, na.ty, nb.ty)
                    .map_err(|c| self.err(e, c))?;
                if matches!(op, Op::Div | Op::Mod) && matches!(nb.kind, Kind::IntLit(0)) {
                    return Err(self.err(e, 62));
                }
                match (na.kind, nb.kind) {
                    (Kind::IntLit(x), Kind::IntLit(y)) if ty != Ty::STRING => {
                        self.constant(fold(op, x, y), ty)
                    }
                    _ => (n.kind, ty),
                }
            }
            _ => unreachable!("expressions only"),
        };
        self.ast.nodes[e as usize].kind = kind;
        self.ast.nodes[e as usize].ty = ty;
        Ok(())
    }

    /// A folded constant: Boolean stays Boolean, an integer takes the smallest type.
    fn constant(&self, v: i64, ty: Ty) -> (Kind, Ty) {
        let ty = if ty == Ty::BOOLEAN {
            ty
        } else {
            self.types.const_ty(v)
        };
        (Kind::IntLit(v), ty)
    }
}
