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
unchanged; behaviour is byte-identical. Measured before the move: about 165 checker lines;
the move took 314, including the predefined-name table and System helper binding, which
are symbol-table work (checker 1,989 to 1,675; types+symbols 659 to 1,059).
Checker allowances for #52-#54 are set at each RED verdict at ~1.5x the implementer's
estimate, with negatives pruned there.

## 35. The `time` host import

`time(what) -> i32`: `time(0)` takes a clock reading and returns days since 1970-01-01
(local); `time(1)` and `time(2)` return seconds since midnight and hundredths of that same
reading. Supersedes plan §4.9's `time() -> i32` (an i32 of seconds overflows in 2038, of
milliseconds within weeks; separate readings could straddle a second). The runner's clock
is fixed by `TP5_NOW_MS` for tests. Crt's `Delay` gets its own contract in Phase 6.

## 36. Host runtime lives in host/ behind injected sources (#59)

**Decision.** The `host` imports are implemented once, in `host/runtime.js` (class `Host`) and
`host/vfs.js` (`Vfs`, `LineInput`), in plain ES modules over `Uint8Array`. Everything outside
is injected: `write(fd, bytes)` (output sink), `input.readLine(max)` (line source), `files`
(name -> bytes snapshot) and `nowMs()` (clock). `tests/run.mjs` is the Node adapter: it wires
the aux directory, stdin and `TP5_NOW_MS`, collects stdout and maps `HostError` to
`RunnerError`. Stack exhaustion (RangeError) is runtime error 202 inside `Host.runMain`.
**Measured.** The host part of `tests/run.mjs` (about 95 lines) plus `tests/vfs.mjs` (109)
became 238 lines in `host/` (runtime 131, vfs 107); the Node adapters are 56 lines in
`tests/vfs.mjs`. Host glue budget: 24 -> 207 of 500, leaving 293 for Phases 6 and 7.

## 37. Crt oracle (accepted, #59)

Measured under headless DOSBox-X with TP 5.0 (`tools/crt/`):
- A unit can read the text screen at `Mem[$B800:0]` (char at offset `row*160+col*2`,
  attribute at +1), the cursor from `Mem[$40:$50/$51]`, and `ExitCode`/`ErrorAddr`; an
  `ExitProc` installed in its initialisation runs when the program ends, also on a runtime
  error, and writes `SCREEN.TXT`. Works with `-silent -nogui`.
- Under Crt, `Write` goes to the screen even with `> RUN.TXT`, and `Input`/`ReadLn` read the
  keyboard even with `< P.IN` (a Crt program that reads stdin hangs). So Crt tests need the
  dump, and key injection is separate.
- `ReadKey`, `KeyPressed` and `ReadLn` under Crt go through BIOS INT 16h. A unit that
  replaces INT 16h with an `interrupt` procedure feeds scripted keys (tested with 45 keys;
  prefilling the 15-entry BIOS ring also works for short scripts).
**Proposal.** `tools/tp5.sh` runs `cls` first (else the screen starts with the DOS command
echo), and for a Crt test (marker: `DIR/NAME.crt` or `uses Crt`) inserts `ScrDump` (and
`KeyFeed` with keys from `DIR/NAME.keys`) as the first units of the program's uses clause
(first = exit procedure runs last), copies them from `tools/crt/`, and prints the dump after
the program output. Dump format: 25 lines of 80-column text with trailing blanks cut, then
`cursor COL ROW` (1-based), `exit CODE [error SEG:OFS]`, and `attr ROW COL LEN HEX` runs for
cells whose attribute is not `$07`. The runner prints the same from its screen model, so
`.out` files for Crt programs hold this dump. Open: the DOS "Runtime error N at" line is
printed after exit procedures, so it is not on the dump; the `exit`/`error` line stands for it
(ErrorAddr is a TP-internal address and must not be compared). Exhausted keys should halt
with a marker instead of repeating Enter.

## 38. Virtual clock for Delay (accepted, #59)

**Proposal.** `Host` takes a clock object `{ nowMs(), advance(ms) }` instead of `nowMs`. The
runner's clock is virtual: it starts at `TP5_NOW_MS` (or a fixed epoch) and moves only
through `advance`. Crt's `Delay(ms)` calls a host import `sleep(ms)`: the runner advances the
virtual clock and returns at once; the browser waits (`Atomics.wait`) and also advances. So
programs that measure `Delay` with `GetTime` see the elapsed time, tests never wait and never
observe real time. `KeyPressed`/`ReadKey` take their keys from the scripted source, not the
clock.

## 39. Screen state lives in the Crt unit; the host only draws cells

**Decision.** Cursor position, the current window and the text attribute live in the Crt
unit (Pascal, runtime budget), as they do in TP's own Crt. The host `screen` import is a
small set of cell operations (put a character with an attribute at a position, scroll a
rectangle, set the cursor) that a JS grid model applies and a canvas renderer draws; the
runner prints the same grid as the decision-37 dump. System's console output goes through a
procedural hook that Crt replaces in its initialisation, so plain `Write` honours
`Window`, `TextColor` and `GotoXY` once Crt is used, and programs without Crt are
unchanged.
**Reason.** After #59 the host has 293 of 500 lines for Phases 6 and 7, below the plan's
~450 estimate; TP itself keeps this state in Crt; the Pascal runtime has ~800 lines free.
**Rejected.** A terminal model with window and attribute state in JS (does not fit; would
duplicate state Crt needs anyway for `WhereX`/`WhereY`).

## 40. IDE oracle pattern (accepted, #67)

**Proposal.** An IDE module is differential-tested like any Crt program: a fixture
`NAME.pas` (`uses Crt, <module>`), a key script `NAME.keys` that ends with a Quit key (Alt-X)
and a golden `NAME.out` made by `tools/tp5.sh` (program output, then the screen dump of
decision 37). The program leaves the screen as the last key left it; a script that forgets
the Quit key ends with `exit 255` in the dump, so the omission is visible. The IDE host is
the only unit the real TP 5.0 cannot compile (`external` is error 200 in user code), so the
fixture directory holds `IDEHOST.PAS`, a stand-in with the same interface and canned
behaviour (compile fails with error 3 at the first line containing `ERROR`; run returns the
line count modulo 256, -2 when the last compile failed or there was none). The compiler finds embedded units first (`driver.rs`, `EMBEDDED_UNITS`), so with the
embedded `IdeHost` in place TP compiles the stand-in and this compiler compiles the embedded
unit; the Node runner then injects a canned compiler with the same two rules
(`host/ (promoted from the #67 drafts in tests/ide/draft/) canned.mjs`; its test replays the texts and results of the toy editor's
run on TP 5.0: `Error 3 in P.PAS 2:1; Run -2; Compiled; Run 2`). `tools/tp5.sh` gains
`DIR/NAME.dirs`: directories (from the repository root) whose files are copied before the
fixture's own, so an IDE module under `ide/` is found by TP and a fixture's `IDEHOST.PAS`
replaces nothing shared. The same list must reach `tp5c` and `tests/run.mjs --dir` (a merged
temporary directory in the Rust driver); until `ide/` exists only the oracle side needs it.
**Shown end to end** by `tests/lang/crt_ide_oracle_pattern_editor_unit_with_host_stub`: two
units (Editor, IdeHost) and a main program, 32 keys including `^K b`, `^Y`, F9, F10, Ctrl-F9
and Alt-X; identical dump on TP 5.0 and this compiler.
**Limits found** (compiler gaps, not fixed here): no `Move`/`FillChar` (a gap buffer shifts
text with loops or System gains a `Move`), `Halt` unknown (Error 3), untyped `var` parameters
(Error 200) and `Bytes(P^)[i]` (typecast then index, Error 85) are rejected.
**Rejected.** Testing against a TP IDE (TURBO.EXE is not in the oracle set and has no
scriptable host); a Pascal-side fake for both runs (it would not exercise the embedded
unit's imports).

## 41. Key chords: table, browser mapping, reachable bindings (accepted, #67)

**Decision (done in #67).** `tools/crt/KEYS.TBL` holds the Reference Guide's Table C.2
(Appendix C, Extended Key Codes, page 424) and the control codes: `^A`..`^Z` (Ctrl+letter,
word `scan*256 + n`, ReadKey returns 1..26), `Alt-A`..`Alt-Z`, `Alt-1`..`Alt-=`, `Shift-F1`..
`Alt-F12`, `F11`, `F12`, `Shift-Tab`, `Ctrl-Left/Right/Home/End/PgUp/PgDn`, `Ctrl-PrtSc`
(135 keys). The Node runner reads the same table, so `tests/keys.mjs` needed no change.
`^H`, `^I`, `^M` give the bytes of Backspace, Tab and Enter, as on a PC. `F13` replaces
`F11` as the example of an unknown key in three runner tests.
**Proposal (browser).** `host/keys.js` `keyBytes` today maps Ctrl+letter correctly but
turns Shift/Ctrl/Alt+F-key into the plain F-key, Alt+letter into the letter, Ctrl-cursor
into the cursor key and Shift-Tab into Tab. The replacement in `host/ (promoted from the #67 drafts in tests/ide/draft/) key_bytes.js`
(38 lines against 19 now, +20 in `host/keys.js`) follows Table C.2, identifies Alt+letter by
`event.code` (Option changes `event.key` on macOS), lets typing win where a layout makes Alt
or AltGr (Ctrl+Alt) produce a character, and leaves Alt+digit out. A test
(`host/ (promoted from the #67 drafts in tests/ide/draft/) key_bytes_test.mjs`) builds the event of every table key and compares the
bytes with the runner's, so the two cannot drift. Applying it makes `F11`/`F12` mapped keys
(extended 133/134): the negative case in `tests/host/keys.mjs` moves to `F13`.
**Reachability.** Measured: Chrome 157 (CDP) and Firefox 160 (WebDriver BiDi) deliver all
77 probed chords to the page, identically; but injected events bypass the browser's own
shortcuts (tab count unchanged after Ctrl+T/N/W), so this proves the page-side mapping, not
what a user's browser leaves alone. **Read from source** (Firefox `browser-sets.inc`, fetched
from gecko-dev): reserved, so never delivered to the page, are Ctrl+N, Ctrl+T, Ctrl+W,
Ctrl+Shift+W, Ctrl+Shift+P, and quit: Ctrl+Q on Linux, Ctrl+Shift+Q on Windows (so Ctrl+Q
reaches the page on Windows, not on Linux; macOS quits by another route). Chromium's
`IsReservedCommandOrKey` says no key is reserved in app windows and, in fullscreen, every
command except fullscreen/exit goes to the page (so Chrome fullscreen frees Ctrl+W/N/T); the
list of reserved commands in normal windows I could not locate in the source: from memory
it is new/close tab and window, tab switching (Ctrl+Tab, Ctrl+PgUp/PgDn) and exit
(Ctrl+Shift+Q). **Unverified, needs a person at a real keyboard:** everything on macOS (the
browsers use Cmd, so Ctrl chords should arrive, except probably Ctrl+Tab, Ctrl+PgUp/PgDn and
the Spaces shortcut Ctrl+Left/Right; F9 to F12 need Fn unless the user changed the keyboard
setting), Alt+letter menu activation on Windows/Linux, F10, and Chrome's reserved list.
Consequently: ^K B/K/C/V, ^Y, Alt-X, F9, Ctrl-F9 are expected reachable everywhere with Fn
caveat on macOS; ^Q F is not on Linux Firefox. **Substitutes (IDE side, a few lines in
Keys):** Alt-K and Alt-Q as synonyms of ^K and ^Q; every F-key command also in the menus
(Alt-R, Alt-F letters), so no command depends on F9/F10; ^A/^F (WordStar word left/right)
for Ctrl-Left/Right. Out of Phase 7 scope and unreachable in normal Win/Linux windows: ^W,
^N, ^T.

## 42. IDE host unit and the compile/run contract (accepted, #67)

**Proposal.** `IdeHost` is a runtime unit embedded in the compiler beside Crt (decision 29),
with the interface the oracle stand-in has (decision 40):
`type CompileResult = record Code, Line, Col: Integer; FileName: string[12] end;`
`procedure Compile(Name: string; Text: Pointer; Len: LongInt; var Result: CompileResult);`
`function Run: Integer;` (`Line` and `Col` are LongInt: a 64 KB buffer of short lines passes
32767 lines) over two `external` routines, imports `ide_compile(namePtr, textPtr,
textLen, resultPtr)` and `ide_run() -> i32` of module `host`, present only in the IDE module.
- **Compile.** `Text` is the unsaved buffer, contiguous (the IDE closes the gap first), `Name`
  the file name shown in errors. The host registers every `.PAS`/`.INC` file of its file
  system except `Name` with the compiler (`add_file`, decision 27), compiles the buffer as the
  main program and writes `Result`: `Code` 0 and nothing else on success, else the error
  code, line, column and the file of the error (`err_file`, decision 28; `Name` for the main
  program). The module stays in the host and never enters IDE memory. A unit being edited is
  compiled from its saved file, not from the buffer.
- **Run.** `Run` runs the last successful module and blocks the IDE until the program ends
  or Stop. It returns the exit code (0..255; a runtime error is printed on the screen by
  the program and returns its number), -1 when stopped, -2 when there is nothing to run.
  The screen is cleared first (a program's Crt does not read the screen; its state is its
  own) and left as the program left it; the keyboard queue is shared. The IDE shows
  "press a key" with ReadKey, then repaints everything (its Crt state is untouched).
- **Files.** Writing happens through ordinary Pascal text files (`fs_*`): File->Open by typed
  name is `Assign`/`Reset`, Save is `Rewrite`/`WriteLn`. No directory listing.
- **Browser.** Plan section 3's two Workers. The IDE Worker is the program Worker with the
  two imports added (compiler.wasm bytes in the start message). `ide_run` posts
  `{ run: module, files }` to the page and sleeps in `Atomics.wait` on a one-cell
  SharedArrayBuffer; the page starts the program Worker with the shared grid and key queue
  (today's `run`), and when it posts `done`, or Stop is clicked (terminate, as today), stores
  exit code + 2 (stop: 1) in the cell and notifies. A blocked Worker cannot receive messages,
  so files a program writes **do not come back**: the program Worker gets the page's file Map
  and its writes are discarded when it ends, as in Phase 6 (`host/vfs.js`: "none of it
  reaches the next run"), in the browser and in the Node runner alike. Compile and File->Open
  read the IDE Worker's file system, which holds the files saved by the IDE. Costs of
  upgrading: persisting a program's files for later runs and sessions is a `{ saved }` post
  from the program Worker (+3 lines, Node must then keep a store across runs); making them
  visible to Open in the same session needs an inbox in shared memory (about 25 lines).
- **Persistence.** The page owns it: `localStorage` keys `tp5:NAME` hold Latin-1 text, read
  at page load into a Map that goes to both Workers. `Vfs` gets an `onWritten(name, bytes)`
  callback fired when a file opened for writing is closed; the IDE Worker posts it as
  `{ saved }` and the page stores it and updates its Map (so the next run sees saved files). (`localStorage` is not available in Workers and IndexedDB is
  asynchronous, which a Worker blocked in wasm cannot use.)
- **Node runner.** `tests/run.mjs --compiler compiler.wasm` adds the imports to the `Host`:
  the compiler is instantiated once, `ide_run` is a nested synchronous run (a second `Host`
  sharing screen, keys, sink and clock; no Stop). For oracle tests `--compiler canned`
  injects `host/ (promoted from the #67 drafts in tests/ide/draft/) canned.mjs`. A nested run discards the program's files
  (`keepFiles` exists in the draft only to show the cost of the alternative). `host/ (promoted from the #67 drafts in tests/ide/draft/) ` holds working drafts of
  `compiler.js` and `ide.js` (42 + 25 lines) with 9 tests against the real compiler.wasm.
**Proof so far.** `host/ (promoted from the #67 drafts in tests/ide/draft/) draft_test.mjs`: error in main (code 3, line 3 and the
file; the oracle line from TP), error in a unit (file `U.PAS`, line 7), the unsaved buffer
beats a saved file of the same name, nothing to run is -2, a runtime-error run returns 200, run after a failed compile is -2 (the stand-in and
the canned compiler agree, from TP's run),
a Crt program run starts on a cleared screen, reads the shared keys and ends with the dump TP
prints, a bad pointer is a host error. Not exercised: a real IDE program (no embedded unit
yet), the browser path (drafted, not built), Stop.

## 43. Host budget transfer (accepted, #67)

Host glue is 471 of 500. Estimated need for decisions 41 and 42. Counted with the `tools/loc.sh` rules on working
drafts: `compiler.js` 42, `ide.js` 25, chord mapping +20. Estimated from sketches, not built:
IDE Worker wiring +12,
page (run relay, persistence, startup files) +28, `Vfs` write hook +5: +132. Removable test-only
code in `host/`: `UNAVAILABLE` stubs for `compile` -6, `LineInput` (the runner's `Input` uses it;
the Worker needs only `{ readLine: () => null }`; move it to
`tests/vfs.mjs`) -18: -24. Kept although only the runner needs them: the `readline`/`input` plumbing (about 8
lines; a program without Crt reads stdin through it) and `KEYS_EXHAUSTED` (4). Net +108, so about 579 (±20), 79 over. **Proposal:** raise the Host
glue budget to 600 and lower the margin from 500 to 400 (`tools/budget.txt`, plan section 1),
total still 15,000. The embedded `IdeHost` unit (about 20 lines of Pascal in `runtime/`, 1020
of 1500) and its `EMBEDDED_UNITS` entry (4 lines of Rust, 861 of 1100) fit their budgets.

## 44. Limits for a 64 KB gap buffer, measured on TP 5.0 (accepted, #67)

Probes in `tests/ide/limits/` (outputs from `tools/tp5.sh`; the two shared with this compiler
run in `compiler/tests/ide_tooling.rs`):
- `GetMem` takes a Word. The Reference Guide (GetMem, Restrictions) gives 65521 bytes
  (64K - $F) as the largest block; 65535 is accepted but a block of it that starts at a
  heap offset above 0 wraps in its segment: a probe filled one after a 100-byte block and
  overwrote 3 bytes of the first block, while 65521 overwrote none (`tp_only/getmem_block_*`).
  MaxAvail was about 536,000 bytes under DOSBox. A LongInt size is truncated (70000 gives 4464 bytes; error 201 under
  `{$R+}`); a constant 65536 is error 76. This compiler's `GetMem` takes a LongInt (a
  deviation that costs nothing, but a Buffer must not rely on it).
- An array type of 65535 bytes is accepted (the Guide says 65520), 65536 is error 22; global
  data above 65535 bytes is error 96. The buffer is therefore a pointer to
  `array[0..65520] of Char` allocated with `GetMem(P, 65521)`: capacity 65521.
- `Integer` wraps at 32768 (`32767 + 1 = -32768`, `20000 + 20000 = -25536` even into a
  LongInt, `Integer := Word 40000` is -25536). `Word` wraps silently at 65536 and 0
  (`65535 + 1 = 0`, also when stored in a LongInt, `3 - 5 = 65534`); `Word := LongInt`
  truncates. Comparison of Word with Integer is done in LongInt. `for` over 65530..65535
  with a Word runs 6 times. A LongInt index, and `LongInt - Word`, address bytes above 32767
  correctly. This compiler gives the same output for all of these.
**Recommendation.** `Buffer` uses `LongInt` for every position, size and index (gap start,
gap end, text length, cursor offset, line starts), `Integer` only for screen coordinates and
small counts, never `Word` for arithmetic (a decrement at 0 gives 65535 silently), and a
capacity of 65521 (`MaxText = 65521`, which is also all TP can allocate in one block). Gap shifting without `Move` is a `for` loop over
`Char`; ask for a `Move` in System if the loop shows in a profile.

## 45. Margin transfer to host glue (accepts 43)

The Host glue component goes from 500 to 600 lines; the plan's unallocated margin goes from
500 to 400 (components now sum to 14,600 under the unchanged 15,000 total). Plan §1 makes
the margin available from Phase 7. Reason: moving the host runtime into host/ (decision
36) for one tested implementation cost ~180 lines the plan did not foresee, and the IDE
needs compile, run handoff and persistence (~130 lines measured from working drafts).

## 46. Browser console ReadLn without Crt: a Pascal line editor in System

In the browser there is no stdin, so `readline` returns -2 ("no line input") there; System
then reads the line from keys (`readkey`) with a small Pascal line editor that echoes
through the console writer (Backspace, Enter), the same technique as Crt's `EditLine`.
The Node runner keeps `readline` from `NAME.in`. Reason: host glue is at 599 / 600; the
editing logic belongs with the console code in Pascal, which the runtime budget can carry.

## 47. IDE View: whole-window repaint, minimal scrolling, tabs as screen columns

`View.Paint` repaints the whole window (25 writes) every time; nothing tracks what changed.
It first scrolls as little as needed so the cursor is inside the 23-line, 80-column edit
area. The Buffer keeps a tab as `#9`; the View shows spaces up to the next multiple of 8.
The Crt cursor and horizontal scrolling use screen columns, the status line shows the
Buffer column. Row 25 is written to column 79 only, because a character at column 80 of the
last row scrolls the screen; `Reset` colours the whole screen so that cell matches the bar.
Reason: simple and always correct; change tracking is the first step if the IDE feels slow.

## 48. IDE Menu: bar, pull-downs, Options placeholder (accepted, #78)

**Decision.** `Menu` draws the bar over View's placeholder (`Paint; DrawBar`) and `Run`
opens the pull-down for `Keys.MenuLetter` (F10 opens the first). The bar has no Edit title
(the plan lists no Edit command; Alt-E does nothing): File (New, Open F3, Save F2, Quit
Alt-X), Run (Run Ctrl-F9), Compile (Compile F9), Options (one placeholder item that shows
the message `No options yet` and returns `CmdNone`). Inside a menu: Up/Down and Left/Right
wrap, Enter chooses, Esc and F10 cancel, an item's first letter chooses it, every other key
is ignored. A pull-down is 20 columns wide, has no border, and closing or switching menus
repaints the whole screen through View (decision 47) instead of saving what it covered.
Titles and items are typed-constant record tables. `CmdNew = 7` lives in Menu; `Show` is
public so a test can look at an open menu (a program blocked in `ReadKey` fails the corpus).
**Reason.** Smallest menu that covers the plan; no screen-saving code.

## 49. IDE Files: names, questions, messages, modified flag (accepted, #79)

**Decision.** `Files` keeps the current name (`NONAME.PAS` until the first Save). Typed names are
upper-cased and get `.PAS` when they contain no `.`. Prompts and questions are on the status line:
`Open file: `, `Save file as: ` (Enter accepts, Esc or an empty name cancels, 40 characters at
most) and `Save changes to NAME? (Y/N)` (Y saves and goes on only if the save worked, N discards,
Esc cancels, other keys ignored). Open asks for the name first, then the question when the text is
modified, then loads. Messages: `File not found` (missing file or path, text and name unchanged),
`File too large` (the Buffer is left empty and the name reset, so a Save cannot overwrite the old
file with nothing), `Cannot read file`, `Saved NAME`, `Cannot write file` (name and modified state
unchanged). Save always writes, also when unmodified. New asks the question when modified, then
clears. `Buffer.Modified` is an exact flag set by every edit and cleared by Clear, Load and Save
(`SetModified` for the start of a session). The status-line line editor is `View.ReadPrompt`,
shared by Keys (find) and Files. Main calls `ResetKeys` after New and Open so block marks do not
point into replaced text.
**Reason.** The exact flag is simpler than a text fingerprint; one prompt editor, in View.
**Rejected.** Hash fingerprint in Files (not exact); a copy of the prompt editor in Files.

## 50. Scope calls from the SWAG sweep (#83)

Typed and untyped files (`file of T`, `file`, `BlockRead`, `BlockWrite`) stay omitted as
plan §2 says, although they are the largest single reason SWAG programs fail (53 of the
portable ones); the budget is locked and the plan's target does not include them. DOS device
names (`COM1`, `LPT1`, `PRN`, `AUX`) are ordinary file names here (D35). Uninitialised
variables read as zero here and as memory garbage in TP (D36); tests never depend on them.
The remaining findings are Phase 8 issues: harmless compiler switches and `{$M}`, `Random`,
`FillChar`/`Move`, `Exp`/`Sin`/`Cos`/`Ln`/`ArcTan`/`Pi`/`MaxInt`, Dos additions, `Val` into
`Word` (D18 retired), large hex literals, Crt `LastMode`, `SetTextBuf`, `Assign(f, '')` as
the console, error 11 for long lines, and heap exhaustion ending in 203.

## 51. Rebalance: 400 lines from codegen to the checker

User direction: the checker may borrow from components with spare lines. Checker 2,200 to
2,600; codegen 2,500 to 2,100 (it used 1,585). The checker + codegen pair stays at 4,700 and
the total at 15,000. The plan's checkpoint (pair at most 5,200) is unchanged. Readability
still comes first (decision 31): the extra room is for Phase 8's compatibility work, not
for denser code elsewhere.

## 52. Rebalance: 200 lines from the IDE to the Pascal runtime

Under the user's borrowing rule (decision 51): the IDE is feature-complete at 1,378 lines and
its plan checkpoint stops features at 2,200, so its limit goes from 2,500 to 2,300; the
System/Crt/Dos runtime (1,396 of 1,500 after the Dos additions, with `Random` and the
empty-name `Assign` still to come) goes from 1,500 to 1,700. Total unchanged at 15,000.
