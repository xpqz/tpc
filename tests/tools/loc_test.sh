#!/bin/sh
# Tests for tools/loc.sh. Runs from any directory; exit status is the failure count > 0.
ROOT=$(cd "$(dirname "$0")/../.." && pwd)
LOC="$ROOT/tools/loc.sh"
FX="$ROOT/tests/tools/fixtures"
BUD="$FX/budgets"
FAILS=0
PASSES=0

pass() { PASSES=$((PASSES + 1)); echo "ok   $1"; }
fail() { FAILS=$((FAILS + 1)); echo "FAIL $1: $2"; }

# run ROOT BUDGET: sets OUT and RC; loc.sh must exist and be executable.
run() {
    OUT=$("$LOC" "$1" "$2" 2>&1)
    RC=$?
}
has_line() { printf '%s\n' "$OUT" | grep -Fxq -- "$1"; }

# check NAME ROOT BUDGET EXPECTED_RC LINE...: exit code and every exact output line.
check() {
    name=$1 root=$2 budget=$3 want_rc=$4
    shift 4
    run "$root" "$budget"
    if [ "$RC" -ne "$want_rc" ]; then
        fail "$name" "exit $RC, want $want_rc; output: $OUT"
        return
    fi
    for line in "$@"; do
        if ! has_line "$line"; then
            fail "$name" "missing line '$line'; output: $OUT"
            return
        fi
    done
    pass "$name"
}

# absent NAME ROOT BUDGET LINE: re-run, require a report, and require LINE not in it.
absent() {
    run "$2" "$3"
    if ! printf '%s\n' "$OUT" | grep -q '^total: '; then fail "$1" "no report; output: $OUT"
    elif has_line "$4"; then fail "$1" "unexpected line '$4'; output: $OUT"; else pass "$1"; fi
}

if [ -x "$LOC" ]; then pass "tools/loc.sh exists and is executable"
else fail "tools/loc.sh exists and is executable" "$LOC"; fi

C="$FX/counts"

# Exact per-component and total numbers over all comment forms, blank lines, directives.
# Rust 8 = a.rs 5 + nested/b.rs 3 (only_comments.rs 0, notes.txt ignored).
# Pascal 11 = p.pas 11 (empty.pas 0). JS 9 = j.js 7 + no_newline.js 2 (only_comments.js 0).
check "counts: per component and total" "$C" "$BUD/ok.txt" 0 \
    "Rust alpha: 8 / 100" "Pascal beta: 11 / 100" "JS gamma: 9 / 100" "total: 28 / 300"

check "counts: limit equal to count passes" "$C" "$BUD/exact.txt" 0 \
    "Rust alpha: 8 / 8" "Pascal beta: 11 / 11" "JS gamma: 9 / 9" "total: 28 / 28"
absent "counts: no OVER at the limit" "$C" "$BUD/exact.txt" "OVER: total"

check "counts: file paths, mixed languages, dir with unrecognised-in-budget files" \
    "$C" "$BUD/files.txt" 0 "Mixed: 12 / 100" "Other: 1 / 100" "total: 13 / 200"

check "counts: nonexistent budget paths count zero" "$C" "$BUD/missing.txt" 0 \
    "Absent: 0 / 10" "total: 0 / 10"

# Gate behaviour.
check "gate: component over limit fails and names it" "$C" "$BUD/comp_over.txt" 1 \
    "Pascal beta: 11 / 10" "OVER: Pascal beta"
absent "gate: other components not flagged (rust)" "$C" "$BUD/comp_over.txt" "OVER: Rust alpha"
absent "gate: other components not flagged (js)" "$C" "$BUD/comp_over.txt" "OVER: JS gamma"
absent "gate: total not flagged when within limit" "$C" "$BUD/comp_over.txt" "OVER: total"

check "gate: total over limit fails" "$C" "$BUD/total_over.txt" 1 \
    "total: 28 / 27" "OVER: total"
absent "gate: components not flagged when only total over" "$C" "$BUD/total_over.txt" \
    "OVER: Rust alpha"

check "gate: all under limits exits zero" "$C" "$BUD/ok.txt" 0 "total: 28 / 300"
if ! printf '%s\n' "$OUT" | grep -q '^total: '; then fail "gate: no OVER text when all under" "no report"
elif printf '%s\n' "$OUT" | grep -q OVER; then fail "gate: no OVER text when all under" "$OUT"
else pass "gate: no OVER text when all under"; fi

# String and character literals containing comment openers are code.
L="$FX/literals"
check "literals: comment openers inside strings do not start comments" "$L" "$BUD/literals.txt" 0 \
    "Rust: 6 / 100" "Pascal: 8 / 100" "JS: 4 / 100" "total: 18 / 300"

# Rust is counted after rustfmt (max_width = 100); the source file is left untouched.
# unformatted.rs formats to 25 counted lines (pick 7, two 4, wide 11, lead 3); its raw
# non-comment line count is 4, so a counter that skips formatting cannot pass.
U="$FX/fmt"
before=$(cksum "$U/unformatted.rs")
check "fmt: Rust counted after rustfmt" "$U" "$BUD/fmt.txt" 0 \
    "Unformatted: 25 / 100" "total: 25 / 100"
after=$(cksum "$U/unformatted.rs")
if ! printf '%s\n' "$OUT" | grep -q '^total: '; then fail "fmt: source file not modified" "no report"
elif [ "$before" = "$after" ]; then pass "fmt: source file not modified"
else fail "fmt: source file not modified" "$before -> $after"; fi

# Fixture precondition: the 'formatted' Rust fixtures are already rustfmt-clean.
if rustfmt --check --edition 2021 --config max_width=100 \
    "$C/rs/a.rs" "$C/rs/nested/b.rs" "$C/rs/nested/only_comments.rs" "$L/l.rs" >/dev/null 2>&1
then pass "fixtures: formatted Rust is rustfmt-clean"
else fail "fixtures: formatted Rust is rustfmt-clean" "rustfmt --check failed"; fi

# Default arguments: repo root and tools/budget.txt, independent of the working directory.
OUT=$(cd / && "$LOC" 2>&1)
RC=$?
if [ "$RC" -eq 0 ] && printf '%s\n' "$OUT" | grep -Eq '^total: [0-9]+ / 15000$'
then pass "defaults: repo budget passes from any cwd"
else fail "defaults: repo budget passes from any cwd" "exit $RC; output: $OUT"; fi

# The real budget file lists every plan.md section 1 component with its limit.
B="$ROOT/tools/budget.txt"
SRC=compiler/src
EXPECTED=$(cat <<EOT
1100|$SRC/diag.rs $SRC/driver.rs $SRC/lib.rs $SRC/main.rs $SRC/units.rs compiler/build.rs
1400|$SRC/types.rs
1500|runtime
2000|$SRC/ast.rs $SRC/parse.rs
2200|$SRC/check.rs
2500|$SRC/codegen.rs
2500|ide
350|$SRC/wasm.rs
450|$SRC/lex.rs
500|host
EOT
)
# Paths within an entry are compared as sorted sets; entries sorted as strings.
EXPECTED=$(printf '%s\n' "$EXPECTED" | while IFS='|' read -r lim paths; do
    printf '%s|%s\n' "$lim" "$(printf '%s\n' $paths | sort | tr '\n' ' ')"
done | sort)
if [ -f "$B" ]; then
    ACTUAL=$(grep -v '^[[:space:]]*#' "$B" | grep -v '^[[:space:]]*$' | grep -v '^TOTAL|' |
        while IFS='|' read -r _ lim paths; do
            printf '%s|%s\n' "$lim" "$(printf '%s\n' $paths | sort | tr '\n' ' ')"
        done | sort)
    if [ "$ACTUAL" = "$EXPECTED" ]; then pass "budget.txt: components, limits and paths match plan"
    else fail "budget.txt: components, limits and paths match plan" \
        "expected:
$EXPECTED
actual:
$ACTUAL"; fi
    if grep -Fxq 'TOTAL|15000' "$B"; then pass "budget.txt: total is 15000"
    else fail "budget.txt: total is 15000" "no 'TOTAL|15000' line"; fi
    sum=$(grep -v '^[[:space:]]*#' "$B" | grep -v '^TOTAL|' | awk -F'|' 'NF >= 3 { s += $2 } END { print s + 0 }')
    if [ "$sum" -eq 14500 ]; then pass "budget.txt: component limits sum to 14500"
    else fail "budget.txt: component limits sum to 14500" "sum $sum"; fi
else
    fail "budget.txt: components, limits and paths match plan" "$B missing"
    fail "budget.txt: total is 15000" "$B missing"
    fail "budget.txt: component limits sum to 14500" "$B missing"
fi

echo "$PASSES passed, $FAILS failed"
[ "$FAILS" -eq 0 ]
