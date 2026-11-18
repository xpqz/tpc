#![forbid(unsafe_code)]

pub mod diag;
pub mod driver;
pub mod lex;
pub mod wasm;

pub use driver::{compile, Diag};
