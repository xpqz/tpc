//! Arrays, records, `with`, aggregate assignment and aggregate parameters. Every `.out` and `.err`
//! (`N LINE`) under tests/fixtures/aggregates and in the corpus lists comes from the real TP 5.0
//! (`tools/tp5.sh`). Strings are not part of this slice; globals are not assumed zero (TP5 does not
//! clear them), so no program prints an uninitialised variable. There is no range checking: only a
//! constant index outside the bounds is an error (76).
mod common;
mod driver;
use driver::*;
use std::path::PathBuf;

fn dir(sub: &str) -> PathBuf {
    root().join("compiler/tests/fixtures/aggregates").join(sub)
}

fn assert_clean(failures: Vec<String>) {
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn stems(sub: &str, ext: &str) -> Vec<String> {
    let mut v: Vec<String> = std::fs::read_dir(dir(sub))
        .unwrap()
        .filter_map(|e| e.unwrap().file_name().into_string().ok())
        .filter_map(|n| n.strip_suffix(ext).map(String::from))
        .collect();
    v.sort();
    v
}

#[test]
fn programs_print_what_tp5_prints() {
    assert!(stems("lang", ".pas").len() >= 12);
    assert_eq!(stems("lang", ".pas"), stems("lang", ".out"));
    for s in stems("lang", ".out") {
        let out = std::fs::read(dir("lang").join(format!("{s}.out"))).unwrap();
        assert!(!out.is_empty(), "{s}: empty expected output");
    }
    assert_clean(check_lang(&dir("lang"), &real_compile));
}

#[test]
fn invalid_programs_report_tp5_number_and_line() {
    assert!(stems("neg", ".pas").len() >= 35);
    assert_eq!(stems("neg", ".pas"), stems("neg", ".err"));
    assert_clean(check_neg(&dir("neg"), &real_compile));
}

/// Phase 3 corpus programs of this slice (tests/lang, tests/abi, tests/neg), sieve and N-queens
/// included. Programs that use strings or Real wait for their own slices.
const LANG: &[&str] = &[
    "aggregate_globals_and_locals_zero_independent",
    "array_1d_basic",
    "array_assignment_copies",
    "array_bubble_sort_and_binary_search",
    "array_const_expression_bounds",
    "array_element_sizes",
    "array_index_by_boolean",
    "array_index_by_char",
    "array_index_by_enum_subrange",
    "array_index_expressions",
    "array_matrix_multiply",
    "array_multi_dim",
    "array_multi_dim_equivalence",
    "array_nonzero_and_negative_bounds",
    "array_of_array_row_assign",
    "array_packed_same_as_unpacked",
    "array_pascal_triangle",
    "array_self_assign_and_swap",
    "array_sieve_of_eratosthenes",
    "array_three_dim",
    "array_types_of_each_element",
    "boolean_array_flags_and_enum_counts",
    "enum_as_array_index_and_const",
    "enum_byte_sized_in_records",
    "nested_proc_with_aggregate_locals",
    "param_aggregate_mixed_value_and_var",
    "param_array_element_and_field_as_var",
    "param_multi_dim_by_value_and_var",
    "param_same_var_passed_twice",
    "param_value_array_copy",
    "param_value_array_of_char_and_boolean",
    "param_var_array_aliases",
    "queens_counts_by_board_size",
    "queens_eight",
    "record_array_field",
    "record_basic",
    "record_linked_via_indexes",
    "record_nested",
    "record_stack_and_queue",
    "record_variant_longint_split",
    "record_variant_nested_cases",
    "record_variant_overlay_bytes",
    "record_variant_with_fixed_part_and_tag",
    "subrange_as_array_index_and_for",
    "with_array_element",
    "with_nested",
    "with_nested_array_in_record",
    "with_shadows_local",
    "with_var_param_and_function_call",
];

const ABI: &[&str] = &[
    "aggregate_param_with_scalar_params_order",
    "aggregate_value_param_exit_early",
    "aggregate_value_param_large_and_recursive",
];

const NEG: &[&str] = &[
    "array_assign_different_bounds",
    "array_assign_different_element_type",
    "array_assign_packed_to_unpacked",
    "array_assign_scalar_to_array",
    "array_compare_equals",
    "array_index_non_array",
    "array_index_type_mismatch_char_on_integer_index",
    "array_index_type_too_large_integer",
    "array_missing_close_bracket",
    "array_multi_dim_wrong_index_type",
    "array_too_many_indexes",
    "array_write_whole",
    "function_result_type_array",
    "function_result_type_record",
    "index_undeclared_array",
    "param_array_inline_type",
    "param_value_array_wrong_type",
    "param_value_record_wrong_type",
    "param_var_aggregate_constant",
    "param_var_array_different_type",
    "record_assign_different_types",
    "record_compare_equals",
    "record_duplicate_field",
    "record_field_of_non_record",
    "record_missing_end",
    "record_unknown_field",
    "record_variant_field_name_clash",
    "record_write_whole",
    "with_field_after_end_of_with",
    "with_missing_do",
    "with_non_record",
    "with_unknown_field_inside",
];

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

#[test]
fn corpus_lang_programs_match_tp5_output() {
    assert_clean(check_lang(
        &subset("lang", LANG, &["pas", "out"]),
        &real_compile,
    ));
}

#[test]
fn corpus_abi_programs_match_tp5_output_and_restore_sp() {
    assert_clean(check_lang(
        &subset("abi", ABI, &["pas", "out"]),
        &real_compile,
    ));
}

#[test]
fn corpus_neg_programs_report_tp5_number_and_line() {
    assert_clean(check_neg(
        &subset("neg", NEG, &["pas", "err"]),
        &real_compile,
    ));
}

#[test]
fn corpus_lists_are_sorted_and_name_existing_programs() {
    for (kind, names, ext) in [
        ("lang", LANG, "out"),
        ("abi", ABI, "out"),
        ("neg", NEG, "err"),
    ] {
        let mut sorted = names.to_vec();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted, names, "{kind} list must be sorted and unique");
        for n in names {
            for e in ["pas", ext] {
                let p = root().join(format!("tests/{kind}/{n}.{e}"));
                assert!(p.exists(), "{}", p.display());
            }
        }
    }
    for n in [
        "array_sieve_of_eratosthenes",
        "queens_eight",
        "queens_counts_by_board_size",
    ] {
        assert!(LANG.contains(&n), "{n} is a Phase 3 exit program");
    }
}

fn compile(src: &str) -> Vec<u8> {
    tp5c::compile(src.as_bytes())
        .unwrap_or_else(|d| panic!("rejected with {} at {}:\n{src}", d.code, d.line))
}

fn has(hay: &[u8], needle: &[u8]) -> bool {
    hay.windows(needle.len()).any(|w| w == needle)
}

/// `memory.copy` (decision 12).
const MEMORY_COPY: [u8; 4] = [0xFC, 0x0A, 0x00, 0x00];

#[test]
fn aggregate_copies_are_memory_copy_instructions() {
    let scalar = "program P;\nvar a, b: Integer;\nbegin\n  a := 1; b := a\nend.";
    assert!(!has(&compile(scalar), &MEMORY_COPY));
    let assign = "program P;\ntype A = array[1..4] of Integer;\nvar a, b: A;\nbegin\n  a[1] := 5; b := a\nend.";
    assert!(has(&compile(assign), &MEMORY_COPY), "array assignment");
    let record = "program P;\ntype R = record x, y: Integer end;\nvar a, b: R;\nbegin\n  a.x := 5; b := a\nend.";
    assert!(has(&compile(record), &MEMORY_COPY), "record assignment");
    let param = "program P;\ntype A = array[1..4] of Integer;\nvar a: A;\nprocedure Q(v: A);\nbegin\n  v[1] := 1\nend;\nbegin\n  Q(a)\nend.";
    assert!(
        has(&compile(param), &MEMORY_COPY),
        "callee copy of a value parameter"
    );
    let by_ref = "program P;\ntype A = array[1..4] of Integer;\nvar a: A;\nprocedure Q(var v: A);\nbegin\n  v[1] := 1\nend;\nbegin\n  Q(a)\nend.";
    assert!(
        !has(&compile(by_ref), &MEMORY_COPY),
        "a var parameter is not copied"
    );
}

#[test]
fn unused_aggregate_routines_leave_the_module_unchanged() {
    let base = "program P;\nbegin\n  WriteLn('live')\nend.";
    let dead = "program P;\ntype A = array[1..4] of Integer; R = record x: A end;\nprocedure Dead(v: A; var r: R);\nvar w: A;\nbegin\n  w := v; r.x := w; r.x[2] := 1\nend;\nbegin\n  WriteLn('live')\nend.";
    assert_eq!(compile(base), compile(dead));
}

#[test]
fn hello_world_stays_small() {
    let hello = compile("program Hello;\nbegin\n  WriteLn('hello')\nend.");
    assert!(hello.len() <= 430, "hello-world is {} bytes", hello.len());
}
