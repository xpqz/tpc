mod lex_util;
use lex_util::*;
use Tk::*;

#[test]
fn brace_comments_are_skipped() {
    assert_eq!(kinds("{ hi }"), []);
    assert_eq!(kinds("a { c } b"), [Ident, Ident]);
    assert_eq!(kinds("a{c}b"), [Ident, Ident]);
    assert_eq!(kinds("{}"), []);
    assert_eq!(kinds("{ \n multi \n line \n } x"), [Ident]);
}

#[test]
fn paren_star_comments_are_skipped() {
    assert_eq!(kinds("(* hi *)"), []);
    assert_eq!(kinds("a (* c *) b"), [Ident, Ident]);
    assert_eq!(kinds("a(*c*)b"), [Ident, Ident]);
    assert_eq!(kinds("(**)"), []);
    assert_eq!(kinds("(***)"), []);
    assert_eq!(kinds("(* \n multi \n *) x"), [Ident]);
}

#[test]
fn comments_do_not_nest() {
    assert_eq!(kinds("{ { } x"), [Ident]);
    assert_eq!(kinds("(* (* *) x"), [Ident]);
    assert_eq!(kinds("{ (* } x"), [Ident]);
    assert_eq!(kinds("(* { *) x"), [Ident]);
    assert_eq!(kinds("(* } *) x"), [Ident]);
    assert_eq!(kinds("{ *) } x"), [Ident]);
}

#[test]
fn stray_close_brace_is_invalid() {
    assert_eq!(err("a }"), (5, 1, 3));
}

#[test]
fn comment_containing_quote_and_text() {
    assert_eq!(kinds("{ it's } x"), [Ident]);
    assert_eq!(kinds("(* 'unterminated *) x"), [Ident]);
}

#[test]
fn comment_between_tokens_separates_them() {
    assert_eq!(kinds("1{c}2"), [IntLit, IntLit]);
    assert_eq!(kinds("a(**)b"), [Ident, Ident]);
    assert_eq!(kinds("<{c}>"), [Lt, Gt]);
    assert_eq!(kinds(":{c}="), [Colon, Eq]);
}

#[test]
fn comments_at_end_of_input() {
    assert_eq!(kinds("end. { trailing }"), [End, Dot]);
    assert_eq!(kinds("end. (* trailing *)"), [End, Dot]);
    assert_eq!(kinds("end.{x}"), [End, Dot]);
    assert_eq!(kinds("end.(*x*)"), [End, Dot]);
    assert_eq!(kinds("end.\n{x}\n"), [End, Dot]);
}

#[test]
fn unterminated_comment_is_error_10() {
    assert_eq!(err("{ never closed").0, 10);
    assert_eq!(err("(* never closed").0, 10);
    assert_eq!(err("{").0, 10);
    assert_eq!(err("(*").0, 10);
    assert_eq!(err("(*)").0, 10);
    assert_eq!(err("x {a\nb\nc").0, 10);
}

#[test]
fn unterminated_comment_position_is_its_opening() {
    assert_eq!(err("x { open"), (10, 1, 3));
    assert_eq!(err("a\n  (* open\nmore"), (10, 2, 3));
    assert_eq!(err("{"), (10, 1, 1));
}

#[test]
fn mismatched_closer_does_not_terminate() {
    assert_eq!(err("{ *)").0, 10);
    assert_eq!(err("(* }").0, 10);
}

#[test]
fn directive_brace() {
    let t = one("{$R+}");
    assert_eq!(t.kind, Directive);
    assert_eq!(t.text, b"R+");
    assert_eq!((t.line, t.col), (1, 1));
}

#[test]
fn directive_paren_star() {
    let t = one("(*$R-*)");
    assert_eq!(t.kind, Directive);
    assert_eq!(t.text, b"R-");
}

#[test]
fn directive_body_case_and_spaces_preserved() {
    assert_eq!(one("{$I Inc.pas}").text, b"I Inc.pas");
    assert_eq!(one("{$ifdef Foo}").text, b"ifdef Foo");
    assert_eq!(one("{$DEFINE X}").text, b"DEFINE X");
    assert_eq!(one("(*$IFNDEF a b *)").text, b"IFNDEF a b ");
    assert_eq!(one("{$}").text, b"");
}

#[test]
fn directive_is_distinct_from_comment() {
    assert_eq!(kinds("{ $R+}"), []);
    assert_eq!(kinds("{$R+} x {$R-}"), [Directive, Ident, Directive]);
    assert_eq!(kinds("(* $R+ *)"), []);
    assert_eq!(kinds("{x$R+}"), []);
}

#[test]
fn directive_position() {
    let v = toks("a\n  {$I+}");
    assert_eq!((v[1].line, v[1].col), (2, 3));
}

#[test]
fn directive_does_not_nest_and_ends_at_first_closer() {
    let v = toks("{$A {} x");
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].text, b"A {");
    assert_eq!(v[1].kind, Ident);
}

#[test]
fn unterminated_directive_is_error_10() {
    assert_eq!(err("{$R+"), (10, 1, 1));
    assert_eq!(err("(*$R+"), (10, 1, 1));
}
