//! The compiler pipeline (load, check, codegen) and the native command line.
//!
//! Unit files come from the files beside the input, then from `--dir`; both are the `.pas` and
//! `.inc` files of a directory, and a later file of the same name wins. The units of the
//! runtime (`EMBEDDED_UNITS`) are found first and need no file.
use crate::check::check;
use crate::codegen::gen;
use crate::diag::Diag;
use crate::units::{load, EmbeddedUnit, SourceFile};
use std::path::Path;

const USAGE: &str = "usage: tp5c in.pas -o out.wasm [--dir AUXDIR]";

/// The units of the runtime that `uses` finds without a file (docs/decisions.md), as `build.rs`
/// writes them: comments removed.
const EMBEDDED_UNITS: &[EmbeddedUnit] = &[
    EmbeddedUnit {
        name: "CRT",
        source: include_bytes!(concat!(env!("OUT_DIR"), "/crt.pas")),
    },
    EmbeddedUnit {
        name: "DOS",
        source: include_bytes!(concat!(env!("OUT_DIR"), "/dos.pas")),
    },
    EmbeddedUnit {
        name: "UTILS",
        source: include_bytes!(concat!(env!("OUT_DIR"), "/utils.pas")),
    },
    EmbeddedUnit {
        name: "IDEHOST",
        source: include_bytes!(concat!(env!("OUT_DIR"), "/idehost.pas")),
    },
];

/// Compiles the program `src` with no unit files.
pub fn compile(src: &[u8]) -> Result<Vec<u8>, Diag> {
    compile_files(src, &[])
}

/// Compiles the program `src`; `files` are the registered source files, whose n-th entry is
/// file number n + 1 in diagnostics.
pub fn compile_files(src: &[u8], files: &[SourceFile]) -> Result<Vec<u8>, Diag> {
    build(
        include_bytes!(concat!(env!("OUT_DIR"), "/system.pas")),
        src,
        files,
    )
}

/// `compile` with `system` as the source of the System declaration part.
pub fn compile_with(system: &[u8], src: &[u8]) -> Result<Vec<u8>, Diag> {
    build(system, src, &[])
}

/// load (lex and parse every file), check, codegen.
fn build(system: &[u8], src: &[u8], files: &[SourceFile]) -> Result<Vec<u8>, Diag> {
    let (mut ast, plan) = load(system, src, files, EMBEDDED_UNITS)?;
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

/// The input, output and optional auxiliary directory named by the command line.
fn parse_args(args: &[String]) -> Option<(&String, &String, Option<&String>)> {
    match args {
        [input, o, output] if o == "-o" => Some((input, output, None)),
        [input, o, output, d, dir] if o == "-o" && d == "--dir" => Some((input, output, Some(dir))),
        _ => None,
    }
}

/// The unit and include files of a compilation of `input`: the files beside it, then those of
/// `aux`, which win on a name clash. An unreadable `aux` is an error message.
fn unit_files(input: &str, aux: Option<&String>) -> Result<Vec<SourceFile>, String> {
    let input_path = Path::new(input);
    let beside = match input_path.parent() {
        Some(dir) if !dir.as_os_str().is_empty() => dir,
        _ => Path::new("."),
    };
    // A directory that cannot be read just has no units.
    let mut files = read_sources(beside, input_path.file_name()).unwrap_or_default();
    if let Some(dir) = aux {
        let aux_files = read_sources(Path::new(dir), None);
        files.extend(aux_files.map_err(|_| ["tp5c: cannot read ", dir].concat())?);
    }
    Ok(files)
}

/// The diagnostic line for `d`, naming the file it is in: the input, or a registered file.
fn describe(d: &Diag, input: &str, files: &[SourceFile]) -> String {
    let name = match d.file {
        0 => input,
        n => &files[n as usize - 1].name,
    };
    crate::diag::render(name, d)
}

/// `tp5c in.pas -o out.wasm [--dir AUXDIR]`; returns the process exit code. Units are looked up
/// in the files beside the input, then in AUXDIR, whose files win on a name clash.
pub fn run(args: &[String]) -> i32 {
    let Some((input, output, aux)) = parse_args(args) else {
        return fail(USAGE, 2);
    };
    let Ok(src) = std::fs::read(input) else {
        return fail(&["tp5c: cannot read ", input].concat(), 2);
    };
    let files = match unit_files(input, aux) {
        Ok(files) => files,
        Err(message) => return fail(&message, 2),
    };
    match compile_files(&src, &files) {
        Ok(wasm) => match std::fs::write(output, wasm) {
            Ok(()) => 0,
            Err(_) => fail(&["tp5c: cannot write ", output].concat(), 2),
        },
        Err(d) => fail(&describe(&d, input, &files), 1),
    }
}

fn fail(msg: &str, code: i32) -> i32 {
    use std::io::Write;
    let mut e = std::io::stderr();
    let _ = e.write_all(msg.as_bytes());
    let _ = e.write_all(b"\n");
    code
}
