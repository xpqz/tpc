//! Nested routines with static links: access to enclosing routines' variables, calls at every
//! depth, frames with the static link slot, stack limit with nested frames.
//! Corpus programs are copied without PENDING into a scratch directory and checked with the
//! real compiler; expected output and `N LINE` come from the real TP 5.0 (`tools/tp5.sh`).
mod common;
use common::OutputGuarded as _;
mod driver;
use common::*;
use driver::*;
use std::path::PathBuf;
use std::process::Command;
use tp5c::wasm::*;

/// tests/abi programs this slice must pass.
const ABI: &[&str] = &[
    "exit_inside_nested_loops_with_locals",
    "exit_nested_routine_exits_only_itself",
    "frame_locals_isolated_between_calls",
    "frame_many_captured_locals",
    "function_result_assigned_through_nested",
    "nested_call_enclosing_chain",
    "nested_called_from_two_depths",
    "nested_captured_in_conditions",
    "nested_captured_types",
    "nested_enclosing_function_result_assigned",
    "nested_exit_with_frames",
    "nested_four_levels_all_variables",
    "nested_function_results_use_outer",
    "nested_functions_in_expressions",
    "nested_link_through_uncle",
    "nested_outer_local_used_before_and_after_call",
    "nested_outer_parameters",
    "nested_passes_outer_local_as_var",
    "nested_reads_outer_loop_variable",
    "nested_recursion_with_static_link",
    "nested_routine_without_captured_variables",
    "nested_same_name_shadowing",
    "nested_sibling_calls",
    "nested_three_levels_modify_outer",
    "nested_value_param_captured_wraps",
    "nested_var_param_forwarded_from_depth_three",
    "recursion_nested_each_level_own_frame",
    "recursion_outer_reentered_from_nested",
    "recursion_through_nested_routine",
    "var_param_nested_routine_sees_it",
];

/// tests/lang programs this slice must pass.
const LANG: &[&str] = &[
    "nested_forward_declaration",
    "nested_function_basic",
    "nested_global_routine_call_from_nested",
    "nested_local_shadows_outer_local",
    "nested_procedure_basic",
    "nested_reads_outer_local",
    "nested_reads_outer_parameter",
    "nested_recursive_function",
    "nested_routine_shadows_enclosing_name",
    "nested_three_levels",
    "nested_two_siblings",
    "nested_writes_outer_local",
];

/// tests/neg programs this slice must pass.
const NEG: &[&str] = &[
    "nested_assign_to_sibling_function_result",
    "nested_call_to_later_declared_uncle",
    "nested_constant_for_var_parameter",
    "nested_duplicate_name_in_same_routine",
    "nested_for_variable_of_enclosing_routine",
    "nested_for_variable_var_parameter_of_enclosing",
    "nested_forward_header_mismatch",
    "nested_forward_never_defined",
    "nested_function_as_statement",
    "nested_local_not_visible_outside",
    "nested_missing_arguments",
    "nested_name_clashes_with_local",
    "nested_name_clashes_with_parameter",
    "nested_parent_uses_child_local",
    "nested_routine_called_before_declaration",
    "nested_routine_not_visible_outside",
    "nested_sibling_local_not_visible",
    "nested_undefined_forward_two_levels_down",
    "nested_wrong_argument_type",
];

/// Programs the compiler must accept (TP5 compiles them).
const ACCEPTED: &[(&str, &str)] = &[
    (
        "nested_procedure",
        "program Prog;\nprocedure Q;\n  procedure R;\n  begin\n  end;\nbegin\n  R\nend;\nbegin\nend.",
    ),
    (
        "nested_function_in_function",
        "program Prog;\nfunction F: Integer;\n  function G: Integer;\n  begin\n    G := 1\n  end;\nbegin\n  F := G\nend;\nbegin\nend.",
    ),
    (
        "nested_three_deep_with_forward",
        "program Prog;\nprocedure A;\n  procedure B;\n    procedure C; forward;\n    procedure C;\n    begin\n    end;\n  begin\n    C\n  end;\nbegin\n  B\nend;\nbegin\nend.",
    ),
    (
        "nested_assigns_enclosing_function_result",
        "program Prog;\nfunction F: Integer;\n  procedure S;\n  begin\n    F := 7\n  end;\nbegin\n  S\nend;\nbegin\nend.",
    ),
];

/// A nested routine whose frame is 40 bytes (static link plus nine LongInts passed as var
/// arguments), recursing `$N` levels; the main program dives to `$A`, prints, then to `$B`.
const DIVE: &str = "program Overflow;
procedure Touch(var x: LongInt);
begin
  x := x + 1
end;
procedure Wrap;
  procedure Dive(n: LongInt);
  var a, b, c, d, e, f, g, h, i: LongInt;
  begin
    Touch(a); Touch(b); Touch(c); Touch(d); Touch(e);
    Touch(f); Touch(g); Touch(h); Touch(i);
    if n > 0 then Dive(n - 1)
  end;
begin
  WriteLn('start');
  Dive($A);
  WriteLn('first dive done');
  Dive($B);
  WriteLn('second dive done')
end;
begin
  Wrap
end.";

/// Frames: Outer 2 bytes (captured `a`), Mid 6 (static link, captured `b`), Inner none (it
/// has a link but captures nothing and has no nested routine).
const SP_PROBE: &str = "program SpNested;
procedure Bump(var x: Integer);
begin
  x := x + 1
end;
procedure Outer;
var a: Integer;
  procedure Mid;
  var b: Integer;
    procedure Inner;
    begin
      a := a + 1;
      b := b + 1;
      WriteLn('i')
    end;
  begin
    b := 0;
    Bump(b);
    WriteLn('m');
    Inner
  end;
begin
  a := 0;
  WriteLn('o');
  Mid
end;
begin
  WriteLn('before');
  Outer;
  WriteLn('after')
end.";

/// Reads the exported `sp` at every `host.write`; prints `sp0 sp1 s1,s2,...` (consecutive
/// duplicates removed).
const SP_SCRIPT: &str = "import fs from 'node:fs';
const m = await WebAssembly.compile(fs.readFileSync(process.argv[1]));
let inst; const seen = [];
const host = { write() { const v = inst.exports.sp.value; if (seen.at(-1) !== v) seen.push(v); }, rterror() {}, fs_write() {} };
inst = await WebAssembly.instantiate(m, { host });
const sp = inst.exports.sp;
const sp0 = sp.value;
inst.exports.main();
console.log(sp0 + ' ' + sp.value + ' ' + seen.join(','));";

const TOP: u32 = 16 * 65536;

fn tmp_dir() -> PathBuf {
    crate::common::fresh_dir("nested")
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
fn nested_routines_are_accepted() {
    let mut fails = Vec::new();
    for &(name, src) in ACCEPTED {
        if let Err(d) = tp5c::compile([src, "\n"].concat().as_bytes()) {
            fails.push(format!("{name}: rejected with {} at {}", d.code, d.line));
        }
    }
    assert!(fails.is_empty(), "{}", fails.join("\n"));
}

/// Compiles `src`, runs it under tests/run.mjs; returns (exit code, stdout, stderr).
fn run_source(name: &str, src: &str) -> (i32, String, String) {
    let wasm = tp5c::compile(src.as_bytes())
        .unwrap_or_else(|d| panic!("{name}: rejected with {} at {}:{}", d.code, d.line, d.col));
    let path = tmp_dir().join([name, ".wasm"].concat());
    std::fs::write(&path, wasm).unwrap();
    let o = Command::new("node")
        .arg(root().join("tests/run.mjs"))
        .arg(&path)
        .output_guarded()
        .expect("node must be installed");
    (
        o.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&o.stdout).into_owned(),
        String::from_utf8_lossy(&o.stderr).into_owned(),
    )
}

fn dive(first: u32, second: u32) -> String {
    DIVE.replace("$A", &first.to_string())
        .replace("$B", &second.to_string())
}

#[test]
fn nested_stack_overflow_is_runtime_error_202() {
    let (code, out, err) = run_source("nested_overflow", &dive(1000, 100_000));
    assert_eq!(err, "");
    assert_eq!(out, "start\nfirst dive done\nRuntime error 202\n");
    assert_eq!(code, 202);
}

#[test]
fn nested_deep_recursion_within_the_stack_does_not_overflow() {
    let (code, out, err) = run_source("nested_no_overflow", &dive(1000, 10));
    assert_eq!(err, "");
    assert_eq!(out, "start\nfirst dive done\nsecond dive done\n");
    assert_eq!(code, 0);
}

/// The static link takes 4 bytes of the frame (docs/abi.md): 40-byte frames fit 1638 times
/// in 64 KB, not 1639. The routine that only contains `Dive` has no variables and adds no bytes.
#[test]
fn nested_frame_includes_the_static_link_slot() {
    let (code, out, _) = run_source("nested_limit_fits", &dive(1637, 0));
    assert_eq!(
        (code, out.as_str()),
        (0, "start\nfirst dive done\nsecond dive done\n")
    );
    let (code, out, _) = run_source("nested_limit_exceeded", &dive(1638, 0));
    assert_eq!((code, out.as_str()), (202, "start\nRuntime error 202\n"));
}

#[test]
fn sp_follows_nested_frames() {
    let wasm = tp5c::compile(SP_PROBE.as_bytes()).unwrap();
    let path = tmp_dir().join("sp_nested.wasm");
    std::fs::write(&path, wasm).unwrap();
    let o = Command::new("node")
        .args(["--input-type=module", "-e", SP_SCRIPT])
        .arg(&path)
        .output_guarded()
        .expect("node must be installed");
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    let want = format!("{TOP} {TOP} {TOP},{},{},{TOP}\n", TOP - 2, TOP - 8);
    assert_eq!(String::from_utf8_lossy(&o.stdout), want);
}

/// Control for the probe script: a hand-built module lowering sp by 8 around a write.
#[test]
fn sp_probe_reads_a_hand_built_module() {
    let mut m = Module::new();
    let write = m.ty(&[I32, I32, I32], &[]);
    let rterror = m.ty(&[I32], &[]);
    let main_ty = m.ty(&[], &[]);
    let write_f = m.import_func("host", "write", write);
    m.import_func("host", "rterror", rterror);
    m.memory(1);
    let sp = m.global_i32(true, TOP as i32);
    let mut c = Code::new();
    for delta in [-8, 8] {
        c.idx(GLOBAL_GET, sp)
            .i32_const(delta)
            .op(I32_ADD)
            .idx(GLOBAL_SET, sp);
        c.i32_const(1)
            .i32_const(256)
            .i32_const(0)
            .idx(CALL, write_f);
    }
    let f = m.func(main_ty, &[], c);
    m.export("memory", MEM, 0);
    m.export("main", FUNC, f);
    m.export("sp", GLOBAL, sp);
    let path = tmp_dir().join("sp_hand_built.wasm");
    std::fs::write(&path, m.finish()).unwrap();
    let o = Command::new("node")
        .args(["--input-type=module", "-e", SP_SCRIPT])
        .arg(&path)
        .output_guarded()
        .unwrap();
    let want = format!("{TOP} {TOP} {},{TOP}\n", TOP - 8);
    assert_eq!(String::from_utf8_lossy(&o.stdout), want);
}

/// Slow (DOSBox-X): `cargo test -- --ignored`. Re-derives the TP5 side of the 202 programs
/// (a `{$M}` directive gives TP the 64 KB stack; the compiler does not read directives yet).
#[test]
#[ignore]
fn oracle_agrees() {
    let run = |name: &str, src: &str| {
        let p = tmp_dir().join([name, ".pas"].concat());
        let src = src.replacen(";\n", ";\n{$M 65520,0,655360}\n", 1);
        std::fs::write(&p, src).unwrap();
        let o = Command::new(root().join("tools/tp5.sh"))
            .arg(&p)
            .output_guarded()
            .unwrap();
        String::from_utf8_lossy(&o.stdout).into_owned()
    };
    let got = run("dive_overflow", &dive(1000, 100_000));
    assert!(
        got.starts_with("start\nfirst dive done\nRuntime error 202 at "),
        "{got:?}"
    );
    assert_eq!(
        run("dive_fits", &dive(1000, 10)),
        "start\nfirst dive done\nsecond dive done\n"
    );
}
