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
