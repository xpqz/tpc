//! Real (decisions 19, 22; D1): f64 values, literals and constants, `+ - * /` with Integer to Real
//! promotion, `/` always Real, comparisons, `Round`, `Trunc`, `Sqrt`, `Abs`, `Sqr`, `Int`, `Frac`,
//! `Pi`, division by zero as runtime error 200, and `Write(x)`, `Write(x:w)`, `Write(x:w:d)` through
//! `host.fmt_real`. Every `.out` and `.err` (`N LINE`) comes from the real TP 5.0 (`tools/tp5.sh`).
//! Programs avoid exact decimal ties of magnitude 8 and above, where TP 5.0 rounds some down.
mod common;
mod driver;
use driver::*;
use std::path::{Path, PathBuf};

fn fx(sub: &str) -> PathBuf {
    root().join("compiler/tests/fixtures/reals").join(sub)
}

fn assert_clean(failures: Vec<String>) {
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn stems(dir: &Path, ext: &str) -> Vec<String> {
    let mut v: Vec<String> = std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.unwrap().file_name().into_string().ok())
        .filter_map(|n| n.strip_suffix(ext).map(String::from))
        .collect();
    v.sort();
    v
}

#[test]
fn own_lang_programs_print_what_tp5_prints() {
    assert!(stems(&fx("lang"), ".pas").len() >= 8);
    assert_eq!(stems(&fx("lang"), ".pas"), stems(&fx("lang"), ".out"));
    for s in stems(&fx("lang"), ".out") {
        let out = std::fs::read(fx("lang").join(format!("{s}.out"))).unwrap();
        assert!(!out.is_empty(), "{s}: empty expected output");
    }
    assert_clean(check_lang(&fx("lang"), &real_compile));
}

#[test]
fn own_division_by_zero_programs_halt_with_200() {
    assert_eq!(stems(&fx("rt"), ".pas"), stems(&fx("rt"), ".out"));
    assert_eq!(stems(&fx("rt"), ".pas").len(), 3);
    for s in stems(&fx("rt"), ".out") {
        let out = std::fs::read_to_string(fx("rt").join(format!("{s}.out"))).unwrap();
        assert!(out.ends_with("Runtime error 200\n"), "{s}");
    }
    assert_clean(check_rt(&fx("rt"), &real_compile));
}

#[test]
fn own_invalid_programs_report_tp5_number_and_line() {
    assert!(stems(&fx("neg"), ".pas").len() >= 30);
    assert_eq!(stems(&fx("neg"), ".pas"), stems(&fx("neg"), ".err"));
    assert_clean(check_neg(&fx("neg"), &real_compile));
}

/// Corpus programs that need Real and nothing else (no `Inc`, `Str`, `Val`, `ParamCount`).
/// `Pi`, `Int` and `Frac` are in scope. The negatives run unstripped: `r: Real` stays declared.
const LANG: &[&str] = &[
    "pointer_generic_pointer_assignments",
    "real_accumulate_in_loops",
    "real_arithmetic_exact_values",
    "real_arrays_and_records",
    "real_comparisons_and_mixed",
    "real_compound_interest_computed",
    "real_constants_and_literals",
    "real_function_parameters_and_results",
    "real_int_frac_and_pi",
    "real_integer_promotion",
    "real_large_and_small_exponents",
    "real_mixed_longint_arithmetic",
    "real_negative_values_formatting",
    "real_newton_iteration_computed",
    "real_round_and_trunc",
    "real_slash_always_gives_real",
    "real_sqrt_irrational_to_few_digits",
    "real_sqrt_sqr_abs",
    "real_write_default_format",
    "real_write_mixed_with_other_types",
    "real_write_rounding_of_exact_ties",
    "real_write_width_and_decimals",
    "real_write_width_without_decimals",
    "real_zero_and_signs",
];

const ABI: &[&str] = &[
    "mixed_pointer_set_real_params_one_call",
    "real_function_in_expression_with_calls",
    "real_record_value_parameter_and_result_use",
    "real_value_parameters_in_frame_order",
    "real_var_parameter_and_locals",
];

const RT: &[&str] = &["div_by_zero_real_slash"];

const NEG: &[&str] = &[
    "address_of_constant",
    "array_index_real_literal",
    "array_non_ordinal_index_type",
    "for_control_variable_real",
    "freemem_missing_size",
    "getmem_non_pointer",
    "ordinal_expression_expected_case_real",
    "pointer_arithmetic_plus",
    "pointer_assign_integer",
    "pointer_assign_other_pointer_type",
    "pointer_deref_non_pointer",
    "pointer_new_non_pointer",
    "pointer_nil_assigned_to_integer",
    "pointer_ordering_comparison",
    "pointer_write",
    "real_and_operator",
    "real_as_array_index",
    "real_assigned_to_integer",
    "real_div_operator",
    "real_ord_argument",
    "round_of_integer_string",
    "set_in_with_non_set",
    "set_of_real",
    "sqrt_of_string",
    "sqrt_without_argument",
    "upcase_integer_argument",
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
fn corpus_abi_programs_match_tp5_output_and_restore_sp() {
    assert_clean(check_lang(
        &subset("abi", ABI, &["pas", "out"]),
        &real_compile,
    ));
}

#[test]
fn corpus_division_by_zero_halts_with_200() {
    assert_clean(check_rt(&subset("rt", RT, &["pas", "out"]), &real_compile));
}

#[test]
fn corpus_neg_programs_report_tp5_number_and_line() {
    assert_clean(check_neg(
        &subset("neg", NEG, &["pas", "err"]),
        &real_compile,
    ));
}

#[test]
fn corpus_lists_are_sorted_name_existing_programs_and_leave_pending() {
    for (kind, names, ext) in [
        ("lang", LANG, "out"),
        ("abi", ABI, "out"),
        ("rt", RT, "out"),
        ("neg", NEG, "err"),
    ] {
        let mut sorted = names.to_vec();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted, names, "{kind} list must be sorted and unique");
        let pending =
            std::fs::read_to_string(root().join(format!("tests/{kind}/PENDING"))).unwrap();
        for n in names {
            for e in ["pas", ext] {
                let p = root().join(format!("tests/{kind}/{n}.{e}"));
                assert!(p.exists(), "{}", p.display());
            }
            assert!(
                !pending.lines().any(|l| l.trim() == *n),
                "{kind}/{n} is still in PENDING"
            );
        }
    }
    assert_eq!((LANG.len(), ABI.len(), RT.len(), NEG.len()), (24, 5, 1, 26));
}

/// `host/fmt_real.js` reproduces TP 5.0 formatting for 400+ (value, width, decimals) cases, maps
/// -0 to 0, and `fmtReal` writes the digits into memory and returns the length.
#[test]
fn fmt_real_matches_tp5_output() {
    let o = std::process::Command::new("node")
        .arg(root().join("tests/host/fmt_real.mjs"))
        .output()
        .expect("node must be installed");
    let err = String::from_utf8_lossy(&o.stderr);
    assert!(o.status.success(), "{err}");
}

/// A program with no Real adds nothing to the module (the formatter is imported only when used).
#[test]
fn programs_without_real_do_not_import_fmt_real() {
    let src = "program Hello;\nbegin\n  WriteLn('hello')\nend.";
    let wasm = tp5c::compile(src.as_bytes()).unwrap();
    assert!(!wasm.windows(8).any(|w| w == b"fmt_real"));
    let src = "program R;\nvar x: Real;\nbegin\n  x := 1.5;\n  WriteLn(x:0:1)\nend.";
    let wasm = tp5c::compile(src.as_bytes()).unwrap();
    assert!(wasm.windows(8).any(|w| w == b"fmt_real"));
}

fn rejected(src: &str) -> (u16, u32) {
    match tp5c::compile(src.as_bytes()) {
        Ok(_) => panic!("accepted:\n{src}"),
        Err(d) => (d.code, d.line),
    }
}

fn compile(src: &str) {
    tp5c::compile(src.as_bytes())
        .unwrap_or_else(|d| panic!("rejected with {} at {}:\n{src}", d.code, d.line));
}

#[test]
fn real_rules_in_the_checker() {
    let wrap = |body: &str| {
        format!(
            "program Prog;\nvar i: Integer; l: LongInt; r, q: Real; b: Boolean; a: array[1..2] of Real;\nbegin\n  {body};\nend."
        )
    };
    for ok in [
        "r := 1; r := i; r := l; r := i + l; r := i / 2; r := -r; r := - -i",
        "r := r + i; r := i * r; r := r / i; r := i / i; r := (r - 1) * 2.5E-1",
        "b := r < i; b := i >= r; b := r = 1; b := r <> q; b := 2.5 > l",
        "l := Round(r); l := Trunc(r); i := Round(r); r := Sqrt(i); r := Sqr(r); r := Abs(i)",
        "r := Int(r); r := Frac(i); r := Pi; r := Pi * 2; l := Trunc(Pi)",
        "a[1] := i; a[2] := a[1] / 2; r := a[1] + a[2]",
        "Write(r); Write(r:5); Write(r:5:2); Write(r:i:i); Write(i / 2:0:1)",
        "WriteLn(r:0:0, l:3, 1.5, ' ', r / 3:i + 2:i - 1)",
    ] {
        compile(&wrap(ok));
    }
    for (bad, want) in [
        ("i := r", (26, 4)),
        ("l := i / 2", (26, 4)),
        ("l := 1.5", (26, 4)),
        ("r := 'a'", (26, 4)),
        ("r := b", (26, 4)),
        ("a := r", (26, 4)),
        ("r := r mod 2", (41, 4)),
        ("r := r div 2", (41, 4)),
        ("b := not r", (41, 4)),
        ("r := r shl 1", (41, 4)),
        ("if r then i := 1", (40, 4)),
        ("Write(i:2:1)", (89, 4)),
        ("Write(r:q)", (38, 4)),
        ("Write(r:2.5)", (38, 4)),
        ("l := Trunc('a')", (79, 4)),
        ("r := Abs('a')", (79, 4)),
        ("r := Sqrt(r, r)", (89, 4)),
        ("l := Round", (88, 4)),
    ] {
        assert_eq!(rejected(&wrap(bad)), want, "{bad}");
    }
}

#[test]
fn real_var_parameters_need_the_identical_type() {
    let p = "program Prog;\nprocedure P(var x: Real);\nbegin\n  x := 1\nend;\nvar i: Integer; r: Real;\nbegin\n  P(r);\n  P(i)\nend.";
    assert_eq!(rejected(p), (26, 9));
}
