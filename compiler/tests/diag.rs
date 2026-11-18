use tp5c::diag::*;
use tp5c::{compile, Diag};

/// Numbers the Phase 1 subset can emit, plus 200.
const REQUIRED: &[u16] = &[
    2, 3, 4, 5, 7, 10, 26, 36, 37, 38, 39, 40, 41, 42, 43, 50, 54, 57, 58, 62, 76, 85, 86, 87, 88,
    89, 90, 91, 94, 95, 97, 112, 113, 200,
];

fn dec(n: i64) -> String {
    let mut s = String::new();
    push_dec(&mut s, n);
    s
}

#[test]
fn lookup_known_codes() {
    assert_eq!(message(3), Some("Unknown identifier"));
    assert_eq!(message(5), Some("Syntax error"));
    assert_eq!(message(26), Some("Type mismatch"));
    assert_eq!(message(85), Some("\";\" expected"));
    assert_eq!(message(200), Some("Feature not supported"));
}

#[test]
fn lookup_unknown_codes_is_none() {
    for c in [0, 146, 199, 201, 999, u16::MAX] {
        assert_eq!(message(c), None, "{c}");
    }
}

#[test]
fn table_covers_the_required_numbers() {
    for c in REQUIRED {
        assert!(message(*c).is_some_and(|m| !m.is_empty()), "missing {c}");
    }
}

#[test]
fn table_is_sorted_and_unique() {
    assert!(!TABLE.is_empty());
    assert!(TABLE.windows(2).all(|w| w[0].0 < w[1].0));
}

#[test]
fn table_matches_the_reference_document() {
    let doc = include_str!("../../docs/tp-errors.md");
    let compiler_section = doc.split("## Runtime errors").next().unwrap();
    for (code, msg) in TABLE.iter().filter(|(c, _)| *c != 200) {
        let row = format!("| {code} | {msg} |");
        assert!(compiler_section.lines().any(|l| l == row), "no row `{row}`");
    }
}

#[test]
fn push_dec_edge_cases() {
    assert_eq!(dec(0), "0");
    assert_eq!(dec(7), "7");
    assert_eq!(dec(10), "10");
    assert_eq!(dec(-1), "-1");
    assert_eq!(dec(65535), "65535");
    assert_eq!(dec(i32::MIN as i64), "-2147483648");
    assert_eq!(dec(u32::MAX as i64), "4294967295");
    assert_eq!(dec(i64::MAX), "9223372036854775807");
    assert_eq!(dec(i64::MIN), "-9223372036854775808");
}

#[test]
fn push_dec_appends() {
    let mut s = String::from("a=");
    push_dec(&mut s, 42);
    push_dec(&mut s, -3);
    assert_eq!(s, "a=42-3");
}

#[test]
fn render_format() {
    let d = Diag {
        code: 5,
        line: 3,
        col: 7,
    };
    assert_eq!(render("a.pas", &d), "a.pas(3,7): Error 5: Syntax error");
    let d = Diag {
        code: 200,
        line: 1,
        col: 1,
    };
    assert_eq!(
        render("dir/x.pas", &d),
        "dir/x.pas(1,1): Error 200: Feature not supported"
    );
    let d = Diag {
        code: 85,
        line: 12345,
        col: 80,
    };
    assert_eq!(
        render("p.pas", &d),
        "p.pas(12345,80): Error 85: \";\" expected"
    );
}

#[test]
fn render_unknown_code_still_has_a_message() {
    let d = Diag {
        code: 999,
        line: 2,
        col: 4,
    };
    assert_eq!(render("p.pas", &d), "p.pas(2,4): Error 999: Unknown error");
}

#[test]
fn compile_without_front_end_reports_200_at_1_1() {
    let want = Err(Diag {
        code: 200,
        line: 1,
        col: 1,
    });
    assert_eq!(compile(b""), want);
    assert_eq!(compile(b"program p; begin end."), want);
}
