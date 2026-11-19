mod lex_util;
use lex_util::*;
use tp5c::lex::Tk as RawTk;
use Tk::*;

#[test]
fn empty_source_is_just_eof() {
    let v = lex(b"").unwrap();
    assert_eq!(v.len(), 1);
    assert_eq!(k(&v[0]), Eof);
    assert_eq!((v[0].line, v[0].col), (1, 1));
}

#[test]
fn whitespace_only_is_just_eof() {
    assert!(kinds(" \t\r\n\n  ").is_empty());
}

#[test]
fn every_reserved_word() {
    let words = [
        ("and", And),
        ("array", Array),
        ("asm", Asm),
        ("begin", Begin),
        ("case", Case),
        ("const", Const),
        ("div", Div),
        ("do", Do),
        ("downto", Downto),
        ("else", Else),
        ("end", End),
        ("file", File),
        ("for", For),
        ("function", Function),
        ("goto", Goto),
        ("if", If),
        ("implementation", Implementation),
        ("in", In),
        ("inline", Inline),
        ("interface", Interface),
        ("label", Label),
        ("mod", Mod),
        ("nil", Nil),
        ("not", Not),
        ("of", Of),
        ("or", Or),
        ("packed", Packed),
        ("procedure", Procedure),
        ("program", Program),
        ("record", Record),
        ("repeat", Repeat),
        ("set", Set),
        ("shl", Shl),
        ("shr", Shr),
        ("string", String),
        ("then", Then),
        ("to", To),
        ("type", Type),
        ("unit", Unit),
        ("until", Until),
        ("uses", Uses),
        ("var", Var),
        ("while", While),
        ("with", With),
        ("xor", Xor),
        ("absolute", Absolute),
        ("external", External),
        ("forward", Forward),
        ("interrupt", Interrupt),
    ];
    for (w, k) in words {
        assert_eq!(kinds(w), [k], "{w}");
        assert_eq!(kinds(&w.to_uppercase()), [k], "{w} upper");
        let mixed: std::string::String = w
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i % 2 == 0 {
                    c.to_ascii_uppercase()
                } else {
                    c
                }
            })
            .collect();
        assert_eq!(kinds(&mixed), [k], "{mixed}");
    }
}

#[test]
fn keyword_tokens_are_kw_with_uppercase_text() {
    for (src, text) in [
        ("begin", "BEGIN"),
        ("BeGiN", "BEGIN"),
        ("implementation", "IMPLEMENTATION"),
        ("string", "STRING"),
        ("interrupt", "INTERRUPT"),
    ] {
        assert_eq!(
            raw_kind(src),
            (RawTk::Kw, text.as_bytes().to_vec()),
            "{src}"
        );
    }
    assert_eq!(raw_kind("Foo"), (RawTk::Ident, b"FOO".to_vec()));
}

#[test]
fn symbol_tokens_are_sym_with_canonical_text() {
    for (src, text) in [
        (":=", ":="),
        ("<>", "<>"),
        ("(.", "["),
        (".)", "]"),
        ("[", "["),
        ("]", "]"),
        ("..", ".."),
        ("^", "^"),
        ("@", "@"),
        (";", ";"),
    ] {
        assert_eq!(
            raw_kind(src),
            (RawTk::Sym, text.as_bytes().to_vec()),
            "{src}"
        );
    }
}

#[test]
fn keywords_carry_no_ident_text() {
    assert_eq!(k(&one("begin")), Begin);
}

#[test]
fn identifiers_are_uppercased() {
    let t = one("fooBar_9");
    assert_eq!(k(&t), Ident);
    assert_eq!(t.text, b"FOOBAR_9");
    assert_eq!(one("_x").text, b"_X");
    assert_eq!(one("X").text, b"X");
}

#[test]
fn identifiers_differing_only_in_case_are_equal() {
    assert_eq!(one("Count").text, one("COUNT").text);
    assert_eq!(one("Count").text, one("count").text);
}

#[test]
fn keyword_prefix_and_suffix_are_identifiers() {
    for w in [
        "ends", "begins", "xend", "ifx", "_end", "end_", "end1", "andy", "string1", "tO0",
    ] {
        let t = one(w);
        assert_eq!(k(&t), Ident, "{w}");
        assert_eq!(t.text, w.to_uppercase().as_bytes());
    }
}

#[test]
fn unreserved_names_are_identifiers() {
    for w in [
        "integer", "byte", "writeln", "true", "self", "object", "private", "far", "near",
    ] {
        assert_eq!(k(&one(w)), Ident, "{w}");
    }
}

#[test]
fn every_operator_and_punctuation() {
    let ops = [
        ("+", Plus),
        ("-", Minus),
        ("*", Star),
        ("/", Slash),
        ("=", Eq),
        ("<", Lt),
        (">", Gt),
        ("<=", Le),
        (">=", Ge),
        ("<>", Ne),
        (":=", Assign),
        ("(", LParen),
        (")", RParen),
        ("[", LBrack),
        ("]", RBrack),
        ("(.", LBrack),
        (".)", RBrack),
        (".", Dot),
        ("..", DotDot),
        (",", Comma),
        (";", Semi),
        (":", Colon),
        ("^", Caret),
        ("@", At),
    ];
    for (s, k) in ops {
        assert_eq!(kinds(s), [k], "{s}");
    }
}

#[test]
fn operators_longest_match() {
    assert_eq!(kinds("<=>="), [Le, Ge]);
    assert_eq!(kinds("<>="), [Ne, Eq]);
    assert_eq!(kinds(":=="), [Assign, Eq]);
    assert_eq!(kinds("::"), [Colon, Colon]);
    assert_eq!(kinds("< ="), [Lt, Eq]);
    assert_eq!(kinds(": ="), [Colon, Eq]);
    assert_eq!(kinds("..."), [DotDot, Dot]);
    assert_eq!(kinds("a:=b"), [Ident, Assign, Ident]);
    assert_eq!(kinds("a<>b"), [Ident, Ne, Ident]);
}

#[test]
fn alternate_brackets_in_context() {
    assert_eq!(kinds("a(.1.)"), [Ident, LBrack, IntLit, RBrack]);
    assert_eq!(
        kinds("a(.1..3.)"),
        [Ident, LBrack, IntLit, DotDot, IntLit, RBrack]
    );
    assert_eq!(kinds("a[1]"), [Ident, LBrack, IntLit, RBrack]);
}

#[test]
fn paren_star_is_a_comment_not_two_tokens() {
    assert_eq!(kinds("(*x*)"), []);
    assert_eq!(kinds("( *x"), [LParen, Star, Ident]);
}

#[test]
fn caret_after_identifier_is_dereference() {
    assert_eq!(kinds("p^.x"), [Ident, Caret, Dot, Ident]);
    assert_eq!(kinds("p^^"), [Ident, Caret, Caret]);
    assert_eq!(kinds("^integer"), [Caret, Ident]);
    assert_eq!(
        kinds("type P = ^Integer;"),
        [Type, Ident, Eq, Caret, Ident, Semi]
    );
    assert_eq!(kinds("p: ^Word"), [Ident, Colon, Caret, Ident]);
}

#[test]
fn caret_after_closer_is_dereference() {
    assert_eq!(kinds("a[1]^"), [Ident, LBrack, IntLit, RBrack, Caret]);
    assert_eq!(kinds("f(x)^"), [Ident, LParen, Ident, RParen, Caret]);
    assert_eq!(
        kinds("f(x)^.y"),
        [Ident, LParen, Ident, RParen, Caret, Dot, Ident]
    );
    assert_eq!(kinds("p^^.x"), [Ident, Caret, Caret, Dot, Ident]);
    assert_eq!(kinds("p^M"), [Ident, Caret, Ident]);
}

#[test]
fn standalone_control_character_is_ctrl_with_original_char() {
    assert_eq!(kinds("c := ^M;"), [Ident, Assign, Ctrl, Semi]);
    assert_eq!(toks("c := ^M;")[2].text, b"M");
    assert_eq!(
        kinds("Write(^G, 'x')"),
        [Ident, LParen, Ctrl, Comma, StrLit, RParen]
    );
    assert_eq!(toks("Write(^G, 'x')")[2].text, b"G");
    assert_eq!(kinds("write(^G)"), [Ident, LParen, Ctrl, RParen]);
    for (src, text) in [
        ("^A", "A"),
        ("^Z", "Z"),
        ("^[", "["),
        ("^@", "@"),
        ("^_", "_"),
        ("^m", "m"),
    ] {
        assert_eq!(
            raw_kind(src),
            (RawTk::Ctrl, text.as_bytes().to_vec()),
            "{src}"
        );
    }
}

#[test]
fn ctrl_in_type_context() {
    assert_eq!(kinds("type PT = ^T;"), [Type, Ident, Eq, Ctrl, Semi]);
    assert_eq!(toks("type PT = ^T;")[3].text, b"T");
}

#[test]
fn joined_control_characters_stay_one_string() {
    assert_eq!(string("^[^]"), [27, 29]);
    assert_eq!(string("^M^J"), [13, 10]);
    assert_eq!(string("^M'a'"), [13, b'a']);
    assert_eq!(string("'a'^M"), [b'a', 13]);
    assert_eq!(string("^M#10"), [13, 10]);
    assert_eq!(string("#10^M"), [10, 13]);
    assert_eq!(kinds("x = ^A'b'"), [Ident, Eq, StrLit]);
}

#[test]
fn control_character_position_is_the_caret() {
    let v = toks("c := ^M;");
    assert_eq!((v[2].line, v[2].col), (1, 6));
}

#[test]
fn small_program_token_stream() {
    let src = "program P; var x: integer; begin x := 1 + 2; writeln('hi', x) end.";
    assert_eq!(
        kinds(src),
        [
            Program, Ident, Semi, Var, Ident, Colon, Ident, Semi, Begin, Ident, Assign, IntLit,
            Plus, IntLit, Semi, Ident, LParen, StrLit, Comma, Ident, RParen, End, Dot
        ]
    );
}

#[test]
fn tokens_adjacent_without_whitespace() {
    assert_eq!(
        kinds("a+b*c-d/e"),
        [Ident, Plus, Ident, Star, Ident, Minus, Ident, Slash, Ident]
    );
    assert_eq!(
        kinds("f(a,b);"),
        [Ident, LParen, Ident, Comma, Ident, RParen, Semi]
    );
    assert_eq!(kinds("x:=-1"), [Ident, Assign, Minus, IntLit]);
    assert_eq!(kinds("1+2"), [IntLit, Plus, IntLit]);
}

#[test]
fn invalid_characters_are_error_5() {
    for (s, col) in [
        ("a ! b", 3),
        ("\"x\"", 1),
        ("x %", 3),
        ("&", 1),
        ("?", 1),
        ("\\", 1),
        ("`", 1),
        ("~", 1),
        ("|", 1),
    ] {
        assert_eq!(err(s), (5, 1, col), "{s}");
    }
    assert_eq!(err("a\n  !"), (5, 2, 3));
    assert_eq!(err("caf\u{e9}"), (5, 1, 4));
}

#[test]
fn nonterminating_garbage_after_valid_tokens_still_errors() {
    assert_eq!(err("begin end ;;; ?"), (5, 1, 15));
}

#[test]
fn string_end_before_a_period_does_not_stop_lexing() {
    assert_eq!(kinds("'END'. x"), [StrLit, Dot, Ident]);
}

#[test]
fn end_period_stops_lexing() {
    assert_eq!(kinds("end. x"), [End, Dot]);
}
