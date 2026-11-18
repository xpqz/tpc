//! Types, symbols and the operator typing rules (decisions 5 and 10).
use crate::ast::{Id, Op};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tag {
    Int,
    Bool,
    Char,
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
    Ord {
        lo: i64,
        hi: i64,
        size: u8,
        signed: bool,
        tag: Tag,
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
    /// Write 0, WriteLn 1, Ord 2, Chr 3, Succ 4, Pred 5, Odd 6, Abs 7, Sqr 8, Exit 9.
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
    /// Its address is taken as a procedural value: it has a table slot.
    pub addressed: bool,
}

pub struct Types {
    pub v: Vec<Type>,
}

impl Types {
    pub fn new() -> Self {
        let ord = |lo, hi, size, signed, tag| Type::Ord {
            lo,
            hi,
            size,
            signed,
            tag,
        };
        Types {
            v: vec![
                Type::None,
                Type::Str(255),
                ord(0, 255, 1, false, Tag::Int),
                ord(-128, 127, 1, true, Tag::Int),
                ord(-32768, 32767, 2, true, Tag::Int),
                ord(0, 65535, 2, false, Tag::Int),
                ord(-(1 << 31), (1 << 31) - 1, 4, true, Tag::Int),
                ord(0, 1, 1, false, Tag::Bool),
                ord(0, 255, 1, false, Tag::Char),
            ],
        }
    }

    pub fn tag(&self, t: Ty) -> Option<Tag> {
        match self.v[t.0 as usize] {
            Type::Ord { tag, .. } => Some(tag),
            _ => None,
        }
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

    /// (lo, hi, size, signed) of an ordinal type; a procedural value is a 4-byte table
    /// index loaded and stored like an unsigned ordinal.
    pub fn ord(&self, t: Ty) -> (i64, i64, u8, bool) {
        match self.v[t.0 as usize] {
            Type::Proc { .. } => (0, u32::MAX as i64, 4, false),
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
        let text = |t: Ty| t == Ty::CHAR || t == Ty::STRING;
        if op == Op::Add && text(a) && text(b) {
            return Ok(Ty::STRING);
        }
        let (Some(ta), Some(tb)) = (self.tag(a), self.tag(b)) else {
            return Err(26);
        };
        if ta != tb {
            return Err(26);
        }
        let compare = matches!(op, Op::Eq | Op::Ne | Op::Lt | Op::Gt | Op::Le | Op::Ge);
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
        let wide = a == Ty::WORD || a == Ty::LONGINT;
        match (tag, op) {
            (Some(Tag::Int), Op::Neg | Op::Abs | Op::Sqr) if wide => Ok(Ty::LONGINT),
            (Some(Tag::Int), Op::Neg | Op::Abs | Op::Sqr) => Ok(Ty::INTEGER),
            (Some(Tag::Int | Tag::Bool), Op::Not) => Ok(a),
            (_, Op::Neg | Op::Not) => Err(41),
            (_, Op::Abs | Op::Sqr) => Err(79),
            (Some(Tag::Int), Op::Odd) => Ok(Ty::BOOLEAN),
            (Some(Tag::Int), Op::Chr) => Ok(Ty::CHAR),
            (Some(Tag::Int), Op::Ord) => Ok(a),
            (Some(_), Op::Ord) => Ok(Ty::INTEGER),
            (Some(_), Op::Succ | Op::Pred) => Ok(a),
            _ => Err(38),
        }
    }

    fn promote(&self, a: Ty) -> Ty {
        if self.ord(a).2 == 1 {
            Ty::INTEGER
        } else {
            a
        }
    }
}

impl Default for Types {
    fn default() -> Self {
        Self::new()
    }
}
