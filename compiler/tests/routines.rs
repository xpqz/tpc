//! Procedures and functions with value parameters, forward declarations, Exit, and the
//! intrinsics Ord, Chr, Succ, Pred, Odd, Abs and Sqr.
//! Expected output and `(code, line)` pairs come from the real TP 5.0 (`oracle_agrees`
//! re-derives them with `tools/tp5.sh`). The target corpus programs are copied without
//! PENDING into a scratch directory and checked with the real compiler.
mod common;
mod driver;
use driver::*;
use std::path::PathBuf;
use std::process::Command;

/// tests/lang programs this slice must pass.
const LANG: &[&str] = &[
    "boolean_logic_table",
    "boolean_short_circuit",
    "case_byte_selector",
    "case_char_selector",
    "case_ranges_lists_else",
    "collatz_steps",
    "comparisons_integers",
    "div_mod_truncate_toward_zero",
    "func_basic",
    "func_result_assigned_repeatedly",
    "func_result_narrowed_on_return",
    "func_result_types",
    "if_dangling_else",
    "if_else_chain",
    "leap_years",
    "odd_abs_sqr",
    "ord_chr_conversions",
    "param_conversions",
    "param_is_a_copy",
    "primes_by_function",
    "proc_calls_proc",
    "proc_no_params",
    "proc_value_params",
    "recursion_ackermann",
    "recursion_depth_1000",
    "recursion_factorial",
    "recursion_fibonacci",
    "recursion_gcd_power_digits",
    "recursion_mutual_via_global_state",
    "scope_global_local_shadow",
    "succ_pred",
    "towers_of_hanoi",
];

/// tests/neg programs this slice must pass.
const NEG: &[&str] = &[
    "duplicate_parameter_and_local",
    "duplicate_procedure_name",
    "function_result_type_missing",
    "missing_begin_in_procedure",
    "missing_left_paren_in_call_args",
    "too_many_arguments",
    "type_mismatch_argument",
    "type_mismatch_function_result",
];

const RUN: &[(&str, &str, &str)] = &[
    ("types1", "program Prog;\nvar b: Byte; s: ShortInt; w: Word; i: Integer; l: LongInt;\nbegin\n  b := 200; s := -128; w := 65535; i := 200; l := 40000;\n  WriteLn(Sqr(b), ' ', Sqr(s), ' ', Sqr(w), ' ', Sqr(i), ' ', Sqr(l));\n  WriteLn(Abs(s), ' ', Abs(b), ' ', Abs(w));\n  i := -32768;\n  WriteLn(Abs(i), ' ', Abs(-32768), ' ', Sqr(i));\n  l := -2147483647 - 1;\n  WriteLn(Abs(l), ' ', Sqr(l))\nend.", "-25536 16384 -131071 -25536 1600000000\n128 200 65535\n-32768 32768 0\n-2147483648 0\n"),
    ("types2", "program Prog;\nvar b: Byte; s: ShortInt; w: Word; i: Integer; l: LongInt;\nbegin\n  b := 255; s := 127; w := 65535; i := 32767; l := 2147483647;\n  WriteLn(Succ(b), ' ', Succ(s), ' ', Succ(w), ' ', Succ(i), ' ', Succ(l));\n  b := 0; s := -128; w := 0; i := -32768; l := -2147483647 - 1;\n  WriteLn(Pred(b), ' ', Pred(s), ' ', Pred(w), ' ', Pred(i), ' ', Pred(l))\nend.", "0 -128 0 -32768 -2147483648\n255 127 65535 32767 2147483647\n"),
    ("odd", "program Prog;\nvar b: Byte; w: Word; l: LongInt;\nbegin\n  b := 255; w := 65535; l := 65536 + 3;\n  WriteLn(Odd(b), ' ', Odd(w), ' ', Odd(l), ' ', Odd(-l))\nend.", "TRUE TRUE TRUE TRUE\n"),
    ("ordchr", "program Prog;\nvar i: Integer; w: Word; l: LongInt;\nbegin\n  i := 321; w := 65535; l := 70000 + 65;\n  WriteLn(Ord(Chr(i)), ' ', Ord(Chr(w)), ' ', Ord(Chr(l)));\n  WriteLn(Ord('a') + 1, ' ', Ord(True) + 1)\nend.", "65 255 177\n98 2\n"),
    ("sqr_const_range", "program Prog;\nvar i: Integer;\nbegin\n  i := Sqr(300);\n  WriteLn(i)\nend.", "24464\n"),
    ("exit_proc", "program Prog;\nprocedure P(n: Integer);\nbegin\n  if n > 2 then Exit;\n  WriteLn('small ', n)\nend;\nfunction F(n: Integer): Integer;\nbegin\n  F := 1;\n  if n = 0 then Exit;\n  F := 2\nend;\nbegin\n  P(1); P(5);\n  WriteLn(F(0), F(1));\n  Exit;\n  WriteLn('not reached')\nend.", "small 1\n12\n"),
    ("forward", "program Prog;\nprocedure B(n: Integer); forward;\nprocedure A(n: Integer);\nbegin\n  if n > 0 then begin Write('a', n, ' '); B(n - 1) end\nend;\nprocedure B;\nbegin\n  if n > 0 then begin Write('b', n, ' '); A(n - 1) end\nend;\nbegin\n  A(4); WriteLn\nend.", "a4 b3 a2 b1 \n"),
    ("forward_repeat_params", "program Prog;\nfunction F(n: Integer): Integer; forward;\nfunction F(n: Integer): Integer;\nbegin\n  if n = 0 then F := 0 else F := n + F(n - 1)\nend;\nbegin\n  WriteLn(F(4))\nend.", "10\n"),
    ("recursion_result_read", "program Prog;\nfunction F(n: Integer): Integer;\nbegin\n  if n <= 0 then F := 0\n  else F := F(n - 1) + 1\nend;\nbegin\n  WriteLn(F(5))\nend.", "5\n"),
    ("local_shadow_param_types", "program Prog;\nprocedure P(a: Byte; b: ShortInt; c: Word);\nvar l: Byte; m: Integer;\nbegin\n  l := a + 100; m := b * 1000;\n  WriteLn(a, ' ', b, ' ', c, ' ', l, ' ', m);\n  a := a + 100; b := b - 100; c := c + 1;\n  WriteLn(a, ' ', b, ' ', c)\nend;\nbegin\n  P(200, -100, 65535)\nend.", "200 -100 65535 44 31072\n44 56 0\n"),
    ("func_ret_types_expr", "program Prog;\nfunction B: Byte; begin B := 200 end;\nfunction I: Integer; begin I := 30000 end;\nbegin\n  WriteLn(B + B, ' ', I + I, ' ', -B)\nend.", "400 -5536 -200\n"),
    ("func_args_order", "program Prog;\nfunction Show(n: Integer): Integer;\nbegin\n  Write('[', n, ']');\n  Show := n\nend;\nfunction Sub(a, b: Integer): Integer;\nbegin\n  Sub := a - b\nend;\nbegin\n  WriteLn(Sub(Show(10), Show(3)))\nend.", "[10][3]7\n"),
    ("func_no_params_in_write", "program Prog;\nvar calls: Integer;\nfunction Next: Integer;\nbegin\n  calls := calls + 1;\n  Next := calls\nend;\nbegin\n  calls := 0;\n  WriteLn(Next, ' ', Next, ' ', Next:3)\nend.", "1 2   3\n"),
    ("constfold", "program Prog;\nconst A = Ord('a'); B = Chr(66); C = Succ(9); Dd = Pred(1); E = Odd(3);\n  F = Abs(-4); I = Abs(-40000); J = Succ('a'); K = Pred(True); L = Ord(True);\nbegin\n  WriteLn(A, ' ', B, ' ', C, ' ', Dd, ' ', E);\n  WriteLn(F, ' ', I, ' ', J, ' ', K, ' ', L)\nend.", "97 B 10 0 TRUE\n4 40000 b FALSE 1\n"),
    ("const_expr_intrinsics", "program Prog;\nconst A = Abs(-5) + 1; B = Chr(Ord('a') + 1); C = Odd(2) or Odd(3); Dd = Succ(255); E = Pred(0);\nbegin\n  WriteLn(A, ' ', B, ' ', C, ' ', Dd, ' ', E)\nend.", "6 b TRUE 256 -1\n"),
    ("const_chr_range", "program Prog;\nvar c: Char;\nbegin\n  c := Chr(300);\n  WriteLn(Ord(c))\nend.", "44\n"),
    ("case_ord_labels", "program Prog;\nvar i: Integer;\nbegin\n  i := 97;\n  case i of Ord('a'): WriteLn('a'); Ord('b')..Ord('z'): WriteLn('b-z') end\nend.", "a\n"),
    ("exit_in_loops", "program Prog;\nfunction Find(n: Integer): Integer;\nvar i, j: Integer;\nbegin\n  Find := -1;\n  for i := 1 to 10 do\n    for j := 1 to 10 do\n      if i * j = n then begin Find := i * 100 + j; Exit end\nend;\nprocedure Count(n: Integer);\nvar i: Integer;\nbegin\n  i := 0;\n  repeat\n    i := i + 1;\n    if i > n then Exit;\n    Write(i, ' ')\n  until False\nend;\nbegin\n  WriteLn(Find(42), ' ', Find(97));\n  Count(3); WriteLn\nend.", "607 -1\n1 2 3 \n"),
    ("calls_in_control", "program Prog;\nvar calls: Integer;\nfunction Five: Integer;\nbegin\n  calls := calls + 1;\n  Five := 5\nend;\nvar i: Integer;\nbegin\n  calls := 0;\n  for i := 1 to Five do Write(i);\n  WriteLn(' ', calls);\n  case Five of 5: WriteLn('five') end;\n  while Five < 3 do WriteLn('never');\n  repeat until Five = 5;\n  WriteLn(calls, ' ', Five:4, ' ', Five * 2:6)\nend.", "12345 1\nfive\n4    5     10\n"),
    ("procedure_decl_order", "program Prog;\nvar g: Integer;\nprocedure A;\nbegin\n  g := g + 1\nend;\nvar h: Integer;\nprocedure B;\nbegin\n  h := g * 2;\n  A\nend;\nbegin\n  g := 1; B; B;\n  WriteLn(g, ' ', h)\nend.", "3 4\n"),
    ("local_const_and_var", "program Prog;\nprocedure P(n: Integer);\nconst K = 10;\nvar a, b: Integer;\nbegin\n  a := n * K;\n  b := a + K;\n  WriteLn(a, ' ', b)\nend;\nbegin\n  P(3); P(4)\nend.", "30 40\n40 50\n"),
    ("global_after_proc_read_in_proc", "program Prog;\nvar g: Integer;\nfunction Get: Integer;\nbegin\n  Get := g\nend;\nprocedure Put(n: Integer);\nbegin\n  g := n\nend;\nbegin\n  Put(7); WriteLn(Get); Put(Get + 1); WriteLn(Get)\nend.", "7\n8\n"),
    ("param_modified_in_loop_recursion", "program Prog;\nfunction Sum(n: Integer): Integer;\nvar s: Integer;\nbegin\n  s := 0;\n  while n > 0 do begin s := s + n; n := n - 1 end;\n  Sum := s\nend;\nbegin\n  WriteLn(Sum(10), ' ', Sum(0), ' ', Sum(-3))\nend.", "55 0 0\n"),
    ("function_name_read_in_body", "program Prog;\nfunction Two: Integer;\nbegin\n  Two := 2\nend;\nfunction Four: Integer;\nbegin\n  Four := Two + Two\nend;\nbegin\n  WriteLn(Four)\nend.", "4\n"),
    ("mutual_forward_func", "program Prog;\nfunction IsOdd(n: Integer): Boolean; forward;\nfunction IsEven(n: Integer): Boolean;\nbegin\n  if n = 0 then IsEven := True else IsEven := IsOdd(n - 1)\nend;\nfunction IsOdd;\nbegin\n  if n = 0 then IsOdd := False else IsOdd := IsEven(n - 1)\nend;\nbegin\n  WriteLn(IsEven(10), ' ', IsOdd(7), ' ', IsEven(7))\nend.", "TRUE TRUE FALSE\n"),
    ("param_names_case_insensitive", "program Prog;\nprocedure P(Alpha: Integer);\nbegin\n  WriteLn(ALPHA + alpha)\nend;\nbegin\n  p(4)\nend.", "8\n"),
    ("succ_pred_types_char_bool", "program Prog;\nvar c: Char; b: Boolean;\nbegin\n  c := 'a'; b := False;\n  WriteLn(Succ(c), Pred(Succ(c)), Succ(b), ' ', Ord(Pred(c)), ' ', Chr(Ord(c) + 1))\nend.", "baTRUE 96 b\n"),
    ("bool_char_params", "program Prog;\nfunction Both(a, b: Boolean; c: Char): Boolean;\nbegin\n  Both := a and b and (c = 'x')\nend;\nfunction Next(c: Char): Char;\nbegin\n  Next := Succ(c)\nend;\nbegin\n  WriteLn(Both(True, True, 'x'), ' ', Both(True, False, 'x'), ' ', Next('a'), Next(Next('a')))\nend.", "TRUE FALSE bc\n"),
];

/// (name, source, TP error number, line)
const ERR: &[(&str, &str, u16, u32)] = &[
    ("too_few_zero_args", "program E;\nprocedure Q;\nbegin\nend;\nbegin\n  Q(1)\nend.", 85, 6),
    ("too_few_args", "program E;\nprocedure Q(a, b: Integer);\nbegin\nend;\nbegin\n  Q(1)\nend.", 87, 6),
    ("no_args_to_params", "program E;\nprocedure Q(a: Integer);\nbegin\nend;\nbegin\n  Q\nend.", 88, 7),
    ("arg_type_char_for_boolean", "program E;\nprocedure Q(a: Boolean);\nbegin\nend;\nbegin\n  Q('a')\nend.", 26, 6),
    ("arg_const_out_of_range", "program E;\nprocedure Q(a: Byte);\nbegin\nend;\nbegin\n  Q(300)\nend.", 76, 6),
    ("arg_unknown", "program E;\nprocedure Q(a: Byte);\nbegin\nend;\nbegin\n  Q(zz)\nend.", 3, 6),
    ("undefined_forward", "program E;\nprocedure Q(a: Byte); forward;\nbegin\nend.", 59, 3),
    ("undefined_forward_function", "program E;\nfunction Q(a: Byte): Byte; forward;\nprocedure R;\nbegin\nend;\nbegin\nend.", 59, 6),
    ("forward_twice", "program E;\nprocedure Q; forward;\nprocedure Q; forward;\nprocedure Q;\nbegin\nend;\nbegin\nend.", 36, 3),
    ("forward_mismatched_params", "program E;\nprocedure Q(a: Integer); forward;\nprocedure Q(a: Byte);\nbegin\nend;\nbegin\nend.", 131, 3),
    ("function_result_not_type", "program E;\nvar v: Integer;\nfunction F: v;\nbegin\nend;\nbegin\nend.", 12, 3),
    ("function_result_unknown_type", "program E;\nfunction F: Zork;\nbegin\nend;\nbegin\nend.", 3, 2),
    ("param_unknown_type", "program E;\nprocedure Q(a: Zork);\nbegin\nend;\nbegin\nend.", 3, 2),
    ("param_missing_type", "program E;\nprocedure Q(a);\nbegin\nend;\nbegin\nend.", 86, 2),
    ("param_missing_close_paren", "program E;\nprocedure Q(a: Integer;\nbegin\nend;\nbegin\nend.", 2, 3),
    ("param_missing_semicolon", "program E;\nprocedure Q(a: Integer b: Integer);\nbegin\nend;\nbegin\nend.", 89, 2),
    ("header_missing_semicolon", "program E;\nprocedure Q\nbegin\nend;\nbegin\nend.", 85, 3),
    ("routine_missing_name", "program E;\nprocedure;\nbegin\nend;\nbegin\nend.", 2, 2),
    ("routine_missing_end_semicolon", "program E;\nprocedure Q;\nbegin\nend\nbegin\nend.", 85, 5),
    ("assign_to_procedure_name", "program E;\nprocedure Q;\nbegin\n  Q := 1\nend;\nbegin\nend.", 85, 4),
    ("assign_function_result_wrong_type", "program E;\nfunction F: Boolean;\nbegin\n  F := 1\nend;\nbegin\nend.", 26, 5),
    ("assign_function_name_outside", "program E;\nfunction F: Integer;\nbegin\n  F := 1\nend;\nbegin\n  F := 2\nend.", 122, 7),
    ("procedure_in_expression", "program E;\nvar i: Integer;\nprocedure Q;\nbegin\nend;\nbegin\n  i := Q\nend.", 143, 7),
    ("function_as_statement", "program E;\nfunction F: Integer;\nbegin\n  F := 1\nend;\nbegin\n  F\nend.", 122, 8),
    ("duplicate_param", "program E;\nprocedure Q(a, a: Integer);\nbegin\nend;\nbegin\nend.", 4, 3),
    (
        "call_before_declaration",
        "program E;\nprocedure A;\nbegin\n  Q\nend;\nprocedure Q;\nbegin\nend;\nbegin\nend.",
        3,
        4,
    ),
    ("call_undeclared_with_args", "program E;\nbegin\n  Frob(1, 2)\nend.", 3, 3),
    ("abs_char", "program E;\nvar c: Char;\nbegin\n  WriteLn(Abs(c))\nend.", 79, 4),
    ("odd_boolean", "program E;\nvar b: Boolean;\nbegin\n  WriteLn(Odd(b))\nend.", 38, 4),
    ("chr_char", "program E;\nvar c: Char;\nbegin\n  WriteLn(Chr(c))\nend.", 38, 4),
    ("succ_no_arg", "program E;\nbegin\n  WriteLn(Succ)\nend.", 88, 3),
    ("succ_two_args", "program E;\nvar i: Integer;\nbegin\n  WriteLn(Succ(i, i))\nend.", 89, 4),
    ("sqr_boolean", "program E;\nvar b: Boolean;\nbegin\n  WriteLn(Sqr(b))\nend.", 79, 4),
    ("exit_with_arg", "program E;\nbegin\n  Exit(1)\nend.", 85, 3),
    ("func_missing_result_colon", "program E;\nfunction F Integer;\nbegin\nend;\nbegin\nend.", 86, 2),
    ("exit_in_const", "program Prog;\nconst A = Sqr(5);\nbegin\nend.", 133, 2),
    ("abs_boolean_const", "program Prog;\nconst A = Abs(True);\nbegin\nend.", 79, 2),
    ("arg_mismatch_second", "program Prog;\nprocedure Q(a: Integer; b: Boolean);\nbegin\nend;\nbegin\n  Q(1, 2)\nend.", 26, 6),
    ("arg_boolean_for_integer", "program Prog;\nprocedure Q(a: Integer);\nbegin\nend;\nbegin\n  Q(True)\nend.", 26, 6),
    ("function_arg_mismatch", "program Prog;\nfunction F(a: Integer): Integer;\nbegin\n  F := a\nend;\nbegin\n  WriteLn(F('x'))\nend.", 26, 7),
    ("function_too_many_args_expr", "program Prog;\nfunction F(a: Integer): Integer;\nbegin\n  F := a\nend;\nbegin\n  WriteLn(F(1, 2))\nend.", 89, 7),
    ("function_too_few_args_expr", "program Prog;\nfunction F(a, b: Integer): Integer;\nbegin\n  F := a\nend;\nbegin\n  WriteLn(F(1))\nend.", 87, 7),
    ("function_no_args_expr", "program Prog;\nfunction F(a: Integer): Integer;\nbegin\n  F := a\nend;\nbegin\n  WriteLn(F)\nend.", 88, 7),
    ("function_parens_on_noparam_expr", "program Prog;\nfunction F: Integer;\nbegin\n  F := 1\nend;\nbegin\n  WriteLn(F(1))\nend.", 89, 7),
    ("function_result_wrong_type_bool_int", "program Prog;\nfunction F: Integer;\nbegin\n  F := 1 = 1\nend;\nbegin\nend.", 26, 5),
    ("function_result_const_range", "program Prog;\nfunction F: Byte;\nbegin\n  F := 300\nend;\nbegin\nend.", 76, 5),
    ("call_function_result_to_wrong_var", "program Prog;\nvar b: Boolean;\nfunction F: Integer;\nbegin\n  F := 1\nend;\nbegin\n  b := F\nend.", 26, 9),
    ("procedure_value_in_condition", "program Prog;\nprocedure Q;\nbegin\nend;\nbegin\n  if Q then WriteLn\nend.", 143, 6),
    ("exit_in_expression", "program Prog;\nvar i: Integer;\nbegin\n  i := Exit\nend.", 42, 4),
    ("undeclared_in_routine_body", "program Prog;\nprocedure Q;\nbegin\n  zz := 1\nend;\nbegin\nend.", 3, 4),
    ("local_not_visible_outside", "program Prog;\nprocedure Q;\nvar loc: Integer;\nbegin\nend;\nbegin\n  loc := 1\nend.", 3, 7),
    ("param_not_visible_outside", "program Prog;\nprocedure Q(par: Integer);\nbegin\nend;\nbegin\n  par := 1\nend.", 3, 6),
    ("routine_dup_with_variable", "program Prog;\nvar Q: Integer;\nprocedure Q;\nbegin\nend;\nbegin\nend.", 4, 3),
    ("routine_dup_with_const", "program Prog;\nconst Q = 1;\nprocedure Q;\nbegin\nend;\nbegin\nend.", 4, 3),
    ("var_dup_with_routine", "program Prog;\nprocedure Q;\nbegin\nend;\nvar Q: Integer;\nbegin\nend.", 4, 5),
    ("routine_named_like_program", "program Prog;\nprocedure Prog;\nbegin\nend;\nbegin\nend.", 4, 2),
    ("forward_function_result_mismatch", "program Prog;\nfunction F: Integer; forward;\nfunction F: Boolean;\nbegin\nend;\nbegin\nend.", 131, 3),
    ("forward_param_count_mismatch", "program Prog;\nprocedure Q(a: Integer); forward;\nprocedure Q(a, b: Integer);\nbegin\nend;\nbegin\nend.", 131, 3),
    ("forward_procedure_then_function", "program Prog;\nprocedure Q; forward;\nfunction Q: Integer;\nbegin\nend;\nbegin\nend.", 4, 3),
    ("forward_without_semicolon", "program Prog;\nprocedure Q forward;\nbegin\nend.", 85, 2),
    ("procedure_forward_then_body_missing", "program Prog;\nprocedure Q;\nforward\nbegin\nend.", 85, 4),
    ("routine_body_missing_end", "program Prog;\nprocedure Q;\nbegin\n  WriteLn\nbegin\nend.", 85, 5),
    ("routine_unclosed_at_eof", "program Prog;\nprocedure Q;\nbegin\n  WriteLn;\n", 10, 5),
    ("program_missing_body_after_routines", "program Prog;\nprocedure Q;\nbegin\nend;\n", 10, 5),
    ("param_list_empty", "program Prog;\nprocedure Q();\nbegin\nend;\nbegin\nend.", 2, 2),
    ("param_list_trailing_semicolon", "program Prog;\nprocedure Q(a: Integer;);\nbegin\nend;\nbegin\nend.", 2, 2),
    ("param_default_char_type_mismatch", "program Prog;\nprocedure Q(c: Char);\nbegin\nend;\nbegin\n  Q(65)\nend.", 26, 6),
    ("call_const_as_procedure", "program Prog;\nconst K = 1;\nbegin\n  K\nend.", 113, 4),
];

/// TP accepts these; the compiler reports 200 (decision 6) on the given line.
const UNSUPPORTED: &[(&str, &str, u32)] = &[];

fn tmp_dir() -> PathBuf {
    crate::common::fresh_dir("integers")
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

#[test]
fn target_lang_programs_match_tp5_output() {
    let fails = check_lang(&subset("lang", LANG, &["pas", "out"]), &real_compile);
    assert!(
        fails.is_empty(),
        "{} of {}:\n{}",
        fails.len(),
        LANG.len(),
        fails.join("\n")
    );
}

#[test]
fn target_neg_programs_report_tp5_number_and_line() {
    let fails = check_neg(&subset("neg", NEG, &["pas", "err"]), &real_compile);
    assert!(
        fails.is_empty(),
        "{} of {}:\n{}",
        fails.len(),
        NEG.len(),
        fails.join("\n")
    );
}

fn run_wasm(name: &str, wasm: &[u8]) -> Result<Vec<u8>, String> {
    let path = tmp_dir().join([name, ".wasm"].concat());
    std::fs::write(&path, wasm).unwrap();
    let out = Command::new("node")
        .arg(root().join("tests/run.mjs"))
        .arg(&path)
        .output()
        .expect("node must be installed");
    if out.status.success() {
        Ok(out.stdout)
    } else {
        Err(String::from_utf8_lossy(&out.stderr).into_owned())
    }
}

#[test]
fn programs_print_what_tp5_prints() {
    let mut fails = Vec::new();
    for &(name, src, want) in RUN {
        let src = [src, "\n"].concat();
        match tp5c::compile(src.as_bytes()) {
            Err(d) => fails.push(format!(
                "{name}: rejected with {} at {}:{}",
                d.code, d.line, d.col
            )),
            Ok(wasm) => match run_wasm(name, &wasm) {
                Err(e) => fails.push(format!("{name}: run failed: {e}")),
                Ok(got) if got != want.as_bytes() => fails.push(format!(
                    "{name}: want {want:?}, got {:?}",
                    String::from_utf8_lossy(&got)
                )),
                Ok(_) => {}
            },
        }
    }
    assert!(fails.is_empty(), "{}", fails.join("\n"));
}

#[test]
fn invalid_programs_report_tp5_number_and_line() {
    let mut fails = Vec::new();
    for &(name, src, code, line) in ERR {
        let src = [src, "\n"].concat();
        match tp5c::compile(src.as_bytes()) {
            Ok(_) => fails.push(format!(
                "{name}: accepted, want error {code} at line {line}"
            )),
            Err(d) if (d.code, d.line) != (code, line) => fails.push(format!(
                "{name}: want error {code} at line {line}, got {} at line {}",
                d.code, d.line
            )),
            Err(_) => {}
        }
    }
    assert!(fails.is_empty(), "{}", fails.join("\n"));
}

#[test]
fn unsupported_features_report_error_200() {
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

#[test]
fn every_run_case_prints_something() {
    assert!(RUN.iter().all(|r| !r.2.is_empty()));
}

/// Slow (DOSBox-X): `cargo test -- --ignored`.
#[test]
#[ignore]
fn oracle_agrees() {
    let dir = tmp_dir();
    let tp5 = |name: &str, src: &str| {
        let p = dir.join([name, ".pas"].concat());
        std::fs::write(&p, [src, "\n"].concat()).unwrap();
        let o = Command::new(root().join("tools/tp5.sh"))
            .arg(&p)
            .output()
            .unwrap();
        String::from_utf8_lossy(&o.stdout).into_owned()
    };
    let mut fails = Vec::new();
    for &(name, src, want) in RUN {
        let got = tp5(name, src);
        if got != want {
            fails.push(format!("{name}: tp5 printed {got:?}, table has {want:?}"));
        }
    }
    for &(name, src, code, line) in ERR {
        let got = tp5(name, src);
        let want = format!("P.PAS({line}): Error {code}:");
        if !got.starts_with(&want) {
            fails.push(format!("{name}: tp5 said {got:?}, table has {want}"));
        }
    }
    assert!(fails.is_empty(), "{}", fails.join("\n"));
}
