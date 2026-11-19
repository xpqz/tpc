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

## 10. Integer typing measured on TP 5.0 (supersedes decision 5 clauses P2-P4)

**Source.** Real TP 5.0 (`tools/tp5.sh`), probe programs in docs/prs of the arithmetic
slice. Measured: `Byte 200+100` = 300, `Byte 1-2` = -1, `Byte 255*255` = -511 (P1
confirmed); `Integer (30000+30000) div 2` = -2768, `i+1 > 0` with i=32767 is FALSE
(16-bit wrap everywhere, decision 5 confirmed); `-Word(40000)` = -40000; `not Byte(200)` =
55; `Byte(200) shl 1` = 400, `Byte(200) shl 9` = -28672; `ShortInt(-1) shr 1` = 32767;
`Word(40000) shl 1` = 14464; `Byte 1 + Word 65535` = 0; `ShortInt -1 + Word 65535` = 65534;
`30000 * 2 div 2` (constants) = 30000; `65535 + 1` (constants) = 65536.
**Decision.**
- P1 stands: byte-sized operands widen to `Integer`, except `Byte` with `Word` gives `Word`.
- P2 replaced: unary minus gives `Integer` for byte-sized and `Integer` operands, `LongInt`
  for `Word` and `LongInt`.
- P3 replaced: `not` keeps the operand's type, including byte-sized types. `shl`/`shr`
  widen a byte-sized left operand to `Integer` (ShortInt sign-extends) and keep `Word`,
  `Integer`, `LongInt`; `shr` is logical within the 16- or 32-bit width.
- Constant expressions are folded in 32-bit (`LongInt`) precision; the folded value then
  has the smallest type containing it (decision 5 constant rule).
- Tests come from `tools/tp5.sh`; decision 5's provisional-test naming is no longer needed.

## 11. System unit before units exist

**Decision.** `runtime/system.pas` (counted, Pascal budget) is embedded in the compiler
with `include_bytes!` and parsed as a plain declaration part in an outer scope that
encloses the program, at the program's routine depth (no static links). Programs may
redeclare System names (TP5 accepts `var Length: Integer;`, measured). Helpers the
compiler calls implicitly (string concat/compare/copy etc.) are bound to symbol handles
right after System is loaded, never looked up by name at use sites, so a user routine
with a helper's name cannot break them. Unit syntax arrives in Phase 5.
**Reason.** Whole-program compilation (plan §3) with no unit machinery yet.
**Note.** Embedding a counted runtime file with `include_bytes!` is the one permitted use
of an include mechanism; the file is counted where it lives.

## 12. Aggregate copies are bulk-memory instructions; the heap moves to Phase 4

**Decision.** Aggregate assignment, value-aggregate callee copies (plan §4.3) and string
moves use WASM `memory.copy`/`memory.fill` emitted inline, not a Pascal `MemMove`. No
raw-memory construct (`Mem[]` etc.) is added to the language. The bump allocator, `New`,
`GetMem` move to Phase 4 with pointers, where a heap is first observable.
**Reason.** Shortest code; nothing in Phase 3 can observe a heap.

## 13. One mechanism for string values

**Decision.** Every string-valued expression evaluates into a 256-byte frame temporary;
one truncating store copies a temporary into a `String[n]` destination (length byte
clamped to n). Concatenation, `Copy`, `Insert`, `Delete` and string function results all
go through it. System string helpers are ordinary Pascal using string indexing.
**Reason.** Aliasing (`s := 'a' + s`) and capacity are handled in one place.

## 14. Function-level dead-code elimination

**Decision.** Codegen emits function bodies from a worklist rooted at `main` (and later
unit initialisers and address-taken routines), assigning WASM indices on first
reference. String-literal data is emitted only when referenced; globals are laid out
before literals.
**Reason.** System grows; hello-world must not.

## 15. Hidden result pointer test moves to Phase 3

The plan's Phase 2 exit test "a function returning through the hidden result pointer"
needs strings; it lands in `tests/abi` with the strings slice.

## 16. Enumeration and subrange storage and typing (measured on TP 5.0)

**Decision.** An enumeration or subrange is stored in the smallest of ShortInt, Byte, Integer,
Word, LongInt that holds both bounds, in that order of preference, so `(a,b,c)` and `0..100` are
signed bytes (`Pred` of the first value reads back -1) while `0..200` and a 200-value enum are
unsigned bytes and `0..1000` and a 257-value enum are signed words. Arithmetic uses the type's own
range (`0..100` plus a Word wraps like Byte plus Word; `-w` of `0..60000` is LongInt); `Succ`,
`Pred` and `not` of an integer subrange keep the subrange type, `not` of a Boolean subrange is
Boolean. Enumerations are compatible only with the same enumeration (and its subranges). A
constant outside the destination's range is error 76 for every ordinal family; run-time values
are not checked (`{$R+}` is Phase 4). `T(x)` casts an ordinal value to an ordinal type;
`SizeOf` of a type identifier or a variable is a folded constant.
**Reason.** Probes in `docs/prs/35.md` (`e4`, `e5`, `e6`).
**Rejected.** Treating the smallest predefined type with Byte first: contradicts the `-1` readback.

## 17. Strings: helpers, temporaries and TP built-ins

**Decision.** `String[n]` types are `Type::Str(n)`, one per form (name equivalence); all are
assignment compatible with each other and with Char. Operators and assignment call System helpers
bound by handle at load from one name table (`ABSLONG STRCAT STRCMP STRASSIGN STRDELETE
STRINSERT`; a missing helper is error 3 at line 0): `a + b` is `StrCat(temp, a, b)`, comparison is
`StrCmp(a, b)` against 0, `s := e` is `StrAssign(s, e, n)` (the one truncating store), a String value
parameter is copied into the callee frame by `StrAssign`. Every string-valued expression that is not a
literal or a variable is evaluated into a 256-byte frame temporary; a statement's temporaries are not
reused within it, and the frame size includes the largest number any statement used (the body is
generated before the prologue). A String function result is a hidden `var` parameter after the static
link; a call passes a fresh temporary. `Delete(s, i, n)` and `Insert(src, s, i)` are rewritten to
`s := StrDelete(s, i, n)` and `s := StrInsert(src, s, i)`, so aliasing and the capacity of `s` come
from the assignment; `Concat` is a chain of `+`; `Length`, `Pos`, `Copy` are ordinary System routines.
Arguments of System routines are diagnosed like TP built-ins (67 string expected, 106 character
expected, 38 integer expected). A missing argument list is reported at the token that follows the name.
`Char` operands become one-character literals or `CharToStr` nodes. Strings are indexed as
`array[0..n] of Char`; the length byte is `s[0]`.
**Reason.** Decision 13 with the least compiler code: the helpers are Pascal.
**Rejected.** Open string parameters carrying a capacity (TP7): the rewrite gives the capacity
for free. Inline `memory.copy` for the truncating store: needs a clamp and locals; the Pascal helper is
shorter and is removed by function-level elimination when unused (decision 14); aggregate copies
still use `memory.copy`.

## 18. Heap layout for Phase 4

**Decision.** Fixed heap: from the end of static data up to the stack limit (initial
memory top minus the 64 KB stack). No `memory.grow`. Exhaustion is runtime error 203.
`MemAvail`/`MaxAvail` report our heap and differ from TP5's; tests print only relations
(`MemAvail > 0`, before/after comparisons), never values.
**Reason.** Pages from `memory.grow` would arrive above the stack, so a growing heap
needs a different layout; a fixed heap is fewer lines and plan §4.1 only says "may".

## 19. Real arithmetic and expectations

**Decision.** `Real` is f64 (D1). `Write` of reals goes through `host.fmt_real`,
implemented in `host/` (counted, host glue budget) and imported by tests/run.mjs. Tests:
TP5 output is the oracle for formatting; programs print values that are exact in both
48-bit and f64 representations, or computed values where TP5 and fpc (with `SizeOf(Real)`
= 8) agree. A program whose digits differ between TP5 and f64 is not used.
**Reason.** TP5 computes in 48 bits; the plan's "matched against Free Pascal" predates the
TP5 oracle.

## 20. Range checking scope

**Decision.** `{$R+}` checks assignments (and value arguments) to subranges and narrower
integer types, and array indexes; runtime error 201. String indexes are not checked:
TP 5.0 does not check them (measured: `s[4]` on `String[3]` under `{$R+}` runs).

## 21. Phase 4 names and pointer casts

**Measured on TP 5.0.** `Include`/`Exclude` do not exist (error 3; sets use `+`/`-`).
`LongInt(p)` and `PtrType(l)` casts are accepted; `MemAvail` and `MaxAvail` exist. The
allocator in System uses these casts for address arithmetic.

## 22. Checker budget pacing

**Decision.** Phase 4 is given ~300 checker lines across its slices and Phase 5 ~200;
if the checker ends Phase 4 above ~1,650, re-plan before Phase 5. Corpus briefs cap
negative tests at one per rule in plan scope; per-context error-number parity beyond
that is pruned at the RED verdict.

## 23. Front-end contract as built (supersedes parts of decision 8)

`docs/design/ast.md` now describes the code: the checker rewrites nodes in place (no
handle return); `taken` covers both captured and address-taken; only procedural types are
interned; string operators are lowered in codegen; `CharToStr` is the only conversion node
so far (`IntToReal` arrives with Real). Where the doc and code disagree, the code wins and
the doc is fixed in the same PR.

## 24. Range checking covers string indexes (corrects decision 20)

**Measured.** Under `{$R+}`, TP 5.0 raises runtime error 201 for a variable string index
beyond the declared size (`s[4]` on `String[3]`, read or write); `s[0]` .. `s[n]` run;
a constant index beyond the size is compile error 76. Decision 20's "string indexes are
not checked" came from a probe where the index appeared inside a comparison; the #39
corpus pins the general rule. `{$R+}` therefore checks assignments/value arguments to
subranges and narrower integers, array indexes, and string indexes against the declared
size.

## 25. Double Dispose is not detected

`Dispose` of a pointer already disposed is runtime error 204 in TP 5.0; detecting it needs
free-list bookkeeping beyond the plan's allocator. Not detected (deviation D13).
`Dispose(nil)`, `FreeMem(nil, n)` and pointers outside the heap are 204, as in TP 5.0.

## 26. Multi-file test fixtures

**Decision.** A test program `DIR/NAME.pas` may have a sibling directory `DIR/NAME/` with
auxiliary files (units, include files, data files; DOS 8.3 upper-case names) and a
sibling `DIR/NAME.in` used as console input. `tools/tp5.sh`, `tools/oracle.sh`, the test
drivers and `tests/run.mjs` all use this convention: the auxiliary files are available to
the compiler (units, includes) and to the program (data files, via the runner's
directory-backed VFS); `.in` is stdin. Program-created files are observed only by the
program reading them back and printing.

## 27. The compiler receives all files before compiling

**Decision.** The host registers every source file (units, includes) by name before
calling `compile`; the compiler looks units and includes up in that table and never calls
back into the host. The wasm API gains `add_file(name_ptr, name_len, src_ptr, src_len)`
(plan §3 amended); the native CLI registers the files in the input's directory.
**Reason.** `ffi.rs` holds its state borrow for all of `compile`; a re-entrant callback
would abort, and a pre-registered table is simpler.

## 28. Diagnostics carry a file

**Decision.** `Diag` gains a file index (0 = the main program); the CLI prints
`FILE(LINE,COL): Error N: MESSAGE` with the file's own name; the wasm API gains
`err_file()` returning the registered index. `.err` files keep `N LINE` for errors in the
main program and use `N LINE FILE` for errors in another file. System stays line 0.

## 29. Host imports declared in runtime sources

**Decision.** A routine declared `external` in an embedded runtime source (System, later
Crt and Dos) becomes an import from module `host` with the routine's name, emitted only
when reachable (DCE). This replaces per-import index shifts. `external` in user code stays
error 200.
**Reason.** Phase 5 and 6 add about a dozen host imports; one mechanism is shorter.

## 30. Phase 5 checker allowance

About 300 checker lines for the whole phase (decision 22 pacing); loading, ordering,
cycles, includes and conditionals live in `units.rs` (Driver component, ~850 lines free).

## 31. Readability over line count

**Decision.** The code is a teaching text for compiler techniques (user direction). It must
be simple and obvious; no code-golf to meet a budget. When a budget and clarity conflict,
the feature is cut or simplified and the cut is recorded. Reviews reject dense or clever
code even when it is correct and short. A readability review of the existing compiler
(#47) removes any golf already present.

## 32. Phase 5 semantics measured on TP 5.0 (#49)

Unit initialisation is the unit's `begin ... end.` part (`initialization` is error 37).
`Append` of a missing file is error 2, so VFS mode 2 fails with -2 when the file is
missing. A pending I/O error (`{$I-}`) makes every later I/O routine, console included, do
nothing until `IOResult` is read. `Read(c)` returns CR then LF at a line end and `Eoln` is
true before the CR, also for console input (the runtime synthesises CR LF after
`readline`). `{$I+/-}` and conditional symbols are local to their source file; an `$IF`
must close in the file that opened it (129).

## 33. Budget after the readability review

The readability review (#47) cost +1,053 lines (checker 1,844, codegen 1,417, Rust 6,035,
total 6,430). Readability stays first (decision 31). Remaining allowances: checker ~356
lines for Phases 5-8 (Phase 5 ~300 per decision 30, so Phase 5 must be lean and Phase 8's
error-table work goes in data, not checker logic); codegen ~1,080; Rust compiler subtotal
~3,965 under the 10,000. If the checker would pass 2,200, cut Phase 5 scope (candidates:
`{$I-}` file-local tracking, qualified names `U.X` beyond the plan, typed-constant edge
cases) before compressing code.

## 34. Symbols and frame layout move to their plan component

Plan §1 budgets "Types, symbols, frame layout" (1,400) separately from the checker
(2,200), but the scope stack, storage choice and frame placement were written in
check.rs. #57 moves them into symbols.rs under the types component. Conditions: symbols.rs
owns its state and never depends on the checker (guarded by a test); limits are
unchanged; behaviour is byte-identical. Measured before the move: about 165 checker lines.
Checker allowances for #52-#54 are set at each RED verdict at ~1.5x the implementer's
estimate, with negatives pruned there.
