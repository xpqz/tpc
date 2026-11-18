//! `unsafe` is confined to compiler/src/ffi.rs, and nothing there is an unsafe block or fn.
use std::path::Path;

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

fn code(text: &str) -> String {
    text.lines()
        .map(|l| l.split("//").next().unwrap())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn unsafe_code_is_allowed_only_in_ffi() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    sources(&src, &mut files);
    for f in files {
        let text = code(&std::fs::read_to_string(&f).unwrap());
        let name = f.file_name().unwrap().to_str().unwrap();
        if name == "ffi.rs" {
            for bad in ["unsafe {", "unsafe fn", "unsafe impl", "unsafe extern"] {
                assert!(!text.contains(bad), "{name}: {bad}");
            }
        } else if name == "lib.rs" {
            assert!(text.starts_with("#![deny(unsafe_code)]"));
        } else {
            assert!(!text.contains("unsafe"), "{}", f.display());
        }
    }
}
