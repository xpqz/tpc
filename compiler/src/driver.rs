//! The compiler pipeline (load, check, codegen) and the native command line.
use crate::check::check;
use crate::codegen::gen;
use crate::diag::Diag;
use crate::units::{load, SourceFile};
use std::path::Path;

const USAGE: &str = "usage: tp5c in.pas -o out.wasm [--dir AUXDIR]";

/// Compiles the program `src` with no unit files.
pub fn compile(src: &[u8]) -> Result<Vec<u8>, Diag> {
    compile_files(src, &[])
}

/// Compiles the program `src`; `files` are the registered source files, whose n-th entry is
/// file number n + 1 in diagnostics.
pub fn compile_files(src: &[u8], files: &[SourceFile]) -> Result<Vec<u8>, Diag> {
    build(include_bytes!("../../runtime/system.pas"), src, files)
}

/// `compile` with `system` as the source of the System declaration part.
pub fn compile_with(system: &[u8], src: &[u8]) -> Result<Vec<u8>, Diag> {
    build(system, src, &[])
}

/// load (lex and parse every file), check, codegen.
fn build(system: &[u8], src: &[u8], files: &[SourceFile]) -> Result<Vec<u8>, Diag> {
    let (mut ast, plan) = load(system, src, files)?;
    let info = check(&mut ast, &plan)?;
    Ok(gen(&ast, &info, plan.program))
}

/// The `.pas` and `.inc` files (units and include files) of directory `dir`, by name; `skip` is
/// left out.
fn read_sources(dir: &Path, skip: Option<&std::ffi::OsStr>) -> std::io::Result<Vec<SourceFile>> {
    let mut paths = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        let is_source = path
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("pas") || e.eq_ignore_ascii_case("inc"));
        if is_source && path.is_file() && path.file_name() != skip {
            paths.push(path);
        }
    }
    paths.sort();
    let mut files = Vec::new();
    for path in paths {
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        files.push(SourceFile {
            name,
            text: std::fs::read(&path)?,
        });
    }
    Ok(files)
}

/// `tp5c in.pas -o out.wasm [--dir AUXDIR]`; returns the process exit code. Units are looked up
/// in the files beside the input, then in AUXDIR, whose files win on a name clash.
pub fn run(args: &[String]) -> i32 {
    let (input, output, aux) = match args {
        [input, o, output] if o == "-o" => (input, output, None),
        [input, o, output, d, dir] if o == "-o" && d == "--dir" => (input, output, Some(dir)),
        _ => return fail(USAGE, 2),
    };
    let Ok(src) = std::fs::read(input) else {
        return fail(&["tp5c: cannot read ", input].concat(), 2);
    };
    let input_path = Path::new(input);
    let beside = match input_path.parent() {
        Some(dir) if !dir.as_os_str().is_empty() => dir,
        _ => Path::new("."),
    };
    // A directory that cannot be read just has no units.
    let mut files = read_sources(beside, input_path.file_name()).unwrap_or_default();
    if let Some(dir) = aux {
        match read_sources(Path::new(dir), None) {
            Ok(aux_files) => files.extend(aux_files),
            Err(_) => return fail(&["tp5c: cannot read ", dir].concat(), 2),
        }
    }
    match compile_files(&src, &files) {
        Ok(wasm) => match std::fs::write(output, wasm) {
            Ok(()) => 0,
            Err(_) => fail(&["tp5c: cannot write ", output].concat(), 2),
        },
        Err(d) => {
            let name = match d.file {
                0 => input,
                n => &files[n as usize - 1].name,
            };
            fail(&crate::diag::render(name, &d), 1)
        }
    }
}

fn fail(msg: &str, code: i32) -> i32 {
    use std::io::Write;
    let mut e = std::io::stderr();
    let _ = e.write_all(msg.as_bytes());
    let _ = e.write_all(b"\n");
    code
}
