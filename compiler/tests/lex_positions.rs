mod lex_util;
use lex_util::*;
use Tk::*;

fn pos(src: &str) -> Vec<(Tk, u32, u32)> {
    toks(src).iter().map(|t| (t.kind, t.line, t.col)).collect()
}

#[test]
fn columns_are_one_based() {
    assert_eq!(pos("a"), [(Ident, 1, 1)]);
    assert_eq!(pos(" a"), [(Ident, 1, 2)]);
}

#[test]
fn positions_on_one_line() {
    assert_eq!(
        pos("x := 10;"),
        [(Ident, 1, 1), (Assign, 1, 3), (IntLit, 1, 6), (Semi, 1, 8)]
    );
}

#[test]
fn positions_across_lines() {
    assert_eq!(
        pos("program p;\nvar x: integer;\nbegin\nend."),
        [
            (Program, 1, 1),
            (Ident, 1, 9),
            (Semi, 1, 10),
            (Var, 2, 1),
            (Ident, 2, 5),
            (Colon, 2, 6),
            (Ident, 2, 8),
            (Semi, 2, 15),
            (Begin, 3, 1),
            (End, 4, 1),
            (Dot, 4, 4)
        ]
    );
}

#[test]
fn tab_counts_as_one_column() {
    assert_eq!(pos("\tx"), [(Ident, 1, 2)]);
    assert_eq!(pos("a\t\tb"), [(Ident, 1, 1), (Ident, 1, 4)]);
}

#[test]
fn crlf_is_one_line_break() {
    assert_eq!(
        pos("a\r\nb\r\n  c"),
        [(Ident, 1, 1), (Ident, 2, 1), (Ident, 3, 3)]
    );
}

#[test]
fn lone_lf_and_blank_lines() {
    assert_eq!(pos("\n\n\n  a"), [(Ident, 4, 3)]);
}

#[test]
fn multi_char_token_position_is_its_first_char() {
    assert_eq!(
        pos("  <>  :=  ..  (."),
        [(Ne, 1, 3), (Assign, 1, 7), (DotDot, 1, 11), (LBrack, 1, 15)]
    );
    assert_eq!(pos("  begin"), [(Begin, 1, 3)]);
}

#[test]
fn positions_after_comments() {
    assert_eq!(pos("{c} a"), [(Ident, 1, 5)]);
    assert_eq!(pos("(*c*) a"), [(Ident, 1, 7)]);
    assert_eq!(pos("{a\nb\n  c} x"), [(Ident, 3, 6)]);
    assert_eq!(pos("(* a\n b *)\nx"), [(Ident, 3, 1)]);
}

#[test]
fn positions_after_strings() {
    assert_eq!(pos("'ab' x"), [(StrLit, 1, 1), (Ident, 1, 6)]);
    assert_eq!(pos("'a''b' x"), [(StrLit, 1, 1), (Ident, 1, 8)]);
    assert_eq!(pos("'a'#13#10'b' x"), [(StrLit, 1, 1), (Ident, 1, 13)]);
}

#[test]
fn eof_position() {
    let v = lex(b"ab\ncd").unwrap();
    let e = v.last().unwrap();
    assert_eq!(e.kind, Eof);
    assert_eq!((e.line, e.col), (2, 3));
    let v = lex(b"ab\n").unwrap();
    assert_eq!((v.last().unwrap().line, v.last().unwrap().col), (2, 1));
}

#[test]
fn error_positions_use_the_same_numbering() {
    assert_eq!(err("a\n b\n  ?"), (5, 3, 3));
    assert_eq!(err("a\r\n?"), (5, 2, 1));
    assert_eq!(err("\t?"), (5, 1, 2));
    assert_eq!(err("{c\n}\n'x"), (8, 3, 1));
}

#[test]
fn many_lines() {
    let src = "x\n".repeat(5000) + "y";
    let v = toks(&src);
    assert_eq!(v.len(), 5001);
    assert_eq!((v[4999].line, v[5000].line, v[5000].col), (5000, 5001, 1));
}
