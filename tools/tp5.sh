#!/bin/sh
# Usage: tools/tp5.sh prog.pas
# Compiles and runs prog.pas with the real Turbo Pascal 5.0 compiler (TPC.EXE) under
# DOSBox-X. Prints the program's output (LF line endings, exit 0), or the compiler's
# `P.PAS(LINE): Error N: Message.` line (exit 1). The TP 5.0 files are not in the repo;
# see docs/references.md. TP5_HOME overrides their location.
set -e
home=${TP5_HOME:-/Users/stefan/work/crust/tp5-oracle/c}
[ -f "$home/TPC.EXE" ] || { echo "tools/tp5.sh: no TPC.EXE in $home" >&2; exit 2; }
w=$(mktemp -d "${TMPDIR:-/tmp}/tp5.XXXXXX")
trap 'rm -rf "$w"' EXIT
cp "$home/TPC.EXE" "$home/TURBO.TPL" "$w/"
sed 's/$/\r/' "$1" > "$w/P.PAS"
dosbox-x -silent -nogui -nomenu -exit -c "mount c $w" -c "c:" -c "tpc P.PAS > TPC.TXT" \
  -c "if exist P.EXE P > RUN.TXT" >/dev/null 2>&1
if [ -f "$w/P.EXE" ]; then
  tr -d '\r' < "$w/RUN.TXT"
else
  tr -d '\r' < "$w/TPC.TXT" | grep -ao 'P.PAS([0-9]*): Error .*'
  exit 1
fi
