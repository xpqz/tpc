#!/bin/sh
# Re-derives every corpus expectation with the real TP 5.0 (tools/tp5.sh) and reports
# differences. Covers tests/{lang,abi,neg} and the per-feature fixture sets under
# compiler/tests/fixtures/*/{lang,neg}. .out must equal TP5 output; .err must equal TP5's
# "N LINE", except unsupported_* (TP5 accepts or differs by design).
cd "$(dirname "$0")/.." || exit 2
bad=0
LANG="tests/lang tests/abi $(ls -d compiler/tests/fixtures/*/lang | grep -v fixtures/lang)"
NEG="tests/neg $(ls -d compiler/tests/fixtures/*/neg | grep -v fixtures/neg)"
for d in $LANG; do
  for f in "$d"/*.pas; do
    tools/tp5.sh "$f" > tmp/oracle.out 2>&1
    cmp -s tmp/oracle.out "${f%.pas}.out" || { echo "DIFF $f"; bad=1; }
  done
done
for f in $(for d in $NEG; do ls "$d"/*.pas; done); do
  case "$f" in */unsupported_*) continue ;; esac
  got=$(tools/tp5.sh "$f" | sed -E 's/^P\.PAS\(([0-9]+)\): Error ([0-9]+).*/\2 \1/')
  [ "$got" = "$(cat "${f%.pas}.err")" ] || { echo "DIFF $f: tp5 '$got'"; bad=1; }
done
exit $bad
