mod common;
mod driver;
use driver::*;

fn neg(name: &str) -> Vec<String> {
    check_neg(&fixture(&format!("neg/{name}")), &fake_compile)
}

#[test]
fn corpus_has_no_unexpected_results() {
    let fails = check_neg(&root().join("tests/neg"), &real_compile);
    assert!(fails.is_empty(), "{}", fails.join("\n"));
}

#[test]
fn matching_number_and_line_passes() {
    assert_eq!(neg("pass"), Vec::<String>::new());
}

#[test]
fn wrong_number_is_reported() {
    assert_eq!(
        neg("wrong_number"),
        ["wrong_number: exit 1, got `5 3`, want `26 3`"]
    );
}

#[test]
fn wrong_line_is_reported() {
    assert_eq!(
        neg("wrong_line"),
        ["wrong_line: exit 1, got `5 3`, want `5 4`"]
    );
}

#[test]
fn program_that_compiles_is_a_failure() {
    assert_eq!(neg("compiles"), ["compiles: compiled without error"]);
}

#[test]
fn malformed_diagnostic_is_a_failure() {
    let f = neg("malformed");
    assert_eq!(f.len(), 1, "{f:?}");
    assert!(f[0].starts_with("malformed: bad diagnostic"), "{f:?}");
}

#[test]
fn pending_program_that_fails_is_accepted() {
    assert_eq!(neg("pending_fails"), Vec::<String>::new());
}

#[test]
fn pending_program_that_the_compiler_accepts_is_still_pending() {
    assert_eq!(neg("pending_compiles"), Vec::<String>::new());
}

#[test]
fn pending_program_with_correct_diagnostic_is_a_failure() {
    assert_eq!(
        neg("pending_passes"),
        ["pending_passes: passes but is listed in PENDING"]
    );
}

#[test]
fn every_failure_is_reported_in_one_run() {
    let f = neg("several");
    assert_eq!(f.len(), 3, "{f:?}");
}

#[test]
fn real_compiler_reports_its_diagnostic() {
    assert_eq!(
        check_neg(&fixture("neg/real_200"), &real_compile),
        Vec::<String>::new()
    );
}

#[test]
fn real_compiler_diagnostic_mismatch_is_reported() {
    let f = check_neg(&fixture("neg/real_wrong"), &real_compile);
    assert_eq!(f, ["real_wrong: exit 1, got `3 1`, want `5 1`"]);
}
