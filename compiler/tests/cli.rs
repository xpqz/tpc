mod common;
mod driver;
use std::path::Path;
use std::process::{Command, Output};

fn tp5c(cwd: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_tp5c"))
        .current_dir(cwd)
        .args(args)
        .output()
        .unwrap()
}

fn err(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}

fn dir() -> std::path::PathBuf {
    let d = driver::scratch("x").parent().unwrap().to_path_buf();
    std::fs::write(d.join("t.pas"), "begin end.\n").unwrap();
    std::fs::create_dir_all(d.join("sub")).unwrap();
    std::fs::write(d.join("sub/u.pas"), "begin end.\n").unwrap();
    d
}

const USAGE: &str = "usage: tp5c in.pas -o out.wasm\n";

#[test]
fn bad_argument_lists_print_usage_and_exit_2() {
    let d = dir();
    let cases: &[&[&str]] = &[
        &[],
        &["t.pas"],
        &["t.pas", "-o"],
        &["t.pas", "out.wasm"],
        &["t.pas", "-o", "a.wasm", "b.wasm"],
        &["-o", "a.wasm", "t.pas"],
        &["-x", "t.pas", "-o", "a.wasm"],
    ];
    for args in cases {
        let o = tp5c(&d, args);
        assert_eq!(o.status.code(), Some(2), "{args:?}");
        assert_eq!(err(&o), USAGE, "{args:?}");
        assert!(o.stdout.is_empty(), "{args:?}");
    }
}

#[test]
fn missing_input_file_exits_2_and_writes_nothing() {
    let d = dir();
    let o = tp5c(&d, &["nope.pas", "-o", "out.wasm"]);
    assert_eq!(o.status.code(), Some(2));
    assert_eq!(err(&o), "tp5c: cannot read nope.pas\n");
    assert!(!d.join("out.wasm").exists());
}

#[test]
fn compile_error_is_one_line_on_stderr_with_exit_1() {
    let d = dir();
    let o = tp5c(&d, &["t.pas", "-o", "out.wasm"]);
    assert_eq!(o.status.code(), Some(1));
    assert_eq!(err(&o), "t.pas(1,1): Error 200: Feature not supported\n");
    assert!(o.stdout.is_empty());
}

#[test]
fn compile_error_does_not_create_the_output_file() {
    let d = dir();
    tp5c(&d, &["t.pas", "-o", "out.wasm"]);
    assert!(!d.join("out.wasm").exists());
}

#[test]
fn diagnostic_name_is_the_input_path_as_given() {
    let d = dir();
    let o = tp5c(&d, &["sub/u.pas", "-o", "out.wasm"]);
    assert_eq!(
        err(&o),
        "sub/u.pas(1,1): Error 200: Feature not supported\n"
    );
    let abs = d.join("t.pas");
    let o = tp5c(&d, &[abs.to_str().unwrap(), "-o", "out.wasm"]);
    let want = format!("{}(1,1): Error 200: Feature not supported\n", abs.display());
    assert_eq!(err(&o), want);
}

#[test]
fn non_utf8_source_is_read_as_bytes() {
    let d = dir();
    std::fs::write(d.join("b.pas"), [0xff, 0xfe, b'\n']).unwrap();
    let o = tp5c(&d, &["b.pas", "-o", "out.wasm"]);
    assert_eq!(o.status.code(), Some(1));
    assert!(err(&o).starts_with("b.pas(1,1): Error 200"));
}
