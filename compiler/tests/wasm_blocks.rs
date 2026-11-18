mod common;
use common::*;
use tp5c::wasm::*;

/// Module with one ()->() function whose code-section entry is `locals_and_body`.
fn expected(body: &[u8]) -> Vec<u8> {
    let mut entry = vec![0]; // no locals
    entry.extend_from_slice(body);
    entry.push(0x0B); // function end
    let mut code = vec![1, entry.len() as u8];
    code.extend(entry);
    concat(&[
        &HEADER,
        &section(1, &[1, 0x60, 0, 0]),
        &section(3, &[1, 0]),
        &section(10, &code),
    ])
}

fn build(f: impl FnOnce(&mut Code)) -> Vec<u8> {
    let mut m = Module::new();
    let t = m.ty(&[], &[]);
    let mut c = Code::new();
    f(&mut c);
    m.func(t, &[], c);
    m.finish()
}

#[test]
fn br_from_three_deep_nesting_uses_relative_depths() {
    let got = build(|c| {
        let a = c.block(VOID);
        let b = c.block(VOID);
        let d = c.block(VOID);
        c.br(a).br(b).br(d);
        c.end().end().end();
    });
    let want = expected(&[
        0x02, 0x40, 0x02, 0x40, 0x02, 0x40, 0x0C, 2, 0x0C, 1, 0x0C, 0, 0x0B, 0x0B, 0x0B,
    ]);
    assert!(validates(&want), "hand-written expectation must be valid");
    assert_eq!(got, want);
    assert!(validates(&got));
}

#[test]
fn br_depth_depends_on_the_current_depth_not_the_label_alone() {
    let got = build(|c| {
        let a = c.block(VOID);
        c.br(a); // depth 0
        let b = c.block(VOID);
        c.br(a); // depth 1
        c.br(b); // depth 0
        c.end();
        c.br(a); // back to depth 0
        c.end();
    });
    let want = expected(&[
        0x02, 0x40, 0x0C, 0, 0x02, 0x40, 0x0C, 1, 0x0C, 0, 0x0B, 0x0C, 0, 0x0B,
    ]);
    assert!(validates(&want), "hand-written expectation must be valid");
    assert_eq!(got, want);
    assert!(validates(&got));
}

#[test]
fn sibling_blocks_at_the_same_depth_reuse_depth_zero() {
    let got = build(|c| {
        let a = c.block(VOID);
        c.br(a).end();
        let b = c.block(VOID);
        c.br(b).end();
    });
    let want = expected(&[0x02, 0x40, 0x0C, 0, 0x0B, 0x02, 0x40, 0x0C, 0, 0x0B]);
    assert!(validates(&want), "hand-written expectation must be valid");
    assert_eq!(got, want);
}

#[test]
fn br_if_and_loop_labels() {
    // block $exit; loop $top; i32.const 1; br_if $exit; br $top; end; end
    let got = build(|c| {
        let exit = c.block(VOID);
        let top = c.loop_(VOID);
        c.i32_const(1).br_if(exit);
        c.br(top);
        c.end().end();
    });
    let want = expected(&[
        0x02, 0x40, 0x03, 0x40, 0x41, 1, 0x0D, 1, 0x0C, 0, 0x0B, 0x0B,
    ]);
    assert!(validates(&want), "hand-written expectation must be valid");
    assert_eq!(got, want);
    assert!(validates(&got));
}

#[test]
fn if_else_keeps_its_label_across_else() {
    // i32.const 0; if $i; br $i; else; br $i; end
    let got = build(|c| {
        c.i32_const(0);
        let i = c.if_(VOID);
        c.br(i);
        c.else_();
        c.br(i);
        c.end();
    });
    let want = expected(&[0x41, 0, 0x04, 0x40, 0x0C, 0, 0x05, 0x0C, 0, 0x0B]);
    assert!(validates(&want), "hand-written expectation must be valid");
    assert_eq!(got, want);
    assert!(validates(&got));
}

#[test]
fn block_types_are_encoded() {
    let got = build(|c| {
        c.block(I32);
        c.i32_const(7);
        c.end().op(0x1A); // drop
        c.block(F64);
        c.f64_const(0.0);
        c.end().op(0x1A);
    });
    let want = expected(&[
        0x02, 0x7F, 0x41, 7, 0x0B, 0x1A, 0x02, 0x7C, 0x44, 0, 0, 0, 0, 0, 0, 0, 0, 0x0B, 0x1A,
    ]);
    assert!(validates(&want), "hand-written expectation must be valid");
    assert_eq!(got, want);
    assert!(validates(&got));
}

#[test]
fn exit_block_around_a_whole_body_like_a_pascal_exit() {
    // Every routine body is `block $exit ... end`; br from inside nested ifs reaches it.
    let got = build(|c| {
        let exit = c.block(VOID);
        c.i32_const(1);
        c.if_(VOID);
        c.i32_const(1);
        c.if_(VOID);
        c.br(exit);
        c.end();
        c.end();
        c.end();
    });
    let want = expected(&[
        0x02, 0x40, 0x41, 1, 0x04, 0x40, 0x41, 1, 0x04, 0x40, 0x0C, 2, 0x0B, 0x0B, 0x0B,
    ]);
    assert!(validates(&want), "hand-written expectation must be valid");
    assert_eq!(got, want);
    assert!(validates(&got));
}

#[test]
fn br_table_encodes_count_relative_depths_and_default() {
    let got = build(|c| {
        let a = c.block(VOID);
        let b = c.block(VOID);
        c.i32_const(0).br_table(&[a, b], a);
        c.end().end();
    });
    let want = expected(&[
        0x02, 0x40, 0x02, 0x40, 0x41, 0, 0x0E, 2, 1, 0, 1, 0x0B, 0x0B,
    ]);
    assert!(validates(&want), "hand-written expectation must be valid");
    assert_eq!(got, want);
    assert!(validates(&got));
}

#[test]
fn br_table_with_no_targets_has_only_a_default() {
    let got = build(|c| {
        let a = c.block(VOID);
        c.i32_const(0).br_table(&[], a);
        c.end();
    });
    let want = expected(&[0x02, 0x40, 0x41, 0, 0x0E, 0, 0, 0x0B]);
    assert!(validates(&want), "hand-written expectation must be valid");
    assert_eq!(got, want);
}

#[test]
#[should_panic(expected = "label")]
fn br_table_with_a_closed_label_is_rejected() {
    let mut c = Code::new();
    let a = c.block(VOID);
    c.end();
    c.br_table(&[], a);
}

#[test]
#[should_panic(expected = "unbalanced")]
fn end_without_open_block_is_rejected() {
    let mut c = Code::new();
    c.end();
}

#[test]
#[should_panic(expected = "unbalanced")]
fn function_with_open_block_is_rejected() {
    let mut m = Module::new();
    let t = m.ty(&[], &[]);
    let mut c = Code::new();
    c.block(VOID);
    m.func(t, &[], c);
}

#[test]
#[should_panic(expected = "unbalanced")]
fn extra_end_after_balanced_blocks_is_rejected() {
    let mut c = Code::new();
    c.block(VOID);
    c.end();
    c.end();
}

#[test]
#[should_panic(expected = "label")]
fn br_to_a_label_that_is_already_closed_is_rejected() {
    let mut c = Code::new();
    let a = c.block(VOID);
    c.end();
    c.br(a);
}

#[test]
#[should_panic(expected = "else")]
fn else_outside_if_is_rejected() {
    let mut c = Code::new();
    c.block(VOID);
    c.else_();
}
