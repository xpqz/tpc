//! The comment stripper of `build.rs`, which writes the embedded runtime units. The build script
//! is included here so the tests call the function the build uses.
#![allow(dead_code)]
include!("../build.rs");

mod sysutil;
use sysutil::*;

fn stripped(source: &str) -> String {
    String::from_utf8(strip_comments(source.as_bytes())).unwrap()
}

#[test]
fn a_brace_comment_becomes_one_space() {
    assert_eq!(stripped("a{note}b"), "a b");
}

#[test]
fn a_paren_star_comment_becomes_one_space() {
    assert_eq!(stripped("a(* note *)b"), "a b");
}

#[test]
fn brace_in_a_string_is_not_a_comment() {
    assert_eq!(
        stripped("x := '{'; { real } y := '}'"),
        "x := '{';   y := '}'"
    );
    assert_eq!(stripped("s := '(*'; (* c *)"), "s := '(*';");
}

#[test]
fn doubled_quote_does_not_end_the_string_early() {
    assert_eq!(
        stripped("s := 'it''s {not}'; {gone}"),
        "s := 'it''s {not}';"
    );
    assert_eq!(stripped("c := ''''; {gone}"), "c := '''';");
}

#[test]
fn a_quote_inside_a_comment_does_not_open_a_string() {
    assert_eq!(stripped("{ don't } x := '{';"), "  x := '{';");
}

#[test]
fn the_two_comment_kinds_ignore_each_others_delimiters() {
    assert_eq!(stripped("a{ (* }b"), "a b");
    assert_eq!(stripped("a(* { *)b"), "a b");
    assert_eq!(stripped("a{ *) }b(* } *)c"), "a b c");
}

#[test]
fn directives_are_kept_whole() {
    assert_eq!(stripped("{$R+} {c} {$I x.inc}"), "{$R+}   {$I x.inc}");
    assert_eq!(stripped("(*$R-*) (*c*)"), "(*$R-*)");
}

#[test]
fn a_comment_across_lines_keeps_its_newlines() {
    let out = stripped("a {one\ntwo\nthree} b\nc");
    assert_eq!(out, "a\n\n b\nc");
    assert_eq!(out.matches('\n').count(), 3);
}

#[test]
fn trailing_blanks_are_removed_but_lines_stay() {
    assert_eq!(stripped("  { only a comment }\nx\n\n"), "\nx\n\n");
}

#[test]
fn an_unterminated_comment_runs_to_the_end() {
    assert_eq!(stripped("a {oops\nb"), "a\n");
}

#[test]
fn every_runtime_unit_keeps_its_line_count_and_directives_and_loses_bytes() {
    let out_dir = std::path::Path::new(env!("OUT_DIR"));
    let mut total = (0, 0);
    for unit in UNITS {
        let file = std::fs::read(root().join("runtime").join(unit)).unwrap();
        let embedded = std::fs::read(out_dir.join(unit)).unwrap();
        let lines = |text: &[u8]| text.iter().filter(|&&b| b == b'\n').count();
        assert_eq!(lines(&embedded), lines(&file), "{unit}");
        assert!(embedded.len() < file.len(), "{unit}");
        let directives = |text: &[u8]| text.windows(2).filter(|w| *w == b"{$").count();
        assert_eq!(directives(&embedded), directives(&file), "{unit}");
        total = (total.0 + file.len(), total.1 + embedded.len());
    }
    println!(
        "runtime units: {} bytes in the files, {} embedded",
        total.0, total.1
    );
}
