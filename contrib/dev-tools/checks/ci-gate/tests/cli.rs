//! End-to-end checks of the `ci-gate` binary against the global CLI output contract.
//!
//! The TTY refusal is not covered here: the test harness always pipes stdout, so the refusal cannot
//! be triggered without a pseudo-terminal. Its code is one early return in `main`, and manual
//! verification covers it (issue #2525, scenario M8).

use std::ffi::OsStr;
use std::io::Write;
use std::process::{Command, Output, Stdio};

use serde_json::Value;

/// Runs the binary with the given arguments and changed paths on stdin, with stdout piped.
fn run_ci_gate<S: AsRef<OsStr>>(arguments: &[S], stdin: &[u8]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ci-gate"))
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the ci-gate binary should start");

    child
        .stdin
        .take()
        .expect("stdin is piped")
        .write_all(stdin)
        .expect("the changed paths should be written to stdin");

    child.wait_with_output().expect("the ci-gate binary should finish")
}

/// Asserts the success contract: exit status 0 and nothing on stderr.
fn assert_succeeded_without_diagnostics(output: &Output) {
    assert!(output.status.success(), "exit status was {:?}", output.status);
    assert!(
        output.stderr.is_empty(),
        "a successful run writes no diagnostics, but stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn group<'a>(result: &'a Value, name: &str) -> &'a Value {
    result["groups"]
        .as_array()
        .expect("the result lists the groups")
        .iter()
        .find(|group| group["group"] == name)
        .unwrap_or_else(|| panic!("the result should list the group `{name}`"))
}

#[test]
fn it_should_print_one_json_object_on_stdout_and_nothing_on_stderr_when_it_classifies_a_change() {
    // Arrange
    let changed_paths = b"README.md\0src/main.rs\0";

    // Act
    let output = run_ci_gate(&["classify", "--base-branch", "develop"], changed_paths);

    // Assert
    assert_succeeded_without_diagnostics(&output);
    let stdout = String::from_utf8(output.stdout).expect("stdout is UTF-8");
    assert_eq!(
        stdout.lines().count(),
        1,
        "stdout must hold exactly one line, but was: {stdout:?}"
    );
    let result: Value = serde_json::from_str(&stdout).expect("stdout is one JSON object");
    assert_eq!(result["base_branch"], "develop");
    assert_eq!(result["changed_path_count"], 2);
    assert_eq!(group(&result, "docs-lint")["needed"], true);
    assert_eq!(group(&result, "testing")["needed"], true, "src/main.rs is not documentation");
    assert_eq!(group(&result, "security-scan")["needed"], false);
}

#[test]
fn it_should_not_need_the_code_groups_when_the_change_is_documentation_only() {
    // Arrange
    let changed_paths = b"docs/index.md\0project-words.txt\0";

    // Act
    let output = run_ci_gate(&["classify", "--base-branch", "develop"], changed_paths);

    // Assert
    assert_succeeded_without_diagnostics(&output);
    let result: Value = serde_json::from_slice(&output.stdout).expect("stdout is one JSON object");
    assert_eq!(
        (
            group(&result, "docs-lint")["needed"].clone(),
            group(&result, "testing")["needed"].clone()
        ),
        (Value::Bool(true), Value::Bool(false)),
        "a documentation-only change needs only docs-lint among these groups"
    );
}

#[test]
fn it_should_write_a_json_usage_error_on_stderr_and_nothing_on_stdout_when_the_base_branch_is_missing() {
    // Act
    // The binary rejects the arguments before it reads stdin, so none is written: writing to a process
    // that has already exited would fail the test with a broken pipe.
    let output = run_ci_gate(&["classify"], b"");

    // Assert
    assert_eq!(output.status.code(), Some(2), "a usage error exits with code 2");
    assert!(
        output.stdout.is_empty(),
        "a failed run writes nothing to stdout, but stdout was: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8(output.stderr).expect("stderr is UTF-8");
    assert_eq!(
        stderr.lines().count(),
        1,
        "one diagnostic record is expected, but stderr was: {stderr:?}"
    );
    let record: Value = serde_json::from_str(&stderr).expect("the diagnostic is one JSON object");
    assert_eq!(record["kind"], "usage_error");
    assert!(
        record["message"]
            .as_str()
            .is_some_and(|message| message.contains("--base-branch")),
        "the message should name the missing argument, but was: {record}"
    );
}

#[cfg(unix)]
#[test]
fn it_should_write_a_json_usage_error_instead_of_panicking_when_an_argument_is_not_valid_utf8() {
    use std::os::unix::ffi::OsStrExt;

    // Arrange
    let invalid_utf8_branch = OsStr::from_bytes(b"develop\xff");
    let arguments = [OsStr::new("classify"), OsStr::new("--base-branch"), invalid_utf8_branch];

    // Act
    // No stdin is written: the binary rejects the arguments before it reads it.
    let output = run_ci_gate(&arguments, b"");

    // Assert
    assert_eq!(
        output.status.code(),
        Some(2),
        "a usage error exits with code 2, but the run ended with {:?}; stderr was: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stdout.is_empty(),
        "a failed run writes nothing to stdout, but stdout was: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8(output.stderr).expect("stderr is UTF-8");
    let record: Value = serde_json::from_str(&stderr).expect("the diagnostic is one JSON object");
    assert_eq!(record["kind"], "usage_error");
    assert!(
        record["message"].as_str().is_some_and(|message| message.contains("UTF-8")),
        "the message should say that an argument is not valid UTF-8, but was: {record}"
    );
}

#[test]
fn it_should_need_the_code_groups_when_a_file_name_has_a_trailing_or_leading_space() {
    // Arrange
    let changed_paths = b"README.md \0 project-words.txt\0";

    // Act
    let output = run_ci_gate(&["classify", "--base-branch", "develop"], changed_paths);

    // Assert
    assert_succeeded_without_diagnostics(&output);
    let result: Value = serde_json::from_slice(&output.stdout).expect("stdout is one JSON object");
    assert_eq!(
        group(&result, "testing")["needed"],
        true,
        "`README.md ` and ` project-words.txt` are not the documentation files the workflows ignore"
    );
}

#[test]
fn it_should_need_the_database_groups_when_a_tracker_core_file_has_a_non_ascii_name() {
    // Arrange
    // `git diff --name-only -z` prints the name as it is; without `-z` git would write it quoted and escaped.
    let changed_paths = "packages/tracker-core/src/café.rs\0".as_bytes();

    // Act
    let output = run_ci_gate(&["classify", "--base-branch", "develop"], changed_paths);

    // Assert
    assert_succeeded_without_diagnostics(&output);
    let result: Value = serde_json::from_slice(&output.stdout).expect("stdout is one JSON object");
    assert_eq!(
        (
            group(&result, "db-compatibility")["needed"].clone(),
            group(&result, "db-benchmarking")["needed"].clone()
        ),
        (Value::Bool(true), Value::Bool(true)),
        "a file below `packages/tracker-core/` needs both database groups, whatever its name"
    );
}
