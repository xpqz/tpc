//! The case statement: labels, lists, ranges, else, and selector typing.
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
    "case_boolean_selector",
    "case_else_optional_semicolon",
    "case_longint_selector_is_integer_sized",
    "case_named_constants_and_expression_selector",
    "case_negative_labels",
    "case_nested_and_compound",
    "case_no_match_no_else",
    "case_ranges_grades",
    "case_word_selector",
];

/// tests/neg programs this slice must pass.
const NEG: &[&str] = &[
    "case_label_out_of_range_integer",
    "case_label_type_mismatch",
    "missing_of_in_case",
];

const RUN: &[(&str, &str, &str)] = &[
    ("sparse_wide_labels", "program D;\nvar i: Integer;\nbegin\n  for i := 0 to 2 do\n  begin\n    case i * 15000 of\n      1: WriteLn('one');\n      30000: WriteLn('thirty thousand');\n      0: WriteLn('zero')\n    else\n      WriteLn('else ', i * 15000)\n    end\n  end;\n  i := -32768;\n  case i of -32768..32767: WriteLn('whole range') end;\n  i := 32767;\n  case i of -32768..-1: WriteLn('neg'); 32767: WriteLn('max') end\nend.", "zero\nelse 15000\nthirty thousand\nwhole range\nmax\n"),
    ("duplicate_and_overlapping", "program D;\nvar i: Integer;\nbegin\n  for i := 1 to 6 do\n  begin\n    case i of\n      1..5: WriteLn('range ', i);\n      3: WriteLn('three');\n      3: WriteLn('three again');\n      6, 6: WriteLn('six')\n    end\n  end\nend.", "range 1\nrange 2\nrange 3\nrange 4\nrange 5\nsix\n"),
    ("else_statement_list", "program D;\nvar i: Integer;\nbegin\n  i := 9;\n  case i of\n    1: WriteLn('one')\n  else\n    WriteLn('a');\n    WriteLn('b');\n    WriteLn('c')\n  end\nend.", "a\nb\nc\n"),
    ("trailing_semicolons", "program D;\nvar i: Integer;\nbegin\n  for i := 1 to 3 do\n    case i of\n      1: WriteLn('one');\n      2: WriteLn('two');\n    end\nend.", "one\ntwo\n"),
    ("selector_char", "program D;\nvar c: Char;\nbegin\n  for c := 'a' to 'g' do\n    case c of\n      'a', 'e': Write('v');\n      'b'..'d': Write('c');\n      #102: Write('F')\n    else\n      Write('?')\n    end;\n  WriteLn\nend.", "vcccvF?\n"),
    ("selector_boolean", "program D;\nvar b: Boolean;\nbegin\n  for b := False to True do\n    case b of\n      False: WriteLn('no');\n      True: WriteLn('yes')\n    end\nend.", "no\nyes\n"),
    ("selector_byte_and_shortint", "program D;\nvar b: Byte; s: ShortInt;\nbegin\n  b := 200; s := -5;\n  case b of\n    0..127: WriteLn('low');\n    128..255: WriteLn('high')\n  end;\n  case s of\n    -128..-1: WriteLn('negative');\n    0..127: WriteLn('non-negative')\n  end;\n  s := -128;\n  case s of -128: WriteLn('min') else WriteLn('other') end;\n  b := 255;\n  case b of 255: WriteLn('max') else WriteLn('other') end\nend.", "high\nnegative\nmin\nmax\n"),
    ("selector_word_truncated", "program D;\nvar w: Word;\nbegin\n  w := 65535;\n  case w of\n    -1: WriteLn('minus one');\n    32767: WriteLn('max')\n  else\n    WriteLn('other')\n  end;\n  w := 40000;\n  case w of\n    -25536: WriteLn('wrapped')\n  else\n    WriteLn('other')\n  end\nend.", "minus one\nwrapped\n"),
    ("selector_longint_truncated", "program D;\nvar l: LongInt;\nbegin\n  l := 65541;\n  case l of\n    5: WriteLn('five');\n  else\n    WriteLn('other')\n  end;\n  l := -65536 - 1;\n  case l of\n    -1: WriteLn('minus one')\n  else\n    WriteLn('other')\n  end;\n  l := 2147483647;\n  case l of\n    -1: WriteLn('minus one')\n  else\n    WriteLn('other')\n  end\nend.", "five\nminus one\nminus one\n"),
    ("shortint_label_unchecked", "program D;\nvar s: ShortInt;\nbegin\n  s := 100;\n  case s of\n    200: WriteLn('never');\n    100: WriteLn('hundred')\n  end\nend.", "hundred\n"),
    ("selector_constant_and_expression", "program D;\nconst K = 3;\nvar i: Integer;\nbegin\n  case K of 3: WriteLn('k3') else WriteLn('no') end;\n  case 7 of 1..6: WriteLn('low') else WriteLn('high') end;\n  i := 4;\n  case i + 1 of\n    5: WriteLn('five');\n    K * 2: WriteLn('six')\n  end;\n  case -i of -4: WriteLn('minus four') end\nend.", "k3\nhigh\nfive\nminus four\n"),
    ("constant_labels_expressions", "program D;\nconst Lo = 10; Hi = Lo * 2;\nvar i: Integer;\nbegin\n  for i := 9 to 21 do\n    case i of\n      Lo - 1: Write('A');\n      Lo..Lo + 2: Write('B');\n      Hi - 1, Hi: Write('C');\n      -1 + 15: Write('D')\n    else\n      Write('.')\n    end;\n  WriteLn\nend.", "ABBB.D....CC.\n"),
    ("nested_and_in_loops", "program D;\nvar a, b, n: Integer;\nbegin\n  n := 0;\n  for a := 1 to 3 do\n    for b := 1 to 3 do\n      case a of\n        1: case b of 1: n := n + 1; 2..3: n := n + 10 end;\n        2: if b = 2 then n := n + 100 else n := n + 1000;\n        3: begin n := n + 10000; case b of 3: n := n + 100000 end end\n      end;\n  WriteLn(n)\nend.", "1049\n"),
    ("case_in_while_repeat", "program D;\nvar i: Integer;\nbegin\n  i := 0;\n  while i < 6 do\n  begin\n    case i mod 3 of\n      0: Write('z');\n      1: Write('o')\n    else Write('t')\n    end;\n    i := i + 1\n  end;\n  WriteLn;\n  repeat\n    i := i - 1;\n    case i of 5: Write('five '); 0..2: Write('low ') end\n  until i = 0;\n  WriteLn\nend.", "zotzot\nfive low low low \n"),
    ("negative_labels_dense", "program D;\nvar i: Integer;\nbegin\n  for i := -3 to 3 do\n    case i of\n      -3: Write('a');\n      -2: Write('b');\n      -1: Write('c');\n      0: Write('d');\n      1: Write('e');\n      2: Write('f');\n      3: Write('g')\n    end;\n  WriteLn\nend.", "abcdefg\n"),
    ("empty_branches_and_reversed_range", "program D;\nvar i: Integer;\nbegin\n  i := 4;\n  case i of 4: ; 5: WriteLn('five') end;\n  case i of 5..1: WriteLn('never') else WriteLn('else') end;\n  case i of 3..5: ; end;\n  WriteLn('done')\nend.", "else\ndone\n"),
];

/// (name, source, TP error number, line)
const ERR: &[(&str, &str, u16, u32)] = &[
    ("label_out_of_range_neg", "program E;\nvar i: Integer;\nbegin\n  case i of\n    -32769: WriteLn\n  end\nend.", 112, 5),
    ("label_range_end_out_of_range", "program E;\nvar i: Integer;\nbegin\n  case i of\n    1..40000: WriteLn\n  end\nend.", 112, 5),
    ("label_out_of_range_word_selector", "program E;\nvar w: Word;\nbegin\n  case w of\n    40000: WriteLn\n  end\nend.", 112, 5),
    ("label_out_of_range_longint_selector", "program E;\nvar l: LongInt;\nbegin\n  case l of\n    70000: WriteLn\n  end\nend.", 112, 5),
    ("label_boolean_for_integer", "program E;\nvar i: Integer;\nbegin\n  case i of\n    True: WriteLn\n  end\nend.", 74, 5),
    ("label_integer_for_char", "program E;\nvar c: Char;\nbegin\n  case c of\n    1: WriteLn\n  end\nend.", 74, 5),
    ("label_char_for_boolean", "program E;\nvar b: Boolean;\nbegin\n  case b of\n    'a': WriteLn\n  end\nend.", 74, 5),
    ("label_range_mixed_types", "program E;\nvar i: Integer;\nbegin\n  case i of\n    1..'a': WriteLn\n  end\nend.", 74, 5),
    ("label_unknown_identifier", "program E;\nvar i: Integer;\nbegin\n  case i of\n    Zork: WriteLn\n  end\nend.", 3, 5),
    ("label_is_variable", "program E;\nvar i, j: Integer;\nbegin\n  case i of\n    j: WriteLn\n  end\nend.", 133, 5),
    ("case_missing_of", "program E;\nvar i: Integer;\nbegin\n  case i 1: WriteLn\n  end\nend.", 54, 4),
    ("case_missing_colon", "program E;\nvar i: Integer;\nbegin\n  case i of\n    1 WriteLn\n  end\nend.", 86, 5),
    ("case_missing_end", "program E;\nvar i: Integer;\nbegin\n  case i of\n    1: WriteLn\nend.", 85, 6),
    ("case_missing_selector", "program E;\nbegin\n  case of\n    1: WriteLn\n  end\nend.", 42, 3),
    ("case_missing_label", "program E;\nvar i: Integer;\nbegin\n  case i of\n    : WriteLn\n  end\nend.", 42, 5),
    ("case_selector_boolean_condition_mismatch", "program E;\nvar i: Integer;\nbegin\n  case i = 1 of\n    1: WriteLn\n  end\nend.", 74, 5),
    ("case_label_list_trailing_comma", "program E;\nvar i: Integer;\nbegin\n  case i of\n    1,: WriteLn\n  end\nend.", 42, 5),
    ("case_else_twice", "program E;\nvar i: Integer;\nbegin\n  case i of\n    1: WriteLn\n  else WriteLn\n  else WriteLn\n  end\nend.", 85, 7),
    ("case_else_before_label", "program E;\nvar i: Integer;\nbegin\n  case i of\n    else WriteLn;\n    1: WriteLn\n  end\nend.", 42, 5),
    ("case_without_labels_semicolon_only", "program E;\nvar i: Integer;\nbegin\n  case i of ; end\nend.", 42, 4),
    ("case_label_division_by_zero", "program E;\nvar i: Integer;\nbegin\n  case i of\n    5 div 0: WriteLn\n  end\nend.", 62, 5),
    ("case_empty_list", "program E;\nvar i: Integer;\nbegin\n  case i of end\nend.", 42, 4),
    ("case_only_else", "program E;\nvar i: Integer;\nbegin\n  case i of else WriteLn end\nend.", 42, 4),
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
