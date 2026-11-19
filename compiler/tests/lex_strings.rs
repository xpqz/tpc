mod lex_util;
use lex_util::*;
use Tk::*;

#[test]
fn simple_strings() {
    assert_eq!(string("'abc'"), b"abc");
    assert_eq!(string("'a'"), b"a");
    assert_eq!(string("'Hello, World!'"), b"Hello, World!");
    assert_eq!(string("'  spaced  '"), b"  spaced  ");
    assert_eq!(string("'{not a comment}'"), b"{not a comment}");
    assert_eq!(string("'(* nor this *)'"), b"(* nor this *)");
    assert_eq!(string("'a;b:=c'"), b"a;b:=c");
}

#[test]
fn string_case_is_preserved() {
    assert_eq!(string("'MiXeD'"), b"MiXeD");
}

#[test]
fn empty_string() {
    let t = one("''");
    assert_eq!(k(&t), StrLit);
    assert!(t.text.is_empty());
}

#[test]
fn doubled_quote_escape() {
    assert_eq!(string("'it''s'"), b"it's");
    assert_eq!(string("''''"), b"'");
    assert_eq!(string("''''''"), b"''");
    assert_eq!(string("'a'''"), b"a'");
    assert_eq!(string("'''a'"), b"'a");
}

#[test]
fn char_codes_decimal_and_hex() {
    assert_eq!(string("#65"), b"A");
    assert_eq!(string("#0"), [0]);
    assert_eq!(string("#255"), [255]);
    assert_eq!(string("#013"), [13]);
    assert_eq!(string("#$41"), b"A");
    assert_eq!(string("#$ff"), [255]);
    assert_eq!(string("#$0"), [0]);
    assert_eq!(string("#$0041"), b"A");
}

#[test]
fn char_code_out_of_range_is_error_76() {
    assert_eq!(err("#256"), (76, 1, 1));
    assert_eq!(err("#$100"), (76, 1, 1));
    assert_eq!(err("#99999"), (76, 1, 1));
    assert_eq!(err("x := 'a' #300"), (76, 1, 10));
}

#[test]
fn char_code_without_digits_is_error_7() {
    assert_eq!(err("#"), (7, 1, 1));
    assert_eq!(err("#$"), (7, 1, 1));
    assert_eq!(err("#a"), (7, 1, 1));
    assert_eq!(err("x := #;"), (7, 1, 6));
}

#[test]
fn control_characters() {
    assert_eq!(string("'a'^A"), [b'a', 1]);
    assert_eq!(string("#65^B"), [b'A', 2]);
    assert_eq!(string("'x'^Z"), [b'x', 26]);
    assert_eq!(string("'x'^@"), [b'x', 0]);
    assert_eq!(string("'x'^["), [b'x', 27]);
    assert_eq!(string("'x'^_"), [b'x', 31]);
    assert_eq!(string("'x'^a"), [b'x', 1]);
}

#[test]
fn adjacent_pieces_form_one_token() {
    assert_eq!(string("'a'#13#10'b'"), [b'a', 13, 10, b'b']);
    assert_eq!(string("#13#10"), [13, 10]);
    assert_eq!(string("'ab''cd'"), b"ab'cd");
    assert_eq!(string("'a''b'"), b"a'b");
    assert_eq!(string("'a'#$42'c'"), b"aBc");
    assert_eq!(string("#65'B'#67"), b"ABC");
    assert_eq!(string("'x'^A'y'"), [b'x', 1, b'y']);
    assert_eq!(string("'a'#65#$41'a'"), b"aAAa");
    assert_eq!(string("''#13''"), [13]);
    assert_eq!(toks("'a'#13#10'b'").len(), 1);
}

#[test]
fn whitespace_separates_string_tokens() {
    let v = toks("'a' 'b'");
    assert_eq!(v.len(), 2);
    assert_eq!(
        (v[0].text.as_slice(), v[1].text.as_slice()),
        (&b"a"[..], &b"b"[..])
    );
    assert_eq!(kinds("'a' #13"), [StrLit, StrLit]);
    assert_eq!(kinds("#13 'a'"), [StrLit, StrLit]);
    assert_eq!(kinds("'a'\n'b'"), [StrLit, StrLit]);
    assert_eq!(kinds("'a'{c}'b'"), [StrLit, StrLit]);
}

#[test]
fn string_followed_by_other_tokens() {
    assert_eq!(kinds("'a',x"), [StrLit, Comma, Ident]);
    assert_eq!(kinds("('a')"), [LParen, StrLit, RParen]);
    assert_eq!(kinds("'a'+'b'"), [StrLit, Plus, StrLit]);
    assert_eq!(kinds("s='a'"), [Ident, Eq, StrLit]);
}

#[test]
fn caret_after_identifier_is_not_a_string() {
    assert_eq!(kinds("p^ A"), [Ident, Caret, Ident]);
}

#[test]
fn strings_may_hold_high_and_control_bytes() {
    let t = lex(b"'\xE9\x01\t'").unwrap();
    assert_eq!(t[0].text, [0xE9, 1, 9]);
}

#[test]
fn long_string() {
    // A whole source line is at most 127 bytes, quotes included.
    let body = "x".repeat(125);
    assert_eq!(string(&format!("'{body}'")), body.as_bytes());
}

#[test]
fn unterminated_string_is_error_8() {
    assert_eq!(err("'abc"), (8, 1, 1));
    assert_eq!(err("'"), (8, 1, 1));
    assert_eq!(err("x := 'abc"), (8, 1, 6));
    assert_eq!(err("'abc\nnext'"), (8, 1, 1));
    assert_eq!(err("'abc\r\n'"), (8, 1, 1));
    assert_eq!(err("a\n  'oops\n"), (8, 2, 3));
    assert_eq!(err("'it''s"), (8, 1, 1));
}

#[test]
fn unterminated_after_pieces_is_error_8() {
    assert_eq!(err("#65'abc"), (8, 1, 1));
    assert_eq!(err("'a'#65'bc"), (8, 1, 1));
}

#[test]
fn string_positions() {
    let v = toks("x := 'a'#13'b' + 'c';");
    let cols: Vec<_> = v.iter().map(|t| t.col).collect();
    assert_eq!(cols, [1, 3, 6, 16, 18, 21]);
}
