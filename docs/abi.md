# ABI

What the compiler emits and what the host provides, as implemented. `plan.md` §4 is the
original design; where this document and the plan differ, this document is right and the
difference is a recorded decision (`docs/decisions.md`). `tests/tools/abi_imports_test.mjs`
(part of `make test`) fails when the import table in section 8 disagrees with the `external`
declarations in `runtime/*.pas` or the imports objects of `host/runtime.js` and
`host/ide.js`.

## 1. Memory map

| Range | Use |
|---|---|
| `0 .. 255` | Reserved (null guard) |
| `256 .. 279` | Fixed header: LF, blank, `FALSE`, `TRUE` and a 12-byte digit buffer, used by `write` |
| `280 ..` | Globals, then the string literals and typed constants the live code uses |
| after static | Heap, grows up (`HeapPtr`, limit 983,040) |
| top of memory | Shadow stack, grows down |

- WASM global 0 is `sp` (mutable i32), always exported as `sp`.
- The shadow stack may use the top 64 KB (`1,048,576 - 65,536` is the limit, TP5's maximum
  `{$M}` stack). A prologue that would put `fp` below the limit raises runtime error 202.
  A frame is the only thing that consumes shadow stack.
- Memory is 1 MB to start.
- The heap is fixed (decision 18): from the end of static data, rounded up to 8, to the stack
  limit (983,040); there is no `memory.grow`. Runtime error 203 is raised when a block does
  not fit. System holds `HeapOrg`, `HeapPtr` (bump pointer) and `FreeList` as Pascal globals;
  the compiler writes the heap origin into `HeapOrg` and `HeapPtr` (adjacent) in the data
  segment. Blocks are multiples of 8 bytes; a free block stores its size and the next free
  block's address.
- `nil` is 0; pointers are 32-bit offsets into linear memory.

## 2. Type layout

| Type | Size (bytes) | WASM repr. |
|---|---|---|
| `Byte`, `Boolean`, `Char`, `ShortInt` | 1 | i32 |
| `Integer`, `Word` | 2 | i32 |
| `LongInt`, pointers, procedural | 4 | i32 |
| `Real` | 8 (IEEE f64) | f64 |
| enum / subrange | smallest of `ShortInt`, `Byte`, `Integer`, `Word`, `LongInt` that holds both bounds (decision 16) | i32 |
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

- A routine that has an enclosing routine reserves frame offset 0..4 for the static link;
  other frame slots follow (they start at 0 in routines without a parent). A routine has a
  frame if it has address-taken or captured variables or nested routines.
- A captured `var` parameter keeps its address in a 4-byte frame slot (`FrameRef`).
- The function result is a variable of the function's activation; nested routines may assign
  it, so it lives in the frame when captured.
- Variable at lexical depth difference *d*: start at `fp`, load the static link *d* times,
  add the slot offset.
- Parameter order: `[static_link] [result_ptr] declared params...`.
- A static link is passed only to routines that have an enclosing routine.
- Return: scalars use the WASM result. Aggregate-returning functions (strings, sets) take a
  hidden `result_ptr` that the caller supplies.
- Every routine body is wrapped as `block $exit ... end` followed by the epilogue.
- `Exit` is `br` to that outer block, so the epilogue always runs.
- Frame layout: slots in declaration order (address-taken locals and value parameters passed
  as `var` arguments; value parameters are copied in after the prologue), no padding. The
  static link slot, when it exists, comes first and shifts the others by 4.
- `var` parameter: an i32 address held in its WASM local; reads and writes use the
  parameter type's width and sign.
- `Halt` calls `host.halt`.
- Invariant (ABI suite): `sp` at program end equals its initial value.

## 5. Procedural values

- A procedural value is a WASM table index (i32).
- Nested routines may not be assigned to procedural variables (compile error).
- The checker interns procedural types by content (parameter types, by-reference flags,
  result), so equal signatures are one type; the emitter interns one WASM function type per
  WASM shape; calls push the arguments, then the index, then `call_indirect` with that type.
- Only global routines can be assigned or passed (a nested routine is error 143). Table
  layout: index 0 is a trap stub (a function that executes `unreachable`, so calling an
  unassigned variable or `nil` traps), then the routines whose address is taken, in routine
  order. The table is always emitted.
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
- The promotion rules are TP 5.0's, measured with `tools/tp5.sh` (decisions 5 and 10): byte-sized
  operands widen to `Integer` (`Byte` with `Word` gives `Word`), `Integer` with `Word` is
  `LongInt`, unary minus gives `Integer` or `LongInt`, `not` keeps the operand type, `shl` and
  `shr` widen a byte-sized left operand. Constant expressions are folded in 32 bits.

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
| `readkey` | `() -> i32` | the next byte of Crt's ReadKey; blocks (`Atomics.wait`) (below) |
| `keypressed` | `() -> i32` | 1 when a key is waiting, else 0; non-blocking, does not consume |
| `readline` | `(ptr, max) -> len` | blocks, line editing in JS; `len` is -1 at end of input |
| `screen` | `(op, a, b, c)` | cell operations on the 80x25 text screen (below) |
| `sleep` | `(ms)` | `Delay`: the clock moves forward by `ms` (decision 38) |
| `fs_open` | `(namePtr, nameLen, mode) -> h` | VFS, text only; `h` < 0 is an error |
| `fs_read` / `fs_write` | `(h, ptr, len) -> n` | VFS; `n` < 0 is an error |
| `fs_close` | `(h) -> status` | VFS; 0, or < 0 on error |
| `fs_delete` | `(namePtr, nameLen) -> status` | VFS; 0, or < 0 on error |
| `fs_rename` | `(oldPtr, oldLen, newPtr, newLen) -> status` | VFS; 0, or < 0 on error |
| `fmt_real` | `(f64, width, decimals, bufPtr) -> len` | TP-style real formatting; `width` and `decimals` are -1 when absent; the buffer holds 512 bytes; imported only by modules that use Real (`host/fmt_real.js`) |
| `halt` | `(code)` | terminate with exit code |
| `rterror` | `(code)` | runtime error report |
| `time` | `(what) -> i32` | the local clock for `GetDate`, `GetTime` and `Randomize` (below) |
| `ide_compile` | `(namePtr, textPtr, textLen, resultPtr)` | IDE only (unit `IdeHost`, decision 42): compiles the buffer as the main program; `namePtr` is a Pascal string, `resultPtr` a 23-byte packed `CompileResult` (Code: Integer, Line and Col: LongInt, FileName: string[12], little endian) |
| `ide_run` | `() -> i32` | IDE only: runs the last compiled module and returns when it ends: exit code 0..255, -1 stopped, -2 nothing to run |

### How a host import is declared

A host import is a routine declared `external` in an embedded runtime source (`runtime/system.pas`,
`runtime/dos.pas`, `runtime/crt.pas`, `runtime/idehost.pas`): `procedure RtError(code: LongInt); external;`. `external` is
accepted only there; in a user program it is error 200. The import is from module `host`; its
name is the routine's name in lower case, or the string after the directive when the routine
cannot have that name (`procedure HostWrite(...); external 'write';`, because `Write` is
predeclared). The import is emitted only when the program reaches it. Imports are numbered
first, in the order they are first reached (`write` and `rterror` always come first, as the
built-in functions call them), then the defined functions.

Parameter and result mapping: a `Real` parameter is an f64 passed by value; every other
parameter (Integer, Word, LongInt, Byte, ShortInt, Boolean, Char, enumeration, pointer) is an
i32 by value, sign- or zero-extended as for any routine. A function returns one i32; a
procedure returns nothing. `var` parameters, strings, sets, arrays, records, a Real result and
a nested routine are error 200 in the runtime source. Currently: `write` is `HostWrite`,
`rterror` is `RtError`, `fmt_real` is `Fmt_Real`, `halt` is `HostHalt`, `readkey` is `HostKey` (System) or
`HostReadKey` (Crt), `time` is `HostTime`.

Compiler API exported from `compiler.wasm` (`plan.md` §3): `alloc(n) -> ptr`,
`compile(src_ptr, src_len, flags) -> status`, `out_ptr()`, `out_len()`, `err_line()`,
`err_col()`, `err_code()`, `err_file()`, `err_message_ptr()` and `err_message_len()` (the TP message
of the last error, static text in the module's memory), `add_file(name_ptr, name_len, src_ptr, src_len) -> status`.

Units (decision 27): the host registers every unit source before `compile`, and the compiler
never calls back. `alloc` hands out one buffer that the next `alloc` invalidates, so the host
allocates the name and the source together, writes the name and then the source into that
buffer, and calls `add_file` with both ranges inside it (status 1 if a range is outside the
buffer, else 0). A unit `U` is the registered file `U.PAS`, any case; registering a name again
replaces the file and keeps its number. Registered files stay registered across `compile`
calls. After a failed `compile`, `err_file()` is 0 for the main program (and System), else the
file's number: its position in registration order, counting from 1. The native CLI registers
the `.pas` and `.inc` files beside the input, then those of `--dir AUXDIR`, which win on a name
clash. Include files (`{$I name}`) are looked up in the same table, `name` or `name.PAS` when it
has no extension; the CLI registers only `.PAS` and `.INC` files, `add_file` takes any name.

### The clock (`time`) and Dos

`runtime/dos.pas` is an embedded unit: `uses Dos` finds it before any registered file, so a
user `DOS.PAS` never replaces it (TP 5.0 takes Dos from its own TURBO.TPL; measured). It is
file 0 in diagnostics, with all tokens at line 0, like System. It declares `external 'time'` in
its implementation part and computes the calendar in Pascal. `time(what)`, local time:

- `time(0)` takes a reading of the clock and returns the whole days since 1970-01-01;
- `time(1)` returns the seconds since midnight of that reading (0 to 86399);
- `time(2)` returns the hundredths of a second of that reading (0 to 99).

`GetDate` and `GetTime` call `time(0)` first, so the date and time of one call come from one
instant. Dates before 1970 are not supported. The test runner reads the local time in
milliseconds since 1970-01-01 from the environment variable `TP5_NOW_MS` when set (tests fix the
clock with it), else the real local time. Any other `what` is a runner error.

### The screen, keys and sleep (decisions 37 to 39)

`screen(op, a, b, c)` applies one cell operation to the grid of 80 columns by 25 rows
(`host/screen.js`). Columns and rows are 0-based; an operand that puts a cell off the screen,
or an unknown `op`, is a host error. Operands are packed in 16-bit words (`lo` is the low byte,
`hi` the high byte):

| op | Operation | a | b | c |
|---|---|---|---|---|
| 0 | put a character | column | row | character `lo`, attribute `hi` |
| 1 | scroll a rectangle up | left `lo`, top `hi` | right `lo`, bottom `hi` | lines `lo`, fill attribute `hi` |
| 2 | scroll a rectangle down | as 1 | as 1 | as 1 |
| 3 | set the cursor | column | row | unused |

The rectangle is inclusive. Scrolling by `n` moves each row `n` rows (rows outside the
rectangle are untouched), and the `n` rows left empty are filled with blanks in the fill
attribute; `n` of 0 changes nothing. A new screen is all blanks, attribute `$07`, cursor 0,0.

`readkey` returns the next byte `Crt.ReadKey` returns: an ASCII code, or 0 for an extended key
(the arrows, F1 to F10, ...) followed by the scan code on the next call. The browser blocks
until a key arrives; the runner, when its key script is used up, ends the program with exit
code 255 instead; `keypressed` then returns 0. The same `Host` arguments serve both: `keys`
`{ available(), next() }`, `clock` `{ nowMs(), advance(ms) }` (the browser's `advance` really
waits) and `screen` (the browser passes a `Screen` over the shared grid of `host/terminal.js`).

`sleep(ms)` calls `clock.advance(ms)`: the runner's virtual clock moves by `ms` milliseconds
and returns at once.

The screen dump is the text of the final screen, printed by the runner after the program's
stdout when the program imports `screen`, and by `tools/tp5.sh` for a program that uses Crt.
It is, line by line: the 25 rows (trailing blanks cut), `cursor COL ROW` (1-based), `exit CODE`,
and one `attr ROW COL LEN HEX` line (1-based, two hex digits) for each run of cells in a row
whose attribute is not `07`, in reading order. A runtime error prints its `Runtime error NNN`
line first (to stdout), then the dump with `exit NNN`.

### The Crt unit

`runtime/crt.pas` is an embedded unit like Dos (`uses Crt` never finds a user `CRT.PAS`). It keeps
the window (`WindMin`, `WindMax`), cursor and `TextAttr` and draws with `screen`; `ReadKey`,
`KeyPressed` and `Delay` call `readkey`, `keypressed` and `sleep`. Its initialisation sets
System's `ConsoleHooked` and two procedural variables: `ConsoleOut(address, count)` receives
console output (a `WriteLn` ends with CR LF), `ConsoleLine(address, max)` reads a console line
like `readline` and does the editing (Backspace, Esc, extended keys ignored). Without Crt the
hooks are unused and output goes to `write`. While hooked, System holds console output in a
128-byte buffer until the end of the statement: the compiler calls `TextFlush` after every
`Write` and `WriteLn`, so `WhereX` inside a statement sees the cursor before the statement's text,
as in TP.

### Text files in System

`Text` is a record in `runtime/system.pas` (host handle, mode, buffer position and length, name,
256-byte buffer); `Input` and `Output` are typed constants of that type. The five file and
console calls are `external` declarations there (`Fs_Open`, `Fs_Read`, `Fs_Write`, `Fs_Close`,
`ReadLine`). The compiler writes by calling `TextOut(file address, pointer, count)` (address 0
is the console) and `TextNewLine`, and reads by calling the `Read*` helpers. Console input is
read a line at a time with `readline` (256-byte buffer, so a line is cut at 254 characters,
D29); the runtime appends CR LF. `InOutRes` holds the pending error; with `{$I+}` the compiler
calls `CheckIo` after each Write statement and each call of a routine whose first parameter is
a `Text`, except `Assign`.

### File and console input contract

Used by the test runner (`tests/vfs.mjs`) and the browser host alike. The runtime (`System`)
turns a negative result `-e` into `InOutRes := e`, so the codes are DOS error numbers.

- `fs_open(namePtr, nameLen, mode)`: `mode` is 0 Reset (read; the file must exist), 1 Rewrite
  (create or truncate; write only), 2 Append (the file must exist, position at the end; write
  only). Returns a handle (1 or more) or a negative error number.
- The file system is flat. A name is 1 to 8 characters, optionally a dot and 0 to 3 more, from
  `A-Z 0-9 _ $ ~ ! # % & - { } ( ) @ ' ^`, matched case-insensitively (files are created in
  upper case). The name is raw bytes, not a path: a name containing `\`, `/` or `:` is a path
  error.
- `fs_read(h, ptr, len)` returns the number of bytes stored at `ptr` (0 to `len`, 0 at end of
  file); `fs_write(h, ptr, len)` returns `len`. Bytes are raw: the host does no line-ending or
  ^Z translation (TP writes CRLF itself and treats ^Z as end of file when reading text, so
  `System` does both).
- `fs_delete(namePtr, nameLen)` removes the name; `fs_rename(oldPtr, oldLen, newPtr, newLen)`
  gives the file a new name. Both return 0 or a negated error. An open handle is unaffected
  by either: it keeps reading and writing the file (renamed, or unlinked after a delete; a
  closed unlinked file is gone). A rename needs an existing source and a new name that does not
  exist; the new name may not contain a path.
- Several handles on one file have independent positions. At most 20 files are open at once.

| Result | DOS error | Cause |
|---|---|---|
| -2 | 2 file not found | Reset or Append, `fs_delete` or the source of `fs_rename` of a missing file; a name that is not a valid 8.3 name (any mode) |
| -3 | 3 path not found | name with `\`, `/` or `:` (in the new name of `fs_rename` only `:`) |
| -4 | 4 too many open files | a 21st open handle |
| -5 | 5 access denied | `fs_rename` onto an existing name or to a new name with `\` or `/`; unknown mode; read on a Rewrite/Append handle; write on a Reset handle |
| -6 | 6 invalid handle | `fs_read`, `fs_write` or `fs_close` on a handle that is not open (never opened or already closed) |

- `readline(ptr, max)` stores the next input line at `ptr`, without its LF or CRLF, cut to
  `max` bytes (the rest of a longer line is discarded), and returns the stored length. At end
  of input it returns -1 and stores nothing. A final line without a terminator is a line.
- Test runner (decision 26): the directory `DIR/NAME/` of a test program `DIR/NAME.pas` is the
  initial file system and `DIR/NAME.in` is the input. The runner reads the directory into
  memory at startup, so files the program creates or changes are never written back and
  every run starts from the same state. Without a directory the file system is empty; without
  an input file `readline` reads the process stdin.

### Test runner contract (`tests/run.mjs`)

- The module exports `main` (`() -> ()`) and `memory`; both are required.
- `write(fd, ptr, len)`: fd 1 is stdout, fd 2 is stderr, other fds are a runner error. Bytes are
  copied raw from `memory`; no text decoding.
- `halt(code)` stops execution; the exit code is `code`.
- `rterror(code)` writes `Runtime error NNN` and LF to stdout (code zero-padded to three digits
  as TP5 prints it, no address), then behaves as `halt(code)`. fpc writes to stderr; TP5 writes
  to the console.
- Command line: `run.mjs [--dir AUXDIR] [--in INPUT] [--keys KEYS] prog.wasm [expected.out]`;
  `--dir` and `--in` supply the file system and the console input described above, `--keys` a
  key script (tools/tp5.sh documents the format: whitespace-separated key names and printable
  characters) for `readkey` and `keypressed`, `--compiler` the compiler of an IDE program (the
  path of a `compiler.wasm`, or `canned`, the canned compiler of the oracle tests). The JS API is
  `run(bytes, { files, input, keys, compiler, onSaved })` with the directory path (or a Map of
  name to bytes) and two Buffers (all default to null), a `Compiler` or `"canned"`, and a
  callback for the files the program closes after writing them. A missing directory, input
  or key file, or an unknown key in the script, is a runner error. The drivers pass
  `NAME.keys` when it exists, as they pass `NAME.in`.
- The clock is virtual (decision 38): it starts at `TP5_NOW_MS`, else at the real local time,
  and moves only through `sleep`.
- A program that imports `screen` ends with the screen dump on stdout (above).
- Exit codes: program code when output matches (or none is expected), 1 on output mismatch,
  2 on runner error (stderr prefix `run.mjs:`). A program `halt(2)` is indistinguishable by code.
- A host `RangeError` (engine stack exhausted by recursion in routines without a frame) is
  reported as `Runtime error 202`, exit code 202, the same as the shadow-stack overflow.
- `ide_compile` and `ide_run` exist only with `--compiler`; `ide_run` is a nested run of the
  compiled program on the same screen, keys and clock, starting on a cleared screen; its console
  output is drawn on the screen and its file changes are discarded. Unknown imports fail
  instantiation.

## 9. Runtime errors

A runtime error prints `Runtime error NNN` (three digits, zero-padded, no address) through
`rterror` and ends the program with exit code NNN. `RunError(n)` raises any code.

| Code | Meaning | Raised by |
|---|---|---|
| 2, 3, 5 | file not found, path not found, access denied | file calls under `{$I+}` (the codes `IOResult` reports) |
| 102 to 106 | file not assigned, not open, not open for input, not open for output, invalid numeric format | text file calls and `Read` under `{$I+}` |
| 200 | division by zero | `div`, `mod` and Real `/` |
| 201 | range check | under `{$R+}`: assignment to a subrange or narrower integer, indexing |
| 202 | stack overflow | frame prologue below the stack limit; an engine stack exhaustion is reported the same way |
| 203 | heap overflow | `New`, `GetMem` when a block does not fit |
| 204 | invalid pointer operation | `Dispose` or `FreeMem` of an address outside the heap or not on a block boundary |
| 205 | floating point overflow | `Exp` above about 709.78 |
| 207 | invalid floating point operation | `Sqrt` of a negative number, `Ln` of zero or a negative number (so `Utils.Power` of such a base) |

- Reading at the end of a text file is not an error: `Read` yields 0, an empty string or ^Z,
  and `Eof` is true.
- Source addresses are not supported.
- `{$D}` is accepted and ignored (D38); no line information is kept.

## 10. Decisions that settle earlier open questions

- Integer expression typing: section 6 (decisions 5 and 10). Sizes and layout that differ from
  fpc (`set` is 32 bytes, pointers are 4, records are packed) are D5 to D7; tests do not print
  `SizeOf` of those.
- `String` is `String[255]` (decision 6). Only strings and sets are returned through
  `result_ptr`; TP 5.0 functions cannot return records or arrays, and a `Real` is returned as an
  f64.
- The initial `sp` is the top of memory, 1,048,576 (decision 6). Static data starts at 256 as
  section 1 shows; the heap starts after it, rounded up to 8.
- `rterror(code)` prints and then stops with exit code `code`; `halt(code)` stops with `code`.
  `write` accepts fd 1 (stdout) and 2 (stderr) only.
- Table index 0 is the trap stub; routines whose address is taken follow in routine order
  (section 5). The table is always emitted.
- `{$R}` defaults to off, as in TP 5.0. Heap helpers are Pascal globals and routines in `System`
  (`HeapOrg`, `HeapPtr`, `FreeList`, `HeapGet`, `HeapPut`, `HeapMark`, `HeapRelease`).

`readline` in the browser returns -2 ("no line input": there is no stdin); System then
edits the line from `readkey` (decision 46). The Node runner returns -1 at end of input.
