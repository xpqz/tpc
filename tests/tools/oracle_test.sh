#!/bin/sh
# Tests for tools/oracle.sh against two small corpora (needs the real TP 5.0 under DOSBox-X):
# `good` agrees with TP5 everywhere, `bad` has one wrong expectation in each category.
ROOT=$(cd "$(dirname "$0")/../.." && pwd)
FX="$ROOT/tests/tools/fixtures/oracle"
FAILS=0

fail() { FAILS=$((FAILS + 1)); echo "FAIL $1: $2"; }

out=$(ORACLE_ROOT="$FX/good" "$ROOT/tools/oracle.sh" 2>&1)
rc=$?
if [ "$rc" -eq 0 ] && [ -z "$out" ]; then echo "ok   agreeing corpus passes, including units, stdin and N LINE FILE"
else fail "agreeing corpus" "rc=$rc out='$out'"; fi

out=$(ORACLE_ROOT="$FX/bad" "$ROOT/tools/oracle.sh" 2>&1)
rc=$?
[ "$rc" -eq 1 ] && echo "ok   disagreeing corpus fails" || fail "disagreeing corpus" "rc=$rc"
for want in "DIFF tests/lang/unit_use.pas" "DIFF tests/lang/stdin_stdout.pas" \
    "DIFF tests/rt/runtime_error_two.pas" "DIFF tests/neg/error_in_unit.pas: tp5 '3 7 BAD.PAS'"; do
    if printf '%s\n' "$out" | grep -Fxq "$want"; then echo "ok   reports '$want'"
    else fail "report" "missing '$want' in: $out"; fi
done

[ "$FAILS" -eq 0 ]
