// Writes the runtime units without their comments to `OUT_DIR`; the driver embeds those copies,
// so the compiler carries the Pascal code and not its documentation.
//
// The stripper keeps what the compiler still reads or what a position depends on:
// - a compiler directive (`{$...}` or `(*$...*)`) is not a comment and is kept whole;
// - text inside a string literal is never a comment (`'{'`, `''''`);
// - `{ }` and `(* *)` do not nest and ignore each other's delimiters, as in TP;
// - every newline stays, so a line number in the stripped unit is the one in the file;
// - a removed comment leaves one space, so its neighbours stay separate tokens.
use std::env;
use std::fs;
use std::path::Path;

const UNITS: [&str; 5] = [
    "system.pas",
    "crt.pas",
    "dos.pas",
    "utils.pas",
    "idehost.pas",
];

fn main() {
    let manifest = env::var("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR");
    let out_dir = env::var("OUT_DIR").expect("cargo sets OUT_DIR");
    for unit in UNITS {
        let path = Path::new(&manifest).join("../runtime").join(unit);
        println!("cargo:rerun-if-changed={}", path.display());
        let source = fs::read(&path).expect("the runtime unit exists");
        fs::write(Path::new(&out_dir).join(unit), strip_comments(&source))
            .expect("OUT_DIR is writable");
    }
}

fn strip_comments(source: &[u8]) -> Vec<u8> {
    let mut stripped = Vec::with_capacity(source.len());
    let mut at = 0;
    while at < source.len() {
        let rest = &source[at..];
        let comment = comment_len(rest);
        let used = if rest[0] == b'\'' {
            let literal = string_literal_len(rest);
            stripped.extend_from_slice(&rest[..literal]);
            literal
        } else if comment > 0 && is_directive(rest) {
            stripped.extend_from_slice(&rest[..comment]);
            comment
        } else if comment > 0 {
            stripped.push(b' ');
            stripped.extend(rest[..comment].iter().filter(|&&byte| byte == b'\n'));
            comment
        } else {
            stripped.push(rest[0]);
            1
        };
        at += used;
    }
    trim_line_ends(&stripped)
}

/// The length of the string literal at the start of `text`: through the closing quote, or to
/// the end of the line if it is not closed. A doubled quote ends one literal and starts the next.
fn string_literal_len(text: &[u8]) -> usize {
    let closing = text[1..].iter().position(|&b| b == b'\'' || b == b'\n');
    match closing {
        Some(offset) if text[1 + offset] == b'\'' => offset + 2,
        Some(offset) => offset + 1,
        None => text.len(),
    }
}

/// The length of the comment at the start of `text`, delimiters included; 0 if there is none.
/// An unterminated comment runs to the end of the text.
fn comment_len(text: &[u8]) -> usize {
    let (opener, closer): (&[u8], &[u8]) = if text.starts_with(b"{") {
        (b"{", b"}")
    } else if text.starts_with(b"(*") {
        (b"(*", b"*)")
    } else {
        return 0;
    };
    let body = &text[opener.len()..];
    match body.windows(closer.len()).position(|w| w == closer) {
        Some(offset) => opener.len() + offset + closer.len(),
        None => text.len(),
    }
}

fn is_directive(comment: &[u8]) -> bool {
    comment.starts_with(b"{$") || comment.starts_with(b"(*$")
}

/// Removes the blanks a stripped comment left at the end of lines.
fn trim_line_ends(text: &[u8]) -> Vec<u8> {
    let lines: Vec<&[u8]> = text
        .split(|&b| b == b'\n')
        .map(|line| line.trim_ascii_end())
        .collect();
    lines.join(&b'\n')
}
