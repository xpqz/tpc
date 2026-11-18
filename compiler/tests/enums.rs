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
    assert!(stems("lang", ".pas").len() >= 15);
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

/// Phase 3 corpus programs (tests/lang, tests/neg) in this slice's scope; the others there need
/// arrays, records or strings. `subrange_sizes` observes storage through `SizeOf` of a type name.
const LANG: &[&str] = &[
    "enum_case_selector",
    "enum_comparisons",
    "enum_for_loop",
    "enum_ord_succ_pred",
    "enum_variable_assign_and_ordinal_cast",
    "subrange_basic",
    "subrange_from_constants",
    "subrange_of_enum",
    "subrange_sizes",
];

const NEG: &[&str] = &[
    "enum_arithmetic",
    "enum_assign_integer",
    "enum_assign_other_enum",
    "enum_case_label_wrong_type",
    "enum_compare_with_integer",
    "enum_duplicate_identifier",
    "enum_for_control_integer_with_enum_bounds",
    "enum_identifier_already_declared",
    "enum_low_high_unavailable",
    "enum_write_enum_value",
    "subrange_assign_wider_type_constant_negative",
    "subrange_bounds_type_mismatch",
    "subrange_constant_out_of_range",
    "subrange_low_exceeds_high",
    "subrange_of_non_ordinal",
];

/// Copies `names` with the given extensions into a fresh directory (no PENDING file).
fn subset(kind: &str, names: &[&str], exts: &[&str]) -> PathBuf {
    let dir = scratch("x").parent().unwrap().to_path_buf();
    for n in names {
        for e in exts {
            let f = format!("{n}.{e}");
            std::fs::copy(root().join("tests").join(kind).join(&f), dir.join(&f)).unwrap();
        }
    }
    dir
}

#[test]
fn corpus_lang_programs_match_tp5_output() {
    assert_clean(check_lang(
        &subset("lang", LANG, &["pas", "out"]),
        &real_compile,
    ));
}

#[test]
fn corpus_neg_programs_report_tp5_number_and_line() {
    assert_clean(check_neg(
        &subset("neg", NEG, &["pas", "err"]),
        &real_compile,
    ));
}

#[test]
fn corpus_lists_are_sorted_and_name_existing_programs() {
    for (kind, names, ext) in [("lang", LANG, "out"), ("neg", NEG, "err")] {
        let mut sorted = names.to_vec();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted, names, "{kind} list must be sorted and unique");
        for n in names {
            for e in ["pas", ext] {
                let p = root().join(format!("tests/{kind}/{n}.{e}"));
                assert!(p.exists(), "{}", p.display());
            }
        }
    }
}
