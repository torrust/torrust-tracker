//! Pins the command-line contract against the outputs recorded from the retired shell scripts.

use std::process::{Command, Output};

use serde_json::Value;

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/review-threads.json");
const LIST_CAPTURE: &str = include_str!("fixtures/retired-scripts/list-unresolved.jsonl");
const SHOW_CAPTURE: &str = include_str!("fixtures/retired-scripts/show-unresolved.txt");
const REPLY_STATUS_CAPTURE: &str = include_str!("fixtures/retired-scripts/reply-status.stdout");
const REPLY_STATUS_EXIT_STATUS: &str = include_str!("fixtures/retired-scripts/reply-status.exit-status");

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

#[test]
fn it_should_list_the_threads_the_retired_script_printed() {
    // Arrange: the retired list script printed one JSON line per unresolved thread.
    let expected_threads = captured_values(LIST_CAPTURE);

    // Act: list the unresolved threads through the binary.
    let output = run(&["list", "--threads-file", FIXTURE]);

    // Assert: one JSON object carries the same rows in the same order, and stderr stays silent.
    assert!(output.status.success());
    assert_eq!(json(&output.stdout)["threads"], Value::Array(expected_threads));
    assert_eq!(text(&output.stderr), "");
}

#[test]
fn it_should_show_the_threads_and_comments_the_retired_script_printed() {
    // Arrange: the retired show script printed a thread header and one URL line per comment.
    let expected_thread_ids = captured_lines_with_prefix(SHOW_CAPTURE, "=== Thread ")
        .into_iter()
        .map(|header| header.trim_end_matches(" ==="))
        .collect::<Vec<_>>();
    let expected_comment_urls = captured_lines_with_prefix(SHOW_CAPTURE, "URL:      ");

    // Act: show the unresolved threads through the binary.
    let output = run(&["show", "--threads-file", FIXTURE]);

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
fn it_should_name_the_threads_without_a_reply_and_keep_the_retired_exit_status() {
    // Arrange: the retired reply-status script printed every row, a summary line, and exited 1.
    let mut expected_rows = captured_values(REPLY_STATUS_CAPTURE);
    let expected_summary = expected_rows.pop().unwrap();
    let expected_exit_status = REPLY_STATUS_EXIT_STATUS.trim().parse::<i32>().unwrap();

    // Act: check the fixture for replies from the login that answered only one thread.
    let output = run(&["reply-status", "--threads-file", FIXTURE, "--login", "author"]);

    // Assert: stdout is empty, and the stderr diagnostic carries every row and the counts.
    assert_eq!(output.status.code(), Some(expected_exit_status));
    assert_eq!(text(&output.stdout), "");
    let diagnostic = json(&output.stderr);
    assert_eq!(diagnostic["kind"], "missing_reply");
    assert_eq!(diagnostic["threads"], Value::Array(expected_rows));
    for count in ["total", "with_reply", "without_reply"] {
        assert_eq!(diagnostic["summary"][count], expected_summary[count]);
    }
}

#[test]
fn it_should_emit_the_reply_status_result_when_every_thread_has_a_reply() {
    // Arrange: the fixture's unresolved threads each carry a comment from the reviewer.
    let login = "reviewer";

    // Act: check the fixture for replies from that login.
    let output = run(&["reply-status", "--threads-file", FIXTURE, "--login", login]);

    // Assert: the result is one JSON object on stdout with no diagnostics.
    assert!(output.status.success());
    assert_eq!(json(&output.stdout)["summary"]["without_reply"], 0);
    assert_eq!(text(&output.stderr), "");
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
