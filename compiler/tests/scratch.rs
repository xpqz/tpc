//! Scratch directories: every call to `scratch` yields a directory that no other test, thread
//! or earlier process has used. Directory names are `PID-COUNTER`, and process ids are reused,
//! so a directory left behind by an earlier process can carry the name a later one picks.
//! Files left in it (a stray `.pas`, `.out`, `.wasm`) would then be swept up by a corpus
//! driver that lists the directory, which made `corpus_abi_programs_match_tp5_output` fail
//! once in a full `make check`.
mod common;
mod driver;
use driver::*;
use std::collections::BTreeSet;

/// Plants a directory with a stale program for every name this process could pick next.
fn plant_leftovers(kind: &str) -> Vec<std::path::PathBuf> {
    let base = root().join("tmp").join(kind);
    std::fs::create_dir_all(&base).unwrap();
    (0..5000)
        .map(|n| base.join(format!("{}-{n}", std::process::id())))
        .filter(|d| std::fs::create_dir(d).is_ok())
        .inspect(|d| std::fs::write(d.join("stale.pas"), "begin end.").unwrap())
        .collect()
}

#[test]
fn scratch_never_reuses_a_directory_left_by_an_earlier_process() {
    let leftovers = plant_leftovers("driver");
    assert!(!leftovers.is_empty(), "no leftover could be planted");
    let dir = scratch("x").parent().unwrap().to_path_buf();
    let entries: Vec<_> = std::fs::read_dir(&dir).unwrap().collect();
    for d in &leftovers {
        std::fs::remove_dir_all(d).unwrap();
    }
    assert!(
        entries.is_empty(),
        "{} was handed out with {} file(s) already in it",
        dir.display(),
        entries.len()
    );
}

#[test]
fn concurrent_scratch_calls_get_distinct_directories() {
    let threads: Vec<_> = (0..16)
        .map(|_| std::thread::spawn(|| (0..50).map(|_| scratch("x")).collect::<Vec<_>>()))
        .collect();
    let all: Vec<_> = threads
        .into_iter()
        .flat_map(|t| t.join().unwrap())
        .collect();
    let distinct: BTreeSet<_> = all.iter().collect();
    assert_eq!(distinct.len(), all.len());
}
