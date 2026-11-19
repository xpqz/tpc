//! Walking skeleton: `Write`/`WriteLn` of string literals through lex, parse, check and
//! codegen. Every source gets a trailing newline. Expected output and `(code, line)` pairs
//! come from the real TP 5.0 (`oracle_agrees` re-derives them with `tools/tp5.sh`).
//! Notes from the oracle: the end-of-file error (10) is reported on the last line of the
//! file, a statement followed by anything but `;` or `end` is 85 (never 37), and a header
//! name that is not an identifier is 2.
mod common;
use std::path::PathBuf;
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// (name, source, expected stdout)
const RUN: &[(&str, &str, &str)] = &[
    ("hello", "program Hello; begin WriteLn('OK') end.", "OK\n"),
    (
        "write_vs_writeln",
        "program W;\nbegin\n  Write('a'); Write('b'); WriteLn;\n  WriteLn('c');\n  Write('d')\nend.",
        "ab\nc\nd",
    ),
    (
        "several_arguments",
        "program M; begin WriteLn('a', 'b', 'c'); Write('x', 'y'); WriteLn('', 'z') end.",
        "abc\nxyz\n",
    ),
    ("bare_writeln", "program B; begin WriteLn; WriteLn; WriteLn end.", "\n\n\n"),
    ("writeln_empty_string", "program B; begin WriteLn(''); Write('') end.", "\n"),
    ("empty_body", "program E; begin end.", ""),
    ("write_empty_string_only", "program Z; begin Write('') end.", ""),
    ("header_input_output", "program P(Input, Output); begin WriteLn('p') end.", "p\n"),
    ("header_output", "program P(Output); begin WriteLn('q') end.", "q\n"),
    ("no_header", "begin WriteLn('x') end.", "x\n"),
    ("begin_end_only", "begin end.", ""),
    (
        "quote_escapes",
        "program S; begin WriteLn('it''s'); WriteLn(''''); WriteLn('a''''b'); WriteLn('') end.",
        "it's\n'\na''b\n\n",
    ),
    (
        "char_codes",
        "program C; begin WriteLn('a'#65'b'); Write(#72#105); WriteLn; WriteLn(#33, 'x'#33#34'y') end.",
        "aAb\nHi\n!x!\"y\n",
    ),
    (
        "control_chars",
        "program K; begin WriteLn('a', ^G, 'b'); WriteLn('p'^I'q'); Write(^I^I); WriteLn end.",
        "a\x07b\np\tq\n\t\t\n",
    ),
    (
        "empty_statements",
        "program T; begin ; WriteLn('a'); ; WriteLn('b'); end.",
        "a\nb\n",
    ),
    (
        "case_insensitive",
        "PROGRAM c; BEGIN WRITELN('A'); writeln('b'); WriteLn('c') END.",
        "A\nb\nc\n",
    ),
    (
        "comments",
        "program Cm; { c } begin (* d *) WriteLn('a') { e } ; WriteLn { f } ('b') end. { t }",
        "a\nb\n",
    ),
    (
        "text_after_final_dot_ignored",
        "program A; begin WriteLn('a') end. this is ignored ((( 'x",
        "a\n",
    ),
    (
        "nested_compound",
        "program N; begin begin WriteLn('a') end; begin end end.",
        "a\n",
    ),
    (
        "multiline",
        "program P;\n\n\nbegin\n  WriteLn('one');\n\n  WriteLn('two')\nend.",
        "one\ntwo\n",
    ),
];

/// (name, source, TP error number, line)
const ERR: &[(&str, &str, u16, u32)] = &[
    ("no_begin", "program P;\nWriteLn('x') end.", 36, 2),
    ("end_without_begin", "program P;\nend.", 36, 2),
    (
        "declaration_before_begin",
        "program P;\n  'x'\nbegin end.",
        36,
        2,
    ),
    (
        "second_program_header",
        "program P; program Q; begin end.",
        36,
        1,
    ),
    ("header_missing_semicolon", "program P\nbegin end.", 85, 2),
    ("header_dot_for_semicolon", "program P.\nbegin end.", 85, 1),
    (
        "statement_missing_semicolon",
        "program P;\nbegin\n  WriteLn('a') WriteLn('b')\nend.",
        85,
        3,
    ),
    (
        "end_missing_before_dot",
        "program P;\nbegin\n  WriteLn('x')\n.",
        85,
        4,
    ),
    (
        "begin_after_statement",
        "program P;\nbegin\n  WriteLn('x') begin\nend.",
        85,
        3,
    ),
    (
        "until_after_statement",
        "program P;\nbegin\n  WriteLn('x') until\nend.",
        85,
        3,
    ),
    (
        "dot_inside_body",
        "program P;\nbegin\n  WriteLn('a').\nend.",
        85,
        3,
    ),
    (
        "paren_omitted",
        "program P;\nbegin\n  WriteLn 'a'\nend.",
        85,
        3,
    ),
    (
        "final_dot_missing_semicolon",
        "program P;\nbegin\nend;\n",
        94,
        3,
    ),
    ("final_dot_missing_text", "program P; begin end ;", 94, 1),
    (
        "second_end",
        "program P;\nbegin\n  WriteLn('x')\n  end\n  end.",
        94,
        5,
    ),
    ("header_name_missing", "program ;\nbegin end.", 2, 1),
    ("header_name_string", "program 'x';\nbegin end.", 2, 1),
    ("header_name_number", "program 5;\nbegin end.", 2, 1),
    ("header_name_keyword", "program begin;\nbegin end.", 2, 1),
    ("eof_empty_file", "", 10, 1),
    ("eof_blank_lines", "\n", 10, 2),
    ("eof_after_program", "program", 10, 1),
    ("eof_after_name", "program P", 10, 1),
    ("eof_after_header", "program P;", 10, 1),
    ("eof_after_begin", "program P;\nbegin", 10, 2),
    ("eof_after_end", "program P;\nbegin\nend", 10, 3),
    ("eof_in_arguments", "program P;\nbegin\n  WriteLn(", 10, 3),
    (
        "eof_after_argument",
        "program P;\nbegin\n  WriteLn('x'",
        10,
        3,
    ),
    ("eof_in_comment", "program P;\nbegin\n  { open\n", 10, 4),
    (
        "rparen_missing",
        "program P;\nbegin\n  WriteLn('a'\nend.",
        89,
        4,
    ),
    (
        "rparen_two_strings",
        "program P;\nbegin\n  WriteLn('a' 'b')\nend.",
        89,
        3,
    ),
    (
        "rparen_after_semicolon",
        "program P;\nbegin\n  Write('a' , 'b'\n  ;\nend.",
        89,
        4,
    ),
    (
        "trailing_comma",
        "program P;\nbegin\n  WriteLn('a',)\nend.",
        42,
        3,
    ),
    (
        "keyword_as_argument",
        "program P;\nbegin\n  WriteLn(end\nend.",
        42,
        3,
    ),
    ("string_statement", "program P;\nbegin\n  'x'\nend.", 113, 3),
    (
        "semicolon_dot",
        "program P;\nbegin\n  WriteLn('x');\n.",
        113,
        4,
    ),
    (
        "unknown_procedure",
        "program P;\nbegin\n  Foo('x')\nend.",
        3,
        3,
    ),
    (
        "unknown_argument",
        "program P;\nbegin\n  WriteLn(x)\nend.",
        3,
        3,
    ),
    (
        "unterminated_string",
        "program P;\nbegin\n  WriteLn('abc\nend.",
        8,
        3,
    ),
    ("bare_hash", "program P;\nbegin\n  WriteLn(#)\nend.", 7, 3),
];

fn wasm_dir() -> PathBuf {
    crate::common::fresh_dir("skeleton")
}

fn run_wasm(name: &str, wasm: &[u8]) -> Result<Vec<u8>, String> {
    let path = wasm_dir().join([name, ".wasm"].concat());
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
fn error_columns_are_one_based() {
    for &(name, src, _, line) in ERR.iter().filter(|e| e.3 > 0) {
        if let Err(d) = tp5c::compile([src, "\n"].concat().as_bytes()) {
            assert!(d.col >= 1 && d.line >= 1, "{name}: {d:?} (tp5 line {line})");
        }
    }
}

#[test]
fn only_the_listed_programs_print_nothing() {
    let silent: Vec<_> = RUN.iter().filter(|r| r.2.is_empty()).map(|r| r.0).collect();
    assert_eq!(
        silent,
        ["empty_body", "write_empty_string_only", "begin_end_only"]
    );
}

/// Slow (DOSBox-X, about a second per program): `cargo test -- --ignored`.
#[test]
#[ignore]
fn oracle_agrees() {
    let dir = wasm_dir();
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
