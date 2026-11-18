//! Types, symbols and the operator typing rules (decisions 5 and 10).
use crate::ast::Op;

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
    /// `Write` (val 0) and `WriteLn` (val 1).
    Intrinsic,
}

#[derive(Clone, Copy)]
pub struct Symbol {
    pub kind: SymKind,
    pub ty: Ty,
    pub val: i64,
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

    /// (lo, hi, size, signed) of an ordinal type.
    pub fn ord(&self, t: Ty) -> (i64, i64, u8, bool) {
        match self.v[t.0 as usize] {
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

    pub fn unary(&self, op: Op, a: Ty) -> Result<Ty, u16> {
        match (self.tag(a), op) {
            (Some(Tag::Int), Op::Neg) if a == Ty::WORD || a == Ty::LONGINT => Ok(Ty::LONGINT),
            (Some(Tag::Int), Op::Neg) => Ok(Ty::INTEGER),
            (Some(Tag::Int | Tag::Bool), Op::Not) => Ok(a),
            _ => Err(41),
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
