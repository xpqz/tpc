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
