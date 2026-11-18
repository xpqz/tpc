//! The compiler crate must not pull in `std::fmt` (plan section 8).
use std::path::Path;

const BANNED: &[&str] = &[
    "format!",
    "print!",
    "println!",
    "eprint!",
    "eprintln!",
    "write!",
    "writeln!",
    "std::fmt",
    "core::fmt",
];

fn sources(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    for e in std::fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            sources(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

fn violations(text: &str) -> Vec<String> {
    let mut v = Vec::new();
    for (n, line) in text.lines().enumerate() {
        let code = line.split("//").next().unwrap();
        for b in BANNED {
            if code.contains(b) {
                v.push(format!("line {}: {b}", n + 1));
            }
        }
        // panic! takes a plain string literal only; arguments pull in formatting.
        for (i, _) in code.match_indices("panic!(") {
            let arg = &code[i + 7..];
            let plain = arg.starts_with('"') && !arg.contains('{') && arg.contains("\")");
            if !plain {
                v.push(format!("line {}: panic! with arguments", n + 1));
            }
        }
    }
    v
}

#[test]
fn compiler_sources_do_not_use_formatting() {
    let mut files = Vec::new();
    sources(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        &mut files,
    );
    assert!(files.len() >= 4, "{files:?}");
    let mut bad = Vec::new();
    for f in files {
        for v in violations(&std::fs::read_to_string(&f).unwrap()) {
            bad.push(format!("{}: {v}", f.display()));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

#[test]
fn guard_recognises_each_banned_form() {
    for s in [
        "format!(\"x\")",
        "println!()",
        "eprintln!(\"{x}\")",
        "write!(f, \"a\")",
        "use std::fmt;",
        "core::fmt::Debug",
        "panic!(\"{}\", x)",
        "panic!(msg)",
    ] {
        assert!(!violations(s).is_empty(), "{s}");
    }
    for s in [
        "panic!(\"bad\")",
        "// println!(\"x\")",
        "#[derive(Debug)]",
        "assert!(a, \"m\")",
    ] {
        assert!(violations(s).is_empty(), "{s}");
    }
}
