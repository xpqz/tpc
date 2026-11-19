//! Node arena shared by parser, checker and codegen (docs/design/ast.md).
//!
//! Nodes are plain `Copy` values in one `Vec`, referred to by `Id` (their index); a child list
//! is a `List`, a slice of the shared `lists` vector. The parser builds the tree with `Name`,
//! `Call`, `Index` and similar unresolved nodes; the checker then rewrites them in place into
//! resolved ones (`Name` becomes `Var`, `IntLit`, `Invoke`, ...) and fills in each node's type,
//! so codegen only ever sees checked nodes.
use crate::types::{Sym, Ty};

pub type Id = u32;

/// A child list: `len` handles starting at `start` in `Ast::lists`.
#[derive(Clone, Copy)]
pub struct List {
    pub start: u32,
    pub len: u32,
}

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
    /// `element in set`.
    In,
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
    /// `/`: always Real.
    Slash,
    /// Intrinsics on a Real (an Integer argument is converted): Round and Trunc give LongInt.
    Round,
    Trunc,
    Sqrt,
    Int,
    Frac,
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
    /// `@x`: the address of a variable.
    AddrOf(Id),
    /// `p^`.
    Deref(Id),
    /// `^name` in a type position.
    PtrType(u32),
    /// `RunError(code)`.
    RunErr(Id),
    /// A parenthesised name: not a variable reference for a `var` argument.
    Paren(Id),
    Exit,
    Assign(Id, Id),
    Compound(List),
    If {
        condition: Id,
        then_branch: Id,
        else_branch: Option<Id>,
    },
    /// Condition, body.
    While(Id, Id),
    /// Body statements, condition.
    Repeat(List, Id),
    /// `arms` are `Arm` nodes; `else_body` holds the `else` statements.
    Case {
        selector: Id,
        arms: List,
        else_body: List,
    },
    /// Labels (ranges), body.
    Arm(List, Id),
    /// Label range; a single label has both ends equal.
    Range(Id, Id),
    For {
        variable: Id,
        first: Id,
        last: Id,
        body: Id,
        downto: bool,
    },
    /// Index into `Ast::strs`.
    StrLit(u32),
    /// Integer, Boolean and Char constants.
    IntLit(i64),
    RealLit(f64),
    /// An Integer-family operand used as a Real.
    IntToReal(Id),
    Var(Sym),
    Unary(Op, Id),
    Binary {
        op: Op,
        left: Id,
        right: Id,
    },
    /// `value : width [: decimals]` in a write argument.
    Fmt {
        value: Id,
        width: Id,
        decimals: Option<Id>,
    },
    /// `Write` / `WriteLn` (true) with its arguments.
    Write(List, bool),
    /// `name` is `NO_NAME` when the program has no header. `uses` is a list of `Name` nodes.
    Program {
        name: u32,
        uses: List,
        declarations: List,
        body: Id,
    },
    /// A unit, stamped at the `begin` or `end` of its initialisation part. System is a unit
    /// whose declarations are all in `interface`. `body` is a `Compound`, empty without a
    /// `begin` part.
    Unit {
        name: u32,
        interface_uses: List,
        interface: List,
        implementation_uses: List,
        implementation: List,
        body: Id,
    },
    /// A unit's initialisation part (made by the checker): `Exit` inside leaves only this part.
    InitPart(Id),
    ConstDecl(u32, Id),
    /// `name: type = init`: constant name, type name, initialiser (an expression, `ArrayInit`
    /// or `RecordInit`).
    TypedConst(u32, u32, Id),
    /// `(a, b, c)`, stamped at the closing parenthesis, where TP reports a list that is too short.
    ArrayInit(List),
    /// `(f: a; g: b)`: `FieldInit` entries, stamped at the closing parenthesis.
    RecordInit(List),
    /// `name: value` in a record initialiser.
    FieldInit(u32, Id),
    /// A procedure or function, stamped after its header. `params` is None when the header
    /// has no parameter list, `result` is `NO_NAME` when it has no result type, `block` is
    /// None for a `forward` or `external` declaration. `external`, allowed only in the runtime
    /// source, is the name of the host import: the string after the directive, else the
    /// routine's own name.
    Routine {
        name: u32,
        params: Option<List>,
        result: u32,
        is_function: bool,
        block: Option<Id>,
        external: Option<u32>,
    },
    /// `by_ref` is set for a `var` parameter.
    Param {
        name: u32,
        type_name: u32,
        by_ref: bool,
    },
    /// Declarations and body, stamped where the declarations start.
    Block(List, Id),
    /// Variable name, type name.
    VarDecl(u32, u32),
    /// Type name and its definition: `Name`, `Range` (a subrange), `Enum` or `ProcType`.
    TypeDecl(u32, Id),
    /// `params` is None when omitted, `result` is `NO_NAME` for a procedure.
    ProcType {
        params: Option<List>,
        result: u32,
        is_function: bool,
    },
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
    /// `base[index]`; `continues` marks an index that continues the same brackets (`a[i, j]`).
    Index {
        base: Id,
        index: Id,
        continues: bool,
    },
    /// `base.name`; the checker replaces the name with the field's offset.
    Field(Id, u32),
    /// Record designators whose fields are visible in the body.
    With(List, Id),
    /// `String[size]` with a constant size expression.
    StrType(Id),
    /// A Char operand used as a String.
    CharToStr(Id),
    /// `[a, b..c]`: elements and `Range` nodes.
    SetLit(List),
    /// `set of` the type definition.
    SetType(Id),
}

/// The name handle of an absent name: no program header, no result type.
pub const NO_NAME: u32 = u32::MAX;

#[derive(Clone, Copy)]
pub struct Node {
    pub kind: Kind,
    pub line: u32,
    pub col: u32,
    /// The source file the node was read from: 0 is the main program, n the n-th registered file.
    pub file: u32,
    /// True when `{$R+}` was in force where the node's last token stands.
    pub range_checks: bool,
    pub ty: Ty,
}

#[derive(Default)]
pub struct Ast {
    pub nodes: Vec<Node>,
    pub lists: Vec<Id>,
    /// Interned identifiers, uppercase.
    pub names: Vec<Vec<u8>>,
    pub strs: Vec<Vec<u8>>,
    /// The file that `add` stamps on new nodes.
    pub file: u32,
    /// The `{$R+}` state that `add` stamps on new nodes.
    pub range_checks: bool,
    /// The qualified names `Unit.Name` the parser interned, for the symbol table.
    pub qualified: Vec<Qualified>,
    /// For each identifier used as an operand without arguments, the line of the token that
    /// follows it, where TP reports a missing argument list.
    pub follow: Vec<FollowLine>,
}

/// A qualified name `unit.member`: `joined` is the interned name the parser used in its place.
pub struct Qualified {
    pub joined: u32,
    pub unit: u32,
    pub member: u32,
}

/// The loaded units in the orders the checker needs them (built by `units.rs`).
pub struct Plan {
    pub system: Id,
    /// Units in the order their interfaces complete: each after the interfaces it uses.
    pub interface_order: Vec<Id>,
    /// Units in initialisation order: each after every unit it uses.
    pub init_order: Vec<Id>,
    pub program: Id,
}

pub struct FollowLine {
    pub node: Id,
    pub line: u32,
}

impl Ast {
    pub fn add(&mut self, kind: Kind, line: u32, col: u32) -> Id {
        self.nodes.push(Node {
            kind,
            line,
            col,
            file: self.file,
            range_checks: self.range_checks,
            ty: Ty::NONE,
        });
        self.nodes.len() as Id - 1
    }

    /// A node of type `ty` at the position of `at`.
    pub fn typed(&mut self, kind: Kind, ty: Ty, at: Id) -> Id {
        let (line, col) = self.position(at);
        let id = self.add(kind, line, col);
        self.nodes[id as usize].file = self.node(at).file;
        self.nodes[id as usize].range_checks = self.node(at).range_checks;
        self.set_ty(id, ty);
        id
    }

    pub fn position(&self, id: Id) -> (u32, u32) {
        let node = self.node(id);
        (node.line, node.col)
    }

    pub fn list(&mut self, items: &[Id]) -> List {
        let start = self.lists.len() as u32;
        self.lists.extend_from_slice(items);
        List {
            start,
            len: items.len() as u32,
        }
    }

    pub fn items(&self, list: List) -> &[Id] {
        &self.lists[list.start as usize..(list.start + list.len) as usize]
    }

    /// The items as an owned vector, for loops that also modify the tree.
    pub fn items_vec(&self, list: List) -> Vec<Id> {
        self.items(list).to_vec()
    }

    pub fn node(&self, id: Id) -> Node {
        self.nodes[id as usize]
    }

    pub fn kind(&self, id: Id) -> Kind {
        self.nodes[id as usize].kind
    }

    pub fn ty(&self, id: Id) -> Ty {
        self.nodes[id as usize].ty
    }

    pub fn set_kind(&mut self, id: Id, kind: Kind) {
        self.nodes[id as usize].kind = kind;
    }

    pub fn set_ty(&mut self, id: Id, ty: Ty) {
        self.nodes[id as usize].ty = ty;
    }

    /// Replaces a node's kind and type together: the checker's rewrite step.
    pub fn resolve(&mut self, id: Id, kind: Kind, ty: Ty) {
        self.set_kind(id, kind);
        self.set_ty(id, ty);
    }

    /// The handle of an identifier, adding it on first use. Names are compared upper-cased.
    pub fn intern(&mut self, name: &[u8]) -> u32 {
        let upper = name.to_ascii_uppercase();
        if let Some(index) = self.names.iter().position(|n| *n == upper) {
            return index as u32;
        }
        self.names.push(upper);
        self.names.len() as u32 - 1
    }

    /// A fresh type name for an anonymous type (`var x: array[1..3] of Byte`). It is empty,
    /// so no identifier can ever be interned to the same handle.
    pub fn anonymous_name(&mut self) -> u32 {
        self.names.push(Vec::new());
        self.names.len() as u32 - 1
    }
}
