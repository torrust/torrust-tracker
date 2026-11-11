//! End-to-end checks of the `http_health_check` binary against the global CLI output contract
//! (`no-stdout-result`): empty stdout, one JSON object per line on stderr, exit `0` healthy, `1`
//! unhealthy or unreachable, `2` usage error (issue #2764).

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener};
use std::process::{Child, Command, Output, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use serde_json::Value;

/// Absolute deadline for the child process (the binary's own request timeout is 5 seconds).
const CHILD_DEADLINE: Duration = Duration::from_secs(30);

/// A local HTTP server that answers every request with a fixed status line; stops on drop.
struct TestServer {
    address: SocketAddr,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl TestServer {
    /// Binds an ephemeral port before returning, so the server is ready when the child starts.
    fn start(status_line: &'static str) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("an ephemeral port should bind");
        listener.set_nonblocking(true).expect("the listener should be non-blocking");
        let address = listener.local_addr().expect("the listener has an address");
        let stop = Arc::new(AtomicBool::new(false));
        let stop_flag = stop.clone();

        let thread = thread::spawn(move || {
            while !stop_flag.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        stream.set_nonblocking(false).ok();
                        stream.set_read_timeout(Some(Duration::from_secs(2))).ok();
                        let mut buffer = [0_u8; 1024];
                        drop(stream.read(&mut buffer));
                        let response = format!("HTTP/1.1 {status_line}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
                        drop(stream.write_all(response.as_bytes()));
                    }
                    Err(_) => thread::sleep(Duration::from_millis(10)),
                }
            }
        });

        Self {
            address,
            stop,
            thread: Some(thread),
        }
    }

    fn url(&self) -> String {
        format!("http://{}/health_check", self.address)
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(thread) = self.thread.take() {
            drop(thread.join());
        }
    }
}

/// A child process that is killed and reaped on drop, so a failed assertion leaks nothing.
struct ChildGuard(Option<Child>);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        if let Some(mut child) = self.0.take() {
            drop(child.kill());
            drop(child.wait());
        }
    }
}

/// Runs the binary with the given arguments, with stdout and stderr piped, within the deadline.
fn run_health_check(arguments: &[&str]) -> Output {
    let child = Command::new(env!("CARGO_BIN_EXE_http_health_check"))
        .args(arguments)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the http_health_check binary should start");
    let mut guard = ChildGuard(Some(child));

    let deadline = Instant::now() + CHILD_DEADLINE;
    loop {
        let child = guard.0.as_mut().expect("the child is owned by the guard");
        if child.try_wait().expect("the child status should be readable").is_some() {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "http_health_check did not finish within {CHILD_DEADLINE:?}"
        );
        thread::sleep(Duration::from_millis(20));
    }

    guard
        .0
        .take()
        .expect("the child is owned by the guard")
        .wait_with_output()
        .expect("the output should be collected")
}

/// Asserts stdout is empty and returns the stderr records, each parsed as one JSON object with a `kind`.
fn records(output: &Output) -> Vec<Value> {
    assert!(
        output.stdout.is_empty(),
        "stdout must be empty, but was: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8(output.stderr.clone()).expect("stderr is UTF-8");
    let records: Vec<Value> = stderr
        .lines()
        .map(|line| serde_json::from_str(line).unwrap_or_else(|error| panic!("stderr line is not JSON ({error}): {line}")))
        .collect();
    assert!(!records.is_empty(), "stderr must carry at least one record");
    for record in &records {
        assert!(record.is_object(), "every stderr line is a JSON object: {record}");
        assert!(record["kind"].is_string(), "every record has a kind: {record}");
    }
    records
}

#[test]
fn it_should_exit_0_and_report_healthy_when_the_endpoint_answers_2xx() {
    let server = TestServer::start("200 OK");

    let output = run_health_check(&[&server.url()]);

    assert_eq!(output.status.code(), Some(0));
    let records = records(&output);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0]["kind"], "healthy");
    assert_eq!(records[0]["status"], 200);
    assert_eq!(records[0]["exit_code"], 0);
}

#[test]
fn it_should_exit_1_and_report_unhealthy_when_the_endpoint_answers_a_non_2xx_status() {
    let server = TestServer::start("503 Service Unavailable");

    let output = run_health_check(&[&server.url()]);

    assert_eq!(output.status.code(), Some(1));
    let records = records(&output);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0]["kind"], "unhealthy");
    assert_eq!(records[0]["status"], 503);
    assert_eq!(records[0]["exit_code"], 1);
}

#[test]
fn it_should_exit_1_and_report_unreachable_when_nothing_listens_on_the_port() {
    // Bind a port to learn its number, then close it before the probe.
    let closed_port = TcpListener::bind("127.0.0.1:0")
        .expect("an ephemeral port should bind")
        .local_addr()
        .expect("the listener has an address")
        .port();

    let output = run_health_check(&[&format!("http://127.0.0.1:{closed_port}/health_check")]);

    assert_eq!(output.status.code(), Some(1));
    let records = records(&output);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0]["kind"], "unreachable");
    assert!(records[0]["message"].is_string());
    assert_eq!(records[0]["exit_code"], 1);
}

#[test]
fn it_should_exit_2_with_a_usage_error_record_when_the_url_argument_is_missing() {
    let output = run_health_check(&[]);

    assert_eq!(output.status.code(), Some(2));
    let records = records(&output);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0]["kind"], "usage_error");
    assert_eq!(records[0]["exit_code"], 2);
}

#[test]
fn it_should_exit_2_with_a_usage_error_record_when_there_are_extra_arguments() {
    let output = run_health_check(&["http://127.0.0.1:1/health_check", "extra"]);

    assert_eq!(output.status.code(), Some(2));
    let records = records(&output);
    assert_eq!(records[0]["kind"], "usage_error");
}
