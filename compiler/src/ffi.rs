//! Raw `extern "C"` exports of the wasm32 build. All state is safe Rust: the source buffer
//! handed out by `alloc` is the `Vec` that `compile` reads, so no raw pointers are formed.
//! `no_mangle` is the only unsafe construct and is allowed in this file alone.
#![allow(unsafe_code)]
use std::cell::RefCell;

#[derive(Default)]
struct State {
    src: Vec<u8>,
    out: Vec<u8>,
    err: crate::Diag,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
}

fn with<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with(|s| f(&mut s.borrow_mut()))
}

/// Resizes the source buffer to `n` bytes and returns its address; valid until the next call.
#[no_mangle]
pub extern "C" fn alloc(n: usize) -> *const u8 {
    with(|s| {
        s.src.clear();
        s.src.resize(n, 0);
        s.src.as_ptr()
    })
}

/// Compiles the buffer from the last `alloc`: 0 on success (output via `out_ptr`/`out_len`),
/// 1 on error (position via `err_*`) or if `src_ptr`/`src_len` is not that buffer.
#[no_mangle]
pub extern "C" fn compile(src_ptr: *const u8, src_len: usize, _flags: u32) -> u32 {
    with(|s| {
        let valid = src_ptr == s.src.as_ptr() && src_len <= s.src.len();
        let res = if valid {
            crate::compile(&s.src[..src_len])
        } else {
            Err(crate::Diag::default())
        };
        match res {
            Ok(wasm) => {
                s.out = wasm;
                0
            }
            Err(d) => {
                s.err = d;
                1
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn out_ptr() -> *const u8 {
    with(|s| s.out.as_ptr())
}

#[no_mangle]
pub extern "C" fn out_len() -> usize {
    with(|s| s.out.len())
}

#[no_mangle]
pub extern "C" fn err_code() -> u32 {
    with(|s| s.err.code as u32)
}

#[no_mangle]
pub extern "C" fn err_line() -> u32 {
    with(|s| s.err.line)
}

#[no_mangle]
pub extern "C" fn err_col() -> u32 {
    with(|s| s.err.col)
}
