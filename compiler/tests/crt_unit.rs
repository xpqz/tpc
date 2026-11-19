//! The Crt unit: an embedded unit (runtime/crt.pas, found by `uses Crt` before any registered
//! file, like Dos) that keeps the cursor, window and text attribute, and drives the host
//! operations `screen`, `readkey`, `keypressed` and `sleep` (decisions 37 to 39, docs/abi.md
//! section 8). Plain Write and ReadLn go through System's console hook, which Crt's
//! initialisation replaces.
//!
//! The corpus `crt_*` programs (tests/{lang,rt,neg}) and `tests/fixtures/crt` come from the
//! real TP 5.0 (`tools/tp5.sh`); a Crt `.out` is the program's output followed by the screen
//! dump. The virtual clock of `Delay` is not observable in TP 5.0 (decision 38), so those
//! expectations are arithmetic on a fixed clock (`TP5_NOW_MS`), not TP output.
mod common;
use common::OutputGuarded as _;
mod driver;
use common::node;
use driver::*;
use std::path::{Path, PathBuf};
use std::process::Command;

fn assert_clean(failures: Vec<String>) {
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn work() -> PathBuf {
    scratch("x").parent().unwrap().to_path_buf()
}

fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let path = entry.unwrap().path();
        let target = to.join(path.file_name().unwrap());
        if path.is_dir() {
            copy_tree(&path, &target);
        } else {
            std::fs::copy(&path, &target).unwrap();
        }
    }
}

/// The corpus programs `crt_*` of `tests/KIND` with everything that belongs to them (expected
/// output, key script, unit directory), in a fresh directory without a PENDING list, so
/// every one of them must pass.
fn crt_corpus(kind: &str) -> PathBuf {
    let from = root().join("tests").join(kind);
    let dir = work();
    let mut count = 0;
    for entry in std::fs::read_dir(&from).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        if !name.starts_with("crt_") {
            continue;
        }
        if path.is_dir() {
            copy_tree(&path, &dir.join(&name));
        } else {
            std::fs::copy(&path, dir.join(&name)).unwrap();
            count += usize::from(name.ends_with(".pas"));
        }
    }
    let want = match kind {
        "lang" => 45,
        "rt" => 4,
        _ => 12,
    };
    assert_eq!(count, want, "crt_* programs in tests/{kind}");
    dir
}

#[test]
fn corpus_lang_programs_match_tp5_output() {
    assert_clean(check_lang(&crt_corpus("lang"), &real_compile));
}

#[test]
fn corpus_runtime_error_programs_match_tp5_output_and_code() {
    assert_clean(check_rt(&crt_corpus("rt"), &real_compile));
}

#[test]
fn corpus_neg_programs_report_tp5_number_and_line() {
    assert_clean(check_neg(&crt_corpus("neg"), &real_compile));
}

#[test]
fn corpus_programs_leave_pending() {
    for kind in ["lang", "rt", "neg"] {
        let pending =
            std::fs::read_to_string(root().join(format!("tests/{kind}/PENDING"))).unwrap();
        let left: Vec<&str> = pending.lines().filter(|l| l.starts_with("crt_")).collect();
        assert!(left.is_empty(), "tests/{kind}/PENDING: {left:?}");
    }
}

#[test]
fn own_programs_match_tp5_output() {
    let dir = root().join("compiler/tests/fixtures/crt/lang");
    let count = std::fs::read_dir(&dir)
        .unwrap()
        .filter(|e| {
            e.as_ref()
                .unwrap()
                .path()
                .extension()
                .is_some_and(|x| x == "pas")
        })
        .count();
    assert_eq!(count, 6, "six programs with an .out each");
    assert_clean(check_lang(&dir, &real_compile));
}

#[test]
fn own_invalid_programs_report_tp5_number_and_line() {
    let dir = root().join("compiler/tests/fixtures/crt/neg");
    let count = std::fs::read_dir(&dir).unwrap().count();
    assert_eq!(count, 14, "seven programs with an .err each");
    assert_clean(check_neg(&dir, &real_compile));
}

// ---- The embedded unit and its imports ----

fn imports(wasm: &[u8]) -> Vec<String> {
    let js = "for (const i of WebAssembly.Module.imports(new WebAssembly.Module(b))) \
              console.log(i.module + '.' + i.name);";
    node(js, wasm).lines().map(String::from).collect()
}

fn count_of(names: &[String], import: &str) -> usize {
    names.iter().filter(|n| *n == import).count()
}

#[test]
fn a_program_without_crt_writes_to_stdout_and_imports_no_screen_operations() {
    let src = b"program P;\nbegin\n  WriteLn('hi');\nend.\n";
    let names = imports(&tp5c::compile(src).unwrap());
    assert!(names.contains(&"host.write".to_string()), "{names:?}");
    for crt_import in ["screen", "readkey", "keypressed", "sleep"] {
        assert_eq!(
            count_of(&names, &format!("host.{crt_import}")),
            0,
            "{names:?}"
        );
    }
}

#[test]
fn the_screen_import_is_present_once_crt_is_used_and_sleep_only_for_delay() {
    let clear = b"program P;\nuses Crt;\nbegin\n  ClrScr;\nend.\n";
    let names = imports(&tp5c::compile(clear).unwrap());
    assert_eq!(count_of(&names, "host.screen"), 1, "{names:?}");
    assert_eq!(count_of(&names, "host.sleep"), 0, "{names:?}");

    let delay = b"program P;\nuses Crt;\nbegin\n  Delay(10);\nend.\n";
    let names = imports(&tp5c::compile(delay).unwrap());
    assert_eq!(count_of(&names, "host.sleep"), 1, "{names:?}");

    let keys = b"program P;\nuses Crt;\nvar c: Char;\nbegin\n  c := ReadKey;\n  \
                 if KeyPressed then c := ReadKey;\nend.\n";
    let names = imports(&tp5c::compile(keys).unwrap());
    assert_eq!(count_of(&names, "host.readkey"), 1, "{names:?}");
    assert_eq!(count_of(&names, "host.keypressed"), 1, "{names:?}");

    let named_only = tp5c::compile(b"program P;\nuses Crt;\nbegin\nend.\n").unwrap();
    assert_eq!(count_of(&imports(&named_only), "host.sleep"), 0);
}

#[test]
fn a_user_file_called_crt_does_not_replace_the_unit() {
    // TP5 takes Crt from its own library whatever is in the directory (measured: the user's
    // CRT.PAS constant is error 3, Unknown identifier, at the use).
    let dir = work();
    std::fs::write(
        dir.join("CRT.PAS"),
        "unit Crt;\ninterface\nconst Marker = 42;\nimplementation\nend.\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("marker.pas"),
        "program P;\nuses Crt;\nbegin\n  WriteLn(Marker);\nend.\n",
    )
    .unwrap();
    let r = real_compile(&dir.join("marker.pas"), &dir.join("marker.wasm"));
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("marker.pas(4,"), "{}", r.stderr);
    assert!(r.stderr.contains("Error 3:"), "{}", r.stderr);
}

// ---- Delay moves the virtual clock (decision 38) ----

const MEASURE_DELAY: &str = "program Measure;\nuses Dos, Crt;\nvar h, m, s, c: Word;\n\
    begin\n  Delay(%MS%);\n  GetTime(h, m, s, c);\n  \
    WriteLn(h, ':', m, ':', s, '.', c);\nend.\n";

/// Runs MEASURE_DELAY with `ms` and the clock fixed at `now_ms`; returns stdout, which for a
/// Crt program is the screen dump.
fn run_measure(ms: u32, now_ms: u64) -> String {
    let dir = work();
    let src = MEASURE_DELAY.replace("%MS%", &ms.to_string());
    std::fs::write(dir.join("measure.pas"), src).unwrap();
    let r = real_compile(&dir.join("measure.pas"), &dir.join("measure.wasm"));
    assert_eq!((r.code, r.stderr.as_str()), (0, ""));
    let o = Command::new("node")
        .arg(root().join("tests/run.mjs"))
        .arg(dir.join("measure.wasm"))
        .env("TP5_NOW_MS", now_ms.to_string())
        .output_guarded()
        .expect("node must be installed");
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap()
}

fn first_screen_row(ms: u32, now_ms: u64) -> String {
    run_measure(ms, now_ms).lines().next().unwrap().to_string()
}

#[test]
fn delay_advances_the_clock_by_the_given_milliseconds() {
    // 2000-02-29 12:34:56.78 (milliseconds since 1970 as in tests/dos_unit.rs).
    let noon = 951827696780;
    // The clock is read after the delay: 12:34:56.78 + 0 / 2.5 s / 65.535 s.
    assert_eq!(first_screen_row(0, noon), "12:34:56.78");
    assert_eq!(first_screen_row(2500, noon), "12:34:59.28");
    assert_eq!(first_screen_row(65535, noon), "12:36:2.31");
}

#[test]
fn crt_output_goes_to_the_screen_not_to_stdout() {
    // Decision 37: the program's text is on the screen; stdout holds the dump only: 25 rows
    // (the text in the first), `cursor` and `exit`, no attribute runs.
    let dump = run_measure(0, 0);
    let lines: Vec<&str> = dump.lines().collect();
    assert_eq!(lines.len(), 27, "{dump}");
    assert_eq!(lines[0], "0:0:0.0");
    assert!(lines[1..25].iter().all(|l| l.is_empty()), "{dump}");
    assert_eq!(&lines[25..], ["cursor 1 2", "exit 0"]);
}
