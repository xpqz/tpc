//! Batch drivers for tests/lang, tests/rt and tests/neg (decision 7). Each returns one message per
//! failing program so a run reports every failure.
#![allow(dead_code)]
use crate::common::{CALL, I32_LT_S, I32_SUB, LOCAL_GET, LOCAL_SET};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};
use tp5c::wasm::*;

pub struct Res {
    pub code: i32,
    pub stderr: String,
}

/// Compiles `src` to `out`.
pub type Compile<'a> = &'a dyn Fn(&Path, &Path) -> Res;

pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

pub fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

pub fn scratch(name: &str) -> PathBuf {
    static N: AtomicU32 = AtomicU32::new(0);
    let dir = root().join("tmp/driver").join(format!(
        "{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst)
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir.join(name)
}

fn res(o: std::process::Output) -> Res {
    Res {
        code: o.status.code().unwrap_or(-1),
        stderr: String::from_utf8_lossy(&o.stderr).into(),
    }
}

/// Runs `tp5c`; a directory `STEM/` next to the source `STEM.pas` is passed as the auxiliary
/// directory that holds its units (decision 26).
pub fn real_compile(src: &Path, out: &Path) -> Res {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_tp5c"));
    cmd.arg(src).arg("-o").arg(out);
    let aux = src.with_extension("");
    if aux.is_dir() {
        cmd.arg("--dir").arg(aux);
    }
    res(cmd.output().unwrap())
}

/// Stand-in compiler for driver fixtures. A source starting `#error N LINE COL [FILE]` fails
/// with that diagnostic (in FILE when given, else in the source), `#raw TEXT` fails with TEXT
/// on stderr. A source whose first line is `#do` becomes a module that runs one directive per
/// following line: `#file NAME` prints what `fs_read` returns for the file NAME (runtime
/// error `-h` when `fs_open` fails), `#stdin` prints the next `readline` result and `|`,
/// `#rt N` calls `rterror(N)`, `#key` reads a key and draws its character at the next column
/// of the top screen row (a Crt program: the runner then prints the screen dump). Any other source becomes a module that prints the source text.
pub fn fake_compile(src: &Path, out: &Path) -> Res {
    let text = std::fs::read(src).unwrap();
    if let Some(rest) = text.strip_prefix(b"#error ") {
        let spec = String::from_utf8_lossy(rest);
        let f: Vec<&str> = spec.lines().next().unwrap().split(' ').collect();
        let file = f
            .get(3)
            .map_or(src.display().to_string(), |n| n.to_string());
        let stderr = format!("{}({},{}): Error {}: fake\n", file, f[1], f[2], f[0]);
        return Res { code: 1, stderr };
    }
    if let Some(rest) = text.strip_prefix(b"#raw ") {
        return Res {
            code: 1,
            stderr: String::from_utf8_lossy(rest).into(),
        };
    }
    std::fs::write(out, fake_module(&text)).unwrap();
    Res {
        code: 0,
        stderr: String::new(),
    }
}

fn fake_module(text: &[u8]) -> Vec<u8> {
    const WRITE: u32 = 0;
    const RTERROR: u32 = 1;
    const READLINE: u32 = 2;
    const FS_OPEN: u32 = 3;
    const FS_READ: u32 = 4;
    const FS_CLOSE: u32 = 5;
    const READKEY: u32 = 6;
    const SCREEN: u32 = 7;
    const BUFFER: i32 = 4096;
    const AND: u8 = 0x71;
    const OR: u8 = 0x72;
    // Only a program with `#key` imports the screen: that is what makes the runner dump it.
    let uses_keys = text.windows(5).any(|w| w == b"\n#key");
    let mut m = Module::new();
    let write = m.ty(&[I32, I32, I32], &[]);
    let rterror = m.ty(&[I32], &[]);
    let readline = m.ty(&[I32, I32], &[I32]);
    let file_op = m.ty(&[I32, I32, I32], &[I32]);
    let close = m.ty(&[I32], &[I32]);
    m.import_func("host", "write", write);
    m.import_func("host", "rterror", rterror);
    m.import_func("host", "readline", readline);
    m.import_func("host", "fs_open", file_op);
    m.import_func("host", "fs_read", file_op);
    m.import_func("host", "fs_close", close);
    if uses_keys {
        let readkey = m.ty(&[], &[I32]);
        let screen = m.ty(&[I32, I32, I32, I32], &[]);
        m.import_func("host", "readkey", readkey);
        m.import_func("host", "screen", screen);
    }
    let main = m.ty(&[], &[]);
    m.memory(1);
    let mut data = Vec::new();
    let mut at = |bytes: &[u8]| {
        let ptr = 256 + data.len() as i32;
        data.extend_from_slice(bytes);
        (ptr, bytes.len() as i32)
    };
    let mut c = Code::new();
    let source = String::from_utf8_lossy(text).into_owned();
    match source.strip_prefix("#do\n") {
        None => {
            let (ptr, len) = at(text);
            c.i32_const(1)
                .i32_const(ptr)
                .i32_const(len)
                .idx(CALL, WRITE);
        }
        Some(directives) => {
            let (bar, _) = at(b"|");
            let mut column = 0;
            for line in directives.lines() {
                let (word, arg) = line.split_once(' ').unwrap_or((line, ""));
                match word {
                    "#file" => {
                        let (ptr, len) = at(arg.as_bytes());
                        c.i32_const(ptr)
                            .i32_const(len)
                            .i32_const(0)
                            .idx(CALL, FS_OPEN);
                        c.idx(LOCAL_SET, 0);
                        c.idx(LOCAL_GET, 0).i32_const(0).op(I32_LT_S).if_(VOID);
                        c.i32_const(0)
                            .idx(LOCAL_GET, 0)
                            .op(I32_SUB)
                            .idx(CALL, RTERROR);
                        c.end();
                        c.i32_const(1).i32_const(BUFFER);
                        c.idx(LOCAL_GET, 0)
                            .i32_const(BUFFER)
                            .i32_const(255)
                            .idx(CALL, FS_READ);
                        c.idx(CALL, WRITE);
                        c.idx(LOCAL_GET, 0).idx(CALL, FS_CLOSE).op(0x1A);
                    }
                    "#stdin" => {
                        c.i32_const(1).i32_const(BUFFER);
                        c.i32_const(BUFFER).i32_const(255).idx(CALL, READLINE);
                        c.idx(CALL, WRITE);
                        c.i32_const(1).i32_const(bar).i32_const(1).idx(CALL, WRITE);
                    }
                    "#key" => {
                        c.i32_const(0).i32_const(column).i32_const(0);
                        c.idx(CALL, READKEY).i32_const(255).op(AND);
                        c.i32_const(0x700).op(OR).idx(CALL, SCREEN);
                        column += 1;
                    }
                    "#rt" => {
                        c.i32_const(arg.parse().unwrap()).idx(CALL, RTERROR);
                    }
                    _ => panic!("unknown directive {line}"),
                }
            }
        }
    }
    let f = m.func(main, &[I32], c);
    m.data(256, &data);
    m.export("memory", MEM, 0);
    m.export("main", FUNC, f);
    m.finish()
}

/// Sorted program stems in `dir` (files `STEM.pas`).
fn programs(dir: &Path) -> Vec<String> {
    let mut v: Vec<String> = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .filter_map(|e| {
            let p = e.unwrap().path();
            let ok = p.extension().is_some_and(|x| x == "pas");
            ok.then(|| p.file_stem().unwrap().to_string_lossy().into_owned())
        })
        .collect();
    v.sort();
    v
}

fn pending(dir: &Path) -> BTreeSet<String> {
    let text = std::fs::read_to_string(dir.join("PENDING")).unwrap_or_default();
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect()
}

/// Runs `check` on every program; applies PENDING: a pending program must fail, a pending
/// name with no program is stale.
fn sweep(dir: &Path, check: &dyn Fn(&str) -> Result<(), String>) -> Vec<String> {
    let progs = programs(dir);
    let pend = pending(dir);
    let mut fails: Vec<String> = pend
        .iter()
        .filter(|n| !progs.contains(n))
        .map(|n| format!("{n}: PENDING lists a program that does not exist"))
        .collect();
    for p in &progs {
        match (check(p), pend.contains(p)) {
            (Err(e), false) => fails.push(format!("{p}: {e}")),
            (Ok(()), true) => fails.push(format!("{p}: passes but is listed in PENDING")),
            _ => {}
        }
    }
    fails
}

fn expect_ok(r: &Res, what: &str) -> Result<(), String> {
    match r.code {
        0 => Ok(()),
        c => Err(format!("{what} exited {c}: {}", r.stderr.trim())),
    }
}

/// Runs `wasm` under tests/run.mjs against `expected`. A directory `STEM/` next to the source
/// `STEM.pas` holds the program's files and `STEM.in` is its console input (decision 26);
/// `STEM.keys` is the key script of a Crt program (tools/tp5.sh documents the format).
fn run_wasm(src: &Path, wasm: &Path, expected: &Path) -> Res {
    let mut cmd = Command::new("node");
    cmd.arg(root().join("tests/run.mjs"));
    let aux = src.with_extension("");
    if aux.is_dir() {
        cmd.arg("--dir").arg(aux);
    }
    let input = src.with_extension("in");
    if input.is_file() {
        cmd.arg("--in").arg(input);
    }
    let keys = src.with_extension("keys");
    if keys.is_file() {
        cmd.arg("--keys").arg(keys);
    }
    res(cmd
        .arg(wasm)
        .arg(expected)
        .output()
        .expect("node must be installed"))
}

/// Every `STEM.pas` compiles, runs under tests/run.mjs, and prints exactly `STEM.out`.
pub fn check_lang(dir: &Path, compile: Compile) -> Vec<String> {
    sweep(dir, &|stem| {
        let wasm = scratch(&format!("{stem}.wasm"));
        let src = dir.join(format!("{stem}.pas"));
        expect_ok(&compile(&src, &wasm), "compile")?;
        expect_ok(
            &run_wasm(&src, &wasm, &dir.join(format!("{stem}.out"))),
            "run",
        )
    })
}

/// Every `STEM.pas` compiles and runs under tests/run.mjs, prints exactly `STEM.out` and
/// exits with the code N of the last `Runtime error N` line of `STEM.out` (TP prints it
/// directly after any unfinished output line; a Crt program's dump follows it).
pub fn check_rt(dir: &Path, compile: Compile) -> Vec<String> {
    sweep(dir, &|stem| {
        let out = dir.join(format!("{stem}.out"));
        let text =
            std::fs::read_to_string(&out).map_err(|e| format!("cannot read {stem}.out: {e}"))?;
        let want: i32 = text
            .lines()
            .rev()
            .find_map(|l| l.rsplit_once("Runtime error "))
            .map(|(_, n)| n)
            .and_then(|n| n.parse().ok())
            .ok_or("no `Runtime error N` line in .out")?;
        let wasm = scratch(&format!("{stem}.wasm"));
        let src = dir.join(format!("{stem}.pas"));
        expect_ok(&compile(&src, &wasm), "compile")?;
        match run_wasm(&src, &wasm, &out) {
            r if r.code == want => Ok(()),
            r => Err(format!(
                "run exited {}, want {want}: {}",
                r.code,
                r.stderr.trim()
            )),
        }
    })
}

/// Every `STEM.pas` fails with exit 1 and `FILE(LINE,COL): Error N: MSG` where `N LINE` (or
/// `N LINE FILE` for an error in a unit or include file) is the content of `STEM.err`.
pub fn check_neg(dir: &Path, compile: Compile) -> Vec<String> {
    sweep(dir, &|stem| {
        let src = dir.join(format!("{stem}.pas"));
        let want = std::fs::read_to_string(dir.join(format!("{stem}.err")))
            .map_err(|e| format!("cannot read {stem}.err: {e}"))?;
        let want: Vec<&str> = want.split_whitespace().collect();
        let r = compile(&src, &scratch(&format!("{stem}.wasm")));
        if r.code == 0 {
            return Err("compiled without error".into());
        }
        let got =
            parse_diag(&src, &r.stderr).ok_or(format!("bad diagnostic: {}", r.stderr.trim()))?;
        if r.code != 1 || want != got {
            return Err(format!(
                "exit {}, got `{}`, want `{}`",
                r.code,
                got.join(" "),
                want.join(" ")
            ));
        }
        Ok(())
    })
}

/// `[N, LINE]` from the first stderr line when the diagnostic is in `src` (which must be
/// named as given), `[N, LINE, FILE]` when it is in another file.
pub fn parse_diag(src: &Path, stderr: &str) -> Option<Vec<String>> {
    let (file, rest) = stderr.lines().next()?.split_once('(')?;
    let (pos, rest) = rest.split_once("): Error ")?;
    let (line, col) = pos.split_once(',')?;
    let (n, msg) = rest.split_once(": ")?;
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    let ok = [n, line, col].iter().all(|s| digits(s)) && !msg.is_empty() && !file.is_empty();
    let mut got = vec![n.to_string(), line.to_string()];
    if file != src.to_str()? {
        got.push(file.to_string());
    }
    ok.then_some(got)
}
