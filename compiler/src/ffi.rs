//! Raw `extern "C"` exports of the wasm32 build. All state is safe Rust: the source buffer
//! handed out by `alloc` is the `Vec` that `compile` reads, so no raw pointers are formed.
//! `no_mangle` is the only unsafe construct and is allowed in this file alone.
#![allow(unsafe_code)]
use std::cell::RefCell;

#[derive(Default)]
struct State {
    src: Vec<u8>,
    /// Registered source files; they stay registered across `compile` calls.
    files: Vec<crate::SourceFile>,
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

/// The `len` bytes at `ptr` if they lie inside `buffer`; only offsets are computed.
fn within(buffer: &[u8], ptr: *const u8, len: usize) -> Option<&[u8]> {
    let start = (ptr as usize).checked_sub(buffer.as_ptr() as usize)?;
    buffer.get(start..start.checked_add(len)?)
}

/// Registers a source file (a unit) under `name`, replacing an earlier one of that name (any
/// case). Both ranges must lie inside the buffer of the last `alloc`, so the host allocates
/// the name and the source in one buffer. Returns 0, or 1 if a range is outside the buffer.
/// The file's number in `err_file` is its position in registration order, counting from 1.
#[no_mangle]
pub extern "C" fn add_file(
    name_ptr: *const u8,
    name_len: usize,
    src_ptr: *const u8,
    src_len: usize,
) -> u32 {
    with(|s| {
        let name = within(&s.src, name_ptr, name_len);
        let text = within(&s.src, src_ptr, src_len);
        let (Some(name), Some(text)) = (name, text) else {
            return 1;
        };
        let file = crate::SourceFile {
            name: String::from_utf8_lossy(name).into_owned(),
            text: text.to_vec(),
        };
        match s
            .files
            .iter_mut()
            .find(|f| f.name.eq_ignore_ascii_case(&file.name))
        {
            Some(existing) => *existing = file,
            None => s.files.push(file),
        }
        0
    })
}

/// Compiles the buffer from the last `alloc`: 0 on success (output via `out_ptr`/`out_len`),
/// 1 on error (position via `err_*`) or if `src_ptr`/`src_len` is not that buffer.
#[no_mangle]
pub extern "C" fn compile(src_ptr: *const u8, src_len: usize, _flags: u32) -> u32 {
    with(|s| {
        let valid = src_ptr == s.src.as_ptr() && src_len <= s.src.len();
        let res = if valid {
            crate::compile_files(&s.src[..src_len], &s.files)
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

/// The file of the last error: 0 for the main program, else the number `add_file` gave.
#[no_mangle]
pub extern "C" fn err_file() -> u32 {
    with(|s| s.err.file)
}
