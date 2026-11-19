//! The hello-world size limit lives in `common::HELLO_SIZE_LIMIT` and nowhere else.
mod common;
use common::*;
use std::path::Path;

#[test]
fn limit_is_the_two_kilobyte_target_of_docs_size() {
    assert_eq!(HELLO_SIZE_LIMIT, 2048);
}

#[test]
fn guard_accepts_the_limit_and_rejects_one_byte_more() {
    assert_hello_fits(&vec![0; HELLO_SIZE_LIMIT]);
    let over = std::panic::catch_unwind(|| assert_hello_fits(&vec![0; HELLO_SIZE_LIMIT + 1]));
    assert!(over.is_err());
}

#[test]
fn no_test_compares_the_hello_world_length_with_its_own_number() {
    let mut offenders = Vec::new();
    for e in std::fs::read_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests")).unwrap() {
        let p = e.unwrap().path();
        if !p.extension().is_some_and(|x| x == "rs") || p.ends_with("size_guard.rs") {
            continue;
        }
        let text = std::fs::read_to_string(&p).unwrap();
        for (n, line) in text.lines().enumerate() {
            if line.contains("hello.len()") {
                offenders.push(format!("{}:{}: {}", p.display(), n + 1, line.trim()));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "use assert_hello_fits:\n{}",
        offenders.join("\n")
    );
}
