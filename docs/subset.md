# Language subset

Source: `plan.md` §2. Reference semantics are Turbo Pascal 5.0. TP5 has units, procedural
types, typed constants, `Mark`/`Release`, and no `object`. Specific behaviours are checked
against the TP5 manual and `fpc -Mtp`; every deviation is recorded in `docs/deviations.md`.

## Supported

| Area | Constructs |
|---|---|
| Types | `Byte`, `ShortInt`, `Integer` (16-bit), `Word`, `LongInt`, `Boolean`, `Char`, `Real`, `String[n]`, `String`, enums, subranges, arrays, records (including variant parts), sets (base type of at most 256 values), pointers, procedural types |
| Statements | `begin/end`, `if`, `case`, `while`, `repeat`, `for`, `with`, assignment, procedure call, `Exit`, `Halt` |
| Declarations | `const`, typed `const`, `type`, `var`, nested procedures and functions, `forward`, units (`interface` / `implementation` / `initialization`) |
| Text files only | `Assign`, `Reset`, `Rewrite`, `Append`, `Close`, `Read`, `ReadLn`, `Write`, `WriteLn`, `Eof`, `Eoln`, `IOResult`, `{$I+/-}` |
| Directives | `{$F+}` / `{$F-}` (accepted, no effect, deviation D11), `{$R}`, `{$I file}` (include), `{$IFDEF}`, `{$IFNDEF}`, `{$ELSE}`, `{$ENDIF}`, `{$DEFINE}` |

Intrinsics and runtime routines named in later phases of `plan.md` §5 (not part of §2):
`Ord`, `Chr`, `Succ`, `Pred`, `Odd`, `Abs`, `Sqr`, `Sqrt`, `Round`, `Trunc`, `Inc`, `Dec`,
`Val`, `Str`, `UpCase`, `ParamStr`, `ParamCount` (empty), `Length`, `Copy`, `Pos`, `Delete`,
`Insert`, `Concat`, `New`, `Dispose`, `GetMem`, `Mark`, `Release`, `MemAvail`, `MaxAvail`,
`Include`, `Exclude`; `Crt` (`ClrScr`, `GotoXY`, `WhereX`, `WhereY`, `TextColor`,
`TextBackground`, `ReadKey`, `KeyPressed`, `Delay`, `Window`, `ClrEol`, `Sound`/`NoSound` as
no-ops); `Dos` subset (`GetDate`, `GetTime`).

## Deliberately omitted

Each yields a specific error, never silent misbehaviour. The error numbers are to be
assigned by the diagnostics table (`plan.md` §5 Phase 1 item 6, Phase 8 item 1): the plan
defines a "distinct 'not supported' error" but no numbers.

| Omitted feature | Error produced |
|---|---|
| `goto` and labels (`label` section) | "not supported" error (number TBD) |
| Inline assembler (`asm`) | "not supported" error (number TBD) |
| `Absolute` | "not supported" error (number TBD) |
| `inline` | "not supported" error (number TBD) |
| Interrupt procedures | "not supported" error (number TBD) |
| Overlays | "not supported" error (number TBD) |
| BGI (`Graph`) | "not supported" error (number TBD) |
| 8087 types `Single`, `Double`, `Extended`, `Comp`; `{$N}` / `{$E}` | "not supported" error (number TBD) |
| Typed and untyped files (`file of T`, `file`) | "not supported" error (number TBD) |
| `Seg`, `Ofs`, `Ptr` | "not supported" error (number TBD) |
| Binary `.TPU` | "not supported" error (number TBD) |
| Untyped `var` parameters (`procedure P(var a)`) | error 200 "Feature not supported" |
| `Chain`, `Execute` | "not supported" error (number TBD) |

Error numbers and wording for omitted features are an open item for the Phase 1
diagnostics issue; they are not decided here.

## Accepted deviations

Also in `docs/deviations.md`.

- `Real` is IEEE f64 (8 bytes), not the 6-byte software real.
- Pointers are 32-bit offsets into linear memory. `nil` is 0.
- `Integer` stays 16-bit; `LongInt` is 32-bit.
- Assigning a nested routine to a procedural variable is a compile error, as in TP.

## Open questions

- Which listed `{$...}` switches beyond the ones named (`{$I+/-}`, `{$R}`, `{$D+}` from
  `plan.md` §4.10) are accepted, ignored, or rejected (for example `{$S}`, `{$V}`, `{$B}`)?
  Not stated; ABI open question 16 covers the `{$R}` default.
- Whether `Write(x:w:d)` on `Integer` / `String`, `Str`, `Val` and `Concat` are fully
  supported: named in §5 but not in §2.
- Whether `ParamStr` / `ParamCount` count as supported (§5 Phase 4 item 4 says "empty").
- Meaning of "sets (base type of at most 256 values)": base types such as `Char`, `Byte`,
  enums, subranges are assumed; an out-of-range base is a compile error with no number yet.
- Whether `Mark` / `Release` and `GetMem` / `FreeMem` are in the supported list: §2 mentions
  `Mark`/`Release` in the TP reference description, §5 Phase 4 implements them.
