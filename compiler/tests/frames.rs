//! Shadow-stack frames and var parameters: the exported `sp`, frame prologue and epilogue,
//! var parameters as addresses, stack overflow (runtime error 202).
//! Corpus programs are copied without PENDING into a scratch directory and checked with the
//! real compiler; expected output and `N LINE` come from the real TP 5.0 (`tools/tp5.sh`).
mod common;
mod driver;
use common::*;
use driver::*;
use std::path::PathBuf;
use std::process::Command;
use tp5c::wasm::*;

/// tests/abi programs this slice must pass.
const ABI: &[&str] = &[
    "args_evaluated_left_to_right_with_side_effects",
    "frame_exit_releases_frame",
    "frame_for_control_variable_in_memory",
    "frame_function_result_with_frame",
    "frame_mixed_sizes_layout",
    "frame_recursion_locals_per_activation",
    "frame_value_param_copied_in",
    "recursion_deep_procedure_var_param",
    "value_param_passed_as_var_inside",
    "var_param_alias_local_and_value",
    "var_param_alias_same_variable",
    "var_param_all_scalar_types",
    "var_param_forwarded_through_chain",
    "var_param_global_also_accessed_directly",
    "var_param_local_and_value_param_arguments",
    "var_param_many_mixed",
    "var_param_narrow_store_wraps",
    "var_param_neighbours_untouched",
    "var_param_recursive",
    "var_param_swap",
];

/// tests/lang programs this slice must pass.
const LANG: &[&str] = &[
    "var_param_alias_two_parameters",
    "var_param_basic",
    "var_param_char_boolean",
    "var_param_declaration_forms",
    "var_param_function_result",
    "var_param_global_argument",
    "var_param_in_conditions",
    "var_param_in_function",
    "var_param_integer_types",
    "var_param_local_argument",
    "var_param_multiple_results",
    "var_param_read_before_write",
    "var_param_sign_extension_on_read",
];

/// tests/neg programs this slice must pass.
const NEG: &[&str] = &[
    "var_param_assigned_wrong_type",
    "var_param_boolean_constant_argument",
    "var_param_boolean_for_integer",
    "var_param_char_literal_argument",
    "var_param_comparison_argument",
    "var_param_constant_argument",
    "var_param_constant_left_operand",
    "var_param_duplicate_of_local",
    "var_param_expression_argument",
    "var_param_for_control_variable",
    "var_param_forward_header_mismatch",
    "var_param_function_call_argument",
    "var_param_function_call_plus_expression",
    "var_param_function_needing_arguments_argument",
    "var_param_function_result_as_argument",
    "var_param_function_without_parameters_argument",
    "var_param_intrinsic_function_argument",
    "var_param_local_constant_inside_routine",
    "var_param_named_constant_argument",
    "var_param_not_operand",
    "var_param_parenthesised_argument",
    "var_param_procedure_name_argument",
    "var_param_product_argument",
    "var_param_second_argument_constant",
    "var_param_shortint_for_byte",
    "var_param_string_literal_argument",
    "var_param_too_few_arguments",
    "var_param_too_many_arguments",
    "var_param_type_must_be_identical_byte_integer",
    "var_param_type_must_be_identical_integer_longint",
    "var_param_type_must_be_identical_word_integer",
    "var_param_unary_minus_argument",
    "var_param_write_argument",
];

/// TP accepts these; the compiler reports 200 (decision 6) on the given line.
const UNSUPPORTED: &[(&str, &str, u32)] = &[(
    "untyped_var_parameter",
    "program Prog;\nprocedure Q(var a);\nbegin\nend;\nbegin\nend.",
    2,
)];

/// Programs the compiler must accept (TP5 compiles them).
const ACCEPTED: &[(&str, &str)] = &[
    (
        "var_parameter",
        "program Prog;\nprocedure Q(var a: Integer);\nbegin\nend;\nbegin\nend.",
    ),
    (
        "var_parameter_in_function",
        "program Prog;\nfunction F(var a: Integer): Integer;\nbegin\n  F := 0\nend;\nbegin\nend.",
    ),
    (
        "var_group_then_value_group",
        "program Prog;\nprocedure Q(var a, b: Byte; c: Byte);\nbegin\nend;\nbegin\nend.",
    ),
    (
        "local_as_control_variable_and_var_argument",
        "program Prog;\nprocedure T(var x: Integer);\nbegin\nend;\nprocedure Q;\nvar v: Integer;\nbegin\n  for v := 1 to 3 do T(v)\nend;\nbegin\nend.",
    ),
];

/// A routine whose frame is 40 bytes (ten LongInts passed as var arguments), recursing
/// `$N` levels; the main program dives to depth `$A`, prints, then dives to depth `$B`.
const DIVE: &str = "program Overflow;
procedure Touch(var x: LongInt);
begin
  x := x + 1
end;
procedure Dive(n: LongInt);
var a, b, c, d, e, f, g, h, i, j: LongInt;
begin
  Touch(a); Touch(b); Touch(c); Touch(d); Touch(e);
  Touch(f); Touch(g); Touch(h); Touch(i); Touch(j);
  if n > 0 then Dive(n - 1)
end;
begin
  WriteLn('start');
  Dive($A);
  WriteLn('first dive done');
  Dive($B);
  WriteLn('second dive done')
end.";

/// A frame of 2 bytes (one Integer) per activation; `R(2)` nests three frames.
const SP_PROBE: &str = "program SpProbe;
procedure Bump(var x: Integer);
begin
  x := x + 1
end;
procedure R(n: Integer);
var v: Integer;
begin
  v := n;
  Bump(v);
  if n > 0 then R(n - 1) else WriteLn('deep')
end;
begin
  WriteLn('before');
  R(2);
  WriteLn('after')
end.";

/// Reads the exported `sp` at every `host.write` and around `main`; prints
/// `sp0 sp1 s1,s2,...` (consecutive duplicates removed), or `no-sp`.
const SP_SCRIPT: &str = "import fs from 'node:fs';
const m = await WebAssembly.compile(fs.readFileSync(process.argv[1]));
let inst; const seen = [];
const host = { write() { const v = inst.exports.sp.value; if (seen.at(-1) !== v) seen.push(v); }, rterror() {}, fs_write() {} };
inst = await WebAssembly.instantiate(m, { host });
const sp = inst.exports.sp;
if (!sp || !(sp instanceof WebAssembly.Global)) { console.log('no-sp'); process.exit(0); }
const sp0 = sp.value;
inst.exports.main();
console.log(sp0 + ' ' + sp.value + ' ' + seen.join(','));";

fn tmp_dir() -> PathBuf {
    let d = root().join("tmp/frames");
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
fn var_parameters_are_accepted() {
    let mut fails = Vec::new();
    for &(name, src) in ACCEPTED {
        if let Err(d) = tp5c::compile([src, "\n"].concat().as_bytes()) {
            fails.push(format!("{name}: rejected with {} at {}", d.code, d.line));
        }
    }
    assert!(fails.is_empty(), "{}", fails.join("\n"));
}

#[test]
fn unsupported_var_forms_report_error_200() {
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

/// Compiles `src`, runs it under tests/run.mjs; returns (exit code, stdout, stderr).
fn run_source(name: &str, src: &str) -> (i32, String, String) {
    let wasm = tp5c::compile(src.as_bytes())
        .unwrap_or_else(|d| panic!("{name}: rejected with {} at {}:{}", d.code, d.line, d.col));
    let path = tmp_dir().join([name, ".wasm"].concat());
    std::fs::write(&path, wasm).unwrap();
    let o = Command::new("node")
        .arg(root().join("tests/run.mjs"))
        .arg(&path)
        .output()
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
fn stack_overflow_is_runtime_error_202() {
    let (code, out, err) = run_source("overflow", &dive(1000, 100_000));
    assert_eq!(err, "");
    assert_eq!(out, "start\nfirst dive done\nRuntime error 202\n");
    assert_eq!(code, 202);
}

/// The host's own stack runs out first in a routine without a frame; the runner reports it
/// as the same runtime error.
#[test]
fn host_stack_exhaustion_is_runtime_error_202() {
    let src = "program Prog;\nprocedure R(n: Integer);\nbegin\n  R(n + 1)\nend;\nbegin\n  WriteLn('go');\n  R(0)\nend.\n";
    let (code, out, err) = run_source("host_stack", src);
    assert_eq!(err, "");
    assert_eq!(out, "go\nRuntime error 202\n");
    assert_eq!(code, 202);
}

#[test]
fn deep_recursion_within_the_stack_does_not_overflow() {
    let (code, out, err) = run_source("no_overflow", &dive(1000, 10));
    assert_eq!(err, "");
    assert_eq!(out, "start\nfirst dive done\nsecond dive done\n");
    assert_eq!(code, 0);
}

/// The limit is the 64 KB of docs/abi.md: a 40-byte frame fits 1638 times, not 1639.
#[test]
fn stack_limit_is_exactly_65536_bytes() {
    let (code, out, _) = run_source("limit_fits", &dive(1637, 0));
    assert_eq!(
        (code, out.as_str()),
        (0, "start\nfirst dive done\nsecond dive done\n")
    );
    let (code, out, _) = run_source("limit_exceeded", &dive(1638, 0));
    assert_eq!((code, out.as_str()), (202, "start\nRuntime error 202\n"));
}

/// Output of SP_SCRIPT for a module.
fn probe_sp(name: &str, wasm: &[u8]) -> String {
    let path = tmp_dir().join([name, ".wasm"].concat());
    std::fs::write(&path, wasm).unwrap();
    let o = Command::new("node")
        .args(["--input-type=module", "-e", SP_SCRIPT])
        .arg(&path)
        .output()
        .expect("node must be installed");
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    String::from_utf8_lossy(&o.stdout).into_owned()
}

const TOP: u32 = 16 * 65536;

/// Control for the probe itself: a hand-built module that lowers sp by 6 around a write.
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
    for delta in [-6, 6] {
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
    let out = probe_sp("sp_hand_built", &m.finish());
    assert_eq!(out, format!("{TOP} {TOP} {},{TOP}\n", TOP - 6));
}

#[test]
fn sp_is_exported_and_follows_the_frames() {
    let wasm = tp5c::compile(SP_PROBE.as_bytes()).unwrap();
    // Three 2-byte frames are live when 'deep' is written.
    let want = format!("{TOP} {TOP} {TOP},{},{TOP}\n", TOP - 6);
    assert_eq!(probe_sp("sp_probe", &wasm), want);
}

#[test]
fn sp_is_exported_when_no_routine_has_a_frame() {
    let src =
        "program Prog;\nprocedure P(a: Integer);\nbegin\n  WriteLn(a)\nend;\nbegin\n  P(1)\nend.\n";
    let wasm = tp5c::compile(src.as_bytes()).unwrap();
    assert_eq!(
        probe_sp("sp_no_frames", &wasm),
        format!("{TOP} {TOP} {TOP}\n")
    );
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
            .output()
            .unwrap();
        String::from_utf8_lossy(&o.stdout).into_owned()
    };
    let got = run("dive_overflow", &dive(1000, 100_000));
    assert!(
        got.starts_with("start\nfirst dive done\nRuntime error 202 at "),
        "{got:?}"
    );
    let got = run("dive_fits", &dive(1000, 10));
    assert_eq!(got, "start\nfirst dive done\nsecond dive done\n");
}
