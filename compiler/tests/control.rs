//! if, while, repeat and for, short-circuit Boolean operators, and the division edge cases.
//! Expected output and `(code, line)` pairs come from the real TP 5.0 (`oracle_agrees`
//! re-derives them with `tools/tp5.sh`). The target corpus programs are copied without
//! PENDING into a scratch directory and checked with the real compiler.
mod common;
use common::OutputGuarded as _;
mod driver;
use driver::*;
use std::path::PathBuf;
use std::process::Command;

/// tests/lang programs this slice must pass.
const LANG: &[&str] = &[
    "fibonacci_iterative",
    "for_accumulate",
    "for_bounds_evaluated_once",
    "for_char_and_boolean_control",
    "for_downto_basic",
    "for_downto_boundaries",
    "for_empty_range",
    "for_equal_bounds",
    "for_nested",
    "for_to_basic",
    "for_type_limits_downto",
    "for_type_limits_to",
    "gcd_and_lcm_loops",
    "if_boolean_variable",
    "multiplication_table",
    "overflow_wrap_16bit",
    "perfect_numbers",
    "primes_trial_division",
    "repeat_basic",
    "repeat_nested_and_multiple_statements",
    "repeat_runs_once",
    "while_basic",
    "while_nested",
    "while_zero_iterations",
];

/// tests/neg programs this slice must pass.
const NEG: &[&str] = &[
    "boolean_expression_expected_if",
    "boolean_expression_expected_until",
    "boolean_expression_expected_while",
    "missing_do_in_for",
    "missing_do_in_while",
    "missing_then",
    "missing_to_in_for",
    "type_mismatch_compare_integer_char",
];

const RUN: &[(&str, &str, &str)] = &[
    ("if_else_dangling", "program D;\nvar x, y: Boolean;\nbegin\n  x := True; y := False;\n  if x then if y then WriteLn('both') else WriteLn('x only');\n  x := False;\n  if x then if y then WriteLn('both') else WriteLn('never');\n  WriteLn('end')\nend.", "x only\nend\n"),
    ("if_no_else_false", "program D;\nvar i: Integer;\nbegin\n  i := 5;\n  if i > 10 then WriteLn('big');\n  if i < 10 then WriteLn('small');\n  if i = 5 then begin WriteLn('five'); WriteLn('again') end else WriteLn('not five')\nend.", "small\nfive\nagain\n"),
    ("if_else_chain", "program D;\nvar i, n: Integer;\nbegin\n  for n := 1 to 4 do\n  begin\n    i := n * 10;\n    if i < 15 then WriteLn('a')\n    else if i < 25 then WriteLn('b')\n    else if i < 35 then WriteLn('c')\n    else WriteLn('d')\n  end\nend.", "a\nb\nc\nd\n"),
    ("if_empty_branches", "program D;\nvar i: Integer;\nbegin\n  i := 1;\n  if i = 1 then else WriteLn('else');\n  if i = 2 then WriteLn('x') else;\n  WriteLn('done')\nend.", "done\n"),
    ("short_circuit_and", "program D;\nvar i: Integer;\nbegin\n  i := 0;\n  if (i <> 0) and (10 div i > 1) then WriteLn('yes') else WriteLn('no');\n  i := 5;\n  if (i <> 0) and (10 div i > 1) then WriteLn('yes') else WriteLn('no')\nend.", "no\nyes\n"),
    ("short_circuit_or", "program D;\nvar i: Integer;\nbegin\n  i := 0;\n  if (i = 0) or (10 div i > 1) then WriteLn('yes') else WriteLn('no');\n  while (i = 0) or (10 div i > 100) do i := i + 1;\n  WriteLn(i)\nend.", "yes\n1\n"),
    ("short_circuit_nested", "program D;\nvar i: Integer; b: Boolean;\nbegin\n  i := 0;\n  b := (i <> 0) and (10 div i > 1) or (i = 0);\n  WriteLn(b);\n  b := not ((i = 0) or (10 div i > 1));\n  WriteLn(b);\n  b := (i <> 0) and ((10 div i > 1) or (10 div i < 0));\n  WriteLn(b)\nend.", "TRUE\nFALSE\nFALSE\n"),
    ("while_countdown", "program D;\nvar i: Integer;\nbegin\n  i := 3;\n  while i > 0 do begin Write(i, ' '); i := i - 1 end;\n  WriteLn;\n  while i > 0 do WriteLn('never');\n  while False do WriteLn('never');\n  WriteLn('done')\nend.", "3 2 1 \ndone\n"),
    ("while_boolean_flag", "program D;\nvar go: Boolean; n: Word;\nbegin\n  go := True; n := 65530;\n  while go do begin n := n + 1; go := n <> 4 end;\n  WriteLn(n)\nend.", "4\n"),
    ("repeat_forms", "program D;\nvar i: Integer;\nbegin\n  i := 0;\n  repeat i := i + 1; Write(i, ' ') until i >= 4;\n  WriteLn;\n  repeat WriteLn('once') until True;\n  repeat until True;\n  repeat ; ; until i > 0\nend.", "1 2 3 4 \nonce\n"),
    ("repeat_compound_body", "program D;\nvar i, s: Integer;\nbegin\n  i := 1; s := 0;\n  repeat\n    begin s := s + i; i := i + 1 end\n  until (i > 10) or (s > 20);\n  WriteLn(i, ' ', s)\nend.", "7 21\n"),
    ("for_to_empty_and_single", "program D;\nvar i: Integer;\nbegin\n  for i := 5 to 4 do WriteLn('never');\n  for i := 3 to 3 do WriteLn('once ', i);\n  for i := -2 to 2 do Write(i, ' ');\n  WriteLn\nend.", "once 3\n-2 -1 0 1 2 \n"),
    ("for_downto_cases", "program D;\nvar i: Integer;\nbegin\n  for i := 3 downto 1 do Write(i, ' ');\n  WriteLn;\n  for i := 1 downto 3 do WriteLn('never');\n  for i := 2 downto 2 do WriteLn('once');\n  for i := 2 downto -2 do Write(i, ' ');\n  WriteLn\nend.", "3 2 1 \nonce\n2 1 0 -1 -2 \n"),
    ("for_bounds_once", "program D;\nvar i, n, m: Integer;\nbegin\n  n := 3; m := 1;\n  for i := m to n do begin n := 10; m := 0; Write(i, ' ') end;\n  WriteLn;\n  n := 1;\n  for i := 3 downto n do begin n := 3; Write(i, ' ') end;\n  WriteLn\nend.", "1 2 3 \n3 2 1 \n"),
    ("for_control_types", "program D;\nvar b: Byte; s: ShortInt; w: Word; l: LongInt; c: Char; t: Boolean;\nbegin\n  for b := 253 to 255 do Write(b, ' ');\n  WriteLn;\n  for b := 2 downto 0 do Write(b, ' ');\n  WriteLn;\n  for s := -128 to -126 do Write(s, ' ');\n  WriteLn;\n  for s := 127 downto 125 do Write(s, ' ');\n  WriteLn;\n  for w := 65534 to 65535 do Write(w, ' ');\n  WriteLn;\n  for l := -2147483647 - 1 to -2147483646 do Write(l, ' ');\n  WriteLn;\n  for l := 2147483647 downto 2147483646 do Write(l, ' ');\n  WriteLn;\n  for c := 'x' to 'z' do Write(c);\n  WriteLn;\n  for c := 'c' downto 'a' do Write(c);\n  WriteLn;\n  for t := False to True do Write(t, ' ');\n  WriteLn;\n  for t := True downto False do Write(t, ' ');\n  WriteLn\nend.", "253 254 255 \n2 1 0 \n-128 -127 -126 \n127 126 125 \n65534 65535 \n-2147483648 -2147483647 -2147483646 \n2147483647 2147483646 \nxyz\ncba\nFALSE TRUE \nTRUE FALSE \n"),
    ("for_char_limits", "program D;\nvar c: Char; n: Integer;\nbegin\n  n := 0;\n  for c := #253 to #255 do n := n + 1;\n  WriteLn(n);\n  for c := #2 downto #0 do n := n + 1;\n  WriteLn(n)\nend.", "3\n6\n"),
    ("for_nested_triangle", "program D;\nvar i, j: Integer;\nbegin\n  for i := 1 to 3 do\n  begin\n    for j := 1 to i do Write('*');\n    WriteLn\n  end\nend.", "*\n**\n***\n"),
    ("for_bounds_expressions", "program D;\nvar i, n: Integer; b: Byte;\nbegin\n  n := 2; b := 250;\n  for i := n * 2 - 1 to n + 5 do Write(i, ' ');\n  WriteLn;\n  for i := -n downto -n - 2 do Write(i, ' ');\n  WriteLn;\n  for b := b to b + 5 do Write(b, ' ');\n  WriteLn\nend.", "3 4 5 6 7 \n-2 -3 -4 \n250 251 252 253 254 255 \n"),
    ("for_long_range_and_word", "program D;\nvar i: Integer; w: Word; n: LongInt;\nbegin\n  n := 0;\n  for i := -32768 to 32767 do n := n + 1;\n  WriteLn(n);\n  n := 0;\n  for w := 1 to 3 do n := n + w;\n  WriteLn(n)\nend.", "65536\n6\n"),
    ("nested_mixed", "program D;\nvar i, j, n: Integer;\nbegin\n  n := 0; i := 0;\n  while i < 5 do\n  begin\n    i := i + 1;\n    j := 0;\n    repeat\n      j := j + 1;\n      if (i + j) mod 3 = 0 then n := n + 1\n    until j >= i\n  end;\n  WriteLn(n)\nend.", "5\n"),
    ("condition_comparisons", "program D;\nvar c: Char; b: Boolean; w: Word; l: LongInt;\nbegin\n  c := 'b'; b := True; w := 65535; l := 65536;\n  if c > 'a' then WriteLn('c');\n  if b then WriteLn('b');\n  if not b then WriteLn('nb');\n  if w < l then WriteLn('w<l');\n  if b = (c = 'b') then WriteLn('eq')\nend.", "c\nb\nw<l\neq\n"),
    ("longint_min_div_minus1", "program D;\nvar a, b: LongInt;\nbegin\n  a := -2147483647 - 1; b := -1;\n  WriteLn(a div b);\n  WriteLn(a mod b);\n  WriteLn(a div b div 1)\nend.", "-2147483648\n0\n-2147483648\n"),
    ("shortint_min_div_minus1", "program D;\nvar a, b: ShortInt;\nbegin\n  a := -128; b := -1;\n  WriteLn(a div b, ' ', a mod b)\nend.", "128 0\n"),
    ("integer_div_minus1_ok", "program D;\nvar a, b: Integer;\nbegin\n  a := -32767; b := -1;\n  WriteLn(a div b, ' ', a mod b);\n  a := 32767;\n  WriteLn(a div b, ' ', a mod b)\nend.", "32767 0\n-32767 0\n"),
    ("word_longint_mixed_div", "program D;\nvar w: Word; l: LongInt;\nbegin\n  w := 65535; l := -1;\n  WriteLn(w div l, ' ', w mod l);\n  l := -2147483647 - 1;\n  WriteLn(l div w, ' ', l mod w)\nend.", "-65535 0\n-32768 -32768\n"),
];

/// Programs that print `before`, then end with `Runtime error 200` (exit code 200).
const RT: &[(&str, &str)] = &[
    ("integer_min_div_minus1", "program D;\nvar a, b: Integer;\nbegin\n  WriteLn('before');\n  a := -32768; b := -1;\n  WriteLn(a div b);\n  WriteLn('after')\nend."),
    ("integer_min_mod_minus1", "program D;\nvar a, b: Integer;\nbegin\n  WriteLn('before');\n  a := -32768; b := -1;\n  WriteLn(a mod b);\n  WriteLn('after')\nend."),
    ("integer_min_div_minus1_nested", "program D;\nvar a, b: Integer;\nbegin\n  WriteLn('before');\n  a := -32768; b := -1;\n  a := (a - 0) div (b * 1);\n  WriteLn(a);\n  WriteLn('after')\nend."),
    ("variable_div_constant_zero", "program D;\nvar i: Integer;\nbegin\n  WriteLn('before');\n  i := 5;\n  WriteLn(i div 0);\n  WriteLn('after')\nend."),
    ("variable_mod_constant_zero", "program D;\nvar i: LongInt;\nbegin\n  WriteLn('before');\n  i := 5;\n  WriteLn(i mod 0);\n  WriteLn('after')\nend."),
];

/// (name, source, TP error number, line)
const ERR: &[(&str, &str, u16, u32)] = &[
    (
        "if_missing_then",
        "program E;\nvar i: Integer;\nbegin\n  if i = 1 WriteLn\nend.",
        57,
        4,
    ),
    (
        "if_missing_condition",
        "program E;\nbegin\n  if then WriteLn\nend.",
        42,
        3,
    ),
    (
        "if_integer_condition",
        "program E;\nvar i: Integer;\nbegin\n  if i then WriteLn\nend.",
        40,
        4,
    ),
    (
        "if_char_condition",
        "program E;\nvar c: Char;\nbegin\n  if c then WriteLn\nend.",
        40,
        4,
    ),
    (
        "if_semicolon_before_else",
        "program E;\nvar i: Integer;\nbegin\n  if i = 1 then WriteLn('a'); else WriteLn('b')\nend.",
        113,
        4,
    ),
    (
        "if_compare_mismatch",
        "program E;\nvar i: Integer; b: Boolean;\nbegin\n  if i = b then WriteLn\nend.",
        26,
        4,
    ),
    (
        "while_missing_do",
        "program E;\nvar i: Integer;\nbegin\n  while i < 3 i := i + 1\nend.",
        50,
        4,
    ),
    (
        "while_integer_condition",
        "program E;\nvar i: Integer;\nbegin\n  while i do i := i - 1\nend.",
        40,
        4,
    ),
    (
        "while_missing_condition",
        "program E;\nbegin\n  while do WriteLn\nend.",
        42,
        3,
    ),
    (
        "repeat_integer_until",
        "program E;\nvar i: Integer;\nbegin\n  repeat i := i + 1 until i\nend.",
        40,
        5,
    ),
    (
        "repeat_missing_until",
        "program E;\nvar i: Integer;\nbegin\n  repeat i := i + 1\nend.",
        85,
        5,
    ),
    (
        "repeat_end_instead_of_until",
        "program E;\nvar i: Integer;\nbegin\n  repeat i := i + 1; end\nend.",
        113,
        4,
    ),
    (
        "repeat_missing_condition",
        "program E;\nbegin\n  repeat WriteLn until\nend.",
        42,
        4,
    ),
    (
        "for_missing_to",
        "program E;\nvar i: Integer;\nbegin\n  for i := 1 3 do WriteLn(i)\nend.",
        58,
        4,
    ),
    (
        "for_missing_do",
        "program E;\nvar i: Integer;\nbegin\n  for i := 1 to 3 WriteLn(i)\nend.",
        50,
        4,
    ),
    (
        "for_missing_assign",
        "program E;\nvar i: Integer;\nbegin\n  for i = 1 to 3 do WriteLn(i)\nend.",
        91,
        4,
    ),
    (
        "for_missing_variable",
        "program E;\nbegin\n  for := 1 to 3 do WriteLn\nend.",
        20,
        3,
    ),
    (
        "for_unknown_variable",
        "program E;\nbegin\n  for k := 1 to 3 do WriteLn\nend.",
        3,
        3,
    ),
    (
        "for_constant_variable",
        "program E;\nconst K = 1;\nbegin\n  for K := 1 to 3 do WriteLn\nend.",
        20,
        4,
    ),
    (
        "for_boolean_bound_for_integer",
        "program E;\nvar i: Integer;\nbegin\n  for i := True to 3 do WriteLn\nend.",
        26,
        4,
    ),
    (
        "for_char_bound_for_integer",
        "program E;\nvar i: Integer;\nbegin\n  for i := 1 to 'a' do WriteLn\nend.",
        26,
        4,
    ),
    (
        "for_integer_bound_for_char",
        "program E;\nvar c: Char;\nbegin\n  for c := 'a' to 5 do WriteLn\nend.",
        26,
        4,
    ),
    (
        "for_missing_start",
        "program E;\nvar i: Integer;\nbegin\n  for i := to 3 do WriteLn\nend.",
        42,
        4,
    ),
    (
        "for_const_end_out_of_range",
        "program E;\nvar b: Byte;\nbegin\n  for b := 1 to 300 do WriteLn\nend.",
        76,
        4,
    ),
    (
        "for_const_start_out_of_range",
        "program E;\nvar b: Byte;\nbegin\n  for b := -1 to 3 do WriteLn\nend.",
        76,
        4,
    ),
    (
        "else_without_if",
        "program E;\nbegin\n  else WriteLn\nend.",
        113,
        3,
    ),
    (
        "then_without_if",
        "program E;\nbegin\n  then WriteLn\nend.",
        113,
        3,
    ),
    (
        "until_without_repeat",
        "program E;\nbegin\n  until True\nend.",
        113,
        3,
    ),
    (
        "do_without_loop",
        "program E;\nbegin\n  do WriteLn\nend.",
        113,
        3,
    ),
    (
        "if_unclosed_compound",
        "program E;\nvar i: Integer;\nbegin\n  if i = 1 then begin WriteLn\nend.",
        85,
        5,
    ),
    (
        "unknown_in_condition",
        "program E;\nbegin\n  if z then WriteLn\nend.",
        3,
        3,
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
        .output_guarded()
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

#[test]
fn runtime_errors_print_before_then_error_200() {
    let mut fails = Vec::new();
    for &(name, src) in RT {
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
            .output_guarded()
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
            .output_guarded()
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
    for &(name, src) in RT {
        let got = tp5(name, src);
        if !got.starts_with("before\nRuntime error 200 at") {
            fails.push(format!("{name}: tp5 printed {got:?}"));
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
