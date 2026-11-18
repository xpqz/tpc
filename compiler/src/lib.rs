#![deny(unsafe_code)]

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

pub use driver::{compile, compile_with, Diag};
