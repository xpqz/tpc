//! tests/run.mjs file and console-input imports (`fs_open`, `fs_read`, `fs_write`, `fs_close`,
//! `readline`) driven through the CLI options `--dir DIR` and `--in FILE`. The contract is
//! docs/abi.md section 8; tests/host/vfs.mjs checks the file system itself in more detail.
mod common;
use common::*;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};
use tp5c::wasm::*;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn aux_dir() -> PathBuf {
    root().join("tests/host/fixtures/vfs")
}

fn scratch(name: &str) -> PathBuf {
    let dir = root().join("tmp/runner_files");
    std::fs::create_dir_all(&dir).unwrap();
    dir.join(name)
}

// Import indices, in the order `module` declares them.
const WRITE: u32 = 0;
const HALT: u32 = 1;
const READLINE: u32 = 2;
const FS_OPEN: u32 = 3;
const FS_READ: u32 = 4;
const FS_WRITE: u32 = 5;
const FS_CLOSE: u32 = 6;

const NAME_AT: i32 = 256;
const PAYLOAD_AT: i32 = 320;
const BUFFER_AT: i32 = 1024;
const RESET: i32 = 0;
const REWRITE: i32 = 1;
const APPEND: i32 = 2;

/// Module with the console and file imports, `name` at 256, `payload` at 320, and `body` as
/// `main`. Local 0 holds a file handle.
fn module(name: &str, payload: &[u8], body: impl FnOnce(&mut Code)) -> Vec<u8> {
    let mut m = Module::new();
    let write = m.ty(&[I32, I32, I32], &[]);
    let halt = m.ty(&[I32], &[]);
    let readline = m.ty(&[I32, I32], &[I32]);
    let file_op = m.ty(&[I32, I32, I32], &[I32]);
    let close = m.ty(&[I32], &[I32]);
    m.import_func("host", "write", write);
    m.import_func("host", "halt", halt);
    m.import_func("host", "readline", readline);
    m.import_func("host", "fs_open", file_op);
    m.import_func("host", "fs_read", file_op);
    m.import_func("host", "fs_write", file_op);
    m.import_func("host", "fs_close", close);
    let main = m.ty(&[], &[]);
    m.memory(1);
    m.data(NAME_AT as u32, name.as_bytes());
    m.data(PAYLOAD_AT as u32, payload);
    let mut c = Code::new();
    body(&mut c);
    let f = m.func(main, &[I32], c);
    m.export("memory", MEM, 0);
    m.export("main", FUNC, f);
    m.finish()
}

fn open(c: &mut Code, name_len: usize, mode: i32) {
    c.i32_const(NAME_AT)
        .i32_const(name_len as i32)
        .i32_const(mode)
        .idx(CALL, FS_OPEN)
        .idx(LOCAL_SET, 0);
}

fn close(c: &mut Code) {
    c.idx(LOCAL_GET, 0).idx(CALL, FS_CLOSE).op(DROP);
}

fn write_payload(c: &mut Code, len: usize) {
    c.idx(LOCAL_GET, 0)
        .i32_const(PAYLOAD_AT)
        .i32_const(len as i32)
        .idx(CALL, FS_WRITE)
        .op(DROP);
}

/// Reads up to 255 bytes from the open file into the buffer and prints them.
fn print_file(c: &mut Code) {
    c.i32_const(1).i32_const(BUFFER_AT);
    c.idx(LOCAL_GET, 0)
        .i32_const(BUFFER_AT)
        .i32_const(255)
        .idx(CALL, FS_READ);
    c.idx(CALL, WRITE);
}

/// Halts with 100 minus the value on the stack: a negative error code `-e` becomes `100 + e`.
fn halt_100_minus(c: &mut Code) {
    c.idx(LOCAL_SET, 0)
        .i32_const(100)
        .idx(LOCAL_GET, 0)
        .op(I32_SUB);
    c.idx(CALL, HALT);
}

const DROP: u8 = 0x1A;

fn run(wasm: &[u8], args: &[&str], stdin: Option<&[u8]>) -> Output {
    static NEXT: AtomicU32 = AtomicU32::new(0);
    let w = scratch(&format!("prog{}.wasm", NEXT.fetch_add(1, Ordering::SeqCst)));
    std::fs::write(&w, wasm).unwrap();
    let mut child = Command::new("node")
        .arg(root().join("tests/run.mjs"))
        .args(args)
        .arg(&w)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("node must be installed");
    let mut pipe = child.stdin.take().unwrap();
    if let Some(bytes) = stdin {
        pipe.write_all(bytes).unwrap();
    }
    drop(pipe);
    child.wait_with_output().unwrap()
}

fn dir_args() -> Vec<String> {
    vec!["--dir".into(), aux_dir().display().to_string()]
}

fn with_dir(wasm: &[u8]) -> Output {
    let args = dir_args();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    run(wasm, &args, None)
}

fn err(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}

#[test]
fn reset_reads_a_file_of_the_aux_directory_whatever_the_name_case() {
    for name in ["DATA.TXT", "data.txt", "Data.Txt"] {
        let w = module(name, b"", |c| {
            open(c, name.len(), RESET);
            print_file(c);
            close(c);
        });
        let o = with_dir(&w);
        assert_eq!(o.status.code(), Some(0), "{name}: {}", err(&o));
        assert_eq!(o.stdout, b"alpha\r\nbeta\r\n", "{name}");
    }
}

#[test]
fn reset_of_a_missing_file_returns_minus_2() {
    let w = module("NOPE.TXT", b"", |c| {
        open(c, 8, RESET);
        c.idx(LOCAL_GET, 0);
        halt_100_minus(c);
    });
    assert_eq!(with_dir(&w).status.code(), Some(102));
}

#[test]
fn without_a_dir_option_the_file_system_is_empty() {
    let w = module("DATA.TXT", b"", |c| {
        open(c, 8, RESET);
        c.idx(LOCAL_GET, 0);
        halt_100_minus(c);
    });
    assert_eq!(run(&w, &[], None).status.code(), Some(102));
}

#[test]
fn rewritten_file_reads_back_and_the_aux_directory_is_untouched() {
    let before = std::fs::read(aux_dir().join("DATA.TXT")).unwrap();
    let listing = || {
        let mut v: Vec<_> = std::fs::read_dir(aux_dir())
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect();
        v.sort();
        v
    };
    let names = listing();
    let w = module("MADE.TXT", b"hello", |c| {
        open(c, 8, REWRITE);
        write_payload(c, 5);
        close(c);
        open(c, 8, RESET);
        print_file(c);
        close(c);
    });
    let o = with_dir(&w);
    assert_eq!(o.status.code(), Some(0), "{}", err(&o));
    assert_eq!(o.stdout, b"hello");
    assert_eq!(listing(), names);
    assert_eq!(std::fs::read(aux_dir().join("DATA.TXT")).unwrap(), before);
}

#[test]
fn append_adds_to_an_existing_aux_file() {
    let w = module("DATA.TXT", b"gamma", |c| {
        open(c, 8, APPEND);
        write_payload(c, 5);
        close(c);
        open(c, 8, RESET);
        print_file(c);
    });
    let o = with_dir(&w);
    assert_eq!(o.stdout, b"alpha\r\nbeta\r\ngamma", "{}", err(&o));
}

#[test]
fn each_run_starts_from_the_aux_directory_again() {
    let w = module("DATA.TXT", b"", |c| {
        open(c, 8, REWRITE);
        close(c);
        open(c, 8, RESET);
        print_file(c);
    });
    assert_eq!(with_dir(&w).stdout, b"");
    let w = module("DATA.TXT", b"", |c| {
        open(c, 8, RESET);
        print_file(c);
    });
    assert_eq!(with_dir(&w).stdout, b"alpha\r\nbeta\r\n");
}

#[test]
fn closing_an_unknown_handle_returns_minus_6() {
    let w = module("", b"", |c| {
        c.i32_const(99).idx(CALL, FS_CLOSE);
        halt_100_minus(c);
    });
    assert_eq!(run(&w, &[], None).status.code(), Some(106));
}

#[test]
fn a_missing_aux_directory_is_a_runner_error() {
    let w = module("", b"", |_| {});
    let o = run(&w, &["--dir", "no/such/dir"], None);
    assert_eq!(o.status.code(), Some(2));
    assert!(err(&o).starts_with("run.mjs: "), "{}", err(&o));
    assert!(err(&o).contains("no/such/dir"), "{}", err(&o));
}

/// Prints three lines read with `readline`, each followed by `|`; a negative result halts
/// with 100 minus it.
fn three_lines() -> Vec<u8> {
    module("", b"|", |c| {
        for _ in 0..3 {
            c.i32_const(BUFFER_AT).i32_const(255).idx(CALL, READLINE);
            c.idx(LOCAL_SET, 0);
            c.i32_const(1)
                .i32_const(BUFFER_AT)
                .idx(LOCAL_GET, 0)
                .idx(CALL, WRITE);
            c.i32_const(1)
                .i32_const(PAYLOAD_AT)
                .i32_const(1)
                .idx(CALL, WRITE);
        }
    })
}

#[test]
fn readline_reads_lines_from_the_in_file() {
    let input = scratch("lines.in");
    std::fs::write(&input, b"one\r\ntwo\n\nlast").unwrap();
    let o = run(&three_lines(), &["--in", input.to_str().unwrap()], None);
    assert_eq!(o.status.code(), Some(0), "{}", err(&o));
    assert_eq!(o.stdout, b"one|two||");
}

#[test]
fn readline_without_in_option_reads_the_process_stdin() {
    let o = run(&three_lines(), &[], Some(b"from\nstdin\n\n"));
    assert_eq!(o.status.code(), Some(0), "{}", err(&o));
    assert_eq!(o.stdout, b"from|stdin||");
}

/// A negative `readline` result at end of input is the contract: the line length is -1.
#[test]
fn readline_returns_minus_1_at_end_of_input() {
    let w = module("", b"", |c| {
        c.i32_const(BUFFER_AT).i32_const(255).idx(CALL, READLINE);
        halt_100_minus(c);
    });
    assert_eq!(run(&w, &[], Some(b"")).status.code(), Some(101));
    let input = scratch("one.in");
    std::fs::write(&input, b"x\n").unwrap();
    let w = module("", b"", |c| {
        for _ in 0..2 {
            c.i32_const(BUFFER_AT)
                .i32_const(255)
                .idx(CALL, READLINE)
                .op(DROP);
        }
        c.i32_const(BUFFER_AT).i32_const(255).idx(CALL, READLINE);
        halt_100_minus(c);
    });
    assert_eq!(
        run(&w, &["--in", input.to_str().unwrap()], None)
            .status
            .code(),
        Some(101)
    );
}

#[test]
fn readline_cuts_a_long_line_to_max() {
    let input = scratch("long.in");
    std::fs::write(&input, b"abcdefgh\n").unwrap();
    let w = module("", b"", |c| {
        c.i32_const(1).i32_const(BUFFER_AT);
        c.i32_const(BUFFER_AT).i32_const(3).idx(CALL, READLINE);
        c.idx(CALL, WRITE);
    });
    let o = run(&w, &["--in", input.to_str().unwrap()], None);
    assert_eq!(o.stdout, b"abc", "{}", err(&o));
}

#[test]
fn a_missing_in_file_is_a_runner_error() {
    let w = module("", b"", |_| {});
    let o = run(&w, &["--in", "no/such.in"], None);
    assert_eq!(o.status.code(), Some(2));
    assert!(err(&o).contains("no/such.in"), "{}", err(&o));
}

#[test]
fn rterror_prints_the_code_with_three_digits() {
    let mut m = Module::new();
    let rt = m.ty(&[I32], &[]);
    m.import_func("host", "rterror", rt);
    let main = m.ty(&[], &[]);
    m.memory(1);
    let mut c = Code::new();
    c.i32_const(2).idx(CALL, 0);
    let f = m.func(main, &[], c);
    m.export("memory", MEM, 0);
    m.export("main", FUNC, f);
    let o = run(&m.finish(), &[], None);
    assert_eq!(o.stdout, b"Runtime error 002\n", "{}", err(&o));
    assert_eq!(o.status.code(), Some(2));
}
