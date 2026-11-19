//! `symbols.rs` is the scope and frame component. It must not depend on the checker, report
//! construct-specific errors, or rewrite syntax-tree nodes: the checker calls it, never the
//! reverse.
use std::path::Path;

/// Substrings that tie a module to the checker, to diagnostics, or to tree rewriting.
const FORBIDDEN: &[(&str, &str)] = &[
    ("crate::check", "depends on the checker"),
    ("super::check", "depends on the checker"),
    ("check::", "depends on the checker"),
    ("crate::diag", "reports diagnostics"),
    ("Diag", "reports diagnostics"),
    (".resolve(", "rewrites a node"),
    (".set_kind(", "rewrites a node"),
    (".set_ty(", "rewrites a node"),
    (".typed(", "builds a node"),
];

fn violations(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    for (n, line) in text.lines().enumerate() {
        let code = line.split("//").next().unwrap();
        for (pattern, why) in FORBIDDEN {
            if code.contains(pattern) {
                found.push(format!("line {}: {pattern} ({why})", n + 1));
            }
        }
    }
    found
}

fn symbols_source() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/symbols.rs");
    std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("missing compiler/src/symbols.rs"))
}

#[test]
fn symbols_module_exists_and_is_not_trivial() {
    let source = symbols_source();
    for needed in [
        "pub struct Symbols",
        "fn place_frame",
        "fn declare",
        "fn lookup",
    ] {
        assert!(source.contains(needed), "symbols.rs lacks {needed}");
    }
}

#[test]
fn symbols_module_does_not_depend_on_the_checker() {
    let bad = violations(&symbols_source());
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

#[test]
fn guard_recognises_each_forbidden_form() {
    for s in [
        "use crate::check::Helper;",
        "use super::check::Info;",
        "check::GLOBALS",
        "use crate::diag::code;",
        "fn f() -> Result<(), Diag>",
        "ast.resolve(id, kind, ty);",
        "ast.set_kind(id, kind);",
        "ast.set_ty(id, ty);",
        "ast.typed(kind, ty, at);",
    ] {
        assert!(!violations(s).is_empty(), "{s}");
    }
    for s in [
        "// uses crate::check",
        "use crate::types::Sym;",
        "self.ast.intern(name)",
    ] {
        assert!(violations(s).is_empty(), "{s}");
    }
}
