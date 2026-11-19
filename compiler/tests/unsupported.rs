//! Omitted features (plan §2) report error 200 at the offending token.
use tp5c::compile;

/// Compiles `src` and expects error 200 at the first occurrence of `at`.
fn unsupported(src: &str, at: &str) {
    let off = src
        .find(at)
        .unwrap_or_else(|| panic!("no `{at}` in source"));
    let before = &src[..off];
    let line = before.matches('\n').count() as u32 + 1;
    let col = (off - before.rfind('\n').map_or(0, |i| i + 1)) as u32 + 1;
    let d = compile(src.as_bytes()).expect_err(src);
    assert_eq!((d.code, d.line, d.col), (200, line, col), "{src}");
}

#[test]
fn control_program_compiles() {
    let src = "program P;\nvar x: Integer;\nbegin\n  x := 1\nend.\n";
    assert!(compile(src.as_bytes()).is_ok());
}

#[test]
fn label_section() {
    unsupported("program P;\nlabel 10;\nbegin\nend.\n", "label");
    unsupported(
        "program P;\nvar x: Integer;\nlabel L1, L2;\nbegin\nend.\n",
        "label",
    );
    unsupported(
        "program P;\nprocedure Q;\n  label 5;\nbegin\nend;\nbegin\nend.\n",
        "label",
    );
}

#[test]
fn goto_statement() {
    unsupported("program P;\nbegin\n  goto 10\nend.\n", "goto");
    unsupported("program P;\nbegin\n  if true then goto 1\nend.\n", "goto");
    unsupported(
        "program P;\nbegin\n  while true do\n    goto 1\nend.\n",
        "goto",
    );
}

#[test]
fn asm_block() {
    unsupported("program P;\nbegin\n  asm\n    nop\n  end\nend.\n", "asm");
    unsupported(
        "program P;\nprocedure Q;\nbegin\n  asm end\nend;\nbegin\nend.\n",
        "asm",
    );
}

#[test]
fn inline_statement() {
    unsupported("program P;\nbegin\n  inline($90)\nend.\n", "inline");
}

#[test]
fn inline_routine() {
    unsupported(
        "program P;\nprocedure Q; inline($90);\nbegin\nend.\n",
        "inline",
    );
    unsupported(
        "program P;\nfunction F: Integer; inline($90);\nbegin\nend.\n",
        "inline",
    );
}

#[test]
fn interrupt_routine() {
    unsupported(
        "program P;\nprocedure Q; interrupt;\nbegin\nend;\nbegin\nend.\n",
        "interrupt",
    );
    unsupported(
        "program P;\nfunction F(a: Integer): Integer; interrupt;\nbegin\nend;\nbegin\nend.\n",
        "interrupt",
    );
}

#[test]
fn absolute_variable() {
    unsupported(
        "program P;\nvar x: Integer;\n    y: Integer absolute x;\nbegin\nend.\n",
        "absolute",
    );
    unsupported(
        "program P;\nvar y: Integer absolute $B800:0;\nbegin\nend.\n",
        "absolute",
    );
}

#[test]
fn file_types() {
    unsupported("program P;\nvar f: file;\nbegin\nend.\n", "file");
    unsupported("program P;\nvar f: file of Integer;\nbegin\nend.\n", "file");
    unsupported(
        "program P;\nprocedure Q(f: file);\nbegin\nend;\nbegin\nend.\n",
        "file",
    );
}

#[test]
fn eighty_eighty_seven_types() {
    for ty in ["Single", "Double", "Extended", "Comp"] {
        let src = format!("program P;\nvar x: {ty};\nbegin\nend.\n");
        // The checker reports a variable's type error at the declared name.
        unsupported(&src, "x:");
    }
}

#[test]
fn seg_ofs_ptr() {
    for call in ["Seg(x)", "Ofs(x)", "Ptr(1, 2)"] {
        let src = format!("program P;\nvar x, y: Integer;\nbegin\n  y := {call}\nend.\n");
        unsupported(&src, call);
    }
}

/// TP accepts `{$N+}` and `{$E+}`; 8087 code and its emulator are omitted (plan §2), so both
/// switches are error 200 in either sign, in a list too, and in either comment form.
#[test]
fn numeric_coprocessor_and_emulation_switches() {
    unsupported("program P;\n{$N+}\nbegin\nend.\n", "{$N+}");
    unsupported("program P;\n{$n-}\nbegin\nend.\n", "{$n-}");
    unsupported("program P;\n{$E+}\nbegin\nend.\n", "{$E+}");
    unsupported("program P;\n(*$E-*)\nbegin\nend.\n", "(*$E-*)");
    unsupported("program P;\n{$A+,N+}\nbegin\nend.\n", "{$A+,N+}");
}
