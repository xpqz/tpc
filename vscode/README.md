# Turbo Pascal 5.0 for VS Code

Pascal highlighting, the compiler's errors as you type, and compile and run commands, from the
TP5-WASM compiler (a Turbo Pascal 5.0 work-alike that compiles to WebAssembly). The compiler is
bundled; nothing else needs installing.

## Features

- Highlighting for `.pas` and `.inc` files.
- Errors as you type: the compiler's own `Error N: message` at TP's position. An error in a unit
  appears in the unit's file. Units and include files are read from the program's directory,
  using the unsaved text of open editors.
- **TP5: Compile** writes `NAME.wasm` beside `NAME.pas`.
- **TP5: Run** compiles and runs the program in a terminal. `Crt` programs draw an 80x25 text
  screen with the DOS colours and read keys as `ReadKey` does.
- **TP5: Stop** ends the running program.

## Keys

In Pascal files only: `F9` compile, `Ctrl+F9` run. `Ctrl+C` in the terminal, closing it or
**TP5: Stop** ends a program.

## Settings

- `tp5.diagnostics.onType` (default on): compile while typing; when off, files are compiled when
  opened and saved.
- `tp5.compilerPath`: another `tp5c.wasm` to use instead of the bundled one.
- `tp5.run.writeFiles` (default on): see below.

## Files written by a program

A program runs in its own directory. With `tp5.run.writeFiles` on, the files it creates, changes,
erases or renames are written through to that directory, as in Turbo Pascal. With it off they stay
in memory and are gone when the program ends.
