#!/bin/sh
# Re-derives every corpus expectation with the real TP 5.0 (tools/tp5.sh) and reports
# differences. Covers tests/{lang,abi,neg} and the per-feature fixture sets under
# compiler/tests/fixtures/*/{lang,neg,rt}. Runtime-error programs drop TP5's
# " at XXXX:YYYY." (wherever it is on the line), as the runner prints `Runtime error NNN`
# (three digits, as TP5) only. A program that uses Crt may have a key script NAME.keys; its
# .out is the program's stdout followed by the screen dump (tools/tp5.sh describes both).
# .out must equal TP5 output; .err must equal TP5's "N LINE" ("N LINE FILE" for an error in
# a unit or include), except unsupported_* (TP5 accepts or differs by design). A program
# NAME.pas may have a directory NAME/ of auxiliary files and an input NAME.in (decision 26),
# which tools/tp5.sh picks up.
# ORACLE_ROOT replaces the repository root (the tests of this script use a small corpus).
tools=$(cd "$(dirname "$0")" && pwd)
cd "${ORACLE_ROOT:-$tools/..}" || exit 2
mkdir -p tmp
# Programs that move a 64 KB gap buffer take ~50 s on the emulated 8086.
export TP5_TIMEOUT="${TP5_TIMEOUT:-180}"
scratch=$(mktemp "${TMPDIR:-/tmp}/oracle.XXXXXX")
trap 'rm -f "$scratch"' EXIT
bad=0
LANG="tests/lang tests/abi $(ls -d compiler/tests/fixtures/*/lang | grep -v fixtures/lang)"
NEG="tests/neg $(ls -d compiler/tests/fixtures/*/neg | grep -v fixtures/neg)"
for d in $LANG; do
  for f in "$d"/*.pas; do
    "$tools/tp5.sh" "$f" > "$scratch" 2>&1
    cmp -s "$scratch" "${f%.pas}.out" || { echo "DIFF $f"; bad=1; }
  done
done
RT="tests/rt $(ls -d compiler/tests/fixtures/*/rt 2>/dev/null | grep -v fixtures/rt)"
for d in $RT; do
  for f in "$d"/*.pas; do
    "$tools/tp5.sh" "$f" | sed -E 's/(Runtime error [0-9]+) at [0-9A-F]{4}:[0-9A-F]{4}\./\1/' > "$scratch" 2>&1
    cmp -s "$scratch" "${f%.pas}.out" || { echo "DIFF $f"; bad=1; }
  done
done
for f in $(for d in $NEG; do ls "$d"/*.pas; done); do
  case "$f" in */unsupported_*) continue ;; esac
  got=$("$tools/tp5.sh" "$f" | sed -E -e 's/^P\.PAS\(([0-9]+)\): Error ([0-9]+).*/\2 \1/' \
    -e 's/^([A-Z0-9_]+\.[A-Z]+)\(([0-9]+)\): Error ([0-9]+).*/\3 \2 \1/')
  [ "$got" = "$(cat "${f%.pas}.err")" ] || { echo "DIFF $f: tp5 '$got'"; bad=1; }
done
exit $bad
