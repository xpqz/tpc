//! Procedural types, the `type` section, indirect calls through the function table, the
//! `{$F}` directives. Corpus programs are copied without PENDING into a scratch directory and
//! checked with the real compiler; expected output and `N LINE` come from the real TP 5.0
//! (`tools/tp5.sh`).
mod common;
mod driver;
use driver::*;
use std::path::PathBuf;
use std::process::Command;

/// tests/abi programs this slice must pass.
const ABI: &[&str] = &[
    "proc_var_argument_conversions",
    "proc_var_as_parameter",
    "proc_var_basic",
    "proc_var_callback_recursion",
    "proc_var_callee_with_frame",
    "proc_var_chosen_by_condition",
    "proc_var_copied_between_variables",
    "proc_var_function_result_in_expression",
    "proc_var_local_variable",
    "proc_var_modifies_globals",
    "proc_var_nested_call_arguments",
    "proc_var_nested_routine_assigns_captured_variable",
    "proc_var_parameter_captured_by_nested_chain",
    "proc_var_parameter_forwarded",
    "proc_var_reassigned",
    "proc_var_recursion_through_variable",
    "proc_var_result_types",
    "proc_var_returning_procedural_value_by_selection",
    "proc_var_var_params",
    "proc_var_wide_signature",
    "proc_var_with_nested_caller",
];

/// tests/lang programs this slice must pass.
const LANG: &[&str] = &[
    "proc_type_alias_of_procedural_type",
    "proc_type_anonymous_variable_type",
    "proc_type_as_parameter",
    "proc_type_basic",
    "proc_type_call_in_conditions",
    "proc_type_function_with_parameters",
    "proc_type_per_routine_far_directive",
    "proc_type_structural_compatibility",
    "proc_type_var_parameter_of_procedural_type",
    "proc_type_without_parameters_function",
    "type_alias_scalars",
];

/// tests/neg programs this slice must pass.
const NEG: &[&str] = &[
    "nested_function_assigned_to_procedural_variable",
    "nested_routine_assigned_to_local_procedural_variable",
    "nested_routine_assigned_to_procedural_variable",
    "nested_routine_assigned_two_levels_deep",
    "nested_routine_passed_as_procedural_argument",
    "procedural_argument_integer_for_procedural_parameter",
    "procedural_argument_signature_mismatch",
    "procedural_argument_with_address_operator",
    "procedural_constant_assigned",
    "procedural_function_variable_as_statement",
    "procedural_function_variable_assigned_to_procedure_variable",
    "procedural_function_variable_needing_arguments_in_expression",
    "procedural_integer_variable_assigned",
    "procedural_parameterless_function_variable_as_statement",
    "procedural_procedure_variable_as_write_argument",
    "procedural_type_function_assigned_to_procedure_variable",
    "procedural_type_function_with_unknown_result_type",
    "procedural_type_parameter_count_mismatch",
    "procedural_type_parameter_type_mismatch",
    "procedural_type_procedure_assigned_to_function_variable",
    "procedural_type_result_type_mismatch",
    "procedural_type_var_versus_value_parameter",
    "procedural_value_with_address_operator",
    "procedural_variable_as_for_control_variable",
    "procedural_variable_assigned_integer",
    "procedural_variable_call_with_argument_for_no_parameters",
    "procedural_variable_call_wrong_argument_count",
    "procedural_variable_call_wrong_argument_type",
    "procedural_variable_comparison",
    "procedural_variable_function_result_to_boolean",
    "procedural_variable_var_parameter_constant_argument",
    "procedural_variable_var_parameter_missing_argument",
    "type_alias_var_parameter_mismatch",
    "type_declared_after_use",
    "type_duplicate_name",
    "type_missing_equals",
    "type_missing_name",
    "type_missing_semicolon",
    "type_procedural_function_without_result_type",
    "type_procedural_parameter_unknown_type",
    "type_procedural_parameter_without_type",
    "type_refers_to_itself",
];

/// Programs the compiler must accept.
const ACCEPTED: &[(&str, &str)] = &[
    (
        "type_section_scalar_alias",
        "program Prog;\ntype Num = Integer;\nvar n: Num;\nbegin\n  n := 1\nend.",
    ),
    (
        "type_section_procedural",
        "program Prog;\ntype P = procedure; F = function(a: Integer; var b: Byte): Boolean;\nbegin\nend.",
    ),
    (
        "anonymous_procedural_variable_types",
        "program Prog;\nvar p: procedure; f: function(x: Integer): Integer;\nbegin\nend.",
    ),
    (
        "far_directives_anywhere",
        "{$F+}\nprogram Prog;\ntype T = procedure;\n{$F-}\nvar p: T;\n{$F+}\nprocedure A;\nbegin\n{$F-}\nend;\nbegin\n  p := A;\n  p\nend.",
    ),
    (
        "far_directive_lower_case",
        "program Prog;\n{$f+}\nbegin\nend.",
    ),
    // TP5 needs {$F+} for this (error 143); the compiler does not (docs/deviations.md D11).
    (
        "routine_assigned_without_far_directive",
        "program Prog;\ntype T = procedure;\nvar p: T;\nprocedure A;\nbegin\nend;\nbegin\n  p := A;\n  p\nend.",
    ),
    (
        "procedural_value_forms",
        "program Prog;\n{$F+}\ntype T = procedure(var x: Integer);\nvar p, q: T; n: Integer;\nprocedure A(var x: Integer);\nbegin\n  x := 1\nend;\nprocedure U(r: T; var s: T);\nbegin\n  s := r;\n  r(n)\nend;\nbegin\n  p := A; q := p; U(q, p); p(n)\nend.",
    ),
];

/// TP accepts these; the compiler reports 200 (decision 6) on the given line.
const UNSUPPORTED: &[(&str, &str, u32)] =
    &[("other_directive", "program Prog;\n{$B+}\nbegin\nend.", 2)];

fn tmp_dir() -> PathBuf {
    let d = root().join("tmp/procedural");
    std::fs::create_dir_all(&d).unwrap();
    d
}

/// Copies `names` with the given extensions into a fresh directory (no PENDING file).
fn subset(kind: &str, names: &[&str], exts: &[&str]) -> PathBuf {
    let dir = scratch("x").parent().unwrap().to_path_buf();
    for n in names {
        for e in exts {
            let f = format!("{n}.{e}");
            std::fs::copy(root().join("tests").join(kind).join(&f), dir.join(&f)).unwrap();
        }
    }
    dir
}

fn report(what: &str, total: usize, fails: Vec<String>) {
    assert!(
        fails.is_empty(),
        "{what}: {} of {total}:\n{}",
        fails.len(),
        fails.join("\n")
    );
}

#[test]
fn target_abi_programs_match_tp5_output() {
    let fails = check_lang(&subset("abi", ABI, &["pas", "out"]), &real_compile);
    report("abi", ABI.len(), fails);
}

#[test]
fn target_lang_programs_match_tp5_output() {
    let fails = check_lang(&subset("lang", LANG, &["pas", "out"]), &real_compile);
    report("lang", LANG.len(), fails);
}

#[test]
fn target_neg_programs_report_tp5_number_and_line() {
    let fails = check_neg(&subset("neg", NEG, &["pas", "err"]), &real_compile);
    report("neg", NEG.len(), fails);
}

#[test]
fn target_lists_name_existing_corpus_programs() {
    for (kind, names, exts) in [
        ("abi", ABI, "pas out"),
        ("lang", LANG, "pas out"),
        ("neg", NEG, "pas err"),
    ] {
        let mut sorted = names.to_vec();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted, names, "{kind} list must be sorted and unique");
        for n in names {
            for e in exts.split(' ') {
                let p = root().join(format!("tests/{kind}/{n}.{e}"));
                assert!(p.exists(), "{}", p.display());
            }
        }
    }
}

#[test]
fn accepted_forms_compile() {
    let mut fails = Vec::new();
    for &(name, src) in ACCEPTED {
        if let Err(d) = tp5c::compile([src, "\n"].concat().as_bytes()) {
            fails.push(format!("{name}: rejected with {} at {}", d.code, d.line));
        }
    }
    assert!(fails.is_empty(), "{}", fails.join("\n"));
}

#[test]
fn unsupported_forms_report_error_200() {
    let mut fails = Vec::new();
    for &(name, src, line) in UNSUPPORTED {
        match tp5c::compile([src, "\n"].concat().as_bytes()) {
            Err(d) if (d.code, d.line) == (200, line) => {}
            Err(d) => fails.push(format!("{name}: got {} at line {}", d.code, d.line)),
            Ok(_) => fails.push(format!("{name}: accepted")),
        }
    }
    assert!(fails.is_empty(), "{}", fails.join("\n"));
}

/// Number of entries in the type section of a module.
fn type_count(wasm: &[u8]) -> u32 {
    let mut at = 8;
    while at < wasm.len() {
        let (id, mut size, mut shift, mut i) = (wasm[at], 0usize, 0, at + 1);
        loop {
            let b = wasm[i];
            size |= ((b & 0x7F) as usize) << shift;
            shift += 7;
            i += 1;
            if b & 0x80 == 0 {
                break;
            }
        }
        if id == 1 {
            return wasm[i] as u32;
        }
        at = i + size;
    }
    panic!("no type section");
}

fn compile(src: &str) -> Vec<u8> {
    tp5c::compile(src.as_bytes())
        .unwrap_or_else(|d| panic!("rejected with {} at {}", d.code, d.line))
}

/// One WASM function type per signature (docs/abi.md): a second routine, a second variable and
/// a second named type with the same signature add no type; another signature adds one.
#[test]
fn one_wasm_type_per_signature() {
    let one = "program Prog;\ntype TA = function(x: Integer): Integer;\nvar f: TA;\nfunction A(x: Integer): Integer;\nbegin\n  A := x\nend;\nbegin\n  f := A;\n  WriteLn(f(1))\nend.\n";
    let same = "program Prog;\ntype TA = function(x: Integer): Integer;\ntype TB = function(y: Integer): Integer;\nvar f: TA; g: TB; h: TA;\nfunction A(x: Integer): Integer;\nbegin\n  A := x\nend;\nfunction B(z: Integer): Integer;\nbegin\n  B := z + 1\nend;\nbegin\n  f := A; g := B; h := f;\n  WriteLn(f(1), g(2), h(3))\nend.\n";
    let other = "program Prog;\ntype TA = function(x: Integer): Integer;\ntype TC = function(x, y, z, w: Integer): Integer;\nvar f: TA; cb: TC;\nfunction A(x: Integer): Integer;\nbegin\n  A := x\nend;\nfunction C(x, y, z, w: Integer): Integer;\nbegin\n  C := x + y + z + w\nend;\nbegin\n  f := A; cb := C;\n  WriteLn(f(1), cb(2, 3, 4, 5))\nend.\n";
    let (one, same, other) = (
        type_count(&compile(one)),
        type_count(&compile(same)),
        type_count(&compile(other)),
    );
    assert_eq!(one, same, "same signature, same WASM type");
    assert_eq!(
        other,
        one + 1,
        "a signature with another WASM shape adds exactly one type"
    );
}

/// Table index 0 is a trap stub: an unassigned procedural variable fails cleanly.
#[test]
fn calling_an_unassigned_procedural_variable_traps() {
    let src = "program Prog;\ntype T = procedure;\nvar p: T;\nbegin\n  WriteLn('before');\n  p;\n  WriteLn('not reached')\nend.\n";
    let path = tmp_dir().join("unassigned.wasm");
    std::fs::write(&path, compile(src)).unwrap();
    let o = Command::new("node")
        .arg(root().join("tests/run.mjs"))
        .arg(&path)
        .output()
        .expect("node must be installed");
    assert_eq!(String::from_utf8_lossy(&o.stdout), "before\n");
    let err = String::from_utf8_lossy(&o.stderr);
    assert!(err.contains("run.mjs: trap: unreachable"), "{err}");
    assert_eq!(o.status.code(), Some(2));
}

/// Slow (DOSBox-X): `cargo test -- --ignored`. TP5 compiles the accepted forms, except that it
/// needs `{$F+}` for the deviation entry (error 143 on the assignment line).
#[test]
#[ignore]
fn oracle_agrees() {
    let tp5 = |name: &str, src: &str| {
        let p = tmp_dir().join([name, ".pas"].concat());
        std::fs::write(&p, [src, "\n"].concat()).unwrap();
        let o = Command::new(root().join("tools/tp5.sh"))
            .arg(&p)
            .output()
            .unwrap();
        String::from_utf8_lossy(&o.stdout).into_owned()
    };
    let mut fails = Vec::new();
    for &(name, src) in ACCEPTED {
        let got = tp5(name, src);
        let want_error = name == "routine_assigned_without_far_directive";
        if want_error != got.starts_with("P.PAS(8): Error 143") || (!want_error && !got.is_empty())
        {
            fails.push(format!("{name}: tp5 printed {got:?}"));
        }
    }
    let got = tp5("range", UNSUPPORTED[0].1);
    if !got.is_empty() {
        fails.push(format!("range directive: tp5 printed {got:?}"));
    }
    assert!(fails.is_empty(), "{}", fails.join("\n"));
}
