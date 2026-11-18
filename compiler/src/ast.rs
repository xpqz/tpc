//! Node arena shared by parser, checker and codegen (docs/design/ast.md).
use crate::types::{Sym, Ty};

pub type Id = u32;

/// A child list: (start, len) into `Ast::lists`.
#[derive(Clone, Copy)]
pub struct List(pub u32, pub u32);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    /// Unary only.
    Neg,
    Not,
}

/// Nodes made by operators, assignments and `:` carry the position of the token after
/// their last operand, where TP reports their errors; other nodes carry their own start.
#[derive(Clone, Copy)]
pub enum Kind {
    /// Parser output for an identifier; the checker overwrites it.
    Name(u32),
    /// `name` or `name(args)` used as a statement; the checker resolves it.
    Call(u32, List),
    Assign(Id, Id),
    Compound(List),
    /// Index into `Ast::strs`.
    StrLit(u32),
    /// Integer, Boolean and Char constants.
    IntLit(i64),
    Var(Sym),
    Unary(Op, Id),
    Binary(Op, Id, Id),
    /// `value : width` in a write argument.
    Fmt(Id, Id),
    /// `Write` / `WriteLn` (true) with its arguments.
    Write(List, bool),
    /// Header name (`NO_NAME` when absent), declarations, body.
    Program(u32, List, Id),
    ConstDecl(u32, Id),
    /// Variable name, type name.
    VarDecl(u32, u32),
}

pub const NO_NAME: u32 = u32::MAX;

#[derive(Clone, Copy)]
pub struct Node {
    pub kind: Kind,
    pub line: u32,
    pub col: u32,
    pub ty: Ty,
}

#[derive(Default)]
pub struct Ast {
    pub nodes: Vec<Node>,
    pub lists: Vec<Id>,
    /// Interned identifiers, uppercase.
    pub names: Vec<Vec<u8>>,
    pub strs: Vec<Vec<u8>>,
}

impl Ast {
    pub fn add(&mut self, kind: Kind, line: u32, col: u32) -> Id {
        self.nodes.push(Node {
            kind,
            line,
            col,
            ty: Ty::NONE,
        });
        self.nodes.len() as Id - 1
    }

    pub fn list(&mut self, items: &[Id]) -> List {
        let start = self.lists.len() as u32;
        self.lists.extend_from_slice(items);
        List(start, items.len() as u32)
    }

    pub fn items(&self, l: List) -> &[Id] {
        &self.lists[l.0 as usize..(l.0 + l.1) as usize]
    }

    pub fn intern(&mut self, name: &[u8]) -> u32 {
        let up = name.to_ascii_uppercase();
        let at = self
            .names
            .iter()
            .position(|n| *n == up)
            .unwrap_or(self.names.len());
        if at == self.names.len() {
            self.names.push(up);
        }
        at as u32
    }
}
