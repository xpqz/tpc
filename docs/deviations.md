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
| D18 | Word code in `Val` | `Val(s, v, code)` accepts an `Integer` or `Word` variable for `code` | Only `Integer` (26 reported as 98 for others) | |
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
| D32 | Crt names left out | `TextMode`, `LastMode`, `AssignCrt` and the mode constants `BW40` to `Font8x8` exist | They are error 3 (Unknown identifier) | No text modes: the screen is always 80x25 colour. `CheckBreak`, `CheckEof`, `CheckSnow`, `DirectVideo` exist as variables and change nothing |
| D33 | Sound and ReadLn length | `Sound` makes a tone; a `ReadLn` line is at most 127 characters | `Sound` and `NoSound` do nothing; the line editor takes up to 254 characters (as D29) | |
| D34 | A parameterless function called with arguments inside `if`, `[]`, `for` bounds or `case` | Error 40, 93, 58 or 54 (the next token the construct expects) | Error 85 | Matching needs an expected-token context through `expr` (#81); assignment and argument positions match TP |
| D35 | DOS device names in Assign (COM1, LPT1, PRN, AUX, CON, NUL) | Devices | Ordinary file names | Decision 50 |
| D36 | Reading a variable never assigned | Memory contents (garbage) | Zero (memory starts zeroed) | Decision 50; tests never depend on it |

## Real math functions (#87)

| # | Deviation | TP5 | This compiler | Notes |
|---|---|---|---|---|
| D37 | `Exp`, `Ln`, `Sin`, `Cos`, `ArcTan` in 64-bit arithmetic | 48-bit Real (11 to 12 significant digits); `Exp(x)` overflows (205) above about 88.7; `Sin` and `Cos` lose accuracy for large arguments in their own way | Results are accurate to about 1E-15 for moderate arguments (decision 19); `Exp` overflows (205) above 709.78; `Sin` and `Cos` are accurate while the argument is below about 1E9; the zero or negative argument of `Ln` is error 207 as in TP5 | Tests print at most the digits both representations agree on. System also declares helper names (`ExpReal`, `Reduce`, `Ln2`, ...) that a program can see, as it can the other System helpers |
