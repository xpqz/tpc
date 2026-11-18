# Turbo Pascal error numbers (reference)

Source: Turbo Pascal 5.5 Norton Guide (third-party conversion of the Borland manual),
https://discmaster.textfiles.com/file/9297/Cream%20of%20the%20Crop%201.iso/CAD/G2P55.ARJ/G2P55.NG/ng2f7d0.html
and neighbouring entries. The guide states the numbers vary only slightly from 4.0.
TP 5.0 is the target; where 5.0 and 5.5 differ (5.5 adds objects), 5.5-only errors are not
used. Numbers missing from the source (108-111, 115, 119, 125, 145) are not used.
Diagnostics in `compiler/src/diag.rs` use these numbers and messages. Errors for omitted
features use a distinct "not supported" code chosen in docs/decisions.md.

## Compiler errors

| N | Message |
|---|---|
| 1 | Out of memory |
| 2 | Identifier expected |
| 3 | Unknown identifier |
| 4 | Duplicate identifier |
| 5 | Syntax error |
| 6 | Error in real constant |
| 7 | Error in integer constant |
| 8 | String constant exceeds line |
| 9 | Too many nested files |
| 10 | Unexpected end of source |
| 11 | Line too long |
| 12 | Type identifier expected |
| 13 | Too many open files |
| 14 | Invalid file name |
| 15 | File not found |
| 16 | Disk full |
| 17 | Invalid compiler directive |
| 18 | Too many files |
| 19 | Undefined type in pointer definition |
| 20 | Variable identifier expected |
| 21 | Error in type |
| 22 | Structure too large |
| 23 | Set base type out of range |
| 24 | File components may not be files |
| 25 | Invalid string length |
| 26 | Type mismatch |
| 27 | Invalid subrange base type |
| 28 | Lower bound greater than upper bound |
| 29 | Ordinal type expected |
| 30 | Integer constant expected |
| 31 | Constant expected |
| 32 | Integer or real constant expected |
| 33 | Type identifier expected |
| 34 | Invalid function result type |
| 35 | Label identifier expected |
| 36 | BEGIN expected |
| 37 | END expected |
| 38 | Integer expression expected |
| 39 | Ordinal expression expected |
| 40 | Boolean expression expected |
| 41 | Operand types do not match operator |
| 42 | Error in expression |
| 43 | Illegal assignment |
| 44 | Field identifier expected |
| 45 | Object file too large |
| 46 | Undefined external |
| 47 | Invalid object file record |
| 48 | Code segment too large |
| 49 | Data segment too large |
| 50 | DO expected |
| 51 | Invalid PUBLIC definition |
| 52 | Invalid EXTRN definition |
| 53 | Too many EXTRN definitions |
| 54 | OF expected |
| 55 | INTERFACE expected |
| 56 | Invalid relocatable reference |
| 57 | THEN expected |
| 58 | TO or DOWNTO expected |
| 59 | Undefined forward |
| 60 | Too many procedures |
| 61 | Invalid typecast |
| 62 | Division by zero |
| 63 | Invalid file type |
| 64 | Cannot Read or Write variables of this type |
| 65 | Pointer variable expected |
| 66 | String variable expected |
| 67 | String expression expected |
| 68 | Circular unit reference |
| 69 | Unit name mismatch |
| 70 | Unit version mismatch |
| 71 | Duplicate unit name |
| 72 | Unit file format error |
| 73 | IMPLEMENTATION expected |
| 74 | Constant and case types do not match |
| 75 | Record variable expected |
| 76 | Constant out of range |
| 77 | File variable expected |
| 78 | Pointer expression expected |
| 79 | Integer or real expression expected |
| 80 | Label not within current block |
| 81 | Label already defined |
| 82 | Undefined label in preceding statement part |
| 83 | Invalid @ argument |
| 84 | UNIT expected |
| 85 | ";" expected |
| 86 | ":" expected |
| 87 | "," expected |
| 88 | "(" expected |
| 89 | ")" expected |
| 90 | "=" expected |
| 91 | ":=" expected |
| 92 | "[" or "(." expected |
| 93 | "]" or ".)" expected |
| 94 | "." expected |
| 95 | ".." expected |
| 96 | Too many variables |
| 97 | Invalid FOR control variable |
| 98 | Integer variable expected |
| 99 | Files and procedure types are not allowed here |
| 100 | String length mismatch |
| 101 | Invalid ordering of fields |
| 102 | String constant expected |
| 103 | Integer or real variable expected |
| 104 | Ordinal variable expected |
| 105 | INLINE error |
| 106 | Character expression expected |
| 107 | Too many relocation items |
| 112 | CASE constant out of range |
| 113 | Error in statement |
| 114 | Cannot call an interrupt procedure |
| 116 | Must be in 8087 mode to compile this |
| 117 | Target address not found |
| 118 | Include files are not allowed here |
| 120 | NIL expected |
| 121 | Invalid qualifier |
| 122 | Invalid variable reference |
| 123 | Too many symbols |
| 124 | Statement part too large |
| 126 | Files must be var parameters |
| 127 | Too many conditional symbols |
| 128 | Misplaced conditional directive |
| 129 | ENDIF directive missing |
| 130 | Error in initial conditional defines |
| 131 | Header does not match previous definition |
| 132 | Critical disk error |
| 133 | Cannot evaluate this expression |
| 134 | Expression incorrectly terminated |
| 135 | Invalid format specifier |
| 136 | Invalid indirect reference |
| 137 | Structured variables are not allowed here |
| 138 | Cannot evaluate without System unit |
| 139 | Cannot access this symbol |
| 140 | Invalid floating-point operation |
| 141 | Cannot compile overlays to memory |
| 142 | Procedure or function variable expected |
| 143 | Invalid procedure or function reference |
| 144 | Cannot overlay this unit |

## Runtime errors

| N | Message |
|---|---|
| 2 | File not found |
| 3 | Path not found |
| 4 | Too many open files |
| 5 | File access denied |
| 6 | Invalid file handle |
| 12 | Invalid file access code |
| 100 | Disk read error |
| 101 | Disk write error |
| 102 | File not assigned |
| 103 | File not open |
| 104 | File not open for input |
| 105 | File not open for output |
| 106 | Invalid numeric format |
| 200 | Division by zero |
| 201 | Range check error |
| 202 | Stack overflow error |
| 203 | Heap overflow error |
| 204 | Invalid pointer operation |
| 205 | Floating point overflow |
| 206 | Floating point underflow |
| 207 | Invalid floating point operation |
