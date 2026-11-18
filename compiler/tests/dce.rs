//! Function-level dead-code elimination (decision 14): bodies are emitted from a worklist rooted
//! at the main program and at every address-taken routine; string literals only when a live body
//! refers to them. Expected program output comes from the real TP 5.0 for the corpus programs;
//! here the programs are small and their output is fixed by construction.
mod sysutil;
use sysutil::*;

const MAIN: &str = "begin\n  WriteLn('main-marker-live')\nend.";

/// (name, program without the unused code, the same program with it).
fn pairs() -> Vec<(&'static str, String, String)> {
    let with = |decls: &str| format!("program P;\n{decls}{MAIN}");
    let rows: [(&str, &str, &str); 8] = [
        (
            "procedure",
            "",
            "procedure Dead;\nbegin\nend;\n",
        ),
        (
            "function with literal",
            "",
            "function Dead(n: Integer): Integer;\nbegin\n  WriteLn('dead-marker-fn');\n  Dead := n + 1\nend;\n",
        ),
        (
            "chain of unused routines",
            "",
            "procedure D2;\nbegin\n  WriteLn('dead-marker-d2')\nend;\nprocedure D1;\nbegin\n  D2\nend;\nprocedure D0;\nbegin\n  D1; D2\nend;\n",
        ),
        (
            "unused recursion",
            "",
            "procedure Dead(n: Integer);\nbegin\n  if n > 0 then Dead(n - 1)\nend;\n",
        ),
        (
            "unused mutual recursion through forward",
            "",
            "procedure B(n: Integer); forward;\nprocedure A(n: Integer);\nbegin\n  if n > 0 then B(n - 1)\nend;\nprocedure B(n: Integer);\nbegin\n  if n > 0 then A(n - 1)\nend;\n",
        ),
        (
            "unused with frame, var parameter and nested routine",
            "",
            "procedure Dead(var x: Integer);\n  var y: Integer;\n  procedure Inner;\n  begin\n    y := y + 1; x := y\n  end;\nbegin\n  Inner\nend;\n",
        ),
        (
            "between global variables",
            "var a: Integer;\n    b: LongInt;\n",
            "var a: Integer;\nprocedure Dead;\nbegin\n  WriteLn('dead-marker-between')\nend;\nvar b: LongInt;\n",
        ),
        (
            "unused function used only by another unused function",
            "const K = 3;\n",
            "const K = 3;\nfunction Inner(n: Integer): Integer;\nbegin\n  Inner := n * K\nend;\nfunction Outer(n: Integer): Integer;\nbegin\n  Outer := Inner(n) + Inner(1)\nend;\n",
        ),
    ];
    rows.iter()
        .map(|&(n, base, extra)| (n, with(base), with(extra)))
        .collect()
}

#[test]
fn unused_routines_leave_the_module_bytes_unchanged() {
    for (name, base, extra) in pairs() {
        let (a, b) = (compile(&base), compile(&extra));
        assert!(!a.is_empty(), "{name}");
        assert_eq!(a, b, "{name}: an unreferenced routine changed the module");
    }
}

#[test]
fn unused_routines_literals_are_not_in_the_module() {
    for (name, _, extra) in pairs() {
        let m = compile(&extra);
        assert!(
            contains(&m, "main-marker-live"),
            "{name}: live literal missing"
        );
        assert!(
            !contains(&m, "dead-marker"),
            "{name}: literal of dead code kept"
        );
    }
}

#[test]
fn referenced_routines_are_emitted_and_run() {
    let used = "program P;\nprocedure C;\nbegin\n  WriteLn('c-marker')\nend;\nfunction B(n: Integer): Integer;\nbegin\n  C;\n  B := n + 1\nend;\nprocedure A;\nbegin\n  WriteLn(B(1))\nend;\nbegin\n  A\nend.";
    let m = compile(used);
    assert_eq!(run(&m), "c-marker\n2\n");
    assert!(contains(&m, "c-marker"));
    let unused = "program P;\nprocedure C;\nbegin\n  WriteLn('c-marker')\nend;\nfunction B(n: Integer): Integer;\nbegin\n  C;\n  B := n + 1\nend;\nprocedure A;\nbegin\n  WriteLn(B(1))\nend;\nbegin\nend.";
    let n = compile(unused);
    assert!(
        n.len() < m.len(),
        "dropping the call chain must shrink the module"
    );
    assert!(!contains(&n, "c-marker"));
    assert_eq!(run(&n), "");
}

#[test]
fn recursion_and_forward_routines_reached_from_main_work() {
    let src = "program P;\nvar n: Integer;\nprocedure B(k: Integer); forward;\nprocedure A(k: Integer);\nbegin\n  if k > 0 then begin Write('a'); B(k - 1) end\nend;\nprocedure B(k: Integer);\nbegin\n  if k > 0 then begin Write('b'); A(k - 1) end\nend;\nfunction Fact(k: Integer): Integer;\nbegin\n  if k <= 1 then Fact := 1 else Fact := k * Fact(k - 1)\nend;\nbegin\n  A(5);\n  WriteLn;\n  WriteLn(Fact(6))\nend.";
    assert_eq!(run(&compile(src)), "ababa\n720\n");
}

#[test]
fn address_taken_routines_are_kept() {
    let prog = |dead_decls: &str, main_body: &str| {
        format!(
            "program P;\ntype Proc = procedure;\nvar ptr: Proc;\nprocedure Target;\nbegin\n  WriteLn('target-marker')\nend;\n{dead_decls}begin\n{main_body}end."
        )
    };
    // Taken and called.
    let called = compile(&prog("", "  ptr := Target;\n  ptr\n"));
    assert_eq!(run(&called), "target-marker\n");
    // Taken but never called: still emitted (every address-taken routine is a root).
    let taken = compile(&prog("", "  ptr := Target\n"));
    assert!(contains(&taken, "target-marker"));
    assert!(run_validate(&taken));
    // Taken only by a routine that is itself dead: kept conservatively, the dead taker is not.
    let dead_taker = "procedure Taker;\nbegin\n  WriteLn('taker-marker');\n  ptr := Target\nend;\n";
    let kept = compile(&prog(dead_taker, ""));
    assert!(contains(&kept, "target-marker"));
    assert!(!contains(&kept, "taker-marker"));
    assert!(run_validate(&kept));
    // An untaken routine next to it is dropped.
    let untaken = "procedure Untaken;\nbegin\n  WriteLn('untaken-marker')\nend;\n";
    let m = compile(&prog(untaken, "  ptr := Target;\n  ptr\n"));
    assert!(!contains(&m, "untaken-marker"));
    assert_eq!(m, called, "an unreferenced routine changed the module");
    assert_eq!(run(&m), "target-marker\n");
}

#[test]
fn indirect_calls_survive_elimination_with_several_signatures() {
    let src = "program P;\ntype Fn = function(a: Integer): Integer; Pr = procedure(var v: Integer);\nvar f: Fn; g: Pr; n: Integer;\nfunction Inc(a: Integer): Integer;\nbegin\n  Inc := a + 1\nend;\nfunction Dead(a: Integer): Integer;\nbegin\n  Dead := a - 1\nend;\nprocedure Dbl(var v: Integer);\nbegin\n  v := v * 2\nend;\nbegin\n  f := Inc; g := Dbl;\n  n := f(20);\n  g(n);\n  WriteLn(n)\nend.";
    let m = compile(src);
    assert_eq!(run(&m), "42\n");
    assert!(run_validate(&m));
    let without_dead = src.replace(
        "function Dead(a: Integer): Integer;\nbegin\n  Dead := a - 1\nend;\n",
        "",
    );
    assert_eq!(m, compile(&without_dead));
}

#[test]
fn function_indices_follow_first_reference_not_declaration_order() {
    let body =
        |name: &str, text: &str| format!("procedure {name};\nbegin\n  WriteLn('{text}')\nend;\n");
    let (x, y) = (body("X", "x-text"), body("Y", "y-text"));
    let main = "begin\n  Y;\n  X\nend.";
    let xy = compile(&format!("program P;\n{x}{y}{main}"));
    let yx = compile(&format!("program P;\n{y}{x}{main}"));
    assert_eq!(run(&xy), "y-text\nx-text\n");
    assert_eq!(xy, yx, "declaration order must not reach the module");
}

#[test]
fn eliminated_modules_validate() {
    for (name, base, extra) in pairs() {
        assert!(run_validate(&compile(&base)), "{name}");
        assert!(run_validate(&compile(&extra)), "{name}");
    }
}

#[test]
fn unused_routine_calling_system_routines_leaves_the_module_unchanged() {
    let base = "program P;\nbegin\n  WriteLn('main-marker-live')\nend.";
    let dead = "program P;\nfunction Dead(n: Integer): Integer;\nbegin\n  Dead := Lo(n) + Hi(n) + Abs(n) + Ord(UpCase('a'))\nend;\nbegin\n  WriteLn('main-marker-live')\nend.";
    assert_eq!(compile(base), compile(dead));
}
