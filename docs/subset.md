# Language subset

What the compiler accepts. The reference is Turbo Pascal 5.0 (`plan.md` §2); the real
compiler is the oracle (`tools/tp5.sh`). Where behaviour differs on purpose the entry is in
`docs/deviations.md`, cited here as D*n*. The sources of truth in the code are
`compiler/src/symbols.rs` (`PREDEFINED`, the names the compiler knows), `runtime/*.pas`
(the names the embedded units declare) and the `NOT_SUPPORTED` (200) sites in
`compiler/src/{parse,check,directives}.rs`.

## Language

| Area | Supported |
|---|---|
| Types | `Byte`, `ShortInt`, `Integer` (16-bit), `Word`, `LongInt`, `Boolean`, `Char`, `Real` (IEEE f64, D1), `String[n]` (1 to 255), `String` (= `String[255]`), enumerations, subranges, arrays (any number of dimensions), records with variant parts, `set of` an ordinal base of at most 256 values (always 32 bytes, D5), `Pointer` and typed pointers (4 bytes, D2), procedural types, `Text` |
| Statements | `begin`/`end`, `if`, `case` (ranges, lists, `else`), `while`, `repeat`, `for` (`to`, `downto`), `with`, assignment, procedure and function calls, `Exit`, `Halt`, `RunError` |
| Declarations | `const` (constant expressions are folded in 32 bits), typed `const` (scalars, strings, sets, arrays, records, pointers, procedural values), `type`, `var`, nested procedures and functions, `forward`, units (`interface` / `implementation` / `initialization`, `uses`) |
| Expressions | all Pascal operators including `div`, `mod`, `shl`, `shr`, `and`, `or`, `xor`, `not`, `in`, set `+ * -`, set comparison, string `+` and comparison, `@` (address of a variable or global routine), `^`, type casts `T(x)` of scalars and pointers |
| Parameters | value, `var` (typed), and procedural parameters. Value parameters that are strings, sets, arrays or records are copied by the callee |
| Procedural values | Only global routines can be assigned or passed (a nested routine is error 143, D4). `{$F+}` and `{$F-}` change nothing (D11) |
| Integer arithmetic | TP 5.0's typing rules, with 16-bit wraparound for `Integer` and `Word` (decisions 5, 10; `docs/abi.md` §6). `{$R+}` adds range checks (runtime error 201) |
| Real | `+ - * /`, comparison, `Trunc`, `Round`, `Int`, `Frac`, `Sqrt`, `Abs`, `Sqr`, `Pi`, and `Exp`, `Ln`, `Sin`, `Cos`, `ArcTan` (D37); `Write(x:w:d)` and `Str(x:w:d, s)` through `host.fmt_real` (D1, D14, D15) |

### Directives

| Directive | Effect |
|---|---|
| `{$R+}` `{$R-}` | range checking; default off |
| `{$I+}` `{$I-}` | I/O checking; default on |
| `{$I file}` | include; only in declaration parts (error 118 inside a statement part); nests 14 deep |
| `{$IFDEF}` `{$IFNDEF}` `{$ELSE}` `{$ENDIF}` `{$DEFINE}` `{$UNDEF}` | conditional compilation. Predefined symbols: `VER50`, `MSDOS`, `CPU86`, `CPU87` (D28). `{$IFOPT}` is error 17 |
| `{$F+/-}` `{$A}` `{$B}` `{$D}` `{$L+/-}` `{$O}` `{$S}` `{$V}` (with `+` or `-`) | accepted, no effect (D11, D38) |
| `{$M stack,heapmin,heapmax}` | checked (stack 1024 to 65520, heap 0 to 655360), no effect (D38) |
| `{$N+}` `{$E+}` | error 200 (8087 code is omitted); `{$N-}` and `{$E-}` are accepted |
| `{$L file}`, any other letter, a malformed directive | error 17 (D45) |

## Standard names

All of these are declared by the compiler (`PREDEFINED`) or by the embedded `System` unit, so
every program sees them.

| Group | Names |
|---|---|
| Constants | `True`, `False`, `nil`, `MaxInt`, `MaxLongInt` |
| Ordinal and arithmetic | `Ord`, `Chr`, `Succ`, `Pred`, `Odd`, `Abs`, `Sqr`, `Inc`, `Dec`, `Lo`, `Hi`, `Swap`, `Round`, `Trunc`, `Int`, `Frac`, `Sqrt`, `Exp`, `Ln`, `Sin`, `Cos`, `ArcTan`, `Pi`, `Random`, `Randomize`, `RandSeed` |
| Strings | `Length`, `Copy`, `Pos`, `Concat`, `Delete`, `Insert`, `UpCase`, `Str`, `Val` |
| Memory | `New`, `Dispose`, `GetMem`, `FreeMem`, `Mark`, `Release`, `MemAvail`, `MaxAvail`, `SizeOf`, `Addr`, `FillChar`, `Move` |
| Control | `Exit`, `Halt`, `RunError`, `ExitCode` (a variable) |
| Program | `ParamCount` (always 0) and `ParamStr` (always `''`) (D19) |
| Text files | `Text`, `Input`, `Output`, `Assign`, `Reset`, `Rewrite`, `Append`, `Close`, `Flush`, `Erase`, `Rename`, `SetTextBuf` (ignored, D40), `Read`, `ReadLn`, `Write`, `WriteLn`, `Eof`, `Eoln`, `IOResult`. `Assign(f, '')` is the console (D42); device names are ordinary file names (D35). File names are 8.3 names of a flat file system (`docs/abi.md` §8, D51) |

Not in TP 5.0 and therefore not here: `Include`, `Exclude`, `Break`, `Continue` (error 3).

### `Crt` (embedded unit)

`ClrScr`, `ClrEol`, `InsLine`, `DelLine`, `GotoXY`, `WhereX`, `WhereY`, `Window`, `TextColor`,
`TextBackground`, `HighVideo`, `LowVideo`, `NormVideo`, `ReadKey`, `KeyPressed`, `Delay`, `Sound` and
`NoSound` (no effect, D33), `TextMode` (D48), `AssignCrt`. Variables `TextAttr`, `WindMin`, `WindMax`,
`LastMode` (D41), `CheckBreak`, `CheckEof`, `CheckSnow`, `DirectVideo` (the last four change
nothing). Constants `Black` to `White`, `Blink`, and the modes `BW40`, `CO40`, `BW80`, `CO80`, `Mono`,
`Font8x8`. The screen is always 80 by 25 colour.

### `Dos` (embedded unit)

`GetDate`, `GetTime`, `PackTime`, `UnpackTime`, `FSplit`, `FExpand`, `GetDir`, `DosVersion`,
`GetEnv`, `EnvCount`, `EnvStr` (D47). Types `DateTime`, `SearchRec`, `PathStr`, `DirStr`,
`NameStr`, `ExtStr`. Variable `DosError` (never set, D39). Constants `ReadOnly`, `Hidden`,
`SysFile`, `VolumeID`, `Directory`, `Archive`, `AnyFile`, `fmClosed`, `fmInput`, `fmOutput`,
`fmInOut`. Dates before 1970 are not supported.

### `Utils` (embedded unit, not part of TP 5.0, D50)

`Power`, `Tan`, `ArcSin`, `ArcCos`, `Log10`, `Hex`, `TrimLeft`, `TrimRight`, `Trim`, `UpperCase`,
`LowerCase`, `Min`, `Max`, `LeadingZero`. It is plain TP 5.0 Pascal, so the oracle compiles the same
file.

`Crt`, `Dos` and `Utils` are found before any unit file of the same name next to the program.
Any other `uses` name is looked up among the registered `.PAS` files; a missing one is error 15.

## Deliberately omitted

Each is an error, never silent misbehaviour. Error 200 is "Feature not supported", outside
TP's own numbers (decision 6). Names that are not declared at all give the ordinary error 3.
Every row was run through `tp5c`.

| Omitted | Error |
|---|---|
| `goto`, `label` | 200 at the `goto` or `label` |
| `asm` blocks | 200 at `asm` |
| `absolute` | 200 |
| `inline` (statement or directive) | 200 |
| `interrupt` procedures | 200 |
| `external` in a user program | 200 (only the embedded runtime may use it) |
| Overlays (`uses Overlay`) | 15, as for any unit that is not found; `{$O}` is ignored |
| BGI (`uses Graph`) | 15 |
| Binary `.TPU` files | never read; the unit is looked up as `.PAS` source, else 15 |
| `Single`, `Double`, `Extended`, `Comp` | 200 at the type name, also without `{$N+}` (D44) |
| `file`, `file of T` | 200 |
| Untyped `var` parameters (`procedure P(var a)`) | 200 |
| `Seg`, `Ofs`, `Ptr` | 200 |
| A type cast of a `Real`, `String`, set, array or record | 200 |
| A procedural type whose result is `Real` or `String` | 200 |
| `Chain`, `Execute` | 3 (not declared) |
| `BlockRead`, `BlockWrite`, `Seek`, `SeekEof`, `SeekEoln`, `FilePos`, `FileSize`, `Truncate` | 3 |
| `ChDir`, `MkDir`, `RmDir`, `Mem`, `MemW`, `Port`, `PortW`, `ExitProc`, `ErrorAddr`, `HeapError`, `FileMode`, `Test8087`, `Cseg`, `Dseg`, `Sseg`, `PrefixSeg` | 3 |
| `Dos`: `FindFirst`, `FindNext`, `FSearch`, `DiskSize`, `DiskFree`, `GetFAttr`, `SetFAttr`, `GetFTime`, `SetFTime`, `SetDate`, `SetTime`, `Exec`, `Intr`, `MsDos`, `GetIntVec`, `SetIntVec`, `Keep`, `SwapVectors`, `GetCBreak`, `SetCBreak`, `GetVerify`, `SetVerify`, `DosExitCode` | 3. The flat virtual file system has no listing, disks or attributes (D39); there are no interrupts or child processes |
| `{$L file}` | 17 (D45) |
| `{$N+}`, `{$E+}` | 200 |

A program that TP rejects for another reason but that uses one of these first gets error 200
(D46). `object` is not TP 5.0 either; it is a syntax error.

## Limits

- Globals, and each routine's locals: at most 65,535 bytes each (error 96 beyond). One type larger than 65,535 bytes is error 22.
- A set base type has at most 256 values (error 23 otherwise).
- Heap and stack: a fixed 1 MB memory, the stack in its top 64 KB (`docs/abi.md` §1).
  `MemAvail` and `MaxAvail` report this heap, not TP's.
- `Text` console lines are cut at 254 characters (D29).
