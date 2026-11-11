//! Runs the real binary against a fake `gh` script, to pin the command-line contract:
//! one JSON result on stdout, NDJSON diagnostics on stderr, and exit codes 0, 1, and 2.

// The fake `gh` is a `/bin/sh` script made executable with Unix permissions, and the signal scenarios
// send `kill`, so these scenarios have no Windows equivalent. The crate still builds on every target.
#![cfg(unix)]

use std::fs;
use std::io::{BufRead, BufReader, Read};
use std::os::unix::fs::PermissionsExt;
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant};

use serde_json::Value;
use tempfile::TempDir;

const FAKE_GH: &str = r#"#!/bin/sh
dir="$(dirname "$0")"
echo "$*" >> "$dir/calls"
case "$*" in
  "api user")
    if [ -f "$dir/user-hangs" ]; then sleep 30; fi
    if [ -f "$dir/user.json" ]; then cat "$dir/user.json"; else echo "no user" >&2; exit 1; fi ;;
  "auth status")
    if [ -f "$dir/auth-fails" ]; then echo "You are not logged in" >&2; exit 1; fi
    if [ -f "$dir/auth-hangs" ]; then sleep 30; fi
    if [ -f "$dir/auth-leaves-pipe-open" ]; then (trap '' HUP; sleep 20 &); fi
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
  "api repos/torrust/torrust-tracker/commits/1111111")
    echo '{"sha": "1111111000000000000000000000000000000000", "commit": {"committer": {"date": "2026-10-28T06:00:00Z"}}}' ;;
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

    /// The path of the fake `gh`, for `--gh-command`.
    fn fake_path(&self) -> String {
        self.directory.path().join("gh").display().to_string()
    }

    /// A poller command whose `PATH` does not contain the fake `gh`, so `--gh-command` is the only route to it.
    fn command_without_fake_on_path(arguments: &[&str]) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_wait-for-pr-review"));
        command.args(arguments).env("PATH", "/usr/bin:/bin").stdin(Stdio::null());
        command
    }

    /// The calls the fake `gh` served, one per line.
    fn calls(&self) -> Vec<String> {
        fs::read_to_string(self.directory.path().join("calls"))
            .unwrap_or_default()
            .lines()
            .map(str::to_owned)
            .collect()
    }

    /// How many times the fake `gh` served the pull request endpoint (not `gh auth status`).
    fn pull_calls(&self) -> u32 {
        fs::read_to_string(self.directory.path().join("pull-calls"))
            .map_or(0, |count| count.trim().parse().expect("`pull-calls` should hold a number"))
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
    assert_eq!(event["cursor"], "2026-10-28T07:00:00Z");
    assert_eq!(kinds(&output), ["wait_started", "gh_identity"]);
}

#[test]
fn it_should_not_report_a_review_again_when_resumed_from_its_cursor() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);
    let first = sandbox.run(&["--pr-number", "7", "--poll-interval-seconds", "1", "--timeout-seconds", "30"]);
    let cursor = json(&first.stdout)["cursor"].as_str().unwrap().to_owned();

    let resumed = sandbox.run(&[
        "--pr-number",
        "7",
        "--since",
        &cursor,
        "--poll-interval-seconds",
        "1",
        "--timeout-seconds",
        "2",
    ]);

    assert_eq!(resumed.status.code(), Some(0));
    assert_eq!(
        json(&resumed.stdout)["kind"],
        "timeout",
        "the review at the cursor must not fire again"
    );
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
    assert_eq!(kinds(&output), ["wait_started", "gh_identity", "gh_not_authenticated"]);
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
fn it_should_exit_two_when_since_is_not_a_utc_timestamp() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);

    let output = sandbox.run(&["--pr-number", "7", "--since", "yesterday"]);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(kinds(&output), ["usage_error"]);
}

#[test]
fn it_should_exit_two_when_since_names_a_day_or_a_signed_field_that_does_not_exist() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);

    for since in ["2026-02-29T07:00:00Z", "2026-10-28T-1:00:00Z"] {
        let output = sandbox.run(&["--pr-number", "7", "--since", since]);

        assert_eq!(output.status.code(), Some(2), "--since {since} should be a usage error");
        assert_eq!(output.stdout, b"");
        assert_eq!(kinds(&output), ["usage_error"]);
    }
}

#[test]
fn it_should_report_a_head_change_at_once_when_the_checked_out_commit_is_not_the_head_even_with_a_cursor() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);

    // The pull request head is abc1234, but the agent says its checkout is 1111111: its tree is stale.
    let output = sandbox.run(&[
        "--pr-number",
        "7",
        "--since-head-sha",
        "1111111",
        "--since",
        "2026-10-28T08:00:00Z",
        "--reviewer",
        "someone-else",
        "--timeout-seconds",
        "30",
    ]);

    assert_eq!(output.status.code(), Some(0));
    let event = json(&output.stdout);
    assert_eq!(event["kind"], "head_changed");
    assert_eq!(event["previous_head_sha"], "1111111000000000000000000000000000000000");
    assert_eq!(event["head_sha"], "abc1234");
}

#[test]
fn it_should_exit_two_when_the_poll_interval_is_zero() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);

    let output = sandbox.run(&["--pr-number", "7", "--poll-interval-seconds", "0"]);

    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn it_should_exit_two_with_a_usage_error_record_when_the_timeout_is_too_large_for_any_deadline() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);

    let output = sandbox.run(&["--pr-number", "7", "--timeout-seconds", "18446744073709551615"]);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout, b"");
    assert_eq!(kinds(&output), ["usage_error"]);
    assert_eq!(
        sandbox.pull_calls(),
        0,
        "an oversized timeout must be rejected before any API request"
    );
}

#[test]
fn it_should_exit_two_when_the_timeout_is_one_second_above_the_maximum() {
    let sandbox = Sandbox::with_pull(MERGED_PULL);

    let output = sandbox.run(&["--pr-number", "7", "--timeout-seconds", "86401"]);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout, b"");
    assert_eq!(kinds(&output), ["usage_error"]);
}

#[test]
fn it_should_run_to_a_result_when_the_timeout_is_the_maximum() {
    let sandbox = Sandbox::with_pull(MERGED_PULL);

    let output = sandbox.run(&["--pr-number", "7", "--timeout-seconds", "86400"]);

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(json(&output.stdout)["kind"], "pr_merged");
}

#[test]
fn it_should_print_help_as_a_json_record_on_stderr_and_exit_zero() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);

    let output = sandbox.run(&["--help"]);

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout, b"");
    assert_eq!(kinds(&output), ["help"]);
    let message = stderr_records(&output)[0]["message"].as_str().unwrap().to_owned();
    assert!(
        message.starts_with("Block until a watched reviewer"),
        "help must keep the program description"
    );
    assert!(
        message.contains("At most 86400 (one day)"),
        "help must state the maximum timeout"
    );
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

/// A command whose stderr is `/dev/full`, so that every write to it fails with "no space left".
#[cfg(target_os = "linux")]
fn with_full_stderr(sandbox: &Sandbox, arguments: &[&str]) -> Output {
    let full = fs::OpenOptions::new().write(true).open("/dev/full").unwrap();

    sandbox.command(arguments).stderr(Stdio::from(full)).output().unwrap()
}

#[cfg(target_os = "linux")]
#[test]
fn it_should_not_report_success_for_help_when_the_help_text_cannot_be_written() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);

    let output = with_full_stderr(&sandbox, &["--help"]);

    assert_eq!(output.status.code(), Some(1), "help that was never written must not exit 0");
    assert_eq!(output.stdout, b"");
}

#[cfg(target_os = "linux")]
#[test]
fn it_should_keep_the_usage_error_exit_code_when_the_diagnostic_cannot_be_written() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);

    let output = with_full_stderr(&sandbox, &[]);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout, b"");
}

#[cfg(target_os = "linux")]
#[test]
fn it_should_still_print_the_result_when_only_the_progress_record_cannot_be_written() {
    let sandbox = Sandbox::with_pull(MERGED_PULL);

    let output = with_full_stderr(&sandbox, &["--pr-number", "7"]);

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(json(&output.stdout)["kind"], "pr_merged");
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
    // The identity record follows at once: the fake answers `api user` immediately.
    let mut identity = String::new();
    stderr.read_line(&mut identity).unwrap();
    assert_eq!(json(identity.as_bytes())["kind"], "gh_identity");

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

/// How long the fake `gh` of the pipe scenarios lets its detached descendant live. The tests must end
/// well before it, so a poller that waits for the descendant fails them.
const DESCENDANT_LIFETIME: Duration = Duration::from_secs(20);

#[test]
fn it_should_end_at_the_deadline_when_gh_exits_but_a_descendant_keeps_its_pipes_open() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);
    sandbox.flag("auth-leaves-pipe-open");
    let started = Instant::now();

    let output = sandbox.run(&["--pr-number", "7", "--timeout-seconds", "2"]);

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(output.stdout, b"");
    assert_eq!(kinds(&output).last().map(String::as_str), Some("deadline_exceeded"));
    assert!(
        started.elapsed() < DESCENDANT_LIFETIME / 2,
        "the wait for the readers must be bounded by the deadline, not by the descendant"
    );
}

#[test]
fn it_should_stop_on_sigterm_when_gh_exits_but_a_descendant_keeps_its_pipes_open() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);
    sandbox.flag("auth-leaves-pipe-open");
    let (child, stderr) = start_and_wait_until_running(&sandbox, &["--pr-number", "7", "--timeout-seconds", "900"]);
    // Let the poller reach the point where `gh` has exited and only the descendant is left.
    std::thread::sleep(Duration::from_millis(500));
    let started = Instant::now();

    send("-TERM", &child);
    let (code, stdout, rest) = finish(child, stderr);

    assert_eq!(code, Some(1));
    assert_eq!(stdout, "");
    assert_eq!(json(rest.trim().as_bytes())["kind"], "interrupted");
    assert!(
        started.elapsed() < DESCENDANT_LIFETIME / 2,
        "the signal must end the wait for the readers promptly"
    );
}

#[test]
fn it_should_run_the_command_named_by_gh_command_for_every_call_when_it_is_not_on_the_path() {
    let sandbox = Sandbox::with_pull(MERGED_PULL);

    let output = Sandbox::command_without_fake_on_path(&["--pr-number", "7", "--gh-command", &sandbox.fake_path()])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(json(&output.stdout)["kind"], "pr_merged");
    assert!(
        sandbox.pull_calls() >= 1,
        "the fake command should have served the pull request"
    );
}

#[test]
fn it_should_not_run_auth_status_for_a_command_named_by_gh_command() {
    let sandbox = Sandbox::with_pull(MERGED_PULL);
    sandbox.flag("auth-fails");

    let output = Sandbox::command_without_fake_on_path(&["--pr-number", "7", "--gh-command", &sandbox.fake_path()])
        .output()
        .unwrap();

    assert_eq!(
        output.status.code(),
        Some(0),
        "a custom command is not checked with `auth status`"
    );
    assert!(!sandbox.calls().iter().any(|call| call == "auth status"));
}

#[test]
fn it_should_run_auth_status_for_the_default_gh() {
    let sandbox = Sandbox::with_pull(MERGED_PULL);

    let output = sandbox.run(&["--pr-number", "7"]);

    assert_eq!(output.status.code(), Some(0));
    assert!(sandbox.calls().iter().any(|call| call == "auth status"));
}

#[test]
fn it_should_report_the_command_in_wait_started_and_the_login_in_gh_identity() {
    let sandbox = Sandbox::with_pull(MERGED_PULL);
    fs::write(sandbox.directory.path().join("user.json"), r#"{"login": "some-account"}"#).unwrap();
    let fake = sandbox.fake_path();

    let output = Sandbox::command_without_fake_on_path(&["--pr-number", "7", "--gh-command", &fake])
        .output()
        .unwrap();

    let records = stderr_records(&output);
    assert_eq!(records[0]["kind"], "wait_started");
    assert_eq!(records[0]["gh_command"], fake.as_str());
    assert_eq!(records[1]["kind"], "gh_identity");
    assert_eq!(records[1]["gh_command"], fake.as_str());
    assert_eq!(records[1]["gh_login"], "some-account");
}

#[test]
fn it_should_report_a_null_login_when_the_lookup_fails_and_still_print_the_result() {
    let sandbox = Sandbox::with_pull(MERGED_PULL);

    let output = sandbox.run(&["--pr-number", "7"]);

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(json(&output.stdout)["kind"], "pr_merged");
    let records = stderr_records(&output);
    assert_eq!(records[1]["kind"], "gh_identity");
    assert_eq!(records[1]["gh_command"], "gh");
    assert!(records[1]["gh_login"].is_null());
}

#[test]
fn it_should_end_a_stalled_lookup_at_its_own_bound_and_still_print_the_result() {
    let sandbox = Sandbox::with_pull(MERGED_PULL);
    sandbox.flag("user-hangs");
    let started = Instant::now();

    // The bound is a quarter of the timeout: two seconds here, far below the fake's 30 second stall.
    let output = sandbox.run(&["--pr-number", "7", "--timeout-seconds", "8"]);

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(json(&output.stdout)["kind"], "pr_merged");
    assert!(stderr_records(&output)[1]["gh_login"].is_null());
    assert!(
        started.elapsed() < Duration::from_secs(7),
        "the lookup must end at its own bound"
    );
}

#[test]
fn it_should_exit_one_with_gh_unavailable_naming_the_command_when_the_command_cannot_start() {
    let output = Sandbox::command_without_fake_on_path(&["--pr-number", "7", "--gh-command", "/nonexistent/gh-command"])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(output.stdout, b"");
    let last = stderr_records(&output).pop().unwrap();
    assert_eq!(last["kind"], "gh_unavailable");
    assert!(last["message"].as_str().unwrap().contains("/nonexistent/gh-command"));
}

#[test]
fn it_should_name_the_command_in_a_gh_api_error() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);
    sandbox.flag("api-fails");
    let fake = sandbox.fake_path();

    let output = Sandbox::command_without_fake_on_path(&["--pr-number", "7", "--gh-command", &fake])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(1));
    let last = stderr_records(&output).pop().unwrap();
    assert_eq!(last["kind"], "gh_api_error");
    assert!(last["message"].as_str().unwrap().contains(&fake));
}

#[test]
fn it_should_document_gh_command_in_the_help() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);

    let output = sandbox.run(&["--help"]);

    let message = stderr_records(&output)[0]["message"].as_str().unwrap().to_owned();
    assert!(message.contains("--gh-command"), "help must document the option");
}

#[test]
fn it_should_exit_two_when_gh_command_is_empty() {
    let sandbox = Sandbox::with_pull(OPEN_PULL);

    let output = sandbox.run(&["--pr-number", "7", "--gh-command", ""]);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(kinds(&output), ["usage_error"]);
}
