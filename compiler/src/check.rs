//! Whole-program checker: declarations, name resolution, typing and constant folding.
//!
//! The checker walks the parser's tree once and rewrites it in place: a `Name` becomes a
//! `Var`, a constant, or a call; every expression node gets its type; constant expressions
//! are folded. It also decides where each variable lives, which is the central technique:
//!
//! - A variable is first a WASM local (`SymKind::Local`), or a global at a fixed address
//!   (`Var`). A `var` parameter is a local holding an address (`Ref`).
//! - A local that must have an address (passed as a `var` argument, `@x`, or an in-memory
//!   type such as a record) or that a nested routine uses is marked `needs_frame_slot` by
//!   `capture`. When the routine has been checked, `place_frame` moves those locals into the
//!   routine's frame in linear memory (`Frame`, `FrameRef`) and lays the frame out.
//! - A routine nested in another receives the enclosing frame pointer as a hidden first
//!   parameter, the static link; a function returning a String receives the address of its
//!   result as a further hidden parameter (see `Routine::hidden_params`).
//!
//! Several intrinsics are rewritten into an assignment of a System helper's result:
//! `Inc(x, n)` is `x := Ord(x) + n`, `New(p)` is `p := HeapGet(size)`, `Delete(s, i, n)` is
//! `s := StrDelete(s, i, n)`, `Str`, `Val`, `Insert`, `GetMem`, `Mark` likewise; `Concat` is
//! a chain of `+`. Codegen then needs no knowledge of them.
use crate::ast::{Ast, Id, Kind, List, Op, NO_NAME};
use crate::diag::{code, Diag};
use crate::types::{
    ConstValue, Field, FrameCopy, Intrinsic, ParamType, Routine, Signature, Sym, SymKind, Symbol,
    Tag, Ty, TypedConst, Types,
};

/// The System routines that the compiler calls on its own behalf, bound by name right after
/// System is loaded (decision 11).
#[derive(Clone, Copy)]
pub enum Helper {
    AbsLong,
    StrCat,
    StrCmp,
    StrAssign,
    StrDelete,
    StrInsert,
    HeapGet,
    HeapPut,
    HeapMark,
    HeapRelease,
    SetOp,
    SetAdd,
    SetIn,
    SetCmp,
    StrInt,
    ValLong,
    ValReal,
    RangeChk,
    HostWrite,
    HostRtError,
    FmtReal,
}

impl Helper {
    const ALL: [Helper; 21] = [
        Helper::AbsLong,
        Helper::StrCat,
        Helper::StrCmp,
        Helper::StrAssign,
        Helper::StrDelete,
        Helper::StrInsert,
        Helper::HeapGet,
        Helper::HeapPut,
        Helper::HeapMark,
        Helper::HeapRelease,
        Helper::SetOp,
        Helper::SetAdd,
        Helper::SetIn,
        Helper::SetCmp,
        Helper::StrInt,
        Helper::ValLong,
        Helper::ValReal,
        Helper::RangeChk,
        Helper::HostWrite,
        Helper::HostRtError,
        Helper::FmtReal,
    ];

    /// The routine's name in `runtime/system.pas`, upper-cased.
    fn name(self) -> &'static [u8] {
        match self {
            Helper::AbsLong => b"ABSLONG",
            Helper::StrCat => b"STRCAT",
            Helper::StrCmp => b"STRCMP",
            Helper::StrAssign => b"STRASSIGN",
            Helper::StrDelete => b"STRDELETE",
            Helper::StrInsert => b"STRINSERT",
            Helper::HeapGet => b"HEAPGET",
            Helper::HeapPut => b"HEAPPUT",
            Helper::HeapMark => b"HEAPMARK",
            Helper::HeapRelease => b"HEAPRELEASE",
            Helper::SetOp => b"SETOP",
            Helper::SetAdd => b"SETADD",
            Helper::SetIn => b"SETIN",
            Helper::SetCmp => b"SETCMP",
            Helper::StrInt => b"STRINT",
            Helper::ValLong => b"VALLONG",
            Helper::ValReal => b"VALREAL",
            Helper::RangeChk => b"RANGECHK",
            Helper::HostWrite => b"HOSTWRITE",
            Helper::HostRtError => b"RTERROR",
            Helper::FmtReal => b"FMT_REAL",
        }
    }
}

/// The most bytes of one structure, of all globals, or of one routine's locals.
const MAX_DATA: u32 = 65535;

/// Static data: globals start here, after the code generator's fixed header (bytes 256 to
/// 278: the characters it writes and its digit buffer), rounded up to a multiple of 8.
pub const GLOBALS: u32 = 280;

/// The first 4 bytes of a frame hold the static link of a nested routine.
const STATIC_LINK_BYTES: u32 = 4;

pub struct Info {
    pub types: Types,
    pub syms: Vec<Symbol>,
    pub routines: Vec<Routine>,
    /// First address after the globals.
    pub data_end: u32,
    /// The typed constants in address order.
    pub typed_consts: Vec<TypedConst>,
    /// The routine index of each `Helper`.
    helpers: [u32; Helper::ALL.len()],
    /// Address of System's `HeapOrg`; the next 4 bytes are `HeapPtr`.
    pub heap: u32,
}

impl Info {
    /// The routine index of a System helper.
    pub fn helper(&self, helper: Helper) -> u32 {
        self.helpers[helper as usize]
    }
}

type R<T> = Result<T, Diag>;

/// Predefined names: types, constants, and the intrinsics the compiler expands itself.
const PREDEFINED: &[(&[u8], SymKind, Ty)] = &[
    (b"BYTE", SymKind::Type, Ty::BYTE),
    (b"SHORTINT", SymKind::Type, Ty::SHORTINT),
    (b"INTEGER", SymKind::Type, Ty::INTEGER),
    (b"WORD", SymKind::Type, Ty::WORD),
    (b"LONGINT", SymKind::Type, Ty::LONGINT),
    (b"BOOLEAN", SymKind::Type, Ty::BOOLEAN),
    (b"CHAR", SymKind::Type, Ty::CHAR),
    (b"REAL", SymKind::Type, Ty::REAL),
    (
        b"PI",
        SymKind::Const(ConstValue::Real(std::f64::consts::PI)),
        Ty::REAL,
    ),
    (b"SINGLE", SymKind::Omitted, Ty::NONE),
    (b"DOUBLE", SymKind::Omitted, Ty::NONE),
    (b"EXTENDED", SymKind::Omitted, Ty::NONE),
    (b"COMP", SymKind::Omitted, Ty::NONE),
    (b"SEG", SymKind::Omitted, Ty::NONE),
    (b"OFS", SymKind::Omitted, Ty::NONE),
    (b"PTR", SymKind::Omitted, Ty::NONE),
    (b"POINTER", SymKind::Type, Ty::POINTER),
    (b"NIL", SymKind::Const(ConstValue::Int(0)), Ty::POINTER),
    (b"FALSE", SymKind::Const(ConstValue::Int(0)), Ty::BOOLEAN),
    (b"TRUE", SymKind::Const(ConstValue::Int(1)), Ty::BOOLEAN),
    (b"STRING", SymKind::Type, Ty::STRING),
    (b"WRITE", SymKind::Intrinsic(Intrinsic::Write), Ty::NONE),
    (b"WRITELN", SymKind::Intrinsic(Intrinsic::WriteLn), Ty::NONE),
    (b"ORD", SymKind::Intrinsic(Intrinsic::Ord), Ty::NONE),
    (b"CHR", SymKind::Intrinsic(Intrinsic::Chr), Ty::NONE),
    (b"SUCC", SymKind::Intrinsic(Intrinsic::Succ), Ty::NONE),
    (b"PRED", SymKind::Intrinsic(Intrinsic::Pred), Ty::NONE),
    (b"ODD", SymKind::Intrinsic(Intrinsic::Odd), Ty::NONE),
    (b"ABS", SymKind::Intrinsic(Intrinsic::Abs), Ty::NONE),
    (b"SQR", SymKind::Intrinsic(Intrinsic::Sqr), Ty::NONE),
    (b"EXIT", SymKind::Intrinsic(Intrinsic::Exit), Ty::NONE),
    (b"SIZEOF", SymKind::Intrinsic(Intrinsic::SizeOf), Ty::NONE),
    (b"CONCAT", SymKind::Intrinsic(Intrinsic::Concat), Ty::NONE),
    (b"DELETE", SymKind::Intrinsic(Intrinsic::Delete), Ty::NONE),
    (b"INSERT", SymKind::Intrinsic(Intrinsic::Insert), Ty::NONE),
    (b"NEW", SymKind::Intrinsic(Intrinsic::New), Ty::NONE),
    (b"DISPOSE", SymKind::Intrinsic(Intrinsic::Dispose), Ty::NONE),
    (b"GETMEM", SymKind::Intrinsic(Intrinsic::GetMem), Ty::NONE),
    (b"FREEMEM", SymKind::Intrinsic(Intrinsic::FreeMem), Ty::NONE),
    (b"MARK", SymKind::Intrinsic(Intrinsic::Mark), Ty::NONE),
    (b"RELEASE", SymKind::Intrinsic(Intrinsic::Release), Ty::NONE),
    (
        b"RUNERROR",
        SymKind::Intrinsic(Intrinsic::RunError),
        Ty::NONE,
    ),
    (b"ROUND", SymKind::Intrinsic(Intrinsic::Round), Ty::NONE),
    (b"TRUNC", SymKind::Intrinsic(Intrinsic::Trunc), Ty::NONE),
    (b"SQRT", SymKind::Intrinsic(Intrinsic::Sqrt), Ty::NONE),
    (b"INT", SymKind::Intrinsic(Intrinsic::Int), Ty::NONE),
    (b"FRAC", SymKind::Intrinsic(Intrinsic::Frac), Ty::NONE),
    (b"INC", SymKind::Intrinsic(Intrinsic::Inc), Ty::NONE),
    (b"DEC", SymKind::Intrinsic(Intrinsic::Dec), Ty::NONE),
    (b"STR", SymKind::Intrinsic(Intrinsic::Str), Ty::NONE),
    (b"VAL", SymKind::Intrinsic(Intrinsic::Val), Ty::NONE),
];

/// A parameter as declared, with the node that declares it.
struct ParamDecl {
    node: Id,
    name: u32,
    type_name: u32,
    by_ref: bool,
}

/// The checker's state that a routine body saves on entry and restores on exit.
struct SavedScope {
    scope_len: usize,
    first_user_scope: usize,
    next_local: u32,
    local_bytes: u32,
}

struct Cx<'a> {
    ast: &'a mut Ast,
    types: Types,
    syms: Vec<Symbol>,
    /// Visible names, innermost last.
    scope: Vec<(u32, Sym)>,
    /// Scope entries below this index are predefined or from System and may be redeclared.
    first_user_scope: usize,
    /// First address after the globals declared so far.
    data_end: u32,
    typed_consts: Vec<TypedConst>,
    routines: Vec<Routine>,
    /// The routines being checked, outermost first; its length is the current depth.
    routine_stack: Vec<usize>,
    /// The next free local index.
    next_local: u32,
    /// Bytes of the current routine's local variables.
    local_bytes: u32,
    /// The record designators of the enclosing `with` statements, innermost last.
    withs: Vec<(Id, Ty)>,
    helpers: [u32; Helper::ALL.len()],
    /// Pointer types whose target is declared later in the same type section.
    forward: Vec<(Ty, u32)>,
}

pub fn check(ast: &mut Ast, root: Id) -> R<Info> {
    let mut cx = Cx {
        ast,
        types: Types::new(),
        syms: Vec::new(),
        scope: Vec::new(),
        first_user_scope: 0,
        data_end: GLOBALS,
        typed_consts: Vec::new(),
        routines: Vec::new(),
        routine_stack: Vec::new(),
        next_local: 0,
        local_bytes: 0,
        withs: Vec::new(),
        helpers: [0; Helper::ALL.len()],
        forward: Vec::new(),
    };
    cx.declare_predefined();
    let Kind::Program {
        name,
        declarations,
        body,
    } = cx.ast.kind(root)
    else {
        unreachable!("the parser's root is a program")
    };
    let declarations = cx.ast.items_vec(declarations);
    let (system, program) = declarations.split_at(cx.ast.system);
    for &d in system {
        cx.decl(d)?;
    }
    let heap = cx.bind_system_names()?;
    cx.first_user_scope = cx.scope.len();
    if name != NO_NAME {
        cx.declare(root, name, Symbol::new(SymKind::Prog, Ty::NONE))?;
    }
    for &d in program {
        cx.decl(d)?;
    }
    cx.resolve_forward(body)?;
    if cx.routines.iter().any(|r| !r.defined) {
        return Err(cx.err(root, code::UNDEFINED_FORWARD));
    }
    cx.stmt(body)?;
    Ok(Info {
        types: cx.types,
        syms: cx.syms,
        routines: cx.routines,
        data_end: cx.data_end,
        typed_consts: cx.typed_consts,
        helpers: cx.helpers,
        heap,
    })
}

/// Value of a constant operator, computed at LongInt precision. The unary operators use only
/// `a`; the operators that are not constant (`Slash`, `Round`, ...) fold to 0 and are never
/// called with constants.
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
        Op::Slash | Op::Round | Op::Trunc | Op::Sqrt | Op::Int | Op::Frac => 0,
    };
    v as i64
}

impl Cx<'_> {
    fn declare_predefined(&mut self) {
        for &(name, kind, ty) in PREDEFINED {
            let name = self.ast.intern(name);
            self.push(name, Symbol::new(kind, ty));
        }
    }

    /// Binds the System routines the compiler calls on its own (`Helper`) and finds `HeapOrg`,
    /// before any program declaration can shadow them. Returns the address of `HeapOrg`.
    fn bind_system_names(&mut self) -> R<u32> {
        let broken_system = Diag {
            code: code::UNKNOWN_IDENTIFIER,
            line: 0,
            col: 0,
        };
        for helper in Helper::ALL {
            let name = self.ast.intern(helper.name());
            let Some((
                _,
                Symbol {
                    kind: SymKind::Routine(r),
                    ..
                },
            )) = self.lookup_symbol(name)
            else {
                return Err(broken_system);
            };
            self.helpers[helper as usize] = r;
        }
        let heap_org = self.ast.intern(b"HEAPORG");
        let Some((
            _,
            Symbol {
                kind: SymKind::Var(address),
                ..
            },
        )) = self.lookup_symbol(heap_org)
        else {
            return Err(broken_system);
        };
        if self.routines.iter().any(|r| !r.defined) {
            return Err(Diag {
                code: code::UNDEFINED_FORWARD,
                ..broken_system
            });
        }
        Ok(address)
    }

    /// Adds a symbol and makes `name` refer to it from here on.
    fn push(&mut self, name: u32, symbol: Symbol) -> Sym {
        self.syms.push(Symbol {
            depth: self.routine_stack.len() as u32,
            ..symbol
        });
        let sym = Sym(self.syms.len() as u32 - 1);
        self.scope.push((name, sym));
        sym
    }

    fn lookup(&self, name: u32) -> Option<Sym> {
        self.scope.iter().rev().find(|e| e.0 == name).map(|e| e.1)
    }

    fn symbol(&self, sym: Sym) -> Symbol {
        self.syms[sym.0 as usize]
    }

    fn lookup_symbol(&self, name: u32) -> Option<(Sym, Symbol)> {
        let sym = self.lookup(name)?;
        Some((sym, self.symbol(sym)))
    }

    /// The name of a `Name` node.
    fn name_of(&self, id: Id) -> u32 {
        match self.ast.kind(id) {
            Kind::Name(name) => name,
            _ => unreachable!("the parser names it"),
        }
    }

    fn err(&self, id: Id, code: u16) -> Diag {
        let (line, col) = self.ast.position(id);
        Diag { code, line, col }
    }

    /// Declares `name` in the current scope; a second declaration in the same scope is an
    /// error, but a user declaration may hide a predefined or System one.
    fn declare(&mut self, at: Id, name: u32, symbol: Symbol) -> R<Sym> {
        if self.scope[self.first_user_scope..]
            .iter()
            .any(|e| e.0 == name)
        {
            return Err(self.err(at, code::DUPLICATE_IDENTIFIER));
        }
        Ok(self.push(name, symbol))
    }

    /// Resolves the pointer types declared with a target not yet declared. Called where a type
    /// section ends; TP reports an undefined target at `at`, the next declaration or the body.
    fn resolve_forward(&mut self, at: Id) -> R<()> {
        for (pointer, name) in std::mem::take(&mut self.forward) {
            let to = self.type_of(at, name).map_err(|d| match d.code {
                code::NOT_SUPPORTED => d,
                _ => Diag {
                    code: code::UNDEFINED_POINTER_TARGET,
                    ..d
                },
            })?;
            self.types.set_pointer_target(pointer, to);
        }
        Ok(())
    }

    // ---- Declarations ----

    fn decl(&mut self, d: Id) -> R<()> {
        // A type section ends at the first declaration that is not a type.
        if !matches!(self.ast.kind(d), Kind::TypeDecl(..)) {
            self.resolve_forward(d)?;
        }
        match self.ast.kind(d) {
            Kind::ConstDecl(name, value) => self.const_decl(d, name, value),
            Kind::TypedConst(name, type_name, init) => {
                self.typed_const_decl(d, name, type_name, init)
            }
            Kind::VarDecl(name, type_name) => self.var_decl(d, name, type_name),
            Kind::TypeDecl(name, spec) => {
                let ty = self.spec(spec, name)?;
                self.declare(d, name, Symbol::new(SymKind::Type, ty))?;
                Ok(())
            }
            Kind::Routine { .. } => self.routine(d),
            _ => unreachable!("declarations only"),
        }
    }

    /// A constant must fold to an integer, Real, string or set of constants.
    fn const_decl(&mut self, d: Id, name: u32, value: Id) -> R<()> {
        self.expr(value)?;
        let node = self.ast.node(value);
        let (constant, ty) = match node.kind {
            Kind::IntLit(v) => (ConstValue::Int(v), node.ty),
            Kind::RealLit(v) => (ConstValue::Real(v), Ty::REAL),
            Kind::StrLit(i) => (ConstValue::Str(i), Ty::STRING),
            Kind::SetLit(items) if self.ast.items(items).iter().all(|&i| self.is_constant(i)) => {
                (ConstValue::Set(value), node.ty)
            }
            _ => return Err(self.err(value, code::CANNOT_EVALUATE)),
        };
        self.declare(d, name, Symbol::new(SymKind::Const(constant), ty))?;
        Ok(())
    }

    /// `name: type = init`: a variable in static storage, even inside a routine, so its value
    /// outlives each call. The initialiser is checked here; codegen lays out its bytes
    /// (`static_image`) when a live body names the constant.
    fn typed_const_decl(&mut self, d: Id, name: u32, type_name: u32, init: Id) -> R<()> {
        let ty = self.type_of(d, type_name)?;
        if self.types.sig(ty).is_some() {
            return Err(self.err(d, code::PROCEDURE_TYPE_NOT_ALLOWED));
        }
        self.initial_value(ty, init)?;
        let address = self.data_end;
        let symbol = self.global_variable(d, ty, self.types.size(ty))?;
        self.typed_consts.push(TypedConst { address, ty, init });
        self.declare(d, name, symbol)?;
        Ok(())
    }

    /// Checks that `init` is a constant of type `ty`, following the shape of the type.
    fn initial_value(&mut self, ty: Ty, init: Id) -> R<()> {
        if matches!(self.ast.kind(init), Kind::RecordInit(_)) && !self.types.is_record(ty) {
            return Err(self.err(init, code::TYPE_MISMATCH));
        }
        if self.types.is_record(ty) {
            return self.record_initialiser(ty, init);
        }
        match self.types.array_of(ty) {
            Some((_, lo, hi, elem)) if !self.types.is_str(ty) => {
                self.array_initialiser(init, (hi - lo + 1) as usize, elem)
            }
            _ => self.scalar_initialiser(ty, init),
        }
    }

    /// `(a, b, c)` with exactly `count` elements, or a string for an array of Char.
    fn array_initialiser(&mut self, init: Id, count: usize, elem: Ty) -> R<()> {
        let Kind::ArrayInit(items) = self.ast.kind(init) else {
            return self.char_array_string(init, count, elem);
        };
        let items = self.ast.items_vec(items);
        for (n, &item) in items.iter().enumerate() {
            if n == count {
                return Err(self.err(item, code::RIGHT_PAREN_EXPECTED));
            }
            self.initial_value(elem, item)?;
        }
        if items.len() < count {
            return Err(self.err(init, code::COMMA_EXPECTED));
        }
        Ok(())
    }

    /// `array[1..4] of Char = 'ABCD'`: the string must have exactly as many characters as the
    /// array has elements.
    fn char_array_string(&mut self, init: Id, count: usize, elem: Ty) -> R<()> {
        self.expr(init)?;
        self.coerce_str(init);
        match self.ast.kind(init) {
            Kind::StrLit(s) if self.types.tag(elem) == Some(Tag::Char) => {
                if self.ast.strs[s as usize].len() == count {
                    return Ok(());
                }
                Err(self.err(init, code::STRING_LENGTH_MISMATCH))
            }
            _ => Err(self.err(init, code::TYPE_MISMATCH)),
        }
    }

    /// `(f: a; g: b)`: the entries name the first fields of the record, in declaration order.
    fn record_initialiser(&mut self, ty: Ty, init: Id) -> R<()> {
        let Kind::RecordInit(entries) = self.ast.kind(init) else {
            return Err(self.err(init, code::FIELD_IDENTIFIER_EXPECTED));
        };
        let fields = self.types.record_fields(ty);
        for (n, entry) in self.ast.items_vec(entries).into_iter().enumerate() {
            let Kind::FieldInit(name, value) = self.ast.kind(entry) else {
                unreachable!("a record initialiser holds field entries")
            };
            let Some(field) = fields.get(n).filter(|field| field.name == name) else {
                let is_a_field = self.types.field(ty, name).is_some();
                let code = if is_a_field {
                    code::INVALID_FIELD_ORDER
                } else {
                    code::FIELD_IDENTIFIER_EXPECTED
                };
                return Err(self.err(entry, code));
            };
            self.initial_value(field.ty, value)?;
        }
        Ok(())
    }

    /// A scalar, string, set or pointer: a constant expression assignable to `ty`. `(value)`
    /// becomes the value itself.
    fn scalar_initialiser(&mut self, ty: Ty, init: Id) -> R<()> {
        if let Kind::ArrayInit(items) = self.ast.kind(init) {
            let &[inner] = self.ast.items(items) else {
                return Err(self.err(init, code::RIGHT_PAREN_EXPECTED));
            };
            self.scalar_initialiser(ty, inner)?;
            let value = self.ast.node(inner);
            self.ast.resolve(init, value.kind, value.ty);
            return Ok(());
        }
        self.expr(init)?;
        self.assignable(init, ty, init)?;
        let is_constant = match self.ast.kind(init) {
            Kind::IntLit(_) | Kind::RealLit(_) | Kind::StrLit(_) => true,
            Kind::SetLit(items) => self.ast.items(items).iter().all(|&i| self.is_constant(i)),
            _ => false,
        };
        if !is_constant {
            return Err(self.err(init, code::CANNOT_EVALUATE));
        }
        Ok(())
    }

    /// A set element or range whose ends are integer constants.
    fn is_constant(&self, item: Id) -> bool {
        let is_literal = |i: Id| matches!(self.ast.kind(i), Kind::IntLit(_));
        match self.ast.kind(item) {
            Kind::Range(lo, hi) => is_literal(lo) && is_literal(hi),
            _ => is_literal(item),
        }
    }

    fn var_decl(&mut self, d: Id, name: u32, type_name: u32) -> R<()> {
        let ty = self.type_of(d, type_name)?;
        let size = self.types.size(ty);
        let symbol = if self.routine_stack.is_empty() {
            self.global_variable(d, ty, size)?
        } else {
            self.local_variable(d, ty, size)?
        };
        self.declare(d, name, symbol)?;
        Ok(())
    }

    /// A global lives at the next free static address.
    fn global_variable(&mut self, d: Id, ty: Ty, size: u32) -> R<Symbol> {
        let address = self.data_end;
        self.data_end += size;
        if self.data_end - GLOBALS > MAX_DATA {
            return Err(self.err(d, code::TOO_MANY_VARIABLES));
        }
        Ok(Symbol::new(SymKind::Var(address), ty))
    }

    /// A local is the next WASM local; aggregates and other in-memory types always live in
    /// the frame.
    fn local_variable(&mut self, d: Id, ty: Ty, size: u32) -> R<Symbol> {
        let index = self.next_local;
        self.next_local += 1;
        self.local_bytes += size;
        if self.local_bytes > MAX_DATA {
            return Err(self.err(d, code::TOO_MANY_VARIABLES));
        }
        Ok(Symbol {
            needs_frame_slot: self.types.in_memory(ty),
            ..Symbol::new(SymKind::Local(index), ty)
        })
    }

    fn type_of(&self, at: Id, name: u32) -> R<Ty> {
        match self.lookup_symbol(name) {
            None => Err(self.err(at, code::UNKNOWN_IDENTIFIER)),
            Some((_, symbol)) => match symbol.kind {
                SymKind::Type => Ok(symbol.ty),
                SymKind::Omitted => Err(self.err(at, code::NOT_SUPPORTED)),
                _ => Err(self.err(at, code::TYPE_IDENTIFIER_EXPECTED)),
            },
        }
    }

    // ---- Type specifications ----

    /// The type a declaration `name = spec` defines; `type T = T` is error 21.
    fn spec(&mut self, spec: Id, name: u32) -> R<Ty> {
        match self.ast.kind(spec) {
            Kind::Name(n) if n == name => Err(self.err(spec, code::ERROR_IN_TYPE)),
            Kind::Name(n) => self.type_of(spec, n),
            Kind::Enum(names) => self.enum_type(names),
            Kind::PtrType(target) => Ok(self.pointer_type(spec, target)),
            Kind::StrType(size) => self.string_type(spec, size),
            Kind::Array(indexes, element) => self.array_type(spec, indexes, element),
            Kind::Record(items) => self.record_type(spec, items),
            Kind::SetType(of) => {
                let base = self.spec(of, NO_NAME)?;
                self.types.set_of(base).map_err(|code| self.err(of, code))
            }
            Kind::Range(lo, hi) => self.subrange_type(spec, lo, hi),
            Kind::ProcType { params, result, .. } => self.proc_type(spec, params, result),
            _ => unreachable!("the parser builds type specs"),
        }
    }

    /// Each member of an enumeration becomes a constant of the new type.
    fn enum_type(&mut self, names: List) -> R<Ty> {
        let members = self.ast.items_vec(names);
        let ty = self.types.enumeration(members.len() as i64);
        for (ordinal, id) in members.into_iter().enumerate() {
            let name = self.name_of(id);
            let value = SymKind::Const(ConstValue::Int(ordinal as i64));
            self.declare(id, name, Symbol::new(value, ty))?;
        }
        Ok(ty)
    }

    /// `^name`; a target not declared yet is resolved where the type section ends.
    fn pointer_type(&mut self, spec: Id, target: u32) -> Ty {
        let to = self.type_of(spec, target).unwrap_or(Ty::NONE);
        let pointer = self.types.pointer(to);
        if to == Ty::NONE {
            self.forward.push((pointer, target));
        }
        pointer
    }

    fn string_type(&mut self, spec: Id, size: Id) -> R<Ty> {
        self.expr(size)?;
        let Kind::IntLit(n) = self.ast.kind(size) else {
            return Err(self.err(spec, code::CANNOT_EVALUATE));
        };
        if !(1..=255).contains(&n) {
            return Err(self.err(spec, code::INVALID_STRING_LENGTH));
        }
        Ok(self.types.string(n as u8))
    }

    /// `array[i, j] of e` is an array of arrays: the innermost dimension is built first.
    fn array_type(&mut self, spec: Id, indexes: List, element: Id) -> R<Ty> {
        let mut index_types = Vec::new();
        for index in self.ast.items_vec(indexes) {
            let ty = self.spec(index, NO_NAME)?;
            if self.types.tag(ty).is_none() {
                return Err(self.err(index, code::ORDINAL_TYPE_EXPECTED));
            }
            index_types.push(ty);
        }
        let mut ty = self.spec(element, NO_NAME)?;
        for &index in index_types.iter().rev() {
            ty = self.types.array(index, ty);
            if self.types.size(ty) > MAX_DATA {
                return Err(self.err(spec, code::STRUCTURE_TOO_LARGE));
            }
        }
        Ok(ty)
    }

    fn record_type(&mut self, spec: Id, items: List) -> R<Ty> {
        let mut fields = Vec::new();
        let size = self.fields(items, 0, &mut fields)?;
        if size > MAX_DATA {
            return Err(self.err(spec, code::STRUCTURE_TOO_LARGE));
        }
        Ok(self.types.record(fields, size))
    }

    /// Lays out the fields `items` from offset `at`, appending them to `out`; returns the end
    /// offset. The arms of a variant part all start at the end of the fixed part.
    fn fields(&mut self, items: List, at: u32, out: &mut Vec<Field>) -> R<u32> {
        let mut end = at;
        for item in self.ast.items_vec(items) {
            match self.ast.kind(item) {
                Kind::FieldDecl(name, spec) => {
                    let ty = self.spec(spec, NO_NAME)?;
                    // An untagged variant's selector type has no name and takes no space.
                    if name != NO_NAME {
                        if out.iter().any(|f| f.name == name) {
                            return Err(self.err(item, code::DUPLICATE_IDENTIFIER));
                        }
                        out.push(Field {
                            name,
                            ty,
                            offset: end,
                        });
                        end += self.types.size(ty);
                    }
                }
                Kind::Variant(arms) => {
                    let start = end;
                    for arm in self.ast.items_vec(arms) {
                        let Kind::Record(list) = self.ast.kind(arm) else {
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

    /// `lo..hi` of integer, Char or Boolean constants.
    fn subrange_type(&mut self, spec: Id, lo: Id, hi: Id) -> R<Ty> {
        let (low, low_ty) = self.range_end(spec, lo)?;
        let (high, high_ty) = self.range_end(spec, hi)?;
        if !self.types.compat(low_ty, high_ty) {
            return Err(self.err(spec, code::TYPE_MISMATCH));
        }
        if low > high {
            return Err(self.err(spec, code::LOWER_BOUND_GREATER));
        }
        Ok(self.types.subrange(low, high, low_ty))
    }

    fn range_end(&mut self, spec: Id, end: Id) -> R<(i64, Ty)> {
        self.expr(end)?;
        let node = self.ast.node(end);
        match node.kind {
            Kind::IntLit(value) => Ok((value, node.ty)),
            _ => Err(self.err(spec, code::CANNOT_EVALUATE)),
        }
    }

    /// `procedure (params)` or `function (params): result`.
    fn proc_type(&mut self, spec: Id, params: Option<List>, result: u32) -> R<Ty> {
        let mut signature = Vec::new();
        for param in self.param_decls(params) {
            let ty = self.type_of(param.node, param.type_name)?;
            signature.push(ParamType {
                ty,
                by_ref: param.by_ref,
            });
        }
        let result = if result == NO_NAME {
            Ty::NONE
        } else {
            self.type_of(spec, result)?
        };
        if self.types.is_str(result) || result == Ty::REAL {
            return Err(self.err(spec, code::NOT_SUPPORTED));
        }
        Ok(self.types.proc(signature, result))
    }

    fn param_decls(&self, params: Option<List>) -> Vec<ParamDecl> {
        let nodes = params.map_or(Vec::new(), |list| self.ast.items_vec(list));
        let decl = |node: Id| {
            let Kind::Param {
                name,
                type_name,
                by_ref,
            } = self.ast.kind(node)
            else {
                unreachable!("the parser lists parameters")
            };
            ParamDecl {
                node,
                name,
                type_name,
                by_ref,
            }
        };
        nodes.into_iter().map(decl).collect()
    }

    // ---- Routines ----

    /// A procedure or function declaration, with its body when it is not `forward`.
    fn routine(&mut self, d: Id) -> R<()> {
        let Kind::Routine {
            name,
            params,
            result,
            is_function,
            block,
            external,
        } = self.ast.kind(d)
        else {
            unreachable!("routine declaration")
        };
        let signature = self.signature_of(d, params, result, is_function)?;
        if external.is_some() {
            self.check_import_signature(d, &signature)?;
        }
        let index = self.declare_routine(d, name, params.is_some(), signature, block)?;
        if external.is_some() {
            // The host provides the body.
            self.routines[index].import = external;
            self.routines[index].defined = true;
        }
        match block {
            Some(block) => self.routine_body(index, block),
            None => Ok(()),
        }
    }

    /// A host import takes only plain scalars by value (a Real is an f64, the rest i32) and
    /// returns one i32 or nothing; it must be a top-level routine, which has no static link.
    fn check_import_signature(&self, d: Id, signature: &Signature) -> R<()> {
        let passed_by_value =
            |p: &ParamType| !p.by_ref && !self.types.is_agg(p.ty) && !self.types.is_str(p.ty);
        let nested = !self.routine_stack.is_empty();
        let result_is_i32 = !self.types.in_memory(signature.result);
        if nested || !result_is_i32 || !signature.params.iter().all(passed_by_value) {
            return Err(self.err(d, code::NOT_SUPPORTED));
        }
        Ok(())
    }

    /// The parameters and result a declaration states, with their types resolved.
    fn signature_of(
        &mut self,
        d: Id,
        params: Option<List>,
        result: u32,
        is_function: bool,
    ) -> R<Signature> {
        let mut types = Vec::new();
        let mut names = Vec::new();
        for param in self.param_decls(params) {
            let ty = self.type_of(param.node, param.type_name)?;
            types.push(ParamType {
                ty,
                by_ref: param.by_ref,
            });
            names.push(param.name);
        }
        let mut result_ty = Ty::NONE;
        if result != NO_NAME {
            result_ty = self.type_of(d, result)?;
            if self.types.is_agg(result_ty) {
                return Err(self.err(d, code::INVALID_RESULT_TYPE));
            }
        }
        Ok(Signature {
            params: types,
            names,
            result: result_ty,
            is_function,
        })
    }

    /// Enters the routine in the routine table, or matches a definition with its earlier
    /// `forward` declaration. Returns the routine's index.
    fn declare_routine(
        &mut self,
        d: Id,
        name: u32,
        has_param_list: bool,
        signature: Signature,
        block: Option<Id>,
    ) -> R<usize> {
        let existing = self.scope[self.first_user_scope..]
            .iter()
            .find(|e| e.0 == name)
            .map(|e| self.symbol(e.1));
        match existing.map(|s| s.kind) {
            None => {
                if signature.is_function && signature.result == Ty::NONE {
                    return Err(self.err(d, code::COLON_EXPECTED));
                }
                let depth = self.routine_stack.len() as u32 + 1;
                let result_ptr = self.types.in_memory(signature.result);
                let builtin = self.first_user_scope == 0;
                self.routines
                    .push(Routine::new(signature, depth, result_ptr, builtin));
                if let Some(&parent) = self.routine_stack.last() {
                    self.routines[parent].has_children = true;
                }
                let index = self.routines.len() - 1;
                let kind = SymKind::Routine(index as u32);
                self.declare(d, name, Symbol::new(kind, Ty::NONE))?;
                Ok(index)
            }
            Some(SymKind::Routine(index)) => {
                let index = index as usize;
                self.match_forward(d, index, has_param_list, signature, block)?;
                Ok(index)
            }
            Some(_) => Err(self.err(d, code::DUPLICATE_IDENTIFIER)),
        }
    }

    /// A definition of a routine declared `forward` must come once, with a body, and repeat
    /// the parameters and result exactly (or leave them out).
    fn match_forward(
        &mut self,
        d: Id,
        index: usize,
        has_param_list: bool,
        signature: Signature,
        block: Option<Id>,
    ) -> R<()> {
        let routine = &self.routines[index];
        if routine.defined || routine.is_function != signature.is_function {
            return Err(self.err(d, code::DUPLICATE_IDENTIFIER));
        }
        if block.is_none() {
            return Err(self.err(d, code::BEGIN_EXPECTED));
        }
        let params_differ = has_param_list && signature.params != routine.params;
        let result_differs = signature.result != Ty::NONE && signature.result != routine.result;
        if params_differ || result_differs {
            return Err(self.err(d, code::HEADER_MISMATCH));
        }
        if has_param_list {
            self.routines[index].names = signature.names;
        }
        Ok(())
    }

    fn enter_routine(&mut self, r: usize) -> SavedScope {
        let saved = SavedScope {
            scope_len: self.scope.len(),
            first_user_scope: self.first_user_scope,
            next_local: self.next_local,
            local_bytes: std::mem::take(&mut self.local_bytes),
        };
        self.first_user_scope = saved.scope_len;
        self.routine_stack.push(r);
        saved
    }

    fn leave_routine(&mut self, saved: SavedScope) {
        self.routine_stack.pop();
        self.scope.truncate(saved.scope_len);
        self.first_user_scope = saved.first_user_scope;
        self.next_local = saved.next_local;
        self.local_bytes = saved.local_bytes;
    }

    /// Checks the body of routine `r`: its parameters and result become locals (hidden
    /// parameters first), then its declarations and statements are checked, and finally
    /// `place_frame` decides which locals move to the frame.
    fn routine_body(&mut self, r: usize, block: Id) -> R<()> {
        let Kind::Block(declarations, body) = self.ast.kind(block) else {
            unreachable!("the parser makes a block")
        };
        self.routines[r].defined = true;
        let first_symbol = self.syms.len();
        let saved = self.enter_routine(r);
        let first_param = self.routines[r].hidden_params();
        self.next_local = first_param + self.routines[r].params.len() as u32;
        let declared_params = self.next_local;
        if self.routines[r].is_function {
            self.declare_result(r);
        }
        self.declare_params(r, block, first_param)?;
        for decl in self.ast.items_vec(declarations) {
            self.decl(decl)?;
        }
        self.resolve_forward(body)?;
        if self.scope[saved.scope_len..].iter().any(|e| {
            let SymKind::Routine(callee) = self.symbol(e.1).kind else {
                return false;
            };
            !self.routines[callee as usize].defined
        }) {
            return Err(self.err(body, code::UNDEFINED_FORWARD));
        }
        self.stmt(body)?;
        self.routines[r].body = body;
        self.routines[r].nlocals = self.next_local - declared_params;
        self.place_frame(r, first_symbol);
        self.leave_routine(saved);
        Ok(())
    }

    /// The function result is a hidden parameter holding its address when it lives in
    /// memory, otherwise a local after the parameters. It has no name: assignments to the
    /// function's own name are redirected to it.
    fn declare_result(&mut self, r: usize) {
        let routine = &self.routines[r];
        let ty = routine.result;
        let kind = if routine.result_ptr {
            SymKind::Ref(routine.has_static_link() as u32)
        } else {
            let index = self.next_local;
            self.next_local += 1;
            SymKind::Local(index)
        };
        self.routines[r].result_sym = Sym(self.syms.len() as u32);
        self.syms.push(Symbol {
            depth: self.routine_stack.len() as u32,
            ..Symbol::new(kind, ty)
        });
    }

    /// Declares the parameters as locals, the first one being local `first_param`.
    fn declare_params(&mut self, r: usize, block: Id, first_param: u32) -> R<()> {
        let params = self.routines[r].params.clone();
        let names = self.routines[r].names.clone();
        for (i, (&name, param)) in names.iter().zip(&params).enumerate() {
            let index = first_param + i as u32;
            let kind = if param.by_ref {
                SymKind::Ref(index)
            } else {
                SymKind::Local(index)
            };
            let symbol = Symbol {
                needs_frame_slot: !param.by_ref && self.types.in_memory(param.ty),
                ..Symbol::new(kind, param.ty)
            };
            self.declare(block, name, symbol)?;
        }
        Ok(())
    }

    /// Moves the locals that need a frame slot (symbols of this routine from `first_symbol`
    /// on) to the frame, in declaration order with no padding. A routine with a static link
    /// keeps it in the first 4 bytes. A `var` parameter keeps its address in the frame. A
    /// parameter moved this way is copied there by the prologue (`FrameCopy`).
    fn place_frame(&mut self, r: usize, first_symbol: usize) {
        let depth = self.routine_stack.len() as u32;
        let routine = &self.routines[r];
        let parameter_locals = routine.params.len() as u32 + routine.hidden_params();
        let mut size = if routine.has_static_link() {
            STATIC_LINK_BYTES
        } else {
            0
        };
        let first_slot = size;
        let mut copies = Vec::new();
        for s in &mut self.syms[first_symbol..] {
            if s.depth != depth || !s.needs_frame_slot {
                continue;
            }
            let (is_ref, local) = match s.kind {
                SymKind::Local(local) => (false, local),
                SymKind::Ref(local) => (true, local),
                _ => continue,
            };
            // A `var` parameter's frame slot holds its 4-byte address.
            let ty = if is_ref { Ty::LONGINT } else { s.ty };
            if local < parameter_locals {
                copies.push(FrameCopy {
                    local,
                    offset: size,
                    ty,
                });
            }
            s.kind = if is_ref {
                SymKind::FrameRef(size)
            } else {
                SymKind::Frame(size)
            };
            size += self.types.size(ty);
        }
        let routine = &mut self.routines[r];
        routine.framed = size > first_slot || routine.has_children;
        routine.frame = size;
        routine.copies = copies;
    }

    /// A reference to `sym` from the current routine: a `Local` or `Ref` of an enclosing
    /// routine is captured, a `Local` passed as a `var` argument is address-taken; both move
    /// it to the frame.
    fn capture(&mut self, sym: Sym, by_ref: bool) {
        let depth = self.routine_stack.len() as u32;
        let symbol = &mut self.syms[sym.0 as usize];
        let is_local_or_ref = matches!(symbol.kind, SymKind::Local(_) | SymKind::Ref(_));
        let used_by_nested_routine = is_local_or_ref && symbol.depth < depth;
        let passed_by_reference = by_ref && matches!(symbol.kind, SymKind::Local(_));
        symbol.needs_frame_slot |= used_by_nested_routine || passed_by_reference;
    }

    // ---- Variable and procedural arguments ----

    /// TP's error for an argument of a `var` parameter that is not a variable, decided by its
    /// leftmost token; None for a variable.
    fn var_arg_error(&self, e: Id) -> Option<u16> {
        match self.ast.kind(e) {
            Kind::Binary { left, .. } => Some(
                self.var_arg_error(left)
                    .unwrap_or(code::RIGHT_PAREN_EXPECTED),
            ),
            Kind::Call(callee, args) => match self.ast.kind(callee) {
                // A type cast of a variable is a variable.
                Kind::Name(n) if self.type_of(callee, n).is_ok() => {
                    match self.ast.items(args).first() {
                        Some(&arg) => self.var_arg_error(arg),
                        None => Some(code::VARIABLE_EXPECTED),
                    }
                }
                Kind::Name(name) => Some(self.function_reference_error(name, true)),
                _ => Some(code::VARIABLE_EXPECTED),
            },
            Kind::Deref(_) => None,
            Kind::Index { base, .. } | Kind::Field(base, _) => self.var_arg_error(base),
            Kind::Name(name) if self.with_field(name).is_some() => None,
            Kind::Name(name) => match self.lookup_symbol(name) {
                None => Some(code::UNKNOWN_IDENTIFIER),
                Some((_, s)) if s.kind == SymKind::Omitted => Some(code::NOT_SUPPORTED),
                Some((_, s)) if s.kind.is_variable() => None,
                Some(_) => Some(self.function_reference_error(name, false)),
            },
            _ => Some(code::VARIABLE_EXPECTED),
        }
    }

    /// The error for a function name used where a variable is wanted.
    fn function_reference_error(&self, name: u32, is_call: bool) -> u16 {
        match self.lookup_symbol(name).map(|(_, s)| s.kind) {
            Some(SymKind::Routine(r)) if self.routines[r as usize].is_function => {
                let takes_arguments = !self.routines[r as usize].params.is_empty();
                if is_call || !takes_arguments {
                    code::INVALID_VARIABLE_REFERENCE
                } else {
                    code::LEFT_PAREN_EXPECTED
                }
            }
            Some(SymKind::Intrinsic(i)) if i.unary_op().is_some() => {
                code::INVALID_VARIABLE_REFERENCE
            }
            _ => code::VARIABLE_EXPECTED,
        }
    }

    /// An argument of a `var` parameter of type `ty`: a variable of that very type. `at` is
    /// the call.
    fn var_arg(&mut self, at: Id, arg: Id, ty: Ty) -> R<()> {
        if let Some(error) = self.var_arg_error(arg) {
            return Err(self.err(arg, error));
        }
        let named = match self.ast.kind(arg) {
            Kind::Name(name) if self.with_field(name).is_none() => Some(name),
            _ => None,
        };
        let Some(name) = named else {
            self.expr(arg)?;
            if self.ast.ty(arg) != ty {
                return Err(self.err(at, code::TYPE_MISMATCH));
            }
            return Ok(());
        };
        let (sym, symbol) = self.resolve(arg, name)?;
        if symbol.ty != ty {
            return Err(self.err(at, code::TYPE_MISMATCH));
        }
        self.capture(sym, true);
        self.ast.resolve(arg, Kind::Var(sym), ty);
        Ok(())
    }

    /// The routine's signature as a procedural type.
    fn proc_type_of(&mut self, r: usize) -> Ty {
        let params = self.routines[r].params.clone();
        let result = self.routines[r].result;
        self.types.proc(params, result)
    }

    /// `e` where a value of procedural type `want` is expected (assignment source, value
    /// argument): a global routine of that signature or a variable of that type. `at` is the
    /// statement or call.
    fn proc_value(&mut self, at: Id, want: Ty, e: Id) -> R<()> {
        let Kind::Name(name) = self.ast.kind(e) else {
            return Err(self.err(e, code::VARIABLE_EXPECTED));
        };
        let (sym, symbol) = self.resolve(e, name)?;
        let kind = match symbol.kind {
            SymKind::Routine(r) => {
                let r = r as usize;
                if self.routines[r].depth > 1 {
                    return Err(self.err(at, code::INVALID_PROCEDURE_REFERENCE));
                }
                if self.proc_type_of(r) != want {
                    return Err(self.err(at, code::TYPE_MISMATCH));
                }
                self.routines[r].addressed = true;
                Kind::ProcRef(r as u32)
            }
            kind if kind.is_variable() => {
                if symbol.ty != want {
                    return Err(self.err(at, code::TYPE_MISMATCH));
                }
                self.capture(sym, false);
                Kind::Var(sym)
            }
            _ => return Err(self.err(e, code::VARIABLE_EXPECTED)),
        };
        self.ast.resolve(e, kind, want);
        Ok(())
    }

    /// The field `name` of an enclosing `with` record: its designator, type and offset.
    fn with_field(&self, name: u32) -> Option<(Id, Ty, u32)> {
        let found = |&(designator, ty): &(Id, Ty)| {
            let (field_ty, offset) = self.types.field(ty, name)?;
            Some((designator, field_ty, offset))
        };
        self.withs.iter().rev().find_map(found)
    }

    /// The symbol a name refers to; a predefined type this compiler omits is error 200.
    fn resolve(&self, id: Id, name: u32) -> R<(Sym, Symbol)> {
        let (sym, symbol) = self
            .lookup_symbol(name)
            .ok_or(self.err(id, code::UNKNOWN_IDENTIFIER))?;
        match symbol.kind {
            SymKind::Omitted => Err(self.err(id, code::NOT_SUPPORTED)),
            _ => Ok((sym, symbol)),
        }
    }

    // ---- Statements ----

    fn stmt(&mut self, id: Id) -> R<()> {
        match self.ast.kind(id) {
            Kind::Compound(statements) => self.statements(statements)?,
            Kind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.condition(id, condition)?;
                self.stmt(then_branch)?;
                if let Some(else_branch) = else_branch {
                    self.stmt(else_branch)?;
                }
            }
            Kind::While(condition, body) => {
                self.condition(id, condition)?;
                self.stmt(body)?;
            }
            Kind::Repeat(body, condition) => {
                self.statements(body)?;
                self.condition(id, condition)?;
            }
            Kind::Case {
                selector,
                arms,
                else_body,
            } => self.case_statement(id, selector, arms, else_body)?,
            Kind::For {
                variable,
                first,
                last,
                body,
                ..
            } => self.for_statement(id, variable, first, last, body)?,
            Kind::Assign(target, value) => self.assign_statement(id, target, value)?,
            Kind::With(records, body) => self.with_statement(records, body)?,
            Kind::Call(callee, args) => self.call_statement(id, callee, args)?,
            _ => unreachable!("statements only"),
        }
        Ok(())
    }

    fn statements(&mut self, list: List) -> R<()> {
        for statement in self.ast.items_vec(list) {
            self.stmt(statement)?;
        }
        Ok(())
    }

    /// `at` is the statement owning the Boolean condition.
    fn condition(&mut self, at: Id, condition: Id) -> R<()> {
        self.expr(condition)?;
        match self.ast.ty(condition) {
            Ty::BOOLEAN => Ok(()),
            _ => Err(self.err(at, code::BOOLEAN_EXPRESSION_EXPECTED)),
        }
    }

    fn case_statement(&mut self, id: Id, selector: Id, arms: List, else_body: List) -> R<()> {
        self.expr(selector)?;
        let selector_ty = self.ast.ty(selector);
        if self.types.tag(selector_ty).is_none() {
            return Err(self.err(id, code::ORDINAL_EXPRESSION_EXPECTED));
        }
        for arm in self.ast.items_vec(arms) {
            let Kind::Arm(labels, body) = self.ast.kind(arm) else {
                unreachable!("the parser lists arms")
            };
            for label in self.ast.items_vec(labels) {
                self.label(label, selector_ty)?;
            }
            self.stmt(body)?;
        }
        self.statements(else_body)
    }

    /// A case label range: constants of the selector's family, within Integer.
    fn label(&mut self, label: Id, selector_ty: Ty) -> R<()> {
        let Kind::Range(lo, hi) = self.ast.kind(label) else {
            unreachable!("the parser lists ranges")
        };
        // A single label has both ends the same node.
        let ends = if lo == hi { vec![lo] } else { vec![lo, hi] };
        for end in ends {
            self.expr(end)?;
            let node = self.ast.node(end);
            let Kind::IntLit(value) = node.kind else {
                return Err(self.err(label, code::CANNOT_EVALUATE));
            };
            if !self.types.compat(selector_ty, node.ty) {
                return Err(self.err(label, code::CASE_TYPES_MISMATCH));
            }
            let is_integer = self.types.tag(selector_ty) == Some(Tag::Int);
            if is_integer && !(-32768..=32767).contains(&value) {
                return Err(self.err(label, code::CASE_CONSTANT_OUT_OF_RANGE));
            }
        }
        Ok(())
    }

    /// The control variable must be a local of this routine or a global of ordinal type.
    fn for_statement(&mut self, id: Id, variable: Id, first: Id, last: Id, body: Id) -> R<()> {
        let (sym, symbol) = self.resolve(variable, self.name_of(variable))?;
        let is_outer = symbol.depth < self.routine_stack.len() as u32;
        let is_reference = matches!(symbol.kind, SymKind::Ref(_));
        let is_outer_local = matches!(symbol.kind, SymKind::Local(_)) && is_outer;
        if is_reference
            || is_outer_local
            || self.types.sig(symbol.ty).is_some()
            || self.types.tag(symbol.ty).is_none()
        {
            return Err(self.err(variable, code::INVALID_FOR_VARIABLE));
        }
        if !matches!(symbol.kind, SymKind::Var(_) | SymKind::Local(_)) {
            return Err(self.err(variable, code::VARIABLE_EXPECTED));
        }
        self.ast.resolve(variable, Kind::Var(sym), symbol.ty);
        for bound in [first, last] {
            self.expr(bound)?;
            self.assignable(id, symbol.ty, bound)?;
        }
        self.stmt(body)
    }

    fn assign_statement(&mut self, id: Id, target: Id, value: Id) -> R<()> {
        let name = match self.ast.kind(target) {
            Kind::Name(name) if self.with_field(name).is_none() => name,
            _ => {
                // A selector chain, or a field named through `with`.
                self.expr(target)?;
                let ty = self.ast.ty(target);
                return self.assign_value(id, ty, value);
            }
        };
        let (sym, symbol) = self.resolve(target, name)?;
        let (kind, ty) = match symbol.kind {
            kind if kind.is_variable() => {
                self.capture(sym, false);
                (Kind::Var(sym), symbol.ty)
            }
            // Assigning to the name of the function being defined sets its result.
            SymKind::Routine(r) if self.is_enclosing_function(r) => {
                let result = self.routines[r as usize].result_sym;
                self.capture(result, false);
                (Kind::Var(result), self.routines[r as usize].result)
            }
            SymKind::Routine(r) if self.routines[r as usize].is_function => {
                return Err(self.err(target, code::INVALID_VARIABLE_REFERENCE));
            }
            SymKind::Routine(_) => return Err(self.err(target, code::SEMICOLON_EXPECTED)),
            _ => return Err(self.err(target, code::ERROR_IN_STATEMENT)),
        };
        self.ast.resolve(target, kind, ty);
        self.assign_value(id, ty, value)
    }

    /// True if routine `r` is a function being defined now (this one or one around it).
    fn is_enclosing_function(&self, r: u32) -> bool {
        self.routine_stack.contains(&(r as usize)) && self.routines[r as usize].is_function
    }

    /// `at` is the assignment of `value` to a target of type `ty`.
    fn assign_value(&mut self, at: Id, ty: Ty, value: Id) -> R<()> {
        if self.types.sig(ty).is_some() {
            return self.proc_value(at, ty, value);
        }
        self.expr(value)?;
        self.assignable(at, ty, value)
    }

    fn with_statement(&mut self, records: List, body: Id) -> R<()> {
        let outer = self.withs.len();
        for designator in self.ast.items_vec(records) {
            self.expr(designator)?;
            let ty = self.ast.ty(designator);
            if !self.types.is_record(ty) {
                return Err(self.err(designator, code::RECORD_VARIABLE_EXPECTED));
            }
            self.withs.push((designator, ty));
        }
        self.stmt(body)?;
        self.withs.truncate(outer);
        Ok(())
    }

    /// A procedure call statement, or an intrinsic that is a statement. The call node is
    /// rewritten to what codegen expects: `Invoke`, `Indirect`, `Write`, `Exit`, or the
    /// assignment of a helper's result (see the module comment).
    fn call_statement(&mut self, id: Id, callee: Id, args: List) -> R<()> {
        let (_, symbol) = self.resolve(callee, self.name_of(callee))?;
        let kind = match symbol.kind {
            SymKind::Intrinsic(i @ (Intrinsic::Write | Intrinsic::WriteLn)) => {
                for arg in self.ast.items_vec(args) {
                    self.write_arg(arg)?;
                }
                Kind::Write(args, i == Intrinsic::WriteLn)
            }
            SymKind::Intrinsic(i @ (Intrinsic::Delete | Intrinsic::Insert)) => {
                self.edit_string(id, i, args)?
            }
            SymKind::Intrinsic(
                i @ (Intrinsic::New
                | Intrinsic::Dispose
                | Intrinsic::GetMem
                | Intrinsic::FreeMem
                | Intrinsic::Mark
                | Intrinsic::Release
                | Intrinsic::RunError),
            ) => self.heap_call(id, i, args)?,
            SymKind::Intrinsic(i @ (Intrinsic::Inc | Intrinsic::Dec)) => {
                self.inc_dec(id, i, args)?
            }
            SymKind::Intrinsic(Intrinsic::Str) => self.str_call(id, args)?,
            SymKind::Intrinsic(Intrinsic::Val) => self.val_call(id, args)?,
            SymKind::Intrinsic(Intrinsic::Exit) => {
                if !self.ast.items(args).is_empty() {
                    return Err(self.err(id, code::SEMICOLON_EXPECTED));
                }
                Kind::Exit
            }
            SymKind::Routine(r) if !self.routines[r as usize].is_function => {
                let params = self.routines[r as usize].params.clone();
                let builtin = self.routines[r as usize].builtin;
                self.arguments(id, params, args, false, builtin)?;
                Kind::Invoke(r, args)
            }
            SymKind::Routine(_) => return Err(self.err(id, code::INVALID_VARIABLE_REFERENCE)),
            kind if kind.is_variable() => match self.types.sig(symbol.ty) {
                Some((params, Ty::NONE)) => self.indirect(id, callee, params, args, false)?,
                _ => return Err(self.err(id, code::ASSIGN_EXPECTED)),
            },
            _ => return Err(self.err(callee, code::ERROR_IN_STATEMENT)),
        };
        self.ast.set_kind(id, kind);
        Ok(())
    }

    // ---- Arguments ----

    /// Checks the arguments of a call against the parameters; `at` is the call. A routine
    /// without parameters takes no parentheses.
    fn arguments(
        &mut self,
        at: Id,
        params: Vec<ParamType>,
        args: List,
        in_expr: bool,
        builtin: bool,
    ) -> R<()> {
        let items = self.ast.items_vec(args);
        match argument_count_error(params.len(), items.len(), in_expr, builtin) {
            Some(code::LEFT_PAREN_EXPECTED) => return Err(self.missing_argument_list(at)),
            Some(error) => return Err(self.err(at, error)),
            None => {}
        }
        for (&arg, param) in items.iter().zip(&params) {
            if matches!(self.ast.kind(arg), Kind::Fmt { .. }) {
                return Err(self.err(at, code::RIGHT_PAREN_EXPECTED));
            }
            if param.by_ref {
                self.var_arg(at, arg, param.ty)?;
            } else if self.types.sig(param.ty).is_some() {
                self.proc_value(at, param.ty, arg)?;
            } else {
                self.expr(arg)?;
                self.argument_assignable(at, param.ty, arg, builtin)?;
            }
        }
        Ok(())
    }

    /// TP reports a missing argument list at the line of the token after the name.
    fn missing_argument_list(&self, at: Id) -> Diag {
        let follow = self.ast.follow.iter().find(|f| f.node == at);
        let line = follow.map_or(self.ast.node(at).line, |f| f.line);
        Diag {
            line,
            ..self.err(at, code::LEFT_PAREN_EXPECTED)
        }
    }

    /// `assignable` for an argument; TP names the kind of value that a System routine wants.
    fn argument_assignable(&mut self, at: Id, ty: Ty, arg: Id, builtin: bool) -> R<()> {
        self.assignable(at, ty, arg).map_err(|d| {
            let code = if d.code != code::TYPE_MISMATCH || !builtin {
                d.code
            } else if self.types.is_str(ty) {
                code::STRING_EXPRESSION_EXPECTED
            } else if self.types.tag(ty) == Some(Tag::Char) {
                code::CHAR_EXPRESSION_EXPECTED
            } else if self.types.tag(ty) == Some(Tag::Int) {
                code::INTEGER_EXPRESSION_EXPECTED
            } else {
                code::TYPE_MISMATCH
            };
            Diag { code, ..d }
        })
    }

    /// A call through the variable named by `callee` with the signature `params`: the
    /// variable becomes the callee value of the call node `at`.
    fn indirect(
        &mut self,
        at: Id,
        callee: Id,
        params: Vec<ParamType>,
        args: List,
        in_expr: bool,
    ) -> R<Kind> {
        let (sym, symbol) = self.resolve(callee, self.name_of(callee))?;
        self.arguments(at, params, args, in_expr, false)?;
        self.capture(sym, false);
        self.ast.resolve(callee, Kind::Var(sym), symbol.ty);
        Ok(Kind::Indirect(callee, args))
    }

    /// `at` is the assignment; a constant must also fit the destination's range.
    fn assignable(&mut self, at: Id, dest: Ty, value: Id) -> R<()> {
        let node = self.ast.node(value);
        if self.types.is_str(dest) {
            if !self.types.is_str(node.ty) && self.types.tag(node.ty) != Some(Tag::Char) {
                return Err(self.err(at, code::TYPE_MISMATCH));
            }
            self.coerce_str(value);
            return Ok(());
        }
        if dest == Ty::REAL {
            self.coerce_real(value);
            if self.ast.ty(value) != Ty::REAL {
                return Err(self.err(at, code::TYPE_MISMATCH));
            }
            return Ok(());
        }
        if self.types.is_agg(dest) {
            // Name equivalence: only the very same type; sets need compatible bases.
            let compatible_sets = self.types.is_set(dest) && self.types.compat(dest, node.ty);
            if node.ty == dest || compatible_sets {
                return Ok(());
            }
            return Err(self.err(at, code::TYPE_MISMATCH));
        }
        if !self.types.compat(dest, node.ty) {
            return Err(self.err(at, code::TYPE_MISMATCH));
        }
        if let Kind::IntLit(constant) = node.kind {
            let range = self.types.ord(dest);
            if constant < range.lo || constant > range.hi {
                return Err(self.err(at, code::CONSTANT_OUT_OF_RANGE));
            }
        }
        Ok(())
    }

    /// An Integer-family operand `e` used as a Real: a literal becomes a real literal, else an
    /// `IntToReal` node over a copy of `e`. The copy takes a new slot so that `e`, which the
    /// parent refers to, can be overwritten by the wrapper.
    fn coerce_real(&mut self, e: Id) {
        let node = self.ast.node(e);
        if self.types.tag(node.ty) != Some(Tag::Int) {
            return;
        }
        let kind = if let Kind::IntLit(v) = node.kind {
            Kind::RealLit(v as f64)
        } else {
            self.ast.nodes.push(node);
            Kind::IntToReal(self.ast.nodes.len() as Id - 1)
        };
        self.ast.resolve(e, kind, Ty::REAL);
    }

    /// A Char operand `e` used as a String: a one-character literal, else a `CharToStr` node
    /// over a copy of `e` (see `coerce_real`).
    fn coerce_str(&mut self, e: Id) {
        let node = self.ast.node(e);
        if self.types.tag(node.ty) != Some(Tag::Char) {
            return;
        }
        let kind = if let Kind::IntLit(c) = node.kind {
            self.ast.strs.push(vec![c as u8]);
            Kind::StrLit(self.ast.strs.len() as u32 - 1)
        } else {
            self.ast.nodes.push(node);
            Kind::CharToStr(self.ast.nodes.len() as Id - 1)
        };
        self.ast.resolve(e, kind, Ty::STRING);
    }

    /// One argument of `Write`/`WriteLn`: `value`, or `value : width [: decimals]`.
    fn write_arg(&mut self, arg: Id) -> R<()> {
        let mut value = arg;
        let mut decimals = None;
        if let Kind::Fmt {
            value: v,
            width,
            decimals: d,
        } = self.ast.kind(arg)
        {
            value = v;
            decimals = d;
            for field in [Some(width), d].into_iter().flatten() {
                self.expr(field)?;
                if self.types.tag(self.ast.ty(field)) != Some(Tag::Int) {
                    return Err(self.err(arg, code::INTEGER_EXPRESSION_EXPECTED));
                }
            }
        }
        self.expr(value)?;
        let ty = self.ast.ty(value);
        let is_text_or_real = self.types.is_str(ty) || ty == Ty::REAL;
        let is_enum = self.types.tag(ty) == Some(Tag::Enum);
        if is_enum || (!is_text_or_real && self.types.tag(ty).is_none()) {
            return Err(self.err(value, code::CANNOT_READ_OR_WRITE));
        }
        if decimals.is_some() && ty != Ty::REAL {
            return Err(self.err(arg, code::RIGHT_PAREN_EXPECTED));
        }
        Ok(())
    }

    // ---- Expressions ----

    /// Checks the expression `e` and rewrites its node to the resolved kind and type.
    fn expr(&mut self, e: Id) -> R<()> {
        let (kind, ty) = self.expr_kind(e)?;
        self.ast.resolve(e, kind, ty);
        Ok(())
    }

    /// The resolved kind and the type of expression `e`.
    fn expr_kind(&mut self, e: Id) -> R<(Kind, Ty)> {
        let node = self.ast.node(e);
        Ok(match node.kind {
            Kind::IntLit(v) => (node.kind, self.types.const_ty(v)),
            Kind::RealLit(_) => (node.kind, Ty::REAL),
            Kind::StrLit(i) => match self.ast.strs[i as usize][..] {
                // A one-character string is a Char constant until used as a String.
                [c] => (Kind::IntLit(c as i64), Ty::CHAR),
                _ => (node.kind, Ty::STRING),
            },
            Kind::Name(name) => self.name_expr(e, name)?,
            Kind::Index {
                base,
                index,
                continues,
            } => self.index_expr(e, base, index, continues)?,
            Kind::Field(base, name) => self.field_expr(e, base, name)?,
            Kind::Call(callee, args) => self.call_expr(e, callee, args)?,
            Kind::AddrOf(operand) => {
                self.expr(operand)?;
                match self.ast.kind(operand) {
                    Kind::Var(sym) => self.capture(sym, true),
                    Kind::Index { .. } | Kind::Field(..) | Kind::Deref(_) => {}
                    _ => return Err(self.err(e, code::VARIABLE_EXPECTED)),
                }
                (node.kind, Ty::POINTER)
            }
            Kind::Deref(pointer) => {
                self.expr(pointer)?;
                match self.types.target(self.ast.ty(pointer)) {
                    Some(to) => (node.kind, to),
                    None => return Err(self.err(e, code::INVALID_QUALIFIER)),
                }
            }
            Kind::Paren(inner) => {
                self.expr(inner)?;
                let inner = self.ast.node(inner);
                (inner.kind, inner.ty)
            }
            Kind::Unary(op, operand) => self.unary(e, op, operand)?,
            Kind::SetLit(items) => self.set_literal(node.kind, items)?,
            Kind::Range(lo, hi) => {
                self.expr(lo)?;
                self.expr(hi)?;
                let (lo_ty, hi_ty) = (self.ast.ty(lo), self.ast.ty(hi));
                if !self.types.compat(lo_ty, hi_ty) {
                    return Err(self.err(hi, code::TYPE_MISMATCH));
                }
                (node.kind, lo_ty)
            }
            Kind::Binary { op, left, right } => self.binary_expr(e, op, left, right)?,
            _ => unreachable!("expressions only"),
        })
    }

    /// `base[index]`: the base is an array or a string, the index of the index type.
    fn index_expr(&mut self, e: Id, base: Id, index: Id, continues: bool) -> R<(Kind, Ty)> {
        self.expr(base)?;
        let Some((index_ty, lo, hi, element)) = self.types.array_of(self.ast.ty(base)) else {
            let error = if continues {
                code::RIGHT_BRACKET_EXPECTED
            } else {
                code::INVALID_QUALIFIER
            };
            return Err(self.err(e, error));
        };
        self.expr(index)?;
        let index_node = self.ast.node(index);
        if !self.types.compat(index_ty, index_node.ty) {
            return Err(self.err(e, code::TYPE_MISMATCH));
        }
        if matches!(index_node.kind, Kind::IntLit(c) if c < lo || c > hi) {
            return Err(self.err(e, code::CONSTANT_OUT_OF_RANGE));
        }
        Ok((self.ast.kind(e), element))
    }

    /// `base.name` becomes the field at its byte offset.
    fn field_expr(&mut self, e: Id, base: Id, name: u32) -> R<(Kind, Ty)> {
        self.expr(base)?;
        let base_ty = self.ast.ty(base);
        if !self.types.is_record(base_ty) {
            return Err(self.err(e, code::INVALID_QUALIFIER));
        }
        let Some((ty, offset)) = self.types.field(base_ty, name) else {
            return Err(self.err(e, code::FIELD_IDENTIFIER_EXPECTED));
        };
        Ok((Kind::Field(base, offset), ty))
    }

    /// A name used as an operand: a field of a `with` record, a constant, a variable (called
    /// when it is a function-valued procedural variable), or a function called without
    /// arguments.
    fn name_expr(&mut self, e: Id, name: u32) -> R<(Kind, Ty)> {
        if let Some((record, ty, offset)) = self.with_field(name) {
            return Ok((Kind::Field(record, offset), ty));
        }
        let (sym, symbol) = self.resolve(e, name)?;
        Ok(match symbol.kind {
            SymKind::Const(ConstValue::Int(v)) => (Kind::IntLit(v), symbol.ty),
            SymKind::Const(ConstValue::Real(v)) => (Kind::RealLit(v), symbol.ty),
            SymKind::Const(ConstValue::Str(i)) => (Kind::StrLit(i), symbol.ty),
            SymKind::Const(ConstValue::Set(literal)) => (self.ast.kind(literal), symbol.ty),
            kind if kind.is_variable() => match self.types.sig(symbol.ty) {
                // A procedural variable in an expression is called.
                Some((_, Ty::NONE)) => return Err(self.err(e, code::INVALID_PROCEDURE_REFERENCE)),
                Some((params, result)) => {
                    let (line, col) = self.ast.position(e);
                    let callee = self.ast.add(Kind::Name(name), line, col);
                    let no_args = self.ast.list(&[]);
                    (self.indirect(e, callee, params, no_args, true)?, result)
                }
                None => {
                    self.capture(sym, false);
                    (Kind::Var(sym), symbol.ty)
                }
            },
            SymKind::Routine(r) => {
                let no_args = self.ast.list(&[]);
                self.function_call(e, e, r, no_args)?
            }
            SymKind::Intrinsic(i) if i.unary_op().is_some() => {
                return Err(self.err(e, code::LEFT_PAREN_EXPECTED));
            }
            _ => return Err(self.err(e, code::ERROR_IN_EXPRESSION)),
        })
    }

    /// A call of routine `r` in an expression: it must be a function. `not_function_at` is
    /// the node TP reports a procedure used as a value at.
    fn function_call(&mut self, e: Id, not_function_at: Id, r: u32, args: List) -> R<(Kind, Ty)> {
        let routine = &self.routines[r as usize];
        let (is_function, result, builtin) = (routine.is_function, routine.result, routine.builtin);
        if !is_function {
            return Err(self.err(not_function_at, code::INVALID_PROCEDURE_REFERENCE));
        }
        let params = routine.params.clone();
        self.arguments(e, params, args, true, builtin)?;
        Ok((Kind::Invoke(r, args), result))
    }

    /// `name(args)` in an expression: a function, a procedural variable, an intrinsic, or a
    /// type cast.
    fn call_expr(&mut self, e: Id, callee: Id, args: List) -> R<(Kind, Ty)> {
        let (_, symbol) = self.resolve(callee, self.name_of(callee))?;
        Ok(match symbol.kind {
            SymKind::Routine(r) => {
                let call = self.function_call(e, callee, r, args)?;
                self.folded_length(callee, r, args).unwrap_or(call)
            }
            kind if kind.is_variable() => match self.types.sig(symbol.ty) {
                Some((_, Ty::NONE)) => {
                    return Err(self.err(callee, code::INVALID_PROCEDURE_REFERENCE))
                }
                Some((params, result)) => (self.indirect(e, callee, params, args, true)?, result),
                None => return Err(self.err(e, code::RIGHT_PAREN_EXPECTED)),
            },
            SymKind::Intrinsic(i) if i.unary_op().is_some() => {
                let items = self.ast.items_vec(args);
                if items.len() > 1 {
                    return Err(self.err(e, code::RIGHT_PAREN_EXPECTED));
                }
                self.unary(e, i.unary_op().unwrap_or(Op::Ord), items[0])?
            }
            SymKind::Intrinsic(Intrinsic::SizeOf) => self.size_of(e, args)?,
            SymKind::Intrinsic(Intrinsic::Concat) => self.concat(e, args)?,
            SymKind::Type => self.cast(e, symbol.ty, args)?,
            SymKind::Intrinsic(_) => return Err(self.err(callee, code::ERROR_IN_EXPRESSION)),
            _ => return Err(self.err(e, code::RIGHT_PAREN_EXPECTED)),
        })
    }

    /// System's `Length` of a string constant is a constant.
    fn folded_length(&self, callee: Id, r: u32, args: List) -> Option<(Kind, Ty)> {
        let is_length = self.routines[r as usize].builtin
            && self.ast.names[self.name_of(callee) as usize] == b"LENGTH";
        let Kind::StrLit(s) = self.ast.kind(*self.ast.items(args).first()?) else {
            return None;
        };
        let length = self.ast.strs[s as usize].len() as i64;
        is_length.then(|| (Kind::IntLit(length), self.types.const_ty(length)))
    }

    /// `[a, b..c]`: the base type is the type of the elements, which must agree.
    fn set_literal(&mut self, kind: Kind, items: List) -> R<(Kind, Ty)> {
        let mut base = Ty::NONE;
        for item in self.ast.items_vec(items) {
            self.expr(item)?;
            let ty = self.ast.ty(item);
            base = self
                .types
                .element(base, ty)
                .map_err(|code| self.err(item, code))?;
        }
        Ok((kind, self.types.set(base)))
    }

    /// A binary operator: typed by `Types::binary`, operands converted to String or Real as
    /// the operator needs, and folded when both operands are constants.
    fn binary_expr(&mut self, e: Id, op: Op, left: Id, right: Id) -> R<(Kind, Ty)> {
        self.expr(left)?;
        self.expr(right)?;
        let (left_ty, right_ty) = (self.ast.ty(left), self.ast.ty(right));
        let ty = self
            .types
            .binary(op, left_ty, right_ty)
            .map_err(|code| self.err(e, code))?;
        if ty == Ty::STRING || self.types.is_str(left_ty) || self.types.is_str(right_ty) {
            self.coerce_str(left);
            self.coerce_str(right);
        }
        if op == Op::Slash || left_ty == Ty::REAL || right_ty == Ty::REAL {
            self.coerce_real(left);
            self.coerce_real(right);
        }
        match (self.ast.kind(left), self.ast.kind(right)) {
            (Kind::IntLit(_), Kind::IntLit(0)) if matches!(op, Op::Div | Op::Mod) => {
                Err(self.err(e, code::DIVISION_BY_ZERO))
            }
            (Kind::RealLit(x), Kind::RealLit(y))
                if ty == Ty::REAL && (op != Op::Slash || y != 0.0) =>
            {
                let value = match op {
                    Op::Add => x + y,
                    Op::Sub => x - y,
                    Op::Mul => x * y,
                    _ => x / y,
                };
                Ok((Kind::RealLit(value), ty))
            }
            (Kind::IntLit(x), Kind::IntLit(y)) if ty != Ty::STRING => {
                Ok(self.constant(fold(op, x, y), ty))
            }
            (Kind::StrLit(x), Kind::StrLit(y)) if op == Op::Add => {
                let mut joined = self.ast.strs[x as usize].clone();
                joined.extend_from_slice(&self.ast.strs[y as usize]);
                joined.truncate(255);
                self.ast.strs.push(joined);
                Ok((Kind::StrLit(self.ast.strs.len() as u32 - 1), ty))
            }
            _ => Ok((self.ast.kind(e), ty)),
        }
    }

    /// A unary operator or intrinsic applied to `a`, folded when `a` is constant (`Sqr` is
    /// not constant in TP).
    fn unary(&mut self, e: Id, op: Op, a: Id) -> R<(Kind, Ty)> {
        self.expr(a)?;
        let arg_ty = self.ast.ty(a);
        let mut ty = self
            .types
            .unary(op, arg_ty)
            .map_err(|code| self.err(e, code))?;
        if matches!(op, Op::Round | Op::Trunc | Op::Sqrt | Op::Int | Op::Frac) {
            self.coerce_real(a);
        }
        let operand = self.ast.node(a);
        if op == Op::Ord && self.types.tag(operand.ty) == Some(Tag::Enum) {
            // TP types Ord(enum) by the enumeration's range (`Ord(c) + w` is a Word sum).
            let range = self.types.ord(operand.ty);
            ty = self.types.subrange(range.lo, range.hi, Ty::INTEGER);
        }
        Ok(match operand.kind {
            Kind::RealLit(v) if op == Op::Neg => (Kind::RealLit(-v), ty),
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

    /// `T(x)`: the ordinal value of `x` as type `ty`.
    fn cast(&mut self, at: Id, ty: Ty, args: List) -> R<(Kind, Ty)> {
        let [arg] = self.ast.items(args)[..] else {
            return Err(self.err(at, code::RIGHT_PAREN_EXPECTED));
        };
        self.expr(arg)?;
        let arg_node = self.ast.node(arg);
        let is_scalar = |t: Ty| self.types.tag(t).is_some() || self.types.is_ptr(t);
        if !is_scalar(ty) || !is_scalar(arg_node.ty) {
            return Err(self.err(at, code::NOT_SUPPORTED));
        }
        Ok(match arg_node.kind {
            Kind::IntLit(v) => self.constant(v, ty),
            _ => (Kind::Unary(Op::Ord, arg), ty),
        })
    }

    /// `Concat(a, b, ...)`: the same as `a + b + ...`.
    fn concat(&mut self, at: Id, args: List) -> R<(Kind, Ty)> {
        let items = self.ast.items_vec(args);
        if items.is_empty() {
            return Err(self.err(at, code::ERROR_IN_EXPRESSION));
        }
        let (line, col) = self.ast.position(at);
        let mut sum = items[0];
        for (i, &item) in items.iter().enumerate() {
            self.expr(item)?;
            let ty = self.ast.ty(item);
            if !self.types.is_str(ty) && self.types.tag(ty) != Some(Tag::Char) {
                return Err(self.err(at, code::STRING_EXPRESSION_EXPECTED));
            }
            self.coerce_str(item);
            if i > 0 {
                let add = Kind::Binary {
                    op: Op::Add,
                    left: sum,
                    right: item,
                };
                sum = self.ast.add(add, line, col);
                self.ast.set_ty(sum, Ty::STRING);
            }
        }
        let node = self.ast.node(sum);
        Ok((node.kind, node.ty))
    }

    /// `SizeOf(type name or variable)`, a constant.
    fn size_of(&mut self, at: Id, args: List) -> R<(Kind, Ty)> {
        let [arg] = self.ast.items(args)[..] else {
            return Err(self.err(at, code::RIGHT_PAREN_EXPECTED));
        };
        let ty = match self.ast.kind(arg) {
            Kind::Name(name) if self.with_field(name).is_none() => {
                let (_, symbol) = self.resolve(arg, name)?;
                if matches!(symbol.kind, SymKind::Const(_) | SymKind::Routine(_)) {
                    return Err(self.err(arg, code::VARIABLE_EXPECTED));
                }
                symbol.ty
            }
            _ => {
                self.expr(arg)?;
                self.ast.ty(arg)
            }
        };
        Ok(self.constant(self.types.size(ty) as i64, Ty::INTEGER))
    }

    // ---- Intrinsics rewritten into assignments of helper results ----

    /// Error 87 (too few) or 89 (too many) at `at` unless `min..=max` arguments were given.
    fn expect_arg_count(&self, at: Id, count: usize, min: usize, max: usize) -> R<()> {
        if count < min {
            return Err(self.err(at, code::COMMA_EXPECTED));
        }
        if count > max {
            return Err(self.err(at, code::RIGHT_PAREN_EXPECTED));
        }
        Ok(())
    }

    /// A variable argument (TP's error otherwise) and its type.
    fn variable(&mut self, e: Id) -> R<Ty> {
        if let Some(error) = self.var_arg_error(e) {
            return Err(self.err(e, error));
        }
        self.expr(e)?;
        Ok(self.ast.ty(e))
    }

    /// A call of System helper `helper` with `args`, as a node of type `ty` at `at`.
    fn helper_call(&mut self, at: Id, helper: Helper, args: &[Id], ty: Ty) -> Id {
        let list = self.ast.list(args);
        let call = Kind::Invoke(self.helpers[helper as usize], list);
        self.ast.typed(call, ty, at)
    }

    /// `Delete(s, i, n)` is `s := StrDelete(s, i, n)`, `Insert(src, s, i)` is
    /// `s := StrInsert(src, s, i)`: the call becomes that assignment.
    fn edit_string(&mut self, at: Id, which: Intrinsic, args: List) -> R<Kind> {
        let items = self.ast.items_vec(args);
        self.expect_arg_count(at, items.len(), 3, 3)?;
        let is_insert = which == Intrinsic::Insert;
        let target = if is_insert { items[1] } else { items[0] };
        let ty = self.variable(target)?;
        if !self.types.is_str(ty) {
            return Err(self.err(target, code::STRING_VARIABLE_EXPECTED));
        }
        // The other two arguments: the string to insert and a position, or two counts.
        let others = if is_insert {
            [(items[0], Ty::STRING), (items[2], Ty::INTEGER)]
        } else {
            [(items[1], Ty::INTEGER), (items[2], Ty::INTEGER)]
        };
        for (arg, wanted) in others {
            self.expr(arg)?;
            self.argument_assignable(at, wanted, arg, true)?;
        }
        let helper = if is_insert {
            Helper::StrInsert
        } else {
            Helper::StrDelete
        };
        let list = self.ast.list(&items);
        let call = Kind::Invoke(self.helpers[helper as usize], list);
        let call = self.ast.typed(call, Ty::STRING, at);
        Ok(Kind::Assign(target, call))
    }

    /// `Inc(x [, n])` is `x := Ord(x) + n`, `Dec` subtracts; `n` defaults to 1.
    fn inc_dec(&mut self, at: Id, which: Intrinsic, args: List) -> R<Kind> {
        let items = self.ast.items_vec(args);
        self.expect_arg_count(at, items.len(), 1, 2)?;
        let target = items[0];
        let ty = self.variable(target)?;
        let result = self
            .types
            .step_type(ty)
            .map_err(|code| self.err(target, code))?;
        let step = match items.get(1) {
            Some(&n) => {
                self.expr(n)?;
                self.argument_assignable(at, Ty::LONGINT, n, true)?;
                n
            }
            None => self.ast.typed(Kind::IntLit(1), Ty::INTEGER, at),
        };
        let current = self.ast.typed(Kind::Unary(Op::Ord, target), result, at);
        let op = if which == Intrinsic::Inc {
            Op::Add
        } else {
            Op::Sub
        };
        let sum = Kind::Binary {
            op,
            left: current,
            right: step,
        };
        let sum = self.ast.typed(sum, result, at);
        // Line 0: outside every `{$R+}` region, so `Inc` and `Dec` are never range checked.
        self.ast.nodes[sum as usize].line = 0;
        Ok(Kind::Assign(target, sum))
    }

    /// `Str(v [: w [: d]], s)` is `s := StrInt(v, w, max)` for an integer and a formatted
    /// Real otherwise, `max` being the capacity of `s`.
    fn str_call(&mut self, at: Id, args: List) -> R<Kind> {
        let items = self.ast.items_vec(args);
        self.expect_arg_count(at, items.len(), 2, 2)?;
        let target = items[1];
        let ty = self.variable(target)?;
        self.write_arg(items[0])?;
        let (value, width, decimals) = match self.ast.kind(items[0]) {
            Kind::Fmt {
                value,
                width,
                decimals,
            } => (value, Some(width), decimals),
            _ => (items[0], None, None),
        };
        let value_ty = self.ast.ty(value);
        let result = self
            .types
            .str_type(ty, value_ty)
            .map_err(|code| self.err(target, code))?;
        let width = match width {
            Some(width) => width,
            None => self.ast.typed(Kind::IntLit(-1), Ty::INTEGER, at),
        };
        let converted = if value_ty == Ty::REAL {
            let format = Kind::Fmt {
                value,
                width,
                decimals,
            };
            self.ast.typed(format, result, at)
        } else {
            let max = self.types.size(ty) as i64 - 1;
            let max = self.ast.typed(Kind::IntLit(max), Ty::INTEGER, at);
            self.helper_call(at, Helper::StrInt, &[value, width, max], result)
        };
        Ok(Kind::Assign(target, converted))
    }

    /// `Val(s, v, code)` is `v := ValLong(s, code)`, or `ValReal` for a Real `v`.
    fn val_call(&mut self, at: Id, args: List) -> R<Kind> {
        let items = self.ast.items_vec(args);
        self.expect_arg_count(at, items.len(), 3, 3)?;
        let (source, target, error_code) = (items[0], items[1], items[2]);
        let ty = self.variable(target)?;
        let result = self
            .types
            .val_type(ty)
            .map_err(|code| self.err(target, code))?;
        self.expr(source)?;
        self.argument_assignable(at, Ty::STRING, source, true)?;
        self.var_arg(at, error_code, Ty::INTEGER)
            .map_err(|d| Diag {
                code: if d.code == code::TYPE_MISMATCH {
                    code::INTEGER_VARIABLE_EXPECTED
                } else {
                    d.code
                },
                ..d
            })?;
        let helper = if result == Ty::REAL {
            Helper::ValReal
        } else {
            Helper::ValLong
        };
        let call = self.helper_call(at, helper, &[source, error_code], result);
        Ok(Kind::Assign(target, call))
    }

    /// `New`, `Dispose`, `GetMem`, `FreeMem`, `Mark`, `Release` and `RunError`: calls of the
    /// System heap helpers, with `SizeOf(p^)` as the size of `New` and `Dispose`. `New`,
    /// `GetMem` and `Mark` assign the helper's result to the variable.
    fn heap_call(&mut self, at: Id, which: Intrinsic, args: List) -> R<Kind> {
        let items = self.ast.items_vec(args);
        let count = if matches!(which, Intrinsic::GetMem | Intrinsic::FreeMem) {
            2
        } else {
            1
        };
        self.expect_arg_count(at, items.len(), count, count)?;
        let p = items[0];
        if which == Intrinsic::RunError {
            self.expr(p)?;
            self.argument_assignable(at, Ty::INTEGER, p, true)?;
            return Ok(Kind::RunErr(p));
        }
        let ty = self.variable(p)?;
        let to = self
            .types
            .target(ty)
            .ok_or(self.err(p, code::POINTER_VARIABLE_EXPECTED))?;
        let size = match items.get(1) {
            Some(&size) => {
                self.expr(size)?;
                self.argument_assignable(at, Ty::LONGINT, size, true)?;
                size
            }
            None => {
                let size_of_target = Kind::IntLit(self.types.size(to) as i64);
                self.ast.typed(size_of_target, Ty::LONGINT, at)
            }
        };
        let (helper, helper_args) = match which {
            Intrinsic::New | Intrinsic::GetMem => (Helper::HeapGet, vec![size]),
            Intrinsic::Dispose | Intrinsic::FreeMem => (Helper::HeapPut, vec![p, size]),
            Intrinsic::Mark => (Helper::HeapMark, vec![]),
            _ => (Helper::HeapRelease, vec![p]),
        };
        let list = self.ast.list(&helper_args);
        let call = Kind::Invoke(self.helpers[helper as usize], list);
        let returns_nothing = matches!(
            which,
            Intrinsic::Dispose | Intrinsic::FreeMem | Intrinsic::Release
        );
        if returns_nothing {
            return Ok(call);
        }
        let call = self.ast.typed(call, Ty::POINTER, at);
        Ok(Kind::Assign(p, call))
    }
}

/// TP's error for the number of arguments in a call, or None if it matches. A routine with
/// no parameters takes no parentheses: error 85, or 89 inside an expression.
fn argument_count_error(params: usize, args: usize, in_expr: bool, builtin: bool) -> Option<u16> {
    match (params, args) {
        (0, 0) => None,
        (0, _) if in_expr && !builtin => Some(code::RIGHT_PAREN_EXPECTED),
        (0, _) => Some(code::SEMICOLON_EXPECTED),
        (_, 0) => Some(code::LEFT_PAREN_EXPECTED),
        (p, a) if a < p => Some(code::COMMA_EXPECTED),
        (p, a) if a > p => Some(code::RIGHT_PAREN_EXPECTED),
        _ => None,
    }
}
