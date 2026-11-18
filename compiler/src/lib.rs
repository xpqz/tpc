#![forbid(unsafe_code)]

pub mod ast;
pub mod check;
pub mod codegen;
pub mod diag;
pub mod driver;
pub mod lex;
pub mod parse;
pub mod types;
pub mod wasm;

pub use driver::{compile, Diag};
