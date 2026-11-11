//! End-to-end checks of the `package-coverage-check` binary against the global CLI output contract.
//!
//! The TTY refusal is not covered here: the test harness always pipes stdout, so the refusal cannot
//! be triggered without a pseudo-terminal. Its code is one early return in `run`, and manual
//! verification covers it (issue #2762).

use std::ffi::OsStr;
use std::fs;
use std::process::{Command, Output, Stdio};

use serde_json::{Value, json};
use tempfile::tempdir;

fn run<S: AsRef<OsStr>>(arguments: &[S]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_package-coverage-check"))
        .args(arguments)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("the package-coverage-check binary should run")
}

/// Asserts the failure contract: empty stdout and exactly one JSON record with the expected `kind` on stderr.
fn assert_failed(output: &Output, expected_kind: &str, expected_exit_code: i32) {
    assert_eq!(output.status.code(), Some(expected_exit_code));
    assert!(output.stdout.is_empty(), "a failed run writes nothing to stdout");
    let stderr = String::from_utf8(output.stderr.clone()).expect("stderr is UTF-8");
    let lines: Vec<&str> = stderr.lines().collect();
    assert_eq!(lines.len(), 1, "stderr should hold one record, but was: {stderr}");
    let record: Value = serde_json::from_str(lines[0]).expect("the stderr record is JSON");
    assert_eq!(record["kind"], expected_kind);
    assert!(record["message"].is_string(), "the record carries a message");
}

#[test]
fn it_should_report_an_unknown_subcommand_as_a_usage_error_with_exit_code_two() {
    let output = run(&["bogus"]);

    assert_failed(&output, "usage_error", 2);
}

#[test]
fn it_should_report_missing_arguments_as_a_usage_error_with_exit_code_two() {
    let output = run(&["summary", "only-one-argument"]);

    assert_failed(&output, "usage_error", 2);
}

#[test]
fn it_should_report_a_missing_subcommand_as_a_usage_error_with_exit_code_two() {
    let output = run::<&str>(&[]);

    assert_failed(&output, "usage_error", 2);
}

#[test]
fn it_should_report_an_unreadable_input_as_a_runtime_error_with_exit_code_one() {
    let directory = tempdir().unwrap();
    let missing = directory.path().join("missing.json");

    let output = run(&[OsStr::new("summary"), missing.as_os_str(), directory.path().as_os_str()]);

    assert_failed(&output, "runtime_error", 1);
}

#[test]
fn it_should_print_the_summary_markdown_in_the_markdown_field_of_one_json_object() {
    // Arrange
    let directory = tempdir().unwrap();
    let discovery = directory.path().join("discovery.json");
    fs::write(
        &discovery,
        json!({ "matrix": { "include": [] }, "unavailable": [] }).to_string(),
    )
    .unwrap();
    let artifacts = directory.path().join("artifacts");
    fs::create_dir(&artifacts).unwrap();

    // Act
    let output = run(&[OsStr::new("summary"), discovery.as_os_str(), artifacts.as_os_str()]);

    // Assert
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty(), "a successful run writes no diagnostics");
    let stdout = String::from_utf8(output.stdout).expect("stdout is UTF-8");
    assert_eq!(stdout.lines().count(), 1, "stdout is one line");
    let result: Value = serde_json::from_str(&stdout).expect("stdout is one JSON object");
    let markdown = result["markdown"].as_str().expect("the result has a markdown field");
    assert!(markdown.starts_with("## Package Coverage Regression"));
    assert!(markdown.contains("No directly changed workspace package was selected."));
}
