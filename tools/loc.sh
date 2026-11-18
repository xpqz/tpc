#!/usr/bin/env -S python3 -I
"""Usage: loc.sh [ROOT [BUDGET]]. Counts non-blank, non-comment-only lines per budget component.

Rust is counted after rustfmt (max_width = 100) on a copy. Exits 1 if any component or the
total exceeds its limit.
"""
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ROOT = sys.argv[1] if len(sys.argv) > 1 else HERE
BUDGET = sys.argv[2] if len(sys.argv) > 2 else os.path.join(HERE, "tools", "budget.txt")
EXT = {".rs": "rs", ".pas": "pas", ".pp": "pas", ".inc": "pas", ".js": "js"}
IDENT = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_"


def rustfmt(text):
    cmd = ["rustfmt", "--edition", "2021", "--config", "max_width=100", "--emit", "stdout"]
    run = subprocess.run(cmd, input=text, capture_output=True, text=True)
    if run.returncode != 0:
        sys.exit("loc.sh: rustfmt failed: " + run.stderr.strip())
    return run.stdout


def char_end(t, i):
    """End index of a char literal starting at t[i] == "'", or -1 for a lifetime."""
    if t[i + 1 : i + 2] == "\\":
        return t.index("'", i + 3) + 1
    return i + 3 if t[i + 2 : i + 3] == "'" else -1


def count(text, lang):
    """Counts lines holding code outside comments; string contents and directives are code."""
    n, i, code = 0, 0, False
    end, string, raw, depth = None, False, False, 0  # end: terminator of the open token
    while i < len(text):
        c, two = text[i], text[i : i + 2]
        if c == "\n":
            n += code
            code = False
            i += 1
        elif end and string:
            code = code or not c.isspace()
            if c == "\\" and not raw and lang != "pas":
                i += 1 if text[i + 1] == "\n" else 2
            elif text.startswith(end, i):
                end, i = None, i + len(end)
            else:
                i += 1
        elif end:
            if lang == "rs" and two == "/*":
                depth, i = depth + 1, i + 2
            elif two == end or c == end:
                depth, i = depth - 1, i + len(end)
                end = end if depth else None
            else:
                i += 1
        elif lang != "pas" and two == "//":
            i = text.find("\n", i)
            i = len(text) if i < 0 else i
        elif lang != "pas" and two == "/*":
            end, depth, i = "*/", 1, i + 2
        elif lang == "pas" and c == "{" and text[i + 1 : i + 2] != "$":
            end, depth, i = "}", 1, i + 1
        elif lang == "pas" and two == "(*":
            end, depth, i = "*)", 1, i + 2
        else:
            code = code or not c.isspace()
            hashes = raw_prefix(text, i) if lang == "rs" else -1
            if hashes >= 0:
                end, string, raw, i = '"' + "#" * hashes, True, True, i + hashes + 2
            elif c == "'" and lang == "rs":
                e = char_end(text, i)
                i = e if e > 0 else i + 1
            elif c in "\"'`" and (lang != "rs" or c == '"'):
                end, string, raw, i = c, True, False, i + 1
            else:
                i += 1
            continue
        string = string and end is not None
    return n + code


def raw_prefix(t, i):
    """Number of # in a raw string opener `r#*"` at t[i], or -1."""
    if t[i] != "r" or (i and t[i - 1] in IDENT and t[i - 1] != "b"):
        return -1
    j = i + 1
    while t[j : j + 1] == "#":
        j += 1
    return j - i - 1 if t[j : j + 1] == '"' else -1


def count_file(path):
    with open(path, encoding="utf-8") as f:
        text = f.read()
    lang = EXT[os.path.splitext(path)[1]]
    return count(rustfmt(text) if lang == "rs" else text, lang)


def files(root, spec):
    p = os.path.join(root, spec)
    if os.path.isfile(p):
        return [p] if os.path.splitext(p)[1] in EXT else []
    return [os.path.join(d, f) for d, _, fs in os.walk(p) for f in fs
            if os.path.splitext(f)[1] in EXT]


rows = []
for entry in open(BUDGET):
    entry = entry.strip()
    if entry and not entry.startswith("#"):
        rows.append(entry.split("|"))
total_limit = next(int(r[1]) for r in rows if r[0] == "TOTAL")
total, over = 0, []
for name, limit, *rest in rows:
    if name == "TOTAL":
        continue
    n = sum(count_file(f) for spec in rest[0].split() for f in files(ROOT, spec))
    total += n
    print(f"{name}: {n} / {limit}")
    if n > int(limit):
        over.append(name)
print(f"total: {total} / {total_limit}")
if total > total_limit:
    over.append("total")
for name in over:
    print(f"OVER: {name}")
sys.exit(1 if over else 0)
