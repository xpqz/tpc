//! Pointers and the heap (decisions 12, 18, 21, 25): pointer types and forward `^T`, `nil`, `^`,
//! `@`, comparison, `LongInt`/pointer casts, `New`/`Dispose`, `GetMem`/`FreeMem`, `Mark`/`Release`,
//! `MemAvail`/`MaxAvail`, runtime errors 203 and 204. Every `.out` and `.err` (`N LINE`) comes from
//! the real TP 5.0 (`tools/tp5.sh`); runtime-error outputs are normalised as in tests/rt. No program
//! prints an address, a byte count of the heap, or reads freed memory.
mod common;
mod driver;
use driver::*;
use std::path::{Path, PathBuf};

fn fx(sub: &str) -> PathBuf {
    root().join("compiler/tests/fixtures/pointers").join(sub)
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
    assert!(stems(&fx("lang"), ".pas").len() >= 7);
    assert_eq!(stems(&fx("lang"), ".pas"), stems(&fx("lang"), ".out"));
    for s in stems(&fx("lang"), ".out") {
        let out = std::fs::read(fx("lang").join(format!("{s}.out"))).unwrap();
        assert!(!out.is_empty(), "{s}: empty expected output");
    }
    assert_clean(check_lang(&fx("lang"), &real_compile));
}

#[test]
fn own_runtime_errors_match_tp5() {
    assert!(stems(&fx("rt"), ".pas").len() >= 4);
    assert_eq!(stems(&fx("rt"), ".pas"), stems(&fx("rt"), ".out"));
    assert_clean(check_rt(&fx("rt"), &real_compile));
}

#[test]
fn own_invalid_programs_report_tp5_number_and_line() {
    assert!(stems(&fx("neg"), ".pas").len() >= 19);
    assert_eq!(stems(&fx("neg"), ".pas"), stems(&fx("neg"), ".err"));
    assert_clean(check_neg(&fx("neg"), &real_compile));
}

/// Corpus programs (tests/lang, tests/abi, tests/rt, tests/neg) that this slice must make pass.
/// Left out: `range_pointer_to_array_index` (needs `{$R+}` array checks) and the Real/set
/// programs that do not use pointers.
const LANG: &[&str] = &[
    "heap_exhaust_with_maxavail_then_release",
    "heap_getmem_array_type",
    "heap_getmem_blocks_do_not_overlap",
    "heap_getmem_freemem_basic",
    "heap_mark_release_basic",
    "heap_mark_release_in_procedure",
    "heap_mark_release_nested",
    "heap_memavail_maxavail_relations",
    "heap_new_dispose_reuses_freed_block",
    "heap_new_of_each_type_size",
    "heap_release_discards_freed_blocks_above_mark",
    "heap_release_reuses_memory",
    "pointer_address_of_variable",
    "pointer_array_of_pointers",
    "pointer_binary_tree_height_count_search",
    "pointer_binary_tree_insert_and_inorder",
    "pointer_cast_byte_view_of_word",
    "pointer_cast_longint_roundtrip",
    "pointer_degenerate_tree_is_a_list",
    "pointer_dispose_in_any_order_then_reuse",
    "pointer_forward_type_declaration",
    "pointer_linked_list_append_and_reverse",
    "pointer_linked_list_sorted_insert_and_delete",
    "pointer_linked_list_stack",
    "pointer_new_dispose_basic",
    "pointer_new_dispose_stress_loop",
    "pointer_nil_comparisons",
    "pointer_record_of_pointers_graph",
    "pointer_to_array_and_string_types",
    "pointer_to_pointer",
    "pointer_to_record_fields",
    "pointer_var_parameter_and_function_result",
];

const ABI: &[&str] = &[
    "heap_alloc_inside_frames_with_mark_release",
    "heap_getmem_in_loop_with_deep_recursion",
    "pointer_function_result_and_string_pointer",
    "pointer_parameters_and_nested_access",
];

const RT: &[&str] = &[
    "heap_overflow_after_release_succeeds_then_fails_again",
    "heap_overflow_getmem_until_exhausted",
    "heap_overflow_new_until_exhausted",
    "pointer_dispose_nil",
    "pointer_dispose_stack_address",
    "pointer_dispose_wild_address",
    "pointer_freemem_nil",
];

const NEG: &[&str] = &[
    "address_of_constant",
    "freemem_missing_size",
    "getmem_non_pointer",
    "pointer_arithmetic_plus",
    "pointer_assign_integer",
    "pointer_assign_other_pointer_type",
    "pointer_deref_non_pointer",
    "pointer_field_access_without_caret",
    "pointer_forward_type_never_defined",
    "pointer_new_non_pointer",
    "pointer_nil_assigned_to_integer",
    "pointer_ordering_comparison",
    "pointer_unknown_field_through_caret",
    "pointer_write",
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
fn corpus_runtime_errors_match_tp5() {
    assert_clean(check_rt(&subset("rt", RT, &["pas", "out"]), &real_compile));
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
        ("rt", RT, "out"),
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
}

fn compile(src: &str) -> Vec<u8> {
    tp5c::compile(src.as_bytes())
        .unwrap_or_else(|d| panic!("rejected with {} at {}:\n{src}", d.code, d.line))
}

fn rejected(src: &str) -> (u16, u32) {
    match tp5c::compile(src.as_bytes()) {
        Ok(_) => panic!("accepted:\n{src}"),
        Err(d) => (d.code, d.line),
    }
}

/// The allocator lives in System and is emitted only when a live body uses it (decision 14).
#[test]
fn allocator_is_emitted_only_when_used() {
    let hello = compile("program Hello;\nbegin\n  WriteLn('hello')\nend.");
    common::assert_hello_fits(&hello);
    let plain = compile("program Prog;\nvar i: Integer;\nbegin\n  i := 1;\n  WriteLn(i)\nend.");
    let dead = compile(
        "program Prog;\ntype PI = ^Integer;\nprocedure Dead;\nvar p: PI;\nbegin\n  New(p);\n  p^ := 1;\n  Dispose(p)\nend;\nvar i: Integer;\nbegin\n  i := 1;\n  WriteLn(i)\nend.",
    );
    assert_eq!(
        plain, dead,
        "an unused routine using New changed the module"
    );
    let used = compile(
        "program Prog;\ntype PI = ^Integer;\nvar p: PI;\nbegin\n  New(p);\n  p^ := 1;\n  WriteLn(p^);\n  Dispose(p)\nend.",
    );
    assert!(used.len() > plain.len());
}

/// User declarations named like allocator state or routines must not capture what `New`,
/// `Dispose`, `GetMem`, `FreeMem` call (helpers are bound by handle, decision 11).
#[test]
fn user_names_do_not_capture_allocator_helpers() {
    let src = "program Clash;\ntype PI = ^Integer;\nvar HeapPtr, FreeList, HeapOrg, HeapEnd, HeapTop: LongInt;\n  p, q: PI; m: Pointer;\nfunction Alloc(n: Integer): Integer;\nbegin\n  Alloc := -n\nend;\nprocedure HeapAlloc(var a: Integer);\nbegin\n  a := 99\nend;\nprocedure Free(i: Integer);\nbegin\n  WriteLn('user free ', i)\nend;\nvar k: Integer;\nbegin\n  HeapPtr := 1; FreeList := 2; HeapOrg := 3; HeapEnd := 4; HeapTop := 5;\n  New(p); New(q);\n  p^ := Alloc(3); HeapAlloc(k); q^ := k;\n  WriteLn(p^, ' ', q^, ' ', p <> q);\n  Dispose(p); Free(1);\n  New(p);\n  GetMem(m, 10); FreeMem(m, 10);\n  Dispose(q); Dispose(p);\n  WriteLn(HeapPtr + FreeList + HeapOrg + HeapEnd + HeapTop)\nend.\n";
    let dir = scratch("x").parent().unwrap().to_path_buf();
    std::fs::write(dir.join("clash.pas"), src).unwrap();
    std::fs::write(dir.join("clash.out"), "-3 99 TRUE\nuser free 1\n15\n").unwrap();
    assert_clean(check_lang(&dir, &real_compile));
}

#[test]
fn pointer_rules_in_the_checker() {
    let wrap = |decl: &str, body: &str| format!("program T;\n{decl}\nbegin\n  {body}\nend.");
    let decl =
        "type PI = ^Integer; PB = ^Byte;\nvar p: PI; b: PB; g: Pointer; i: Integer; l: LongInt;";
    for ok in [
        "p := nil; g := nil; b := nil",
        "g := p; g := @i; p := @i; p := @l; p := g",
        "b := PB(p); l := LongInt(p); p := PI(l); p := PI(LongInt(p))",
        "New(p); p^ := 1; i := p^; Dispose(p)",
        "GetMem(g, 10); FreeMem(g, 10); GetMem(p, 2); FreeMem(p, 2)",
        "Mark(g); Release(g)",
        "l := MemAvail; l := MaxAvail",
        "if (p = nil) or (g <> nil) or (p = g) or (g = p) then i := 0",
    ] {
        compile(&wrap(decl, ok));
    }
    for (bad, want) in [
        ("p := b", (26, 6)),
        ("b := p", (26, 6)),
        ("p := 1", (26, 6)),
        ("i := p", (26, 6)),
        ("i := nil", (26, 6)),
        ("p := p + 1", (26, 6)),
        ("if p < p then i := 1", (41, 5)),
        ("i := i^", (121, 5)),
        ("New(g, 1)", (89, 5)),
        ("Dispose(i)", (65, 5)),
        ("FreeMem(p)", (87, 5)),
        ("Mark(i)", (65, 5)),
        ("Write(p)", (64, 5)),
        ("l := MemAvail(1)", (85, 5)),
    ] {
        assert_eq!(rejected(&wrap(decl, bad)), want, "{bad}");
    }
}
