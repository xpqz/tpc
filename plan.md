# TP5-WASM: Implementation Plan

A Turbo Pascal 5.0 re-implementation in Rust, targeting WebAssembly. The runtime library and IDE are written in Pascal and compiled by the compiler itself.

## 1. Goals, constraints, budgets

**Goals**
- Compile a defined TP5.0 subset to a single WASM module, in the browser.
- Compiler, runtime and IDE total at most **15,000 non-blank, non-comment lines**.
- No third-party crates in the compiler (`[dependencies]` stays empty).

**Provisional size targets (revise once, after Phase 4)**

| Metric | Target |
|---|---|
| Compiler WASM, stripped | 250 KB |
| Hello-world output WASM | 20 KB |
| IDE WASM | 120 KB |
| Source LOC, all languages | 15,000 |

**LOC budget**

| Component | Lang | LOC |
|---|---|---|
| WASM binary emitter | Rust | 350 |
| Lexer | Rust | 450 |
| AST + parser | Rust | 2,000 |
| Types, symbols, frame layout | Rust | 1,400 |
| Checker (annotates AST, inserts coercions) | Rust | 2,200 |
| Codegen | Rust | 2,500 |
| Units, directives, driver, diagnostics, native CLI | Rust | 1,100 |
| **Compiler subtotal** | | **10,000** |
| System, Crt, Dos subset | Pascal | 1,500 |
| IDE | Pascal | 2,500 |
| Host glue (workers, terminal, VFS, real formatting) | JS | 500 |
| **Total** | | **14,500** |

The 500-line margin is the only slack. Do not spend it before Phase 7.

**Counting rule.** Run `rustfmt` with `max_width = 100`. Count non-blank lines that are not pure comments. Use the same rule for Pascal and JS (`tools/loc.sh`, about 20 lines). CI fails over budget, per component and in total.

## 2. Language definition

**Reference:** Turbo Pascal 5.0 semantics. TP5 has units, procedural types, typed constants, `Mark`/`Release`, and no `object`. Verify specific behaviours against the TP5 manual and Free Pascal `-Mtp`, and record every deviation in `docs/deviations.md`.

**Supported**
- Types: `Byte`, `ShortInt`, `Integer` (16-bit), `Word`, `LongInt`, `Boolean`, `Char`, `Real`, `String[n]`, `String`, enums, subranges, arrays, records (including variant parts), sets (base type of at most 256 values), pointers, procedural types.
- Statements: `begin/end`, `if`, `case`, `while`, `repeat`, `for`, `with`, assignment, procedure call, `Exit`, `Halt`.
- Declarations: `const`, typed `const`, `type`, `var`, nested procedures and functions, `forward`, units (`interface`/`implementation`/`initialization`).
- Text files only: `Assign`, `Reset`, `Rewrite`, `Append`, `Close`, `Read`, `ReadLn`, `Write`, `WriteLn`, `Eof`, `Eoln`, `IOResult`, `{$I+/-}`.
- Directives: `{$R}`, `{$I file}` (include), `{$IFDEF}`/`{$IFNDEF}`/`{$ELSE}`/`{$ENDIF}`/`{$DEFINE}`.

**Deliberately omitted (each yields a specific error, never silent misbehaviour)**
`goto` and labels, inline assembler, `Absolute`, `inline`, interrupt procedures, overlays, BGI, 8087/`Single`/`Double`/`Extended`/`Comp`, typed and untyped files, `Seg`/`Ofs`/`Ptr`, binary `.TPU`, `Chain`/`Execute`.

**Deviations accepted up front**
- `Real` is IEEE f64 (8 bytes), not the 6-byte software real.
- Pointers are 32-bit offsets into linear memory. `nil` is 0.
- `Integer` stays 16-bit; `LongInt` is 32-bit.
- Assigning a nested routine to a procedural variable is a compile error, as in TP.

## 3. Architecture

```
 Browser main thread: terminal render + key events, owns SharedArrayBuffers
        |                                   |
 IDE Worker                            Program Worker
 (IDE.wasm, written in Pascal)         (user program wasm)
   |  host.compile(...)                   ^ blocks on Atomics.wait for keys
   v                                      |
 compiler.wasm (Rust) --- wasm bytes ---> terminate Worker = stop program
```

- **Whole-program compilation.** Parse the main program and every unit it uses, in dependency order, with units resolved from the VFS source. Emit one module. No linker, no object files.
- **Roots for dead-code elimination:** program body, unit initializers, every routine whose address is taken (conservatively, every routine with a compatible signature that is address-taken), and routines the codegen calls implicitly (string, set and heap helpers).
- **Compiler API** (exported from `compiler.wasm`):
  - `alloc(n) -> ptr`
  - `compile(src_ptr, src_len, flags) -> status`
  - `out_ptr()`, `out_len()` (the module bytes)
  - `err_line()`, `err_col()`, `err_code()`
- **Native CLI.** The same crate builds as a native binary: `tp5c in.pas -o out.wasm`. This bootstraps the IDE build and drives all tests through Node.
- **Cross-origin isolation.** `SharedArrayBuffer` requires COOP/COEP headers. Document the required headers in `README` and ship a 15-line dev server that sets them.

## 4. ABI (write this as `docs/abi.md` before Phase 2)

### 4.1 Memory map

| Range | Use |
|---|---|
| `0 .. 255` | Reserved (null guard, scratch) |
| `256 ..` | Static data: typed constants, string literals, globals |
| after static | Heap, grows up (`HeapPtr`, `HeapEnd`) |
| top of memory | Shadow stack, grows down |

WASM global 0 is `sp` (mutable i32). Memory is 1 MB to start. The runtime may call `memory.grow` when the heap is exhausted, and runtime error 203 is raised when growth fails.

### 4.2 Type layout

| Type | Size | WASM repr. |
|---|---|---|
| `Byte`, `Boolean`, `Char`, `ShortInt` | 1 | i32 |
| `Integer`, `Word` | 2 | i32 |
| `LongInt`, pointers, procedural | 4 | i32 |
| `Real` | 8 | f64 |
| enum / subrange | smallest of 1, 2, 4 that fits | i32 |
| `String[n]` | n+1: byte 0 = length, then chars | by address |
| `set of T` | always 32 bytes, bit i = element i | by address |
| record | fields in order, no padding (TP is packed); variants overlay | by address |
| array | row-major, no padding | by address |

Sub-word integers are sign-extended (`Integer`, `ShortInt`) or zero-extended (`Word`, `Byte`) on load and truncated on store.

### 4.3 Storage classes

| Variable | Location |
|---|---|
| Uncaptured, non-address-taken scalar local | WASM local |
| Captured by a nested routine, address-taken (`var` arg or `@`), or aggregate | Frame in linear memory |
| Global | Static data |
| Value parameter, scalar | WASM param, copied to frame if the above applies |
| Value parameter, aggregate | Caller passes address of source, **callee copies** into its frame |
| `var` parameter | i32 address |

The checker computes a `captured`/`addr_taken` flag per variable. Codegen reads the flags. A flag set late (because a nested routine appears after the variable is used) is why the checker must complete a full routine before codegen starts.

### 4.4 Frames and static links

```
prologue:  fp = sp - framesize ; sp = fp
           store static_link at [fp+0]           (routines that have a parent)
epilogue:  sp = fp + framesize
```

- Frame offset 0 holds the static link (4 bytes). Other frame slots follow.
- Access to a variable at lexical depth difference *d*: start at `fp`, load the static link *d* times, add the slot offset.
- Parameter order: `[static_link] [result_ptr] declared params...`. A static link is passed only to routines that have an enclosing routine.
- Return: scalars use the WASM result. Aggregate-returning functions (strings, sets) take a hidden `result_ptr` that the caller supplies.
- `Exit` is `br` to an outer block around the body, so the epilogue always runs. `Halt` calls `host.halt`.
- Every routine's body is wrapped as `block $exit ... end`, followed by the epilogue.

### 4.5 Procedural values

A procedural value is a WASM table index (i32). Nested routines may not be assigned to procedural variables (compile error). The checker interns one WASM function type per signature, and calls go through `call_indirect` with that type. Index 0 of the table is a trap stub so that `nil` calls fail cleanly.

### 4.6 Integer semantics

- Operands are promoted per TP rules. Arithmetic is done in i32.
- After `+`, `-`, `*` and unary `-` whose result type is 16-bit or narrower, re-narrow: `i32.shl 16; i32.shr_s 16` for `Integer`, `and 0xFFFF` for `Word`, `and 0xFF` for `Byte`, and the 8-bit signed shift pair for `ShortInt`.
- Comparisons, `div`, `mod`, `and`, `or` and `xor` over already-normalized operands need no narrowing.
- Under `{$R+}`, assignment to a subrange or narrower integer type calls a range check that raises runtime error 201.
- **Verify the promotion rules** (for example `Byte + Byte` result type) with differential tests before codegen is considered done.

### 4.7 Strings and sets

- Assignment to `String[n]` copies `min(len, n)` bytes and sets the length byte.
- Concatenation, `Copy`, `Pos`, `Delete`, `Insert`, comparisons: Pascal helpers in `System`.
- `Char` used where `String` is expected: the checker inserts a `CharToStr` coercion node.
- Set constructors lower to a sequence of bit sets into a temp. `in`, `+`, `*`, `-`, `=`, `<=` call `System` helpers (32-byte loops).

### 4.9 Host imports (module `host`)

| Import | Signature | Purpose |
|---|---|---|
| `write` | `(fd, ptr, len)` | console / terminal output |
| `readkey` | `() -> i32` | blocks (`Atomics.wait`) |
| `keypressed` | `() -> i32` | non-blocking |
| `readline` | `(ptr, max) -> len` | blocks, line editing in JS |
| `screen` | `(op, a, b, c)` | gotoxy, clrscr, textattr, cursor |
| `fs_open` | `(namePtr, nameLen, mode) -> h` | VFS, text only |
| `fs_read` / `fs_write` / `fs_close` | `(h, ptr, len) -> n` | VFS |
| `fmt_real` | `(f64, width, decimals, bufPtr) -> len` | TP-style real formatting |
| `halt` | `(code)` | terminate with exit code |
| `rterror` | `(code)` | runtime error report |
| `time` | `() -> i32` | ticks for `Delay`, `GetTime` |
| `compile` | IDE worker only | invokes `compiler.wasm` |

### 4.10 Runtime errors

Report `Runtime error N`. Codes in use: 2 and 3 (file not found / path), 100 (read past EOF), 106 (invalid numeric format), 200 (divide by zero), 201 (range check), 203 (heap overflow), 204 (invalid pointer operation). Source addresses are not supported. If a runtime error line number is wanted, add `{$D+}`, which stores the current line in a global at each statement. It is opt-in because it costs size.

## 5. Phases

Each phase has deliverables, a LOC budget and an **exit test**. Do not start the next phase until the exit test passes in CI.

### Phase 0: Foundations (about 350 Rust, 40 JS)

**Deliverables**
1. Repo layout: `compiler/`, `runtime/` (Pascal), `ide/` (Pascal), `host/` (JS), `tests/`, `tools/`, `docs/`.
2. `tools/loc.sh` and the CI budget check.
3. WASM emitter: LEB128 (signed and unsigned), section writers (type, import, function, table, memory, global, export, element, code, data), an expression builder with block depth tracking.
4. `docs/abi.md` from section 4 above, plus `docs/subset.md` from section 2.
5. Node test runner: loads a `.wasm`, supplies `host.write`, compares stdout with `expected/*.out`.

**Exit test:** a Rust unit test builds a module that imports `host.write` and prints "OK"; it validates and runs in Node.

### Phase 1: Scalars and control flow (about 3,800 Rust)

**Deliverables**
1. Lexer: identifiers, numbers (decimal, `$hex`, reals), strings, `{ }` / `(* *)` comments, directive comments, keywords.
2. Parser and AST for: program, `const`, `var`, `if`, `while`, `repeat`, `for`, `case`, expressions, procedures and functions with value parameters. AST nodes live in `Vec`s and are referenced by `u32` index.
3. Checker v1: scalar types, constants, scopes, promotion rules, narrowing nodes.
4. Codegen v1: WASM locals for scalars, 16/32-bit arithmetic with narrowing, structured control flow, direct calls.
5. `Write`/`WriteLn` for integers, chars, booleans and string literals, via compiler intrinsics calling `host.write`.
6. Diagnostics: a static error table keyed by TP error numbers, with line and column.

**Exit tests:** sieve, Fibonacci, Collatz, Towers of Hanoi, a 16-bit overflow wrap test, and `case` with ranges produce output identical to Free Pascal `-Mtp`. Negative tests: 15 invalid programs (undeclared identifier, type mismatch, missing `end`, and so on) produce the expected error number and line.

### Phase 2: Memory model and calling convention (about 1,200 Rust)

**Deliverables**
1. Shadow stack, `sp` global, prologue/epilogue.
2. Checker flags (`captured`, `addr_taken`) and frame layout.
3. `var` parameters, nested procedures and functions with static links, `Exit`, recursion.
4. Procedural types with `call_indirect`, signature interning, the nested-routine assignment error.

**Exit tests (ABI suite, `tests/abi/`):** deep recursion restores `sp` exactly; a three-level nested procedure modifies an outer local; `var` parameter aliasing; `Exit` from nested loops with epilogue verification; indirect call through a procedural variable; a function returning through the hidden result pointer. Each test asserts `sp` at program end equals its initial value.

### Phase 3: Aggregates, strings, minimal runtime (about 1,500 Rust, about 500 Pascal)

**Deliverables**
1. Arrays (including multidimensional), records (including variants), subranges, enums.
2. `String[n]` and `String`: layout, assignment, indexing, comparison.
3. `Char` to `String` coercion; string concatenation and the string intrinsics `Length`, `Copy`, `Pos`, `Delete`, `Insert`, `Concat`.
4. **Minimal `System` unit in Pascal**, compiled by this compiler: bump allocator, `New`/`GetMem`, `MemMove`, string helpers. Whole-program compilation and function-level DCE land here.
5. Value-aggregate parameters (callee copy) and aggregate assignment (memcpy).

**Exit tests:** string manipulation programs; an array and record test set; N-queens; hello-world module size is measured and recorded as the first data point in `docs/size.md`.

**Checkpoint:** if total Rust LOC exceeds 6,500 at this point, stop and review the checker before continuing.

### Phase 4: Full System unit, sets, pointers (about 1,000 Rust, about 600 Pascal)

**Deliverables**
1. Pointers, `New`, `Dispose`, `@`, `^`, `nil`, linked structures.
2. Heap: bump allocation plus a free list; `Mark`/`Release` resets the bump pointer and discards free-list entries above the mark; `MemAvail`/`MaxAvail`.
3. Sets: constructors, `in`, `+`, `*`, `-`, comparisons, 32-byte layout, `Include`/`Exclude`.
4. Remaining intrinsics: `Ord`, `Chr`, `Succ`, `Pred`, `Odd`, `Abs`, `Sqr`, `Sqrt`, `Round`, `Trunc`, `Inc`, `Dec`, `Val`, `Str`, `UpCase`, `ParamStr`/`ParamCount` (empty).
5. Real arithmetic and `Write(x:w:d)` via `host.fmt_real`.
6. Runtime errors 200, 201, 203, 204.

**Exit tests:** linked list, binary tree, a program stressing `New`/`Dispose`/`Mark`/`Release`; set tests (sieve with sets, character classification); real-number formatting matched against Free Pascal for 50 values (tolerating documented deviations only).

**Checkpoint:** measure combined size of Rust checker and codegen. The hard ceiling is 5,200 for the pair. If you are over, cut features before any phase 5 work.

### Phase 5: Units and text files (about 700 Rust, about 300 Pascal)

**Deliverables**
1. `uses`, `interface`/`implementation`, initialization sections, unit dependency ordering and cycle detection, whole-program emission.
2. VFS-backed unit resolution (`NAME.PAS` lookup).
3. Text files in `System`: `Assign`, `Reset`, `Rewrite`, `Append`, `Close`, `Read`, `ReadLn`, `Write`, `WriteLn`, `Eof`, `Eoln`, `IOResult`, `{$I+/-}`; `Input`/`Output` bound to the host console.
4. `{$I file}`, conditional compilation directives.
5. Typed constants (arrays and records).
6. `Dos` subset: `GetDate`, `GetTime`, `FSearch`-free.

**Exit tests:** a two-unit program with cross-unit procedural types; a report generator reading a VFS file and writing another; an error-handling test using `{$I-}`/`IOResult` for a missing file.

### Phase 6: Crt and the browser terminal (about 100 Rust, about 300 Pascal, about 350 JS)

**Deliverables**
1. Main-thread terminal: an 80x25 cell grid on `<canvas>`, CP437-style glyphs, 16 colours, blinking cursor.
2. Two-Worker setup, the `SharedArrayBuffer` key queue, `Atomics.wait`-based `readkey`.
3. `Crt` unit: `ClrScr`, `GotoXY`, `WhereX`, `WhereY`, `TextColor`, `TextBackground`, `ReadKey`, `KeyPressed`, `Delay`, `Window`, `ClrEol`, `Sound`/`NoSound` as no-ops.
4. Stop button terminates the program Worker.

**Exit tests:** a Snake game runs interactively; an infinite-loop program is stopped and the page remains responsive; a keyboard-latency check (under 50 ms per key in Chrome and Firefox).

### Phase 7: IDE (about 2,500 Pascal, about 100 JS)

**Scope**
- One buffer, gap buffer implementation, up to 64 KB.
- Cursor movement, page up/down, home/end, insert/overwrite, delete line (`^Y`), block copy/paste (`^K B`, `^K K`, `^K C`, `^K V`) restricted to one block, find (`^Q F`), auto-indent.
- Menu bar (File: New, Open, Save, Quit; Run: Compile, Run; Options).
- Status line with the line and column indicator.
- Compile (`F9`) and Run (`^F9`); error display with cursor jumped to the error position; run output in the program Worker's terminal.
- Load and save to the VFS (`localStorage` or IndexedDB-backed, so it persists).

**Out of scope:** multiple windows, mouse, debugger, watches, syntax highlighting, project management.

**Process.** Write IDE modules in this order so each can be tested on its own: `Buffer` (gap buffer, pure logic, tested under Node via the CLI-built wasm), `View` (screen painting), `Keys` (dispatch), `Menu`, `Files`, `Main`.

**Exit test:** inside the IDE, open Snake, edit a line, compile, run, introduce a syntax error, see the jump to it, fix it, run again.

**Checkpoint:** if IDE exceeds 2,200 lines at feature-complete, stop adding features.

### Phase 8: Compatibility, diagnostics, size (about 300 Rust)

**Deliverables**
1. Fill in the full TP error-number table for supported constructs. Unsupported features report a distinct "not supported" error.
2. Runtime error output format `Runtime error N`.
3. Directive edge cases; `{$I}` include path handling.
4. Size pass: profile with `twiggy` or a hand-written section size dump; target the numbers in section 1.
5. Remove dead helper functions the DCE still retains; shrink the name section (or omit it in release).
6. Documentation: `README`, `docs/subset.md`, `docs/deviations.md`.

**Exit test:** the full differential suite passes, sizes are within the provisional targets (or the targets are revised once, explicitly, in `docs/size.md`), and the LOC check is green.

## 6. Testing strategy

| Layer | What | Where |
|---|---|---|
| Semantics | Small programs with `.pas` and `.out`; one feature per program | `tests/lang/` |
| Differential | Same programs through Free Pascal `-Mtp`; deviations recorded | `tests/diff/` |
| ABI | `sp` restoration, static links, `var` aliasing, indirect calls, result pointers | `tests/abi/` |
| Negative | Invalid programs expect a specific error number and line; **accepting an invalid program is a failure** | `tests/neg/` |
| Runtime errors | Programs that must halt with a given runtime error code | `tests/rt/` |
| Host | Worker interruption, key queue, VFS persistence | `tests/host/` (Playwright, optional) |
| Self-hosting | IDE compiles under the CLI and its unit tests pass | `tests/ide/` |

Run everything with `make test`. All except `host/` run in Node with no browser.

**Conformance:** run an ISO 7185 test program such as Scott Moore's `iso7185pat.pas` on the standard-Pascal subset, if it is still available. Treat failures on omitted features as expected and record them.

## 7. Risks

| # | Risk | Mitigation |
|---|---|---|
| 1 | Checker and codegen exceed their combined 4,700 lines | Checkpoints at phases 3 and 4; annotate the AST with coercion nodes rather than adding an IR; cut features, not the cap |
| 2 | ABI redesign after codegen exists | ABI doc is a phase 0 deliverable; the ABI suite in phase 2 must pass before aggregates |
| 3 | Compiler bugs break the Pascal runtime or IDE | Keep `tests/lang` as the regression gate; build the minimal runtime in phase 3 |
| 4 | Integer promotion semantics wrong | Differential tests on mixed-width arithmetic before phase 1 exits |
| 5 | Text file semantics larger than budget | Text only; `Read` of numbers implemented once in `System` |
| 6 | COOP/COEP hosting limits | Document headers; ship the dev server; a fallback to a non-blocking `ReadKey` mode is out of scope |
| 7 | IDE scope creep | Fixed feature list in phase 7; checkpoint at 2,200 lines |

## 8. Conventions

- **Rust:** `#![forbid(unsafe_code)]` except in the two-line FFI exports. `panic = "abort"`, `opt-level = "z"`, `lto = true`, `codegen-units = 1`. No `format!` or `std::fmt` in the compiler; diagnostics are built from static tables and a small integer-to-string routine.
- **AST:** `Vec<Node>` arenas, `u32` handles, no `Box` or `Rc`.
- **No `wasm-bindgen`.** Raw `extern "C"` exports and a manually managed buffer.
- **Pascal:** the runtime uses only constructs already covered by completed phases; do not add a compiler feature just to make the runtime prettier.
- **Commits:** every phase ends with a tagged commit, `docs/size.md` updated with the three tracked metrics (source LOC, compiler WASM size, hello-world WASM size).

## 9. Suggested first week

1. Day 1: repo, `tools/loc.sh`, CI, the Node runner.
2. Day 2: emitter and the hardcoded-module exit test.
3. Days 3 and 4: write `docs/abi.md` and `docs/subset.md`. Review them against section 4, then freeze.
4. Day 5: lexer and the first 30 tests in `tests/lang` (write the tests before the code).
