use crate::driver::Diag;

/// TP error numbers and messages (docs/tp-errors.md) the compiler can emit, ascending.
pub const TABLE: &[(u16, &str)] = &[
    (2, "Identifier expected"),
    (3, "Unknown identifier"),
    (4, "Duplicate identifier"),
    (5, "Syntax error"),
    (6, "Error in real constant"),
    (7, "Error in integer constant"),
    (8, "String constant exceeds line"),
    (10, "Unexpected end of file"),
    (12, "Type identifier expected"),
    (17, "Invalid compiler directive"),
    (20, "Variable identifier expected"),
    (22, "Structure too large"),
    (26, "Type mismatch"),
    (28, "Lower bound greater than upper bound"),
    (29, "Ordinal type expected"),
    (30, "Integer constant expected"),
    (31, "Constant expected"),
    (33, "Type identifier expected"),
    (34, "Invalid function result type"),
    (36, "BEGIN expected"),
    (37, "END expected"),
    (38, "Integer expression expected"),
    (39, "Ordinal expression expected"),
    (40, "Boolean expression expected"),
    (41, "Operand types do not match operator"),
    (42, "Error in expression"),
    (43, "Illegal assignment"),
    (44, "Field identifier expected"),
    (50, "DO expected"),
    (54, "OF expected"),
    (57, "THEN expected"),
    (58, "TO or DOWNTO expected"),
    (59, "Undefined forward"),
    (62, "Division by zero"),
    (64, "Cannot Read or Write variables of this type"),
    (74, "Constant and case types do not match"),
    (75, "Record variable expected"),
    (76, "Constant out of range"),
    (85, "\";\" expected"),
    (86, "\":\" expected"),
    (87, "\",\" expected"),
    (88, "\"(\" expected"),
    (89, "\")\" expected"),
    (90, "\"=\" expected"),
    (91, "\":=\" expected"),
    (93, "\"]\" or \".)\" expected"),
    (94, "\".\" expected"),
    (95, "\"..\" expected"),
    (96, "Too many variables"),
    (97, "Invalid FOR control variable"),
    (112, "CASE constant out of range"),
    (113, "Error in statement"),
    (121, "Invalid qualifier"),
    (133, "Cannot evaluate this expression"),
    (200, "Feature not supported"),
];

pub fn message(code: u16) -> Option<&'static str> {
    let i = TABLE.binary_search_by_key(&code, |e| e.0).ok()?;
    Some(TABLE[i].1)
}

/// Appends the decimal form of `n` to `out`.
pub fn push_dec(out: &mut String, n: i64) {
    if n < 0 {
        out.push('-');
    }
    let mut v = n.unsigned_abs();
    let mut buf = [0u8; 20];
    let mut i = buf.len();
    loop {
        i -= 1;
        buf[i] = b'0' + (v % 10) as u8;
        v /= 10;
        if v == 0 {
            break;
        }
    }
    buf[i..].iter().for_each(|&b| out.push(b as char));
}

/// `NAME(LINE,COL): Error N: MESSAGE`, without a trailing newline.
pub fn render(name: &str, d: &Diag) -> String {
    let mut s = String::from(name);
    for (open, n) in [("(", d.line), (",", d.col)] {
        s.push_str(open);
        push_dec(&mut s, n as i64);
    }
    s.push_str("): Error ");
    push_dec(&mut s, d.code as i64);
    s.push_str(": ");
    s.push_str(message(d.code).unwrap_or("Unknown error"));
    s
}
