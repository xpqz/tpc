//! Include files and conditional compilation: `{$I name}`, `{$DEFINE}`, `{$UNDEF}`, `{$IFDEF}`,
//! `{$IFNDEF}`, `{$ELSE}`, `{$ENDIF}`, the predefined symbols, and diagnostics that name the
//! include file they are in (decision 28). Every `.out` and `.err` (`N LINE`, `N LINE FILE` for
//! an error in a unit or include file) comes from the real TP 5.0 (`tools/tp5.sh`). Where include
//! files are searched is a decision of this compiler (decision 27), not TP's.
//!
//! TP rules the fixtures pin that the plan does not state:
//! - an include is allowed anywhere in a declaration part, also inside a record type, a routine
//!   heading or a constant expression, and before `program`; it is error 118 from `begin` up to
//!   the token after the matching `end` (so also between `end` and its `;`), in a unit's
//!   initialisation part too; an include file that ends inside a statement part is error 10;
//! - the name ends at the first blank, a name without a dot means `NAME.PAS`, an empty name is
//!   error 15; includes nest 14 deep, the 15th is error 9 at the directive in the 14th file;
//! - a conditional must close in the file that opened it: `$ENDIF` or `$ELSE` in an include for
//!   a conditional of the includer is 128, an `$IF` left open in an include is 129 (reported on
//!   the include's last line); a conditional open at the final `end.` is accepted;
//! - skipped text is scanned only for strings (to the end of the line, no error if unterminated),
//!   comments and conditional directives: a string or comment hides a directive, other
//!   directives (also invalid ones) and `end.` are ignored; a `{` never closed in skipped text
//!   runs to the end of the file and is 129 on its last line;
//! - text after the symbol is ignored, a second `$ELSE` switches back, symbols are not case
//!   sensitive, `$DEFINE`/`$UNDEF` need an identifier (2), `$ENDIF` without `$IF` is 128;
//! - TP defines VER50, MSDOS, CPU86 and CPU87 (CPU87 because the oracle machine has a
//!   coprocessor, a deviation to record: the compiler always defines it);
//! - a `{$R+}` or `{$R-}` in an include stays in force after it; one in a unit does not reach
//!   the program.
mod common;
mod driver;
use driver::*;
use std::path::{Path, PathBuf};
use std::process::Command;

fn fx(sub: &str) -> PathBuf {
    root().join("compiler/tests/fixtures/directives").join(sub)
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
    assert_eq!(stems(&fx("lang"), ".pas").len(), 22);
    assert_eq!(stems(&fx("lang"), ".pas"), stems(&fx("lang"), ".out"));
    for s in stems(&fx("lang"), ".out") {
        let out = std::fs::read(fx("lang").join(format!("{s}.out"))).unwrap();
        assert!(!out.is_empty(), "{s}: empty expected output");
    }
    assert_clean(check_lang(&fx("lang"), &real_compile));
}

#[test]
fn own_runtime_error_program_reports_201_after_the_range_switch_of_an_include() {
    assert_eq!(
        stems(&fx("rt"), ".pas"),
        ["range_switch_in_an_include_stays_on_after_it"]
    );
    let out = std::fs::read_to_string(fx("rt/range_switch_in_an_include_stays_on_after_it.out"));
    assert_eq!(out.unwrap(), "before\nRuntime error 201\n");
    assert_clean(check_rt(&fx("rt"), &real_compile));
}

#[test]
fn own_invalid_programs_report_tp5_number_line_and_file() {
    assert_eq!(stems(&fx("neg"), ".pas").len(), 28);
    assert_eq!(stems(&fx("neg"), ".pas"), stems(&fx("neg"), ".err"));
    assert_clean(check_neg(&fx("neg"), &real_compile));
}

// ---- The corpus programs this slice must make pass ----

const LANG: &[&str] = &[
    "ifdef_conditional_around_include",
    "ifdef_conditional_spanning_an_include_boundary",
    "ifdef_define_in_include_continues_after_include",
    "ifdef_define_in_program_is_not_visible_in_unit",
    "ifdef_define_in_unit_is_not_visible_in_program",
    "ifdef_defined_symbol_selects_text",
    "ifdef_else_branch_taken_when_undefined",
    "ifdef_inside_routines_and_between_statements",
    "ifdef_long_comment_form",
    "ifdef_nested_conditionals",
    "ifdef_nested_inside_skipped_text_is_balanced",
    "ifdef_selects_declarations",
    "ifdef_skipped_text_ignores_other_directives",
    "ifdef_skipped_text_may_be_invalid_pascal",
    "ifdef_symbol_names_ignore_case",
    "ifdef_then_branch_taken_when_defined_with_else",
    "ifdef_undefined_symbol_skips_text",
    "ifndef_selects_text_when_undefined",
    "include_constants_types_and_variables",
    "include_default_extension_is_pas",
    "include_directive_spellings",
    "include_file_with_comments_and_directives",
    "include_in_a_nested_routine_declaration_part",
    "include_inside_a_unit",
    "include_nested_files",
    "include_routines_between_declarations",
    "include_skipped_by_conditional_is_not_read",
];

const NEG: &[&str] = &[
    "conditional_else_without_if",
    "conditional_if_never_closed",
    "conditional_if_opened_in_include_closed_outside",
    "conditional_ifdef_without_symbol",
    "error_inside_an_include_names_the_include_file",
    "include_file_not_found",
    "include_inside_statement_part",
];

fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), target).unwrap();
        }
    }
}

/// A fresh scratch directory.
fn work() -> PathBuf {
    scratch("x").parent().unwrap().to_path_buf()
}

/// Copies `names` of a corpus directory with their expectation and auxiliary directory into a
/// fresh directory (no PENDING file).
fn subset(kind: &str, names: &[&str], expected: &str) -> PathBuf {
    let from = root().join("tests").join(kind);
    let dir = work();
    for n in names {
        for e in ["pas", expected] {
            std::fs::copy(from.join(format!("{n}.{e}")), dir.join(format!("{n}.{e}"))).unwrap();
        }
        if from.join(n).is_dir() {
            copy_dir(&from.join(n), &dir.join(n));
        }
    }
    dir
}

#[test]
fn corpus_lang_programs_match_tp5_output() {
    assert_clean(check_lang(&subset("lang", LANG, "out"), &real_compile));
}

#[test]
fn corpus_neg_programs_report_tp5_number_line_and_file() {
    assert_clean(check_neg(&subset("neg", NEG, "err"), &real_compile));
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
            // These name an include that is never read, or none at all.
            let no_files = [
                "include_file_not_found",
                "include_skipped_by_conditional_is_not_read",
            ];
            let needs_files = n.contains("include") && !no_files.contains(n);
            let has_files = root().join(format!("tests/{kind}/{n}")).is_dir();
            assert!(
                has_files || !needs_files,
                "{kind}/{n} has no auxiliary directory"
            );
            assert!(
                !pending.lines().any(|l| l.trim() == *n),
                "{kind}/{n} is still in PENDING"
            );
        }
    }
    assert_eq!((LANG.len(), NEG.len()), (27, 7));
}

// ---- Where include files are found (the CLI registers the files, decision 27) ----

fn tp5c(cwd: &Path, args: &[&str]) -> (i32, String) {
    let o = Command::new(env!("CARGO_BIN_EXE_tp5c"))
        .current_dir(cwd)
        .args(args)
        .output()
        .unwrap();
    (
        o.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&o.stderr).into_owned(),
    )
}

fn run(dir: &Path, wasm: &str) -> String {
    let o = Command::new("node")
        .arg(root().join("tests/run.mjs"))
        .arg(dir.join(wasm))
        .output()
        .expect("node must be installed");
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap()
}

fn write(dir: &Path, name: &str, text: &str) {
    std::fs::write(dir.join(name), text).unwrap();
}

#[test]
fn include_files_beside_the_input_are_found_by_every_fixture() {
    // The lang fixtures, copied with their files next to the program: same output.
    let mut with_files = 0;
    for s in stems(&fx("lang"), ".pas") {
        if !fx("lang").join(&s).is_dir() {
            continue;
        }
        with_files += 1;
        let dir = work();
        copy_dir(&fx("lang").join(&s), &dir);
        std::fs::copy(fx("lang").join(format!("{s}.pas")), dir.join("p.pas")).unwrap();
        let (code, err) = tp5c(&dir, &["p.pas", "-o", "p.wasm"]);
        assert_eq!((code, err.as_str()), (0, ""), "{s}");
        let want = std::fs::read_to_string(fx("lang").join(format!("{s}.out"))).unwrap();
        assert_eq!(run(&dir, "p.wasm"), want, "{s}");
    }
    assert_eq!(with_files, 13);
}

#[test]
fn include_file_names_ignore_case_and_a_name_without_dot_means_pas() {
    let dir = work();
    write(
        &dir,
        "p.pas",
        "{$I part.inc}\n{$i PIECE}\nbegin WriteLn(A + B); end.\n",
    );
    write(&dir, "PART.INC", "const A = 1;\n");
    write(&dir, "piece.Pas", "const B = 20;\n");
    assert_eq!(tp5c(&dir, &["p.pas", "-o", "p.wasm"]), (0, String::new()));
    assert_eq!(run(&dir, "p.wasm"), "21\n");
}

#[test]
fn auxiliary_directory_include_wins_over_the_input_directory() {
    let dir = work();
    write(&dir, "p.pas", "{$I PART.INC}\nbegin WriteLn(A); end.\n");
    write(&dir, "PART.INC", "const A = 1;\n");
    std::fs::create_dir(dir.join("aux")).unwrap();
    write(&dir.join("aux"), "PART.INC", "const A = 2;\n");
    let args = ["p.pas", "-o", "p.wasm", "--dir", "aux"];
    assert_eq!(tp5c(&dir, &args), (0, String::new()));
    assert_eq!(run(&dir, "p.wasm"), "2\n");
}

#[test]
fn only_the_include_files_a_program_reaches_are_read() {
    let dir = work();
    write(
        &dir,
        "p.pas",
        "{$IFDEF X}\n{$I JUNK.INC}\n{$ENDIF}\n{$I GOOD.INC}\nbegin WriteLn(A); end.\n",
    );
    write(&dir, "JUNK.INC", "this is not pascal ((\n");
    write(&dir, "UNUSED.INC", "{ unterminated\n");
    write(&dir, "GOOD.INC", "const A = 3;\n");
    assert_eq!(tp5c(&dir, &["p.pas", "-o", "p.wasm"]), (0, String::new()));
    assert_eq!(run(&dir, "p.wasm"), "3\n");
}

#[test]
fn an_error_in_an_include_prints_its_file_and_one_after_it_the_input_name() {
    let dir = work();
    write(
        &dir,
        "p.pas",
        "{$I BAD.INC}\nconst\n  K = 1;\nbegin Nope := 1; end.\n",
    );
    write(&dir, "BAD.INC", "const\n  A = 1;\nvar x: Nope;\n");
    let (code, err) = tp5c(&dir, &["p.pas", "-o", "p.wasm"]);
    assert_eq!(code, 1);
    assert!(err.starts_with("BAD.INC(3,"), "{err}");
    assert!(err.contains(": Error 3: "), "{err}");
    write(&dir, "BAD.INC", "const\n  A = 1;\n");
    let (code, err) = tp5c(&dir, &["p.pas", "-o", "p.wasm"]);
    assert_eq!(code, 1);
    assert!(err.starts_with("p.pas(4,"), "{err}");
}

// ---- Switch letters that share `{$I`: the include form must not swallow `{$I+}` / `{$I-}`
// (`{$I+}` and `{$I-}` themselves are tested in text_files.rs) ----

fn compile_text(text: &str) -> (i32, String) {
    let dir = work();
    write(&dir, "p.pas", text);
    tp5c(&dir, &["p.pas", "-o", "p.wasm"])
}

#[test]
fn io_check_switches_in_skipped_text_are_ignored() {
    let (code, err) = compile_text("program P;\n{$IFDEF X}\n{$I-}\n{$ENDIF}\nbegin end.\n");
    assert_eq!((code, err.as_str()), (0, ""));
}

#[test]
fn an_include_with_a_name_that_starts_with_a_switch_letter_is_an_include() {
    // `{$R...}` names stay switches; `{$I name}` and `{$I+name}` differ only by what follows.
    let dir = work();
    write(&dir, "p.pas", "{$I R.INC}\nbegin WriteLn(A); end.\n");
    write(&dir, "R.INC", "const A = 5;\n");
    assert_eq!(tp5c(&dir, &["p.pas", "-o", "p.wasm"]), (0, String::new()));
    assert_eq!(run(&dir, "p.wasm"), "5\n");
}

#[test]
fn conditional_directives_need_whole_words() {
    // `IFDEFX`, `ENDIFS` and `DEFINED` are not directives; TP reports error 17 for them.
    for directive in ["{$IFDEFX}", "{$ENDIFS}", "{$DEFINED X}", "{$ELSEX}"] {
        let (code, err) = compile_text(&format!("program P;\n{directive}\nbegin end.\n"));
        assert_eq!(code, 1, "{directive}");
        assert!(err.starts_with("p.pas(2,"), "{directive}: {err}");
        assert!(err.contains(": Error 17: "), "{directive}: {err}");
    }
}

// ---- Directives belong to the front end, not the checker ----

#[test]
fn a_program_with_only_taken_branches_equals_the_same_program_without_directives() {
    let with = "program P;\n{$DEFINE A}\nvar x: Integer;\nbegin\n{$IFDEF A}\n  x := 1;\n{$ELSE}\n  x := 2;\n{$ENDIF}\n  WriteLn(x);\nend.\n";
    let without =
        "program P;\n\n\nvar x: Integer;\nbegin\n\n  x := 1;\n\n\n\n  WriteLn(x);\nend.\n";
    let build = |text: &str| {
        let dir = work();
        write(&dir, "p.pas", text);
        assert_eq!(tp5c(&dir, &["p.pas", "-o", "p.wasm"]), (0, String::new()));
        std::fs::read(dir.join("p.wasm")).unwrap()
    };
    let a = build(with);
    assert!(a.len() > 8);
    assert_eq!(a, build(without));
}

// ---- The wasm API: include files are registered like units (decision 27) ----

#[test]
fn wasm_api_reads_include_files_from_the_file_table_and_reports_their_file() {
    let target = Command::new("sh")
        .arg(root().join("scripts/cargo-target.sh"))
        .output()
        .unwrap();
    let target = String::from_utf8(target.stdout).unwrap();
    let target = target.trim();
    let build = Command::new("cargo")
        .args([
            "build",
            "--release",
            "--lib",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .current_dir(root())
        .env("CARGO_TARGET_DIR", target)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "{}",
        String::from_utf8_lossy(&build.stderr)
    );
    let wasm = PathBuf::from(target).join("wasm32-unknown-unknown/release/tp5c.wasm");
    let out = Command::new("node")
        .arg(root().join("tests/wasm_include.mjs"))
        .arg(&wasm)
        .arg(fx("lang/include_nested_fourteen_deep.pas"))
        .arg(fx(
            "neg/error_in_an_include_nested_in_an_include_names_the_inner_file.pas",
        ))
        .arg(fx("neg/include_inside_a_case_statement.pas"))
        .output()
        .expect("node must be installed");
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}
