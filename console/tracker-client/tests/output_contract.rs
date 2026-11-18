//! The global CLI output contract (`docs/adrs/20260519000000_define_global_cli_output_contract.md`)
//! for `tracker_client`, a `stdout-result-data` command:
//!
//! - it refuses to run when stdout is a terminal (exit `2`, one `tty_refusal` record on stderr);
//! - stdout holds one JSON object on success and nothing on failure;
//! - every stderr line is one JSON object.

mod common;

use std::io::{Read as _, Write as _};
use std::net::TcpListener;
use std::process::{Command, Output, Stdio};
use std::thread;

use serde_json::Value;

fn tracker_client(args: &[&str]) -> Command {
    let mut command = Command::new(common::resolve_tracker_client_binary());
    command
        .args(args)
        .env_remove("TORRUST_CHECKER_CONFIG")
        .env_remove("TORRUST_CHECKER_CONFIG_PATH");
    command
}

fn stdout_text(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout should be UTF-8")
}

/// Every stderr line parsed as JSON; fails on a line that is not one JSON object.
fn stderr_records(output: &Output) -> Vec<Value> {
    String::from_utf8(output.stderr.clone())
        .expect("stderr should be UTF-8")
        .lines()
        .map(|line| {
            let record: Value = serde_json::from_str(line).unwrap_or_else(|_| panic!("stderr line should be JSON: {line}"));
            assert!(record.is_object(), "stderr line should be a JSON object: {line}");
            record
        })
        .collect()
}

const EMPTY_CHECK_CONFIG: &str = r#"{"udp_trackers":[],"http_trackers":[],"health_checks":[]}"#;

#[cfg(unix)]
mod tty_refusal {
    use std::os::fd::OwnedFd;

    use rustix::fs::{Mode, OFlags};
    use rustix::pty::OpenptFlags;

    use super::*;

    /// A pseudo-terminal pair. The slave end is what the child sees as its stdout.
    struct Pty {
        master: OwnedFd,
        slave: OwnedFd,
    }

    impl Pty {
        fn open() -> Self {
            let master = rustix::pty::openpt(OpenptFlags::RDWR | OpenptFlags::NOCTTY).expect("openpt should succeed");
            rustix::pty::grantpt(&master).expect("grantpt should succeed");
            rustix::pty::unlockpt(&master).expect("unlockpt should succeed");
            let name = rustix::pty::ptsname(&master, Vec::new()).expect("ptsname should succeed");
            let slave =
                rustix::fs::open(name.as_c_str(), OFlags::RDWR | OFlags::NOCTTY, Mode::empty()).expect("the slave should open");
            Self { master, slave }
        }
    }

    /// Runs `command` with its stdout on a pseudo-terminal. Returns the process output (stdout is
    /// empty because it was the terminal) and whatever the child wrote to the terminal.
    fn run_on_terminal(mut command: Command) -> (Output, String) {
        let pty = Pty::open();
        command
            .stdin(Stdio::null())
            .stdout(Stdio::from(pty.slave.try_clone().expect("slave should clone")))
            .stderr(Stdio::piped());
        let child = command.spawn().expect("tracker_client should start");
        // Close every copy of the slave held here (the command owns one), so that reading the
        // master ends when the child exits.
        drop(command);
        drop(pty.slave);
        let output = child.wait_with_output().expect("tracker_client should finish");

        let mut master = std::fs::File::from(pty.master);
        let mut written = Vec::new();
        // Reading a master whose slaves are all closed fails with EIO: that is the end.
        drop(master.read_to_end(&mut written));

        (output, String::from_utf8_lossy(&written).into_owned())
    }

    fn assert_refused(command: Command) {
        let (output, terminal_output) = run_on_terminal(command);

        assert_eq!(output.status.code(), Some(2));
        assert_eq!(terminal_output, "", "nothing may be written to the terminal");
        let records = stderr_records(&output);
        assert_eq!(records.len(), 1, "records: {records:?}");
        assert_eq!(records[0]["kind"], "tty_refusal");
        assert!(records[0]["message"].as_str().is_some_and(|message| message.contains("TTY")));
    }

    #[test]
    fn it_should_refuse_to_run_the_check_subcommand_when_stdout_is_a_terminal() {
        let mut command = tracker_client(&["check"]);
        command.env("TORRUST_CHECKER_CONFIG", EMPTY_CHECK_CONFIG);

        assert_refused(command);
    }

    #[test]
    fn it_should_refuse_to_run_an_http_command_when_stdout_is_a_terminal() {
        assert_refused(tracker_client(&[
            "http",
            "scrape",
            "http://127.0.0.1:1",
            "9c38422213e30bff212b30c360d26f9a02136422",
        ]));
    }

    #[test]
    fn it_should_refuse_before_parsing_the_arguments() {
        assert_refused(tracker_client(&["--not-an-option"]));
    }
}

#[test]
fn it_should_write_the_check_results_as_one_json_object_in_both_formats() {
    for format in ["json", "text"] {
        let output = tracker_client(&["check", "--format", format])
            .env("TORRUST_CHECKER_CONFIG", EMPTY_CHECK_CONFIG)
            .output()
            .expect("tracker_client should run");

        assert_eq!(output.status.code(), Some(0), "format {format}");
        let result: Value = serde_json::from_str(&stdout_text(&output)).expect("stdout should be one JSON value");
        assert!(result.is_object(), "format {format}: {result}");
        assert_eq!(result["results"], serde_json::json!([]), "format {format}");
        assert_eq!(stderr_records(&output), Vec::<Value>::new(), "format {format}");
    }
}

#[test]
fn it_should_write_the_check_results_of_a_failing_tracker_inside_the_object() {
    let output = tracker_client(&["check"])
        .env(
            "TORRUST_CHECKER_CONFIG",
            r#"{"udp_trackers":[],"http_trackers":[],"health_checks":["http://127.0.0.1:1/api/health_check"]}"#,
        )
        .output()
        .expect("tracker_client should run");

    assert_eq!(output.status.code(), Some(0));
    let result: Value = serde_json::from_str(&stdout_text(&output)).expect("stdout should be one JSON value");
    assert_eq!(result["results"].as_array().map(Vec::len), Some(1), "result: {result}");
}

#[test]
fn it_should_report_a_panic_as_a_json_record_and_leave_stdout_empty() {
    // A UDP tracker URL without a port passes validation and panics while it is resolved.
    let output = tracker_client(&["check"])
        .env(
            "TORRUST_CHECKER_CONFIG",
            r#"{"udp_trackers":["udp://127.0.0.1"],"http_trackers":[],"health_checks":[]}"#,
        )
        .output()
        .expect("tracker_client should run");

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(stdout_text(&output), "");
    let records = stderr_records(&output);
    assert_eq!(records.len(), 2, "records: {records:?}");
    assert_eq!(records[0]["kind"], "panic");
    assert!(records[0]["message"].as_str().is_some_and(|m| m.contains("No port number")));
    assert_eq!(records[1]["error"]["kind"], "runtime_failure");
}

#[test]
fn it_should_report_help_as_a_stderr_record_and_leave_stdout_empty() {
    let output = tracker_client(&["--help"]).output().expect("tracker_client should run");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout_text(&output), "");
    let records = stderr_records(&output);
    assert_eq!(records.len(), 1, "records: {records:?}");
    assert_eq!(records[0]["kind"], "help");
    assert!(
        records[0]["message"]
            .as_str()
            .is_some_and(|message| message.contains("check")),
        "records: {records:?}"
    );
}

#[test]
fn it_should_report_a_usage_error_as_a_json_record_with_exit_2() {
    for args in [&["--not-an-option"][..], &["check", "--", "--not-an-option"][..]] {
        let output = tracker_client(args).output().expect("tracker_client should run");

        assert_eq!(output.status.code(), Some(2), "args {args:?}");
        assert_eq!(stdout_text(&output), "", "args {args:?}");
        let records = stderr_records(&output);
        assert_eq!(records.len(), 1, "args {args:?}: {records:?}");
        assert_eq!(records[0]["error"]["kind"], "usage_error", "args {args:?}");
    }
}

#[test]
fn it_should_report_a_runtime_failure_as_a_json_record_and_leave_stdout_empty() {
    let output = tracker_client(&["http", "announce", "http://127.0.0.1:7070", "invalid_info_hash"])
        .output()
        .expect("tracker_client should run");

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(stdout_text(&output), "");
    let records = stderr_records(&output);
    assert_eq!(records.len(), 1, "records: {records:?}");
    assert_eq!(records[0]["error"]["kind"], "runtime_failure");
}

/// A server that answers the first request with `body`, which is not a bencoded tracker response.
fn serve_once(body: &'static [u8]) -> (String, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("fake tracker should bind");
    let url = format!("http://{}", listener.local_addr().expect("fake tracker address")); // DevSkim: ignore DS137138
    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("fake tracker should accept");
        let mut request = [0_u8; 4096];
        drop(stream.read(&mut request));
        let head = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        );
        stream.write_all(head.as_bytes()).expect("fake tracker should answer");
        stream.write_all(body).expect("fake tracker should answer");
    });
    (url, handle)
}

#[test]
fn it_should_report_an_unrecognized_http_response_on_stderr_and_leave_stdout_empty() {
    for (command, body) in [
        (["http", "announce"], &b"not bencode \xff"[..]),
        (["http", "scrape"], &b"d3:fooi1ee"[..]),
    ] {
        let (url, server) = serve_once(Box::leak(body.to_vec().into_boxed_slice()));
        let output = tracker_client(&[command[0], command[1], &url, "9c38422213e30bff212b30c360d26f9a02136422"])
            .output()
            .expect("tracker_client should run");
        server.join().expect("fake tracker should finish");

        assert_eq!(output.status.code(), Some(1), "command {command:?}");
        assert_eq!(stdout_text(&output), "", "command {command:?}");
        let records = stderr_records(&output);
        assert_eq!(records.len(), 2, "command {command:?}: {records:?}");
        assert_eq!(records[0]["kind"], "unrecognized_response", "command {command:?}");
        assert_eq!(records[0]["command"], command[1], "command {command:?}");
        assert_eq!(records[1]["error"]["kind"], "runtime_failure", "command {command:?}");
    }
}
