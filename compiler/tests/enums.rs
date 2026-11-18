//! Enumerated and subrange types. Every `.out` and `.err` (`N LINE`) under
//! tests/fixtures/enums comes from the real TP 5.0 (`tools/tp5.sh`); no range checking is
//! involved (that is `{$R+}`, Phase 4), so values outside a subrange are stored and printed.
mod common;
mod driver;
use driver::*;
use std::path::PathBuf;

fn dir(sub: &str) -> PathBuf {
    root().join("compiler/tests/fixtures/enums").join(sub)
}

fn assert_clean(failures: Vec<String>) {
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn stems(sub: &str, ext: &str) -> Vec<String> {
    let mut v: Vec<String> = std::fs::read_dir(dir(sub))
        .unwrap()
        .filter_map(|e| e.unwrap().file_name().into_string().ok())
        .filter_map(|n| n.strip_suffix(ext).map(String::from))
        .collect();
    v.sort();
    v
}

#[test]
fn programs_print_what_tp5_prints() {
    assert!(stems("lang", ".pas").len() >= 20);
    assert_eq!(stems("lang", ".pas"), stems("lang", ".out"));
    assert_clean(check_lang(&dir("lang"), &real_compile));
}

#[test]
fn invalid_programs_report_tp5_number_and_line() {
    assert!(stems("neg", ".pas").len() >= 40);
    assert_eq!(stems("neg", ".pas"), stems("neg", ".err"));
    assert_clean(check_neg(&dir("neg"), &real_compile));
}

#[test]
fn expected_output_is_not_empty() {
    for s in stems("lang", ".out") {
        let out = std::fs::read(dir("lang").join(format!("{s}.out"))).unwrap();
        assert!(!out.is_empty(), "{s}");
    }
}

/// Range checking is Phase 4: the directive is still reported as unsupported (200).
#[test]
fn range_check_directive_is_still_unsupported() {
    let src = b"program P;\n{$R+}\ntype T = 1..3;\nbegin\nend.";
    assert_eq!(tp5c::compile(src).unwrap_err().code, 200);
}
