//! Bulk-memory opcodes for aggregate copies (decision 12), pinned as raw bytes before the emitter
//! gets helpers: `memory.fill` is `FC 0B 00`, `memory.copy` is `FC 0A 00 00`. The module is
//! validated and run under Node (bulk memory is on by default).
mod common;
use common::*;
use tp5c::wasm::*;

const FILL: [u8; 3] = [0xFC, 0x0B, 0x00];
const COPY: [u8; 4] = [0xFC, 0x0A, 0x00, 0x00];

/// `main` runs `body`; the module exports its memory.
fn module(body: impl FnOnce(&mut Code)) -> Vec<u8> {
    let mut m = Module::new();
    let ty = m.ty(&[], &[]);
    m.memory(1);
    let mut c = Code::new();
    body(&mut c);
    let f = m.func(ty, &[], c);
    m.export("memory", MEM, 0);
    m.export("main", FUNC, f);
    m.finish()
}

fn raw(c: &mut Code, bytes: &[u8]) {
    for &b in bytes {
        c.op(b);
    }
}

fn run_and_dump(wasm: &[u8], from: u32, to: u32) -> String {
    let js = format!(
        "const i = new WebAssembly.Instance(new WebAssembly.Module(b), {{}}); i.exports.main(); \
         console.log(Array.from(new Uint8Array(i.exports.memory.buffer, {from}, {to} - {from})).join(','))"
    );
    node(&js, wasm).trim().to_string()
}

#[test]
fn memory_fill_and_copy_validate_and_run() {
    let m = module(|c| {
        // fill [100, 104) with 7, then copy it to [200, 204)
        c.i32_const(100).i32_const(7).i32_const(4);
        raw(c, &FILL);
        c.i32_const(200).i32_const(100).i32_const(4);
        raw(c, &COPY);
    });
    assert!(validates(&m));
    assert_eq!(run_and_dump(&m, 99, 106), "0,7,7,7,7,0,0");
    assert_eq!(run_and_dump(&m, 199, 206), "0,7,7,7,7,0,0");
}

#[test]
fn memory_copy_handles_overlap_both_ways() {
    let init = |c: &mut Code| {
        for (i, v) in [1, 2, 3, 4, 5].iter().enumerate() {
            c.i32_const(100 + i as i32).i32_const(*v).mem(0x3A, 0, 0);
        }
    };
    let forward = module(|c| {
        init(c);
        c.i32_const(102).i32_const(100).i32_const(5);
        raw(c, &COPY);
    });
    assert_eq!(run_and_dump(&forward, 100, 107), "1,2,1,2,3,4,5");
    let backward = module(|c| {
        init(c);
        c.i32_const(100).i32_const(102).i32_const(3);
        raw(c, &COPY);
    });
    assert_eq!(run_and_dump(&backward, 100, 105), "3,4,5,4,5");
}

#[test]
fn zero_length_copy_and_fill_are_no_ops() {
    let m = module(|c| {
        c.i32_const(300).i32_const(9).i32_const(0);
        raw(c, &FILL);
        c.i32_const(300).i32_const(310).i32_const(0);
        raw(c, &COPY);
    });
    assert!(validates(&m));
    assert_eq!(run_and_dump(&m, 299, 302), "0,0,0");
}

#[test]
fn out_of_bounds_copy_traps() {
    let m = module(|c| {
        c.i32_const(65530).i32_const(0).i32_const(100);
        raw(c, &COPY);
    });
    assert!(validates(&m));
    let js = "const i = new WebAssembly.Instance(new WebAssembly.Module(b), {}); \
              try { i.exports.main(); console.log('no trap') } catch (e) { console.log(e instanceof WebAssembly.RuntimeError) }";
    assert_eq!(node(js, &m).trim(), "true");
}

#[test]
fn emitter_helper_writes_the_pinned_bytes() {
    let mut a = Code::new();
    a.memory_copy();
    let mut b = Code::new();
    raw(&mut b, &COPY);
    let (ma, mb) = (module(|c| *c = a), module(|c| *c = b));
    assert_eq!(ma, mb);
}
