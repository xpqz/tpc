# SWAG validation sweep

Every `.pas` file of the SWAG archive (4,188 files; cloned at `~/work/crust/swag`, never
committed here, referred to by path relative to the archive) is compiled and run by the real
TP 5.0 (`tools/tp5.sh`) and by `tp5c` (debug build, `node tests/run.mjs`) and the results are
classified. Tool: `tools/swag.sh` (`tools/swag.py`, stdlib only). Results: one line per file in
`~/work/crust/swag-results/results.tsv` (path, content hash, class, TP result, ours, note,
tags), outputs of both sides in `swag-results/out/HASH.{tp,ours}`. `tools/swag.sh run` is
resumable (a file is skipped when its path and hash are cached; `--redo CLASS` repeats a
class, `--sample N` runs a stratified sample); `tools/swag.sh report` prints the tables below.

Method. A text scan (strings and comments blanked) filters out programs that cannot run
here, before any DOSBox run. The rest run with an empty stdin, both sides with LF line ends
(`tp5.sh` adds a CR to every line; SWAG files are CRLF and the doubled CR makes some real
programs misbehave), a 20 s limit per run, no key script (a Crt program that blocks in
`ReadKey` ends with `exit 255` on both sides) and the Crt screen dump when the program uses
Crt. Runtime-error addresses are normalised as in `tools/oracle.sh`.

The sample of 200 files (`--sample 200`) was run first to calibrate; it showed that about 90 %
of the archive never reaches a run (see "Not portable"), which is why the full sweep takes
minutes rather than hours. The full run is the one reported here.

## Counts per class

| Class | Files | Meaning |
|---|---:|---|
| Not portable (pre-filter, "skip" and "not-portable" together) | 3,775 | never run, see below |
| TP rejects | 134 | TP gives a compile error (ours rejects too, except the 4 below) |
| omitted 200 | 71 | TP accepts, ours gives 200 (omitted feature) |
| gap | 64 | TP accepts, ours gives another compile error |
| same | 67 | both run, output equal |
| differ | 12 | both run, output differs |
| needs input | 35 | a Crt program blocks in `ReadKey` (or TP times out and ours ends `exit 255`); compared on compile only |
| timeout | 22 | a side timed out at 20 s (TP in 21, ours in 7; see Timeouts) |
| TP rejects, ours accepts | 4 | accepting invalid code |
| TP no dump | 4 | Crt program whose run left no screen dump in TP (`tools/tp5.sh` prints nothing); ours gives a compile error in all 4 |

Of the 413 programs that reach a run, 67 + 35 compile and run identically (or to the point
of waiting for input) on both sides, and 12 differ for reasons that are not compiler bugs.
Because tp5c reports only the first error per file, a file blocked by one gap may hide
others: the gap counts are a lower bound of what is missing.

## Not portable (never counted as bugs)

3,775 of 4,188 files are filtered. First the structural skips, one reason each:

| Reason | Files |
|---|---:|
| unit without a program (first token is `unit`) | 955 |
| fragment (no `end.`: snippets, prose, procedures) | 886 |
| Delphi (`delphi/` directory, `{$mode}`, `{$apptype}`) | 464 |
| empty after removing comments | 61 |
| over 60,000 bytes | 12 |

Then 1,397 programs with a DOS or TP 5.5+ feature (a file counts under every reason it
matches, so the rows add up to more than 1,397). A `uses` clause is only read when it is a
plain identifier list ending in `;`; the word scan matches an identifier anywhere, so a few
programs that name an ordinary variable `Port` or `Keep` are filtered too (about 1 in 10 of
the single-reason `port`/`keep` rows).

| Reason | Files | Reason | Files |
|---|---:|---|---:|
| unit other than System/Crt/Dos | 961 | `msdos` | 74 |
| `asm` | 621 | `keep` | 73 |
| `$B800`/`$A000`/`$B000` addresses | 333 | `constructor` | 70 |
| `ptr` | 256 | `memw` | 58 |
| `port` | 244 | `exec` | 51 |
| `mem` | 243 | `swapvectors` | 47 |
| `registers` | 235 | `destructor` | 30 |
| `absolute` | 214 | `portw` | 16 |
| `seg` | 205 | `dseg`, `sseg`, `cseg`, `sptr`, `meml` | 44 |
| `ofs`, `intr` | 176 each | EMS, XMS, DPMI identifiers | 109 |
| `object` | 157 | `inline` | 101 |
| `interrupt` | 116 | `getintvec`, `setintvec` | 96, 107 |
| `virtual` | 90 | | |

Units named most often: `Graph` (about 90), Turbo Vision (`Objects`, `Views`, `App`,
`Drivers`, `Menus`, `Dialogs`, `MsgBox`, `StdDlg`; 72 files), `Strings` (39), `Printer` (24),
Windows units (`WinCrt`, `WinProcs`, `WinTypes`, `OWindows`; about 60), `Overlay` (3), and
hundreds of third-party or local units (`OpCrt`, `TPString`, ...).

## Omitted features that block real programs (71 files)

| Cause | Files | Examples (archive paths) |
|---|---:|---|
| typed or untyped `file` | 53 | `ansi/0003.pas`, `archives/0001.pas`, `files/0034.pas` |
| a compiler switch letter other than F, R, I | 16 | `datetime/0057.pas`, `drives/0100.pas` |
| `label` / `goto` | 2 | `menu/0014.pas`, `misc/0178.pas` |

The switch rows split into harmless switches that TP accepts and ignores in practice (`S`, `V`,
`B`, `D`, `A`, `L`, `O`, `R-`: 7, 6, 4, 3, 3, 2, 2 occurrences) and real omissions (`N+`, `E+`:
6 and 3). Every directive is reported at its `{`, so one file shows several letters. Of the
16 files, those without `N` or `E` (about 10) fail only because of switches that change
nothing here (proposed issue 1).

## Gaps: TP accepts, ours gives another error (64 files)

First error of each file, grouped. Reproducers are tests (`tests/lang/swag_*`, `tests/neg`),
all listed in the PENDING files, with expectations from `tools/tp5.sh`.

| Cause | Files | Ours | Examples | Reproducer |
|---|---:|---|---|---|
| Dos types `DateTime`, `SearchRec`, `PathStr` (13) and Dos routines `DiskSize`, `ChDir`, `GetDir`, `FSearch` (7) | 20 | 3 Unknown identifier | `datetime/0014.pas`, `dirs/0004.pas`, `drives/0007.pas`, `dirs/0012.pas` | `swag_dos_types` |
| `Randomize`, `Random`, `RandSeed` | 13 | 3 | `datatype/0021.pas`, `mail/0032.pas`, `sorting/0036.pas`, `encrypt/0019.pas` | `swag_random_seeded` |
| `{$M stack,min,max}` and `(*$M ...*)` | 7 | 17 Invalid compiler directive | `comm/0072.pas`, `copymove/0005.pas`, `textfile/0062.pas` | `swag_m_directive`, `swag_m_directive_comment_form` |
| `FillChar`, `Move` | 7 | 3 | `math/0014.pas`, `math/0018.pas`, `strings/0026.pas` | `swag_fillchar_and_move` |
| `Exp`, `Sin`, `Cos` | 7 | 3 | `math/0048.pas`, `math/0050.pas`, `math/0092.pas` | `swag_exp_sin_cos_ln_arctan` |
| `SetTextBuf` | 3 | 3 | `findrepl/0007.pas`, `pointers/0025.pas` | none (needs a buffer semantics test, see issue 7) |
| `Val` into `Word` variables | 2 | 98 Integer variable expected | `datetime/0038.pas`, `numbers/0047.pas` | `swag_val_into_word_variables` |
| indexing the result of a `string` function (`Name[0] := ...`) | 2 | 88 `(` expected; the same source with a body that assigns elements panics the compiler | `encrypt/0005.pas`, `numbers/0013.pas` | `swag_string_function_result_index` |
| hex literal above `$7FFFFFFF` (`$80000000`) in a `LongInt` constant | 1 | 89 `)` expected | `numbers/0002.pas` | `swag_hex_literal_above_longint` |
| `MaxInt` | 1 | 3 | `math/0030.pas` | `swag_maxint` |
| Crt variable `LastMode` | 1 | 3 | `egavga/0056.pas` | `swag_crt_lastmode` |

Run-time gaps found among programs that compile:

| Cause | Examples | Reproducer |
|---|---|---|
| `Assign(f, '')` then `Reset`/`ReadLn`: TP binds the file to standard input (`Rewrite` to standard output); ours: runtime error 2 | `misc/0021.pas`, `misc/0159.pas`, `findrepl/0008.pas` | `swag_assign_empty_name_is_stdin` |

## Accepting invalid code: TP rejects, ours accepts (4 files)

| Cause | Examples | Reproducer |
|---|---|---|
| a source line longer than 126 characters (TP error 11); in SWAG these are UTF-8 art or long string lines, whose byte length counts | `math/0119.pas`, `math/0125.pas`, `textfile/0065.pas` | `tests/neg/swag_line_longer_than_126` |
| TP error 49 *Data segment too large* on `textfile/0061.pas` (`array[1..16000] of Pointer` plus Crt and the oracle's ScrDump/KeyFeed units); not reproduced standalone: the oracle's helper units add to the data segment, so this may not be a deviation of the program itself | `textfile/0061.pas` | none |

## Outputs that differ (12 files, 4 of them the `Assign` bug): no compiler bug found

| Files | Cause |
|---|---|
| `datatype/0012.pas`, `math/0057.pas`, `math/0071.pas`, `encrypt/0016.pas`, `strings/0100.pas` | the program reads a variable it never assigned (uninitialised global, local or an empty `ReadLn` result): TP shows memory garbage, ours zero. Not specified by the language; no deviation entry exists yet (proposed issue 8) |
| `math/0112.pas` | `Real` is f64 here and a 6-byte real in TP: TP prints `1234567.89120000` for `1234567.891234` (deviations: Real) |
| `pointers/0006.pas`, `datatype/0001.pas` | `MemAvail`/`MaxAvail` values (the heap differs by design) |
| `findrepl/0008.pas`, `misc/0021.pas`, `misc/0159.pas`, `ansi/0017.pas` | the `Assign(f,'')` bug above (ansi/0017 differs in which open fails first) |

## Timeouts and input

22 timeouts: TP timed out in 21 and ours in 6 of the same (programs that retry reading
input until a valid answer, or wait for a keypress the key script does not model;
`entry/0016.pas`, `math/0031.pas`, `screen/0009.pas`, `sound/0108.pas`); 11 of TP's timeouts
have a compile error on our side anyway. Two where TP times out and ours aborts: `entry/0030.pas`
and `pointers/0013.pas` exhaust node's 4 GB heap (an allocation loop; TP's heap would end in
runtime error 203, ours grows the linear memory without bound: proposed issue 13, to be
checked). Two where only TP is slow or hangs: `math/0028.pas` is a prime search that needs
more than 20 s on the emulated 8086 and ends at once here; `pointers/0003.pas` reads an
uninitialised pointer (TP hangs, ours writes through nil). One where only ours loops:
`comm/0039.pas` writes to `COM2`: TP ends with runtime error 2 (no such device), ours opens a
file of that name (proposed issue 12).

35 needs-input programs wait in `ReadKey` and end `exit 255` (both sides): `color/0020.pas`,
`datetime/0003.pas`, `menu/0001.pas`, `textedit/0002.pas`. They compile and run to the same
first screen; a key script per program would compare more.

## Both reject (134 files)

TP errors: 17 invalid compiler directive 27 (switches TP5 lacks, e.g. `{$X+}`; ours says 200 for
most), 36 *BEGIN expected* 26 (prose), 3 unknown identifier 16, 122 *invalid value
parameter* 13, 11 line too long 12, 15 file not found 8. Of the 134, 49 agree on error number
and line; the others differ in number: 84 programs, most of them 200 (omitted features hit
before TP's error: 27 + 7 + 4 + 3 + ...), and TP 36 against ours 5/7/8/17 (12; prose files,
where the first token is not Pascal). Diagnostics numbers in `tests/neg` are exact, so the
12 *BEGIN expected* disagreements show how far recovery of unusual tokens is from TP.

Re-run after #94 (`tools/swag.sh` cache, tp5c from this branch, error number and line must both
match): 72 of 134 agreed before, 103 after. Fixed: error 17 against 200 for a `{$A+,...,G+}` list
whose bad letter follows `N`/`E` (23 files), and TP 36/11 and others against our scanner errors
(about 8 files; scanning errors are now deferred, see D43 to D46 in deviations.md). The 31 left are 20
omitted features that TP accepts (D46), 6 syntax errors after a semantic one (D43), and
`{$L}`, `extended`, `FindFirst`, and a data segment overflow.

## Proposed bug issues (for the lead to file)

1. **Accept the harmless compiler switches.** `{$A+ B- D- L- S- V- O-}` and the other letters
   TP accepts and this implementation ignores (not `N`, `E`, which stay omitted) give error
   200 today; 10 SWAG programs stop on them (`datetime/0057.pas`, `drives/0100.pas`). Test
   `swag_ignored_compiler_switches`.
2. **`{$M stack,min,max}` directive, brace and `(*$M*)` form, is error 17.** Seven programs;
   accept and ignore (deviation entry). Tests `swag_m_directive`,
   `swag_m_directive_comment_form`.
3. **Random numbers: `Random`, `Randomize`, `RandSeed`.** 13 programs. TP's generator is a
   linear congruential generator, so `RandSeed := 5; Random(100)` is 82 in TP; match it
   (test `swag_random_seeded`).
4. **`FillChar`, `Move`, and `Exp`, `Sin`, `Cos`, `Ln`, `ArcTan`, `Pi`, `MaxInt`.** 7 + 7 + 1
   programs. Tests `swag_fillchar_and_move`, `swag_exp_sin_cos_ln_arctan`, `swag_maxint`.
5. **Dos unit lacks `DateTime`, `SearchRec`, `PathStr` and the directory routines.** 20
   programs: `PackTime`, `UnpackTime`, `FindFirst`, `FindNext`, `GetDir`, `ChDir`, `DiskSize`,
   `DiskFree`, `FSearch`, `FExpand`. Test `swag_dos_types` covers the types.
6. **A string function result cannot be indexed, and indexed assignment panics the
   compiler.** `Name[0] := #3` inside `function Name: string` gives error 88 in some
   positions and a panic at `compiler/src/codegen.rs:1086` for `swag_string_function_result_index`;
   a panic must never escape the compiler. Highest priority of the list.
7. **Smaller language gaps.** `Val` into `Word` variables (error 98, 2 programs); hex
   literals from `$80000000` to `$FFFFFFFF` as `LongInt` values (wrap negative; error 89);
   Crt `LastMode`; `SetTextBuf`. Tests `swag_val_into_word_variables`,
   `swag_hex_literal_above_longint`, `swag_crt_lastmode`.
8. **`Assign(f, '')` is standard input or output.** `Reset` on it reads the console, `Rewrite`
   writes it (4 programs, runtime error 2 today). Test `swag_assign_empty_name_is_stdin`
   (only the input side; in the oracle a program that also does `Rewrite` on `Output`
   truncates the redirected output file, so that side has no stable expectation).
9. **Line longer than 126 characters is not rejected (error 11).** Test
   `tests/neg/swag_line_longer_than_126`. Low priority; matters for UTF-8 art in comments.
10. **Document uninitialised variables.** TP shows garbage, ours zero. Add a deviation entry
    so the five SWAG "differ" programs are known and tests do not depend on it.
11. **Typed and untyped files are the largest omission (53 programs, 74 %).** Not a bug; the
    cost/benefit of `file of T`, `BlockRead`, `BlockWrite` belongs to the plan owner.
13. **Endless output exhausted the test runner (resolved, #92).** Not a heap problem: both
    programs loop at end of input writing a prompt, as TP does; the runner now stops at an
    output limit. Original note: `entry/0030.pas`, `pointers/0013.pas`: a
    program that allocates in a loop exhausts the node heap instead of ending with runtime
    error 203 (needs a reduced reproducer; not confirmed against TP, which also timed out
    in 20 s).
12. **Device names.** `Assign(f, 'COM2')` or `LPT1` opens a file here; TP reports error 2 in the
    oracle. Optional.

## Reproducers

All fresh programs; expectation files are the output of `tools/tp5.sh`. Listed in
`tests/lang/PENDING` and `tests/neg/PENDING` until fixed (the driver fails the build when a
pending program passes, so the lists cannot go stale):

`swag_assign_empty_name_is_stdin` (with `.in`), `swag_crt_lastmode`, `swag_dos_types`,
`swag_exp_sin_cos_ln_arctan`, `swag_fillchar_and_move`, `swag_hex_literal_above_longint`,
`swag_ignored_compiler_switches`, `swag_m_directive`, `swag_m_directive_comment_form`,
`swag_maxint`, `swag_random_seeded`, `swag_string_function_result_index`,
`swag_val_into_word_variables` in `tests/lang`; `swag_line_longer_than_126` (`11 2`) in
`tests/neg`.
