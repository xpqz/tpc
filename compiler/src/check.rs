//! Whole-program checker: declarations, name resolution, typing and constant folding.
use crate::ast::{Ast, Id, Kind, List, Op, NO_NAME};
use crate::driver::Diag;
use crate::types::{Routine, Sym, SymKind, Symbol, Tag, Ty, Types};

/// Static data: globals start here, after the codegen's fixed header.
pub const GLOBALS: u32 = 280;

pub struct Info {
    pub types: Types,
    pub syms: Vec<Symbol>,
    pub routines: Vec<Routine>,
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
    (b"ORD", SymKind::Intrinsic, Ty::NONE, 2),
    (b"CHR", SymKind::Intrinsic, Ty::NONE, 3),
    (b"SUCC", SymKind::Intrinsic, Ty::NONE, 4),
    (b"PRED", SymKind::Intrinsic, Ty::NONE, 5),
    (b"ODD", SymKind::Intrinsic, Ty::NONE, 6),
    (b"ABS", SymKind::Intrinsic, Ty::NONE, 7),
    (b"SQR", SymKind::Intrinsic, Ty::NONE, 8),
    (b"EXIT", SymKind::Intrinsic, Ty::NONE, 9),
];

/// The operators behind the intrinsic functions, by symbol value.
const INTRINSIC_OPS: [Op; 7] = [
    Op::Ord,
    Op::Chr,
    Op::Succ,
    Op::Pred,
    Op::Odd,
    Op::Abs,
    Op::Sqr,
];

struct Cx<'a> {
    ast: &'a mut Ast,
    types: Types,
    syms: Vec<Symbol>,
    scope: Vec<(u32, Sym)>,
    /// Scope entries below this index are predefined and may be redeclared.
    user: usize,
    end: u32,
    routines: Vec<Routine>,
    /// The routine being checked and the next free local index.
    cur: Option<usize>,
    locals: u32,
}

pub fn check(ast: &mut Ast, root: Id) -> R<Info> {
    let mut cx = Cx {
        ast,
        types: Types::new(),
        syms: Vec::new(),
        scope: Vec::new(),
        user: 0,
        end: GLOBALS,
        routines: Vec::new(),
        cur: None,
        locals: 0,
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
    if cx.routines.iter().any(|r| !r.defined) {
        return Err(cx.err(root, 59));
    }
    cx.stmt(body)?;
    Ok(Info {
        types: cx.types,
        syms: cx.syms,
        routines: cx.routines,
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
        Op::Ord => x,
        Op::Chr => x & 255,
        Op::Succ => x.wrapping_add(1),
        Op::Pred => x.wrapping_sub(1),
        Op::Odd => x & 1,
        Op::Abs => x.wrapping_abs(),
        Op::Sqr => x.wrapping_mul(x),
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
                let ty = self.type_of(d, type_name)?;
                let sym = if self.cur.is_some() {
                    self.locals += 1;
                    Symbol {
                        kind: SymKind::Local,
                        ty,
                        val: self.locals as i64 - 1,
                    }
                } else {
                    let val = self.end as i64;
                    self.end += self.types.ord(ty).2 as u32;
                    Symbol {
                        kind: SymKind::Var,
                        ty,
                        val,
                    }
                };
                self.declare(d, name, sym)?;
            }
            Kind::Routine(..) => self.routine(d)?,
            _ => unreachable!("declarations only"),
        }
        Ok(())
    }

    fn type_of(&self, at: Id, name: u32) -> R<Ty> {
        match self.lookup(name).map(|s| self.syms[s.0 as usize]) {
            None => Err(self.err(at, 3)),
            Some(s) if s.kind == SymKind::Type => Ok(s.ty),
            Some(s) if s.kind == SymKind::Omitted => Err(self.err(at, 200)),
            Some(_) => Err(self.err(at, 12)),
        }
    }

    /// A procedure or function declaration, with its body when it is not `forward`.
    fn routine(&mut self, d: Id) -> R<()> {
        let Kind::Routine(name, params, result, func, block) = self.ast.nodes[d as usize].kind
        else {
            unreachable!("routine declaration")
        };
        if self.cur.is_some() {
            return Err(self.err(d, 200));
        }
        let (mut tys, mut names) = (Vec::new(), Vec::new());
        for p in params.map_or(Vec::new(), |l| self.ast.items(l).to_vec()) {
            let Kind::Param(n, t) = self.ast.nodes[p as usize].kind else {
                unreachable!("the parser lists parameters")
            };
            tys.push(self.type_of(p, t)?);
            names.push(n);
        }
        let rty = if result == NO_NAME {
            None
        } else {
            Some(self.type_of(d, result)?)
        };
        let known = self.scope[self.user..]
            .iter()
            .find(|e| e.0 == name)
            .map(|e| e.1);
        let r = match known.map(|s| self.syms[s.0 as usize]) {
            None => {
                if func && rty.is_none() {
                    return Err(self.err(d, 86));
                }
                let (result, defined) = (rty.unwrap_or(Ty::NONE), false);
                let (params, body, nlocals) = (tys.clone(), 0, 0);
                self.routines.push(Routine {
                    params,
                    names,
                    result,
                    func,
                    defined,
                    body,
                    nlocals,
                });
                let val = self.routines.len() as i64 - 1;
                let kind = SymKind::Routine;
                self.declare(
                    d,
                    name,
                    Symbol {
                        kind,
                        ty: Ty::NONE,
                        val,
                    },
                )?;
                self.routines.len() - 1
            }
            Some(s) if s.kind == SymKind::Routine => {
                let r = &self.routines[s.val as usize];
                if r.defined || r.func != func {
                    return Err(self.err(d, 4));
                }
                if block.is_none() {
                    return Err(self.err(d, 36));
                }
                if params.is_some() && tys != r.params || rty.is_some_and(|t| t != r.result) {
                    return Err(self.err(d, 131));
                }
                if params.is_some() {
                    self.routines[s.val as usize].names = names;
                }
                s.val as usize
            }
            Some(_) => return Err(self.err(d, 4)),
        };
        let Some(block) = block else {
            return Ok(());
        };
        let Kind::Block(decls, body) = self.ast.nodes[block as usize].kind else {
            unreachable!("the parser makes a block")
        };
        self.routines[r].defined = true;
        let (params, names) = (
            self.routines[r].params.clone(),
            self.routines[r].names.clone(),
        );
        let (scope_len, user) = (self.scope.len(), self.user);
        (self.user, self.cur) = (scope_len, Some(r));
        self.locals = params.len() as u32 + func as u32;
        for (i, (&n, &ty)) in names.iter().zip(&params).enumerate() {
            let sym = Symbol {
                kind: SymKind::Local,
                ty,
                val: i as i64,
            };
            self.declare(block, n, sym)?;
        }
        for decl in self.ast.items(decls).to_vec() {
            self.decl(decl)?;
        }
        self.stmt(body)?;
        self.routines[r].body = body;
        self.routines[r].nlocals = self.locals - params.len() as u32;
        self.scope.truncate(scope_len);
        (self.user, self.cur) = (user, None);
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
            Kind::If(cond, then, other) => {
                self.condition(id, cond)?;
                self.stmt(then)?;
                if let Some(s) = other {
                    self.stmt(s)?;
                }
            }
            Kind::While(cond, body) => {
                self.condition(id, cond)?;
                self.stmt(body)?;
            }
            Kind::Repeat(body, cond) => {
                for s in self.ast.items(body).to_vec() {
                    self.stmt(s)?;
                }
                self.condition(id, cond)?;
            }
            Kind::Case(selector, arms, other) => {
                self.expr(selector)?;
                let Some(tag) = self.types.tag(self.ast.nodes[selector as usize].ty) else {
                    return Err(self.err(id, 39));
                };
                for arm in self.ast.items(arms).to_vec() {
                    let Kind::Arm(labels, body) = self.ast.nodes[arm as usize].kind else {
                        unreachable!("the parser lists arms")
                    };
                    for label in self.ast.items(labels).to_vec() {
                        self.label(label, tag)?;
                    }
                    self.stmt(body)?;
                }
                for s in self.ast.items(other).to_vec() {
                    self.stmt(s)?;
                }
            }
            Kind::For(var, first, last, body, _) => {
                let Kind::Name(name) = self.ast.nodes[var as usize].kind else {
                    unreachable!("the parser names the control variable")
                };
                let (sym, s) = self.resolve(var, name)?;
                if !matches!(s.kind, SymKind::Var | SymKind::Local) {
                    return Err(self.err(var, 20));
                }
                self.ast.nodes[var as usize].kind = Kind::Var(sym);
                self.ast.nodes[var as usize].ty = s.ty;
                for bound in [first, last] {
                    self.expr(bound)?;
                    self.assignable(id, s.ty, bound)?;
                }
                self.stmt(body)?;
            }
            Kind::Assign(target, value) => {
                let Kind::Name(name) = self.ast.nodes[target as usize].kind else {
                    unreachable!("the parser assigns to names")
                };
                let (sym, s) = self.resolve(target, name)?;
                let (kind, ty) = match s.kind {
                    SymKind::Var | SymKind::Local => (Kind::Var(sym), s.ty),
                    SymKind::Routine
                        if self.cur == Some(s.val as usize)
                            && self.routines[s.val as usize].func =>
                    {
                        (Kind::Result, self.routines[s.val as usize].result)
                    }
                    SymKind::Routine if self.routines[s.val as usize].func => {
                        return Err(self.err(target, 122));
                    }
                    SymKind::Routine => return Err(self.err(target, 85)),
                    _ => return Err(self.err(target, 113)),
                };
                self.ast.nodes[target as usize].kind = kind;
                self.ast.nodes[target as usize].ty = ty;
                self.expr(value)?;
                self.assignable(id, ty, value)?;
            }
            Kind::Call(callee, args) => {
                let Kind::Name(name) = self.ast.nodes[callee as usize].kind else {
                    unreachable!("the parser names the callee")
                };
                let (_, s) = self.resolve(callee, name)?;
                let kind = match s.kind {
                    SymKind::Intrinsic if s.val <= 1 => {
                        for a in self.ast.items(args).to_vec() {
                            self.write_arg(a)?;
                        }
                        Kind::Write(args, s.val == 1)
                    }
                    SymKind::Intrinsic if s.val == 9 => {
                        if !self.ast.items(args).is_empty() {
                            return Err(self.err(id, 85));
                        }
                        Kind::Exit
                    }
                    SymKind::Routine if !self.routines[s.val as usize].func => {
                        self.arguments(id, s.val as usize, args, false)?;
                        Kind::Invoke(s.val as u32, args)
                    }
                    SymKind::Routine => return Err(self.err(id, 122)),
                    SymKind::Var | SymKind::Local => return Err(self.err(callee, 91)),
                    _ => return Err(self.err(callee, 113)),
                };
                self.ast.nodes[id as usize].kind = kind;
            }
            _ => unreachable!("statements only"),
        }
        Ok(())
    }

    /// Checks the arguments of a call of routine `r` against its parameters; `at` is the
    /// call. A routine without parameters takes no parentheses.
    fn arguments(&mut self, at: Id, r: usize, args: List, in_expr: bool) -> R<()> {
        let items = self.ast.items(args).to_vec();
        let params = self.routines[r].params.clone();
        let code = match (params.len(), items.len()) {
            (0, 0) => 0,
            (0, _) => 85 + 4 * in_expr as u16,
            (_, 0) => 88,
            (p, a) if a < p => 87,
            (p, a) if a > p => 89,
            _ => 0,
        };
        if code != 0 {
            return Err(self.err(at, code));
        }
        for (&arg, &ty) in items.iter().zip(&params) {
            if matches!(self.ast.nodes[arg as usize].kind, Kind::Fmt(..)) {
                return Err(self.err(at, 89));
            }
            self.expr(arg)?;
            self.assignable(at, ty, arg)?;
        }
        Ok(())
    }

    /// A case label range: constants of the selector's family, within Integer.
    fn label(&mut self, label: Id, tag: Tag) -> R<()> {
        let Kind::Range(lo, hi) = self.ast.nodes[label as usize].kind else {
            unreachable!("the parser lists ranges")
        };
        for end in if lo == hi { vec![lo] } else { vec![lo, hi] } {
            self.expr(end)?;
            let n = self.ast.nodes[end as usize];
            let Kind::IntLit(v) = n.kind else {
                return Err(self.err(label, 133));
            };
            if self.types.tag(n.ty) != Some(tag) {
                return Err(self.err(label, 74));
            }
            if tag == Tag::Int && !(-32768..=32767).contains(&v) {
                return Err(self.err(label, 112));
            }
        }
        Ok(())
    }

    /// `at` is the statement owning the Boolean condition.
    fn condition(&mut self, at: Id, cond: Id) -> R<()> {
        self.expr(cond)?;
        match self.ast.nodes[cond as usize].ty {
            Ty::BOOLEAN => Ok(()),
            _ => Err(self.err(at, 40)),
        }
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
                    SymKind::Var | SymKind::Local => (Kind::Var(sym), s.ty),
                    SymKind::Routine => {
                        let r = &self.routines[s.val as usize];
                        let (func, result) = (r.func, r.result);
                        if !func {
                            return Err(self.err(e, 143));
                        }
                        let none = self.ast.list(&[]);
                        self.arguments(e, s.val as usize, none, true)?;
                        (Kind::Invoke(s.val as u32, none), result)
                    }
                    SymKind::Intrinsic if (2..=8).contains(&s.val) => {
                        return Err(self.err(e, 88));
                    }
                    _ => return Err(self.err(e, 42)),
                }
            }
            Kind::Call(callee, args) => {
                let Kind::Name(name) = self.ast.nodes[callee as usize].kind else {
                    unreachable!("the parser names the callee")
                };
                let (_, s) = self.resolve(callee, name)?;
                match s.kind {
                    SymKind::Routine => {
                        let r = &self.routines[s.val as usize];
                        let (func, result) = (r.func, r.result);
                        if !func {
                            return Err(self.err(callee, 143));
                        }
                        self.arguments(e, s.val as usize, args, true)?;
                        (Kind::Invoke(s.val as u32, args), result)
                    }
                    SymKind::Intrinsic if (2..=8).contains(&s.val) => {
                        let items = self.ast.items(args).to_vec();
                        if items.len() > 1 {
                            return Err(self.err(e, 89));
                        }
                        self.unary(e, INTRINSIC_OPS[s.val as usize - 2], items[0])?
                    }
                    SymKind::Intrinsic => return Err(self.err(callee, 42)),
                    _ => return Err(self.err(e, 89)),
                }
            }
            Kind::Unary(op, a) => self.unary(e, op, a)?,
            Kind::Binary(op, a, b) => {
                self.expr(a)?;
                self.expr(b)?;
                let (na, nb) = (self.ast.nodes[a as usize], self.ast.nodes[b as usize]);
                let ty = self
                    .types
                    .binary(op, na.ty, nb.ty)
                    .map_err(|c| self.err(e, c))?;
                match (na.kind, nb.kind) {
                    (Kind::IntLit(_), Kind::IntLit(0)) if matches!(op, Op::Div | Op::Mod) => {
                        return Err(self.err(e, 62));
                    }
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

    /// A unary operator or intrinsic applied to `a`, folded when `a` is constant (`Sqr` is
    /// not constant in TP).
    fn unary(&mut self, e: Id, op: Op, a: Id) -> R<(Kind, Ty)> {
        self.expr(a)?;
        let na = self.ast.nodes[a as usize];
        let ty = self.types.unary(op, na.ty).map_err(|c| self.err(e, c))?;
        Ok(match na.kind {
            Kind::IntLit(v) if op == Op::Not && ty == Ty::BOOLEAN => self.constant(v ^ 1, ty),
            Kind::IntLit(v) if op != Op::Sqr => self.constant(fold(op, v, 0), ty),
            _ => (Kind::Unary(op, a), ty),
        })
    }

    /// A folded constant: Boolean stays Boolean, an integer takes the smallest type.
    fn constant(&self, v: i64, ty: Ty) -> (Kind, Ty) {
        let ty = if ty == Ty::BOOLEAN || ty == Ty::CHAR {
            ty
        } else {
            self.types.const_ty(v)
        };
        (Kind::IntLit(v), ty)
    }
}
