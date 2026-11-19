# TP5-WASM

A Turbo Pascal 5.0 work-alike. A compiler written in Rust (no dependencies) translates a
defined subset of TP 5.0 to one WebAssembly module. The runtime (`System`, `Crt`, `Dos`) and
the IDE are written in Pascal and compiled by this compiler. The compiler itself builds to a
WASM module, so the whole system runs in a browser: edit, compile and run Pascal on a
80x25 text screen.

The compiler, runtime, IDE and host glue together are limited to 15,000 lines (counted by
`tools/loc.sh`; tests are free). The limit is there so that the code stays small enough to
read: the source is meant to be studied as a worked example of a one-pass-per-stage compiler
(lexer, parser, checker, code generator, WASM emitter). Readability wins over brevity.

Where the program and the real TP 5.0 differ, the difference is listed in
`docs/deviations.md`. Features that are left out give an error, never wrong behaviour
(`docs/subset.md`).

## Building

```
make build
```

Build output goes to a per-checkout directory chosen by `scripts/cargo-target.sh`
(`/Volumes/Space2/tpa/<checkout name>` when that disk is mounted, else `target-shared` in the
checkout), never to `target/`. The Makefile exports it as `CARGO_TARGET_DIR`. To run `cargo`
directly, set it first:

```
export CARGO_TARGET_DIR=$(scripts/cargo-target.sh)
```

The native compiler is then `$CARGO_TARGET_DIR/debug/tp5c`. The WebAssembly build of the
compiler (used by the browser) comes from:

```
make wasm
```

and is `$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/tp5c.wasm`.

## Compiling and running a program

```
tp5c in.pas -o out.wasm [--dir AUXDIR]
```

Units (`uses`) and `{$I}` includes are the `.pas` and `.inc` files beside `in.pas`, then
those in `AUXDIR`, which win on a name clash. `Crt`, `Dos` and `Utils` are built in. An error
is printed as `file(line,col): Error N: message` and the exit code is 1.

`tests/run.mjs` runs a compiled module in Node:

```
node tests/run.mjs [--in INPUT] [--dir FILES] [--keys KEYS] out.wasm
```

`--in` is the console input, `--dir` a directory used as the program's file system, `--keys` a
key script for `ReadKey`. A program that uses `Crt` ends with a dump of the final screen.

Example:

```
mkdir -p tmp
echo "begin WriteLn('hi') end." > tmp/h.pas
$CARGO_TARGET_DIR/debug/tp5c tmp/h.pas -o tmp/h.wasm
node tests/run.mjs tmp/h.wasm
```

## The browser and the IDE

The page (`host/index.html`) and the Worker that runs the program share memory (the screen and
the key queue are `SharedArrayBuffer`s). Browsers offer `SharedArrayBuffer` only to
cross-origin isolated pages, so whatever serves the files must send these headers on every
response:

```
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

`tools/serve.mjs` is a dev server that serves the repository root with them. Without the
headers `crossOriginIsolated` is false and the page cannot run programs; there is no fallback.

To run a program in the browser, compile it to a file inside the repository and name it in the
URL:

```
node tools/serve.mjs 8080
# http://localhost:8080/host/index.html?wasm=/tmp/h.wasm   (the repository's tmp/ directory)
```

To run the IDE, compile `ide/ide.pas` and pass the compiler module too (copy it where the
server can reach it):

```
$CARGO_TARGET_DIR/debug/tp5c ide/ide.pas -o tmp/ide.wasm
cp $CARGO_TARGET_DIR/wasm32-unknown-unknown/release/tp5c.wasm tmp/tp5c.wasm
node tools/serve.mjs 8080
# http://localhost:8080/host/index.html?wasm=/tmp/ide.wasm&compiler=/tmp/tp5c.wasm
```

The IDE's files live in the browser's `localStorage`. Run and Stop are the buttons under the
screen; Stop ends the program that is running, or the IDE when none is.

### IDE keys

| Key | Action |
|---|---|
| Arrows, Home, End, PgUp, PgDn | move |
| Ins | toggle insert and overwrite |
| Del, Backspace | delete |
| Enter | new line, indented like the line above |
| Ctrl-Y | delete the line |
| `^K B`, `^K K` | mark the block start and end |
| `^K C`, `^K V` | copy and move the block to the cursor |
| `^Q F`, Ctrl-L | find; find again |
| F2, F3 | save, open |
| F9, Ctrl-F9 | compile, run (an error puts the cursor on its line) |
| F10, Alt-F, Alt-R, Alt-C, Alt-O | menus (File, Run, Compile, Options) |
| Alt-X | quit |

A browser does not deliver Ctrl-K and Ctrl-Q to a page, so **Alt-K** and **Alt-Q** are accepted
in their place: `Alt-K B` is `^K B`, `Alt-Q F` is `^Q F` (decision 41). Every command is also in
the menus, so none depends on F9 or F10. Only the Alt-letter menu keys in the table above
exist; the bar has no Edit menu.

## Units and routines

`System` (always present), `Crt`, `Dos` and `Utils` are built in. `Utils` is not part of TP 5.0:
it gathers `Power`, `Tan`, `ArcSin`, `ArcCos`, `Log10`, `Hex`, `Trim`, `TrimLeft`, `TrimRight`,
`UpperCase`, `LowerCase`, `Min`, `Max` and `LeadingZero`, which many programs write
themselves (decision 53). The full list of supported and omitted names is `docs/subset.md`.

## Tests

```
make test          # everything except the browser tests: Rust, Node, tool scripts
make check         # fmt, clippy, make test and the line budget: the merge gate
make test-browser  # the browser tests; drives an installed Chrome (and Firefox if found)
tools/loc.sh       # line counts per component against the budget
```

Test programs are paired with expectations from the real TP 5.0:

| Directory | Contents |
|---|---|
| `tests/lang/` | `NAME.pas` and the expected output `NAME.out`; optional `NAME.in` (input), `NAME.keys` (keys), `NAME/` (files) |
| `tests/neg/` | invalid programs and `NAME.err`, the error number and line |
| `tests/rt/` | programs that must stop with a given runtime error |
| `tests/abi/` | frames, `sp` restoration, static links, `var` aliasing |
| `tests/halt/` | `Halt` and exit codes |
| `tests/ide/`, `tests/host/` | the IDE modules and the browser host |
| `compiler/tests/` | the Rust tests, and per-feature corpora under `fixtures/` |

### The TP 5.0 oracle

Expected output is never written by hand. `tools/tp5.sh prog.pas` compiles and runs the program
with Borland's `TPC.EXE` under DOSBox-X and prints its output, or `P.PAS(LINE): Error N:
Message.` and exit code 1. `make oracle` re-derives every corpus expectation with it and
reports differences.

Setup: install DOSBox-X (`dosbox-x` on the path), then extract `TPC.EXE` and `TURBO.TPL` from
Borland Turbo Pascal 5.0 disk 1 into one directory and point `TP5_HOME` at it (the default is
`/Users/stefan/work/crust/tp5-oracle/c`). The files are not in the repository; see
`docs/references.md` for where to get them. Free Pascal (`fpc -Mtp`) is only a secondary check.

## Where to read next

- `plan.md`: the specification, phases and test strategy.
- `docs/subset.md`: what is supported and what is omitted, with the error each omission gives.
- `docs/deviations.md`: every difference from TP 5.0.
- `docs/abi.md`: memory map, calling convention, host imports and the file-system contract.
- `docs/decisions.md`: numbered design decisions with their reasons.
- `docs/size.md`: output sizes and line counts at each phase.
- `docs/references.md`, `docs/tp-errors.md`: sources and the error-number table.
