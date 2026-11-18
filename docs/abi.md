# ABI

Source: `plan.md` §4. This document is a transcription and organisation of that section;
where the plan is silent the gap is listed in [Open questions](#open-questions), not
filled. Section 4.8 does not exist in the plan and is not defined here.

## 1. Memory map

| Range | Use |
|---|---|
| `0 .. 255` | Reserved (null guard, scratch) |
| `256 ..` | Static data: typed constants, string literals, globals |
| after static | Heap, grows up (`HeapPtr`, `HeapEnd`) |
| top of memory | Shadow stack, grows down |

- WASM global 0 is `sp` (mutable i32).
- Memory is 1 MB to start.
- The runtime may call `memory.grow` when the heap is exhausted. Runtime error 203 is
  raised when growth fails.
- `nil` is 0; pointers are 32-bit offsets into linear memory.

## 2. Type layout

| Type | Size (bytes) | WASM repr. |
|---|---|---|
| `Byte`, `Boolean`, `Char`, `ShortInt` | 1 | i32 |
| `Integer`, `Word` | 2 | i32 |
| `LongInt`, pointers, procedural | 4 | i32 |
| `Real` | 8 (IEEE f64) | f64 |
| enum / subrange | smallest of 1, 2, 4 that fits | i32 |
| `String[n]` | n+1: byte 0 = length, then chars | by address |
| `set of T` | always 32, bit i = element i | by address |
| record | fields in order, no padding (TP is packed); variants overlay | by address |
| array | row-major, no padding | by address |

Load and store of sub-word integers:

| Type | On load | On store |
|---|---|---|
| `Integer`, `ShortInt` | sign-extend | truncate |
| `Word`, `Byte` | zero-extend | truncate |

## 3. Storage classes

| Variable | Location |
|---|---|
| Uncaptured, non-address-taken scalar local | WASM local |
| Captured by a nested routine, address-taken (`var` arg or `@`), or aggregate | Frame in linear memory |
| Global | Static data |
| Value parameter, scalar | WASM param, copied to frame if the rows above apply |
| Value parameter, aggregate | Caller passes address of source; **callee copies** into its frame |
| `var` parameter | i32 address |

- The checker computes a `captured` / `addr_taken` flag per variable; codegen reads them.
- A flag can be set late (a nested routine appearing after the variable's first use), so
  the checker must finish a whole routine before codegen starts on it.

## 4. Frames and static links

```
prologue:  fp = sp - framesize ; sp = fp
           store static_link at [fp+0]           (routines that have a parent)
epilogue:  sp = fp + framesize
```

- Frame offset 0 holds the static link (4 bytes). Other frame slots follow.
- Variable at lexical depth difference *d*: start at `fp`, load the static link *d* times,
  add the slot offset.
- Parameter order: `[static_link] [result_ptr] declared params...`.
- A static link is passed only to routines that have an enclosing routine.
- Return: scalars use the WASM result. Aggregate-returning functions (strings, sets) take a
  hidden `result_ptr` that the caller supplies.
- Every routine body is wrapped as `block $exit ... end` followed by the epilogue.
- `Exit` is `br` to that outer block, so the epilogue always runs.
- `Halt` calls `host.halt`.
- Invariant (ABI suite): `sp` at program end equals its initial value.

## 5. Procedural values

- A procedural value is a WASM table index (i32).
- Nested routines may not be assigned to procedural variables (compile error).
- The checker interns one WASM function type per signature; calls use `call_indirect` with
  that type.
- Table index 0 is a trap stub, so a `nil` call fails cleanly.
- DCE roots include every address-taken routine (conservatively, every routine with a
  compatible signature that is address-taken); see `plan.md` §3.

## 6. Integer semantics

- Operands are promoted per TP rules. Arithmetic is done in i32.
- After `+`, `-`, `*` and unary `-` whose result type is 16-bit or narrower, re-narrow:

| Result type | Re-narrow |
|---|---|
| `Integer` | `i32.shl 16; i32.shr_s 16` |
| `Word` | `and 0xFFFF` |
| `Byte` | `and 0xFF` |
| `ShortInt` | `shl 24; shr_s 24` (the 8-bit signed pair) |

- Comparisons, `div`, `mod`, `and`, `or`, `xor` over already-normalised operands need no
  narrowing.
- Under `{$R+}`, assignment to a subrange or narrower integer type calls a range check that
  raises runtime error 201.
- The promotion rules are to be verified with differential tests before codegen is
  considered done; see the measurements in Open questions 1 to 3.

## 7. Strings and sets

- Assignment to `String[n]` copies `min(len, n)` bytes and sets the length byte.
- Concatenation, `Copy`, `Pos`, `Delete`, `Insert`, comparisons: Pascal helpers in `System`.
- `Char` where `String` is expected: the checker inserts a `CharToStr` coercion node.
- Set constructors lower to a sequence of bit sets into a temp.
- `in`, `+`, `*`, `-`, `=`, `<=` call `System` helpers (32-byte loops).

## 8. Host imports (module `host`)

(Plan §4.9.)

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

Compiler API exported from `compiler.wasm` (`plan.md` §3): `alloc(n) -> ptr`,
`compile(src_ptr, src_len, flags) -> status`, `out_ptr()`, `out_len()`, `err_line()`,
`err_col()`, `err_code()`.

### Test runner contract (`tests/run.mjs`)

- The module exports `main` (`() -> ()`) and `memory`; both are required.
- `write(fd, ptr, len)`: fd 1 is stdout, fd 2 is stderr, other fds are a runner error. Bytes are
  copied raw from `memory`; no text decoding.
- `halt(code)` stops execution; the exit code is `code`.
- `rterror(code)` writes `Runtime error N` and LF to stdout (no address), then behaves as
  `halt(code)`. fpc writes to stderr; TP5 writes to the console.
- Exit codes: program code when output matches (or none is expected), 1 on output mismatch,
  2 on runner error (stderr prefix `run.mjs:`). A program `halt(2)` is indistinguishable by code.
- Imports other than `write`, `halt` and `rterror` are stubs that fail with
  `host import X not available in test runner`; unknown imports fail instantiation.

## 9. Runtime errors

(Plan §4.10.) Report `Runtime error N`.

| Code | Meaning |
|---|---|
| 2, 3 | file not found / path not found |
| 100 | read past EOF |
| 106 | invalid numeric format |
| 200 | divide by zero |
| 201 | range check |
| 203 | heap overflow |
| 204 | invalid pointer operation |

- Source addresses are not supported.
- `{$D+}` is opt-in: it stores the current line in a global at each statement (costs size).

## Open questions

Resolved: 1, 2, 4 and 5 by decision 5 (docs/decisions.md); 9 and 11 (initial `sp`,
static start) by decision 6; 12 by the test runner contract in §8.

Measurements use Free Pascal 3.2.2 (aarch64, macOS) with `fpc -Mtp -v0`, programs in
`tmp/` (git-ignored; reproduced in `docs/prs/14.md`).

1. **Intermediate expression type is not observable through fpc.** fpc evaluates 16-bit
   operands in native-width arithmetic and truncates only on assignment. `sizeof(b+b)` is 8
   for `b: Byte`. Measured with `Byte=200/100`, `Integer=30000`, `Word=60000`:
   `b+b2` as `LongInt` is 300; `i+i2` as `LongInt` is 60000; `(i+i2) div 2` is 30000;
   `i+i2 > 0` is TRUE; `(w+w) div 2` is 60000; `Write(i+i2)` prints 60000;
   `i := i+i2` gives -5536; `b := b+b2` gives 44. So fpc treats `Integer+Integer` as
   wider than 16 bit, whereas §6 re-narrows to `Integer` whenever "the result type is 16-bit
   or narrower". The plan does not say what the TP result type is for each operand pair
   (`Byte+Byte`, `ShortInt+ShortInt`, `Word+Word`, `Word+Integer`, `Integer*Integer`).
   fpc cannot be the oracle for expressions whose value is used without an intervening
   assignment; the TP5 manual must decide. **Needs a decision** before codegen:
   (a) TP rule (what is the TP5 result-type table?) with differential tests restricted to
   assignment-truncated cases, or (b) fpc's wider behaviour recorded as a deviation.
2. **`Word` / `Byte` unary minus and mixed signed/unsigned.** fpc gives `-w` = -60000 and
   `-b` = -200 (as `LongInt`). §6 lists unary `-` among re-narrowed operations for 16-bit
   results; the result type of `-Word` is unspecified. Also unspecified: `Integer` compared
   with `Word` (fpc: `-1 < 65535` TRUE, so comparison is done in a wider signed type).
3. **Assignment-truncating behaviour confirmed (no `{$R+}`).** `i := 32767; i := i+1` gives
   -32768; `i := -32768; i := i-1` gives 32767; `-minint` and `abs(minint)` give -32768;
   `i := 200; i := i*i` gives -25536; `LongInt` 65535 to `Integer` gives -1; 70000 to `Word`
   gives 4464; 300 to `Byte` gives 44; `LongInt` `2147483647+1` gives -2147483648;
   `100000*100000` gives 1410065408. `div` and `mod` truncate toward zero
   (`-7 div 2` = -3, `-7 mod 2` = -1). These match §6 narrowing. With `{$R+}` fpc raises
   runtime error 201 on `LongInt` to `Byte` and `LongInt` to subrange assignment.
4. **Shifts and `not`.** fpc: `b shl 7` (b=3) = 384, `w shl 1` (w=40000) = 80000,
   `i shl 15` (i=1) = 32768 (all wide, not narrowed); `not b` with b=255 prints 0, `not w`
   with w=0 prints 65535, `not i` with i=0 prints -1. §6 does not list `shl`, `shr`, `not`
   under re-narrowing. Decision needed: narrow results of `shl` / `not`?
5. **Enum / subrange storage size, measured.** `(a,b,c)` is 1 byte; an enum of 257 values is
   2 bytes. Subranges: `-1..1` 1; `-128..127` 1; `-129..127` 2; `-1..200` 2; `0..300` 2;
   `-129..0` 2; `0..65535` 2; `-1..40000` 4. "Smallest of 1, 2, 4 that fits" in §2 is
   consistent, but the plan does not say whether a non-negative subrange up to 255 is
   unsigned (zero-extend) and a negative-lower-bound subrange signed. Assumed from the
   load/store row; needs confirmation. fpc has no 3-byte case, matching the plan.
6. **Set size is a deliberate deviation from fpc.** Plan: always 32 bytes. fpc:
   `set of 1..3`, `set of Boolean`, `set of (a,b,c)` are 1 byte each; `set of Byte` and
   `set of Char` are 32. `SizeOf` on a set will differ from fpc; tests must not print it.
   Recorded in `docs/deviations.md`.
7. **Record padding.** fpc `-Mtp` aligns plain records: `record a: Byte; b: Integer;
   c: LongInt end` is 8 bytes; with `packed` it is 7. Plan: no padding. Differential tests
   must declare records `packed` (or never print `SizeOf`). Recorded in deviations.
8. **Pointer size.** `SizeOf(Pointer)` is 8 under fpc on this host, 4 in the plan.
   Recorded in deviations; tests must not print it.
9. **`String` default capacity and layout.** fpc: `SizeOf(String)` = 256, `string[5]` = 6.
   Plan §2 lists `String` and `String[n]`; §4.2 gives only `String[n]`. Presumably
   `String` is `String[255]` (256 bytes); not stated.
10. **Aggregate function results and `result_ptr`.** §4.4 names strings and sets. Not stated:
    whether arrays and records returned by functions (if allowed by the subset) also use
    `result_ptr`; whether `result_ptr` is present before or after `static_link` when both
    apply (the order given is `[static_link] [result_ptr]`, taken as normative).
11. **Static data layout order and alignment** within `256 ..` (typed constants, literals,
    globals), and whether string literals are deduplicated: not specified. Initial `sp`,
    `HeapPtr` and `HeapEnd` values after static data: not specified (only "top of memory").
12. **Halt exit code and `rterror` flow.** §4.4 says `Halt` calls `host.halt`; §4.10 says
    report `Runtime error N` via `host.rterror`. Not stated: whether `rterror` is followed
    by `halt(N)` or itself terminates, and the exit code used.
13. **`host.write` `fd` values** (0/1/2?), `screen` `op` numbering, `fs_open` `mode`
    encoding, `host.time` unit, and error return conventions of `fs_*` (negative `n`?):
    unspecified.
14. **Table layout beyond index 0.** Whether the element section assigns indices in
    declaration order or by address-taken order; not specified.
15. **Heap helper contracts** (`HeapPtr`, `HeapEnd` as Pascal globals in `System` or WASM
    globals?). §4.1 names them in the memory map only; WASM global 0 is the only global
    defined.
16. **`{$R}` default.** Plan lists `{$R}` as supported but not whether the default is `+` or
    `-`. TP default is `{$R-}`; assumed, to be confirmed.
