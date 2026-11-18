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
    /// Unary only; the intrinsics Ord, Chr, Succ, Pred, Odd, Abs and Sqr are unary operators.
    Neg,
    Not,
    Ord,
    Chr,
    Succ,
    Pred,
    Odd,
    Abs,
    Sqr,
}

/// Nodes made by operators, assignments and `:` carry the position of the token after
/// their last operand, where TP reports their errors; other nodes carry their own start.
#[derive(Clone, Copy)]
pub enum Kind {
    /// Parser output for an identifier; the checker overwrites it.
    Name(u32),
    /// A `Name` callee and its arguments, as a statement or an expression; the checker
    /// resolves it (the node is stamped after the arguments, the callee at the name).
    Call(Id, List),
    /// A checked call of the routine with that index; the type is the result type.
    Invoke(u32, List),
    /// A checked reference to a global routine as a procedural value: its table index.
    ProcRef(u32),
    /// A call through the procedural value of the node, with arguments; the type is the
    /// result type.
    Indirect(Id, List),
    /// `@x`; always an error for now.
    AddrOf(Id),
    /// A parenthesised name: not a variable reference for a `var` argument.
    Paren(Id),
    Exit,
    Assign(Id, Id),
    Compound(List),
    /// Condition, then-branch, else-branch.
    If(Id, Id, Option<Id>),
    /// Condition, body.
    While(Id, Id),
    /// Body statements, condition.
    Repeat(List, Id),
    /// Selector, arms, `else` statements.
    Case(Id, List, List),
    /// Labels (ranges), body.
    Arm(List, Id),
    /// Label range; a single label has both ends equal.
    Range(Id, Id),
    /// Control variable, first, last, body, `downto`.
    For(Id, Id, Id, Id, bool),
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
    /// Name, parameter list (None when omitted), result type name (`NO_NAME` when
    /// omitted), function, block (None for `forward`). Stamped after the header.
    Routine(u32, Option<List>, u32, bool, Option<Id>),
    /// Parameter name, type name, passed by reference (`var`).
    Param(u32, u32, bool),
    /// Declarations and body, stamped where the declarations start.
    Block(List, Id),
    /// Variable name, type name.
    VarDecl(u32, u32),
    /// Type name and its definition: `Name`, `Range` (a subrange), `Enum` or `ProcType`.
    TypeDecl(u32, Id),
    /// Parameters (None when omitted), result type name (`NO_NAME` for a procedure), function.
    ProcType(Option<List>, u32, bool),
    /// An enumerated type: its identifiers as `Name` nodes.
    Enum(List),
    /// Index types (one per dimension) and element type.
    Array(List, Id),
    /// Field declarations and at most one `Variant`, in order.
    Record(List),
    /// Field name (`NO_NAME` for an untagged variant selector type) and type.
    FieldDecl(u32, Id),
    /// The arms of a variant part, each a `Record`.
    Variant(List),
    /// `base[index]`; the flag marks an index that continues the same brackets (`a[i, j]`).
    Index(Id, Id, bool),
    /// `base.name`; the checker replaces the name with the field's offset.
    Field(Id, u32),
    /// Record designators whose fields are visible in the body.
    With(List, Id),
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
    /// Number of leading declarations of the program that come from System.
    pub system: usize,
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
