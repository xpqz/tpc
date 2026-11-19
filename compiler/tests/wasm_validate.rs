mod common;
use common::*;
use tp5c::wasm::*;

fn hello() -> Vec<u8> {
    let mut m = Module::new();
    let write = m.ty(&[I32, I32], &[]);
    let main = m.ty(&[], &[]);
    m.import_func("host", "write", write);
    m.memory(1);
    m.data(256, b"OK");
    let mut c = Code::new();
    c.i32_const(256).i32_const(2).idx(CALL, 0);
    let f = m.func(main, &[], c);
    m.export("memory", MEM, 0);
    m.export("main", FUNC, f);
    m.finish()
}

const RUN: &str = "const i = new WebAssembly.Instance(new WebAssembly.Module(b), IMPORTS); \
                   const e = i.exports; ";

fn js(imports: &str, body: &str) -> String {
    format!("{}{body}", RUN.replace("IMPORTS", imports))
}

#[test]
fn hello_module_exact_bytes() {
    let want = concat(&[
        &HEADER,
        &section(1, &[2, 0x60, 2, 0x7F, 0x7F, 0, 0x60, 0, 0]),
        &section(
            2,
            &[
                1, 4, b'h', b'o', b's', b't', 5, b'w', b'r', b'i', b't', b'e', 0, 0,
            ],
        ),
        &section(3, &[1, 1]),
        &section(5, &[1, 0, 1]),
        &section(
            7,
            &[
                2, 6, b'm', b'e', b'm', b'o', b'r', b'y', 2, 0, 4, b'm', b'a', b'i', b'n', 0, 1,
            ],
        ),
        &section(10, &[1, 9, 0, 0x41, 0x80, 0x02, 0x41, 2, 0x10, 0, 0x0B]),
        &section(11, &[1, 0, 0x41, 0x80, 0x02, 0x0B, 2, b'O', b'K']),
    ]);
    assert!(validates(&want), "hand-written expectation must be valid");
    assert_eq!(hello(), want);
}

#[test]
fn hello_module_validates_and_prints_ok() {
    let w = hello();
    assert!(validates(&w));
    let script = js(
        "{host: {write: (p, n) => { out += Buffer.from(i.exports.memory.buffer, p, n).toString(); }}}",
        "e.main(); console.log(out);",
    );
    let out = node(&format!("let out = ''; {script}"), &w);
    assert_eq!(out.trim(), "OK");
}

#[test]
fn imports_and_exports_are_visible_to_node() {
    let out = node(
        "const m = new WebAssembly.Module(b); \
         console.log(JSON.stringify([WebAssembly.Module.imports(m), WebAssembly.Module.exports(m)]));",
        &hello(),
    );
    assert_eq!(
        out.trim(),
        r#"[[{"module":"host","name":"write","kind":"function"}],[{"name":"memory","kind":"memory"},{"name":"main","kind":"function"}]]"#
    );
}

#[test]
fn stack_pointer_global_with_loads_and_stores() {
    let mut m = Module::new();
    let t = m.ty(&[I32], &[I32]);
    m.memory(1);
    let sp = m.global_i32(true, 65536);
    let mut c = Code::new();
    // fp = sp - 16; sp = fp
    c.idx(GLOBAL_GET, sp)
        .i32_const(16)
        .op(I32_SUB)
        .idx(LOCAL_TEE, 1)
        .idx(GLOBAL_SET, sp);
    c.idx(LOCAL_GET, 1).idx(LOCAL_GET, 0).mem(I32_STORE, 2, 0);
    c.idx(LOCAL_GET, 1)
        .idx(LOCAL_GET, 0)
        .i32_const(2)
        .op(I32_MUL)
        .mem(I32_STORE, 2, 4);
    c.idx(LOCAL_GET, 1)
        .mem(I32_LOAD, 2, 0)
        .idx(LOCAL_GET, 1)
        .mem(I32_LOAD, 2, 4)
        .op(I32_ADD);
    // sp = fp + 16
    c.idx(LOCAL_GET, 1)
        .i32_const(16)
        .op(I32_ADD)
        .idx(GLOBAL_SET, sp);
    let f = m.func(t, &[I32], c);
    m.export("f", FUNC, f);
    m.export("sp", GLOBAL, sp);
    m.export("mem", MEM, 0);
    let w = m.finish();
    assert!(validates(&w));
    let out = node(&js("{}", "console.log(e.f(7), e.sp.value, new DataView(e.mem.buffer).getInt32(65536 - 16, true));"), &w);
    assert_eq!(out.trim(), "21 65536 7");
}

#[test]
fn call_and_call_indirect_through_table_and_element_segment() {
    let mut m = Module::new();
    let un = m.ty(&[I32], &[I32]);
    let sel = m.ty(&[I32, I32], &[I32]);
    let mut c = Code::new();
    c.op(UNREACHABLE);
    let trap = m.func(un, &[], c); // table slot 0: trap stub
    let mut c = Code::new();
    c.idx(LOCAL_GET, 0).i32_const(1).op(I32_ADD);
    let inc = m.func(un, &[], c);
    let mut c = Code::new();
    c.idx(LOCAL_GET, 0).i32_const(2).op(I32_MUL);
    let dbl = m.func(un, &[], c);
    let mut c = Code::new();
    c.idx(LOCAL_GET, 1).idx(LOCAL_GET, 0).call_indirect(un);
    let ind = m.func(sel, &[], c);
    let mut c = Code::new();
    c.idx(LOCAL_GET, 0).idx(CALL, dbl).idx(CALL, inc);
    let both = m.func(un, &[], c);
    m.table(3);
    m.elem(0, &[trap, inc, dbl]);
    m.export("ind", FUNC, ind);
    m.export("both", FUNC, both);
    let w = m.finish();
    assert!(validates(&w));
    let script = js(
        "{}",
        "let t = 'no trap'; try { e.ind(0, 5); } catch (x) { t = x instanceof WebAssembly.RuntimeError; } \
         console.log(e.ind(1, 10), e.ind(2, 10), e.both(10), t);",
    );
    assert_eq!(node(&script, &w).trim(), "11 20 21 true");
}

#[test]
fn f64_arithmetic_memory_compare_and_convert() {
    let mut m = Module::new();
    let fd = m.ty(&[F64], &[F64]);
    let fi = m.ty(&[F64], &[I32]);
    let id = m.ty(&[I32], &[F64]);
    m.memory(1);
    let mut c = Code::new();
    c.idx(LOCAL_GET, 0)
        .f64_const(2.5)
        .op(F64_MUL)
        .f64_const(1.0)
        .op(F64_ADD);
    let lin = m.func(fd, &[], c);
    let mut c = Code::new();
    c.i32_const(8).idx(LOCAL_GET, 0).mem(F64_STORE, 3, 0);
    c.i32_const(8)
        .mem(F64_LOAD, 3, 0)
        .f64_const(0.5)
        .op(F64_ADD);
    let rt = m.func(fd, &[], c);
    let mut c = Code::new();
    c.idx(LOCAL_GET, 0).f64_const(3.0).op(F64_LT);
    let lt = m.func(fi, &[], c);
    let mut c = Code::new();
    c.idx(LOCAL_GET, 0)
        .op(F64_CONVERT_I32_S)
        .f64_const(1.5)
        .op(F64_MUL);
    let cv = m.func(id, &[], c);
    for (n, f) in [("lin", lin), ("rt", rt), ("lt", lt), ("cv", cv)] {
        m.export(n, FUNC, f);
    }
    let w = m.finish();
    assert!(validates(&w));
    let out = node(
        &js(
            "{}",
            "console.log(e.lin(4), e.rt(-2.25), e.lt(2.5), e.lt(3), e.cv(-3));",
        ),
        &w,
    );
    assert_eq!(out.trim(), "11 -1.75 1 0 -4.5");
}

#[test]
fn if_else_with_result_and_loop_with_exit_block() {
    let mut m = Module::new();
    let t = m.ty(&[I32], &[I32]);
    let mut c = Code::new();
    c.idx(LOCAL_GET, 0).i32_const(0).op(I32_LT_S);
    c.if_(I32);
    c.i32_const(-1).else_().i32_const(1).end();
    let sign = m.func(t, &[], c);
    let mut c = Code::new();
    let exit = c.block(VOID);
    let top = c.loop_(VOID);
    c.idx(LOCAL_GET, 0).op(I32_EQZ).br_if(exit);
    c.idx(LOCAL_GET, 1)
        .idx(LOCAL_GET, 0)
        .op(I32_ADD)
        .idx(LOCAL_SET, 1);
    c.idx(LOCAL_GET, 0)
        .i32_const(1)
        .op(I32_SUB)
        .idx(LOCAL_SET, 0);
    c.br(top);
    c.end().end();
    c.idx(LOCAL_GET, 1);
    let sum = m.func(t, &[I32], c);
    m.export("sign", FUNC, sign);
    m.export("sum", FUNC, sum);
    let w = m.finish();
    assert!(validates(&w));
    let out = node(
        &js(
            "{}",
            "console.log(e.sign(-9), e.sign(0), e.sign(9), e.sum(10), e.sum(0));",
        ),
        &w,
    );
    assert_eq!(out.trim(), "-1 1 1 55 0");
}

#[test]
fn i32_constants_round_trip_through_node() {
    let mut m = Module::new();
    let t32 = m.ty(&[], &[I32]);
    let mut names = Vec::new();
    for (i, v) in [0, -1, 63, 64, -64, -65, i32::MIN, i32::MAX]
        .into_iter()
        .enumerate()
    {
        let mut c = Code::new();
        c.i32_const(v);
        let f = m.func(t32, &[], c);
        m.export(&format!("a{i}"), FUNC, f);
        names.push(format!("e.a{i}()"));
    }
    let w = m.finish();
    assert!(validates(&w));
    let out = node(
        &js(
            "{}",
            &format!("console.log([{}].join(' '))", names.join(", ")),
        ),
        &w,
    );
    assert_eq!(out.trim(), "0 -1 63 64 -64 -65 -2147483648 2147483647");
}

#[test]
fn harness_rejects_a_function_body_missing_its_final_end() {
    let head = concat(&[&HEADER, &section(1, &[1, 0x60, 0, 0]), &section(3, &[1, 0])]);
    let good = concat(&[&head, &section(10, &[1, 3, 0, 0x01, 0x0B])]);
    let bad = concat(&[&head, &section(10, &[1, 3, 0, 0x01, 0x01])]);
    assert!(validates(&good));
    assert!(!validates(&bad));
}

#[test]
fn harness_rejects_an_unbalanced_block_in_a_body() {
    let head = concat(&[&HEADER, &section(1, &[1, 0x60, 0, 0]), &section(3, &[1, 0])]);
    // block end <no function end>: the second 0x0B closes the block, the body is unterminated.
    let bad = concat(&[&head, &section(10, &[1, 5, 0, 0x02, 0x40, 0x0B, 0x01])]);
    let extra_end = concat(&[&head, &section(10, &[1, 3, 0, 0x0B, 0x0B])]);
    assert!(!validates(&bad));
    assert!(!validates(&extra_end));
}
