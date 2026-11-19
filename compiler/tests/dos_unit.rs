//! The Dos unit: an embedded unit (found by `uses Dos` before any registered file, as TP5 takes
//! it from its own TURBO.TPL) with `GetDate` and `GetTime` computed in Pascal from the host
//! import `time` (docs/abi.md section 8).
//!
//! The corpus `dos_*` programs and `tests/fixtures/dos/neg` come from the real TP 5.0. The
//! oracle's clock cannot be set, so the date arithmetic is pinned with fixed clocks: the runner
//! (`tests/run.mjs`) reads the local time in milliseconds since 1970-01-01 from the environment
//! variable `TP5_NOW_MS`. Those expectations are calendar facts (weekdays and leap years
//! worked out independently of the runtime), not TP output.
mod common;
use common::OutputGuarded as _;
mod driver;
use common::node;
use driver::*;
use std::path::{Path, PathBuf};
use std::process::Command;

const LANG: &[&str] = &[
    "dos_get_date_values_are_in_range",
    "dos_get_time_values_are_in_range",
    "dos_names_can_be_qualified_and_variables_reused",
];
const NEG: &[&str] = &["dos_get_date_needs_four_arguments"];

fn assert_clean(failures: Vec<String>) {
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn work() -> PathBuf {
    scratch("x").parent().unwrap().to_path_buf()
}

/// Copies the corpus programs `names` and their expectation into a fresh directory.
fn subset(kind: &str, names: &[&str], expected: &str) -> PathBuf {
    let from = root().join("tests").join(kind);
    let dir = work();
    for n in names {
        for e in ["pas", expected] {
            std::fs::copy(from.join(format!("{n}.{e}")), dir.join(format!("{n}.{e}"))).unwrap();
        }
    }
    dir
}

#[test]
fn corpus_lang_programs_match_tp5_output() {
    assert_clean(check_lang(&subset("lang", LANG, "out"), &real_compile));
}

#[test]
fn corpus_neg_program_reports_tp5_number_and_line() {
    assert_clean(check_neg(&subset("neg", NEG, "err"), &real_compile));
}

#[test]
fn corpus_programs_leave_pending() {
    for (kind, names) in [("lang", LANG), ("neg", NEG)] {
        let pending =
            std::fs::read_to_string(root().join(format!("tests/{kind}/PENDING"))).unwrap();
        for n in names {
            assert!(!pending.lines().any(|l| l.trim() == *n), "{kind}/{n}");
        }
    }
}

#[test]
fn own_invalid_programs_report_tp5_number_and_line() {
    let dir = root().join("compiler/tests/fixtures/dos/neg");
    let count = std::fs::read_dir(&dir).unwrap().count();
    assert_eq!(count, 8, "four programs with an .err each");
    assert_clean(check_neg(&dir, &real_compile));
}

// ---- Date and time arithmetic with a fixed clock ----

const SHOW_CLOCK: &str = "program Clock;\nuses Dos;\nvar y, mo, d, w, h, mi, s, c: Word;\n\
    begin\n  GetDate(y, mo, d, w);\n  GetTime(h, mi, s, c);\n  \
    WriteLn(y, '-', mo, '-', d, ' ', w, ' ', h, ':', mi, ':', s, '.', c);\nend.\n";

/// Local clock in milliseconds since 1970-01-01 00:00:00 and what the program prints: date,
/// day of week (0 is Sunday), time and hundredths.
const CLOCKS: &[(u64, &str)] = &[
    (0, "1970-1-1 4 0:0:0.0"),
    (94694399990, "1972-12-31 0 23:59:59.99"),
    (315532800000, "1980-1-1 2 0:0:0.0"),
    (946684799990, "1999-12-31 5 23:59:59.99"),
    (951827696780, "2000-2-29 2 12:34:56.78"),
    (951868801010, "2000-3-1 3 0:0:1.1"),
    (1709186828090, "2024-2-29 4 6:7:8.9"),
    (1767139200000, "2025-12-31 3 0:0:0.0"),
    (2147483648500, "2038-1-19 2 3:14:8.50"),
    (4102444799999, "2099-12-31 4 23:59:59.99"),
    (4107542399000, "2100-2-28 0 23:59:59.0"),
    (4107542400000, "2100-3-1 1 0:0:0.0"),
];

fn compile_clock(dir: &Path) {
    std::fs::write(dir.join("clock.pas"), SHOW_CLOCK).unwrap();
    let r = real_compile(&dir.join("clock.pas"), &dir.join("clock.wasm"));
    assert_eq!((r.code, r.stderr.as_str()), (0, ""));
}

fn run_with_clock(dir: &Path, now_ms: u64) -> String {
    let o = Command::new("node")
        .arg(root().join("tests/run.mjs"))
        .arg(dir.join("clock.wasm"))
        .env("TP5_NOW_MS", now_ms.to_string())
        .output_guarded()
        .expect("node must be installed");
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap()
}

#[test]
fn a_fixed_clock_gives_the_calendar_date_weekday_and_time() {
    let dir = work();
    compile_clock(&dir);
    for (now, want) in CLOCKS {
        let got = run_with_clock(&dir, *now);
        assert!(!got.is_empty(), "{now}: no output");
        assert_eq!(got, format!("{want}\n"), "clock {now}");
    }
}

#[test]
fn the_runner_clock_defaults_to_the_real_time() {
    let dir = work();
    compile_clock(&dir);
    let o = Command::new("node")
        .arg(root().join("tests/run.mjs"))
        .arg(dir.join("clock.wasm"))
        .env_remove("TP5_NOW_MS")
        .output_guarded()
        .unwrap();
    let out = String::from_utf8(o.stdout).unwrap();
    let year: u32 = out.split('-').next().unwrap().parse().unwrap();
    assert!((2024..2100).contains(&year), "{out}");
}

// ---- The embedded unit and its import ----

fn imports(wasm: &[u8]) -> Vec<String> {
    let js = "for (const i of WebAssembly.Module.imports(new WebAssembly.Module(b))) \
              console.log(i.module + '.' + i.name);";
    node(js, wasm).lines().map(String::from).collect()
}

#[test]
fn the_time_import_is_present_only_when_dos_routines_are_called() {
    let used = tp5c::compile(SHOW_CLOCK.as_bytes()).unwrap();
    let names = imports(&used);
    assert!(names.contains(&"host.write".to_string()), "{names:?}");
    assert_eq!(
        names.iter().filter(|n| *n == "host.time").count(),
        1,
        "{names:?}"
    );

    let named_only = tp5c::compile(b"program P;\nuses Dos;\nbegin\nend.\n").unwrap();
    assert!(!imports(&named_only).contains(&"host.time".to_string()));
}

#[test]
fn a_user_file_called_dos_does_not_replace_the_unit() {
    // TP5 takes Dos from its own library whatever is in the directory (measured: the user's
    // DOS.PAS constant is an unknown identifier).
    let dir = work();
    std::fs::write(
        dir.join("DOS.PAS"),
        "unit Dos;\ninterface\nconst Marker = 42;\nimplementation\nend.\n",
    )
    .unwrap();
    std::fs::write(dir.join("clock.pas"), SHOW_CLOCK).unwrap();
    let r = real_compile(&dir.join("clock.pas"), &dir.join("clock.wasm"));
    assert_eq!((r.code, r.stderr.as_str()), (0, ""));
    assert_eq!(run_with_clock(&dir, 0), "1970-1-1 4 0:0:0.0\n");

    std::fs::write(
        dir.join("marker.pas"),
        "program P;\nuses Dos;\nbegin\n  WriteLn(Marker);\nend.\n",
    )
    .unwrap();
    let r = real_compile(&dir.join("marker.pas"), &dir.join("marker.wasm"));
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("Error 3:"), "{}", r.stderr);
}
