#!/bin/sh
# Usage: tools/tp5.sh DIR/NAME.pas
# Compiles and runs NAME.pas (as P.PAS) with the real Turbo Pascal 5.0 compiler (TPC.EXE)
# under DOSBox-X. Prints the program's output (LF line endings, exit 0), or the compiler's
# `FILE(LINE): Error N: Message.` line (exit 1), where FILE is P.PAS or the unit or
# include file that contains the error.
#
# Multi-file fixtures (decision 26): the files of the directory DIR/NAME/ are copied next
# to the program, so units and include files are found (`tpc /M` compiles used units from
# source) and data files can be opened by the program. Files named *.PAS and *.INC are
# given CRLF line endings, as the compiler expects; all others are copied byte for byte.
# DIR/NAME.dirs adds shared directories (see below), for tests of units that live elsewhere.
# If DIR/NAME.in exists, it is the program's stdin (LF is converted to CRLF); stdin and
# stdout are both redirected, which DOSBox-X supports. Without it stdin is not redirected.
# Crt programs (decision 37): a program whose uses clause names Crt gets ScrDump and KeyFeed
# (tools/crt/) as its first units, so the program's own output goes to the screen, which is
# cleared (`cls`) before the program starts. The keys come from DIR/NAME.keys, a text file of
# whitespace-separated tokens: a key name from tools/crt/KEYS.TBL (Enter, Esc, Tab, Backspace,
# Space, Up, Down, Left, Right, Home, End, PgUp, PgDn, Ins, Del, F1 to F12; any case) or one
# printable character, typed literally. `h i Enter` types hi and Enter. The table also has the
# chords of the Reference Guide's Table C.2 (Appendix C, page 424): ^A to ^Z (Ctrl+letter,
# the control codes 1 to 26), Alt-A to Alt-Z, Alt-1 to Alt-=, Shift-F1 to Shift-F12, Ctrl-F1 to
# Ctrl-F12, Alt-F1 to Alt-F12, Shift-Tab, Ctrl-Left, Ctrl-Right, Ctrl-Home, Ctrl-End, Ctrl-PgUp,
# Ctrl-PgDn and Ctrl-PrtSc. ReadKey returns the control code for ^A.. and 0 plus the extended
# code for the others; ^H, ^I and ^M give the same byte as Backspace, Tab and Enter. A blocking read after
# the last key halts the program with exit code 255. After the program's own stdout the script
# prints the screen dump (tools/crt/SCRDUMP.PAS): the 25 text rows with trailing blanks cut,
# `cursor COL ROW`, `exit CODE` and `attr ROW COL LEN HEX` runs for cells whose attribute is
# not 07. Files the program creates are discarded. The TP 5.0 files are not in the repo;
# see docs/references.md. TP5_HOME overrides their location. A run is killed after
# TP5_TIMEOUT seconds (default 60) and reported as a timeout (exit 3).
set -e
export LC_ALL=C
home=${TP5_HOME:-/Users/stefan/work/crust/tp5-oracle/c}
[ -f "$home/TPC.EXE" ] || { echo "tools/tp5.sh: no TPC.EXE in $home" >&2; exit 2; }
w=$(mktemp -d "${TMPDIR:-/tmp}/tp5.XXXXXX")
trap 'rm -rf "$w"' EXIT
cp "$home/TPC.EXE" "$home/TURBO.TPL" "$w/"
sed 's/$/\r/' "$1" > "$w/P.PAS"
base=${1%.pas}
# DIR/NAME.dirs lists directories (paths from the repository root, one per line) whose files
# are copied first, so a test of the IDE's modules finds them in ide/ and its own directory
# NAME/ can hold a stand-in for a unit (the IDE host unit) that replaces a shared file.
if [ -f "$base.dirs" ]; then
  repo=$(cd "$(dirname "$0")/.." && pwd)
  while read -r shared; do
    [ -z "$shared" ] || cp "$repo/$shared"/* "$w/"
  done < "$base.dirs"
fi
if [ -d "$base" ] || [ -f "$base.dirs" ]; then
  [ ! -d "$base" ] || cp "$base"/* "$w/"
  for f in "$w"/*.[Pp][Aa][Ss] "$w"/*.[Ii][Nn][Cc]; do
    [ -f "$f" ] && sed 's/$/\r/' "$f" > "$f.crlf" && mv "$f.crlf" "$f"
  done
fi
crt=
if tr '\n' ' ' < "$1" | grep -qiE 'uses[^;]*[ ,]crt[ ,;]'; then
  crt=1
  crtdir=$(cd "$(dirname "$0")" && pwd)/crt
  sed 's/$/\r/' "$crtdir/SCRDUMP.PAS" > "$w/SCRDUMP.PAS"
  sed 's/$/\r/' "$crtdir/KEYFEED.PAS" > "$w/KEYFEED.PAS"
  # The first `uses` gets the two units in front; the line count of P.PAS is unchanged.
  awk '!done && match(tolower($0), /(^|[ \t;])uses([ \t]|$)/) {
    $0 = substr($0, 1, RSTART + RLENGTH - 1) " ScrDump, KeyFeed," substr($0, RSTART + RLENGTH)
    done = 1
  } { print }' "$1" | sed 's/$/\r/' > "$w/P.PAS"
  if [ -f "$base.keys" ]; then
    awk -f "$crtdir/keys.awk" "$crtdir/KEYS.TBL" "$base.keys" > "$w/KEYS.LF" ||
      { echo "tools/tp5.sh: bad key in $base.keys" >&2; exit 2; }
    sed 's/$/\r/' "$w/KEYS.LF" > "$w/KEYS.DAT"
  fi
fi
run="P > RUN.TXT"
if [ -f "$base.in" ]; then
  sed 's/$/\r/' "$base.in" > "$w/P.IN"
  run="P < P.IN > RUN.TXT"
fi
# RUN.BAT clears the screen and runs the program without echoing its own command line.
printf '@echo off\r\ncls\r\n%s\r\n' "$run" > "$w/RUN.BAT"
dosbox-x -silent -nogui -nomenu -exit -c "mount c $w" -c "c:" -c "tpc /M P.PAS > TPC.TXT" \
  -c "if exist P.EXE RUN" >/dev/null 2>&1 &
pid=$!
n=0
while kill -0 $pid 2>/dev/null; do
  n=$((n + 1))
  if [ $n -gt $((${TP5_TIMEOUT:-60} * 10)) ]; then
    kill $pid; echo "tools/tp5.sh: timeout" >&2; exit 3
  fi
  sleep 0.1
done
if [ -f "$w/P.EXE" ]; then
  tr -d '\r' < "$w/RUN.TXT"
  [ -z "$crt" ] || tr -d '\r' < "$w/SCREEN.TXT"
else
  # TPC prefixes the line with a progress `NAME(0)` per file compiled, separated by CR.
  tr -d '\r' < "$w/TPC.TXT" | grep -ao '[A-Za-z0-9_]*\.[A-Za-z]*([0-9]*): Error .*'
  exit 1
fi
