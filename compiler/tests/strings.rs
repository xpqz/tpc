//! Strings (decision 13): `String[n]`, `String`, indexing, `s[0]`, comparison, `+`, `Length`, `Copy`,
//! `Pos`, `Delete`, `Insert`, `Concat`, `Char` to `String` coercion, `Write` with widths and string
//! function results through the hidden result pointer. Every `.out` and `.err` (`N LINE`) under
//! tests/fixtures/strings and in the corpus lists comes from the real TP 5.0 (`tools/tp5.sh`, run with
//! `LC_ALL=C`). No test prints a byte above 127, and no program reads an uninitialised variable.
mod common;
mod driver;
use driver::*;
use std::path::PathBuf;

fn dir(sub: &str) -> PathBuf {
    root().join("compiler/tests/fixtures/strings").join(sub)
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
    assert!(stems("lang", ".pas").len() >= 13);
    assert_eq!(stems("lang", ".pas"), stems("lang", ".out"));
    for s in stems("lang", ".out") {
        let out = std::fs::read(dir("lang").join(format!("{s}.out"))).unwrap();
        assert!(!out.is_empty(), "{s}: empty expected output");
        assert!(
            out.iter().all(|&b| (b != 0) && b < 128),
            "{s}: byte outside 1..127"
        );
    }
    assert_clean(check_lang(&dir("lang"), &real_compile));
}

#[test]
fn invalid_programs_report_tp5_number_and_line() {
    assert!(stems("neg", ".pas").len() >= 25);
    assert_eq!(stems("neg", ".pas"), stems("neg", ".err"));
    assert_clean(check_neg(&dir("neg"), &real_compile));
}

/// Phase 3 corpus programs left for this slice (tests/lang, tests/abi, tests/neg): every string
/// program, the aggregates that hold strings, and the hidden-result-pointer ABI programs. The four
/// Real programs wait for the Real slice.
const LANG: &[&str] = &[
    "array_char_histogram",
    "param_value_record_copy",
    "record_aggregate_assignment",
    "record_array_of_records",
    "record_field_sizes_packed_layout",
    "string_255_length_boundary",
    "string_array_of_strings",
    "string_as_loop_state_machine",
    "string_assign_and_length",
    "string_assign_truncates_to_string_n",
    "string_byte_values_above_127",
    "string_caesar_cipher",
    "string_char_comparisons_with_strings",
    "string_char_function_and_string_loop",
    "string_char_plus_char_is_string",
    "string_char_to_string_coercion",
    "string_comparisons_high_bytes",
    "string_comparisons_lexicographic",
    "string_comparisons_mixed_types",
    "string_concat_function",
    "string_concat_overflow_truncates",
    "string_concat_plus",
    "string_const_strings",
    "string_copy",
    "string_copy_edge_indexes",
    "string_copy_negative_arguments",
    "string_delete",
    "string_delete_edge_indexes",
    "string_delete_negative_arguments",
    "string_function_result",
    "string_function_result_reassigned",
    "string_function_result_truncates",
    "string_in_record",
    "string_indexing",
    "string_indexing_writes_in_place",
    "string_insert",
    "string_insert_char_argument",
    "string_insert_truncates_to_string_n",
    "string_insert_zero_and_negative_index",
    "string_length_byte_s0",
    "string_n_sizes_and_nested_types",
    "string_number_to_text_manual",
    "string_palindrome_and_reverse",
    "string_param_value_copy",
    "string_pos",
    "string_pos_scan_loop",
    "string_pos_with_char_argument",
    "string_replace_all",
    "string_sort_array_of_strings",
    "string_string_to_char_by_index",
    "string_var_param_same_type",
    "string_var_param_string_n_exact",
    "string_word_count_and_split",
    "string_write_control_and_empty",
    "string_write_width_expression",
    "string_write_with_width",
    "with_basic",
];

const ABI: &[&str] = &[
    "aggregate_sp_restored_after_many_calls",
    "frame_deep_recursion_with_string_locals",
    "frame_function_returns_string_with_array_local",
    "frame_with_aggregate_and_string_locals",
    "string_function_nested_and_var_args",
    "string_function_recursive",
    "string_function_result_as_argument",
    "string_function_result_hidden_pointer",
    "string_function_result_in_condition_and_index",
    "string_function_result_string_n",
    "string_value_params_many_and_mixed",
    "with_inside_function_and_nested",
];

const NEG: &[&str] = &[
    "array_index_string",
    "enum_succ_of_string",
    "string_assign_integer",
    "string_assign_to_char",
    "string_case_selector",
    "string_compare_with_integer",
    "string_concat_non_string",
    "string_copy_too_few_arguments",
    "string_copy_wrong_argument_type",
    "string_delete_index_not_integer",
    "string_delete_literal_not_variable",
    "string_for_control_variable",
    "string_index_with_string",
    "string_insert_target_not_variable",
    "string_insert_wrong_order",
    "string_length_no_argument",
    "string_length_of_integer",
    "string_minus_operator",
    "string_ord_of_string",
    "string_param_type_without_identifier",
    "string_pos_integer_argument",
    "string_size_missing_bracket",
    "string_size_not_constant",
    "string_size_zero_over_255",
    "string_var_param_anonymous_string_n",
    "string_var_param_different_size",
    "string_var_param_expression",
    "string_var_param_literal",
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
    assert!(ABI.contains(&"string_function_result_hidden_pointer"));
}

fn compile(src: &str) -> Vec<u8> {
    tp5c::compile(src.as_bytes())
        .unwrap_or_else(|d| panic!("rejected with {} at {}:\n{src}", d.code, d.line))
}

/// Strings cost nothing until used (decision 14): the helpers live in System and are emitted only
/// when a live body calls them.
#[test]
fn string_helpers_are_emitted_only_when_used() {
    let hello = compile("program Hello;\nbegin\n  WriteLn('hello')\nend.");
    assert!(hello.len() <= 430, "hello-world is {} bytes", hello.len());
    let with = |body: &str| {
        compile(&format!(
            "program P;\nvar s: String;\nbegin\n  s := 'abc';\n  {body}\nend."
        ))
    };
    let none = with("WriteLn(s)");
    let pos = with("WriteLn(Pos('b', s))");
    let both = with("WriteLn(Pos('b', s), Copy(s, 1, 2))");
    let dead = compile("program P;\nvar s: String;\nprocedure Dead;\nbegin\n  s := Copy(s, 1, 2) + s;\n  Delete(s, 1, 1);\n  Insert(s, s, 1)\nend;\nbegin\n  s := 'abc';\n  WriteLn(s)\nend.");
    assert!(none.len() < pos.len() && pos.len() < both.len());
    assert_eq!(none, dead, "an unused routine changed the module");
}

#[test]
fn string_literals_in_unused_code_are_not_in_the_module() {
    let m = compile("program P;\nprocedure Dead;\nvar s: String;\nbegin\n  s := 'dead-marker' + 'x'\nend;\nbegin\n  WriteLn('live-marker')\nend.");
    let has = |n: &str| m.windows(n.len()).any(|w| w == n.as_bytes());
    assert!(has("live-marker") && !has("dead-marker"));
}
