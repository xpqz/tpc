#!/bin/sh
# Tests for tools/tp5.sh (needs the real TP 5.0 under DOSBox-X). Set TP5_SH to test another
# copy of the script. Exit status is 1 when any check fails.
ROOT=$(cd "$(dirname "$0")/../.." && pwd)
TP5=${TP5_SH:-$ROOT/tools/tp5.sh}
FX="$ROOT/tests/tools/fixtures/tp5"
FAILS=0

# check NAME FIXTURE EXPECTED_RC EXPECTED_OUTPUT: exact stdout (LF endings) and exit code.
check() {
    got=$("$TP5" "$FX/$2.pas" 2>&1; echo "rc=$?")
    want=$(printf '%s\nrc=%s' "$4" "$3")
    if [ "$got" = "$want" ]; then echo "ok   $1"
    else FAILS=$((FAILS + 1)); echo "FAIL $1: got '$got', want '$want'"; fi
}

NL='
'
check "plain program" hello 0 "Hello, world"
check "unit from the NAME directory is compiled and used" unit_use 0 "hello unit"
check "NAME.in is stdin, stdout is captured" stdin_stdout 0 "[first line]${NL}[second]${NL}[]${NL}[last]"
check "data file in the NAME directory is readable" data_file 0 "alpha${NL}beta"
check "error in a unit names the unit" error_in_unit 1 "BAD.PAS(7): Error 3: Unknown identifier."
check "error in an include names the include" error_in_include 1 \
    "DECLS.INC(2): Error 3: Unknown identifier."
check "error in the main program names P.PAS" error_in_main 1 "P.PAS(3): Error 3: Unknown identifier."
check "circular interface uses" circular_interface 1 "CB.PAS(3): Error 68: Circular unit reference (CA)."

# The run-time error address varies; the code is printed with three digits.
got=$("$TP5" "$FX/runtime_error_two.pas" 2>&1)
case "$got" in
    "Runtime error 002 at "*) echo "ok   one-digit runtime error prints three digits" ;;
    *) FAILS=$((FAILS + 1)); echo "FAIL one-digit runtime error: '$got'" ;;
esac

[ "$FAILS" -eq 0 ]
