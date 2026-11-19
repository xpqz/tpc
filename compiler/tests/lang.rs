mod common;
mod driver;
use driver::*;

fn lang(name: &str) -> Vec<String> {
    check_lang(&fixture(&format!("lang/{name}")), &fake_compile)
}

fn has(fails: &[String], parts: &[&str]) -> bool {
    fails.iter().any(|f| parts.iter().all(|p| f.contains(p)))
}

#[test]
fn corpus_has_no_unexpected_results() {
    let fails: Vec<String> = ["lang", "abi"]
        .iter()
        .flat_map(|d| check_lang(&root().join("tests").join(d), &real_compile))
        .collect();
    assert!(fails.is_empty(), "{}", fails.join("\n"));
}

#[test]
fn runtime_error_corpus_has_no_unexpected_results() {
    let fails = check_rt(&root().join("tests/rt"), &real_compile);
    assert!(fails.is_empty(), "{}", fails.join("\n"));
}

#[test]
fn runtime_error_program_that_exits_normally_is_reported() {
    let f = check_rt(&fixture("rt/exit_mismatch"), &fake_compile);
    assert_eq!(f, ["exit_mismatch: run exited 0, want 200: "]);
}

#[test]
fn runtime_error_out_without_error_line_is_reported() {
    let f = check_rt(&fixture("rt/no_error_line"), &fake_compile);
    assert_eq!(f, ["no_error_line: no `Runtime error N` line in .out"]);
}

#[test]
fn matching_program_passes() {
    assert_eq!(lang("pass"), Vec::<String>::new());
}

#[test]
fn output_mismatch_is_reported() {
    let f = lang("mismatch");
    assert_eq!(f.len(), 1, "{f:?}");
    assert!(has(&f, &["mismatch: run exited 1", "FAIL"]), "{f:?}");
}

#[test]
fn compile_failure_is_reported_with_its_diagnostic() {
    let f = lang("compile_fails");
    assert_eq!(f.len(), 1, "{f:?}");
    assert!(
        has(&f, &["compile_fails: compile exited 1", "Error 5"]),
        "{f:?}"
    );
}

#[test]
fn pending_program_that_fails_is_accepted() {
    assert_eq!(lang("pending_fails"), Vec::<String>::new());
}

#[test]
fn pending_program_that_passes_is_a_failure() {
    let f = lang("pending_passes");
    assert_eq!(f, ["pending_passes: passes but is listed in PENDING"]);
}

#[test]
fn pending_name_without_program_is_a_failure() {
    let f = lang("stale_pending");
    assert_eq!(f, ["gone: PENDING lists a program that does not exist"]);
}

#[test]
fn every_failure_is_reported_in_one_run() {
    let f = lang("several");
    assert_eq!(f.len(), 3, "{f:?}");
    for name in ["bad_a", "bad_b", "bad_c"] {
        assert!(has(&f, &[name]), "{name} missing from {f:?}");
    }
}

#[test]
fn failing_compile_fails_real_programs_that_are_not_pending() {
    let f = check_lang(&fixture("lang/real_unsupported"), &real_compile);
    assert_eq!(f.len(), 1, "{f:?}");
    assert!(
        has(
            &f,
            &[
                "hello: compile exited 1",
                "hello.pas(1,7): Error 3: Unknown identifier"
            ]
        ),
        "{f:?}"
    );
}

#[test]
fn failing_compile_is_accepted_for_pending_programs() {
    assert_eq!(
        check_lang(&fixture("lang/real_pending"), &real_compile),
        Vec::<String>::new()
    );
}

#[test]
fn aux_directory_is_available_to_the_program() {
    assert_eq!(lang("aux_file"), Vec::<String>::new());
}

#[test]
fn program_does_not_see_files_that_are_not_in_its_aux_directory() {
    let f = lang("aux_missing");
    assert_eq!(f.len(), 1, "{f:?}");
    assert!(has(&f, &["p: run exited 1", "Runtime error 002"]), "{f:?}");
}

#[test]
fn shared_directories_are_merged_with_the_aux_directory_and_the_own_unit_wins() {
    let dir = fixture("lang/shared_dirs");
    assert_eq!(check_lang(&dir, &real_compile), Vec::<String>::new());
}

#[test]
fn input_file_is_the_program_stdin() {
    assert_eq!(lang("stdin"), Vec::<String>::new());
}

#[test]
fn runtime_error_programs_get_their_aux_directory_and_input() {
    for dir in ["rt/aux_file", "rt/stdin"] {
        assert_eq!(
            check_rt(&fixture(dir), &fake_compile),
            Vec::<String>::new(),
            "{dir}"
        );
    }
}

#[test]
fn key_script_and_screen_dump_reach_the_program_and_the_comparison() {
    assert_eq!(lang("crt_keys"), Vec::<String>::new());
    assert_eq!(
        check_rt(&fixture("rt/crt_rterror"), &fake_compile),
        Vec::<String>::new()
    );
}

#[test]
fn a_wrong_screen_dump_is_a_mismatch() {
    let f = lang("crt_wrong_dump");
    assert_eq!(f.len(), 1, "{f:?}");
    assert!(
        has(
            &f,
            &["p: run exited 1", "FAIL", "-cursor 2 1", "+cursor 1 1"]
        ),
        "{f:?}"
    );
}
