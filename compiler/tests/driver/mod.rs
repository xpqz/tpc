//! Batch drivers for tests/lang and tests/neg (decision 7). Each returns one message per
//! failing program so a run reports every failure.
#![allow(dead_code)]
use crate::common::CALL;
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

pub fn real_compile(src: &Path, out: &Path) -> Res {
    let o = Command::new(env!("CARGO_BIN_EXE_tp5c"))
        .arg(src)
        .arg("-o")
        .arg(out)
        .output();
    res(o.unwrap())
}

/// Stand-in compiler for driver fixtures. A source starting `#error N LINE COL` fails with
/// that diagnostic, `#raw TEXT` fails with TEXT on stderr; any other source becomes a module that prints the source text.
pub fn fake_compile(src: &Path, out: &Path) -> Res {
    let text = std::fs::read(src).unwrap();
    if let Some(rest) = text.strip_prefix(b"#error ") {
        let spec = String::from_utf8_lossy(rest);
        let f: Vec<&str> = spec.lines().next().unwrap().split(' ').collect();
        let stderr = format!(
            "{}({},{}): Error {}: fake\n",
            src.display(),
            f[1],
            f[2],
            f[0]
        );
        return Res { code: 1, stderr };
    }
    if let Some(rest) = text.strip_prefix(b"#raw ") {
        return Res {
            code: 1,
            stderr: String::from_utf8_lossy(rest).into(),
        };
    }
    let mut m = Module::new();
    let write = m.ty(&[I32, I32, I32], &[]);
    let main = m.ty(&[], &[]);
    m.import_func("host", "write", write);
    m.memory(1);
    m.data(256, &text);
    let mut c = Code::new();
    c.i32_const(1)
        .i32_const(256)
        .i32_const(text.len() as i32)
        .idx(CALL, 0);
    let f = m.func(main, &[], c);
    m.export("memory", MEM, 0);
    m.export("main", FUNC, f);
    std::fs::write(out, m.finish()).unwrap();
    Res {
        code: 0,
        stderr: String::new(),
    }
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

/// Every `STEM.pas` compiles, runs under tests/run.mjs, and prints exactly `STEM.out`.
pub fn check_lang(dir: &Path, compile: Compile) -> Vec<String> {
    sweep(dir, &|stem| {
        let wasm = scratch(&format!("{stem}.wasm"));
        expect_ok(&compile(&dir.join(format!("{stem}.pas")), &wasm), "compile")?;
        let o = Command::new("node")
            .arg(root().join("tests/run.mjs"))
            .arg(&wasm)
            .arg(dir.join(format!("{stem}.out")))
            .output()
            .expect("node must be installed");
        expect_ok(&res(o), "run")
    })
}

/// Every `STEM.pas` fails with exit 1 and `NAME(LINE,COL): Error N: MSG` where `N LINE` is
/// the content of `STEM.err`.
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
        let got = [got.0, got.1];
        if r.code != 1 || want != got {
            return Err(format!(
                "exit {}, got `{} {}`, want `{}`",
                r.code,
                got[0],
                got[1],
                want.join(" ")
            ));
        }
        Ok(())
    })
}

/// (error number, line) from the first stderr line, which must start with the path as given.
pub fn parse_diag(src: &Path, stderr: &str) -> Option<(String, String)> {
    let rest = stderr
        .lines()
        .next()?
        .strip_prefix(src.to_str()?)?
        .strip_prefix('(')?;
    let (pos, rest) = rest.split_once("): Error ")?;
    let (line, col) = pos.split_once(',')?;
    let (n, msg) = rest.split_once(": ")?;
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    let ok = [n, line, col].iter().all(|s| digits(s)) && !msg.is_empty();
    ok.then(|| (n.to_string(), line.to_string()))
}
