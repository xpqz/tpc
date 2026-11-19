//! tests/run.mjs with the Crt imports: `screen`, `readkey`, `keypressed` and `sleep`, the key
//! script option `--keys`, and the screen dump printed after a program that imports `screen`
//! (decisions 37 to 39, docs/abi.md section 8).
mod common;
use common::*;
use std::path::PathBuf;
use std::process::{Command, Output};
use tp5c::wasm::*;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn scratch(name: &str) -> PathBuf {
    fresh_dir("runner_crt").join(name)
}

const PUT: i32 = 0;
const CURSOR: i32 = 3;
const I32_ADD: u8 = 0x6A;
const I32_AND: u8 = 0x71;
const I32_OR: u8 = 0x72;
const SCREEN_ARGS: [u8; 4] = [I32, I32, I32, I32];

enum Step<'a> {
    Write(&'a [u8]),
    /// screen(PUT, col, row, char | attr << 8)
    Put(i32, i32, u8, u8),
    /// screen(op, a, b, c) with raw operands
    Screen(i32, i32, i32, i32),
    /// reads a key and draws its low byte at (col, 0)
    Key(i32),
    /// draws '1' or '0' at (col, 0): keypressed
    Pressed(i32),
    /// time, sleep(ms), time: draws the seconds that passed, as a digit, at (col, 0)
    SleepSeconds(i32, i32),
    RtError(i32),
}

fn module(imports: &[&str], steps: &[Step]) -> Vec<u8> {
    let mut m = Module::new();
    let sigs: [(&str, &[u8], &[u8]); 7] = [
        ("write", &[I32, I32, I32], &[]),
        ("readkey", &[], &[I32]),
        ("keypressed", &[], &[I32]),
        ("screen", &SCREEN_ARGS, &[]),
        ("time", &[I32], &[I32]),
        ("sleep", &[I32], &[]),
        ("rterror", &[I32], &[]),
    ];
    // Function index of each import the module declares.
    let mut ix = std::collections::HashMap::new();
    for (name, params, results) in sigs {
        if imports.contains(&name) {
            let ty = m.ty(params, results);
            ix.insert(name, m.import_func("host", name, ty));
        }
    }
    let main = m.ty(&[], &[]);
    m.memory(1);
    let mut data = Vec::new();
    let mut c = Code::new();
    for step in steps {
        match step {
            Step::Write(bytes) => {
                let at = 256 + data.len() as i32;
                data.extend_from_slice(bytes);
                c.i32_const(1).i32_const(at).i32_const(bytes.len() as i32);
                c.idx(CALL, ix["write"]);
            }
            Step::Put(col, row, ch, attr) => {
                let cell = *ch as i32 | (*attr as i32) << 8;
                c.i32_const(PUT)
                    .i32_const(*col)
                    .i32_const(*row)
                    .i32_const(cell);
                c.idx(CALL, ix["screen"]);
            }
            Step::Screen(op, a, b, cell) => {
                c.i32_const(*op)
                    .i32_const(*a)
                    .i32_const(*b)
                    .i32_const(*cell);
                c.idx(CALL, ix["screen"]);
            }
            Step::Key(col) => {
                c.i32_const(PUT).i32_const(*col).i32_const(0);
                c.idx(CALL, ix["readkey"]).i32_const(255).op(I32_AND);
                c.i32_const(0x700).op(I32_OR).idx(CALL, ix["screen"]);
            }
            Step::Pressed(col) => {
                c.i32_const(PUT).i32_const(*col).i32_const(0);
                c.idx(CALL, ix["keypressed"])
                    .i32_const(0x730)
                    .op(I32_ADD)
                    .idx(CALL, ix["screen"]);
            }
            Step::SleepSeconds(col, ms) => {
                c.i32_const(0).idx(CALL, ix["time"]).op(DROP);
                c.i32_const(1).idx(CALL, ix["time"]).idx(LOCAL_SET, 0);
                c.i32_const(*ms).idx(CALL, ix["sleep"]);
                c.i32_const(0).idx(CALL, ix["time"]).op(DROP);
                c.i32_const(PUT).i32_const(*col).i32_const(0);
                c.i32_const(1).idx(CALL, ix["time"]);
                c.idx(LOCAL_GET, 0).op(I32_SUB).i32_const(0x730).op(I32_ADD);
                c.idx(CALL, ix["screen"]);
            }
            Step::RtError(code) => {
                c.i32_const(*code).idx(CALL, ix["rterror"]);
            }
        }
    }
    let f = m.func(main, &[I32], c);
    m.data(256, &data);
    m.export("memory", MEM, 0);
    m.export("main", FUNC, f);
    m.finish()
}

const DROP: u8 = 0x1A;

/// Runs `wasm` with a key script file when `keys` is given.
fn run(name: &str, wasm: &[u8], keys: Option<&str>, now_ms: &str) -> Output {
    let w = scratch(&format!("{name}.wasm"));
    std::fs::write(&w, wasm).unwrap();
    let mut cmd = Command::new("node");
    cmd.arg(root().join("tests/run.mjs"))
        .env("TP5_NOW_MS", now_ms);
    if let Some(text) = keys {
        let k = scratch(&format!("{name}.keys"));
        std::fs::write(&k, text).unwrap();
        cmd.arg("--keys").arg(k);
    }
    cmd.arg(w).output().expect("node must be installed")
}

fn stdout(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).into_owned()
}

fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}

/// A dump: the first row of text, 24 blank rows, then `tail` (cursor, exit, attribute lines).
fn dump(row0: &str, tail: &str) -> String {
    format!("{row0}\n{}{tail}", "\n".repeat(24))
}

#[test]
fn a_program_importing_screen_ends_with_the_dump_after_its_stdout() {
    let w = module(
        &["write", "screen"],
        &[
            Step::Write(b"out\n"),
            Step::Put(0, 0, b'H', 7),
            Step::Put(1, 0, b'i', 7),
        ],
    );
    let o = run("dump_after_stdout", &w, None, "0");
    assert_eq!(o.status.code(), Some(0), "{}", stderr(&o));
    assert_eq!(
        stdout(&o),
        format!("out\n{}", dump("Hi", "cursor 1 1\nexit 0\n"))
    );
}

#[test]
fn a_program_without_the_screen_import_prints_no_dump() {
    let w = module(&["write"], &[Step::Write(b"plain\n")]);
    let o = run("no_dump", &w, None, "0");
    assert_eq!(stdout(&o), "plain\n");
}

#[test]
fn the_dump_has_attribute_runs_and_the_cursor() {
    let w = module(
        &["screen"],
        &[
            Step::Put(9, 11, b'c', 0x1E),
            Step::Put(10, 11, b'u', 0x1E),
            Step::Screen(CURSOR, 11, 11, 0),
        ],
    );
    let o = run("attrs", &w, None, "0");
    let out = stdout(&o);
    assert!(
        out.ends_with("cursor 12 12\nexit 0\nattr 12 10 2 1E\n"),
        "{out}"
    );
    assert_eq!(out.lines().nth(11), Some("         cu"));
}

#[test]
fn keys_come_from_the_key_script_in_order() {
    let w = module(
        &["screen", "readkey"],
        &[Step::Key(0), Step::Key(1), Step::Key(2), Step::Key(3)],
    );
    let o = run("keys_in_order", &w, Some("a Space Enter Z"), "0");
    assert_eq!(o.status.code(), Some(0), "{}", stderr(&o));
    let row0 = stdout(&o).lines().next().unwrap().to_string();
    assert_eq!(row0, "a \rZ");
}

#[test]
fn an_extended_key_is_zero_then_its_scan_code() {
    let w = module(
        &["screen", "readkey"],
        &[Step::Key(0), Step::Key(1), Step::Key(2)],
    );
    let o = run("extended", &w, Some("Up b"), "0");
    let out = stdout(&o);
    assert!(out.starts_with("\0Hb\n"), "{out:?}");
}

#[test]
fn keypressed_sees_pending_keys_without_consuming_them() {
    let w = module(
        &["screen", "readkey", "keypressed"],
        &[
            Step::Pressed(0),
            Step::Pressed(1),
            Step::Key(2),
            Step::Pressed(3),
        ],
    );
    let o = run("keypressed", &w, Some("x"), "0");
    assert_eq!(o.status.code(), Some(0), "{}", stderr(&o));
    assert_eq!(stdout(&o).lines().next(), Some("11x0"));
}

#[test]
fn a_program_without_a_key_script_has_no_keys() {
    let w = module(&["screen", "keypressed"], &[Step::Pressed(0)]);
    let o = run("no_keys", &w, None, "0");
    assert_eq!(stdout(&o).lines().next(), Some("0"));
}

#[test]
fn reading_a_key_after_the_last_one_halts_with_exit_255() {
    let w = module(
        &["screen", "readkey"],
        &[Step::Key(0), Step::Key(1), Step::Key(2)],
    );
    let o = run("exhausted", &w, Some("p q"), "0");
    assert_eq!(o.status.code(), Some(255), "{}", stderr(&o));
    assert_eq!(stdout(&o), dump("pq", "cursor 1 1\nexit 255\n"));
}

#[test]
fn reading_a_key_with_no_script_halts_with_exit_255() {
    let w = module(&["screen", "readkey"], &[Step::Key(0)]);
    let o = run("exhausted_empty", &w, None, "0");
    assert_eq!(o.status.code(), Some(255));
    assert!(
        stdout(&o).ends_with("cursor 1 1\nexit 255\n"),
        "{}",
        stdout(&o)
    );
}

#[test]
fn a_runtime_error_prints_its_line_then_the_dump() {
    let w = module(
        &["write", "screen", "rterror"],
        &[Step::Put(0, 0, b'e', 7), Step::RtError(200)],
    );
    let o = run("rterror_dump", &w, None, "0");
    assert_eq!(o.status.code(), Some(200));
    assert_eq!(
        stdout(&o),
        format!("Runtime error 200\n{}", dump("e", "cursor 1 1\nexit 200\n"))
    );
}

#[test]
fn sleep_moves_the_clock_by_that_many_milliseconds_and_returns_at_once() {
    let w = module(
        &["screen", "time", "sleep"],
        &[Step::SleepSeconds(0, 3000), Step::SleepSeconds(1, 7500)],
    );
    let started = std::time::Instant::now();
    // 10:20:30.000 local time
    let o = run(
        "sleep",
        &w,
        None,
        &(37_230_000u64 + 86_400_000 * 20_000).to_string(),
    );
    assert!(started.elapsed().as_secs() < 5, "the runner waited");
    assert_eq!(o.status.code(), Some(0), "{}", stderr(&o));
    assert_eq!(stdout(&o).lines().next(), Some("37"));
}

#[test]
fn an_unknown_key_in_the_script_is_a_runner_error() {
    let w = module(&["screen", "readkey"], &[Step::Key(0)]);
    for bad in ["a Bogus", "ab", "F13"] {
        let o = run("bad_key", &w, Some(bad), "0");
        assert_eq!(o.status.code(), Some(2), "{bad}: {}", stderr(&o));
        assert!(
            stderr(&o).contains("run.mjs: bad key script: unknown key"),
            "{}",
            stderr(&o)
        );
    }
}

#[test]
fn a_missing_key_script_file_is_a_runner_error() {
    let w = scratch("missing_keys.wasm");
    std::fs::write(&w, module(&["screen"], &[])).unwrap();
    let o = Command::new("node")
        .arg(root().join("tests/run.mjs"))
        .arg("--keys")
        .arg(scratch("does_not_exist.keys"))
        .arg(w)
        .output()
        .unwrap();
    assert_eq!(o.status.code(), Some(2));
    assert!(stderr(&o).contains("cannot read"), "{}", stderr(&o));
}

#[test]
fn screen_operations_off_the_screen_or_unknown_are_runner_errors() {
    for (name, step) in [
        ("off_col", Step::Screen(PUT, 80, 0, 0x41)),
        ("off_row", Step::Screen(PUT, 0, 25, 0x41)),
        ("bad_op", Step::Screen(9, 0, 0, 0)),
    ] {
        let o = run(name, &module(&["screen"], &[step]), None, "0");
        assert_eq!(o.status.code(), Some(2), "{name}: {}", stderr(&o));
        assert!(stderr(&o).contains("host.screen"), "{name}: {}", stderr(&o));
    }
}
