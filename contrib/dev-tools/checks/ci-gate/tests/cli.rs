//! End-to-end checks of the `ci-gate` binary against the global CLI output contract.
//!
//! The TTY refusal is not covered here: the test harness always pipes stdout, so the refusal cannot
//! be triggered without a pseudo-terminal. Its code is one early return in `main`, and manual
//! verification covers it (issue #2525, scenario M8).

use std::io::Write;
use std::process::{Command, Output, Stdio};

use serde_json::Value;

/// Runs the binary with the given arguments and changed paths on stdin, with stdout piped.
fn run_ci_gate(arguments: &[&str], stdin: &str) -> Output {
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
        .write_all(stdin.as_bytes())
        .expect("the changed paths should be written to stdin");

    child.wait_with_output().expect("the ci-gate binary should finish")
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
    let changed_paths = "README.md\nsrc/main.rs\n";

    // Act
    let output = run_ci_gate(&["classify", "--base-branch", "develop"], changed_paths);

    // Assert
    assert!(output.status.success(), "exit status was {:?}", output.status);
    assert!(
        output.stderr.is_empty(),
        "a successful run writes no diagnostics, but stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );
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
    let changed_paths = "docs/index.md\nproject-words.txt\n";

    // Act
    let output = run_ci_gate(&["classify", "--base-branch", "develop"], changed_paths);

    // Assert
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
    let output = run_ci_gate(&["classify"], "src/main.rs\n");

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
