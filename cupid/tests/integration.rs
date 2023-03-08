use std::path::PathBuf;
use std::process::ExitStatus;
use std::{env, fs, process::Command};

extern crate cupid_fmt;
extern crate regex;
extern crate test_generator;

use cupid_fmt::color;
use regex::Regex;
use test_generator::test_resources;

fn cupid_command() -> Command {
    // Create full path to binary
    let mut path = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned();
    path.push(env!("CARGO_PKG_NAME"));
    path.set_extension(env::consts::EXE_EXTENSION);
    Command::new(path.into_os_string())
}

#[derive(Debug)]
struct RuntimeError {
    line_prefix: String,
    message: String,
}

struct Expected {
    out: Vec<String>,
    compile_err: Vec<String>,
    runtime_err: Option<RuntimeError>,
}

fn parse_comments(path: &PathBuf) -> Expected {
    let output_re = Regex::new(r"-- expect: ?(.*)").unwrap();
    let error_re = Regex::new(r"-- (Error.*)").unwrap();
    let error_line_re = Regex::new(r"-- \[(?:c )?line (\d+)\] (Error.*)").unwrap();
    let runtime_error_re = Regex::new(r"-- expect runtime error: (.+)").unwrap();

    let mut expected = Expected {
        out: vec![],
        compile_err: vec![],
        runtime_err: None,
    };

    println!("{}", path.display());
    let content = fs::read_to_string(path).unwrap();
    for (i, line) in content.lines().enumerate() {
        if let Some(m) = output_re.captures(line) {
            let s = m.get(1).unwrap().as_str().to_owned();
            expected.out.push(s);
        }
        if let Some(m) = error_line_re.captures(line) {
            let line = m.get(1).unwrap().as_str();
            let msg = m.get(2).unwrap().as_str();
            let s = format!("[line {}] {}", line, msg);
            expected.compile_err.push(s);
        }
        if let Some(m) = error_re.captures(line) {
            let msg = m.get(1).unwrap().as_str();
            let s = format!("[line {}] {}", i + 1, msg);
            expected.compile_err.push(s);
        }
        if let Some(m) = runtime_error_re.captures(line) {
            let message = m.get(1).unwrap().as_str().to_owned();
            let line_prefix = format!("[line {}]", i + 1);
            expected.runtime_err = Some(RuntimeError {
                line_prefix,
                message,
            });
        }
    }
    expected
}

fn fmt_test_problem(msg: &str, expected: &Expected, errors: &[String]) -> String {
    let expected_compile_err = !expected.compile_err.is_empty();
    let expected_runtime_err = expected.runtime_err.is_some();
    let expected_err = match (expected_compile_err, expected_runtime_err) {
        (true, false) => format!("{:?}", expected.compile_err),
        (false, true) => format!("{:?}", expected.runtime_err),
        (false, false) => String::from("success"),
        (true, true) => format!(
            "compile error {:?}, runtime error {:?}",
            expected.compile_err, expected.runtime_err
        ),
    };
    let pipe = color("│").dim().ok();
    vec![
        color("\n╭───").dim().ok(),
        format!("{pipe} {}", color(msg).bold().red()),
        format!("{pipe} {} {}", color("Expected ──▶︎").red(), expected_err),
        format!("{pipe} {} {:?}", color("Found ─────▶︎").red(), errors),
        color("╰───\n").dim().ok(),
    ]
    .join("\n")
}

#[test_resources("tests/integration/*/*.cupid")]
fn run_file_test(filename: &str) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(filename);
    let expected = parse_comments(&path);

    let output = cupid_command().arg(path).output().unwrap();

    let out: Vec<String> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|x| x.to_owned())
        .collect();
    let err: Vec<String> = String::from_utf8(output.stderr)
        .unwrap()
        .lines()
        .map(|x| x.to_owned())
        .collect();

    let formatted = |msg| fmt_test_problem(msg, &expected, &err);

    match (
        expected.runtime_err.is_none(),
        expected.compile_err.is_empty(),
    ) {
        (true, true) => assert!(
            output.status.success(),
            "{}",
            formatted("Program unexpectedly exited with failure")
        ),
        (false, true) => assert_eq!(
            output.status.code().unwrap(),
            70,
            "{}",
            formatted("Found runtime error when expecting compile error")
        ),
        (true, false) => assert_eq!(
            output.status.code().unwrap(),
            65,
            "{}",
            formatted("Found compile error when expecting runtime error")
        ),
        (false, false) => panic!("{}", formatted("Simultaneous error and compile error")),
    }

    if let Some(e) = &expected.runtime_err {
        assert_eq!(
            e.message,
            err[0],
            "{}",
            formatted("Runtime error should match")
        );
        assert_eq!(
            err[1][0..e.line_prefix.len()],
            e.line_prefix,
            "{}",
            formatted("Runtime error line should match")
        );
    } else {
        if !err.is_empty() {
            assert_eq!(
                output.status.code().unwrap(),
                65,
                "{}",
                formatted("Compile errors should have error code 65")
            );
        }
        assert_eq!(
            expected.compile_err,
            err,
            "{}",
            formatted("Compile error should match")
        );
    }

    assert_eq!(expected.out, out, "{}", formatted("Output should match"));
}
