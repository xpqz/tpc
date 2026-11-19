//! Diagnostics: the error type, TP error numbers with their messages, and rendering.
//! There is no `format!`: messages come from a static table and numbers are printed by
//! `push_dec`. The numbers appear only in `code`, which is kept in ascending order.

/// A compile error: TP error number and the source position TP reports it at. `file` is 0 for
/// the main program (and System), n for the n-th registered file.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Diag {
    pub code: u16,
    pub line: u32,
    pub col: u32,
    pub file: u32,
}

impl Diag {
    /// The same position and file with another error number.
    pub fn with_code(self, code: u16) -> Diag {
        Diag { code, ..self }
    }
}

/// Names for the TP error numbers the compiler reports (see `docs/tp-errors.md`).
pub mod code {
    pub const IDENTIFIER_EXPECTED: u16 = 2;
    pub const UNKNOWN_IDENTIFIER: u16 = 3;
    pub const DUPLICATE_IDENTIFIER: u16 = 4;
    pub const SYNTAX_ERROR: u16 = 5;
    pub const REAL_CONSTANT_ERROR: u16 = 6;
    pub const INTEGER_CONSTANT_ERROR: u16 = 7;
    pub const STRING_EXCEEDS_LINE: u16 = 8;
    pub const TOO_MANY_NESTED_FILES: u16 = 9;
    pub const UNEXPECTED_END_OF_FILE: u16 = 10;
    pub const TYPE_IDENTIFIER_EXPECTED: u16 = 12;
    pub const FILE_NOT_FOUND: u16 = 15;
    pub const INVALID_DIRECTIVE: u16 = 17;
    pub const UNDEFINED_POINTER_TARGET: u16 = 19;
    pub const VARIABLE_EXPECTED: u16 = 20;
    pub const ERROR_IN_TYPE: u16 = 21;
    pub const STRUCTURE_TOO_LARGE: u16 = 22;
    pub const SET_BASE_OUT_OF_RANGE: u16 = 23;
    pub const INVALID_STRING_LENGTH: u16 = 25;
    pub const TYPE_MISMATCH: u16 = 26;
    pub const LOWER_BOUND_GREATER: u16 = 28;
    pub const ORDINAL_TYPE_EXPECTED: u16 = 29;
    pub const INVALID_RESULT_TYPE: u16 = 34;
    pub const BEGIN_EXPECTED: u16 = 36;
    pub const END_EXPECTED: u16 = 37;
    pub const INTEGER_EXPRESSION_EXPECTED: u16 = 38;
    pub const ORDINAL_EXPRESSION_EXPECTED: u16 = 39;
    pub const BOOLEAN_EXPRESSION_EXPECTED: u16 = 40;
    pub const OPERAND_TYPES_MISMATCH: u16 = 41;
    pub const ERROR_IN_EXPRESSION: u16 = 42;
    pub const ILLEGAL_ASSIGNMENT: u16 = 43;
    pub const FIELD_IDENTIFIER_EXPECTED: u16 = 44;
    pub const DO_EXPECTED: u16 = 50;
    pub const OF_EXPECTED: u16 = 54;
    pub const INTERFACE_EXPECTED: u16 = 55;
    pub const THEN_EXPECTED: u16 = 57;
    pub const TO_OR_DOWNTO_EXPECTED: u16 = 58;
    pub const UNDEFINED_FORWARD: u16 = 59;
    pub const DIVISION_BY_ZERO: u16 = 62;
    pub const CANNOT_READ_OR_WRITE: u16 = 64;
    pub const POINTER_VARIABLE_EXPECTED: u16 = 65;
    pub const STRING_VARIABLE_EXPECTED: u16 = 66;
    pub const STRING_EXPRESSION_EXPECTED: u16 = 67;
    pub const CIRCULAR_UNIT_REFERENCE: u16 = 68;
    pub const UNIT_NAME_MISMATCH: u16 = 69;
    pub const IMPLEMENTATION_EXPECTED: u16 = 73;
    pub const CASE_TYPES_MISMATCH: u16 = 74;
    pub const RECORD_VARIABLE_EXPECTED: u16 = 75;
    pub const CONSTANT_OUT_OF_RANGE: u16 = 76;
    pub const FILE_VARIABLE_EXPECTED: u16 = 77;
    pub const INTEGER_OR_REAL_EXPRESSION_EXPECTED: u16 = 79;
    pub const UNIT_EXPECTED: u16 = 84;
    pub const SEMICOLON_EXPECTED: u16 = 85;
    pub const COLON_EXPECTED: u16 = 86;
    pub const COMMA_EXPECTED: u16 = 87;
    pub const LEFT_PAREN_EXPECTED: u16 = 88;
    pub const RIGHT_PAREN_EXPECTED: u16 = 89;
    pub const EQUALS_EXPECTED: u16 = 90;
    pub const ASSIGN_EXPECTED: u16 = 91;
    pub const RIGHT_BRACKET_EXPECTED: u16 = 93;
    pub const PERIOD_EXPECTED: u16 = 94;
    pub const TOO_MANY_VARIABLES: u16 = 96;
    pub const INVALID_FOR_VARIABLE: u16 = 97;
    pub const INTEGER_VARIABLE_EXPECTED: u16 = 98;
    pub const PROCEDURE_TYPE_NOT_ALLOWED: u16 = 99;
    pub const STRING_LENGTH_MISMATCH: u16 = 100;
    pub const INVALID_FIELD_ORDER: u16 = 101;
    pub const NUMERIC_VARIABLE_EXPECTED: u16 = 103;
    pub const ORDINAL_VARIABLE_EXPECTED: u16 = 104;
    pub const CHAR_EXPRESSION_EXPECTED: u16 = 106;
    pub const CASE_CONSTANT_OUT_OF_RANGE: u16 = 112;
    pub const ERROR_IN_STATEMENT: u16 = 113;
    pub const INCLUDE_NOT_ALLOWED: u16 = 118;
    pub const INVALID_QUALIFIER: u16 = 121;
    pub const INVALID_VARIABLE_REFERENCE: u16 = 122;
    pub const FILE_MUST_BE_VAR: u16 = 126;
    pub const MISPLACED_CONDITIONAL: u16 = 128;
    pub const ENDIF_MISSING: u16 = 129;
    pub const HEADER_MISMATCH: u16 = 131;
    pub const CANNOT_EVALUATE: u16 = 133;
    pub const INVALID_PROCEDURE_REFERENCE: u16 = 143;
    pub const NOT_SUPPORTED: u16 = 200;
}

/// TP error numbers and messages (docs/tp-errors.md) the compiler can emit, ascending.
pub const TABLE: &[(u16, &str)] = &[
    (2, "Identifier expected"),
    (3, "Unknown identifier"),
    (4, "Duplicate identifier"),
    (5, "Syntax error"),
    (6, "Error in real constant"),
    (7, "Error in integer constant"),
    (8, "String constant exceeds line"),
    (9, "Too many nested files"),
    (10, "Unexpected end of file"),
    (12, "Type identifier expected"),
    (15, "File not found"),
    (17, "Invalid compiler directive"),
    (19, "Undefined type in pointer definition"),
    (20, "Variable identifier expected"),
    (21, "Error in type"),
    (22, "Structure too large"),
    (23, "Set base type out of range"),
    (25, "Invalid string length"),
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
    (55, "INTERFACE expected"),
    (57, "THEN expected"),
    (58, "TO or DOWNTO expected"),
    (59, "Undefined forward"),
    (62, "Division by zero"),
    (64, "Cannot Read or Write variables of this type"),
    (68, "Circular unit reference"),
    (69, "Unit name mismatch"),
    (73, "IMPLEMENTATION expected"),
    (74, "Constant and case types do not match"),
    (75, "Record variable expected"),
    (76, "Constant out of range"),
    (77, "File variable expected"),
    (84, "UNIT expected"),
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
    (98, "Integer variable expected"),
    (99, "Files and procedure types are not allowed here"),
    (100, "String length mismatch"),
    (101, "Invalid ordering of fields"),
    (112, "CASE constant out of range"),
    (113, "Error in statement"),
    (118, "Include files are not allowed here"),
    (121, "Invalid qualifier"),
    (126, "Files must be var parameters"),
    (128, "Misplaced conditional directive"),
    (129, "ENDIF directive missing"),
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
    s.push('(');
    push_dec(&mut s, d.line as i64);
    s.push(',');
    push_dec(&mut s, d.col as i64);
    s.push_str("): Error ");
    push_dec(&mut s, d.code as i64);
    s.push_str(": ");
    s.push_str(message(d.code).unwrap_or("Unknown error"));
    s
}
