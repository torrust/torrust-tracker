//! Runs the real binary against a fake `gh` script, to pin the command-line contract:
//! one JSON result on stdout, NDJSON diagnostics on stderr, and exit codes 0, 1, and 2.

use std::fs;
use std::io::{BufRead, BufReader, Read};
use std::os::unix::fs::PermissionsExt;
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant};

use serde_json::Value;
use tempfile::TempDir;

const FAKE_GH: &str = r#"#!/bin/sh
dir="$(dirname "$0")"
case "$*" in
  "auth status")
    if [ -f "$dir/auth-fails" ]; then echo "You are not logged in" >&2; exit 1; fi
    if [ -f "$dir/auth-hangs" ]; then sleep 30; fi
    exit 0 ;;
  "api repos/torrust/torrust-tracker/pulls/7")
    if [ -f "$dir/api-fails" ]; then echo "HTTP 502" >&2; exit 1; fi
    n=$(cat "$dir/pull-calls" 2>/dev/null || echo 0)
    echo $((n + 1)) > "$dir/pull-calls"
    if [ -f "$dir/pull-hangs-later" ] && [ "$n" -ge 1 ]; then sleep 30; fi
    cat "$dir/pull.json" ;;
  "api --paginate --slurp repos/torrust/torrust-tracker/pulls/7/reviews?per_page=100")
    n=$(cat "$dir/review-calls" 2>/dev/null || echo 0)
    echo $((n + 1)) > "$dir/review-calls"
    if [ "$n" -eq 0 ]; then echo '[[]]'; else cat "$dir/reviews-later.json"; fi ;;
  "api --paginate --slurp repos/torrust/torrust-tracker/issues/7/comments?per_page=100") echo '[[]]' ;;
  "api --paginate --slurp repos/torrust/torrust-tracker/pulls/7/comments?per_page=100") echo '[[]]' ;;
  *) echo "unexpected gh call: $*" >&2; exit 1 ;;
esac
"#;

const OPEN_PULL: &str = r#"{"head": {"sha": "abc1234"}, "state": "open", "merged": false}"#;
const MERGED_PULL: &str = r#"{"head": {"sha": "abc1234"}, "state": "closed", "merged": true}"#;
const REVIEW_BY_DA2CE7: &str = r#"[[{"id": 5, "user": {"login": "da2ce7"}, "state": "APPROVED", "commit_id": "abc1234", "html_url": "https://example.test/review/5", "submitted_at": "2026-10-28T07:00:00Z"}]]"#;

struct Sandbox {
    directory: TempDir,
}

impl Sandbox {
    fn with_pull(pull: &str) -> Self {
        let directory = TempDir::new().unwrap();
        let gh = directory.path().join("gh");
        fs::write(&gh, FAKE_GH).unwrap();
        fs::set_permissions(&gh, fs::Permissions::from_mode(0o755)).unwrap();
        fs::write(directory.path().join("pull.json"), pull).unwrap();
        fs::write(directory.path().join("reviews-later.json"), REVIEW_BY_DA2CE7).unwrap();

        Self { directory }
    }

    fn flag(&self, name: &str) {
        fs::write(self.directory.path().join(name), "").unwrap();
    }

    fn command(&self, arguments: &[&str]) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_wait-for-pr-review"));
        command
            .args(arguments)
            .env("PATH", format!("{}:/usr/bin:/bin", self.directory.path().display()))
            .stdin(Stdio::null());
        command
    }

    fn run(&self, arguments: &[&str]) -> Output {
        self.command(arguments).output().unwrap()
    }
}

fn json(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).expect("the stream should hold exactly one JSON value")
}

fn stderr_records(output: &Output) -> Vec<Value> {
    String::from_utf8_lossy(&output.stderr)
        .lines()
        .map(|line| serde_json::from_str(line).expect("every stderr line should be a JSON record"))
        .collect()
}

fn kinds(output: &Output) -> Vec<String> {
    stderr_records(output)
        .iter()
        .map(|record| record["kind"].as_str().unwrap().to_owned())
        .collect()
}

#[test]
fn it_should_print_the_review_that_arrives_after_the_wait_began_as_one_json_object_and_exit_zero() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);

    let output = sandbox.run(&["--pr-number", "7", "--poll-interval-seconds", "1", "--timeout-seconds", "30"]);

    assert_eq!(output.status.code(), Some(0));
    let event = json(&output.stdout);
    assert_eq!(event["kind"], "review");
    assert_eq!(event["reviewer"], "da2ce7");
    assert_eq!(event["state"], "APPROVED");
    assert_eq!(event["head_sha"], "abc1234");
    assert_eq!(kinds(&output), ["wait_started"]);
}

#[test]
fn it_should_report_a_timeout_as_a_result_with_exit_zero_when_nothing_happens() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);
    // The review that appears comes from da2ce7, who is not in the watched set here, so it is ignored.
    let output = sandbox.run(&[
        "--pr-number",
        "7",
        "--reviewer",
        "someone-else",
        "--poll-interval-seconds",
        "1",
        "--timeout-seconds",
        "2",
    ]);

    assert_eq!(output.status.code(), Some(0));
    let event = json(&output.stdout);
    assert_eq!(event["kind"], "timeout");
    assert_eq!(event["waited_seconds"], 2);
}

#[test]
fn it_should_report_an_already_merged_pull_request_immediately() {
    let sandbox = Sandbox::with_pull(MERGED_PULL);

    let output = sandbox.run(&["--pr-number", "7"]);

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(json(&output.stdout)["kind"], "pr_merged");
}

#[test]
fn it_should_leave_stdout_empty_and_exit_one_when_gh_is_not_authenticated() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);
    sandbox.flag("auth-fails");

    let output = sandbox.run(&["--pr-number", "7"]);

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(output.stdout, b"");
    assert_eq!(kinds(&output), ["wait_started", "gh_not_authenticated"]);
}

#[test]
fn it_should_leave_stdout_empty_and_exit_one_when_an_api_request_fails() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);
    sandbox.flag("api-fails");

    let output = sandbox.run(&["--pr-number", "7"]);

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(output.stdout, b"");
    assert_eq!(kinds(&output).last().map(String::as_str), Some("gh_api_error"));
}

#[test]
fn it_should_leave_stdout_empty_and_exit_one_when_gh_is_not_installed() {
    let empty = TempDir::new().unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_wait-for-pr-review"))
        .args(["--pr-number", "7"])
        .env("PATH", empty.path())
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(output.stdout, b"");
    assert_eq!(kinds(&output).last().map(String::as_str), Some("gh_unavailable"));
}

#[test]
fn it_should_exit_two_with_a_usage_error_record_when_the_pull_request_number_is_missing() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);

    let output = sandbox.run(&[]);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout, b"");
    assert_eq!(kinds(&output), ["usage_error"]);
}

#[test]
fn it_should_exit_two_when_since_head_sha_is_not_a_commit_sha() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);

    let output = sandbox.run(&["--pr-number", "7", "--since-head-sha", "../../etc/passwd"]);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(kinds(&output), ["usage_error"]);
}

#[test]
fn it_should_exit_two_when_the_poll_interval_is_zero() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);

    let output = sandbox.run(&["--pr-number", "7", "--poll-interval-seconds", "0"]);

    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn it_should_print_help_as_a_json_record_on_stderr_and_exit_zero() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);

    let output = sandbox.run(&["--help"]);

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout, b"");
    assert_eq!(kinds(&output), ["help"]);
}

#[test]
fn it_should_kill_a_hung_gh_call_at_the_deadline_without_a_signal_and_exit_one_before_the_baseline() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);
    sandbox.flag("auth-hangs");
    let started = Instant::now();

    let output = sandbox.run(&["--pr-number", "7", "--timeout-seconds", "2"]);

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(output.stdout, b"");
    assert_eq!(kinds(&output).last().map(String::as_str), Some("deadline_exceeded"));
    assert!(
        started.elapsed() < Duration::from_secs(10),
        "the hung call should be killed at the deadline"
    );
}

#[test]
fn it_should_report_a_timeout_when_a_gh_call_hangs_past_the_deadline_after_the_baseline() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);
    sandbox.flag("pull-hangs-later");
    let started = Instant::now();

    let output = sandbox.run(&[
        "--pr-number",
        "7",
        "--reviewer",
        "someone-else",
        "--poll-interval-seconds",
        "1",
        "--timeout-seconds",
        "3",
    ]);

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(json(&output.stdout)["kind"], "timeout");
    assert!(
        started.elapsed() < Duration::from_secs(10),
        "the hung call should be killed at the deadline"
    );
}

fn send(signal: &str, child: &Child) {
    let status = Command::new("kill").args([signal, &child.id().to_string()]).status().unwrap();
    assert!(status.success());
}

/// Starts the poller, waits until it has installed its signal handlers, and returns the child.
fn start_and_wait_until_running(sandbox: &Sandbox, arguments: &[&str]) -> (Child, BufReader<std::process::ChildStderr>) {
    let mut child = sandbox
        .command(arguments)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stderr = BufReader::new(child.stderr.take().unwrap());
    let mut first = String::new();
    stderr.read_line(&mut first).unwrap();
    assert_eq!(json(first.as_bytes())["kind"], "wait_started");

    (child, stderr)
}

fn finish(mut child: Child, mut stderr: BufReader<std::process::ChildStderr>) -> (Option<i32>, String, String) {
    let mut stdout = String::new();
    child.stdout.take().unwrap().read_to_string(&mut stdout).unwrap();
    let mut rest = String::new();
    stderr.read_to_string(&mut rest).unwrap();
    let status = child.wait().unwrap();

    (status.code(), stdout, rest)
}

#[test]
fn it_should_stop_on_sigterm_with_an_interrupted_record_and_exit_one() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);
    let (child, stderr) = start_and_wait_until_running(&sandbox, &["--pr-number", "7", "--poll-interval-seconds", "60"]);
    let started = Instant::now();

    send("-TERM", &child);
    let (code, stdout, rest) = finish(child, stderr);

    assert_eq!(code, Some(1));
    assert_eq!(stdout, "");
    assert_eq!(json(rest.trim().as_bytes())["kind"], "interrupted");
    assert!(started.elapsed() < Duration::from_secs(10), "the poller should stop promptly");
}

#[test]
fn it_should_stop_on_sigint_with_an_interrupted_record_and_exit_one() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);
    let (child, stderr) = start_and_wait_until_running(&sandbox, &["--pr-number", "7", "--poll-interval-seconds", "60"]);

    send("-INT", &child);
    let (code, stdout, rest) = finish(child, stderr);

    assert_eq!(code, Some(1));
    assert_eq!(stdout, "");
    assert_eq!(json(rest.trim().as_bytes())["kind"], "interrupted");
}

#[test]
fn it_should_not_wait_for_a_hung_gh_call_when_a_signal_arrives() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);
    sandbox.flag("auth-hangs");
    let (child, stderr) = start_and_wait_until_running(&sandbox, &["--pr-number", "7"]);
    let started = Instant::now();

    send("-TERM", &child);
    let (code, _, rest) = finish(child, stderr);

    assert_eq!(code, Some(1));
    assert_eq!(json(rest.trim().as_bytes())["kind"], "interrupted");
    assert!(
        started.elapsed() < Duration::from_secs(10),
        "the hung gh call should be killed, not awaited"
    );
}
