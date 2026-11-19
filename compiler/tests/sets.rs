//! Sets (decisions 11, 13, 21, 22; D5): `set of T` (32 bytes, bit i = element i), constructors with
//! constant and variable elements and ranges, `in`, `+ * -`, `= <> <= >=`, assignment, value and
//! `var` parameters, sets in records and arrays, and untyped set constants. Every `.out` and `.err`
//! (`N LINE`) comes from the real TP 5.0 (`tools/tp5.sh`). No program prints `SizeOf` of a set,
//! uses `Include`/`Exclude` (error 3 in TP 5.0), or builds an element outside 0..255.
mod common;
mod driver;
use driver::*;
use std::path::{Path, PathBuf};

fn fx(sub: &str) -> PathBuf {
    root().join("compiler/tests/fixtures/sets").join(sub)
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
    assert!(stems(&fx("lang"), ".pas").len() >= 5);
    assert_eq!(stems(&fx("lang"), ".pas"), stems(&fx("lang"), ".out"));
    for s in stems(&fx("lang"), ".out") {
        let out = std::fs::read(fx("lang").join(format!("{s}.out"))).unwrap();
        assert!(!out.is_empty(), "{s}: empty expected output");
    }
    assert_clean(check_lang(&fx("lang"), &real_compile));
}

#[test]
fn own_invalid_programs_report_tp5_number_and_line() {
    assert!(stems(&fx("neg"), ".pas").len() >= 20);
    assert_eq!(stems(&fx("neg"), ".pas"), stems(&fx("neg"), ".err"));
    assert_clean(check_neg(&fx("neg"), &real_compile));
}

/// Corpus programs (tests/lang, tests/abi, tests/neg) that this slice must make pass. Left out:
/// `mixed_pointer_set_real_params_one_call` and `set_of_real` (need Real). `set_in_with_non_set`
/// runs below with its Real declarations removed.
const LANG: &[&str] = &[
    "set_algebra_identities_over_all_elements",
    "set_assignment_copies_and_aliasing",
    "set_comparisons",
    "set_constant_declaration",
    "set_constructors_and_ranges",
    "set_distinct_letters_and_word_membership",
    "set_dynamic_bounds_and_empty_ranges",
    "set_in_case_and_loops",
    "set_membership_with_in",
    "set_of_char_boundaries",
    "set_of_char_classification",
    "set_of_enum",
    "set_of_subrange_and_boolean",
    "set_sieve_of_byte_with_gaps",
    "set_sieve_of_eratosthenes",
    "set_union_intersection_difference",
    "set_value_parameter_is_a_copy",
];

const ABI: &[&str] = &[
    "set_frames_with_set_locals_and_recursion",
    "set_in_record_and_array_of_sets",
    "set_value_parameter_32_byte_copy",
];

const NEG: &[&str] = &[
    "set_assign_different_base_type",
    "set_base_type_range_over_255",
    "set_constructor_range_wrong_type",
    "set_element_type_mismatch",
    "set_function_result_type",
    "set_include_unavailable",
    "set_less_than",
    "set_plus_integer",
    "set_write",
];

/// Corpus negatives whose shared `var` line declares Real variables, which wait for the Real
/// type: they run here with `r: Real` removed and `pr: ^Real` made `^Byte` (same line, so the
/// same diagnostic).
const NEG_REAL: &[&str] = &["set_in_with_non_set"];

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
fn corpus_neg_programs_report_tp5_number_and_line() {
    assert_clean(check_neg(
        &subset("neg", NEG, &["pas", "err"]),
        &real_compile,
    ));
}

#[test]
fn corpus_neg_programs_without_real_declarations_report_tp5_number_and_line() {
    let dir = subset("neg", NEG_REAL, &["err"]);
    for n in NEG_REAL {
        let src = std::fs::read_to_string(root().join(format!("tests/neg/{n}.pas"))).unwrap();
        assert!(
            src.contains("r: Real; ") && src.contains("pr: ^Real; "),
            "{n}"
        );
        let src = src
            .replace("r: Real; ", "")
            .replace("pr: ^Real; ", "pr: ^Byte; ");
        std::fs::write(dir.join(format!("{n}.pas")), src).unwrap();
    }
    assert_clean(check_neg(&dir, &real_compile));
}

#[test]
fn corpus_lists_are_sorted_and_name_existing_programs() {
    for (kind, names, ext) in [
        ("lang", LANG, "out"),
        ("abi", ABI, "out"),
        ("neg", NEG, "err"),
        ("neg", NEG_REAL, "err"),
    ] {
        assert!(!names.is_empty());
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
    assert_eq!((LANG.len(), ABI.len(), NEG.len()), (17, 3, 9));
}

fn compile(src: &str) -> Vec<u8> {
    tp5c::compile(src.as_bytes())
        .unwrap_or_else(|d| panic!("rejected with {} at {}:\n{src}", d.code, d.line))
}

fn rejected(src: &str) -> (u16, u32) {
    match tp5c::compile(src.as_bytes()) {
        Ok(_) => panic!("accepted:\n{src}"),
        Err(d) => (d.code, d.line),
    }
}

/// Set helpers live in System and are emitted only when a live body uses them (decision 14); sets
/// add nothing to a program without them.
#[test]
fn set_helpers_are_emitted_only_when_used() {
    let hello = compile("program Hello;\nbegin\n  WriteLn('hello')\nend.");
    assert!(hello.len() <= 430, "hello-world is {} bytes", hello.len());
    let plain = compile("program Prog;\nvar i: Integer;\nbegin\n  i := 1;\n  WriteLn(i)\nend.");
    let dead = compile(
        "program Prog;\ntype S = set of Byte;\nprocedure Dead;\nvar a, b: S; i: Integer;\nbegin\n  a := [1, 2..9] + b * [i];\n  if (3 in a) or (a <= b) or (a = b) then a := a - b\nend;\nvar i: Integer;\nbegin\n  i := 1;\n  WriteLn(i)\nend.",
    );
    assert_eq!(
        plain, dead,
        "an unused routine using sets changed the module"
    );
    let used = compile(
        "program Prog;\ntype S = set of Byte;\nvar a: S; i: Integer;\nbegin\n  i := 1;\n  a := [i] + [3];\n  WriteLn(7 in a)\nend.",
    );
    assert!(used.len() > plain.len());
}

/// User declarations named like the System helpers must not capture what the compiler calls
/// (helpers are bound by handle, decision 11). The expected output follows from the user code
/// only: the user routines have a different meaning from any plausible helper.
#[test]
fn user_names_do_not_capture_set_helpers() {
    let src = "program Clash;\ntype S = set of Byte;\nvar a, b: S; SetUnion, SetOp, SetIn, SetCmp, SetLe, SetEq, SetAdd, SetRange, SetInter, SetDiff: Integer;\nfunction SetMember(i: Integer): Integer;\nbegin\n  SetMember := -i\nend;\nprocedure SetCopy(var x: Integer);\nbegin\n  x := 77\nend;\nbegin\n  SetUnion := 1; SetOp := 2; SetIn := 3; SetCmp := 4; SetLe := 5; SetEq := 6; SetAdd := 7;\n  SetRange := 8; SetInter := 9; SetDiff := 10;\n  a := [1, 2..5]; b := a + [9] - [2] * a;\n  SetCopy(SetUnion);\n  WriteLn(3 in b, ' ', 2 in b, ' ', a <= b, ' ', a = b, ' ', SetMember(4), ' ', SetUnion);\n  WriteLn(SetOp + SetIn + SetCmp + SetLe + SetEq + SetAdd + SetRange + SetInter + SetDiff)\nend.\n";
    let dir = scratch("x").parent().unwrap().to_path_buf();
    std::fs::write(dir.join("clash.pas"), src).unwrap();
    std::fs::write(dir.join("clash.out"), "TRUE FALSE FALSE FALSE -4 77\n54\n").unwrap();
    assert_clean(check_lang(&dir, &real_compile));
}

#[test]
fn set_rules_in_the_checker() {
    let wrap = |decl: &str, body: &str| format!("program Q;\n{decl}\nbegin\n  {body}\nend.");
    let decl = "type BS = set of Byte; CS = set of Char;\nvar s, t: BS; c: CS; i: Integer; ch: Char; b: Boolean;";
    for ok in [
        "s := [1, 2, 3]; t := s + [4]; s := s * t - [1]",
        "b := (3 in s) and (ch in c)",
        "b := s = t; b := s <> t; b := s <= t; b := s >= t; b := [] = s",
        "s := []; c := ['a'..'z', '_']; i := 3; s := [i, i + 1..i + 5]",
        "b := ch in ['a'..'z']; b := i in [1, 2]",
        "b := i in s + t",
        "s := s + [] - []; c := [] + c",
        "b := s <= [1..3] + t",
    ] {
        compile(&wrap(decl, ok));
    }
    for (bad, want) in [
        ("s := c", (26, 6)),
        ("b := 'a' in s", (26, 6)),
        ("b := s < t", (41, 6)),
        ("b := s > t", (41, 6)),
        ("s := s + 1", (26, 6)),
        ("s := -s", (41, 6)),
        ("b := c = s", (26, 6)),
        ("Write(s)", (64, 5)),
        ("b := s in s", (26, 6)),
        ("c := [1]", (26, 6)),
        ("s := [1, 'a']", (26, 5)),
        ("i := s", (26, 6)),
        ("s := [i..ch]", (26, 5)),
        ("b := i in i", (41, 6)),
        ("s := s mod t", (41, 6)),
        ("b := s <= 1", (26, 6)),
        ("b := 1 in 1", (41, 6)),
        ("s := [s]", (39, 5)),
        ("s := [t]", (39, 5)),
        ("c := c + s", (26, 6)),
        ("b := s xor t", (41, 6)),
        ("b := [1] = [ch]", (26, 6)),
        ("b := ch in s", (26, 6)),
        ("b := s <= c", (26, 6)),
    ] {
        assert_eq!(rejected(&wrap(decl, bad)), want, "{bad}");
    }
}
