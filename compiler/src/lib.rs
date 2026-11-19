#![deny(unsafe_code)]
//! A Turbo Pascal 5.0 compiler to WebAssembly. The pipeline, in reading order:
//!
//! - `lex`: bytes to tokens.
//! - `parse`: tokens to the syntax tree (`ast`), an arena of nodes.
//! - `check`: names, types and constants resolved, the tree rewritten in place; `types` holds
//!   the type table and the operator typing rules, `diag` the errors.
//! - `codegen`: the checked tree to WASM instructions; `wasm` writes the binary module.
//! - `driver` runs the stages, and `ffi` exports them to the host on wasm32.

pub mod ast;
pub mod check;
pub mod codegen;
pub mod diag;
pub mod driver;
#[cfg(target_arch = "wasm32")]
pub mod ffi;
pub mod lex;
pub mod parse;
pub mod types;
pub mod wasm;

pub use diag::Diag;
pub use driver::{compile, compile_with};
