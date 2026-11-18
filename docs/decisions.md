# Design decisions

Numbered, append-only. Each entry: question, decision, reason, alternatives rejected.
A later decision may supersede an earlier one by number; the earlier entry stays.

## 1. Implementers work in worktrees

**Question.** How do parallel implementers and the lead share the repository?
**Decision.** The main checkout stays on `main` for the lead. Each implementer works in
`worktrees/{id}-{slug}` with its own Cargo target directory.
**Reason.** A merge in a checkout that an implementer had switched to its branch landed on
the wrong branch. Separate checkouts remove that failure and the cargo lock contention.
**Rejected.** One shared checkout with serial implementers: no parallelism.

## 2. Counted paths and test code

**Question.** What counts towards the LOC budget?
**Decision.** `compiler/src/` (plus `build.rs`), `runtime/`, `ide/`, `host/`. No test code
counts: Rust tests, the Node runner, test programs and fixtures are all outside the budget.
Test code never lives in a counted path.
**Reason.** User direction; tests are meant to be rich.

## 3. Opcode constants live in the emitter, added on demand

**Question.** Should `wasm.rs` carry a table of opcode constants?
**Decision.** No table up front. Codegen adds named constants to `wasm.rs` for the opcodes
it uses, when it first uses them; they count as emitter lines. Emitter tests keep their own.
**Reason.** A full opcode table costs lines for opcodes never emitted.
**Rejected.** Raw hex literals in codegen: unreadable and error-prone.

## 4. Pre-commit hook versus merge gate

**Question.** What does the pre-commit hook enforce?
**Decision.** fmt, clippy and `tools/loc.sh` only. `make check` (all tests) is the merge
gate, run by the lead on the branch with `main` merged in.
**Reason.** RED commits contain failing tests by design, and `--no-verify` is forbidden.
**Rejected.** Running `make check` in the hook: blocks every RED commit.

## 5. Integer expression typing

**Question.** What are the result types and precision of integer expressions?
**Source.** TP 5.0 Reference Guide (docs/references.md), Ch. 3 "Integer Types" and Ch. 6.
Quoted: "The type of an integer constant is the predefined integer type with the smallest
range that includes the value"; "both operands are converted to their common type before
the operation. The common type is the predefined integer type with the smallest range that
includes all possible values of both types ... the common type of integer and byte is
integer, and the common type of integer and word is longint. The operation is performed
using the precision of the common type, and the result type is the common type."; "The
expression on the right of an assignment statement is evaluated independently from the
size or type of the variable on the left."; "Any byte-sized operand is converted to an
intermediate word-sized operand that is compatible with both integer and word before any
arithmetic operation is performed."; sign negation and `not`: "the result is of the same
integer type"; shifts: "The type of the result is the same as the type of i."
**Decision.** Sourced clauses are applied as quoted. Where the source leaves room, the
following are **provisional** (tests pinning them live in `tests/lang/int_typing_*` and
name this decision):
- (P1) Byte-sized operands of `+ - * div mod` and comparisons are widened before the
  common type is formed: `Byte` and `ShortInt` both widen to `Integer`, except that `Byte`
  paired with `Word` gives `Word`. So `Byte op Byte` is `Integer` (`200+200` = 400,
  `1-2` = -1), `ShortInt op Word` is `LongInt`.
- (P2) Unary minus on a byte-sized operand widens first (arithmetic), giving `Integer`;
  on `Word` the result is `Word` (wraps mod 65536), as sourced.
- (P3) `not`, `shl`, `shr` keep the operand's own type, including byte-sized ones
  (`not Byte(0)` = 255, `Byte(200) shl 1` = 144). `shr` is logical within that width.
- (P4) A constant whose value fits both `ShortInt` and `Byte` is `ShortInt` if negative,
  else `Byte`; this is unobservable given P1.
- `div`/`mod` truncate toward zero; `Word` and `Byte` use unsigned division.
- Results are re-narrowed to the result type after `+ - *` and unary minus (plan §4.6), so
  `Integer` arithmetic wraps at 16 bits: `30000+30000` is -5536 even inside a larger
  expression. fpc differs (native width); see docs/deviations.md D8.
- Subrange base type is the smallest predefined type containing both bounds; signedness
  follows it (`0..200` unsigned, `-1..100` signed).
**Rejected.** fpc's native-width evaluation: contradicts the TP5 manual.
**Superseding.** If a TP5 binary or further text contradicts P1-P4, a new decision
supersedes the clause and its tests change with a reference to that decision.

## 6. Walking-skeleton ABI constants

**Decision.** Memory starts at 16 pages (1 MB). Static data starts at 256. `sp` (global 0)
starts at the top of initial memory (1,048,576) and grows down. `String` is `String[255]`.
`{$R-}` is the default, as in TP5. Errors for omitted features use compiler error number
**200, "Feature not supported"**, outside TP's 1-146 range, so it can never be confused
with a TP error. The module exports `main` and `memory` (#13 contract).
**Reason.** Plan §4.1 fixes the layout but not the numbers; these are the simplest values.

## 7. Phase 1 test formats and the sieve

**Decision.** `tests/lang/NAME.pas` + `NAME.out` (fpc output). `tests/neg/NAME.pas` +
`NAME.err` holding one line `N LINE` (TP error number and 1-based line). A shrinking
`tests/lang/PENDING` / `tests/neg/PENDING` lists programs not yet expected to pass; the
driver fails if a pending program passes, so the list cannot go stale. The compiler
reports errors as `NAME.pas(LINE,COL): Error N: MESSAGE` on stderr, exit code 1.
The Phase 1 exit test "sieve" needs arrays, which arrive in Phase 3; it moves to the
Phase 3 exit tests. Phase 1 uses a trial-division prime listing instead.
**Reason.** Test-first corpus without blocking on later phases; arrays are Phase 3 scope.

## 8. Front-end contract

**Decision.** `docs/design/ast.md`: one `Vec<Node>` arena with `u32` handles and child
lists as ranges into a shared `Vec<u32>`; the checker resolves names in place, sets
`Node::ty`, and inserts only `IntToReal` / `CharToStr` conversion nodes by returning a
(possibly new) handle that the caller stores; integers stay normalised i32 so widening
needs no node and narrowing is driven by `ty` in codegen.
**Reason.** Smallest design meeting plan §8 (arenas, no Box/Rc, no IR) and §4.6.
**Rejected.** A separate typed IR (lines); coercion nodes for every widening (no-ops).

## 9. Keyword and operator tokens carry text, not enum variants

**Decision.** The lexer emits `Kw` and `Sym` token kinds whose uppercase / canonical text
identifies the keyword or operator; the parser matches on text.
**Reason.** One variant per keyword and operator cost ~100 lexer lines plus a mapping
table; a packed keyword array is ~10 lines.
**Rejected.** Per-keyword enum variants.
