# Deviations from Turbo Pascal 5.0

Every deviation from TP5 behaviour is recorded here, with the reason. Append new entries
at the end. "fpc" means Free Pascal 3.2.2 with `-Mtp`; where fpc and TP5 differ, the TP5
manual is the reference.

## Accepted up front (`plan.md` §2)

| # | Deviation | TP5 | This compiler | Notes |
|---|---|---|---|---|
| D1 | `Real` representation | 6-byte software real | IEEE f64, 8 bytes | `Write(x:w:d)` goes through `host.fmt_real`; formatting diverges from TP5 where noted |
| D2 | Pointers | 16:16 segment:offset, 4 bytes | 32-bit offset into linear memory, 4 bytes; `nil` is 0 | `Seg`, `Ofs`, `Ptr` are omitted |
| D3 | `Integer` width | 16-bit | 16-bit (unchanged); `LongInt` 32-bit | Recorded because the WASM repr. is i32 with explicit re-narrowing (`docs/abi.md` §6) |
| D4 | Nested routine assigned to a procedural variable | Compile error | Compile error | Same as TP; listed because the plan lists it |

D3 and D4 are not deviations from TP5 behaviour; they are kept as entries because
`plan.md` §2 lists them under accepted deviations.

## Found while writing the ABI (not in `plan.md` §2)

| # | Deviation | Reference | This compiler | Notes |
|---|---|---|---|---|
| D5 | `set of T` size | fpc: minimal size for the base range (`set of 1..3` is 1 byte); TP5 likewise | Always 32 bytes | `SizeOf` on a set differs; do not print it in differential tests |
| D6 | `SizeOf(Pointer)` | fpc native: 8 on this host; TP5: 4 | 4 | Do not print in differential tests |
| D7 | Record layout | fpc `-Mtp` aligns plain records (`Byte, Integer, LongInt` is 8 bytes); TP5 packs (7) | Packed, no padding | Differential tests declare records `packed` |

D5 to D7 apply to what `SizeOf` and memory layout reveal. Intermediate-expression typing
(`docs/abi.md`, open questions 1 to 4) is unresolved and will be recorded here once decided.

## Integer expression width (decision 5)

| # | Deviation | Reference | This compiler | Notes |
|---|---|---|---|---|
| D8 | Intermediate integer width | fpc evaluates at native width (`(i+i2) div 2` with 30000+30000 is 30000) | TP5 common-type rule: 16-bit `Integer` arithmetic wraps (-2768) | Not a deviation from TP5 (measured, decision 10); fpc is not the oracle |

## Routines (Phase 1)

| # | Deviation | TP5 | This compiler | Notes |
|---|---|---|---|---|
| D9 | Order of operand evaluation | The right operand of a binary operator is evaluated first when it contains calls (`Show(1) + Show(2) * Show(3)` runs 3, 2, 1) | Left to right (arguments of calls and of `Write` are left to right in both) | Observable only through side effects inside one expression; Pascal leaves it undefined |
| D10 | Order of reported errors | One pass: the first error in source order, syntax or semantic | Parse of the whole program, then check: a syntax error anywhere precedes a semantic error before it | Only for programs with two errors; tests use single-error programs |

## Procedural types (Phase 2)

| # | Deviation | TP5 | This compiler | Notes |
|---|---|---|---|---|
| D11 | `{$F+}` for procedural values | A routine can be assigned to a procedural variable or passed as a procedural argument only if it was declared under `{$F+}`; otherwise error 143 | `{$F+}` and `{$F-}` are accepted and ignored; every global routine can be assigned | Accepts programs TP5 rejects; no valid TP5 program changes meaning. Any other directive is error 200 until the directives phase |

## System unit (Phase 3)

| # | Deviation | TP5 | This compiler | Notes |
|---|---|---|---|---|
| D12 | Names of implicit System helpers are visible | `AbsLong`, `StrCat`, `StrCmp`, `StrAssign`, `StrDelete` and `StrInsert` are not declared identifiers (error 3) | They are System routines a program can call or redeclare; redeclaring one never affects the compiler's own calls | Only a program using an undeclared identifier of that name changes (accepted instead of 3); redeclaring it never affects `Abs` (decision 11) |
| D13 | Disposing an already-disposed pointer | Runtime error 204 | Not detected (decision 25) | |

## Real (Phase 4)

| # | Deviation | TP5 | This compiler | Notes |
|---|---|---|---|---|
| D14 | Exact decimal ties in `Write` formatting | Ties round away from zero below magnitude 8 (`2.5:0:0` is 3, `4.25:0:1` is 4.3) but not above: `8.5:0:0` is 8, `9.5` is 9, `10.5` is 10, `12.5` is 13, `1234.5` is 1234, `12.375:0:2` is 12.37, `24.75:0:1` is 24.7, `123456.25:0:1` is 123456.2, `10.5:6` is `1.0E+01` | Ties round away from zero on the exact binary value at every magnitude (`8.5:0:0` is 9) | TP5 digit generation works on a 48-bit real; no test prints a tie of magnitude 8 or above. `Round` is half away from zero in both |
| D15 | Large magnitudes with decimals | Fixed formats show 11 significant digits, then zeros (`1.0E30:0:0`) | All digits of the f64 value | Applies to magnitudes of 1E11 and above. `-0` does not exist in TP5; it prints as `0` |
| D16 | `Trunc`/`Round` of a Real outside LongInt range | Runtime error | Saturates to the LongInt limit | |

## Intrinsics (Phase 4)

| # | Deviation | TP5 | This compiler | Notes |
|---|---|---|---|---|
| D17 | Side effects in the variable of `Inc`/`Dec` | The designator is evaluated once | `Inc(x, n)` is `x := Ord(x) + n`, so a designator with a function call (`Inc(a[F]);`) runs the call twice | No test depends on it |
| D19 | `ParamStr(0)` | The path of the program | `''`; `ParamCount` is 0 and `ParamStr(n)` is `''` | |
| D20 | Width wider than the target in `Str` of a Real | The width is cut to the capacity of the target string (`Str(1.5:8:2, t)` with `String[6]` is `  1.50`) | Done for integers; a Real is formatted at the full width and then truncated on the left | |
| D21 | `Inc`/`Dec` of a subrange under `{$R+}` | `Inc(s)` of `s: 1..10` at 10 is runtime error 201 (a Byte at 255 wraps, unchecked) | `Inc` and `Dec` are never range-checked; the value wraps into the storage size (D17 rewrite) | `lang/inc_dec_of_byte_at_the_limit_is_not_checked` covers the Byte case only |
| D22 | `Succ` of the last and `Pred` of the first value under `{$R+}` | Runtime error 201 | Not checked; the value is stored truncated | |

## Typed constants (Phase 5)

| # | Deviation | TP5 | This compiler | Notes |
|---|---|---|---|---|
| D23 | Line of the error for a list with too many elements | Error 89 is reported at the comma before the first extra element | Reported at the first extra element | Differs only when the element starts a new line; `neg/typedconst_too_many_elements` and the own tests keep the list on one line |
| D24 | A number for a `string[n]` constant | `n: string[5] = 42` is error 102 (String constant expected) | Error 26 (Type mismatch) | Still rejected; no test |
| D25 | Fields after a record prefix | `(x: 1)` for a record with more fields is accepted and the rest is undefined | The remaining fields are zero | No test reads them |

## Units (Phase 5)

| # | Deviation | TP5 | This compiler | Notes |
|---|---|---|---|---|
| D26 | A variable with the name of a used unit | Error 4 in the same scope; in a nested routine the variable hides the unit name | A unit name followed by `.` and an identifier is always the qualified name, so `U.x` in a routine that declares its own `U` reads the unit's `x` | Needs a local named like a used unit; no test |
| D27 | A variable named like a used unit, in the same scope | Error 4 | Accepted | Not tested |
| D28 | `CPU87` | Defined when an 8087 is present at compile time | Always defined | Conditional symbols that depend on the machine; the oracle machine has a coprocessor (`predefined_symbols_are_ver50_msdos_cpu86_and_cpu87`) |

## Text files (Phase 5)

| # | Deviation | TP5 | This compiler | Notes |
|---|---|---|---|---|
| D29 | Length of a console input line | A redirected line may be any length | A line longer than 254 characters is cut (`readline` is given the 256-byte buffer less the CR LF it appends) | Interactive TP5 input is limited to 127 characters; no test pins the cut |
| D30 | Size and fields of `Text` | `SizeOf(Text)` is 256 and a text file has no visible fields | `SizeOf(Text)` is 347 (System's record); fields of a `Text` are error 121 in user code | A System-internal layout |
| D31 | Text file output is not buffered | Data reaches the file when the buffer fills or at `Close`/`Flush` | Every `Write` piece goes to the host at once | Unobservable except through a second handle on the same file |

## Crt (Phase 6)

| # | Deviation | TP5 | This compiler | Notes |
|---|---|---|---|---|
| D32 | (retired by #99) Crt names left out | `TextMode`, `LastMode`, `AssignCrt` and the mode constants `BW40` to `Font8x8` exist | They exist now: see D41 (`LastMode`, `TextMode`, the mode constants) and D44 | The screen is always 80x25 colour. `CheckBreak`, `CheckEof`, `CheckSnow`, `DirectVideo` exist as variables and change nothing |
| D33 | Sound and ReadLn length | `Sound` makes a tone; a `ReadLn` line is at most 127 characters | `Sound` and `NoSound` do nothing; the line editor takes up to 254 characters (as D29) | |
| D34 | A parameterless function called with arguments inside `if`, `[]`, `for` bounds or `case` | Error 40, 93, 58 or 54 (the next token the construct expects) | Error 85 | Matching needs an expected-token context through `expr` (#81); assignment and argument positions match TP |
| D35 | DOS device names in Assign (COM1, LPT1, PRN, AUX, CON, NUL) | Devices | Ordinary file names | Decision 50 |
| D36 | Reading a variable never assigned | Memory contents (garbage) | Zero (memory starts zeroed) | Decision 50; tests never depend on it |

## Real math functions (#87)

| # | Deviation | TP5 | This compiler | Notes |
|---|---|---|---|---|
| D37 | `Exp`, `Ln`, `Sin`, `Cos`, `ArcTan` in 64-bit arithmetic | 48-bit Real (11 to 12 significant digits); `Exp(x)` overflows (205) above about 88.7; `Sin` and `Cos` lose accuracy for large arguments in their own way | Results are accurate to about 1E-15 for moderate arguments (decision 19); `Exp` overflows (205) above 709.78; `Sin` and `Cos` are accurate while the argument is below about 1E9; the zero or negative argument of `Ln` is error 207 as in TP5 | Tests print at most the digits both representations agree on. System also declares helper names (`ExpReal`, `Reduce`, `Ln2`, ...) that a program can see, as it can the other System helpers |

## Compiler switches (SWAG sweep)

| # | Deviation | TP5 | This compiler | Notes |
|---|---|---|---|---|
| D38 | `{$A B D F L O S V}` switches and `{$M stack,heapmin,heapmax}` | Change code generation, the stack and the heap | Checked as TP checks them (stack 1024 to 65520, heap sizes 0 to 655360) and ignored | `{$N+}` and `{$E+}`, which TP accepts, stay error 200 (8087 code is omitted); their `-` forms are accepted and ignored; other letters are error 17 as in TP, wherever they stand in a list (an `E+` before a bad letter does not hide it). Odd forms TP tolerates (`{$A++}`, `{$A+B-}`, a `;` between the `$M` numbers) are error 17 here |
| D40 | `SetTextBuf(f, buf[, size])` | Gives the file a program-supplied buffer | Checked as TP checks it (a Text variable, any variable, an optional Word size) and ignored: buffering is internal to System. Output is the same unless the program relies on the buffer's contents | `lang/settextbuf_with_and_without_size_is_ignored` |
| D41 | `LastMode` at start | `LastMode` is the video mode before the program started | A Word variable that starts as 3 (`CO80`); `TextMode(m)` sets it to `m` (see D48) | `lang/swag_crt_lastmode` |
| D42 | A file assigned the empty name (`Assign(f, '')`) | Reset, Rewrite and Append bind it to DOS standard input or output. Reads are block-buffered, so a second reader of standard input (Input, or `Reset` again) finds the data already consumed; under Crt the output goes through DOS, so Crt's `WhereX`/`WhereY` do not see it, and typed input is echoed by DOS | It is the console exactly as `Input` and `Output` are: one line at a time from standard input (Crt's line editor under Crt), output through the same writer as `Output` (Crt's screen and cursor under Crt). Close does not close the console | `lang/assign_empty_*` test only the cases that do not depend on the buffering: one reader, whole lines, numbers, Eof, Eoln, wrong-direction errors 104/105 |

## Dos file names (#88)

| # | Deviation | TP5 | This compiler | Notes |
|---|---|---|---|---|
| D39 | `FExpand`, `GetDir`, `DosError` on a flat virtual file system | `FExpand` and `GetDir` use the real current directory of each drive (the oracle runs with `C:\` current, so the corpus cannot tell); `DosError` holds whatever the unit data held at start | The current directory of every drive is its root; `GetDir(d)` is the letter `Chr(64 + d)` (0 is `C`) and `:\`, with no check that the drive exists; `DosError` is 0 and nothing sets it. `FExpand` upper-cases ASCII only | `FindFirst`, `FindNext`, `FSearch`, `DiskSize`, `DiskFree`, `ChDir`, `GetFAttr`, `GetFTime` are omitted (see subset.md) |

## Diagnostics order and fidelity (#94)

TP is a one-pass compiler that scans one token at a time; this compiler parses the whole file,
then checks it. Scanning errors (illegal character, bad number, unterminated string, line too
long, bad directive, missing include) are deferred to the point where the parser reaches them,
so a syntax error earlier in the file wins as in TP, and text after the final `end.` is never
scanned. The rows below are what remains.

| # | Deviation | TP5 | This compiler | Notes |
|---|---|---|---|---|
| D43 | A syntax error after a semantic error | The semantic error (3, 26, 44, 88, ...) at its own line, because checking happens while reading | The syntax error (85, 89, 93, 113, ...) wins: parsing finishes before checking starts | Six of the 134 SWAG both-reject files (`ansi/0014`, `findrepl/0006`, `math/0075`, `math/0083`, `textedit/0009`, `memory/0021`). Fixing it needs the checker to run over the part of the program parsed before the error (error recovery in the parser); the checker is at its budget. `new[q)`: TP 88, ours 93 is the same cause. Tests: none (PENDING would need the fix) |
| D44 | `extended`, `double`, `single`, `comp` without `{$N+}` | Declaring them is accepted; the first expression that uses one is error 116 *Must be in 8087 mode* | Error 200 at the declaration (8087 types are omitted) | `{$N+}` is also 200 here. `math/0104` (TP 116 at the first use) |
| D45 | `{$L file.OBJ}` | Links the object file; error 15 at the end of the program when the file is missing (the usual case in SWAG) | Error 17 at the directive | The external routines it supplies cannot be linked here. Two SWAG files |
| D46 | A program that TP rejects for a feature that this compiler omits | The error of the TP rule (3, 122, 11, 42, 26, ...) | Error 200 at the omitted feature when it comes first in the file | 20 of 134: the omitted feature (typed file, label, `{$N+}`, ...) is accepted by TP, so no ordering fix is possible |

An include whose name has a directory part (`{$I c:\ra\x.inc}`) that cannot be found is error 14
as in TP when the directory is missing; TP says 15 when the directory exists and the file does
not, which a flat file table cannot tell apart (14 is reported).

## Cheap standard routines (#99)

| # | Deviation | TP5 | This compiler | Notes |
|---|---|---|---|---|
| D47 | `GetEnv`, `EnvCount`, `EnvStr`; `DosVersion` | The program sees the DOS environment (the oracle's holds `PATH`, `COMSPEC` and one more; `EnvCount` is 3, `EnvStr(1)` is `COMSPEC=Z:\COMMAND.COM`); `DosVersion` is `$0005` (DOS 5.0) under the oracle | There is no environment: `GetEnv` returns `''`, `EnvCount` is 0, `EnvStr(i)` is `''` for every `i`. `DosVersion` returns 5, as the oracle does | `lang/dos_getenv_and_dosversion` (the agreeing cases: an unset name, `EnvStr` out of range); `compiler/tests/cheap_routines.rs` |
| D48 | `TextMode(m)` for modes with another screen size | `CO40` and `BW40` give a 40-column screen (`WindMax` column 39), `Font8x8` added to a mode gives 43 or 50 lines | `TextMode` always leaves the 80x25 screen: it clears it, sets `TextAttr` to 7, the window to the whole screen and the cursor to the top left, and sets `LastMode` to `m` (any `m`), as TP does for `BW80`, `CO80` and `Mono` | `lang/crt_textmode_*` for the modes that agree; `compiler/tests/cheap_routines.rs` for the others |
| D49 | (retired by #101) `Erase` and `Rename` omitted | `Erase(f)` and `Rename(f, name)` delete and rename a file | They exist, on `Text` files: see D50 | `Erase` and `Rename` were 74 and 38 of the SWAG files |
| D51 | Names given to `Erase` and `Rename` | DOS resolves wildcards (`*`, `?`), silently truncates names longer than 8.3, treats device names as devices, and reports a missing or busy directory as 2, 3 or 5 | A name must be a valid 8.3 name of the flat file system: a wildcard or over-long name is error 2, a name with `\`, `/` or `:` is error 3 (the new name of `Rename`: `:` is 3, `\` or `/` is 5, as TP); device names are ordinary names (D35) | Decision 55. Everything else matches TP: erasing or renaming an open file works (0) and the open file follows it |

## Additions (#100)

| # | Deviation | TP5 | This compiler | Notes |
|---|---|---|---|---|
| D50 | The `Utils` unit | No such unit: `uses Utils` is error 15 unless the program supplies its own `UTILS.PAS` | An embedded unit (`runtime/utils.pas`) with `Power`, `Tan`, `ArcSin`, `ArcCos`, `Log10`, `Hex`, `TrimLeft`, `TrimRight`, `Trim`, `UpperCase`, `LowerCase`, `Min`, `Max`, `LeadingZero` | An addition, not a change of TP behaviour (decision 53). The unit is plain TP 5.0 Pascal, so the oracle compiles the same file (`NAME.dirs` names `runtime/utils.pas`) and the tests `tests/*/utils_*` have TP 5.0 output. A program that has its own `UTILS.PAS` next to it still gets the embedded unit, as for `Crt` and `Dos` |
