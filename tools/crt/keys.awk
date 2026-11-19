# Usage: awk -f keys.awk KEYS.TBL NAME.keys
# Turns a key script into KEYFEED's KEYS.DAT: one `$AAAA` word per key. A token is a key
# name from KEYS.TBL (any case) or one printable character. Exit 2 on any other token.
FNR == NR { named[tolower($1)] = $2; next }
BEGIN { for (c = 32; c < 127; c++) ascii[sprintf("%c", c)] = c }
{
  for (i = 1; i <= NF; i++) {
    key = tolower($i)
    if (key in named) print named[key]
    else if ($i in ascii) printf "$%04X\n", ascii[$i]
    else { print "keys.awk: unknown key " $i > "/dev/stderr"; failed = 1 }
  }
}
END { exit failed ? 2 : 0 }
