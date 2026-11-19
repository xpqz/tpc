#![deny(unsafe_code)]
//! A Turbo Pascal 5.0 compiler to WebAssembly. The pipeline, in reading order:
//!
//! - `units`: finds and orders the units a program uses; per file, `lex` turns bytes to tokens,
//!   `directives` applies conditional compilation and include files to them, and `parse` turns
//!   the result into the syntax tree (`ast`), an arena of nodes shared by all files.
//! - `check`: names, types and constants resolved, the tree rewritten in place; `types` holds
//!   the type table and the operator typing rules, `symbols` the scopes, storage and frame
//!   layout, `diag` the errors.
//! - `codegen`: the checked tree to WASM instructions; `wasm` writes the binary module.
//! - `driver` runs the stages, and `ffi` exports them to the host on wasm32.

pub mod ast;
pub mod check;
pub mod codegen;
pub mod diag;
pub mod directives;
pub mod driver;
#[cfg(target_arch = "wasm32")]
pub mod ffi;
pub mod lex;
pub mod parse;
pub mod symbols;
pub mod types;
pub mod units;
pub mod wasm;

pub use diag::Diag;
pub use driver::{compile, compile_files, compile_with};
pub use units::SourceFile;
