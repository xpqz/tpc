//! Text files and console input (`Text`, `Input`, `Output`, Assign/Reset/Rewrite/Append/Close,
//! Read/ReadLn/Write/WriteLn with an optional file, Eof/Eoln, IOResult, `{$I+}`/`{$I-}`).
//! Every `.out` and `.err` (`N LINE`) of the fixtures comes from the real TP 5.0
//! (`tools/tp5.sh`); the corpus programs this slice must make pass are in tests/{lang,abi,rt,neg}.
//!
//! TP rules the fixtures pin that the plan does not state:
//! - while an I/O error is pending (`{$I-}`), Reset, Rewrite, Append and Close still run and a
//!   failure of theirs replaces the pending code; Read, ReadLn, Write, WriteLn, Eof and Eoln do
//!   nothing, except that the items of a Read become 0, '', ^Z (a Char) and the Eofs are False;
//!   Assign never fails and never raises the pending error, program end ignores it;
//! - the check of `{$I+}` follows each I/O statement (and each Eof/Eoln call) in the source
//!   text where the switch is on; the switch is positional, also across routines, and an
//!   include file's switch stays after it;
//! - a line ends at CR (then an optional LF); a LF alone is a character; `ReadLn` after a
//!   `Read(c)` that returned the CR skips up to the next CR;
//! - `Read` of a number into a narrower type keeps the low bits; a number beyond LongInt,
//!   or one followed by letters, is 106; `Eof` of a file open for output is 104;
//! - a Text variable is not a record: no fields (121), no comparison (41), no assignment
//!   (43), not a value parameter (126), not a function result (34), `Eof(n)` is 77.
mod common;
mod driver;
mod sysutil;
use common::{assert_hello_fits, node};
use driver::*;
use std::path::{Path, PathBuf};
use sysutil::{compile_with, system_pas};

fn fx(sub: &str) -> PathBuf {
    root().join("compiler/tests/fixtures/text_files").join(sub)
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

// ---- Own programs ----

#[test]
fn own_lang_programs_print_what_tp5_prints() {
    assert_eq!(stems(&fx("lang"), ".pas").len(), 17);
    assert_eq!(stems(&fx("lang"), ".pas"), stems(&fx("lang"), ".out"));
    for s in stems(&fx("lang"), ".out") {
        let out = std::fs::read(fx("lang").join(format!("{s}.out"))).unwrap();
        assert!(!out.is_empty(), "{s}: empty expected output");
    }
    assert_clean(check_lang(&fx("lang"), &real_compile));
}

#[test]
fn own_runtime_error_programs_end_with_the_tp5_error() {
    assert_eq!(stems(&fx("rt"), ".pas").len(), 6);
    assert_eq!(stems(&fx("rt"), ".pas"), stems(&fx("rt"), ".out"));
    assert_clean(check_rt(&fx("rt"), &real_compile));
}

#[test]
fn own_invalid_programs_report_tp5_number_and_line() {
    assert_eq!(stems(&fx("neg"), ".pas").len(), 25);
    assert_eq!(stems(&fx("neg"), ".pas"), stems(&fx("neg"), ".err"));
    assert_clean(check_neg(&fx("neg"), &real_compile));
}

// ---- The corpus programs this slice must make pass ----

const LANG: &[&str] = &[
    "console_count_lines_words_and_characters",
    "console_echo_lines_until_end_of_input",
    "console_eof_on_empty_input",
    "console_eoln_reports_end_of_line",
    "console_input_and_files_together",
    "console_input_and_output_named_explicitly",
    "console_menu_loop_reads_commands",
    "console_mixed_number_then_text_on_one_line",
    "console_prompt_then_read_answers",
    "console_read_chars_returns_line_end_bytes",
    "console_read_integers_and_sum",
    "console_read_real_and_longint",
    "console_readln_without_arguments_skips_a_line",
    "ioresult_append_to_missing_file_fails",
    "ioresult_bad_number_format_is_106",
    "ioresult_checking_on_again_after_ioplus",
    "ioresult_error_codes_for_misuse",
    "ioresult_file_exists_idiom",
    "ioresult_is_zero_after_successful_operations",
    "ioresult_missing_file_is_reported",
    "ioresult_pending_error_skips_following_io",
    "ioresult_reading_at_end_of_file_is_not_an_error",
    "ioresult_unit_with_checking_off_reports_codes",
    "ioresult_value_is_cleared_when_read",
    "ioresult_write_after_failed_open_with_output_to_console",
    "report_generator_reads_data_writes_report_and_reads_it_back",
    "report_line_statistics_of_a_text_file",
    "report_merge_two_files_into_third",
    "report_numbers_survive_a_round_trip_through_a_file",
    "report_sorted_records_written_and_read_back",
    "textfile_append_continues_after_existing_text",
    "textfile_append_to_existing_data_file",
    "textfile_array_of_files",
    "textfile_ctrl_z_ends_the_text",
    "textfile_eof_on_empty_file",
    "textfile_eoln_inside_and_at_end_of_lines",
    "textfile_file_name_ignores_case",
    "textfile_file_passed_as_var_parameter",
    "textfile_final_line_without_line_end",
    "textfile_line_longer_than_buffer_and_string",
    "textfile_many_lines_cross_buffer_boundaries",
    "textfile_read_char_returns_line_end_bytes",
    "textfile_read_integers_across_lines",
    "textfile_read_mixed_fields_of_a_record_line",
    "textfile_read_real_numbers",
    "textfile_read_string_into_short_string_keeps_rest",
    "textfile_read_string_stops_at_end_of_line",
    "textfile_readln_lines_until_eof",
    "textfile_readln_skips_rest_of_the_line",
    "textfile_reset_rereads_from_the_start",
    "textfile_rewrite_truncates_existing_file",
    "textfile_two_files_open_at_once",
    "textfile_write_formats",
    "textfile_write_then_read_back",
    "textfile_writeln_without_arguments_writes_empty_lines",
];

const ABI: &[&str] = &[
    "textfile_as_record_field_and_local_variable",
    "textfile_local_variables_in_recursive_procedure",
    "textfile_string_parameters_and_function_results_around_io",
    "unit_text_file_variable_owned_by_unit",
];

const RT: &[&str] = &[
    "console_bad_integer_in_input",
    "file_append_to_missing_file",
    "file_bad_integer_in_text",
    "file_bad_real_in_text",
    "file_close_of_file_never_opened",
    "file_eof_of_unassigned_file",
    "file_error_after_partial_output_line",
    "file_error_inside_unit_routine",
    "file_io_checking_back_on_after_ioplus",
    "file_io_checking_of_unit_does_not_leak_into_program",
    "file_read_after_close",
    "file_read_from_file_never_opened",
    "file_read_from_file_open_for_output",
    "file_reset_of_file_in_missing_directory",
    "file_reset_of_missing_file",
    "file_reset_of_unassigned_file",
    "file_rewrite_in_missing_directory",
    "file_write_after_close",
    "file_write_to_file_open_for_input",
];

const NEG: &[&str] = &[
    "text_assign_name_must_be_a_string",
    "text_file_assignment_is_not_allowed",
    "text_file_passed_by_value",
    "text_ioresult_is_read_only",
    "text_read_boolean_from_file",
    "text_reset_of_integer_variable",
    "text_write_record_to_file",
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

/// Copies `names` of a corpus directory with their expectation, console input and auxiliary
/// directory into a fresh directory (no PENDING file).
fn subset(kind: &str, names: &[&str], expected: &str) -> PathBuf {
    let from = root().join("tests").join(kind);
    let dir = scratch("x").parent().unwrap().to_path_buf();
    for n in names {
        for e in ["pas", expected, "in"] {
            let file = from.join(format!("{n}.{e}"));
            if file.exists() {
                std::fs::copy(&file, dir.join(format!("{n}.{e}"))).unwrap();
            }
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
fn corpus_abi_programs_match_tp5_output() {
    assert_clean(check_lang(&subset("abi", ABI, "out"), &real_compile));
}

#[test]
fn corpus_runtime_error_programs_end_with_the_tp5_error() {
    assert_clean(check_rt(&subset("rt", RT, "out"), &real_compile));
}

#[test]
fn corpus_neg_programs_report_tp5_number_and_line() {
    assert_clean(check_neg(&subset("neg", NEG, "err"), &real_compile));
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
    assert_eq!((LANG.len(), ABI.len(), RT.len(), NEG.len()), (55, 4, 19, 7));
}

// ---- Which host calls a program needs ----

fn compiled(program: &str) -> Vec<u8> {
    compile_with(&system_pas(), program).unwrap_or_else(|d| panic!("{d:?}\n{program}"))
}

/// The imports of a module as `name` strings (module `host` only).
fn imports(wasm: &[u8]) -> Vec<String> {
    let js = "for (const i of WebAssembly.Module.imports(new WebAssembly.Module(b))) \
              console.log(i.module + '.' + i.name);";
    node(js, wasm)
        .lines()
        .map(|line| line.strip_prefix("host.").expect(line).to_string())
        .collect()
}

fn has_import(names: &[String], name: &str) -> bool {
    names.iter().any(|n| n == name)
}

const HELLO: &str = "program Hello;\nbegin\n  WriteLn('hello')\nend.";

#[test]
fn hello_world_stays_small_although_output_goes_through_system() {
    // GUARD: Write routes through a System routine that knows files; DCE keeps hello small.
    let wasm = compiled(HELLO);
    assert_hello_fits(&wasm);
    assert!(wasm.len() <= 1024, "hello-world is {} bytes", wasm.len());
}

#[test]
fn a_program_that_only_writes_needs_no_input_or_open_and_close_calls() {
    // GUARD: the positive half first, so an empty list cannot pass.
    let names = imports(&compiled(HELLO));
    assert!(has_import(&names, "write"), "{names:?}");
    for unwanted in ["readline", "fs_open", "fs_read", "fs_close"] {
        assert!(!has_import(&names, unwanted), "{names:?}");
    }
}

#[test]
fn reading_the_console_imports_readline() {
    let names = imports(&compiled(
        "program P;\nvar n: Integer;\nbegin\n  ReadLn(n);\n  WriteLn(n)\nend.",
    ));
    assert!(has_import(&names, "readline"), "{names:?}");
    assert!(has_import(&names, "write"), "{names:?}");
}

#[test]
fn a_program_that_uses_a_file_imports_the_file_calls() {
    let names = imports(&compiled(
        "program P;\nvar f: Text; s: string;\nbegin\n  Assign(f, 'A.TXT');\n  Reset(f);\n  \
         ReadLn(f, s);\n  Close(f);\n  Assign(f, 'B.TXT');\n  Rewrite(f);\n  WriteLn(f, s);\n  \
         Close(f)\nend.",
    ));
    for wanted in ["fs_open", "fs_read", "fs_write", "fs_close"] {
        assert!(has_import(&names, wanted), "{names:?}");
    }
}

// ---- The I/O checking switch ----

fn compile_text(program: &str) -> Result<Vec<u8>, String> {
    compile_with(&system_pas(), program).map_err(|d| format!("{d:?}"))
}

#[test]
fn io_check_switches_are_accepted_and_are_not_includes() {
    // Formerly error 200; never 15 (file not found), as `{$I name}` is an include.
    for directive in ["{$I+}", "{$I-}", "{$i-}", "(*$I+*)", "{$I-,R+}", "{$R+,I-}"] {
        let program = format!("program P;\n{directive}\nbegin WriteLn(1) end.\n");
        assert!(compile_text(&program).is_ok(), "{directive}");
    }
}

#[test]
fn the_switch_decides_at_compile_time_whether_a_statement_is_checked() {
    // With checking off the statement is followed by no call of the check, so the module is
    // smaller; with checking on again it is the same module as a program without a switch.
    let body = "var f: Text;\nbegin\n  Assign(f, 'A.TXT');\n  Reset(f);\n  WriteLn(f, 1);\n  \
                Close(f)\nend.";
    let on = compiled(&format!("program P;\n{body}"));
    let off = compiled(&format!("program P;\n{{$I-}}\n{body}"));
    let off_then_on = compiled(&format!("program P;\n{{$I-}}\n{{$I+}}\n{body}"));
    assert!(off.len() < on.len(), "off {} on {}", off.len(), on.len());
    assert_eq!(on, off_then_on);
}

#[test]
fn a_checked_program_runs_its_check_after_every_io_statement() {
    // The default is on: a failing Reset ends the program with runtime error 2 right there.
    let wasm = compiled(
        "program P;\nvar f: Text;\nbegin\n  Assign(f, 'NONE.TXT');\n  Reset(f);\n  WriteLn(1)\nend.",
    );
    let js = "const m = new WebAssembly.Module(b); const out = []; \
        const host = { write: (fd, p, n) => out.push(n), halt: () => {}, \
          rterror: (c) => { out.push('E' + c); throw new Error('stop'); }, \
          fs_open: () => -2, fs_read: () => 0, fs_write: (h, p, n) => n, fs_close: () => 0, \
          readline: () => -1 }; \
        try { new WebAssembly.Instance(m, { host }).exports.main(); } catch (e) {} \
        console.log(out.join(' '));";
    assert_eq!(node(js, &wasm).trim(), "E2");
}
