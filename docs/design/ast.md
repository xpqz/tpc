# AST, types and symbols: the front-end contract

Binding for parser, checker and codegen. Changing it is a decision in `docs/decisions.md`.

## Pipeline

`lex → parse (whole program) → check (whole program) → codegen`. The checker finishes
every routine before codegen starts, because `captured`/`addr_taken` flags (plan §4.3) can
be set by code that appears after a variable's first use. There is no IR: codegen walks
the checked AST.

## Arena

```rust
pub type Id = u32;                 // node handle; 0 is a valid node
pub struct Ast {
    pub nodes: Vec<Node>,
    pub lists: Vec<Id>,            // child lists live here as (start, len) ranges
    pub names: Vec<Vec<u8>>,       // interned identifiers, uppercase
}
pub struct Node { pub kind: Kind, pub line: u32, pub col: u32, pub ty: Ty }
#[derive(Clone, Copy)] pub struct List(pub u32, pub u32); // start, len into Ast::lists
```

- `Kind` is one enum covering declarations, statements and expressions. Variants hold only
  `Copy` payloads: `Id`, `List`, `Ty`, `Sym`, small enums, `i64`, `f64`, a name index, or a
  string-literal index. No `Box`, `Rc` or `Vec` inside `Kind`.
- String literal bytes live in `Ast::strs: Vec<Vec<u8>>` (indexed), or a single byte pool.
- `ty` is `Ty::NONE` (0) after parsing; the checker sets it on every expression node.

## What the checker does to the tree

1. **Name resolution in place.** `Kind::Name(n)` becomes `Kind::Var(sym)`, `Kind::Const(..)`,
   `Kind::Call(..)` etc. by overwriting `kind`. Codegen never sees `Name`.
2. **Coercions are inserted by handle return.** Checking an expression is
   `fn expr(&mut self, e: Id, want: Ty) -> Id`; it returns either `e` or a new node that
   wraps it (`Kind::Conv(ConvOp, Id)`), and the caller stores the returned id back into its
   own field. Conversion ops: `IntToReal`, `CharToStr`. Nothing else is a node.
3. **Integer width is carried by `ty`.** All integers are i32 in WASM, always kept
   normalised to their type's range. Widening is therefore a no-op and gets no node.
   Codegen re-narrows after `+ - *` and unary minus from the node's `ty` (plan §4.6), and
   truncates on store by the destination's size. Result types follow decision 5.
4. **Constant folding** of constant expressions happens in the checker (needed for
   `const`, case labels, subrange bounds); a folded expression becomes `Kind::IntLit` /
   `RealLit` / `StrLit` with its type.

## Types (`types.rs`)

```rust
#[derive(Clone, Copy, PartialEq, Eq)] pub struct Ty(pub u32);   // index into Types::v
pub struct Types { pub v: Vec<Type> }
```

Predefined types occupy fixed low indices as constants (`Ty::BYTE`, `Ty::SHORTINT`,
`Ty::INTEGER`, `Ty::WORD`, `Ty::LONGINT`, `Ty::BOOLEAN`, `Ty::CHAR`, `Ty::REAL`,
`Ty::STRING` = `String[255]`), created by `Types::new()`. `Type` is an enum: ordinal
(`lo`, `hi`, `size`, `signed`, plus a tag for Bool/Char/Enum/Int and the base type for
subranges), `Real`, `Str(n)`, and later `Array`, `Record`, `Set`, `Ptr`, `Proc`. Size and
signedness are fields, never recomputed. Procedure signatures are interned to one WASM
type each (plan §4.5).

## Symbols

```rust
#[derive(Clone, Copy)] pub struct Sym(pub u32);   // index into the symbol table
```

A symbol has a name index, a kind (`Const`, `Type`, `Var`, `Param`, `Proc`, `Func`,
`Field`, `Unit`, built-in `Intrinsic`) and kind data: for variables, `ty`, lexical
`depth`, routine owner, storage (`Local(n)` / `Frame(offset)` / `Global(addr)`) decided
after checking, and the `captured` / `addr_taken` flags. Scopes are a stack of
`(name, Sym)` vectors searched innermost-first; TP5 identifiers are case-insensitive, so
names are interned uppercase by the lexer/parser.

## Errors

Every phase returns `Result<_, Diag>` with `Diag { code: u16, line: u32, col: u32 }` and
stops at the first error, as TP does. Codes come from `docs/tp-errors.md`; 200 is
"Feature not supported" (decision 6).
