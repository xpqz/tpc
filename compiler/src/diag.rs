use crate::driver::Diag;

/// TP error numbers and messages (docs/tp-errors.md) the compiler can emit, ascending.
pub const TABLE: &[(u16, &str)] = &[];

pub fn message(_code: u16) -> Option<&'static str> {
    None
}

/// Appends the decimal form of `n` to `out`.
pub fn push_dec(_out: &mut String, _n: i64) {}

/// `NAME(LINE,COL): Error N: MESSAGE`, without a trailing newline.
pub fn render(_name: &str, _d: &Diag) -> String {
    String::new()
}
