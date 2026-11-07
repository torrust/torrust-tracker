//! Pins the command-line contract against the outputs recorded from the retired shell scripts.

use std::process::{Command, Output};

use serde_json::Value;

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/review-threads.json");
const LIST_CAPTURE: &str = include_str!("fixtures/retired-scripts/list-unresolved.jsonl");
const SHOW_CAPTURE: &str = include_str!("fixtures/retired-scripts/show-unresolved.txt");
const REPLY_STATUS_CAPTURE: &str = include_str!("fixtures/retired-scripts/reply-status.stdout");

fn run(arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_github-review-threads"))
        .args(arguments)
        .output()
        .expect("the binary should run")
}

fn json(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).expect("the stream should hold exactly one JSON value")
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn captured_values(capture: &str) -> Vec<Value> {
    serde_json::Deserializer::from_str(capture)
        .into_iter::<Value>()
        .collect::<Result<_, _>>()
        .expect("the capture should be a JSON stream")
}

fn captured_lines_with_prefix<'a>(capture: &'a str, prefix: &str) -> Vec<&'a str> {
    capture.lines().filter_map(|line| line.strip_prefix(prefix)).collect()
}

/// Keeps only the fields the retired script printed, so added evidence fields do not break parity.
fn with_fields_of(thread: &Value, captured_row: &Value) -> Value {
    captured_row
        .as_object()
        .unwrap()
        .keys()
        .map(|key| (key.clone(), thread[key].clone()))
        .collect::<serde_json::Map<_, _>>()
        .into()
}

fn thread_ids(stdout: &[u8]) -> Vec<String> {
    json(stdout)["threads"]
        .as_array()
        .unwrap()
        .iter()
        .map(|thread| thread["id"].as_str().unwrap().to_owned())
        .collect()
}

#[test]
fn it_should_list_the_threads_the_retired_script_printed_when_unresolved_only_is_selected() {
    // Arrange: the retired list script printed one JSON line per unresolved thread.
    let expected_threads = captured_values(LIST_CAPTURE);

    // Act: list the unresolved threads through the binary.
    let output = run(&["list", "--threads-file", FIXTURE, "--unresolved-only"]);

    // Assert: one JSON object carries the same rows in the same order, and stderr stays silent.
    assert!(output.status.success());
    let threads = json(&output.stdout)["threads"].as_array().cloned().unwrap();
    assert_eq!(threads.len(), expected_threads.len());
    for (thread, expected) in threads.iter().zip(&expected_threads) {
        assert_eq!(&with_fields_of(thread, expected), expected);
    }
    assert_eq!(text(&output.stderr), "");
}

#[test]
fn it_should_list_resolved_threads_by_default() {
    // Arrange: the fixture's first thread is resolved, so the retired script never printed it.
    let resolved_thread_id = "THREAD_RESOLVED_OUTDATED";

    // Act: list the threads without selecting unresolved-only.
    let output = run(&["list", "--threads-file", FIXTURE]);

    // Assert: the resolved thread is part of the default evidence view.
    assert!(output.status.success());
    assert!(thread_ids(&output.stdout).iter().any(|id| id == resolved_thread_id));
}

#[test]
fn it_should_show_resolved_threads_by_default() {
    // Arrange: the fixture's first thread is resolved, so the retired script never printed it.
    let resolved_thread_id = "THREAD_RESOLVED_OUTDATED";

    // Act: show the threads without selecting unresolved-only.
    let output = run(&["show", "--threads-file", FIXTURE]);

    // Assert: the resolved thread is part of the default evidence view.
    assert!(output.status.success());
    assert!(thread_ids(&output.stdout).iter().any(|id| id == resolved_thread_id));
}

#[test]
fn it_should_show_the_threads_and_comments_the_retired_script_printed_when_unresolved_only_is_selected() {
    // Arrange: the retired show script printed a thread header and one URL line per comment.
    let expected_thread_ids = captured_lines_with_prefix(SHOW_CAPTURE, "=== Thread ")
        .into_iter()
        .map(|header| header.trim_end_matches(" ==="))
        .collect::<Vec<_>>();
    let expected_comment_urls = captured_lines_with_prefix(SHOW_CAPTURE, "URL:      ");

    // Act: show the unresolved threads through the binary.
    let output = run(&["show", "--threads-file", FIXTURE, "--unresolved-only"]);

    // Assert: the JSON object holds the same threads and the same comments in order.
    assert!(output.status.success());
    let threads = json(&output.stdout)["threads"].as_array().cloned().unwrap();
    let thread_ids = threads
        .iter()
        .map(|thread| thread["id"].as_str().unwrap())
        .collect::<Vec<_>>();
    let comment_urls = threads
        .iter()
        .flat_map(|thread| thread["comments"].as_array().unwrap().clone())
        .map(|comment| comment["url"].as_str().unwrap().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(thread_ids, expected_thread_ids);
    assert_eq!(comment_urls, expected_comment_urls);
}

#[test]
fn it_should_report_the_threads_without_a_reply_as_a_result_with_exit_zero() {
    // Arrange: the retired reply-status script printed every row and a summary line.
    let mut expected_rows = captured_values(REPLY_STATUS_CAPTURE);
    let expected_summary = expected_rows.pop().unwrap();

    // Act: check the fixture for replies from the login that answered only one thread.
    let output = run(&["reply-status", "--threads-file", FIXTURE, "--login", "author"]);

    // Assert: a missing reply is an answer, so exit 0, the result on stdout, and no diagnostic.
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(text(&output.stderr), "");
    let result = json(&output.stdout);
    assert_eq!(result["kind"], "missing_reply");
    assert_eq!(result["threads"], Value::Array(expected_rows));
    for count in ["total", "with_reply", "without_reply"] {
        assert_eq!(result["summary"][count], expected_summary[count]);
    }
}

#[test]
fn it_should_report_all_replied_as_a_result_with_exit_zero_when_every_thread_has_a_reply() {
    // Arrange: the fixture's unresolved threads each carry a comment from the reviewer.
    let login = "reviewer";

    // Act: check the fixture for replies from that login.
    let output = run(&["reply-status", "--threads-file", FIXTURE, "--login", login]);

    // Assert: the result is one JSON object on stdout with no diagnostics.
    assert_eq!(output.status.code(), Some(0));
    let result = json(&output.stdout);
    assert_eq!(result["kind"], "all_replied");
    assert_eq!(result["summary"]["without_reply"], 0);
    assert_eq!(text(&output.stderr), "");
}

#[test]
fn it_should_fail_with_exit_one_and_a_diagnostic_when_the_threads_file_cannot_be_read() {
    // Arrange: a threads file that does not exist.
    let missing = "/nonexistent/threads.json";

    // Act: ask for the reply status.
    let output = run(&["reply-status", "--threads-file", missing, "--login", "author"]);

    // Assert: could not answer, so exit 1, empty stdout, and a JSON diagnostic.
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(text(&output.stdout), "");
    assert_eq!(json(&output.stderr)["kind"], "reply_status_error");
}

#[test]
fn it_should_fail_with_exit_one_and_a_diagnostic_when_the_threads_file_is_not_valid_json() {
    // Arrange: a threads file that exists and is readable but holds malformed JSON.
    let malformed = format!("{}/malformed-threads.json", env!("CARGO_TARGET_TMPDIR"));
    std::fs::write(&malformed, "{ not json").unwrap();

    // Act: ask for the reply status.
    let output = run(&["reply-status", "--threads-file", &malformed, "--login", "author"]);

    // Assert: could not answer, so exit 1, empty stdout, and a JSON diagnostic.
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(text(&output.stdout), "");
    assert_eq!(json(&output.stderr)["kind"], "reply_status_error");
}

#[test]
fn it_should_report_a_missing_login_argument_as_a_usage_error() {
    // Arrange: a valid threads file, so only the required login is absent.
    let arguments = ["reply-status", "--threads-file", FIXTURE];

    // Act: run it with stdout redirected.
    let output = run(&arguments);

    // Assert: exit 2, empty stdout, and a usage diagnostic that names the missing login.
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(text(&output.stdout), "");
    let diagnostic = json(&output.stderr);
    assert_eq!(diagnostic["kind"], "usage_error");
    assert!(
        diagnostic["message"]
            .as_str()
            .unwrap()
            .contains("required arguments were not provided:\n  --login <LOGIN>\n"),
    );
}

#[test]
fn it_should_report_help_as_a_json_usage_diagnostic() {
    // Arrange: the output contract allows no plain text, so help is a usage diagnostic.
    let arguments = ["--help"];

    // Act: request help with stdout redirected.
    let output = run(&arguments);

    // Assert: exit code 2, empty stdout, and a JSON record on stderr.
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(text(&output.stdout), "");
    assert_eq!(json(&output.stderr)["kind"], "usage_error");
}
