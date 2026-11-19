//! `{$R+}` / `{$R-}` range checking (decisions 20, 22, 24): runtime error 201 on assignment and
//! value arguments to subranges, enums and narrower integers, on array indexes and on string
//! indexes against the declared size. The switch applies to the code that follows it, statement
//! by statement (measured on TP 5.0). Every `.out` and `.err` (`N LINE`) under
//! tests/fixtures/range comes from the real TP 5.0 (`tools/tp5.sh`); runtime-error outputs are
//! normalised as in tests/rt (the ` at XXXX:YYYY.` suffix is dropped). `Inc`/`Dec` are never
//! checked (D17), which is why `Inc` of a subrange differs from TP (deviation to be recorded).
mod common;
mod driver;
use driver::*;
use std::path::{Path, PathBuf};

fn fx(sub: &str) -> PathBuf {
    root().join("compiler/tests/fixtures/range").join(sub)
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
    assert!(stems(&fx("lang"), ".pas").len() >= 6);
    assert_eq!(stems(&fx("lang"), ".pas"), stems(&fx("lang"), ".out"));
    for s in stems(&fx("lang"), ".out") {
        let out = std::fs::read(fx("lang").join(format!("{s}.out"))).unwrap();
        assert!(!out.is_empty(), "{s}: empty expected output");
    }
    assert_clean(check_lang(&fx("lang"), &real_compile));
}

#[test]
fn own_runtime_errors_are_201_after_the_expected_output() {
    assert!(stems(&fx("rt"), ".pas").len() >= 25);
    assert_eq!(stems(&fx("rt"), ".pas"), stems(&fx("rt"), ".out"));
    for s in stems(&fx("rt"), ".out") {
        let out = std::fs::read_to_string(fx("rt").join(format!("{s}.out"))).unwrap();
        assert!(
            out.trim_end().ends_with("Runtime error 201"),
            "{s}: {out:?}"
        );
    }
    assert_clean(check_rt(&fx("rt"), &real_compile));
}

#[test]
fn own_invalid_programs_report_tp5_number_and_line() {
    assert!(stems(&fx("neg"), ".pas").len() >= 5);
    assert_eq!(stems(&fx("neg"), ".pas"), stems(&fx("neg"), ".err"));
    assert_clean(check_neg(&fx("neg"), &real_compile));
}

/// Corpus programs this slice must make pass. `rangecheck_string_index_is_not_checked` is
/// listed under RT: its `.out` ends in `Runtime error 201` (decision 24), so it only passes
/// the runtime-error driver; it lives in tests/lang.
const LANG: &[&str] = &[
    "rangecheck_directive_switches_within_program",
    "rangecheck_in_range_values_pass",
    "rangecheck_string_index_within_declared_size",
    "rangecheck_value_argument_in_range",
];

const RT: &[(&str, &str)] = &[
    ("lang", "rangecheck_string_index_is_not_checked"),
    ("rt", "range_array_enum_index"),
    ("rt", "range_array_index_above"),
    ("rt", "range_array_index_below"),
    ("rt", "range_array_index_read_in_loop"),
    ("rt", "range_array_two_dimensional_index"),
    ("rt", "range_byte_assignment"),
    ("rt", "range_byte_negative"),
    ("rt", "range_char_subrange"),
    ("rt", "range_pointer_to_array_index"),
    ("rt", "range_shortint_assignment"),
    ("rt", "range_string_index_beyond_declared_size"),
    ("rt", "range_subrange_assignment"),
    ("rt", "range_subrange_enum"),
    ("rt", "range_value_argument"),
];

/// Copies the corpus programs into a fresh directory (no PENDING file). A `.out` that still
/// carries TP's ` at XXXX:YYYY.` suffix is normalised on the copy.
fn subset(items: &[(&str, &str)], exts: &[&str]) -> PathBuf {
    let dir = scratch("x").parent().unwrap().to_path_buf();
    for (kind, n) in items {
        for e in exts {
            let f = format!("{n}.{e}");
            let text = std::fs::read(root().join("tests").join(kind).join(&f)).unwrap();
            let cut = String::from_utf8_lossy(&text).find(" at 0000:");
            let keep = cut.unwrap_or(text.len());
            let tail = if cut.is_some() { &b"\n"[..] } else { b"" };
            std::fs::write(dir.join(&f), [&text[..keep], tail].concat()).unwrap();
        }
    }
    dir
}

#[test]
fn corpus_lang_programs_match_tp5_output() {
    let items: Vec<(&str, &str)> = LANG.iter().map(|n| ("lang", *n)).collect();
    assert_clean(check_lang(&subset(&items, &["pas", "out"]), &real_compile));
}

#[test]
fn corpus_runtime_errors_match_tp5() {
    assert_clean(check_rt(&subset(RT, &["pas", "out"]), &real_compile));
}

#[test]
fn corpus_lists_are_sorted_and_name_existing_programs() {
    let mut sorted = LANG.to_vec();
    sorted.sort();
    assert_eq!(sorted, LANG);
    for n in LANG {
        assert!(root().join(format!("tests/lang/{n}.pas")).exists(), "{n}");
    }
    for (kind, n) in RT {
        for e in ["pas", "out"] {
            let p = root().join(format!("tests/{kind}/{n}.{e}"));
            assert!(p.exists(), "{}", p.display());
        }
    }
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

const BODY: &str = "var b: Byte; i: Integer;\nbegin\n  i := 300;\n  b := i;\n  WriteLn(b)\nend.\n";

#[test]
fn range_directive_is_accepted_in_every_form() {
    for dir in [
        "{$R+}",
        "{$R-}",
        "{$r+}",
        "{$R+,F+}",
        "{$F-,R+}",
        "{$R-,R+,F+}",
    ] {
        compile(&format!("program P;\n{dir}\n{BODY}"));
    }
}

#[test]
fn other_directives_in_a_list_are_still_unsupported() {
    for dir in ["{$R+,I+}", "{$I-,R+}", "{$R+,F+,L+}"] {
        assert_eq!(
            rejected(&format!("program P;\n{dir}\n{BODY}")),
            (200, 2),
            "{dir}"
        );
    }
}

#[test]
fn range_checks_cost_nothing_while_off() {
    let off = compile(&format!("program P;\n{BODY}"));
    assert_eq!(off, compile(&format!("program P;\n{{$R-}}\n{BODY}")));
    assert_ne!(off, compile(&format!("program P;\n{{$R+}}\n{BODY}")));
    // Nothing to check: the module is the same with the switch on.
    let plain =
        "program P;\nvar i, j: Integer;\nbegin\n  i := 3;\n  j := i + 1;\n  WriteLn(j)\nend.\n";
    assert_eq!(
        compile(plain),
        compile(&plain.replace("program P;\n", "program P;\n{$R+}\n"))
    );
}

#[test]
fn the_switch_applies_from_where_it_stands() {
    let on_after = "program P;\nvar b: Byte; i: Integer;\nbegin\n  i := 300;\n  b := i;\n{$R+}\n  WriteLn(b)\nend.\n";
    let off_before = format!("program P;\n{{$R+}}\n{BODY}");
    // The assignment before the directive is unchecked, so this module is the unchecked one.
    assert_eq!(compile(on_after), compile(&on_after.replace("{$R+}\n", "")));
    assert_ne!(
        compile(&off_before),
        compile(&format!("program P;\n{BODY}"))
    );
}

#[test]
fn checks_that_can_never_fail_are_not_emitted() {
    let wide = |dir: &str| {
        format!(
            "program P;\n{dir}\nvar b: Byte; i: Integer; l: LongInt; a: array[Byte] of Integer;\n\
             begin\n  b := 7;\n  i := b;\n  l := i;\n  a[b] := i;\n  a[255] := 1;\n  WriteLn(l)\nend.\n"
        )
    };
    assert_eq!(compile(&wide("")), compile(&wide("{$R+}")));
}

#[test]
fn directive_errors_keep_their_position() {
    assert_eq!(rejected("program P;\nbegin\n{$R}\nend."), (17, 3));
    assert_eq!(rejected("program P;\nbegin\n  {$Z+}\nend."), (200, 3));
}
