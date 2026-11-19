//! Deviations of the cheap standard routines (docs/deviations.md D43 and D44). The oracle
//! cannot give these expectations: its environment holds variables, and its 40-column and
//! 50-line modes change the screen size. The corpus programs `dos_getenv_and_dosversion` and
//! `crt_textmode_*` hold the parts of these routines that agree with TP 5.0.
mod common;
mod driver;
use driver::*;

/// Compiles and runs `source` and compares its output with `expected`.
fn assert_prints(stem: &str, source: &str, expected: &str) {
    let dir = scratch("x").parent().unwrap().to_path_buf();
    std::fs::write(dir.join(format!("{stem}.pas")), source).unwrap();
    std::fs::write(dir.join(format!("{stem}.out")), expected).unwrap();
    let failures = check_lang(&dir, &real_compile);
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn environment_is_empty() {
    assert_prints(
        "env_empty",
        "program Env;\nuses Dos;\nbegin\n  WriteLn(EnvCount);\n  WriteLn('[', GetEnv('PATH'), ']');\n  \
         WriteLn('[', EnvStr(1), ']');\n  WriteLn('[', GetEnv('COMSPEC'), ']');\nend.\n",
        "0\n[]\n[]\n[]\n",
    );
}

#[test]
fn dos_version_is_five() {
    assert_prints(
        "dos_version",
        "program V;\nuses Dos;\nbegin\n  WriteLn(DosVersion);\nend.\n",
        "5\n",
    );
}

#[test]
fn forty_column_and_fifty_line_modes_keep_the_80x25_screen() {
    // WindMax is column 79, row 24 for every mode: 24 * 256 + 79 = 6223.
    let source = "program M;\nuses Crt;\nvar a, b, c: Word;\nbegin\n  TextMode(CO40);\n  \
        a := WindMax;\n  TextMode(BW40);\n  b := WindMax;\n  TextMode(CO80 + Font8x8);\n  \
        c := WindMax;\n  TextMode(CO80);\n  WriteLn(a, ' ', b, ' ', c);\nend.\n";
    let screen = "\n".repeat(24);
    let expected = format!("6223 6223 6223\n{screen}cursor 1 2\nexit 0\n");
    assert_prints("mode_sizes", source, &expected);
}
