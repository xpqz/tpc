//! The embedded System unit (decision 11): `runtime/system.pas` is parsed as a declaration part in
//! an outer scope; programs may redeclare its names; helpers the compiler calls implicitly are
//! bound to symbol handles right after it is loaded. Expected output and `N LINE` of the corpus
//! programs in tests/fixtures/system come from the real TP 5.0 (`tools/tp5.sh`). Programs for a
//! synthetic System (the `compile_with` hook) have no TP 5.0 counterpart, except
//! `hook/synthetic`, whose `.out` came from the real TP 5.0 on the System text and the program
//! joined into one source.
mod common;
mod driver;
mod sysutil;
use driver::*;
use sysutil::{compile_with, contains, run, system_pas};

const HELLO: &str = "program Hello;\nbegin\n  WriteLn('hello')\nend.";

/// A System that only provides the helpers the compiler binds (stubs; `AbsLong` is observable).
const HELPER_ONLY: &str = "function AbsLong(x: LongInt): LongInt;\nbegin\n  AbsLong := 777\nend;\n\
procedure StrCat(var d, a, b: String);\nbegin\nend;\n\
function StrCmp(var a, b: String): Integer;\nbegin\n  StrCmp := 0\nend;\n\
procedure StrAssign(var d, s: String; max: Integer);\nbegin\nend;\n\
function StrDelete(s: String; i, n: Integer): String;\nbegin\n  StrDelete := s\nend;\n\
function StrInsert(src, s: String; i: Integer): String;\nbegin\n  StrInsert := s\nend;\n\
var HeapOrg, HeapPtr: LongInt;\n\
function HeapGet(n: LongInt): Pointer;\nbegin\n  HeapGet := nil\nend;\n\
procedure HeapPut(p: Pointer; n: LongInt);\nbegin\nend;\n\
function HeapMark: Pointer;\nbegin\n  HeapMark := nil\nend;\n\
procedure HeapRelease(p: Pointer);\nbegin\nend;\n\
type SetBuf = array[0..31] of Byte;\n\
procedure SetOp(var d, a, b: SetBuf; op: Integer);\nbegin\nend;\n\
procedure SetAdd(var d: SetBuf; lo, hi: Integer);\nbegin\nend;\n\
function SetIn(var s: SetBuf; i: Integer): Boolean;\nbegin\n  SetIn := False\nend;\n\
function SetCmp(var a, b: SetBuf): Integer;\nbegin\n  SetCmp := 0\nend;\n\
function StrInt(v: LongInt; w, max: Integer): String;\nbegin\n  StrInt := ''\nend;\n\
function ValLong(s: String; var code: Integer): LongInt;\nbegin\n  ValLong := 0\nend;\n\
function ValReal(s: String; var code: Integer): Real;\nbegin\n  ValReal := 0\nend;\n\
function RangeChk(v, lo, hi: LongInt): LongInt;\nbegin\n  RangeChk := v\nend;\n\
procedure HostWrite(fd, address, count: LongInt); external 'write';\nprocedure RtError(code: LongInt); external;\nfunction Fmt_Real(x: Real; width, decimals, buffer: LongInt): LongInt; external;\n\
type Text = record handle: LongInt; end;\nconst Input: Text = (handle: 0);\nprocedure TextOut(target, address, count: LongInt);\nbegin\n  HostWrite(1, address, count)\nend;\nprocedure TextNewLine(target: LongInt);\nvar c: Char;\nbegin\n  c := Chr(10);\n  HostWrite(1, LongInt(@c), 1)\nend;\nprocedure CheckIo;\nbegin\nend;\nprocedure Assign(var f: Text; name: String);\nbegin\nend;\nfunction Eof(var f: Text): Boolean;\nbegin\n  Eof := True\nend;\nfunction Eoln(var f: Text): Boolean;\nbegin\n  Eoln := True\nend;\nfunction ReadChar(var f: Text): Char;\nbegin\n  ReadChar := 'x'\nend;\nfunction ReadLong(var f: Text): LongInt;\nbegin\n  ReadLong := 0\nend;\nfunction ReadReal(var f: Text): Real;\nbegin\n  ReadReal := 0\nend;\nfunction ReadStr(var f: Text; max: Integer): String;\nbegin\n  ReadStr := ''\nend;\nprocedure ReadLnEnd(var f: Text);\nbegin\nend;\n";

fn fixtures(sub: &str) -> std::path::PathBuf {
    driver::root()
        .join("compiler/tests/fixtures/system")
        .join(sub)
}

fn assert_clean(failures: Vec<String>) {
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn system_pas_exists_and_is_embedded() {
    let file = system_pas();
    assert!(!file.is_empty(), "runtime/system.pas is empty");
    assert_eq!(
        tp5c::compile(HELLO.as_bytes()),
        compile_with(&file, HELLO),
        "the embedded System differs from runtime/system.pas"
    );
}

#[test]
fn system_pas_compiles_cleanly() {
    let m = compile_with(&system_pas(), HELLO).expect("runtime/system.pas has a diagnostic");
    assert_eq!(run(&m), "hello\n");
    let empty = compile_with(&system_pas(), "program P;\nbegin\nend.");
    assert!(empty.is_ok(), "{empty:?}");
}

#[test]
fn system_names_are_usable_and_match_tp5() {
    assert_clean(check_lang(&fixtures("lang"), &real_compile));
}

#[test]
fn system_name_errors_match_tp5() {
    assert_clean(check_neg(&fixtures("neg"), &real_compile));
}

#[test]
fn unused_system_routines_do_not_change_a_program() {
    let real = system_pas();
    let mut more = real.clone();
    more.extend_from_slice(
        b"\nconst Unused = 9;\nprocedure UnusedB(n: Integer);\nbegin\n  WriteLn('unused-system-marker', n)\nend;\n\
          function UnusedA(n: Integer): Integer;\nbegin\n  UnusedB(n);\n  UnusedA := Lo(n) + Unused\nend;\n",
    );
    let base = compile_with(&real, HELLO).unwrap();
    assert!(!base.is_empty());
    let grown = compile_with(&more, HELLO).unwrap();
    assert!(!contains(&grown, "unused-system-marker"));
    assert_eq!(base, grown, "an unused System routine changed hello-world");
    // The same holds for a program that does use other System routines.
    let uses = "program P;\nbegin\n  WriteLn(Lo(513), Hi(513))\nend.";
    assert_eq!(
        compile_with(&real, uses).unwrap(),
        compile_with(&more, uses).unwrap()
    );
}

#[test]
fn synthetic_system_constant_variable_and_routines_are_usable() {
    let sys = std::fs::read(fixtures("hook/synthetic.sys")).unwrap();
    let src = std::fs::read_to_string(fixtures("hook/synthetic.pas")).unwrap();
    let want = std::fs::read_to_string(fixtures("hook/synthetic.out")).unwrap();
    let m = compile_with(&sys, &src).expect("synthetic System program");
    assert!(!want.is_empty());
    assert_eq!(run(&m), want);
}

#[test]
fn implicit_helper_is_the_system_routine() {
    // Not TP5 behaviour: the helper returns 777 so a call through it is observable.
    let src = "program P;\nvar i: Integer;\nbegin\n  i := -5;\n  WriteLn(Abs(i))\nend.";
    let m = compile_with(HELPER_ONLY.as_bytes(), src).unwrap();
    assert_eq!(run(&m), "777\n");
}

#[test]
fn user_routine_named_like_the_helper_does_not_change_it() {
    let sys = HELPER_ONLY.as_bytes();
    let user = "program P;\nvar i: Integer;\nfunction AbsLong(x: LongInt): LongInt;\nbegin\n  AbsLong := 42\nend;\nbegin\n  i := -5;\n  WriteLn(Abs(i), ' ', AbsLong(i))\nend.";
    assert_eq!(run(&compile_with(sys, user).unwrap()), "777 42\n");
    let var = "program P;\nvar AbsLong, i: Integer;\nbegin\n  i := -5;\n  AbsLong := 3;\n  WriteLn(Abs(i), ' ', AbsLong)\nend.";
    assert_eq!(run(&compile_with(sys, var).unwrap()), "777 3\n");
    let local = "program P;\nfunction F(AbsLong: Integer): Integer;\nvar i: Integer;\nbegin\n  i := -AbsLong;\n  F := Abs(i)\nend;\nbegin\n  WriteLn(F(4))\nend.";
    assert_eq!(run(&compile_with(sys, local).unwrap()), "777\n");
}

#[test]
fn diagnostics_inside_system_have_line_zero() {
    let at = |sys: &str| compile_with(sys.as_bytes(), HELLO).expect_err(sys);
    let helper = HELPER_ONLY;
    // Semantic error (3 unknown identifier), syntax error (86), end of file (10).
    let semantic = at(&format!(
        "procedure Bad;\nbegin\n  Undeclared := 1\nend;\n{helper}"
    ));
    assert_eq!((semantic.code, semantic.line), (3, 0));
    let syntax = at(&format!("\n\nvar x Integer;\n{helper}"));
    assert_eq!((syntax.code, syntax.line), (86, 0));
    let eof = at(&format!("{helper}procedure P;\nbegin\n"));
    assert_eq!((eof.code, eof.line), (10, 0));
    // A System without its helper is a diagnostic of System too.
    let missing = at("");
    assert_eq!((missing.code, missing.line), (3, 0));
}

#[test]
fn user_lines_do_not_depend_on_the_size_of_system() {
    let big = format!("{HELPER_ONLY}procedure Pad;\nbegin\nend;\n\n\n\n\n\n");
    let src = "program P;\nbegin\n  x := 1\nend.";
    let d = compile_with(big.as_bytes(), src).unwrap_err();
    assert_eq!((d.code, d.line), (3, 3));
    let eof = compile_with(big.as_bytes(), "program P;\nbegin\n").unwrap_err();
    assert_eq!((eof.code, eof.line), (10, 2));
}
