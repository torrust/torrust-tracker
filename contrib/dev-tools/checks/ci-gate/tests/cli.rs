//! End-to-end checks of the `ci-gate` binary against the global CLI output contract.
//!
//! The TTY refusal is not covered here: the test harness always pipes stdout, so the refusal cannot
//! be triggered without a pseudo-terminal. Its code is one early return in `main`, and manual
//! verification covers it (issue #2525, scenario M8).

use std::ffi::OsStr;
use std::io::Write;
use std::process::{Command, Output, Stdio};

use serde_json::{Value, json};

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

/// The `classify` output for a change to `changed_paths` into `develop`, produced by the binary itself.
fn classify_output(changed_paths: &[u8]) -> Value {
    let output = run_ci_gate(&["classify", "--base-branch", "develop"], changed_paths);
    assert_succeeded_without_diagnostics(&output);
    serde_json::from_slice(&output.stdout).expect("classify prints one JSON object")
}

/// Runs `verdict` with the given extra arguments on the `classify` output and the given results.
fn run_verdict(extra_arguments: &[&str], classification: &Value, results: &Value) -> Output {
    let input = json!({ "classification": classification, "results": results });
    let arguments: Vec<&str> = std::iter::once("verdict").chain(extra_arguments.iter().copied()).collect();
    run_ci_gate(&arguments, input.to_string().as_bytes())
}

fn parse_verdict_stdout(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).expect("stdout is one JSON object")
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

#[test]
fn it_should_pass_when_it_is_given_the_classify_output_and_every_needed_group_succeeded() {
    // Arrange
    let classification = classify_output(b"docs/index.md\0");
    let results = json!({ "docs-lint": "success" });

    // Act
    let output = run_verdict(&[], &classification, &results);

    // Assert
    assert_succeeded_without_diagnostics(&output);
    let result = parse_verdict_stdout(&output);
    assert_eq!(result["verdict"], "pass");
    assert_eq!(result["problems"], json!([]));
    assert_eq!(result["overridden_by"], Value::Null);
}

#[test]
fn it_should_exit_with_zero_and_print_a_failing_verdict_naming_the_group_when_a_needed_group_failed() {
    // Arrange
    let classification = classify_output(b"src/main.rs\0");
    let results = json!({ "docs-lint": "success", "testing": "failure" });

    // Act
    let output = run_verdict(&[], &classification, &results);

    // Assert
    // The global CLI output contract gives a result exit code 0, so a failing verdict is data that the
    // workflow reads from `verdict`, not a failed command.
    assert_succeeded_without_diagnostics(&output);
    let result = parse_verdict_stdout(&output);
    assert_eq!(result["verdict"], "fail");
    let explanation = result["explanation"].as_str().expect("the explanation is a string");
    assert!(
        explanation.contains("testing (failure)") && explanation.contains("os-compatibility (missing)"),
        "the explanation should name a failed group and a group that reported nothing, but was: {explanation}"
    );
}

#[test]
fn it_should_pass_naming_the_actor_when_the_override_actor_is_given_and_a_needed_group_failed() {
    // Arrange
    let classification = classify_output(b"src/main.rs\0");
    let results = json!({ "docs-lint": "success", "testing": "failure" });

    // Act
    let output = run_verdict(&["--override-actor", "josecelano"], &classification, &results);

    // Assert
    assert_succeeded_without_diagnostics(&output);
    let result = parse_verdict_stdout(&output);
    assert_eq!(
        (result["verdict"].clone(), result["overridden_by"].clone()),
        (json!("pass"), json!("josecelano"))
    );
}

#[test]
fn it_should_write_a_json_runtime_error_and_nothing_on_stdout_when_stdin_is_not_a_verdict_input() {
    // Arrange
    let not_a_verdict_input = b"not json";

    // Act
    let output = run_ci_gate(&["verdict"], not_a_verdict_input);

    // Assert
    assert_eq!(
        output.status.code(),
        Some(1),
        "a runtime failure exits with code 1, so the gate fails closed"
    );
    assert!(
        output.stdout.is_empty(),
        "a failed run writes nothing to stdout, but stdout was: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8(output.stderr).expect("stderr is UTF-8");
    let record: Value = serde_json::from_str(&stderr).expect("the diagnostic is one JSON object");
    assert_eq!(record["kind"], "runtime_error");
}

/// A stand-in for the `gh` program, made of fixtures in a temporary directory. A request for which there is no fixture
/// fails, as a request that GitHub refuses does.
#[cfg(unix)]
struct FakeGh {
    directory: std::path::PathBuf,
}

#[cfg(unix)]
impl FakeGh {
    fn new(test_name: &str) -> Self {
        use std::os::unix::fs::PermissionsExt;

        let directory = std::env::temp_dir().join(format!("ci-gate-fake-gh-{}-{test_name}", std::process::id()));
        std::fs::create_dir_all(&directory).expect("the fixture directory can be created");
        // `gh api --paginate <path>`: the fixture of a path is the file named after it, with `/`, `?` and `=` as `_`.
        let script = format!(
            "#!/bin/sh\nfile=\"{}/$(printf '%s' \"$3\" | tr '/?=' '___')\"\nif [ -f \"$file\" ]; then cat \"$file\"; else echo \"no fixture for $3\" >&2; exit 1; fi\n",
            directory.display()
        );
        let program = directory.join("gh");
        std::fs::write(&program, script).expect("the fake gh can be written");
        std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o755)).expect("the fake gh can be made executable");

        Self { directory }
    }

    fn program(&self) -> String {
        self.directory.join("gh").display().to_string()
    }

    fn respond(&self, path: &str, body: &Value) {
        let name = path.replace(['/', '?', '='], "_");
        std::fs::write(self.directory.join(name), body.to_string()).expect("the fixture can be written");
    }

    /// Answers a path with text as it is, for a response that is not one JSON document.
    fn respond_with_text(&self, path: &str, text: &str) {
        let name = path.replace(['/', '?', '='], "_");
        std::fs::write(self.directory.join(name), text).expect("the fixture can be written");
    }

    fn write_classification(&self, classification: &Value) -> String {
        let path = self.directory.join("classification.json");
        std::fs::write(&path, classification.to_string()).expect("the classification can be written");
        path.display().to_string()
    }
}

#[cfg(unix)]
impl Drop for FakeGh {
    fn drop(&mut self) {
        drop(std::fs::remove_dir_all(&self.directory));
    }
}

#[cfg(unix)]
const SHA: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

#[cfg(unix)]
fn run_github_verdict(gh: &FakeGh, classification_path: &str) -> Output {
    run_ci_gate(
        &[
            "github-verdict",
            "--repo",
            "owner/repo",
            "--pull-request",
            "7",
            "--head-sha",
            SHA,
            "--classification",
            classification_path,
            "--gh",
            &gh.program(),
        ],
        b"",
    )
}

#[cfg(unix)]
fn check_run(id: u64, name: &str, conclusion: &str) -> Value {
    json!({ "id": id, "name": name, "status": "completed", "conclusion": conclusion, "app": { "slug": "github-actions" } })
}

#[cfg(unix)]
fn respond_with_check_runs(gh: &FakeGh, runs: &[Value]) {
    gh.respond(
        &format!("repos/owner/repo/commits/{SHA}/check-runs?per_page=100"),
        &json!({ "total_count": runs.len(), "check_runs": runs }),
    );
}

#[cfg(unix)]
fn respond_with_an_override(gh: &FakeGh, actor: &str, applied_at: &str, first_check_suite_at: &str) {
    gh.respond(
        "repos/owner/repo/issues/7/labels?per_page=100",
        &json!([{ "name": "ci-override" }]),
    );
    gh.respond(
        "repos/owner/repo/issues/7/events?per_page=100",
        &json!([{ "event": "labeled", "label": { "name": "ci-override" }, "actor": { "login": actor }, "created_at": applied_at }]),
    );
    gh.respond(
        &format!("repos/owner/repo/commits/{SHA}/check-suites?per_page=100"),
        &json!({ "check_suites": [{ "created_at": first_check_suite_at }] }),
    );
    gh.respond(
        &format!("repos/owner/repo/collaborators/{actor}/permission"),
        &json!({ "permission": "admin" }),
    );
}

#[cfg(unix)]
#[test]
fn it_should_print_a_passing_verdict_when_the_github_check_runs_show_every_needed_group_succeeded() {
    // Arrange
    let gh = FakeGh::new("pass");
    let classification = gh.write_classification(&classify_output(b"docs/index.md\0"));
    respond_with_check_runs(&gh, &[check_run(1, "docs-lint", "success")]);

    // Act
    let output = run_github_verdict(&gh, &classification);

    // Assert
    assert_succeeded_without_diagnostics(&output);
    let result = parse_verdict_stdout(&output);
    assert_eq!(
        (result["verdict"].clone(), result["override_ignored"].clone()),
        (json!("pass"), Value::Null)
    );
}

#[cfg(unix)]
#[test]
fn it_should_exit_with_zero_and_print_a_failing_verdict_when_a_group_failed_and_there_is_no_override() {
    // Arrange
    let gh = FakeGh::new("fail");
    let classification = gh.write_classification(&classify_output(b"packages/persistence-benchmark/note.md\0"));
    respond_with_check_runs(
        &gh,
        &[
            check_run(1, "docs-lint", "success"),
            check_run(2, "db-benchmarking", "failure"),
        ],
    );
    gh.respond("repos/owner/repo/issues/7/labels?per_page=100", &json!([]));
    gh.respond("repos/owner/repo/issues/7/events?per_page=100", &json!([]));

    // Act
    let output = run_github_verdict(&gh, &classification);

    // Assert
    assert_succeeded_without_diagnostics(&output);
    let result = parse_verdict_stdout(&output);
    assert_eq!(result["verdict"], "fail");
    assert_eq!(
        result["explanation"],
        "needed groups without a success: db-benchmarking (failure)"
    );
}

#[cfg(unix)]
#[test]
fn it_should_print_a_passing_verdict_naming_the_actor_when_a_maintainer_applied_the_label_after_the_push() {
    // Arrange
    let gh = FakeGh::new("override");
    let classification = gh.write_classification(&classify_output(b"packages/persistence-benchmark/note.md\0"));
    respond_with_check_runs(
        &gh,
        &[
            check_run(1, "docs-lint", "success"),
            check_run(2, "db-benchmarking", "failure"),
        ],
    );
    respond_with_an_override(&gh, "another-maintainer", "2026-10-21T10:49:13Z", "2026-10-21T10:46:06Z");

    // Act
    let output = run_github_verdict(&gh, &classification);

    // Assert
    assert_succeeded_without_diagnostics(&output);
    let result = parse_verdict_stdout(&output);
    assert_eq!(
        (result["verdict"].clone(), result["overridden_by"].clone()),
        (json!("pass"), json!("another-maintainer"))
    );
}

#[cfg(unix)]
#[test]
fn it_should_print_a_failing_verdict_with_the_reason_when_the_label_was_applied_before_the_push() {
    // Arrange
    let gh = FakeGh::new("stale");
    let classification = gh.write_classification(&classify_output(b"packages/persistence-benchmark/note.md\0"));
    respond_with_check_runs(
        &gh,
        &[
            check_run(1, "docs-lint", "success"),
            check_run(2, "db-benchmarking", "failure"),
        ],
    );
    respond_with_an_override(&gh, "josecelano", "2026-10-21T10:44:46Z", "2026-10-21T10:46:06Z");

    // Act
    let output = run_github_verdict(&gh, &classification);

    // Assert
    assert_succeeded_without_diagnostics(&output);
    let result = parse_verdict_stdout(&output);
    assert_eq!(result["verdict"], "fail");
    assert!(
        result["override_ignored"]
            .as_str()
            .is_some_and(|reason| reason.contains("a push revoked it")),
        "the reason should say that a push revoked the override, but was: {result}"
    );
}

#[cfg(unix)]
#[test]
fn it_should_write_a_json_runtime_error_and_nothing_on_stdout_when_the_check_runs_cannot_be_read() {
    // Arrange
    let gh = FakeGh::new("unreadable");
    let classification = gh.write_classification(&classify_output(b"docs/index.md\0"));

    // Act
    let output = run_github_verdict(&gh, &classification);

    // Assert
    assert_eq!(
        output.status.code(),
        Some(1),
        "a runtime failure exits with code 1, so the gate fails closed"
    );
    assert!(
        output.stdout.is_empty(),
        "a failed run writes nothing to stdout, but stdout was: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let record: Value = serde_json::from_slice(&output.stderr).expect("the diagnostic is one JSON object");
    assert_eq!(record["kind"], "runtime_error");
    assert!(
        record["message"]
            .as_str()
            .is_some_and(|message| message.contains("cannot read the check runs")
                && message.contains("api --paginate repos/owner/repo/commits/")),
        "the message should say what could not be read, but was: {record}"
    );
}

#[cfg(unix)]
#[test]
fn it_should_write_a_json_runtime_error_when_the_classification_file_does_not_exist() {
    // Arrange
    let gh = FakeGh::new("no-classification");

    // Act
    let output = run_github_verdict(&gh, "/nonexistent/classification.json");

    // Assert
    assert_eq!(output.status.code(), Some(1));
    let record: Value = serde_json::from_slice(&output.stderr).expect("the diagnostic is one JSON object");
    assert!(
        record["message"]
            .as_str()
            .is_some_and(|message| message.contains("/nonexistent/classification.json")),
        "the message should name the file, but was: {record}"
    );
}

#[test]
fn it_should_write_a_json_usage_error_when_the_repository_is_not_owner_and_name() {
    // Arrange
    let arguments = [
        "github-verdict",
        "--repo",
        "repo",
        "--pull-request",
        "7",
        "--head-sha",
        "a",
        "--classification",
        "c.json",
    ];

    // Act
    let output = run_ci_gate(&arguments, b"");

    // Assert
    assert_eq!(output.status.code(), Some(2), "a usage error exits with code 2");
    assert!(
        output.stdout.is_empty(),
        "a failed run writes nothing to stdout, but stdout was: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let record: Value = serde_json::from_slice(&output.stderr).expect("the diagnostic is one JSON object");
    assert_eq!(record["kind"], "usage_error");
}

#[cfg(unix)]
#[test]
fn it_should_read_every_page_when_gh_prints_one_json_document_per_page() {
    // Arrange
    // `gh api --paginate` prints the pages one after the other, with nothing between them. The second group's
    // check run is on the second page, so reading only the first page would leave it missing.
    let gh = FakeGh::new("pages");
    let classification = gh.write_classification(&classify_output(b"packages/persistence-benchmark/note.md\0"));
    let first_page = json!({ "check_runs": [check_run(1, "docs-lint", "success")] });
    let second_page = json!({ "check_runs": [check_run(2, "db-benchmarking", "success")] });
    gh.respond_with_text(
        &format!("repos/owner/repo/commits/{SHA}/check-runs?per_page=100"),
        &format!("{first_page}{second_page}"),
    );

    // Act
    let output = run_github_verdict(&gh, &classification);

    // Assert
    assert_succeeded_without_diagnostics(&output);
    assert_eq!(parse_verdict_stdout(&output)["verdict"], "pass");
}
