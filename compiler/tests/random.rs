//! Random, Randomize and RandSeed. The sequences are pinned by the corpus programs
//! `random_*` and `swag_random_seeded` (expected output from the real TP 5.0). Randomize
//! seeds from the clock, which the oracle cannot fix, so it is tested here with the runner's
//! fixed clock (`TP5_NOW_MS`, see dos_unit.rs): the seed is the time of day in hundredths of
//! a second.
mod common;
use common::OutputGuarded as _;
mod driver;
use common::node;
use driver::*;
use std::process::Command;

const SHOW_SEED: &str = "program Seeded;\nbegin\n  Randomize;\n  WriteLn(RandSeed);\n  \
    WriteLn(Random(1000));\nend.\n";

fn run_with_clock(wasm: &std::path::Path, now_ms: u64) -> Vec<String> {
    let o = Command::new("node")
        .arg(root().join("tests/run.mjs"))
        .arg(wasm)
        .env("TP5_NOW_MS", now_ms.to_string())
        .output_guarded()
        .expect("node must be installed");
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn compile(source: &str, name: &str) -> std::path::PathBuf {
    let src = scratch(&format!("{name}.pas"));
    std::fs::write(&src, source).unwrap();
    let wasm = src.with_extension("wasm");
    let r = real_compile(&src, &wasm);
    assert_eq!((r.code, r.stderr.as_str()), (0, ""));
    wasm
}

#[test]
fn randomize_seeds_from_the_clock_in_hundredths_of_a_second() {
    let wasm = compile(SHOW_SEED, "seeded");
    // 12:34:56.78 local time on 2000-02-29.
    let out = run_with_clock(&wasm, 951827696780);
    assert_eq!(out.len(), 2, "{out:?}");
    assert_eq!(out[0], ((12 * 3600 + 34 * 60 + 56) * 100 + 78).to_string());
    // The first draw follows from that seed: ((seed * 134775813 + 1) >> 16) mod 1000.
    let seed: u64 = out[0].parse().unwrap();
    let next = (seed * 134775813 + 1) & 0xffff_ffff;
    assert_eq!(out[1], ((next >> 16) % 1000).to_string());
}

#[test]
fn the_same_clock_gives_the_same_seed_and_another_clock_another() {
    let wasm = compile(SHOW_SEED, "seeded");
    let a = run_with_clock(&wasm, 951827696780);
    let b = run_with_clock(&wasm, 951827696780);
    let c = run_with_clock(&wasm, 951827696790);
    assert_eq!(a, b);
    assert_ne!(a[0], c[0]);
    assert!(a[0].parse::<i64>().unwrap() > 0);
}

#[test]
fn only_randomize_needs_the_time_import() {
    let imports = |src: &str| {
        let wasm = tp5c::compile(src.as_bytes()).unwrap();
        let js = "for (const i of WebAssembly.Module.imports(new WebAssembly.Module(b))) \
                  console.log(i.module + '.' + i.name);";
        node(js, &wasm)
    };
    let with = imports(SHOW_SEED);
    assert!(with.lines().any(|l| l == "host.time"), "{with}");
    let without =
        imports("program P;\nbegin\n  RandSeed := 3;\n  WriteLn(Random(9), Random);\nend.\n");
    assert!(!without.lines().any(|l| l == "host.time"), "{without}");
}

#[test]
fn corpus_programs_leave_pending() {
    for (kind, name) in [
        ("lang", "swag_random_seeded"),
        ("lang", "random_sequences"),
        ("lang", "random_real"),
    ] {
        let pending =
            std::fs::read_to_string(root().join(format!("tests/{kind}/PENDING"))).unwrap();
        assert!(!pending.lines().any(|l| l.trim() == name), "{kind}/{name}");
    }
}
