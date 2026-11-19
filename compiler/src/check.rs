//! Whole-program checker: declarations, name resolution, typing and constant folding.
use crate::ast::{Ast, Id, Kind, List, Op, NO_NAME};
use crate::driver::Diag;
use crate::types::{Routine, Sym, SymKind, Symbol, Tag, Ty, Type, Types};

/// System routines bound by handle right after System is loaded (decision 11).
const HELPERS: [&[u8]; 14] = [
    b"ABSLONG",
    b"STRCAT",
    b"STRCMP",
    b"STRASSIGN",
    b"STRDELETE",
    b"STRINSERT",
    b"HEAPGET",
    b"HEAPPUT",
    b"HEAPMARK",
    b"HEAPRELEASE",
    b"SETOP",
    b"SETADD",
    b"SETIN",
    b"SETCMP",
];
pub const H_ABS: usize = 0;
pub const H_CAT: usize = 1;
pub const H_CMP: usize = 2;
pub const H_ASSIGN: usize = 3;
const H_DELETE: usize = 4;
const H_INSERT: usize = 5;
const H_GET: usize = 6;
const H_PUT: usize = 7;
const H_MARK: usize = 8;
const H_RELEASE: usize = 9;
pub const H_SETOP: usize = 10;
pub const H_SETADD: usize = 11;
pub const H_SETIN: usize = 12;
pub const H_SETCMP: usize = 13;

/// The most bytes of one structure, of all globals, or of one routine's locals.
const MAX_DATA: u32 = 65535;

/// Static data: globals start here, after the codegen's fixed header.
pub const GLOBALS: u32 = 280;

pub struct Info {
    pub types: Types,
    pub syms: Vec<Symbol>,
    pub routines: Vec<Routine>,
    /// First address after the globals.
    pub data_end: u32,
    /// The System routines the compiler calls implicitly, by the `H_*` indexes.
    pub helpers: [u32; HELPERS.len()],
    /// Address of System's `HeapOrg`; the next 4 bytes are `HeapPtr`.
    pub heap: u32,
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
    (b"SEG", SymKind::Omitted, Ty::NONE, 0),
    (b"OFS", SymKind::Omitted, Ty::NONE, 0),
    (b"PTR", SymKind::Omitted, Ty::NONE, 0),
    (b"POINTER", SymKind::Type, Ty::POINTER, 0),
    (b"NIL", SymKind::Const, Ty::POINTER, 0),
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
    (b"SIZEOF", SymKind::Intrinsic, Ty::NONE, 10),
    (b"CONCAT", SymKind::Intrinsic, Ty::NONE, 11),
    (b"DELETE", SymKind::Intrinsic, Ty::NONE, 12),
    (b"INSERT", SymKind::Intrinsic, Ty::NONE, 13),
    (b"STRING", SymKind::Type, Ty::STRING, 0),
    (b"NEW", SymKind::Intrinsic, Ty::NONE, 14),
    (b"DISPOSE", SymKind::Intrinsic, Ty::NONE, 15),
    (b"GETMEM", SymKind::Intrinsic, Ty::NONE, 16),
    (b"FREEMEM", SymKind::Intrinsic, Ty::NONE, 17),
    (b"MARK", SymKind::Intrinsic, Ty::NONE, 18),
    (b"RELEASE", SymKind::Intrinsic, Ty::NONE, 19),
    (b"RUNERROR", SymKind::Intrinsic, Ty::NONE, 20),
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
    /// The routines being checked, outermost first; its length is the current depth.
    chain: Vec<usize>,
    /// The next free local index.
    locals: u32,
    /// Bytes of the current routine's local variables.
    local_bytes: u32,
    /// The record designators of the enclosing `with` statements, innermost last.
    withs: Vec<(Id, Ty)>,
    helpers: [u32; HELPERS.len()],
    /// Pointer types whose target is declared later in the same type section.
    forward: Vec<(Ty, u32)>,
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
        chain: Vec::new(),
        locals: 0,
        local_bytes: 0,
        withs: Vec::new(),
        helpers: [0; HELPERS.len()],
        forward: Vec::new(),
    };
    for &(name, kind, ty, val) in PREDEFINED {
        let name = cx.ast.intern(name);
        cx.push(name, Symbol::new(kind, ty, val));
    }
    let Kind::Program(name, decls, body) = cx.ast.nodes[root as usize].kind else {
        unreachable!("the parser's root is a program")
    };
    let decls = cx.ast.items(decls).to_vec();
    let (system, program) = decls.split_at(cx.ast.system);
    for &d in system {
        cx.decl(d)?;
    }
    // Implicit helpers are bound here, before any program declaration can shadow them.
    let in_system = Diag {
        code: 3,
        line: 0,
        col: 0,
    };
    let mut helpers = [0; HELPERS.len()];
    for (slot, name) in helpers.iter_mut().zip(HELPERS) {
        let name = cx.ast.intern(name);
        *slot = match cx.lookup(name).map(|s| cx.syms[s.0 as usize]) {
            Some(s) if s.kind == SymKind::Routine => s.val as u32,
            _ => return Err(in_system),
        };
    }
    cx.helpers = helpers;
    let org = cx.ast.intern(b"HEAPORG");
    let heap = match cx.lookup(org).map(|s| cx.syms[s.0 as usize]) {
        Some(s) if s.kind == SymKind::Var => s.val as u32,
        _ => return Err(in_system),
    };
    if cx.routines.iter().any(|r| !r.defined) {
        return Err(Diag {
            code: 59,
            ..in_system
        });
    }
    cx.user = cx.scope.len();
    if name != NO_NAME {
        cx.declare(root, name, Symbol::new(SymKind::Prog, Ty::NONE, 0))?;
    }
    for &d in program {
        cx.decl(d)?;
    }
    cx.resolve_forward(body)?;
    if cx.routines.iter().any(|r| !r.defined) {
        return Err(cx.err(root, 59));
    }
    cx.stmt(body)?;
    Ok(Info {
        types: cx.types,
        syms: cx.syms,
        routines: cx.routines,
        data_end: cx.end,
        helpers,
        heap,
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
        Op::In => 0,
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
        self.syms.push(Symbol {
            depth: self.chain.len() as u32,
            ..s
        });
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

    /// Resolves the pointer types declared with a target not yet declared. Called where a type
    /// section ends; TP reports an undefined target at `at`, the next declaration or the body.
    fn resolve_forward(&mut self, at: Id) -> R<()> {
        for (ty, name) in std::mem::take(&mut self.forward) {
            let to = self.type_of(at, name).map_err(|d| match d.code {
                200 => d,
                _ => Diag { code: 19, ..d },
            })?;
            self.types.v[ty.0 as usize] = Type::Ptr(to);
        }
        Ok(())
    }

    fn decl(&mut self, d: Id) -> R<()> {
        if !matches!(self.ast.nodes[d as usize].kind, Kind::TypeDecl(..)) {
            self.resolve_forward(d)?;
        }
        match self.ast.nodes[d as usize].kind {
            Kind::ConstDecl(name, e) => {
                self.expr(e)?;
                let n = self.ast.nodes[e as usize];
                let (val, ty) = match n.kind {
                    Kind::IntLit(v) => (v, n.ty),
                    Kind::StrLit(i) => (i as i64, Ty::STRING),
                    Kind::SetLit(l) if self.ast.items(l).iter().all(|&i| self.fixed(i)) => {
                        (e as i64, n.ty)
                    }
                    _ => return Err(self.err(e, 133)),
                };
                self.declare(d, name, Symbol::new(SymKind::Const, ty, val))?;
            }
            Kind::VarDecl(name, type_name) => {
                let ty = self.type_of(d, type_name)?;
                let size = self.types.size(ty);
                let sym = if !self.chain.is_empty() {
                    self.locals += 1;
                    self.local_bytes += size;
                    if self.local_bytes > MAX_DATA {
                        return Err(self.err(d, 96));
                    }
                    // Aggregates always live in the frame.
                    let taken = self.types.in_memory(ty);
                    let local = Symbol::new(SymKind::Local, ty, self.locals as i64 - 1);
                    Symbol { taken, ..local }
                } else {
                    let val = self.end as i64;
                    self.end += size;
                    if self.end - GLOBALS > MAX_DATA {
                        return Err(self.err(d, 96));
                    }
                    Symbol::new(SymKind::Var, ty, val)
                };
                self.declare(d, name, sym)?;
            }
            Kind::TypeDecl(name, spec) => {
                let ty = self.spec(spec, name)?;
                self.declare(d, name, Symbol::new(SymKind::Type, ty, 0))?;
            }
            Kind::Routine(..) => self.routine(d)?,
            _ => unreachable!("declarations only"),
        }
        Ok(())
    }

    /// The type a declaration `name = spec` defines; `type T = T` is error 21.
    fn spec(&mut self, spec: Id, name: u32) -> R<Ty> {
        match self.ast.nodes[spec as usize].kind {
            Kind::Name(n) if n == name => Err(self.err(spec, 21)),
            Kind::Name(n) => self.type_of(spec, n),
            Kind::Enum(names) => {
                let ty = self.types.enumeration(self.ast.items(names).len() as i64);
                for (i, &id) in self.ast.items(names).to_vec().iter().enumerate() {
                    let Kind::Name(n) = self.ast.nodes[id as usize].kind else {
                        unreachable!("the parser lists identifiers")
                    };
                    self.declare(id, n, Symbol::new(SymKind::Const, ty, i as i64))?;
                }
                Ok(ty)
            }
            Kind::PtrType(name) => {
                let to = self.type_of(spec, name).unwrap_or(Ty::NONE);
                let ty = self.types.pointer(to);
                if to == Ty::NONE {
                    self.forward.push((ty, name));
                }
                Ok(ty)
            }
            Kind::StrType(size) => {
                self.expr(size)?;
                let Kind::IntLit(n) = self.ast.nodes[size as usize].kind else {
                    return Err(self.err(spec, 133));
                };
                if !(1..=255).contains(&n) {
                    return Err(self.err(spec, 25));
                }
                Ok(self.types.string(n as u8))
            }
            Kind::Array(indexes, elem) => {
                let mut index = Vec::new();
                for ix in self.ast.items(indexes).to_vec() {
                    let ty = self.spec(ix, NO_NAME)?;
                    if self.types.tag(ty).is_none() {
                        return Err(self.err(ix, 29));
                    }
                    index.push(ty);
                }
                let mut ty = self.spec(elem, NO_NAME)?;
                for &ix in index.iter().rev() {
                    ty = self.types.array(ix, ty);
                    if self.types.size(ty) > MAX_DATA {
                        return Err(self.err(spec, 22));
                    }
                }
                Ok(ty)
            }
            Kind::Record(items) => {
                let mut fields = Vec::new();
                let size = self.fields(items, 0, &mut fields)?;
                if size > MAX_DATA {
                    return Err(self.err(spec, 22));
                }
                Ok(self.types.record(fields, size))
            }
            Kind::SetType(of) => {
                let base = self.spec(of, NO_NAME)?;
                self.types.set_of(base).map_err(|code| self.err(of, code))
            }
            Kind::Range(lo, hi) => {
                let mut ends = [(0, Ty::NONE); 2];
                for (end, id) in ends.iter_mut().zip([lo, hi]) {
                    self.expr(id)?;
                    let n = self.ast.nodes[id as usize];
                    let Kind::IntLit(v) = n.kind else {
                        return Err(self.err(spec, 133));
                    };
                    *end = (v, n.ty);
                }
                let [(a, ta), (b, tb)] = ends;
                if !self.types.compat(ta, tb) {
                    return Err(self.err(spec, 26));
                }
                if a > b {
                    return Err(self.err(spec, 28));
                }
                Ok(self.types.subrange(a, b, ta))
            }
            Kind::ProcType(params, result, _) => {
                let mut sig = Vec::new();
                for p in params.map_or(Vec::new(), |l| self.ast.items(l).to_vec()) {
                    let Kind::Param(_, t, byref) = self.ast.nodes[p as usize].kind else {
                        unreachable!("the parser lists parameters")
                    };
                    sig.push((self.type_of(p, t)?, byref));
                }
                let result = if result == NO_NAME {
                    Ty::NONE
                } else {
                    self.type_of(spec, result)?
                };
                if self.types.is_str(result) {
                    return Err(self.err(spec, 200));
                }
                Ok(self.types.proc(sig, result))
            }
            _ => unreachable!("the parser builds type specs"),
        }
    }

    /// Lays out the fields `items` from offset `at`, appending them to `out`; returns the end
    /// offset. The arms of a variant part all start at the end of the fixed part.
    fn fields(&mut self, items: List, at: u32, out: &mut Vec<(u32, Ty, u32)>) -> R<u32> {
        let mut end = at;
        for item in self.ast.items(items).to_vec() {
            match self.ast.nodes[item as usize].kind {
                Kind::FieldDecl(name, spec) => {
                    let ty = self.spec(spec, NO_NAME)?;
                    if name != NO_NAME {
                        if out.iter().any(|f| f.0 == name) {
                            return Err(self.err(item, 4));
                        }
                        out.push((name, ty, end));
                        end += self.types.size(ty);
                    }
                }
                Kind::Variant(arms) => {
                    let start = end;
                    for arm in self.ast.items(arms).to_vec() {
                        let Kind::Record(list) = self.ast.nodes[arm as usize].kind else {
                            unreachable!("the parser lists record arms")
                        };
                        end = end.max(self.fields(list, start, out)?);
                    }
                }
                _ => unreachable!("the parser lists fields"),
            }
        }
        Ok(end)
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
        let (mut tys, mut names, mut refs) = (Vec::new(), Vec::new(), Vec::new());
        for p in params.map_or(Vec::new(), |l| self.ast.items(l).to_vec()) {
            let Kind::Param(n, t, byref) = self.ast.nodes[p as usize].kind else {
                unreachable!("the parser lists parameters")
            };
            tys.push(self.type_of(p, t)?);
            names.push(n);
            refs.push(byref);
        }
        let rty = if result == NO_NAME {
            None
        } else {
            let ty = self.type_of(d, result)?;
            if self.types.is_agg(ty) {
                return Err(self.err(d, 34));
            }
            Some(ty)
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
                    byref: refs.clone(),
                    names,
                    result,
                    func,
                    defined,
                    body,
                    nlocals,
                    depth: self.chain.len() as u32 + 1,
                    children: false,
                    framed: false,
                    frame: 0,
                    result_sym: Sym(0),
                    result_ptr: self.types.is_str(result),
                    builtin: self.user == 0,
                    addressed: false,
                    copies: Vec::new(),
                });
                if let Some(&parent) = self.chain.last() {
                    self.routines[parent].children = true;
                }
                let val = self.routines.len() as i64 - 1;
                self.declare(d, name, Symbol::new(SymKind::Routine, Ty::NONE, val))?;
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
                if params.is_some() && (tys != r.params || refs != r.byref)
                    || rty.is_some_and(|t| t != r.result)
                {
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
        let (params, names, refs) = (
            self.routines[r].params.clone(),
            self.routines[r].names.clone(),
            self.routines[r].byref.clone(),
        );
        let first = self.syms.len();
        let (scope_len, user, outer_locals) = (self.scope.len(), self.user, self.locals);
        let outer_bytes = std::mem::take(&mut self.local_bytes);
        self.user = scope_len;
        self.chain.push(r);
        let link = self.chain.len() > 1;
        // Hidden parameters come first: the static link, then the address of a String result.
        let hidden = link as u32 + self.routines[r].result_ptr as u32;
        let nparams = params.len() as u32 + hidden;
        self.locals = nparams;
        if func {
            let ty = self.routines[r].result;
            let ret = if self.routines[r].result_ptr {
                Symbol::new(SymKind::Ref, ty, link as i64)
            } else {
                self.locals += 1;
                Symbol::new(SymKind::Local, ty, nparams as i64)
            };
            self.routines[r].result_sym = Sym(self.syms.len() as u32);
            self.syms.push(Symbol {
                depth: self.chain.len() as u32,
                ..ret
            });
        }
        for (i, (&n, &ty)) in names.iter().zip(&params).enumerate() {
            let kind = if refs[i] {
                SymKind::Ref
            } else {
                SymKind::Local
            };
            let param = Symbol::new(kind, ty, (i as u32 + hidden) as i64);
            let taken = !refs[i] && self.types.in_memory(ty);
            self.declare(block, n, Symbol { taken, ..param })?;
        }
        for decl in self.ast.items(decls).to_vec() {
            self.decl(decl)?;
        }
        self.resolve_forward(body)?;
        if self.scope[scope_len..].iter().any(|e| {
            let s = self.syms[e.1 .0 as usize];
            s.kind == SymKind::Routine && !self.routines[s.val as usize].defined
        }) {
            return Err(self.err(body, 59));
        }
        self.stmt(body)?;
        self.routines[r].body = body;
        self.routines[r].nlocals = self.locals - nparams;
        self.place_frame(r, first);
        self.chain.pop();
        self.scope.truncate(scope_len);
        (self.user, self.locals, self.local_bytes) = (user, outer_locals, outer_bytes);
        Ok(())
    }

    /// Moves the locals that are captured or passed by reference (symbols of this routine
    /// from `first` on) to the frame, in declaration order with no padding. A routine with a
    /// parent keeps the static link in the first 4 bytes. A captured `var` parameter keeps
    /// its address in the frame.
    fn place_frame(&mut self, r: usize, first: usize) {
        let depth = self.chain.len() as u32;
        let base = 4 * (depth > 1) as u32;
        let nparams = self.routines[r].params.len() as i64
            + (depth > 1) as i64
            + self.routines[r].result_ptr as i64;
        let (mut size, mut copies) = (base, Vec::new());
        for s in &mut self.syms[first..] {
            if s.depth != depth || !s.taken || !matches!(s.kind, SymKind::Local | SymKind::Ref) {
                continue;
            }
            let ty = if s.kind == SymKind::Ref {
                Ty::LONGINT
            } else {
                s.ty
            };
            if s.val < nparams {
                copies.push((s.val as u32, size, ty));
            }
            s.kind = if s.kind == SymKind::Ref {
                SymKind::FrameRef
            } else {
                SymKind::Frame
            };
            (s.val, size) = (size as i64, size + self.types.size(ty));
        }
        let r = &mut self.routines[r];
        r.framed = size > base || r.children;
        (r.frame, r.copies) = (size, copies);
    }

    /// A reference to `sym` from the current routine: a `Local` or `Ref` of an enclosing
    /// routine is captured, a `Local` passed as a `var` argument is address-taken; both move
    /// it to the frame.
    fn capture(&mut self, sym: Sym, by_ref: bool) {
        let depth = self.chain.len() as u32;
        let s = &mut self.syms[sym.0 as usize];
        let outer = s.depth < depth;
        s.taken |= matches!(s.kind, SymKind::Local | SymKind::Ref) && outer
            || by_ref && s.kind == SymKind::Local;
    }

    /// TP's error for an argument of a `var` parameter that is not a variable, decided by its
    /// leftmost token; 0 for a variable.
    fn bad_var_arg(&self, e: Id) -> u16 {
        let function =
            |name: u32, call: bool| match self.lookup(name).map(|s| self.syms[s.0 as usize]) {
                Some(s) if s.kind == SymKind::Routine && self.routines[s.val as usize].func => {
                    if call || self.routines[s.val as usize].params.is_empty() {
                        122
                    } else {
                        88
                    }
                }
                Some(s) if s.kind == SymKind::Intrinsic && (2..=8).contains(&s.val) => 122,
                _ => 20,
            };
        match self.ast.nodes[e as usize].kind {
            Kind::Binary(_, a, _) => match self.bad_var_arg(a) {
                0 => 89,
                c => c,
            },
            Kind::Call(callee, _) => match self.ast.nodes[callee as usize].kind {
                Kind::Name(name) => function(name, true),
                _ => 20,
            },
            Kind::Deref(_) => 0,
            Kind::Index(base, ..) | Kind::Field(base, _) => self.bad_var_arg(base),
            Kind::Name(name) if self.with_field(name).is_some() => 0,
            Kind::Name(name) => match self.lookup(name).map(|s| self.syms[s.0 as usize]) {
                None => 3,
                Some(s) if s.kind == SymKind::Omitted => 200,
                Some(s) if matches!(s.kind, SymKind::Var | SymKind::Local | SymKind::Ref) => 0,
                Some(_) => function(name, false),
            },
            _ => 20,
        }
    }

    /// An argument of a `var` parameter of type `ty`: a variable of that very type. `at` is
    /// the call.
    fn var_arg(&mut self, at: Id, arg: Id, ty: Ty) -> R<()> {
        match self.bad_var_arg(arg) {
            0 => {}
            code => return Err(self.err(arg, code)),
        }
        let named = match self.ast.nodes[arg as usize].kind {
            Kind::Name(name) if self.with_field(name).is_none() => Some(name),
            _ => None,
        };
        let Some(name) = named else {
            self.expr(arg)?;
            return match self.ast.nodes[arg as usize].ty == ty {
                true => Ok(()),
                false => Err(self.err(at, 26)),
            };
        };
        let (sym, s) = self.resolve(arg, name)?;
        if s.ty != ty {
            return Err(self.err(at, 26));
        }
        self.capture(sym, true);
        self.ast.nodes[arg as usize].kind = Kind::Var(sym);
        self.ast.nodes[arg as usize].ty = ty;
        Ok(())
    }

    /// The routine's signature as a procedural type.
    fn signature(&mut self, r: usize) -> Ty {
        let r = &self.routines[r];
        let sig = r.params.iter().copied().zip(r.byref.iter().copied());
        let (sig, result) = (sig.collect(), r.result);
        self.types.proc(sig, result)
    }

    /// `e` where a value of procedural type `want` is expected (assignment source, value
    /// argument): a global routine of that signature or a variable of that type. `at` is the
    /// statement or call.
    fn proc_value(&mut self, at: Id, want: Ty, e: Id) -> R<()> {
        let Kind::Name(name) = self.ast.nodes[e as usize].kind else {
            return Err(self.err(e, 20));
        };
        let (sym, s) = self.resolve(e, name)?;
        let kind = match s.kind {
            SymKind::Routine => {
                let r = s.val as usize;
                if self.routines[r].depth > 1 {
                    return Err(self.err(at, 143));
                }
                if self.signature(r) != want {
                    return Err(self.err(at, 26));
                }
                self.routines[r].addressed = true;
                Kind::ProcRef(r as u32)
            }
            SymKind::Var | SymKind::Local | SymKind::Ref => {
                if s.ty != want {
                    return Err(self.err(at, 26));
                }
                self.capture(sym, false);
                Kind::Var(sym)
            }
            _ => return Err(self.err(e, 20)),
        };
        self.ast.nodes[e as usize].kind = kind;
        self.ast.nodes[e as usize].ty = want;
        Ok(())
    }

    /// The field `name` of an enclosing `with` record: its designator, type and offset.
    fn with_field(&self, name: u32) -> Option<(Id, Ty, u32)> {
        let found = |&(d, ty): &(Id, Ty)| self.types.field(ty, name).map(|(t, off)| (d, t, off));
        self.withs.iter().rev().find_map(found)
    }

    fn resolve(&self, id: Id, name: u32) -> R<(Sym, Symbol)> {
        let s = self.lookup(name).ok_or(self.err(id, 3))?;
        let sym = self.syms[s.0 as usize];
        match sym.kind {
            SymKind::Omitted => Err(self.err(id, 200)),
            _ => Ok((s, sym)),
        }
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
                let sel = self.ast.nodes[selector as usize].ty;
                if self.types.tag(sel).is_none() {
                    return Err(self.err(id, 39));
                }
                for arm in self.ast.items(arms).to_vec() {
                    let Kind::Arm(labels, body) = self.ast.nodes[arm as usize].kind else {
                        unreachable!("the parser lists arms")
                    };
                    for label in self.ast.items(labels).to_vec() {
                        self.label(label, sel)?;
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
                let outer = s.depth < self.chain.len() as u32;
                if s.kind == SymKind::Ref
                    || s.kind == SymKind::Local && outer
                    || self.types.sig(s.ty).is_some()
                    || self.types.tag(s.ty).is_none()
                {
                    return Err(self.err(var, 97));
                }
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
                let name = match self.ast.nodes[target as usize].kind {
                    Kind::Name(name) if self.with_field(name).is_none() => name,
                    _ => {
                        self.expr(target)?;
                        let ty = self.ast.nodes[target as usize].ty;
                        return self.assign_value(id, ty, value);
                    }
                };
                let (sym, s) = self.resolve(target, name)?;
                let (kind, ty) = match s.kind {
                    SymKind::Var | SymKind::Local | SymKind::Ref => {
                        self.capture(sym, false);
                        (Kind::Var(sym), s.ty)
                    }
                    SymKind::Routine
                        if self.chain.contains(&(s.val as usize))
                            && self.routines[s.val as usize].func =>
                    {
                        let ret = self.routines[s.val as usize].result_sym;
                        self.capture(ret, false);
                        (Kind::Var(ret), self.routines[s.val as usize].result)
                    }
                    SymKind::Routine if self.routines[s.val as usize].func => {
                        return Err(self.err(target, 122));
                    }
                    SymKind::Routine => return Err(self.err(target, 85)),
                    _ => return Err(self.err(target, 113)),
                };
                self.ast.nodes[target as usize].kind = kind;
                self.ast.nodes[target as usize].ty = ty;
                self.assign_value(id, ty, value)?;
            }
            Kind::With(records, body) => {
                let outer = self.withs.len();
                for d in self.ast.items(records).to_vec() {
                    self.expr(d)?;
                    let ty = self.ast.nodes[d as usize].ty;
                    if !self.types.is_record(ty) {
                        return Err(self.err(d, 75));
                    }
                    self.withs.push((d, ty));
                }
                self.stmt(body)?;
                self.withs.truncate(outer);
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
                    SymKind::Intrinsic if s.val == 12 || s.val == 13 => {
                        self.edit_string(id, s.val == 13, args)?
                    }
                    SymKind::Intrinsic if s.val >= 14 => self.heap_call(id, s.val, args)?,
                    SymKind::Intrinsic if s.val == 9 => {
                        if !self.ast.items(args).is_empty() {
                            return Err(self.err(id, 85));
                        }
                        Kind::Exit
                    }
                    SymKind::Routine if !self.routines[s.val as usize].func => {
                        let (params, refs) = self.routine_params(s.val as usize);
                        let builtin = self.routines[s.val as usize].builtin;
                        self.arguments(id, params, refs, args, false, builtin)?;
                        Kind::Invoke(s.val as u32, args)
                    }
                    SymKind::Routine => return Err(self.err(id, 122)),
                    SymKind::Var | SymKind::Local | SymKind::Ref => match self.types.sig(s.ty) {
                        Some((params, refs, Ty::NONE)) => {
                            self.indirect(id, callee, params, refs, args, false)?
                        }
                        _ => return Err(self.err(id, 91)),
                    },
                    _ => return Err(self.err(callee, 113)),
                };
                self.ast.nodes[id as usize].kind = kind;
            }
            _ => unreachable!("statements only"),
        }
        Ok(())
    }

    /// `at` is the assignment of `value` to a target of type `ty`.
    fn assign_value(&mut self, at: Id, ty: Ty, value: Id) -> R<()> {
        if self.types.sig(ty).is_some() {
            return self.proc_value(at, ty, value);
        }
        self.expr(value)?;
        self.assignable(at, ty, value)
    }

    /// Checks the arguments of a call of routine `r` against its parameters; `at` is the
    /// call. A routine without parameters takes no parentheses.
    fn arguments(
        &mut self,
        at: Id,
        params: Vec<Ty>,
        refs: Vec<bool>,
        args: List,
        in_expr: bool,
        builtin: bool,
    ) -> R<()> {
        let items = self.ast.items(args).to_vec();
        let code = match (params.len(), items.len()) {
            (0, 0) => 0,
            (0, _) => 85 + 4 * (in_expr && !builtin) as u16,
            (_, 0) => 88,
            (p, a) if a < p => 87,
            (p, a) if a > p => 89,
            _ => 0,
        };
        if code == 88 {
            let follow = self.ast.follow.iter().find(|f| f.0 == at);
            let line = follow.map_or(self.ast.nodes[at as usize].line, |f| f.1);
            return Err(Diag {
                line,
                ..self.err(at, 88)
            });
        }
        if code != 0 {
            return Err(self.err(at, code));
        }
        for ((&arg, &ty), &byref) in items.iter().zip(&params).zip(&refs) {
            if matches!(self.ast.nodes[arg as usize].kind, Kind::Fmt(..)) {
                return Err(self.err(at, 89));
            }
            if byref {
                self.var_arg(at, arg, ty)?;
                continue;
            }
            if self.types.sig(ty).is_some() {
                self.proc_value(at, ty, arg)?;
                continue;
            }
            self.expr(arg)?;
            self.argument_assignable(at, ty, arg, builtin)?;
        }
        Ok(())
    }

    /// `assignable` for an argument; TP names the kind of value that a System routine wants.
    fn argument_assignable(&mut self, at: Id, ty: Ty, arg: Id, builtin: bool) -> R<()> {
        self.assignable(at, ty, arg).map_err(|d| {
            let code = match () {
                _ if d.code != 26 || !builtin => d.code,
                _ if self.types.is_str(ty) => 67,
                _ if self.types.tag(ty) == Some(Tag::Char) => 106,
                _ if self.types.tag(ty) == Some(Tag::Int) => 38,
                _ => 26,
            };
            Diag { code, ..d }
        })
    }

    fn routine_params(&self, r: usize) -> (Vec<Ty>, Vec<bool>) {
        (
            self.routines[r].params.clone(),
            self.routines[r].byref.clone(),
        )
    }

    /// A call through the variable named by `callee` with the signature `params`/`refs`:
    /// the variable becomes the callee value of the call node `at`.
    fn indirect(
        &mut self,
        at: Id,
        callee: Id,
        params: Vec<Ty>,
        refs: Vec<bool>,
        args: List,
        in_expr: bool,
    ) -> R<Kind> {
        let Kind::Name(name) = self.ast.nodes[callee as usize].kind else {
            unreachable!("the parser names the callee")
        };
        let (sym, s) = self.resolve(callee, name)?;
        self.arguments(at, params, refs, args, in_expr, false)?;
        self.capture(sym, false);
        self.ast.nodes[callee as usize].kind = Kind::Var(sym);
        self.ast.nodes[callee as usize].ty = s.ty;
        Ok(Kind::Indirect(callee, args))
    }

    /// A case label range: constants of the selector's family, within Integer.
    fn label(&mut self, label: Id, sel: Ty) -> R<()> {
        let Kind::Range(lo, hi) = self.ast.nodes[label as usize].kind else {
            unreachable!("the parser lists ranges")
        };
        for end in if lo == hi { vec![lo] } else { vec![lo, hi] } {
            self.expr(end)?;
            let n = self.ast.nodes[end as usize];
            let Kind::IntLit(v) = n.kind else {
                return Err(self.err(label, 133));
            };
            if !self.types.compat(sel, n.ty) {
                return Err(self.err(label, 74));
            }
            if self.types.tag(sel) == Some(Tag::Int) && !(-32768..=32767).contains(&v) {
                return Err(self.err(label, 112));
            }
        }
        Ok(())
    }

    /// A Char operand `e` used as a String: a one-character literal, else a `CharToStr` node.
    fn coerce_str(&mut self, e: Id) {
        let n = self.ast.nodes[e as usize];
        if self.types.tag(n.ty) != Some(Tag::Char) {
            return;
        }
        self.ast.nodes[e as usize].ty = Ty::STRING;
        self.ast.nodes[e as usize].kind = if let Kind::IntLit(c) = n.kind {
            self.ast.strs.push(vec![c as u8]);
            Kind::StrLit(self.ast.strs.len() as u32 - 1)
        } else {
            self.ast.nodes.push(n);
            Kind::CharToStr(self.ast.nodes.len() as Id - 1)
        };
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
    fn assignable(&mut self, at: Id, dest: Ty, value: Id) -> R<()> {
        let v = self.ast.nodes[value as usize];
        if self.types.is_str(dest) {
            if !self.types.is_str(v.ty) && self.types.tag(v.ty) != Some(Tag::Char) {
                return Err(self.err(at, 26));
            }
            self.coerce_str(value);
            return Ok(());
        }
        if self.types.is_agg(dest) {
            // Name equivalence: only the very same type; sets need compatible bases.
            let set = self.types.is_set(dest) && self.types.compat(dest, v.ty);
            return if v.ty == dest || set {
                Ok(())
            } else {
                Err(self.err(at, 26))
            };
        }
        if !self.types.compat(dest, v.ty) {
            return Err(self.err(at, 26));
        }
        match v.kind {
            Kind::IntLit(c) => {
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
        let text = self.types.is_str(n.ty);
        if self.types.tag(n.ty) == Some(Tag::Enum) || !text && self.types.tag(n.ty).is_none() {
            return Err(self.err(value, 64));
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
            Kind::Name(name) if self.with_field(name).is_some() => {
                let (record, ty, offset) = self.with_field(name).unwrap_or_default();
                (Kind::Field(record, offset), ty)
            }
            Kind::Index(base, index, more) => {
                self.expr(base)?;
                let Some((ix, lo, hi, elem)) =
                    self.types.array_of(self.ast.nodes[base as usize].ty)
                else {
                    return Err(self.err(e, if more { 93 } else { 121 }));
                };
                self.expr(index)?;
                let i = self.ast.nodes[index as usize];
                if !self.types.compat(ix, i.ty) {
                    return Err(self.err(e, 26));
                }
                if matches!(i.kind, Kind::IntLit(c) if c < lo || c > hi) {
                    return Err(self.err(e, 76));
                }
                (n.kind, elem)
            }
            Kind::Field(base, name) => {
                self.expr(base)?;
                let bt = self.ast.nodes[base as usize].ty;
                if !self.types.is_record(bt) {
                    return Err(self.err(e, 121));
                }
                let Some((ty, offset)) = self.types.field(bt, name) else {
                    return Err(self.err(e, 44));
                };
                (Kind::Field(base, offset), ty)
            }
            Kind::Name(name) => {
                let (sym, s) = self.resolve(e, name)?;
                match s.kind {
                    SymKind::Const if s.ty == Ty::STRING => (Kind::StrLit(s.val as u32), s.ty),
                    SymKind::Const if self.types.is_set(s.ty) => {
                        (self.ast.nodes[s.val as usize].kind, s.ty)
                    }
                    SymKind::Const => (Kind::IntLit(s.val), s.ty),
                    SymKind::Var | SymKind::Local | SymKind::Ref => match self.types.sig(s.ty) {
                        // A procedural variable in an expression is called.
                        Some((_, _, Ty::NONE)) => return Err(self.err(e, 143)),
                        Some((params, refs, result)) => {
                            let callee = self.ast.add(Kind::Name(name), n.line, n.col);
                            let none = self.ast.list(&[]);
                            (self.indirect(e, callee, params, refs, none, true)?, result)
                        }
                        None => {
                            self.capture(sym, false);
                            (Kind::Var(sym), s.ty)
                        }
                    },
                    SymKind::Routine => {
                        let r = &self.routines[s.val as usize];
                        let (func, result) = (r.func, r.result);
                        if !func {
                            return Err(self.err(e, 143));
                        }
                        let none = self.ast.list(&[]);
                        let (params, refs) = self.routine_params(s.val as usize);
                        self.arguments(
                            e,
                            params,
                            refs,
                            none,
                            true,
                            self.routines[s.val as usize].builtin,
                        )?;
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
                        let (params, refs) = self.routine_params(s.val as usize);
                        self.arguments(
                            e,
                            params,
                            refs,
                            args,
                            true,
                            self.routines[s.val as usize].builtin,
                        )?;
                        (Kind::Invoke(s.val as u32, args), result)
                    }
                    SymKind::Var | SymKind::Local | SymKind::Ref => match self.types.sig(s.ty) {
                        Some((_, _, Ty::NONE)) => return Err(self.err(callee, 143)),
                        Some((params, refs, result)) => {
                            (self.indirect(e, callee, params, refs, args, true)?, result)
                        }
                        None => return Err(self.err(e, 89)),
                    },
                    SymKind::Intrinsic if (2..=8).contains(&s.val) => {
                        let items = self.ast.items(args).to_vec();
                        if items.len() > 1 {
                            return Err(self.err(e, 89));
                        }
                        self.unary(e, INTRINSIC_OPS[s.val as usize - 2], items[0])?
                    }
                    SymKind::Intrinsic if s.val == 10 => self.size_of(e, args)?,
                    SymKind::Intrinsic if s.val == 11 => self.concat(e, args)?,
                    SymKind::Type => self.cast(e, s.ty, args)?,
                    SymKind::Intrinsic => return Err(self.err(callee, 42)),
                    _ => return Err(self.err(e, 89)),
                }
            }
            Kind::AddrOf(a) => {
                self.expr(a)?;
                match self.ast.nodes[a as usize].kind {
                    Kind::Var(sym) => self.capture(sym, true),
                    Kind::Index(..) | Kind::Field(..) | Kind::Deref(_) => {}
                    _ => return Err(self.err(e, 20)),
                }
                (n.kind, Ty::POINTER)
            }
            Kind::Deref(p) => {
                self.expr(p)?;
                match self.types.target(self.ast.nodes[p as usize].ty) {
                    Some(to) => (n.kind, to),
                    None => return Err(self.err(e, 121)),
                }
            }
            Kind::Paren(a) => {
                self.expr(a)?;
                let inner = self.ast.nodes[a as usize];
                (inner.kind, inner.ty)
            }
            Kind::Unary(op, a) => self.unary(e, op, a)?,
            Kind::SetLit(items) => {
                let mut base = Ty::NONE;
                for item in self.ast.items(items).to_vec() {
                    self.expr(item)?;
                    let ty = self.ast.nodes[item as usize].ty;
                    let el = self.types.element(base, ty);
                    base = el.map_err(|code| self.err(item, code))?;
                }
                (n.kind, self.types.set(base))
            }
            Kind::Range(lo, hi) => {
                self.expr(lo)?;
                self.expr(hi)?;
                let (a, b) = (
                    self.ast.nodes[lo as usize].ty,
                    self.ast.nodes[hi as usize].ty,
                );
                if !self.types.compat(a, b) {
                    return Err(self.err(hi, 26));
                }
                (n.kind, a)
            }
            Kind::Binary(op, a, b) => {
                self.expr(a)?;
                self.expr(b)?;
                let (na, nb) = (self.ast.nodes[a as usize], self.ast.nodes[b as usize]);
                let ty = self
                    .types
                    .binary(op, na.ty, nb.ty)
                    .map_err(|c| self.err(e, c))?;
                if ty == Ty::STRING || self.types.is_str(na.ty) || self.types.is_str(nb.ty) {
                    self.coerce_str(a);
                    self.coerce_str(b);
                }
                match (
                    self.ast.nodes[a as usize].kind,
                    self.ast.nodes[b as usize].kind,
                ) {
                    (Kind::IntLit(_), Kind::IntLit(0)) if matches!(op, Op::Div | Op::Mod) => {
                        return Err(self.err(e, 62));
                    }
                    (Kind::IntLit(x), Kind::IntLit(y)) if ty != Ty::STRING => {
                        self.constant(fold(op, x, y), ty)
                    }
                    (Kind::StrLit(x), Kind::StrLit(y)) if op == Op::Add => {
                        let mut joined = self.ast.strs[x as usize].clone();
                        joined.extend_from_slice(&self.ast.strs[y as usize]);
                        joined.truncate(255);
                        self.ast.strs.push(joined);
                        (Kind::StrLit(self.ast.strs.len() as u32 - 1), ty)
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

    /// A set element or range whose ends are constants.
    fn fixed(&self, item: Id) -> bool {
        let lit = |i: Id| matches!(self.ast.nodes[i as usize].kind, Kind::IntLit(_));
        match self.ast.nodes[item as usize].kind {
            Kind::Range(lo, hi) => lit(lo) && lit(hi),
            _ => lit(item),
        }
    }

    /// `T(x)`: the ordinal value of `x` as type `ty`.
    fn cast(&mut self, at: Id, ty: Ty, args: List) -> R<(Kind, Ty)> {
        let [arg] = self.ast.items(args)[..] else {
            return Err(self.err(at, 89));
        };
        self.expr(arg)?;
        let a = self.ast.nodes[arg as usize];
        let scalar = |t: Ty| self.types.tag(t).is_some() || self.types.is_ptr(t);
        if !scalar(ty) || !scalar(a.ty) {
            return Err(self.err(at, 200));
        }
        Ok(match a.kind {
            Kind::IntLit(v) => self.constant(v, ty),
            _ => (Kind::Unary(Op::Ord, arg), ty),
        })
    }

    /// `Delete(s, i, n)` is `s := StrDelete(s, i, n)`, `Insert(src, s, i)` is
    /// `s := StrInsert(src, s, i)`: the call becomes that assignment.
    fn edit_string(&mut self, at: Id, insert: bool, args: List) -> R<Kind> {
        let items = self.ast.items(args).to_vec();
        if items.len() != 3 {
            return Err(self.err(at, if items.len() < 3 { 87 } else { 89 }));
        }
        let target = items[insert as usize];
        self.expr(target)?;
        let t = self.ast.nodes[target as usize];
        if !matches!(
            t.kind,
            Kind::Var(_) | Kind::Index(..) | Kind::Field(..) | Kind::Deref(_)
        ) {
            return Err(self.err(target, 20));
        }
        if !self.types.is_str(t.ty) {
            return Err(self.err(target, 66));
        }
        let rest = if insert {
            [items[0], items[2]]
        } else {
            [items[1], items[2]]
        };
        for (x, ty) in rest.into_iter().zip(if insert {
            [Ty::STRING, Ty::INTEGER]
        } else {
            [Ty::INTEGER; 2]
        }) {
            self.expr(x)?;
            self.argument_assignable(at, ty, x, true)?;
        }
        let helper = self.helpers[if insert { H_INSERT } else { H_DELETE }];
        let (line, col) = (
            self.ast.nodes[at as usize].line,
            self.ast.nodes[at as usize].col,
        );
        let call = self.ast.add(Kind::Invoke(helper, args), line, col);
        self.ast.nodes[call as usize].ty = Ty::STRING;
        Ok(Kind::Assign(target, call))
    }

    /// `New`, `Dispose`, `GetMem`, `FreeMem`, `Mark`, `Release` and `RunError` (`which` is the
    /// intrinsic number): calls of the System heap helpers, with `SizeOf(p^)` as the size of
    /// `New` and `Dispose`. `New`, `GetMem` and `Mark` assign the helper's result to the variable.
    fn heap_call(&mut self, at: Id, which: i64, args: List) -> R<Kind> {
        let items = self.ast.items(args).to_vec();
        let want = [1, 1, 2, 2, 1, 1, 1][which as usize - 14];
        if items.len() != want {
            return Err(self.err(at, if items.len() < want { 87 } else { 89 }));
        }
        let p = items[0];
        if which == 20 {
            self.expr(p)?;
            self.argument_assignable(at, Ty::INTEGER, p, true)?;
            return Ok(Kind::RunErr(p));
        }
        match self.bad_var_arg(p) {
            0 => self.expr(p)?,
            code => return Err(self.err(p, code)),
        }
        let to = self.types.target(self.ast.nodes[p as usize].ty);
        let to = to.ok_or(self.err(p, 65))?;
        let pos = self.ast.nodes[at as usize];
        let size = items.get(1).copied().unwrap_or_else(|| {
            let k = self
                .ast
                .add(Kind::IntLit(self.types.size(to) as i64), pos.line, pos.col);
            self.ast.nodes[k as usize].ty = Ty::LONGINT;
            k
        });
        if let Some(&n) = items.get(1) {
            self.expr(n)?;
            self.argument_assignable(at, Ty::LONGINT, n, true)?;
        }
        let (helper, args) = match which {
            14 | 16 => (H_GET, vec![size]),
            15 | 17 => (H_PUT, vec![p, size]),
            18 => (H_MARK, vec![]),
            _ => (H_RELEASE, vec![p]),
        };
        let call = Kind::Invoke(self.helpers[helper], self.ast.list(&args));
        if matches!(which, 15 | 17 | 19) {
            return Ok(call);
        }
        let call = self.ast.add(call, pos.line, pos.col);
        self.ast.nodes[call as usize].ty = Ty::POINTER;
        Ok(Kind::Assign(p, call))
    }

    /// `Concat(a, b, ...)`: the same as `a + b + ...`.
    fn concat(&mut self, at: Id, args: List) -> R<(Kind, Ty)> {
        let items = self.ast.items(args).to_vec();
        if items.is_empty() {
            return Err(self.err(at, 42));
        }
        let (line, col) = (
            self.ast.nodes[at as usize].line,
            self.ast.nodes[at as usize].col,
        );
        let mut sum = items[0];
        for (i, &x) in items.iter().enumerate() {
            self.expr(x)?;
            let ty = self.ast.nodes[x as usize].ty;
            if !self.types.is_str(ty) && self.types.tag(ty) != Some(Tag::Char) {
                return Err(self.err(at, 67));
            }
            self.coerce_str(x);
            if i > 0 {
                sum = self.ast.add(Kind::Binary(Op::Add, sum, x), line, col);
                self.ast.nodes[sum as usize].ty = Ty::STRING;
            }
        }
        let n = self.ast.nodes[sum as usize];
        Ok((n.kind, n.ty))
    }

    /// `SizeOf(type name or variable)`, a constant.
    fn size_of(&mut self, at: Id, args: List) -> R<(Kind, Ty)> {
        let [arg] = self.ast.items(args)[..] else {
            return Err(self.err(at, 89));
        };
        let ty = match self.ast.nodes[arg as usize].kind {
            Kind::Name(name) if self.with_field(name).is_none() => {
                let (_, s) = self.resolve(arg, name)?;
                if s.kind == SymKind::Const || s.kind == SymKind::Routine {
                    return Err(self.err(arg, 20));
                }
                s.ty
            }
            _ => {
                self.expr(arg)?;
                self.ast.nodes[arg as usize].ty
            }
        };
        Ok(self.constant(self.types.size(ty) as i64, Ty::INTEGER))
    }

    /// A unary operator or intrinsic applied to `a`, folded when `a` is constant (`Sqr` is
    /// not constant in TP).
    fn unary(&mut self, e: Id, op: Op, a: Id) -> R<(Kind, Ty)> {
        self.expr(a)?;
        let na = self.ast.nodes[a as usize];
        let mut ty = self.types.unary(op, na.ty).map_err(|c| self.err(e, c))?;
        if op == Op::Ord && self.types.tag(na.ty) == Some(Tag::Enum) {
            // TP types Ord(enum) by the enumeration's range (`Ord(c) + w` is a Word sum).
            let (lo, hi, ..) = self.types.ord(na.ty);
            ty = self.types.subrange(lo, hi, Ty::INTEGER);
        }
        Ok(match na.kind {
            Kind::IntLit(v) if op == Op::Not && ty == Ty::BOOLEAN => self.constant(v ^ 1, ty),
            Kind::IntLit(v) if op != Op::Sqr => self.constant(fold(op, v, 0), ty),
            _ => (Kind::Unary(op, a), ty),
        })
    }

    /// A folded constant: an integer takes the smallest type, other ordinals keep theirs.
    fn constant(&self, v: i64, ty: Ty) -> (Kind, Ty) {
        let ty = if self.types.tag(ty) == Some(Tag::Int) {
            self.types.const_ty(v)
        } else {
            ty
        };
        (Kind::IntLit(v), ty)
    }
}
