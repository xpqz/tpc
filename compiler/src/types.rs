//! Type table (docs/design/ast.md). Predefined types sit at fixed indices.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Ty(pub u32);

impl Ty {
    pub const NONE: Ty = Ty(0);
    pub const STRING: Ty = Ty(1);
}

pub enum Type {
    None,
    /// `String[n]`.
    Str(u8),
}

pub struct Types {
    pub v: Vec<Type>,
}

impl Types {
    pub fn new() -> Self {
        Types {
            v: vec![Type::None, Type::Str(255)],
        }
    }
}

impl Default for Types {
    fn default() -> Self {
        Self::new()
    }
}
