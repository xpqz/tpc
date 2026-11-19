//! `Inc`, `Dec`, `Str`, `Val`, `ParamCount`, `ParamStr`, `Swap`, `Hi`, `Lo` (plan phase 4, item 4).
//! Every `.out` and `.err` (`N LINE`) comes from the real TP 5.0 (`tools/tp5.sh`). Programs print
//! only values TP 5.0 and f64 agree on (no exact ties of magnitude 8 and above, at most 10
//! significant digits, decisions 19). `ParamStr(0)` is not tested (non-empty in TP 5.0).
mod common;
mod driver;
use driver::*;
use std::path::{Path, PathBuf};

fn fx(sub: &str) -> PathBuf {
    root().join("compiler/tests/fixtures/intrinsics").join(sub)
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
    assert_eq!(stems(&fx("lang"), ".pas").len(), 10);
    assert_eq!(stems(&fx("lang"), ".pas"), stems(&fx("lang"), ".out"));
    for s in stems(&fx("lang"), ".out") {
        let out = std::fs::read(fx("lang").join(format!("{s}.out"))).unwrap();
        assert!(!out.is_empty(), "{s}: empty expected output");
    }
    assert_clean(check_lang(&fx("lang"), &real_compile));
}

#[test]
fn own_invalid_programs_report_tp5_number_and_line() {
    assert_eq!(stems(&fx("neg"), ".pas").len(), 34);
    assert_eq!(stems(&fx("neg"), ".pas"), stems(&fx("neg"), ".err"));
    for s in stems(&fx("neg"), ".err") {
        let err = std::fs::read_to_string(fx("neg").join(format!("{s}.err"))).unwrap();
        assert!(err.trim_end().split(' ').count() == 2, "{s}: {err:?}");
    }
    assert_clean(check_neg(&fx("neg"), &real_compile));
}

/// Corpus programs of this slice. The `rangecheck_*` and `range_*` programs belong to range checking.
const LANG: &[&str] = &[
    "inc_dec_default_step",
    "inc_dec_enum_and_array_element",
    "inc_dec_with_step",
    "param_count_and_str_without_arguments",
    "str_integer_forms",
    "str_into_short_string_truncates",
    "str_real_forms",
    "str_val_roundtrip",
    "val_integer_forms",
    "val_into_variable_types_and_position",
    "val_out_of_range_quirks_without_range_checks",
    "val_real_forms",
];

const NEG: &[&str] = &[
    "inc_literal",
    "inc_real",
    "paramcount_with_argument",
    "str_target_not_string",
    "val_code_not_integer_variable",
    "val_missing_arguments",
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
fn corpus_lists_are_sorted_name_existing_programs_and_leave_pending() {
    for (kind, names, ext) in [("lang", LANG, "out"), ("neg", NEG, "err")] {
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
    assert_eq!((LANG.len(), NEG.len()), (12, 6));
}

/// Range checking is a separate slice: its programs stay pending here.
#[test]
fn range_checking_programs_stay_pending() {
    for (kind, prefix, count) in [("lang", "rangecheck_", 5), ("rt", "range_", 14)] {
        let pending =
            std::fs::read_to_string(root().join(format!("tests/{kind}/PENDING"))).unwrap();
        let n = pending.lines().filter(|l| l.starts_with(prefix)).count();
        assert_eq!(n, count, "{kind}/{prefix}*");
    }
}

fn rejected(src: &str) -> (u16, u32) {
    match tp5c::compile(src.as_bytes()) {
        Ok(_) => panic!("accepted:\n{src}"),
        Err(d) => (d.code, d.line),
    }
}

/// A program whose statements are `body` (split one per line; TP 5.0 limits a line to 127).
fn wrap(body: &str) -> String {
    format!(
        "program Prog;\ntype C3 = (Red, Green, Blue);\nvar i, e: Integer; l: LongInt; w: Word; b: Byte; sh: ShortInt;\n  r: Real; c: Char; bo: Boolean; s: String; t: String[3];\n  p: ^Integer; a: array[1..3] of Integer; cc: C3;\nbegin\n  {};\nend.",
        body.replace("; ", ";\n  ")
    )
}

/// Each group is accepted by TP 5.0 as well.
#[test]
fn accepted_forms_compile() {
    for ok in [
        "Inc(i); Dec(i); Inc(i, 2); Dec(i, l); Inc(l, w); Inc(c); Dec(bo); Inc(cc); Inc(a[1], 2); Inc(p^); Inc(i, Ord(c))",
        "Str(i, s); Str(l:5, s); Str(r:0:2, s); Str(r, s); Str(i:w, s); Str(w, s); Str(b * 2, s); Str(i + 1, s); Str(3.5:4:1, s); Str(i, t)",
        "Val(s, i, e); Val(s, l, e); Val(s, w, e); Val(s, b, e); Val(s, r, e); Val(c, i, e); Val(Copy(s, 1, 2), i, e); Val(s, a[1], e); Val(s, sh, e)",
        "i := ParamCount; s := ParamStr(1); s := ParamStr(i); s := ParamStr(l); i := Swap(w); w := Swap(i); i := Hi(l) + Lo(l); b := Hi(i); l := Swap(l)",
    ] {
        let src = wrap(ok);
        tp5c::compile(src.as_bytes())
            .unwrap_or_else(|d| panic!("rejected with {} at {}:\n{src}", d.code, d.line));
    }
}

/// Errors a statement reports at its own line (line 7 of `wrap`).
#[test]
fn statements_inside_a_larger_body_report_their_own_line() {
    let src = wrap("Inc(i); Str(i, s); Val(s, i, e); Inc(r)");
    assert_eq!(rejected(&src), (104, 10));
    let src = wrap("Inc(i); Str(i, s); Val(s, i, r); Inc(i)");
    assert_eq!(rejected(&src), (98, 9));
    let src = wrap("Inc(i); Str(r:0:1, c); Inc(i)");
    assert_eq!(rejected(&src), (66, 8));
}

/// `Inc` and `Dec` are not measurably more than the assignment they stand for; `Str` of an
/// integer and `Val` do not import the real formatter; `Str` of a Real does.
#[test]
fn intrinsics_cost_only_what_they_use() {
    let build = |body: &str| tp5c::compile(wrap(body).as_bytes()).unwrap();
    let has = |wasm: &[u8]| wasm.windows(8).any(|w| w == b"fmt_real");
    assert!(build("Inc(i, 3)").len() <= build("i := i + 3").len() + 8);
    assert!(build("Dec(c, 2)").len() <= build("c := Chr(Ord(c) - 2)").len() + 8);
    assert!(!has(&build("Str(i, s)")));
    assert!(!has(&build("Val(s, r, e)")));
    assert!(has(&build("Str(r:0:2, s)")));
    assert!(!has(&build("Inc(i)")));
}
