mod lex_util;
use lex_util::*;
use Tk::*;

#[test]
fn decimal_integers() {
    assert_eq!(int("0"), 0);
    assert_eq!(int("7"), 7);
    assert_eq!(int("42"), 42);
    assert_eq!(int("007"), 7);
    assert_eq!(int("32767"), 32767);
    assert_eq!(int("32768"), 32768);
    assert_eq!(int("65535"), 65535);
    assert_eq!(int("65536"), 65536);
}

#[test]
fn longint_boundary() {
    assert_eq!(int("2147483647"), 2147483647);
    assert_eq!(err("2147483648"), (7, 1, 1));
    assert_eq!(err("4294967295"), (7, 1, 1));
    assert_eq!(err("99999999999999999999999999"), (7, 1, 1));
    assert_eq!(err("0002147483648"), (7, 1, 1));
    assert_eq!(int("0002147483647"), 2147483647);
}

#[test]
fn hex_integers() {
    assert_eq!(int("$0"), 0);
    assert_eq!(int("$F"), 15);
    assert_eq!(int("$ff"), 255);
    assert_eq!(int("$Ff"), 255);
    assert_eq!(int("$00FF"), 255);
    assert_eq!(int("$7FFF"), 32767);
    assert_eq!(int("$8000"), 32768);
    assert_eq!(int("$FFFF"), 65535);
    assert_eq!(int("$10000"), 65536);
    assert_eq!(int("$7FFFFFFF"), 2147483647);
    assert_eq!(int("$80000000"), 2147483648);
    assert_eq!(int("$abcdef"), 0xABCDEF);
}

#[test]
fn hex_upper_boundary() {
    // The value is the unsigned 32-bit pattern; the checker reinterprets it as LongInt.
    assert_eq!(int("$FFFFFFFF"), 4294967295);
    assert_eq!(int("$0000FFFFFFFF"), 4294967295);
    assert_eq!(err("$100000000"), (7, 1, 1));
    assert_eq!(err("$FFFFFFFFF"), (7, 1, 1));
    assert_eq!(err("$FFFFFFFFFFFFFFFFFFFF"), (7, 1, 1));
}

#[test]
fn hex_without_digits_is_error_7() {
    assert_eq!(err("$"), (7, 1, 1));
    assert_eq!(err("x := $;"), (7, 1, 6));
    assert_eq!(err("$G"), (7, 1, 1));
    assert_eq!(err("$ 1"), (7, 1, 1));
}

#[test]
fn hex_followed_by_identifier_letters() {
    // `$12G`: the digits end at G, which starts an identifier.
    assert_eq!(kinds("$12 G"), [IntLit, Ident]);
}

#[test]
fn reals() {
    assert_eq!(real("1.5"), 1.5);
    assert_eq!(real("0.0"), 0.0);
    assert_eq!(real("0.1"), 0.1);
    assert_eq!(real("6.02214"), 6.02214);
    assert_eq!(real("123.456"), 123.456);
    assert_eq!(real("1e10"), 1e10);
    assert_eq!(real("1E10"), 1e10);
    assert_eq!(real("1.5E-3"), 1.5e-3);
    assert_eq!(real("1.5e+3"), 1500.0);
    assert_eq!(real("2e0"), 2.0);
    assert_eq!(real("1e-5"), 1e-5);
    assert_eq!(real("5E+2"), 500.0);
    assert_eq!(real("1.0e1"), 10.0);
    assert_eq!(real("0.000001"), 1e-6);
    assert_eq!(real("2147483648.0"), 2147483648.0);
    assert_eq!(real("99999999999999999999.0"), 1e20);
    assert_eq!(real("2147483648e0"), 2147483648.0);
}

#[test]
fn real_has_no_sign() {
    assert_eq!(kinds("-1.5"), [Minus, RealLit]);
    assert_eq!(kinds("+1e3"), [Plus, RealLit]);
}

#[test]
fn real_exponent_range() {
    assert_eq!(real("1e308"), 1e308);
    assert_eq!(err("1e400"), (6, 1, 1));
    assert_eq!(err("1.5e999"), (6, 1, 1));
    assert_eq!(real("1e-400"), 0.0);
}

#[test]
fn real_errors_are_6() {
    assert_eq!(err("1e"), (6, 1, 1));
    assert_eq!(err("1e+"), (6, 1, 1));
    assert_eq!(err("1E-"), (6, 1, 1));
    assert_eq!(err("1.5e"), (6, 1, 1));
    assert_eq!(err("1.5e;"), (6, 1, 1));
    assert_eq!(err("x := 1.5E+;"), (6, 1, 6));
    assert_eq!(err("a\n  3e"), (6, 2, 3));
}

#[test]
fn integer_dot_dot_is_not_a_real() {
    assert_eq!(kinds("1..2"), [IntLit, DotDot, IntLit]);
    let v = toks("1..2");
    assert_eq!((v[0].int, v[2].int), (1, 2));
    assert_eq!(kinds("0..255"), [IntLit, DotDot, IntLit]);
    assert_eq!(kinds("$10..$20"), [IntLit, DotDot, IntLit]);
    assert_eq!(kinds("1.."), [IntLit, DotDot]);
}

#[test]
fn integer_followed_by_dot_without_digit_is_integer_and_dot() {
    assert_eq!(kinds("1."), [IntLit, Dot]);
    assert_eq!(kinds("1.)"), [IntLit, RBrack]);
    assert_eq!(kinds("end 5."), [End, IntLit, Dot]);
    assert_eq!(kinds("a[1].b"), [Ident, LBrack, IntLit, RBrack, Dot, Ident]);
}

#[test]
fn integer_dot_identifier() {
    assert_eq!(kinds("1.x"), [IntLit, Dot, Ident]);
}

#[test]
fn numbers_in_expressions() {
    assert_eq!(kinds("x:=1.5*2"), [Ident, Assign, RealLit, Star, IntLit]);
    assert_eq!(kinds("[1,2]"), [LBrack, IntLit, Comma, IntLit, RBrack]);
    assert_eq!(
        kinds("a[1..9]"),
        [Ident, LBrack, IntLit, DotDot, IntLit, RBrack]
    );
}

#[test]
fn number_positions() {
    let v = toks("  12 $1F 1.5");
    assert_eq!(v.iter().map(|t| t.col).collect::<Vec<_>>(), [3, 6, 10]);
}

#[test]
fn int_tokens_have_no_text_requirement_but_reals_keep_value() {
    let t = one("2.5");
    assert_eq!(t.kind, RealLit);
    assert_eq!(t.real, 2.5);
}
