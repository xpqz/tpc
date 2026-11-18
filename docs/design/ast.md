# AST, types and symbols: the front-end contract

Describes `compiler/src/{ast,types,check,codegen,parse,driver}.rs`. Changing the contract is a
decision in `docs/decisions.md`; the code and this file change together.

## Pipeline

`compile` (`driver.rs`): lex System, lex the program, `parse`, `check`, `gen`. There is no IR;
codegen walks the checked AST.

- **System (decision 11).** `runtime/system.pas` is embedded with `include_bytes!` and parsed as a
  plain declaration part. Every System token has line 0, so System diagnostics never carry a user
  line. The program's declaration list starts with System's declarations (`Ast::system` of them).
  The checker checks those first with `user == 0`, so their routines are `builtin`, and the
  program's own declarations start a new redeclaration region (`user`): predefined and System
  names may be redeclared, a duplicate within the program is error 4. A forward routine left
  undefined in System is error 59 at line 0.
- **Helpers by handle.** Right after System is checked, the checker looks up the names in `HELPERS`
  (`ABSLONG STRCAT STRCMP STRASSIGN STRDELETE STRINSERT`) and stores their routine indexes in
  `Info::helpers`, addressed by `H_*`. A missing helper is error 3 at line 0. Later code never
  looks helpers up by name, so a user routine with a helper's name cannot capture them.
- **Whole program.** The checker finishes every routine before codegen starts: storage of a
  routine's locals is decided when its body is done (see Storage), and a routine's frame size is
  needed by its callers' code only through `Info`.
- **Errors.** Every phase returns `Result<_, Diag>`, `Diag { code, line, col }`, and stops at the
  first error, as TP does. Codes come from `docs/tp-errors.md`; 200 is "Feature not supported".
  End of file is error 10 and `driver.rs` moves it to the last line of the file.

## Arena (`ast.rs`)

`Ast { nodes: Vec<Node>, lists: Vec<Id>, names: Vec<Vec<u8>>, strs: Vec<Vec<u8>>, system, follow }`.
`Id = u32` (0 is a valid node); `List(start, len)` is a range of `lists`; `intern` stores
identifiers uppercase (TP is case-insensitive) and returns the index; `strs` holds string-literal
bytes. `Node { kind: Kind, line, col, ty: Ty }`; `ty` is `Ty::NONE` after parsing and set by the
checker on every expression. `Kind` is `Copy` (handles, lists, `Sym`, small enums, numbers); no
`Box`, `Rc` or `Vec` inside. `NO_NAME` marks an absent optional name. `Op` covers binary
operators, `Neg`, `Not` and the unary intrinsics `Ord Chr Succ Pred Odd Abs Sqr`.

Node kinds, by group:

- **Declarations.** `Program(name, decls, body)`, `ConstDecl`, `VarDecl(name, type name)`,
  `TypeDecl(name, def)`, `Routine(name, params, result type, is_func, block)` (block `None` for
  `forward`), `Param(name, type name, by_ref)`, `Block(decls, body)`.
- **Type definitions** (the `def` of a `TypeDecl`, or a type position): `Name`, `Range`, `Enum`,
  `StrType(size)`, `Array(index types, elem)`, `Record(fields, at most one Variant)`,
  `FieldDecl(name, type)`, `Variant(arms)`, `ProcType(params, result, is_func)`.
- **Statements.** `Assign`, `Compound`, `If`, `While`, `Repeat`, `Case(selector, arms, else)`,
  `Arm(labels, body)`, `Range(lo, hi)` (case label), `For(var, first, last, body, down)`, `With`,
  `Exit`, `Write(args, newline)`.
- **Calls.** Parser output `Call(callee Name, args)`, as statement or expression; checked forms
  `Invoke(routine, args)`, `Indirect(callee value, args)`, `ProcRef(routine)`.
- **Expressions.** `Name`, `IntLit` (Integer, Boolean, Char and enum constants), `StrLit`, `Var(sym)`,
  `Unary`, `Binary`, `Index(base, index, continues)`, `Field(base, name | offset)`, `Fmt(value,
  width)` (write arguments only), `Paren`, `AddrOf` (always error 200), `CharToStr`.

## What the checker does to the tree

The checker mutates nodes in place; `expr(e)` and `stmt(s)` return `R<()>`, and a later phase reads
`nodes[e].kind` and `.ty`. Codegen never sees `Name`, `Call`, `Paren` or `AddrOf`.

1. **Name resolution in place.** `Name` becomes `Var(sym)`, `IntLit` (constant), `StrLit` (string
   constant), `Invoke` (a parameterless function), `Indirect`, `ProcRef` or `Field` (a field of a
   `with` record). `Call` becomes `Write`, `Exit`, `Invoke`, `Indirect`, an `Assign` (see below),
   or, in an expression, a unary operator, a constant (`SizeOf`, casts of constants), a cast
   (`T(x)` becomes `Unary(Ord, x)` of type `T`) or `Invoke`. A `For` control variable and a `var`
   argument are resolved by the statement or call that owns them.
2. **Conversions.** Only `CharToStr(c)` exists (`coerce_str`): a Char used where a String is wanted
   becomes a one-character `StrLit` when constant, else wraps a copy of the Char node. It is applied
   to an assignment to a string, to either operand of a binary operator that involves a string, and
   to `Concat` arguments. A one-character string literal is typed Char (`IntLit`) by `expr`.
   Integer widening and narrowing have no nodes: all integers are i32 normalised to their type's
   range, and codegen narrows from the node's `ty` after arithmetic and on store (plan §4.6).
3. **Constant folding.** `IntLit` operands fold in the checker at LongInt precision (`fold`); the
   result takes the smallest predefined integer type (`const_ty`), other ordinals keep theirs.
   Two string literals joined by `+` fold into a new `StrLit`. `Sqr` is not folded (not constant in
   TP). A constant outside a destination's range is error 76; division by constant zero is 62.
4. **`ProcRef` and `Indirect`.** A global routine named where a procedural value of type `want` is
   expected (assignment source, value argument) becomes `ProcRef(r)` after the signature is
   matched by `Ty` identity, and marks `Routine::addressed`; nested routines are error 143. A call
   of a variable of procedural type becomes `Indirect(callee, args)`, `callee` being a `Var` node
   whose `ty` is the procedural type. A procedural variable named in an expression is called.
5. **`Paren`.** The checker copies the inner node's kind and type over the `Paren` node. A `Paren`
   met by `var_arg` is not a variable (error 20 from `bad_var_arg`), which is its only purpose.
6. **`with`.** The designators are checked and pushed on `Cx::withs` with their record types. A bare
   `Name` that is a field of an enclosing record (innermost first) becomes `Field(designator, offset)`
   sharing the designator's node; codegen evaluates each designator's address once into a local and
   uses it wherever it meets that node (`Gen::withs`). `Field(base, name)` becomes
   `Field(base, offset)`.
7. **String statements.** `Delete(s, i, n)` becomes `Assign(s, Invoke(StrDelete, [s, i, n]))` and
   `Insert(src, s, i)` becomes `Assign(s, Invoke(StrInsert, [src, s, i]))` (the helper handle comes
   from `Info::helpers`); `Concat(a, b, ...)` becomes a left-nested chain of `Binary(Add)` typed
   String. These calls reuse the original argument list and node positions.
8. **String operators are lowered in codegen**, not in the tree. The checker types `a + b` as
   `Ty::STRING` and a string comparison as Boolean and leaves `Binary`. Codegen's `str` evaluates
   `Binary(Add, a, b)` into a temporary with `StrCat(temp, a, b)`; a comparison is
   `StrCmp(a, b)` compared with 0 by the operator; `s := e` is `StrAssign(s, e, capacity)`, the one
   truncating store (decision 17).
9. **Argument checking.** `arguments` checks count (85/86 none expected, 87 too few, 88 none given,
   89 too many), then per parameter: by-reference needs a variable of the identical type, a
   procedural parameter goes through `proc_value`, else `expr` plus `assignable`. For a `builtin`
   (System) routine a type mismatch reports the TP built-in codes (67, 106, 38) instead of 26.

## Types (`types.rs`)

`Ty(u32)` indexes `Types::v: Vec<Type>`. Fixed indices: `NONE 0`, `STRING 1` (`String[255]`), `BYTE`,
`SHORTINT`, `INTEGER`, `WORD`, `LONGINT`, `BOOLEAN`, `CHAR` (2 to 8). `Real` and the other
floating types are `SymKind::Omitted` and give error 200.

```rust
pub enum Type {
    None,
    Str(u8),                                  // String[n]: capacity; size n + 1
    Proc { params: Vec<(Ty, bool)>, result: Ty },   // (type, by_ref); result NONE for a procedure
    Array { index: Ty, elem: Ty, size: u32 }, // one dimension; a[i, j] is an array of arrays
    Record { fields: Vec<(u32, Ty, u32)>, size: u32 },  // (name, type, offset); variants overlaid
    Ord { lo, hi, size: u8, signed, tag: Tag, base: Ty },
}
```

- **Ordinals.** `Tag` is `Int`, `Bool`, `Char` or `Enum`. `base` is the type that decides arithmetic
  and compatibility: the predefined integer, `Char`, `Boolean` or the enumeration itself; a type is
  its own base unless it is a subrange. `compat(a, b)` holds when the tags are equal and, for
  `Enum`, the bases are the same. Enumerations and subranges are stored in the smallest of ShortInt,
  Byte, Integer, Word, LongInt that holds both bounds, in that order (decision 16). `ord(t)`
  returns `(lo, hi, size, signed)`; for `Proc` it is a 4-byte unsigned table index.
- **Operator typing.** `binary` and `unary` return the result type or the TP error number (26 operand
  families differ, 41 operator undefined, 38/39/79 for the intrinsics). Integer results are the
  smallest of Integer, Word, LongInt that holds both operand ranges; shifts promote the left
  operand.
- **Strings.** Every `String[n]` form creates its own `Type::Str` (name equivalence); all are
  assignment compatible with each other and with Char. A string is an array of Char from 0 for
  indexing (`array_of`). `in_memory` is true for strings and aggregates: they are passed by address.
- **Aggregates.** Assignment needs the identical `Ty` (name equivalence) and is a `memory.copy`.
  A function result may not be an aggregate (error 34), but may be a String.
- **Interning.** Only `Proc` is interned (by `params` and `result`), so equal signatures are one
  `Ty` and `==` is type equality; codegen maps each `Proc` type to one WASM function type.
  `Array`, `Record`, `Str` and ranges are created fresh per declaration.

## Symbols and storage

`Sym(u32)` indexes `Info::syms: Vec<Symbol>`. `Symbol { kind: SymKind, ty, val: i64, taken, depth }`.
Scopes are `Cx::scope: Vec<(name, Sym)>` searched innermost-first; a routine truncates it on exit.

| `SymKind` | `val` |
|---|---|
| `Prog`, `Type`, `Omitted` | unused (`Type` and `Const` carry `ty`) |
| `Const` | the value, or the `strs` index when `ty == STRING` |
| `Var` | static-data address of a global (globals start at `GLOBALS` = 280) |
| `Local` | index of the WASM local (parameters, hidden parameters and the result first) |
| `Frame` | offset from the routine's frame pointer; the variable lives in the shadow stack |
| `Ref` | index of the WASM local that holds the address of a `var` parameter |
| `FrameRef` | frame offset of the stored address of a captured `var` parameter |
| `Routine` | index into `Info::routines` |
| `Intrinsic` | Write 0, WriteLn 1, Ord 2, Chr 3, Succ 4, Pred 5, Odd 6, Abs 7, Sqr 8, Exit 9, SizeOf 10, Concat 11, Delete 12, Insert 13 |

`depth` is the lexical depth of the owning routine (0 for the program and its globals).

**When storage is decided.** A local starts as `Local` or `Ref`. `taken` (one flag for both
address-taken and captured) is set (a) at declaration for strings and aggregates, which always
live in the frame, and for String/aggregate value parameters; (b) by `capture` when a name is
resolved inside a deeper routine than its owner (`Local` or `Ref`); (c) by `capture` when a
`Local` is passed as a `var` argument. When a routine's body has been checked, `place_frame` walks
the symbols created for it and rewrites each `taken` `Local` to `Frame` and each `taken` `Ref` to
`FrameRef`, assigning offsets in declaration order with no padding. `Kind::Var(sym)` holds the
handle, so every use sees the final kind at codegen. Static link, if any, is at offset 0; a
captured `var` parameter stores its 4-byte address. A `taken` parameter that arrived in a WASM
local is recorded in `Routine::copies` (local, offset, type) and copied into the frame by the
prologue (strings through `StrAssign`, aggregates by `memory.copy`, scalars by store).
`Routine::framed` is set when the frame holds anything or the routine has nested routines.

Function results: a scalar result is the WASM local after the parameters, with a symbol in
`Routine::result_sym` that an assignment to the function name resolves to. A String result is a
`Ref` to the hidden result parameter.

## Routines, frames and static links

`Info::routines: Vec<Routine>` (params, `byref`, names, result, `func`, `defined`, `body`, `nlocals`,
`depth`, `children`, `framed`, `frame`, `result_sym`, `copies`, `builtin`, `result_ptr`, `addressed`).
A routine is entered at its header (so `forward` and recursion resolve) and completed at its block.
Top-level routines have depth 1; the program body is depth 0.

- **Parameter order** in WASM: the static link (depth 2 or more), then the hidden String-result
  address (`result_ptr`), then the declared parameters. By-reference and in-memory parameters are
  i32 addresses.
- **Frame.** A framed routine subtracts `frame` plus its string temporaries from the shadow-stack
  pointer in the prologue (trapping with 202 below the limit), stores the static link at offset 0,
  runs the `copies`, runs the body inside `block $exit` (which `Exit` branches to), and restores
  the pointer. The frame size is read after the body is generated, because it also holds the
  string temporaries. The main program has a frame only for temporaries.
- **Static links.** A routine at depth `d` reaches a variable of depth `v` by `frame_ptr(d - v)`:
  its own `fp` for 0, else local 0 followed by `d - v - 1` loads through offset 0. A call of a
  routine of depth `r` from depth `d` passes `frame_ptr(d + 1 - r)` first. Only `Frame` and
  `FrameRef` variables are reached this way; a `Local` is never visible to another routine.
- **Procedural values.** A routine with `addressed` set gets a table slot (slot 0 is a trap stub)
  and is a root of the worklist. A procedural value is its 4-byte table index; `Indirect` pushes
  the arguments, the index, then `call_indirect` with the WASM type of the `Proc` type.

## String temporaries and the hidden result

A string-valued expression that is not a literal or a variable (concatenation, `CharToStr`, a
String function call) is evaluated into a 256-byte frame temporary. Temporaries live after the
declared frame, are not reused within a statement, and the frame holds the largest number any
statement used (`Gen::temp` resets at each statement, `temps` is the maximum). A call of a String
function passes the address of a fresh temporary as the hidden parameter and the call's value is
that temporary. Value parameters of String type are copied into the callee's frame (`copies`).
Truncation to capacity happens only in `StrAssign`.

## Dead-code elimination (decision 14)

`gen` emits the main body first, then every `addressed` routine, then loops over
`Layout::order` until it is exhausted. `Layout::func(r)` assigns a WASM function index on first
reference, so only reachable routines (user and System, helpers included) are emitted. String
literal data is likewise emitted only when a live body references it (`Layout::lit`), after the
globals. WASM function indices: host imports, the built-in `wstr`, `wint`, `quot`, then routines in
reference order, then `main`.

## Error-position conventions

TP reports at the lookahead token, so the parser does too.

- `Parser::err` is at the lookahead token (end of file is always 10). `Parser::after` stamps a
  node with the position of the token after its last operand: `Unary`, `Binary`, `Assign`, `Fmt`
  and similar nodes. `Call` is stamped after its argument list; its callee `Name` keeps the name's
  position. `Routine` is stamped after its header; `Block` where the declarations start; compound
  statements at `begin`. Other nodes carry their own start.
- The checker reports an error at the node it is checking (`Cx::err`), so a mismatch in an
  assignment is at the assignment's stamp and in a call at the call's stamp.
- A missing argument list (error 88) is reported on the line of the token after the identifier,
  from `Ast::follow` (callee id, line of the following token).
- Errors in System have line 0.
