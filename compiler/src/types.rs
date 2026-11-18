//! Types, symbols and the operator typing rules (decisions 5 and 10).
use crate::ast::{Id, Op};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Ty(pub u32);

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
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tag {
    Int,
    Bool,
    Char,
    /// Enumerated types (and their subranges) are compatible only with the same enumeration.
    Enum,
}

pub enum Type {
    None,
    /// `String[n]`.
    Str(u8),
    /// A procedural type: parameters (type, by reference) and the result (`Ty::NONE` for a
    /// procedure). Interned by content, so equal signatures are the same type.
    Proc {
        params: Vec<(Ty, bool)>,
        result: Ty,
    },
    /// A pointer to the type (`Ty::NONE` for the untyped `Pointer`): 4 bytes, unsigned.
    Ptr(Ty),
    /// One dimension; `a[i, j]` is an array of arrays. `size` saturates at u32::MAX.
    Array {
        index: Ty,
        elem: Ty,
        size: u32,
    },
    /// Fields (name, type, offset) in declaration order, variants overlaid, and the size.
    Record {
        fields: Vec<(u32, Ty, u32)>,
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SymKind {
    Prog,
    Type,
    /// A predefined type this compiler does not support.
    Omitted,
    /// `val` is the value, or the string index when the type is `Ty::STRING`.
    Const,
    /// `val` is the static-data address.
    Var,
    /// `val` is the index of the local; parameters and the result come first.
    Local,
    /// A local that lives in the frame; `val` is its offset from `fp`.
    Frame,
    /// A `var` parameter; `val` is the index of the local holding the address.
    Ref,
    /// A `var` parameter captured by a nested routine; `val` is the frame offset of its
    /// address.
    FrameRef,
    /// `val` is the index into the routine table.
    Routine,
    /// Write 0, WriteLn 1, Ord 2, Chr 3, Succ 4, Pred 5, Odd 6, Abs 7, Sqr 8, Exit 9, SizeOf 10.
    Intrinsic,
}

#[derive(Clone, Copy)]
pub struct Symbol {
    pub kind: SymKind,
    pub ty: Ty,
    pub val: i64,
    /// Passed as a `var` argument; such a local moves to the frame once its routine is checked.
    pub taken: bool,
    /// Lexical depth of the owning routine (0 for the program).
    pub depth: u32,
}

impl Symbol {
    pub fn new(kind: SymKind, ty: Ty, val: i64) -> Self {
        Symbol {
            kind,
            ty,
            val,
            taken: false,
            depth: 0,
        }
    }
}

/// A declared procedure or function.
pub struct Routine {
    pub params: Vec<Ty>,
    /// Per parameter: passed by reference.
    pub byref: Vec<bool>,
    pub names: Vec<u32>,
    /// `Ty::NONE` for a procedure.
    pub result: Ty,
    pub func: bool,
    pub defined: bool,
    /// Body statement, once defined.
    pub body: Id,
    /// Locals after the parameters, the function result included.
    pub nlocals: u32,
    /// Lexical depth: top-level routines are 1; deeper ones take the static link as
    /// parameter 0.
    pub depth: u32,
    /// Contains nested routines, which receive its frame pointer.
    pub children: bool,
    /// The routine has a frame of `frame` bytes (the static link slot included).
    pub framed: bool,
    pub frame: u32,
    /// The symbol of the function result.
    pub result_sym: Sym,
    /// Value parameters moved to the frame: (parameter local, offset, type).
    pub copies: Vec<(u32, u32, Ty)>,
    /// Declared in System: its arguments are diagnosed like those of a TP built-in.
    pub builtin: bool,
    /// The result is a String: the caller passes its address as a hidden parameter.
    pub result_ptr: bool,
    /// Its address is taken as a procedural value: it has a table slot.
    pub addressed: bool,
}

pub struct Types {
    pub v: Vec<Type>,
}

impl Types {
    pub fn new() -> Self {
        let ord = |lo, hi, size, signed, tag, base| Type::Ord {
            lo,
            hi,
            size,
            signed,
            tag,
            base: Ty(base),
        };
        Types {
            v: vec![
                Type::None,
                Type::Str(255),
                ord(0, 255, 1, false, Tag::Int, 2),
                ord(-128, 127, 1, true, Tag::Int, 3),
                ord(-32768, 32767, 2, true, Tag::Int, 4),
                ord(0, 65535, 2, false, Tag::Int, 5),
                ord(-(1 << 31), (1 << 31) - 1, 4, true, Tag::Int, 6),
                ord(0, 1, 1, false, Tag::Bool, 7),
                ord(0, 255, 1, false, Tag::Char, 8),
                Type::Ptr(Ty::NONE),
            ],
        }
    }

    pub fn tag(&self, t: Ty) -> Option<Tag> {
        match self.v[t.0 as usize] {
            Type::Ord { tag, .. } => Some(tag),
            _ => None,
        }
    }

    /// Storage size in bytes.
    pub fn size(&self, t: Ty) -> u32 {
        match &self.v[t.0 as usize] {
            Type::Array { size, .. } | Type::Record { size, .. } => *size,
            Type::Str(n) => *n as u32 + 1,
            Type::Proc { .. } | Type::Ptr(_) => 4,
            Type::Ord { size, .. } => *size as u32,
            Type::None => 0,
        }
    }

    /// Arrays and records, copied whole by `memory.copy`.
    pub fn is_agg(&self, t: Ty) -> bool {
        matches!(
            self.v[t.0 as usize],
            Type::Array { .. } | Type::Record { .. }
        )
    }

    pub fn is_ptr(&self, t: Ty) -> bool {
        matches!(self.v[t.0 as usize], Type::Ptr(_))
    }

    /// A new pointer type to `to`.
    pub fn pointer(&mut self, to: Ty) -> Ty {
        self.v.push(Type::Ptr(to));
        Ty(self.v.len() as u32 - 1)
    }

    /// What a pointer type points to (`Ty::NONE` for `Pointer`).
    pub fn target(&self, t: Ty) -> Option<Ty> {
        match self.v[t.0 as usize] {
            Type::Ptr(to) => Some(to),
            _ => None,
        }
    }

    pub fn is_str(&self, t: Ty) -> bool {
        matches!(self.v[t.0 as usize], Type::Str(_))
    }

    /// Types that live in memory and are passed by address.
    pub fn in_memory(&self, t: Ty) -> bool {
        self.is_agg(t) || self.is_str(t)
    }

    /// A new `String[n]`; each form is a type of its own.
    pub fn string(&mut self, n: u8) -> Ty {
        self.v.push(Type::Str(n));
        Ty(self.v.len() as u32 - 1)
    }

    /// Capacity of a string type.
    pub fn capacity(&self, t: Ty) -> u8 {
        match self.v[t.0 as usize] {
            Type::Str(n) => n,
            _ => 0,
        }
    }

    /// `array[index] of elem`.
    pub fn array(&mut self, index: Ty, elem: Ty) -> Ty {
        let (lo, hi, ..) = self.ord(index);
        let size = ((hi - lo + 1) * self.size(elem) as i64).min(u32::MAX as i64) as u32;
        self.v.push(Type::Array { index, elem, size });
        Ty(self.v.len() as u32 - 1)
    }

    /// (index type, lower bound, upper bound, element type) of an array type; a string is
    /// an array of Char from 0 (the length byte).
    pub fn array_of(&self, t: Ty) -> Option<(Ty, i64, i64, Ty)> {
        match self.v[t.0 as usize] {
            Type::Array { index, elem, .. } => {
                let (lo, hi, ..) = self.ord(index);
                Some((index, lo, hi, elem))
            }
            Type::Str(n) => Some((Ty::INTEGER, 0, n as i64, Ty::CHAR)),
            _ => None,
        }
    }

    pub fn is_record(&self, t: Ty) -> bool {
        matches!(self.v[t.0 as usize], Type::Record { .. })
    }

    pub fn record(&mut self, fields: Vec<(u32, Ty, u32)>, size: u32) -> Ty {
        self.v.push(Type::Record { fields, size });
        Ty(self.v.len() as u32 - 1)
    }

    /// (type, offset) of a record field.
    pub fn field(&self, t: Ty, name: u32) -> Option<(Ty, u32)> {
        match &self.v[t.0 as usize] {
            Type::Record { fields, .. } => fields.iter().find(|f| f.0 == name).map(|f| (f.1, f.2)),
            _ => None,
        }
    }

    /// The type that decides arithmetic and compatibility of `t`.
    pub fn base(&self, t: Ty) -> Ty {
        match self.v[t.0 as usize] {
            Type::Ord { base, .. } => base,
            _ => t,
        }
    }

    /// Ordinal types of one family (enumerations must also be the same enumeration), or
    /// pointers of one type or with the untyped `Pointer`.
    pub fn compat(&self, a: Ty, b: Ty) -> bool {
        match (self.tag(a), self.tag(b)) {
            (Some(x), Some(y)) => x == y && (x != Tag::Enum || self.base(a) == self.base(b)),
            _ => {
                self.is_ptr(a) && self.is_ptr(b) && (a == b || a == Ty::POINTER || b == Ty::POINTER)
            }
        }
    }

    /// A new enumeration of `n` values.
    pub fn enumeration(&mut self, n: i64) -> Ty {
        let at = Ty(self.v.len() as u32);
        self.range(0, n - 1, Tag::Enum, at)
    }

    /// The subrange `lo..hi` of the ordinal type `of`.
    pub fn subrange(&mut self, lo: i64, hi: i64, of: Ty) -> Ty {
        let tag = self.tag(of).unwrap_or(Tag::Int);
        self.range(lo, hi, tag, self.base(of))
    }

    /// A range type stored like the smallest of ShortInt, Byte, Integer, Word, LongInt holding
    /// both bounds (as TP stores enumerations and subranges: `0..100` is signed).
    fn range(&mut self, lo: i64, hi: i64, tag: Tag, base: Ty) -> Ty {
        let fit = [Ty::SHORTINT, Ty::BYTE, Ty::INTEGER, Ty::WORD]
            .into_iter()
            .find(|&t| self.ord(t).0 <= lo && hi <= self.ord(t).1)
            .unwrap_or(Ty::LONGINT);
        let (_, _, size, signed) = self.ord(fit);
        self.v.push(Type::Ord {
            lo,
            hi,
            size,
            signed,
            tag,
            base,
        });
        Ty(self.v.len() as u32 - 1)
    }

    pub fn proc(&mut self, params: Vec<(Ty, bool)>, result: Ty) -> Ty {
        let same = |t: &Type| match t {
            Type::Proc {
                params: p,
                result: r,
            } => *p == params && *r == result,
            _ => false,
        };
        let at = self.v.iter().position(same).unwrap_or(self.v.len());
        if at == self.v.len() {
            self.v.push(Type::Proc { params, result });
        }
        Ty(at as u32)
    }

    /// Parameter types, by-reference flags and result of a procedural type.
    pub fn sig(&self, t: Ty) -> Option<(Vec<Ty>, Vec<bool>, Ty)> {
        match &self.v[t.0 as usize] {
            Type::Proc { params, result } => Some((
                params.iter().map(|p| p.0).collect(),
                params.iter().map(|p| p.1).collect(),
                *result,
            )),
            _ => None,
        }
    }

    /// (lo, hi, size, signed) of an ordinal type; a procedural value (a table index) and a
    /// pointer are 4 bytes loaded and stored like an unsigned ordinal.
    pub fn ord(&self, t: Ty) -> (i64, i64, u8, bool) {
        match self.v[t.0 as usize] {
            Type::Proc { .. } | Type::Ptr(_) => (0, u32::MAX as i64, 4, false),
            Type::Ord {
                lo,
                hi,
                size,
                signed,
                ..
            } => (lo, hi, size, signed),
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
        let (la, ha, ..) = self.ord(a);
        let (lb, hb, ..) = self.ord(b);
        let (lo, hi) = (la.min(lb), ha.max(hb));
        [Ty::INTEGER, Ty::WORD]
            .into_iter()
            .find(|&t| self.ord(t).0 <= lo && hi <= self.ord(t).1)
            .unwrap_or(Ty::LONGINT)
    }

    /// Result type of a binary operator, or the TP error number: 26 when the operand
    /// families differ, 41 when the operator is not defined for them.
    pub fn binary(&self, op: Op, a: Ty, b: Ty) -> Result<Ty, u16> {
        if self.is_agg(a) || self.is_agg(b) {
            return Err(41);
        }
        if self.is_ptr(a) || self.is_ptr(b) {
            return match (self.compat(a, b), op) {
                (false, _) => Err(26),
                (_, Op::Eq | Op::Ne) => Ok(Ty::BOOLEAN),
                _ => Err(41),
            };
        }
        let text = |t: Ty| self.is_str(t) || self.tag(t) == Some(Tag::Char);
        let compare = matches!(op, Op::Eq | Op::Ne | Op::Lt | Op::Gt | Op::Le | Op::Ge);
        if self.is_str(a) || self.is_str(b) || op == Op::Add && text(a) && text(b) {
            return match (text(a) && text(b), op) {
                (false, _) => Err(26),
                (_, Op::Add) => Ok(Ty::STRING),
                (_, _) if compare => Ok(Ty::BOOLEAN),
                _ => Err(41),
            };
        }
        let (Some(ta), Some(_)) = (self.tag(a), self.tag(b)) else {
            return Err(26);
        };
        if !self.compat(a, b) {
            return Err(26);
        }
        match (ta, op) {
            (_, _) if compare => Ok(Ty::BOOLEAN),
            (Tag::Int, Op::Shl | Op::Shr) => Ok(self.promote(a)),
            (Tag::Int, _) => Ok(self.common(a, b)),
            (Tag::Bool, Op::And | Op::Or | Op::Xor) => Ok(Ty::BOOLEAN),
            _ => Err(41),
        }
    }

    /// Result type of a unary operator or intrinsic, or the TP error number.
    pub fn unary(&self, op: Op, a: Ty) -> Result<Ty, u16> {
        let tag = self.tag(a);
        let wide = tag == Some(Tag::Int) && {
            let (lo, hi, ..) = self.ord(a);
            lo < -32768 || hi > 32767
        };
        match (tag, op) {
            (Some(Tag::Int), Op::Neg | Op::Abs | Op::Sqr) if wide => Ok(Ty::LONGINT),
            (Some(Tag::Int), Op::Neg | Op::Abs | Op::Sqr) => Ok(Ty::INTEGER),
            (Some(Tag::Bool), Op::Not) => Ok(Ty::BOOLEAN),
            (Some(Tag::Int), Op::Not) => Ok(a),
            (_, Op::Neg | Op::Not) => Err(41),
            (_, Op::Abs | Op::Sqr) => Err(79),
            (Some(Tag::Int), Op::Odd) => Ok(Ty::BOOLEAN),
            (Some(Tag::Int), Op::Chr) => Ok(Ty::CHAR),
            (Some(Tag::Int), Op::Ord) => Ok(a),
            (Some(_), Op::Ord) => Ok(Ty::INTEGER),
            (Some(_), Op::Succ | Op::Pred) => Ok(a),
            (None, Op::Succ | Op::Pred | Op::Ord) => Err(39),
            _ => Err(38),
        }
    }

    fn promote(&self, a: Ty) -> Ty {
        match self.ord(a) {
            (_, _, 1, _) | (_, _, 2, true) => Ty::INTEGER,
            (_, _, 2, false) => Ty::WORD,
            _ => Ty::LONGINT,
        }
    }
}

impl Default for Types {
    fn default() -> Self {
        Self::new()
    }
}
