//! Node arena shared by parser, checker and codegen (docs/design/ast.md).
use crate::types::Ty;

pub type Id = u32;

/// A child list: (start, len) into `Ast::lists`.
#[derive(Clone, Copy)]
pub struct List(pub u32, pub u32);

#[derive(Clone, Copy)]
pub enum Kind {
    /// Parser output for an identifier; the checker overwrites it.
    Name(u32),
    /// `name` or `name(args)` used as a statement; the checker resolves it.
    Call(u32, List),
    Compound(List),
    /// Index into `Ast::strs`.
    StrLit(u32),
    /// `Write` / `WriteLn` (true) with its arguments.
    Write(List, bool),
}

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
