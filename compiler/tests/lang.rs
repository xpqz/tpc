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
    let fails = check_lang(&root().join("tests/lang"), &real_compile);
    assert!(fails.is_empty(), "{}", fails.join("\n"));
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
