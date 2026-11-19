//! Integer, Boolean and Char variables, constants, arithmetic and Write with field widths.
//! Expected output and `(code, line)` pairs come from the real TP 5.0 (`oracle_agrees`
//! re-derives them with `tools/tp5.sh`). The target corpus programs are copied without
//! PENDING into a scratch directory and checked with the real compiler.
mod common;
mod driver;
use driver::*;
use std::path::PathBuf;
use std::process::Command;

/// tests/lang programs this slice must pass.
const LANG: &[&str] = &[
    "arithmetic_precedence",
    "assign_between_types_truncates",
    "bitwise_operators_integer",
    "bitwise_operators_word_byte",
    "boolean_precedence",
    "byte_div_mod",
    "byte_wraps_on_assignment",
    "char_literals_and_codes",
    "comments_all_styles",
    "comparisons_char_boolean",
    "comparisons_longint_word",
    "const_expressions",
    "const_scalars",
    "const_string",
    "div_mod_longint",
    "hex_literals",
    "int_typing_byte_negation_is_integer",
    "int_typing_byte_plus_byte",
    "int_typing_byte_plus_word_is_word",
    "int_typing_byte_times_byte",
    "int_typing_comparison_widens_bytes",
    "int_typing_integer_negation_of_minimum",
    "int_typing_integer_overflow_before_div",
    "int_typing_integer_plus_longint",
    "int_typing_integer_product_wraps_in_expression",
    "int_typing_integer_word_common_longint",
    "int_typing_longint_plus_wraps_at_32_bits",
    "int_typing_mixed_expression_chain",
    "int_typing_not_keeps_operand_type",
    "int_typing_shl_keeps_operand_type",
    "int_typing_shortint_plus_shortint",
    "int_typing_shortint_with_word_is_longint",
    "int_typing_shr_is_logical_within_width",
    "int_typing_word_negation",
    "int_typing_word_plus_word_wraps",
    "integer_wraps_on_assignment",
    "longint_wraps_on_assignment",
    "shifts_small_positive",
    "shifts_word_longint",
    "shortint_wraps_on_assignment",
    "unary_minus_and_plus",
    "word_div_mod_unsigned",
    "word_wraps_on_assignment",
    "write_boolean",
    "write_char",
    "write_field_widths",
    "write_integer_extremes",
    "write_multiple_arguments",
    "write_negative_with_width",
    "write_width_narrower_than_value",
    "write_width_on_strings_chars_booleans",
];

/// tests/neg programs this slice must pass.
const NEG: &[&str] = &[
    "assign_expected_got_equals",
    "assign_to_constant",
    "call_integer_variable_as_procedure",
    "constant_out_of_range_byte_negative",
    "constant_out_of_range_byte",
    "constant_out_of_range_integer",
    "constant_out_of_range_shortint",
    "constant_out_of_range_word",
    "division_by_constant_zero",
    "duplicate_const_and_variable",
    "duplicate_variable",
    "expression_expected",
    "missing_colon_in_var",
    "missing_equals_in_const",
    "missing_semicolon_after_declaration",
    "operands_do_not_match_operator_boolean_plus",
    "type_mismatch_and_with_integer_and_boolean",
    "type_mismatch_boolean_to_integer",
    "type_mismatch_char_to_integer",
    "type_mismatch_integer_to_boolean",
    "type_mismatch_integer_to_char",
    "undeclared_identifier_in_expression",
    "undeclared_procedure_call",
    "undeclared_variable_assigned",
    "unknown_type_name",
    "unsupported_single_type",
];

const RUN: &[(&str, &str, &str)] = &[
    ("multi_decl", "program Md;\nvar a, b, c: Integer; x: Word; y, z: Byte;\nbegin\n  a := 1; b := 2; c := a + b; x := c; y := 200; z := y;\n  WriteLn(a, ' ', b, ' ', c, ' ', x, ' ', y, ' ', z)\nend.", "1 2 3 3 200 200\n"),
    ("const_chain", "program Kc;\nconst A = 3; B = A * A; C = B - A * 4; D = -C; E = A + 65535;\n  F = 2147483647; G = -2147483647 - 1;\nbegin\n  WriteLn(A, ' ', B, ' ', C, ' ', D, ' ', E);\n  WriteLn(F, ' ', G)\nend.", "3 9 -3 3 65538\n2147483647 -2147483648\n"),
    ("const_fold_ops", "program Kf;\nconst A = 7 div 2; B = 7 mod 3; C = -7 div 2; D = -7 mod 3; E = 1 shl 4;\n  F = 256 shr 2; G = 12 and 10; H = 12 or 10; I = 12 xor 10; J = not 0;\nbegin\n  WriteLn(A, ' ', B, ' ', C, ' ', D, ' ', E);\n  WriteLn(F, ' ', G, ' ', H, ' ', I, ' ', J)\nend.", "3 1 -3 -1 16\n64 8 14 6 -1\n"),
    ("const_bool", "program Kb;\nconst T = True; F = not T; X = 3 < 4; Y = 'a' < 'b'; Z = (1 = 2) or T;\nbegin\n  WriteLn(T, ' ', F, ' ', X, ' ', Y, ' ', Z)\nend.", "TRUE FALSE TRUE TRUE TRUE\n"),
    ("const_char", "program Kch;\nconst Cc = 'A'; Dd = ^G;\nvar c: Char;\nbegin\n  c := Cc;\n  WriteLn(c, Dd, Dd:3)\nend.", "A\x07  \x07\n"),
    ("char_var", "program Cv;\nvar c, d: Char;\nbegin\n  c := 'x'; d := ^I;\n  WriteLn(c, d, c);\n  c := #65;\n  Write(c, c:3);\n  WriteLn\nend.", "x\tx\nA  A\n"),
    ("bool_ops", "program Bo;\nvar a, b: Boolean;\nbegin\n  a := True; b := False;\n  WriteLn(a and b, ' ', a or b, ' ', a xor b, ' ', not a);\n  WriteLn(a = b, ' ', a <> b, ' ', a > b, ' ', a < b)\nend.", "FALSE TRUE TRUE FALSE\nFALSE TRUE TRUE FALSE\n"),
    ("int_compare_all", "program Ic;\nvar i, j: Integer;\nbegin\n  i := 3; j := 4;\n  WriteLn(i < j, ' ', i > j, ' ', i <= j, ' ', i >= j, ' ', i = j, ' ', i <> j)\nend.", "TRUE FALSE TRUE FALSE FALSE TRUE\n"),
    ("range_limits", "program Rl;\nvar b: Byte; s: ShortInt; i: Integer; w: Word; l: LongInt;\nbegin\n  b := 255; s := -128; i := -32768; w := 65535; l := -2147483647 - 1;\n  WriteLn(b, ' ', s, ' ', i, ' ', w);\n  WriteLn(l)\nend.", "255 -128 -32768 65535\n-2147483648\n"),
    ("store_truncation", "program St;\nvar b: Byte; s: ShortInt; i: Integer; w: Word; l, x: LongInt;\nbegin\n  x := 300; b := x; WriteLn(b);\n  x := 200; s := x; WriteLn(s);\n  l := 70000; i := l; WriteLn(i);\n  w := l; WriteLn(w);\n  l := 2147483647; l := l + 1; WriteLn(l)\nend.", "44\n-56\n4464\n4464\n-2147483648\n"),
    ("widths_mixed", "program Wm;\nvar b: Boolean; c: Char;\nbegin\n  b := True; c := 'z';\n  WriteLn(b:6, '|', c:4, '|', False:3, '|', 'ab':5, '|', -12:6, '|', 5:1)\nend.", "  TRUE|   z|FALSE|   ab|   -12|5\n"),
    ("precedence_mixed", "program Pm;\nvar a, b: Integer;\nbegin\n  a := 2; b := 3;\n  WriteLn(a + b * 4, ' ', (a + b) * 4, ' ', -a * b, ' ', a - b - 1);\n  WriteLn(17 div a * b, ' ', 17 mod 5 + 1, ' ', a shl 1 + 1);\n  WriteLn(not a and 7, ' ', a or b and 1)\nend.", "14 20 -6 -2\n24 3 5\n5 3\n"),
    ("identifiers_case_insensitive", "program Ci;\nVAR Abc: integer; ABC2: BYTE;\nbegin\n  abc := 5; abc2 := ABC + 1; WRITELN(Abc, ' ', AbC2)\nend.", "5 6\n"),
    ("char_compare", "program Cc2;\nvar a, b: Char;\nbegin\n  a := 'a'; b := 'b';\n  WriteLn(a < b, ' ', a = b, ' ', a >= b, ' ', a <> b)\nend.", "TRUE FALSE FALSE TRUE\n"),
    ("negative_div_mod", "program Nd;\nvar a, b: Integer;\nbegin\n  a := -7; b := 2;\n  WriteLn(a div b, ' ', a mod b, ' ', -a div b)\nend.", "-3 -1 3\n"),
    ("self_reference", "program Sr;\nvar a: Integer;\nbegin\n  a := 1; a := a + a; a := a * a; a := a - 10; WriteLn(a)\nend.", "-6\n"),
    ("char_const_in_expr", "program Ce;\nconst Star = '*';\nvar n: Integer;\nbegin\n  n := 3;\n  Write(Star, Star:n, Star:1);\n  WriteLn\nend.", "*  **\n"),
    ("longint_widths", "program Lw;\nvar l: LongInt; w: Word;\nbegin\n  l := -2147483647 - 1; w := 65535;\n  WriteLn(l:12, w:7, (l + 1):12)\nend.", " -2147483648  65535 -2147483647\n"),
];

/// (name, source, TP error number, line)
const ERR: &[(&str, &str, u16, u32)] = &[
    (
        "assign_char_to_boolean",
        "program E;\nvar b: Boolean;\nbegin\n  b := 'a'\nend.",
        26,
        5,
    ),
    (
        "assign_boolean_to_char",
        "program E;\nvar c: Char;\nbegin\n  c := True\nend.",
        26,
        5,
    ),
    (
        "unknown_type_in_second_var",
        "program E;\nvar a: Integer; b: Frob;\nbegin\nend.",
        3,
        2,
    ),
    (
        "duplicate_const",
        "program E;\nconst A = 1;\n  A = 2;\nbegin\nend.",
        4,
        3,
    ),
    (
        "duplicate_var_same_line",
        "program E;\nvar a, b, a: Integer;\nbegin\nend.",
        4,
        2,
    ),
    (
        "var_named_like_program",
        "program E;\nvar e: Integer;\nbegin\nend.",
        4,
        2,
    ),
    (
        "const_overflow_assign",
        "program E;\nconst A = 40000;\nvar i: Integer;\nbegin\n  i := A\nend.",
        76,
        6,
    ),
    (
        "const_division_zero_mod",
        "program E;\nconst A = 5 mod 0;\nbegin\nend.",
        62,
        2,
    ),
    (
        "not_on_char",
        "program E;\nvar c: Char;\nbegin\n  c := not c\nend.",
        41,
        5,
    ),
    (
        "plus_on_chars",
        "program E;\nvar c: Char;\nbegin\n  c := c + c\nend.",
        26,
        5,
    ),
    (
        "and_on_chars",
        "program E;\nvar c: Char; b: Boolean;\nbegin\n  b := c and c\nend.",
        41,
        5,
    ),
    (
        "compare_boolean_integer",
        "program E;\nvar b: Boolean; i: Integer;\nbegin\n  b := i = b\nend.",
        26,
        5,
    ),
    (
        "var_missing_name",
        "program E;\nvar : Integer;\nbegin\nend.",
        2,
        2,
    ),
    (
        "const_missing_value",
        "program E;\nconst A = ;\nbegin\nend.",
        42,
        2,
    ),
    (
        "assign_unknown_const",
        "program E;\nconst A = Frob;\nbegin\nend.",
        3,
        2,
    ),
    (
        "write_width_not_integer",
        "program E;\nbegin\n  WriteLn(1:'a')\nend.",
        38,
        3,
    ),
    (
        "write_width_boolean",
        "program E;\nbegin\n  WriteLn(1:True)\nend.",
        38,
        3,
    ),
    (
        "assign_missing_rhs_before_end",
        "program E;\nvar x: Integer;\nbegin\n  x :=\nend.",
        42,
        5,
    ),
    (
        "expression_unclosed_paren",
        "program E;\nvar x: Integer;\nbegin\n  x := (1 + 2\nend.",
        89,
        5,
    ),
    (
        "shl_with_boolean",
        "program E;\nvar b: Boolean; i: Integer;\nbegin\n  i := 1 shl b\nend.",
        26,
        5,
    ),
    (
        "assign_string_to_integer",
        "program E;\nvar i: Integer;\nbegin\n  i := 'ab'\nend.",
        26,
        5,
    ),
];

fn tmp_dir() -> PathBuf {
    crate::common::fresh_dir("integers")
}

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
fn target_lang_programs_match_tp5_output() {
    let fails = check_lang(&subset("lang", LANG, &["pas", "out"]), &real_compile);
    assert!(
        fails.is_empty(),
        "{} of {}:\n{}",
        fails.len(),
        LANG.len(),
        fails.join("\n")
    );
}

#[test]
fn target_neg_programs_report_tp5_number_and_line() {
    let fails = check_neg(&subset("neg", NEG, &["pas", "err"]), &real_compile);
    assert!(
        fails.is_empty(),
        "{} of {}:\n{}",
        fails.len(),
        NEG.len(),
        fails.join("\n")
    );
}

fn run_wasm(name: &str, wasm: &[u8]) -> Result<Vec<u8>, String> {
    let path = tmp_dir().join([name, ".wasm"].concat());
    std::fs::write(&path, wasm).unwrap();
    let out = Command::new("node")
        .arg(root().join("tests/run.mjs"))
        .arg(&path)
        .output()
        .expect("node must be installed");
    if out.status.success() {
        Ok(out.stdout)
    } else {
        Err(String::from_utf8_lossy(&out.stderr).into_owned())
    }
}

#[test]
fn programs_print_what_tp5_prints() {
    let mut fails = Vec::new();
    for &(name, src, want) in RUN {
        let src = [src, "\n"].concat();
        match tp5c::compile(src.as_bytes()) {
            Err(d) => fails.push(format!(
                "{name}: rejected with {} at {}:{}",
                d.code, d.line, d.col
            )),
            Ok(wasm) => match run_wasm(name, &wasm) {
                Err(e) => fails.push(format!("{name}: run failed: {e}")),
                Ok(got) if got != want.as_bytes() => fails.push(format!(
                    "{name}: want {want:?}, got {:?}",
                    String::from_utf8_lossy(&got)
                )),
                Ok(_) => {}
            },
        }
    }
    assert!(fails.is_empty(), "{}", fails.join("\n"));
}

#[test]
fn invalid_programs_report_tp5_number_and_line() {
    let mut fails = Vec::new();
    for &(name, src, code, line) in ERR {
        let src = [src, "\n"].concat();
        match tp5c::compile(src.as_bytes()) {
            Ok(_) => fails.push(format!(
                "{name}: accepted, want error {code} at line {line}"
            )),
            Err(d) if (d.code, d.line) != (code, line) => fails.push(format!(
                "{name}: want error {code} at line {line}, got {} at line {}",
                d.code, d.line
            )),
            Err(_) => {}
        }
    }
    assert!(fails.is_empty(), "{}", fails.join("\n"));
}

/// (name, source): a run-time division by zero, after `before` was printed.
const ZERO_DIV: &[(&str, &str)] = &[
    ("div_integer", "var a, z: Integer;\nbegin\n  WriteLn('before'); a := 7; z := 0; WriteLn(a div z); WriteLn('after')\nend."),
    ("mod_integer", "var a, z: Integer;\nbegin\n  WriteLn('before'); a := 7; z := 0; WriteLn(a mod z); WriteLn('after')\nend."),
    ("div_longint", "var a, z: LongInt;\nbegin\n  WriteLn('before'); a := 7; z := 0; WriteLn(a div z); WriteLn('after')\nend."),
    ("mod_longint", "var a, z: LongInt;\nbegin\n  WriteLn('before'); a := 7; z := 0; WriteLn(a mod z); WriteLn('after')\nend."),
    ("div_word", "var a, z: Word;\nbegin\n  WriteLn('before'); a := 7; z := 0; WriteLn(a div z); WriteLn('after')\nend."),
    ("div_by_expression", "var a: Integer;\nbegin\n  WriteLn('before'); a := 3; WriteLn(10 div (a - 3)); WriteLn('after')\nend."),
];

#[test]
fn division_by_zero_at_run_time_is_runtime_error_200() {
    let mut fails = Vec::new();
    for &(name, src) in ZERO_DIV {
        let src = [src, "\n"].concat();
        let wasm = match tp5c::compile(src.as_bytes()) {
            Ok(w) => w,
            Err(d) => {
                fails.push(format!(
                    "{name}: rejected with {} at {}:{}",
                    d.code, d.line, d.col
                ));
                continue;
            }
        };
        let path = tmp_dir().join([name, ".wasm"].concat());
        std::fs::write(&path, wasm).unwrap();
        let out = Command::new("node")
            .arg(root().join("tests/run.mjs"))
            .arg(&path)
            .output()
            .unwrap();
        let text = String::from_utf8_lossy(&out.stdout).into_owned();
        if text != "before\nRuntime error 200\n" || out.status.code() != Some(200) {
            fails.push(format!(
                "{name}: stdout {text:?}, exit {:?}",
                out.status.code()
            ));
        }
    }
    assert!(fails.is_empty(), "{}", fails.join("\n"));
}

#[test]
fn every_run_case_prints_something() {
    assert!(RUN.iter().all(|r| !r.2.is_empty()));
}

/// Slow (DOSBox-X): `cargo test -- --ignored`.
#[test]
#[ignore]
fn oracle_agrees() {
    let dir = tmp_dir();
    let tp5 = |name: &str, src: &str| {
        let p = dir.join([name, ".pas"].concat());
        std::fs::write(&p, [src, "\n"].concat()).unwrap();
        let o = Command::new(root().join("tools/tp5.sh"))
            .arg(&p)
            .output()
            .unwrap();
        String::from_utf8_lossy(&o.stdout).into_owned()
    };
    let mut fails = Vec::new();
    for &(name, src, want) in RUN {
        let got = tp5(name, src);
        if got != want {
            fails.push(format!("{name}: tp5 printed {got:?}, table has {want:?}"));
        }
    }
    for &(name, src, code, line) in ERR {
        let got = tp5(name, src);
        let want = format!("P.PAS({line}): Error {code}:");
        if !got.starts_with(&want) {
            fails.push(format!("{name}: tp5 said {got:?}, table has {want}"));
        }
    }
    assert!(fails.is_empty(), "{}", fails.join("\n"));
}
