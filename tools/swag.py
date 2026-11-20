"""SWAG validation sweep: compares TP 5.0 with tp5c on the programs of the SWAG archive.

usage: python3 -I tools/swag.py run [--sample N] [--jobs J] [--only PREFIX] [--redo CLASS]
       python3 -I tools/swag.py report

Environment: SWAG_DIR (a clone of the archive, default: swag/ beside the main checkout),
SWAG_RESULTS (default: swag-results/ beside it), TP5C (compiler binary, default: the debug
build of this checkout).

`run` is resumable: results.tsv in SWAG_RESULTS holds one line per file, keyed by path and
content hash; files already there are skipped. Each line:
  path  hash  class  tp  ours  note  tags
The outputs of both sides are kept in SWAG_RESULTS/out/HASH.{tp,ours} for triage. Nothing of
the archive is copied into the repository.
"""
import concurrent.futures
import hashlib
import os
import re
import subprocess
import sys
import tempfile
import threading
from collections import Counter, defaultdict

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
# The directory that holds the main checkout (a worktree's git common dir names it).
BESIDE = os.path.dirname(os.path.dirname(subprocess.run(
    ["git", "rev-parse", "--path-format=absolute", "--git-common-dir"],
    cwd=ROOT, capture_output=True, text=True).stdout.strip()))
SWAG = os.environ.get("SWAG_DIR", BESIDE + "/swag")
RESULTS = os.environ.get("SWAG_RESULTS", BESIDE + "/swag-results")
TSV = RESULTS + "/results.tsv"
TP5C = os.environ.get("TP5C") or subprocess.run(
    [ROOT + "/scripts/cargo-target.sh"], capture_output=True, text=True).stdout.strip() + "/debug/tp5c"
MAX_SIZE = 60000
RUN_TIMEOUT = 20

# Identifiers (outside strings and comments) that tie a program to real DOS hardware or to
# language features the subset leaves out on purpose; the lead's list, not a TP error list.
NOT_PORTABLE_WORDS = {
    "asm", "inline", "interrupt", "absolute", "port", "portw", "mem", "memw", "meml",
    "intr", "msdos", "registers", "ptr", "seg", "ofs", "cseg", "dseg", "sseg", "sptr",
    "exec", "swapvectors", "keep", "getintvec", "setintvec", "object", "constructor",
    "destructor", "virtual", "doserror_exec", "dosexitcode",
}
NOT_PORTABLE_PREFIXES = ("ems", "xms", "dpmi")
VIDEO_ADDRESS = re.compile(r"\$(b800|b000|a000|0b800|0a000|0b000)\b", re.I)
PORTABLE_UNITS = {"system", "crt", "dos"}
NONDET = re.compile(r"\b(random|randomize|gettime|getdate|memavail|maxavail|getftime|delay)\b", re.I)


def code_only(text):
    """`text` with string literals, comments and compiler directives blanked."""
    out, i, n = [], 0, len(text)
    while i < n:
        c = text[i]
        if c == "'":
            j = i + 1
            while j < n and text[j] not in "'\n":
                j += 1
            i = j + 1
            out.append(" ")
        elif c == "{":
            j = text.find("}", i)
            i = n if j < 0 else j + 1
            out.append(" ")
        elif text.startswith("(*", i):
            j = text.find("*)", i + 2)
            i = n if j < 0 else j + 2
            out.append(" ")
        else:
            out.append(c)
            i += 1
    return "".join(out)


def prefilter(path, text):
    """None when the program is worth running; else (class, reasons)."""
    if len(text) > MAX_SIZE:
        return "skip", ["too-big"]
    if path.startswith("delphi/") or re.search(r"\{\$(mode|apptype)", text, re.I):
        return "skip", ["delphi"]
    code = code_only(text)
    words = [w.lower() for w in re.findall(r"[A-Za-z_][A-Za-z0-9_]*", code)]
    if not words:
        return "skip", ["empty"]
    if words[0] == "unit":
        return "skip", ["unit-without-program"]
    if not re.search(r"\bend\s*\.", code, re.I):
        return "skip", ["fragment"]
    reasons = []
    # A uses clause is a plain identifier list; prose that happens to contain "uses" is not.
    ident = r"[A-Za-z_][A-Za-z0-9_]*"
    for m in re.finditer(r"\buses\s+(%s(?:\s*,\s*%s)*)\s*;" % (ident, ident), code, re.I):
        for unit in re.findall(r"[A-Za-z_][A-Za-z0-9_]*", m.group(1)):
            if unit.lower() not in PORTABLE_UNITS:
                reasons.append("unit:" + unit.lower())
    seen = set(words)
    for w in sorted(seen):
        if w in NOT_PORTABLE_WORDS or w.startswith(NOT_PORTABLE_PREFIXES):
            reasons.append("word:" + w)
    if VIDEO_ADDRESS.search(text):
        reasons.append("video-address")
    if reasons:
        return "not-portable", sorted(set(reasons))
    return None


def tp5(program, work):
    """(status, text): status is ok, error, timeout or fail."""
    src = work + "/prog.pas"
    with open(src, "wb") as f:
        f.write(program)
    open(work + "/prog.in", "w").close()
    env = dict(os.environ, TP5_TIMEOUT=str(RUN_TIMEOUT))
    p = subprocess.run([ROOT + "/tools/tp5.sh", src], capture_output=True, env=env)
    text = p.stdout.decode("latin1")
    if p.returncode == 0:
        return "ok", text
    if p.returncode == 1:
        # A Crt program that ends before the screen dump leaves tp5.sh with nothing to print.
        return ("error" if ERR_TP.search(text) else "nodump"), text.strip()
    if p.returncode == 3:
        return "timeout", ""
    return "fail", p.stderr.decode("latin1")


def ours(program, work):
    """(status, text, detail): status is ok, error, timeout or fail."""
    src, wasm = work + "/prog.pas", work + "/prog.wasm"
    c = subprocess.run([TP5C, src, "-o", wasm], capture_output=True)
    if c.returncode != 0:
        return "error", c.stderr.decode("latin1").strip().splitlines()[0] if c.stderr.strip() else "", ""
    try:
        r = subprocess.run(["node", ROOT + "/tests/run.mjs", "--in", work + "/prog.in", wasm],
                           capture_output=True, timeout=RUN_TIMEOUT)
    except subprocess.TimeoutExpired:
        return "timeout", "", ""
    text = r.stdout.decode("latin1")
    if r.returncode == 2 and b"output limit" in r.stderr:  # endless output: TP times out too
        return "timeout", "", ""
    killed = r.returncode < 0  # node aborted (out of memory) or crashed
    if killed or (r.returncode == 2 and b"run.mjs:" in r.stderr):  # else the program's exit code 2
        return "fail", text, r.stderr.decode("latin1").strip()
    return "ok", text, "exit %d" % r.returncode


ERR_TP = re.compile(r"^(\w+)\.\w+\((\d+)\): Error (\d+)", re.M)
ERR_OURS = re.compile(r"^.*?\((\d+),(\d+)\): Error (\d+): (.*)$")
RT = re.compile(r"(Runtime error \d+) at [0-9A-F]{4}:[0-9A-F]{4}\.")


def normalise(text):
    return RT.sub(r"\1", text.replace("\r", ""))


def token_at(program, line, col):
    lines = program.decode("latin1").split("\n")
    if 1 <= line <= len(lines):
        rest = lines[line - 1][col - 1:]
        if rest.startswith("{$"):
            return rest.split("}")[0].replace(" ", "").upper() + "}"
        m = re.match(r"[A-Za-z_][A-Za-z0-9_]*|\S", rest)
        return m.group(0).lower() if m else "?"
    return "?"


def sweep_one(path):
    raw = open(SWAG + "/" + path, "rb").read()
    digest = hashlib.sha1(raw).hexdigest()[:16]
    # Both sides get LF line ends: tp5.sh adds a CR to every line, and a doubled CR makes
    # the real compiler's program misbehave.
    program = raw.replace(b"\r\n", b"\n")
    text = program.decode("latin1")
    tags = "nondet" if NONDET.search(code_only(text)) else ""
    pre = prefilter(path, text)
    if pre:
        return [path, digest, pre[0], "-", "-", ",".join(pre[1]), tags], None, None
    return run_both(path, digest, program, tags)


def run_both(path, digest, program, tags):
    with tempfile.TemporaryDirectory(dir=ROOT + "/tmp") as work:
        ts, tt = tp5(program, work)
        os_, ot, detail = ours(program, work + "/")
    tp_cell = ours_cell = note = ""
    if ts == "error":
        m = ERR_TP.match(tt)
        tp_cell = "error %s@%s" % (m.group(3), m.group(2)) if m else "error ?"
    else:
        tp_cell = ts
    if os_ == "error":
        m = ERR_OURS.match(ot)
        ours_cell = "error %s@%s" % (m.group(3), m.group(1)) if m else "error ?"
        if m and m.group(3) == "200":
            note = "%s [%s]" % (m.group(4), token_at(program, int(m.group(1)), int(m.group(2))))
        elif m:
            note = "%s [%s]" % (m.group(4), token_at(program, int(m.group(1)), int(m.group(2))))
    else:
        ours_cell = os_
        note = detail.replace("\n", " ")[:200]  # the exit code of the run
    tn, on = normalise(tt), normalise(ot)
    if ts == "fail":
        cls = "tool-failure"
    elif ts == "error":
        cls = "tp-rejects-ours-accepts" if os_ != "error" else "tp-rejects"
    elif ts == "nodump":
        cls = "tp-no-dump"
    elif ts == "timeout" and "\nexit 255" in "\n" + on:
        cls = "needs-input"  # a Crt program blocked in ReadKey: no key script is given
    elif ts == "timeout" or os_ == "timeout":
        cls = "timeout"
    elif os_ == "error":
        cls = "omitted-200" if ours_cell.startswith("error 200") else "gap"
    elif os_ == "fail":
        cls = "ours-run-failure"
    elif "\nexit 255" in "\n" + tn or "\nexit 255" in "\n" + on:
        cls = "needs-input"
    elif tn == on:
        cls = "same"
    else:
        cls = "differ"
    row = [path, digest, cls, tp_cell, ours_cell, note, tags]
    return row, (tt if ts != "fail" else tt), ot


def all_files():
    found = []
    for d, _, names in os.walk(SWAG):
        for n in names:
            if n.lower().endswith(".pas"):
                found.append(os.path.relpath(d + "/" + n, SWAG))
    return sorted(found)


def sample(files, n):
    """n files spread over the categories, picked by path hash so the pick is stable."""
    by_cat = defaultdict(list)
    for f in files:
        by_cat[f.split("/")[0]].append(f)
    chosen = []
    for cat in sorted(by_cat):
        group = sorted(by_cat[cat], key=lambda p: hashlib.sha1(p.encode()).hexdigest())
        chosen += group[: max(1, round(n * len(group) / len(files)))]
    return sorted(chosen, key=lambda p: hashlib.sha1(p.encode()).hexdigest())[:n]


def load_rows():
    rows = {}
    if os.path.exists(TSV):
        for line in open(TSV, encoding="latin1"):
            cells = line.rstrip("\n").split("\t")
            if len(cells) == 7:
                rows[(cells[0], cells[1])] = cells
    return rows


PREFILTER_CLASSES = ("skip", "not-portable")


def sweep_one_prefilter(path, digest):
    text = open(SWAG + "/" + path, "rb").read().decode("latin1")
    pre = prefilter(path, text)
    if pre is None:
        return None
    tags = "nondet" if NONDET.search(code_only(text)) else ""
    return [path, digest, pre[0], "-", "-", ",".join(pre[1]), tags]


def write_rows(rows):
    with open(TSV, "w", encoding="latin1") as f:
        for row in rows:
            f.write("\t".join(c.replace("\t", " ") for c in row) + "\n")


def digest_of(path):
    return hashlib.sha1(open(SWAG + "/" + path, "rb").read()).hexdigest()[:16]


def cmd_run(args):
    n = jobs = None
    only = ""
    redo = set()  # classes whose cached rows are run again
    jobs = 4
    while args:
        a = args.pop(0)
        if a == "--sample":
            n = int(args.pop(0))
        elif a == "--jobs":
            jobs = min(4, int(args.pop(0)))
        elif a == "--redo":
            redo.add(args.pop(0))
        elif a == "--only":
            only = args.pop(0)
    os.makedirs(RESULTS + "/out", exist_ok=True)
    os.makedirs(ROOT + "/tmp", exist_ok=True)
    files = [f for f in all_files() if f.startswith(only)]
    if n:
        files = sample(files, n)
    done = load_rows()
    # The cheap pre-filter is applied afresh to every file, so changing it reclassifies the
    # cached rows; only files that pass it and have no cached run need DOSBox.
    todo, rows = [], []
    for f in files:
        digest = digest_of(f)
        cached = done.get((f, digest))
        row = sweep_one_prefilter(f, digest)
        if row is not None:
            rows.append(row)
        elif cached is not None and cached[2] not in PREFILTER_CLASSES and cached[2] not in redo:
            rows.append(cached)
        else:
            todo.append(f)
    print("%d files, %d to do" % (len(files), len(todo)), flush=True)
    lock = threading.Lock()
    write_rows(rows)
    out = open(TSV, "a", encoding="latin1")

    def work(path):
        try:
            row, tp_out, our_out = sweep_one(path)
        except Exception as e:  # keep the sweep going; the row records the failure
            row, tp_out, our_out = [path, digest_of(path), "tool-failure", "-", "-", repr(e)[:150], ""], None, None
        with lock:
            if tp_out is not None:
                open("%s/out/%s.tp" % (RESULTS, row[1]), "w", encoding="latin1").write(tp_out)
                open("%s/out/%s.ours" % (RESULTS, row[1]), "w", encoding="latin1").write(our_out)
            out.write("\t".join(c.replace("\t", " ") for c in row) + "\n")
            out.flush()

    with concurrent.futures.ThreadPoolExecutor(jobs) as pool:
        list(pool.map(work, todo))
    print("done")


def cmd_report(args):
    rows = [r for r in load_rows().values()]
    current = {p: d for p, d in ((r[0], r[1]) for r in rows)}
    print("total", len(rows))
    for cls, k in Counter(r[2] for r in rows).most_common():
        print("%6d  %s" % (k, cls))
    print("\nnot-portable reasons (a file counts under each)")
    c = Counter(x for r in rows if r[2] in ("not-portable", "skip") for x in r[5].split(","))
    for k, v in c.most_common(60):
        print("%6d  %s" % (v, k))
    print("\nomitted-200 tokens")
    c = Counter(r[5] for r in rows if r[2] == "omitted-200")
    for k, v in c.most_common(40):
        print("%6d  %s" % (v, k))
    print("\nboth reject: TP error vs ours")
    both = [r for r in rows if r[2] == "tp-rejects" and r[4].startswith("error")]
    same = Counter()
    for r in both:
        tp_n, tp_l = r[3].split()[1].split("@")
        ou_n, ou_l = r[4].split()[1].split("@")
        kind = "same number and line" if (tp_n, tp_l) == (ou_n, ou_l) else \
            "same number, other line" if tp_n == ou_n else "other number"
        same[kind] += 1
    for k, v in same.most_common():
        print("%6d  %s" % (v, k))
    pairs = Counter((r[3].split()[1].split("@")[0], r[4].split()[1].split("@")[0]) for r in both
                    if r[3].split()[1].split("@")[0] != r[4].split()[1].split("@")[0])
    for (a, b), v in pairs.most_common(15):
        print("%6d  TP %s, ours %s" % (v, a, b))
    for cls in ("gap", "tp-rejects-ours-accepts"):
        print("\n" + cls)
        c = Counter((r[3] + " | " + r[4] + " | " + r[5]) for r in rows if r[2] == cls)
        for k, v in c.most_common(40):
            print("%6d  %s" % (v, k))


if __name__ == "__main__":
    cmds = {"run": cmd_run, "report": cmd_report}
    if len(sys.argv) < 2 or sys.argv[1] not in cmds:
        sys.exit(__doc__)
    cmds[sys.argv[1]](sys.argv[2:])
