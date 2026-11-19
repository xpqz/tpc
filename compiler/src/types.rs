//! Types, symbols and the operator typing rules (decisions 5 and 10).
//!
//! `Types` is a table of `Type` values; a `Ty` is an index into it, and the predefined types
//! have fixed indexes (`Ty::INTEGER`, ...). Types compare by identity (name equivalence),
//! except that `compat` decides which ordinal, pointer and set types may be mixed. A
//! subrange or enumeration keeps the predefined type that decides its arithmetic as its
//! `base`. Symbols record what each declared name stands for and where it is stored.
use crate::ast::{Id, Op};
use crate::diag::code;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Ty(pub u32);

/// Bytes of a set: 256 bits, bit `i` of byte `i div 8` is element `i`.
pub const SET_BYTES: u32 = 32;

impl Ty {
    pub const NONE: Ty = Ty(0);
    pub const STRING: Ty = Ty(1);
    pub const BYTE: Ty = Ty(2);
    pub const SHORTINT: Ty = Ty(3);
    pub const INTEGER: Ty = Ty(4);
    pub const WORD: Ty = Ty(5);
    pub const LONGINT: Ty = Ty(6);
    pub const BOOLEAN: Ty = Ty(7);
    pub const CHAR: Ty = Ty(8);
    /// The untyped `Pointer`, also the type of `nil` and `@x`.
    pub const POINTER: Ty = Ty(9);
    /// IEEE f64, kept in memory like a string; arithmetic is on the WASM stack.
    pub const REAL: Ty = Ty(10);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tag {
    Int,
    Bool,
    Char,
    /// Enumerated types (and their subranges) are compatible only with the same enumeration.
    Enum,
}

/// A record field and its byte offset.
pub struct Field {
    pub name: u32,
    pub ty: Ty,
    pub offset: u32,
}

/// Range, storage size and signedness of an ordinal type.
#[derive(Clone, Copy)]
pub struct OrdInfo {
    pub lo: i64,
    pub hi: i64,
    pub size: u8,
    pub signed: bool,
}

pub enum Type {
    None,
    Real,
    /// `String[n]`.
    Str(u8),
    /// A procedural type: parameters and the result (`Ty::NONE` for a procedure). Interned by
    /// content, so equal signatures are the same type.
    Proc {
        params: Vec<ParamType>,
        result: Ty,
    },
    /// A pointer to the type (`Ty::NONE` for the untyped `Pointer`): 4 bytes, unsigned.
    Ptr(Ty),
    /// A set of the ordinal type (`Ty::NONE` for `[]`): 32 bytes, bit i is element i.
    Set(Ty),
    /// One dimension; `a[i, j]` is an array of arrays. `size` saturates at u32::MAX.
    Array {
        index: Ty,
        elem: Ty,
        size: u32,
    },
    /// Fields in declaration order, variants overlaid, and the size.
    Record {
        fields: Vec<Field>,
        size: u32,
    },
    Ord {
        lo: i64,
        hi: i64,
        size: u8,
        signed: bool,
        tag: Tag,
        /// The type that decides arithmetic: a predefined integer, Char, Boolean or the
        /// enumeration itself; a type is its own base unless it is a subrange.
        base: Ty,
    },
}

/// Index into the symbol table.
#[derive(Clone, Copy)]
pub struct Sym(pub u32);

/// The value of a named constant.
#[derive(Clone, Copy, PartialEq)]
pub enum ConstValue {
    /// Integer, Boolean and Char constants, and enumeration members (their ordinal).
    Int(i64),
    Real(f64),
    /// Index into `Ast::strs`.
    Str(u32),
    /// The node of a set constructor whose elements are all constants.
    Set(Id),
}

/// What a name stands for, with the one number that locates it.
#[derive(Clone, Copy, PartialEq)]
pub enum SymKind {
    Prog,
    Type,
    /// A predefined type this compiler does not support.
    Omitted,
    Const(ConstValue),
    /// A global variable at this static-data address.
    Var(u32),
    /// The WASM local with this index; parameters and the result come first.
    Local(u32),
    /// A local that lives in the frame, at this offset from `fp`.
    Frame(u32),
    /// A `var` parameter: the WASM local with this index holds the address.
    Ref(u32),
    /// A `var` parameter captured by a nested routine: the frame offset of its address.
    FrameRef(u32),
    /// The index into the routine table.
    Routine(u32),
    Intrinsic(Intrinsic),
}

impl SymKind {
    /// Kinds that name a storage location a program can read and assign.
    pub fn is_variable(self) -> bool {
        matches!(self, SymKind::Var(_) | SymKind::Local(_) | SymKind::Ref(_))
    }
}

/// The predefined routines that the compiler expands itself instead of calling System.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Intrinsic {
    Write,
    WriteLn,
    Ord,
    Chr,
    Succ,
    Pred,
    Odd,
    Abs,
    Sqr,
    Exit,
    SizeOf,
    Concat,
    Delete,
    Insert,
    New,
    Dispose,
    GetMem,
    FreeMem,
    Mark,
    Release,
    RunError,
    Round,
    Trunc,
    Sqrt,
    Int,
    Frac,
    Inc,
    Dec,
    Str,
    Val,
}

impl Intrinsic {
    /// The operator behind an intrinsic function of one argument.
    pub fn unary_op(self) -> Option<Op> {
        match self {
            Intrinsic::Ord => Some(Op::Ord),
            Intrinsic::Chr => Some(Op::Chr),
            Intrinsic::Succ => Some(Op::Succ),
            Intrinsic::Pred => Some(Op::Pred),
            Intrinsic::Odd => Some(Op::Odd),
            Intrinsic::Abs => Some(Op::Abs),
            Intrinsic::Sqr => Some(Op::Sqr),
            Intrinsic::Round => Some(Op::Round),
            Intrinsic::Trunc => Some(Op::Trunc),
            Intrinsic::Sqrt => Some(Op::Sqrt),
            Intrinsic::Int => Some(Op::Int),
            Intrinsic::Frac => Some(Op::Frac),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Symbol {
    pub kind: SymKind,
    pub ty: Ty,
    /// The variable must live in the frame: it is passed as a `var` argument, has its address
    /// taken, or is used by a nested routine. The routine's frame layout moves it there.
    pub needs_frame_slot: bool,
    /// Lexical depth of the owning routine (0 for the program).
    pub depth: u32,
}

impl Symbol {
    pub fn new(kind: SymKind, ty: Ty) -> Self {
        Symbol {
            kind,
            ty,
            needs_frame_slot: false,
            depth: 0,
        }
    }
}

/// One parameter of a procedural type or routine.
#[derive(Clone, Copy, PartialEq)]
pub struct ParamType {
    pub ty: Ty,
    pub by_ref: bool,
}

/// A value parameter copied into the frame by the routine's prologue.
pub struct FrameCopy {
    pub local: u32,
    pub offset: u32,
    pub ty: Ty,
}

/// A declared procedure or function.
pub struct Routine {
    pub params: Vec<ParamType>,
    /// The parameter names, in the same order.
    pub names: Vec<u32>,
    /// `Ty::NONE` for a procedure.
    pub result: Ty,
    pub is_function: bool,
    pub defined: bool,
    /// Body statement, once defined.
    pub body: Id,
    /// Locals after the parameters, the function result included.
    pub nlocals: u32,
    /// Lexical depth: top-level routines are 1; deeper ones take the static link as
    /// parameter 0.
    pub depth: u32,
    /// Contains nested routines, which receive its frame pointer.
    pub has_children: bool,
    /// The routine has a frame of `frame` bytes (the static link slot included).
    pub framed: bool,
    pub frame: u32,
    /// The symbol of the function result.
    pub result_sym: Sym,
    /// Value parameters moved to the frame.
    pub copies: Vec<FrameCopy>,
    /// Declared in System: its arguments are diagnosed like those of a TP built-in.
    pub builtin: bool,
    /// The result is a String, Real or other in-memory value: the caller passes its address
    /// as a hidden parameter.
    pub result_ptr: bool,
    /// Its address is taken as a procedural value: it has a table slot.
    pub addressed: bool,
    /// An `external` routine: the name of the host import that implements it.
    pub import: Option<u32>,
}

impl Routine {
    /// A routine as just declared: no body, no frame yet.
    pub fn new(signature: Signature, depth: u32, result_ptr: bool, builtin: bool) -> Self {
        Routine {
            params: signature.params,
            names: signature.names,
            result: signature.result,
            is_function: signature.is_function,
            defined: false,
            body: 0,
            nlocals: 0,
            depth,
            has_children: false,
            framed: false,
            frame: 0,
            result_sym: Sym(0),
            copies: Vec::new(),
            builtin,
            result_ptr,
            addressed: false,
            import: None,
        }
    }

    /// A routine nested in another receives the enclosing frame pointer, its static link, as
    /// its first parameter.
    pub fn has_static_link(&self) -> bool {
        self.depth > 1
    }

    /// Parameters the call passes that the declaration does not list: the static link, then
    /// the address of the result when it lives in memory. They come before the declared ones.
    pub fn hidden_params(&self) -> u32 {
        self.has_static_link() as u32 + self.result_ptr as u32
    }
}

/// What a routine declaration states.
pub struct Signature {
    pub params: Vec<ParamType>,
    pub names: Vec<u32>,
    pub result: Ty,
    pub is_function: bool,
}

pub struct Types {
    table: Vec<Type>,
}

impl Types {
    pub fn new() -> Self {
        // lo, hi, size in bytes, signed, family, and the type that decides arithmetic.
        let ord = |lo, hi, size, signed, tag, base| Type::Ord {
            lo,
            hi,
            size,
            signed,
            tag,
            base,
        };
        Types {
            table: vec![
                Type::None,
                Type::Str(255),
                ord(0, 255, 1, false, Tag::Int, Ty::BYTE),
                ord(-128, 127, 1, true, Tag::Int, Ty::SHORTINT),
                ord(-32768, 32767, 2, true, Tag::Int, Ty::INTEGER),
                ord(0, 65535, 2, false, Tag::Int, Ty::WORD),
                ord(-(1 << 31), (1 << 31) - 1, 4, true, Tag::Int, Ty::LONGINT),
                ord(0, 1, 1, false, Tag::Bool, Ty::BOOLEAN),
                ord(0, 255, 1, false, Tag::Char, Ty::CHAR),
                Type::Ptr(Ty::NONE),
                Type::Real,
            ],
        }
    }

    pub fn get(&self, t: Ty) -> &Type {
        &self.table[t.0 as usize]
    }

    /// Replaces the placeholder of a forward pointer type once its target is known.
    pub fn set_pointer_target(&mut self, pointer: Ty, to: Ty) {
        self.table[pointer.0 as usize] = Type::Ptr(to);
    }

    pub fn count(&self) -> usize {
        self.table.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Type> {
        self.table.iter()
    }

    fn add(&mut self, t: Type) -> Ty {
        self.table.push(t);
        Ty(self.table.len() as u32 - 1)
    }

    pub fn tag(&self, t: Ty) -> Option<Tag> {
        match self.get(t) {
            Type::Ord { tag, .. } => Some(*tag),
            _ => None,
        }
    }

    /// Storage size in bytes.
    pub fn size(&self, t: Ty) -> u32 {
        match self.get(t) {
            Type::Array { size, .. } | Type::Record { size, .. } => *size,
            Type::Str(n) => *n as u32 + 1,
            Type::Proc { .. } | Type::Ptr(_) => 4,
            Type::Set(_) => SET_BYTES,
            Type::Real => 8,
            Type::Ord { size, .. } => *size as u32,
            Type::None => 0,
        }
    }

    /// Arrays, records and sets, copied whole by `memory.copy`.
    pub fn is_agg(&self, t: Ty) -> bool {
        matches!(
            self.get(t),
            Type::Array { .. } | Type::Record { .. } | Type::Set(_)
        )
    }

    /// The element type of a set type.
    pub fn set_base(&self, t: Ty) -> Option<Ty> {
        match self.get(t) {
            Type::Set(base) => Some(*base),
            _ => None,
        }
    }

    pub fn is_set(&self, t: Ty) -> bool {
        self.set_base(t).is_some()
    }

    /// A new set type; each constructor and declaration makes its own.
    pub fn set(&mut self, base: Ty) -> Ty {
        self.add(Type::Set(base))
    }

    /// `set of base` as declared: an ordinal type within 0..255, else the TP error number.
    pub fn set_of(&mut self, base: Ty) -> Result<Ty, u16> {
        if self.tag(base).is_none() {
            return Err(code::ORDINAL_TYPE_EXPECTED);
        }
        let info = self.ord(base);
        if info.lo < 0 || info.hi > 255 {
            return Err(code::SET_BASE_OUT_OF_RANGE);
        }
        Ok(self.set(base))
    }

    /// The element type of a constructor holding `base` (`Ty::NONE` if empty) and an element
    /// of type `ty`, or the TP error number.
    pub fn element(&self, base: Ty, ty: Ty) -> Result<Ty, u16> {
        if self.tag(ty).is_none() {
            return Err(code::ORDINAL_EXPRESSION_EXPECTED);
        }
        if base != Ty::NONE && !self.compat(base, ty) {
            return Err(code::TYPE_MISMATCH);
        }
        Ok(ty)
    }

    pub fn is_ptr(&self, t: Ty) -> bool {
        matches!(self.get(t), Type::Ptr(_))
    }

    /// A new pointer type to `to`.
    pub fn pointer(&mut self, to: Ty) -> Ty {
        self.add(Type::Ptr(to))
    }

    /// What a pointer type points to (`Ty::NONE` for `Pointer`).
    pub fn target(&self, t: Ty) -> Option<Ty> {
        match self.get(t) {
            Type::Ptr(to) => Some(*to),
            _ => None,
        }
    }

    pub fn is_str(&self, t: Ty) -> bool {
        matches!(self.get(t), Type::Str(_))
    }

    /// Types that live in memory and are passed by address.
    pub fn in_memory(&self, t: Ty) -> bool {
        self.is_agg(t) || self.is_str(t) || t == Ty::REAL
    }

    /// A new `String[n]`; each form is a type of its own.
    pub fn string(&mut self, n: u8) -> Ty {
        self.add(Type::Str(n))
    }

    /// Capacity of a string type.
    pub fn capacity(&self, t: Ty) -> u8 {
        match self.get(t) {
            Type::Str(n) => *n,
            _ => 0,
        }
    }

    /// `array[index] of elem`.
    pub fn array(&mut self, index: Ty, elem: Ty) -> Ty {
        let info = self.ord(index);
        let count = info.hi - info.lo + 1;
        let size = (count * self.size(elem) as i64).min(u32::MAX as i64) as u32;
        self.add(Type::Array { index, elem, size })
    }

    /// (index type, lower bound, upper bound, element type) of an array type; a string is
    /// an array of Char from 0 (the length byte).
    pub fn array_of(&self, t: Ty) -> Option<(Ty, i64, i64, Ty)> {
        match self.get(t) {
            Type::Array { index, elem, .. } => {
                let info = self.ord(*index);
                Some((*index, info.lo, info.hi, *elem))
            }
            Type::Str(n) => Some((Ty::INTEGER, 0, *n as i64, Ty::CHAR)),
            _ => None,
        }
    }

    pub fn is_record(&self, t: Ty) -> bool {
        matches!(self.get(t), Type::Record { .. })
    }

    pub fn record(&mut self, fields: Vec<Field>, size: u32) -> Ty {
        self.add(Type::Record { fields, size })
    }

    /// (type, offset) of a record field.
    pub fn field(&self, t: Ty, name: u32) -> Option<(Ty, u32)> {
        match self.get(t) {
            Type::Record { fields, .. } => fields
                .iter()
                .find(|f| f.name == name)
                .map(|f| (f.ty, f.offset)),
            _ => None,
        }
    }

    /// The type that decides arithmetic and compatibility of `t`.
    pub fn base(&self, t: Ty) -> Ty {
        match self.get(t) {
            Type::Ord { base, .. } => *base,
            _ => t,
        }
    }

    /// Ordinal types of one family (enumerations must also be the same enumeration), or
    /// pointers of one type or with the untyped `Pointer`.
    pub fn compat(&self, a: Ty, b: Ty) -> bool {
        if let (Some(x), Some(y)) = (self.set_base(a), self.set_base(b)) {
            return x == Ty::NONE || y == Ty::NONE || self.compat(x, y);
        }
        match (self.tag(a), self.tag(b)) {
            (Some(x), Some(y)) => x == y && (x != Tag::Enum || self.base(a) == self.base(b)),
            _ => {
                self.is_ptr(a) && self.is_ptr(b) && (a == b || a == Ty::POINTER || b == Ty::POINTER)
            }
        }
    }

    /// A new enumeration of `n` values.
    pub fn enumeration(&mut self, n: i64) -> Ty {
        // An enumeration is its own base: the handle it is about to get.
        let own_handle = Ty(self.table.len() as u32);
        self.range(0, n - 1, Tag::Enum, own_handle)
    }

    /// The subrange `lo..hi` of the ordinal type `of`.
    pub fn subrange(&mut self, lo: i64, hi: i64, of: Ty) -> Ty {
        let tag = self.tag(of).unwrap_or(Tag::Int);
        self.range(lo, hi, tag, self.base(of))
    }

    /// The first of `candidates` (in order) whose range holds `lo..hi`.
    fn first_holding(&self, lo: i64, hi: i64, candidates: &[Ty]) -> Option<Ty> {
        candidates.iter().copied().find(|&t| {
            let info = self.ord(t);
            info.lo <= lo && hi <= info.hi
        })
    }

    /// A range type stored like the smallest of ShortInt, Byte, Integer, Word, LongInt holding
    /// both bounds (as TP stores enumerations and subranges: `0..100` is signed).
    fn range(&mut self, lo: i64, hi: i64, tag: Tag, base: Ty) -> Ty {
        let candidates = [Ty::SHORTINT, Ty::BYTE, Ty::INTEGER, Ty::WORD];
        let storage = self
            .first_holding(lo, hi, &candidates)
            .unwrap_or(Ty::LONGINT);
        let OrdInfo { size, signed, .. } = self.ord(storage);
        self.add(Type::Ord {
            lo,
            hi,
            size,
            signed,
            tag,
            base,
        })
    }

    /// The procedural type with these parameters and result, shared by equal signatures.
    pub fn proc(&mut self, params: Vec<ParamType>, result: Ty) -> Ty {
        let same = |t: &Type| match t {
            Type::Proc {
                params: p,
                result: r,
            } => *p == params && *r == result,
            _ => false,
        };
        if let Some(existing) = self.table.iter().position(same) {
            return Ty(existing as u32);
        }
        self.add(Type::Proc { params, result })
    }

    /// Parameters and result of a procedural type.
    pub fn sig(&self, t: Ty) -> Option<(Vec<ParamType>, Ty)> {
        match self.get(t) {
            Type::Proc { params, result } => Some((params.clone(), *result)),
            _ => None,
        }
    }

    /// Range, size and signedness of an ordinal type; a procedural value (a table index) and
    /// a pointer are 4 bytes loaded and stored like an unsigned ordinal.
    pub fn ord(&self, t: Ty) -> OrdInfo {
        match self.get(t) {
            Type::Proc { .. } | Type::Ptr(_) => OrdInfo {
                lo: 0,
                hi: u32::MAX as i64,
                size: 4,
                signed: false,
            },
            Type::Ord {
                lo,
                hi,
                size,
                signed,
                ..
            } => OrdInfo {
                lo: *lo,
                hi: *hi,
                size: *size,
                signed: *signed,
            },
            _ => unreachable!("not an ordinal type"),
        }
    }

    /// The type of an integer constant: the smallest predefined type holding it.
    pub fn const_ty(&self, v: i64) -> Ty {
        match v {
            0..=255 => Ty::BYTE,
            -128..=-1 => Ty::SHORTINT,
            -32768..=32767 => Ty::INTEGER,
            32768..=65535 => Ty::WORD,
            _ => Ty::LONGINT,
        }
    }

    /// Smallest of Integer, Word, LongInt holding both ranges; byte-sized operands widen.
    fn common(&self, a: Ty, b: Ty) -> Ty {
        let (info_a, info_b) = (self.ord(a), self.ord(b));
        let lo = info_a.lo.min(info_b.lo);
        let hi = info_a.hi.max(info_b.hi);
        self.first_holding(lo, hi, &[Ty::INTEGER, Ty::WORD])
            .unwrap_or(Ty::LONGINT)
    }

    fn is_text(&self, t: Ty) -> bool {
        self.is_str(t) || self.tag(t) == Some(Tag::Char)
    }

    fn is_numeric(&self, t: Ty) -> bool {
        t == Ty::REAL || self.tag(t) == Some(Tag::Int)
    }

    /// Result type of a binary operator, or the TP error number: 26 when the operand
    /// families differ, 41 when the operator is not defined for them. The operand family
    /// decides which rules apply, tried in this order.
    pub fn binary(&self, op: Op, a: Ty, b: Ty) -> Result<Ty, u16> {
        if op == Op::In {
            return self.in_operator(a, b);
        }
        if self.is_set(a) || self.is_set(b) {
            return self.set_binary(op, a, b);
        }
        if self.is_agg(a) || self.is_agg(b) {
            return Err(code::OPERAND_TYPES_MISMATCH);
        }
        if self.is_ptr(a) || self.is_ptr(b) {
            return self.pointer_binary(op, a, b);
        }
        let concatenation = op == Op::Add && self.is_text(a) && self.is_text(b);
        if self.is_str(a) || self.is_str(b) || concatenation {
            return self.string_binary(op, a, b);
        }
        if a == Ty::REAL || b == Ty::REAL {
            return self.real_binary(op, a, b);
        }
        self.ordinal_binary(op, a, b)
    }

    fn in_operator(&self, element: Ty, set: Ty) -> Result<Ty, u16> {
        match self.set_base(set) {
            None => Err(code::OPERAND_TYPES_MISMATCH),
            Some(base) if self.compat(base, element) => Ok(Ty::BOOLEAN),
            Some(_) => Err(code::TYPE_MISMATCH),
        }
    }

    fn set_binary(&self, op: Op, a: Ty, b: Ty) -> Result<Ty, u16> {
        let a_is_empty_set = self.set_base(a) == Some(Ty::NONE);
        match (self.compat(a, b), op) {
            (false, _) => Err(code::TYPE_MISMATCH),
            (_, Op::Add | Op::Sub | Op::Mul) if a_is_empty_set => Ok(b),
            (_, Op::Add | Op::Sub | Op::Mul) => Ok(a),
            (_, Op::Eq | Op::Ne | Op::Le | Op::Ge) => Ok(Ty::BOOLEAN),
            _ => Err(code::OPERAND_TYPES_MISMATCH),
        }
    }

    fn pointer_binary(&self, op: Op, a: Ty, b: Ty) -> Result<Ty, u16> {
        match (self.compat(a, b), op) {
            (false, _) => Err(code::TYPE_MISMATCH),
            (_, Op::Eq | Op::Ne) => Ok(Ty::BOOLEAN),
            _ => Err(code::OPERAND_TYPES_MISMATCH),
        }
    }

    fn string_binary(&self, op: Op, a: Ty, b: Ty) -> Result<Ty, u16> {
        match (self.is_text(a) && self.is_text(b), op) {
            (false, _) => Err(code::TYPE_MISMATCH),
            (_, Op::Add) => Ok(Ty::STRING),
            _ if is_comparison(op) => Ok(Ty::BOOLEAN),
            _ => Err(code::OPERAND_TYPES_MISMATCH),
        }
    }

    fn real_binary(&self, op: Op, a: Ty, b: Ty) -> Result<Ty, u16> {
        if !self.is_numeric(a) || !self.is_numeric(b) {
            return Err(code::TYPE_MISMATCH);
        }
        match op {
            Op::Add | Op::Sub | Op::Mul | Op::Slash => Ok(Ty::REAL),
            _ if is_comparison(op) => Ok(Ty::BOOLEAN),
            _ => Err(code::OPERAND_TYPES_MISMATCH),
        }
    }

    fn ordinal_binary(&self, op: Op, a: Ty, b: Ty) -> Result<Ty, u16> {
        let (Some(tag), Some(_)) = (self.tag(a), self.tag(b)) else {
            return Err(code::TYPE_MISMATCH);
        };
        if !self.compat(a, b) {
            return Err(code::TYPE_MISMATCH);
        }
        match (tag, op) {
            _ if is_comparison(op) => Ok(Ty::BOOLEAN),
            (Tag::Int, Op::Shl | Op::Shr) => Ok(self.promote(a)),
            (Tag::Int, Op::Slash) => Ok(Ty::REAL),
            (Tag::Int, _) => Ok(self.common(a, b)),
            (Tag::Bool, Op::And | Op::Or | Op::Xor) => Ok(Ty::BOOLEAN),
            _ => Err(code::OPERAND_TYPES_MISMATCH),
        }
    }

    /// Result type of a unary operator or intrinsic, or the TP error number.
    pub fn unary(&self, op: Op, a: Ty) -> Result<Ty, u16> {
        let tag = self.tag(a);
        let real = a == Ty::REAL;
        if matches!(op, Op::Round | Op::Trunc | Op::Sqrt | Op::Int | Op::Frac) {
            if !real && tag != Some(Tag::Int) {
                return Err(code::INTEGER_OR_REAL_EXPRESSION_EXPECTED);
            }
            let rounds_to_integer = matches!(op, Op::Round | Op::Trunc);
            return Ok(if rounds_to_integer {
                Ty::LONGINT
            } else {
                Ty::REAL
            });
        }
        if real && matches!(op, Op::Neg | Op::Abs | Op::Sqr) {
            return Ok(Ty::REAL);
        }
        // Neg, Abs and Sqr are Integer unless the operand needs more than 16 bits.
        let wide = tag == Some(Tag::Int) && {
            let info = self.ord(a);
            info.lo < -32768 || info.hi > 32767
        };
        match (tag, op) {
            (Some(Tag::Int), Op::Neg | Op::Abs | Op::Sqr) if wide => Ok(Ty::LONGINT),
            (Some(Tag::Int), Op::Neg | Op::Abs | Op::Sqr) => Ok(Ty::INTEGER),
            (Some(Tag::Bool), Op::Not) => Ok(Ty::BOOLEAN),
            (Some(Tag::Int), Op::Not) => Ok(a),
            (_, Op::Neg | Op::Not) => Err(code::OPERAND_TYPES_MISMATCH),
            (_, Op::Abs | Op::Sqr) => Err(code::INTEGER_OR_REAL_EXPRESSION_EXPECTED),
            (Some(Tag::Int), Op::Odd) => Ok(Ty::BOOLEAN),
            (Some(Tag::Int), Op::Chr) => Ok(Ty::CHAR),
            (Some(Tag::Int), Op::Ord) => Ok(a),
            (Some(_), Op::Ord) => Ok(Ty::INTEGER),
            (Some(_), Op::Succ | Op::Pred) => Ok(a),
            (None, Op::Succ | Op::Pred | Op::Ord) => Err(code::ORDINAL_EXPRESSION_EXPECTED),
            _ => Err(code::INTEGER_EXPRESSION_EXPECTED),
        }
    }

    /// The type of the step added by `Inc` or `Dec` to an ordinal variable of type `target`.
    pub fn step_type(&self, target: Ty) -> Result<Ty, u16> {
        match self.tag(target) {
            Some(_) => Ok(Ty::LONGINT),
            None => Err(code::ORDINAL_VARIABLE_EXPECTED),
        }
    }

    /// The type `Str(value, target)` assigns to the String variable `target`; `value` must be
    /// an integer or a Real.
    pub fn str_type(&self, target: Ty, value: Ty) -> Result<Ty, u16> {
        if !self.is_str(target) {
            return Err(code::STRING_VARIABLE_EXPECTED);
        }
        self.unary(Op::Abs, value).map(|_| Ty::STRING)
    }

    /// The type `Val(s, target, code)` assigns to the integer or Real variable `target`.
    pub fn val_type(&self, target: Ty) -> Result<Ty, u16> {
        match self.tag(target) {
            Some(Tag::Int) => Ok(Ty::LONGINT),
            _ if target == Ty::REAL => Ok(Ty::REAL),
            _ => Err(code::NUMERIC_VARIABLE_EXPECTED),
        }
    }

    /// The type of `a shl n` and `a shr n`: the operand widened to at least Integer.
    fn promote(&self, a: Ty) -> Ty {
        let info = self.ord(a);
        match (info.size, info.signed) {
            (1, _) | (2, true) => Ty::INTEGER,
            (2, false) => Ty::WORD,
            _ => Ty::LONGINT,
        }
    }
}

fn is_comparison(op: Op) -> bool {
    matches!(op, Op::Eq | Op::Ne | Op::Lt | Op::Gt | Op::Le | Op::Ge)
}

impl Default for Types {
    fn default() -> Self {
        Self::new()
    }
}
