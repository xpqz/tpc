//! Units: `uses`, `unit`/`interface`/`implementation`, qualified names `U.X`, unit files found in
//! the registered file table (decision 27), and diagnostics that name their file (decision 28).
//! Every `.out` and `.err` (`N LINE`, `N LINE FILE` for an error in a unit) comes from the real
//! TP 5.0 (`tools/tp5.sh`). Where units are searched is a decision of this compiler, not TP's:
//! the files of the input's directory, then the auxiliary directory given with `--dir`, which
//! wins on a name clash; those tests state their expectations in the test.
//!
//! TP rules the fixtures pin that the plan does not state: a unit name may appear once per
//! unit across both `uses` clauses and is not the unit's own name or `System` (error 4); a cycle
//! is error 68 when a unit is used while its interface is still being read, whichever part
//! (interface or implementation) does the using, so the same two units compile or fail
//! depending on which is used first; `Exit` in a unit's `begin ... end.` leaves only that unit's
//! initialisation; a unit without `interface` is error 55.
//!
//! Measured but not pinned: a variable named like a used unit is error 4 (unit names are
//! identifiers of the `uses` part's scope).
mod common;
use common::OutputGuarded as _;
mod driver;
use driver::*;
use std::path::{Path, PathBuf};
use std::process::Command;

fn fx(sub: &str) -> PathBuf {
    root().join("compiler/tests/fixtures/units").join(sub)
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
    assert_eq!(stems(&fx("lang"), ".pas").len(), 4);
    assert_eq!(stems(&fx("lang"), ".pas"), stems(&fx("lang"), ".out"));
    for s in stems(&fx("lang"), ".out") {
        let out = std::fs::read(fx("lang").join(format!("{s}.out"))).unwrap();
        assert!(!out.is_empty(), "{s}: empty expected output");
    }
    assert_clean(check_lang(&fx("lang"), &real_compile));
}

#[test]
fn own_invalid_programs_report_tp5_number_line_and_file() {
    assert_eq!(stems(&fx("neg"), ".pas").len(), 8);
    assert_eq!(stems(&fx("neg"), ".pas"), stems(&fx("neg"), ".err"));
    assert_clean(check_neg(&fx("neg"), &real_compile));
}

// ---- The corpus programs this slice must make pass ----

/// Excluded because they need text files, includes, conditional compilation or Dos:
/// lang `ifdef_define_in_program_is_not_visible_in_unit`,
/// `ifdef_define_in_unit_is_not_visible_in_program`, `include_inside_a_unit`,
/// `ioresult_unit_with_checking_off_reports_codes`, `dos_*`; abi
/// `unit_text_file_variable_owned_by_unit`; rt `file_error_inside_unit_routine`,
/// `file_io_checking_of_unit_does_not_leak_into_program`.
const LANG: &[&str] = &[
    "unit_circular_uses_in_implementation",
    "unit_constants_size_arrays_of_other_unit",
    "unit_cross_unit_procedural_types",
    "unit_implementation_header_may_omit_parameters",
    "unit_initialization_order_follows_dependencies",
    "unit_initialization_order_follows_uses_list",
    "unit_initialization_runs_before_program",
    "unit_initialization_sets_variables_used_by_other_unit",
    "unit_interface_const_type_var_proc",
    "unit_private_names_do_not_clash_with_program",
    "unit_qualified_name_picks_among_clashes",
    "unit_qualified_names",
    "unit_shared_dependency_initialized_once",
    "unit_swapped_uses_order_changes_unqualified_name",
    "unit_typed_constants_and_variables_keep_state",
    "unit_types_enumerations_and_sets_exported",
    "unit_variable_shared_between_units",
    "unit_without_initialization_part",
];

const ABI: &[&str] = &[
    "unit_callback_into_program_through_unit_recursion",
    "unit_procedural_type_in_record_across_units",
    "unit_procedural_variable_holds_routine_of_another_unit",
    "unit_real_and_set_parameters_across_units",
    "unit_records_and_arrays_pass_across_units",
    "unit_routines_with_locals_recurse_across_units",
    "unit_static_data_of_several_units_does_not_overlap",
    "unit_strings_pass_across_units",
    "unit_variables_passed_by_reference_to_other_unit",
];

const RT: &[&str] = &["unit_initialization_runtime_error"];

const NEG: &[&str] = &[
    "error_inside_a_unit_names_the_unit_file",
    "unit_circular_reference_in_interfaces",
    "unit_cross_unit_procedural_type_mismatch",
    "unit_identifier_declared_in_interface_and_implementation",
    "unit_implementation_header_differs_from_interface",
    "unit_initialization_keyword",
    "unit_interface_routine_has_no_body",
    "unit_name_differs_from_file_name",
    "unit_names_of_used_unit_are_not_reexported",
    "unit_not_found",
    "unit_private_name_used_by_program",
    "unit_program_used_as_unit",
    "unit_qualified_name_not_in_unit",
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

/// Copies `names` of a corpus directory with their expectation, auxiliary directory and
/// console input into a fresh directory (no PENDING file).
fn subset(kind: &str, names: &[&str], expected: &str) -> PathBuf {
    let from = root().join("tests").join(kind);
    let dir = scratch("x").parent().unwrap().to_path_buf();
    for n in names {
        for e in ["pas", expected, "in"] {
            let f = format!("{n}.{e}");
            if from.join(&f).exists() {
                std::fs::copy(from.join(&f), dir.join(&f)).unwrap();
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
fn corpus_runtime_error_programs_match_tp5() {
    assert_clean(check_rt(&subset("rt", RT, "out"), &real_compile));
}

#[test]
fn corpus_neg_programs_report_tp5_number_line_and_file() {
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
                root().join(format!("tests/{kind}/{n}")).is_dir() || *n == "unit_not_found",
                "{kind}/{n} has no auxiliary directory"
            );
            assert!(
                !pending.lines().any(|l| l.trim() == *n),
                "{kind}/{n} is still in PENDING"
            );
        }
    }
    assert_eq!((LANG.len(), ABI.len(), RT.len(), NEG.len()), (18, 9, 1, 13));
}

// ---- Where units are found (the CLI registers the files, decision 27) ----

/// A fresh scratch directory.
fn work() -> PathBuf {
    scratch("x").parent().unwrap().to_path_buf()
}

fn tp5c(cwd: &Path, args: &[&str]) -> (i32, String) {
    let o = Command::new(env!("CARGO_BIN_EXE_tp5c"))
        .current_dir(cwd)
        .args(args)
        .output_guarded()
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
        .output_guarded()
        .expect("node must be installed");
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap()
}

const SHOW_K: &str = "program P;\nuses Vault;\nbegin\n  WriteLn(Vault.K);\nend.\n";

fn unit_with(k: i32) -> String {
    format!("unit Vault;\ninterface\nconst K = {k};\nimplementation\nend.\n")
}

#[test]
fn units_beside_the_input_are_found_by_every_fixture() {
    // The lang fixtures, copied with their unit files next to the program: same output.
    for s in stems(&fx("lang"), ".pas") {
        let dir = work();
        copy_dir(&fx("lang").join(&s), &dir);
        std::fs::copy(fx("lang").join(format!("{s}.pas")), dir.join("p.pas")).unwrap();
        let (code, err) = tp5c(&dir, &["p.pas", "-o", "p.wasm"]);
        assert_eq!((code, err.as_str()), (0, ""), "{s}");
        let want = std::fs::read_to_string(fx("lang").join(format!("{s}.out"))).unwrap();
        assert_eq!(run(&dir, "p.wasm"), want, "{s}");
    }
}

#[test]
fn unit_file_names_ignore_case() {
    let dir = work();
    std::fs::write(dir.join("p.pas"), SHOW_K.replace("Vault", "VAULT")).unwrap();
    std::fs::write(dir.join("vault.pas"), unit_with(5)).unwrap();
    assert_eq!(tp5c(&dir, &["p.pas", "-o", "p.wasm"]), (0, String::new()));
    assert_eq!(run(&dir, "p.wasm"), "5\n");
}

#[test]
fn auxiliary_directory_is_searched_and_wins_over_the_input_directory() {
    let dir = work();
    std::fs::write(dir.join("p.pas"), SHOW_K).unwrap();
    std::fs::create_dir(dir.join("aux")).unwrap();
    std::fs::write(dir.join("aux/VAULT.PAS"), unit_with(2)).unwrap();
    let args = ["p.pas", "-o", "p.wasm", "--dir", "aux"];
    assert_eq!(tp5c(&dir, &args), (0, String::new()));
    assert_eq!(run(&dir, "p.wasm"), "2\n");
    std::fs::write(dir.join("vault.pas"), unit_with(1)).unwrap();
    assert_eq!(tp5c(&dir, &args), (0, String::new()));
    assert_eq!(run(&dir, "p.wasm"), "2\n");
    // Without the option only the input directory is searched.
    assert_eq!(tp5c(&dir, &["p.pas", "-o", "q.wasm"]), (0, String::new()));
    assert_eq!(run(&dir, "q.wasm"), "1\n");
}

#[test]
fn a_missing_auxiliary_directory_is_a_usage_level_failure() {
    let dir = work();
    std::fs::write(dir.join("p.pas"), "begin end.\n").unwrap();
    let (code, err) = tp5c(&dir, &["p.pas", "-o", "p.wasm", "--dir", "nowhere"]);
    assert_eq!(code, 2);
    assert!(err.starts_with("tp5c: cannot read nowhere"), "{err}");
}

#[test]
fn only_the_units_a_program_uses_are_read() {
    let dir = work();
    std::fs::write(dir.join("p.pas"), SHOW_K).unwrap();
    std::fs::write(dir.join("VAULT.PAS"), unit_with(3)).unwrap();
    std::fs::write(dir.join("JUNK.PAS"), "unit Junk; this is not pascal ((\n").unwrap();
    assert_eq!(tp5c(&dir, &["p.pas", "-o", "p.wasm"]), (0, String::new()));
    assert_eq!(run(&dir, "p.wasm"), "3\n");
}

#[test]
fn a_unit_error_prints_the_unit_file_name_and_a_program_error_the_input_name() {
    let dir = work();
    std::fs::write(dir.join("p.pas"), SHOW_K).unwrap();
    let broken = "unit Vault;\ninterface\nconst K = 1;\nimplementation\nvar x: Integer;\nbegin\n  Nope := 1;\nend.\n";
    std::fs::write(dir.join("VAULT.PAS"), broken).unwrap();
    let (code, err) = tp5c(&dir, &["p.pas", "-o", "p.wasm"]);
    assert_eq!(code, 1);
    assert!(err.starts_with("VAULT.PAS(7,3): Error 3: "), "{err}");
    std::fs::write(dir.join("VAULT.PAS"), unit_with(1)).unwrap();
    std::fs::write(dir.join("p.pas"), SHOW_K.replace("K)", "Nope)")).unwrap();
    let (code, err) = tp5c(&dir, &["p.pas", "-o", "p.wasm"]);
    assert_eq!(code, 1);
    assert!(err.starts_with("p.pas(4,"), "{err}");
}

// ---- Reachability: unit routines are eliminated unless used, initialisation is a root ----

#[test]
fn unused_unit_routines_are_not_in_the_module() {
    let build = |with_unused: bool| {
        let dir = work();
        let extra = if with_unused {
            "procedure Big;\nvar i: Integer;\nbegin\n  for i := 1 to 3 do WriteLn(i, 'unused routine text');\nend;\n"
        } else {
            ""
        };
        let heading = if with_unused { "procedure Big;\n" } else { "" };
        let unit =
            format!("unit Vault;\ninterface\nconst K = 7;\n{heading}implementation\n{extra}end.\n");
        std::fs::write(dir.join("VAULT.PAS"), unit).unwrap();
        std::fs::write(dir.join("p.pas"), SHOW_K).unwrap();
        assert_eq!(tp5c(&dir, &["p.pas", "-o", "p.wasm"]), (0, String::new()));
        std::fs::read(dir.join("p.wasm")).unwrap()
    };
    let lean = build(false);
    let with_unused = build(true);
    assert!(lean.len() > 8);
    assert_eq!(lean, with_unused);
}

#[test]
fn unit_initialisation_is_kept_though_nothing_else_of_the_unit_is_used() {
    let dir = work();
    std::fs::write(dir.join("p.pas"), "program P;\nuses Boot;\nbegin\nend.\n").unwrap();
    let boot = "unit Boot;\ninterface\nimplementation\nbegin\n  WriteLn('booted');\nend.\n";
    std::fs::write(dir.join("BOOT.PAS"), boot).unwrap();
    assert_eq!(tp5c(&dir, &["p.pas", "-o", "p.wasm"]), (0, String::new()));
    assert_eq!(run(&dir, "p.wasm"), "booted\n");
}

// ---- The wasm API: add_file, err_file (decision 27 and 28) ----

#[test]
fn wasm_api_registers_files_and_reports_the_file_of_an_error() {
    let target = Command::new("sh")
        .arg(root().join("scripts/cargo-target.sh"))
        .output_guarded()
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
    let neg = root().join("tests/neg");
    let out = Command::new("node")
        .arg(root().join("tests/wasm_units.mjs"))
        .arg(&wasm)
        .arg(fx("lang/unit_name_qualifies_its_own_names.pas"))
        .arg(neg.join("error_inside_a_unit_names_the_unit_file.pas"))
        .arg(fx("neg/interface_cycle_of_three_units.pas"))
        .arg(neg.join("unit_not_found.pas"))
        .output_guarded()
        .expect("node must be installed");
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}
