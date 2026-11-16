//! End-to-end checks of the `http_health_check` binary against the global CLI output contract
//! (`no-stdout-result`): empty stdout, one JSON object per line on stderr, exit `0` healthy, `1`
//! unhealthy or unreachable, `2` usage error (issue #2764).

use std::ffi::OsStr;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpListener};
use std::path::PathBuf;
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

/// The binary under test.
///
/// nextest sets `NEXTEST_BIN_EXE_http_health_check` at run time, remapped to where the archive was
/// extracted. The container's `test` stages run from an archive built in another stage, where the
/// path Cargo compiled into `CARGO_BIN_EXE_http_health_check` no longer exists, so the nextest
/// variable is read first and the compiled-in path is the fallback for `cargo test`.
fn health_check_binary() -> PathBuf {
    std::env::var_os("NEXTEST_BIN_EXE_http_health_check")
        .filter(|path| !path.is_empty())
        .map_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_http_health_check")), PathBuf::from)
}

/// Proxy variables `reqwest` honours by default; removed from the child so every probe reaches the
/// local test server whatever the host's proxy settings are.
const PROXY_VARIABLES: [&str; 6] = [
    "HTTP_PROXY",
    "http_proxy",
    "HTTPS_PROXY",
    "https_proxy",
    "ALL_PROXY",
    "all_proxy",
];

/// Runs the binary with the given arguments, with stdout and stderr piped, within the deadline.
fn run_health_check<S: AsRef<OsStr>>(arguments: &[S]) -> Output {
    let mut command = Command::new(health_check_binary());
    for variable in PROXY_VARIABLES {
        command.env_remove(variable);
    }
    let child = command
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

/// A socket bound to a loopback port that never listens: while it is held, nothing else can bind the
/// port, and a connection to it is refused.
fn hold_a_port_that_refuses_connections() -> (tokio::net::TcpSocket, u16) {
    let socket = tokio::net::TcpSocket::new_v4().expect("create a TCP socket");
    socket
        .bind(SocketAddr::from((Ipv4Addr::LOCALHOST, 0)))
        .expect("bind the TCP socket without listening");
    let port = socket.local_addr().expect("read the bound address").port();
    (socket, port)
}

/// Asserts the exit code, showing stderr (the record that explains the outcome) on failure.
fn assert_exit_code(output: &Output, expected: i32) {
    assert_eq!(
        output.status.code(),
        Some(expected),
        "unexpected exit code; stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );
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

/// Asserts that no part of a credential-bearing URL reaches stderr.
fn assert_no_credentials(output: &Output) {
    let stderr = String::from_utf8_lossy(&output.stderr);
    for secret in ["probe-user", "userinfo-secret", "query-secret", "path-secret"] {
        assert!(!stderr.contains(secret), "stderr exposes `{secret}`: {stderr}");
    }
}

/// A URL for the given port whose userinfo, path and query carry credentials.
fn credential_url(port: u16) -> String {
    format!("http://probe-user:userinfo-secret@127.0.0.1:{port}/path-secret/health_check?token=query-secret")
}

#[test]
fn it_should_exit_0_and_report_healthy_when_the_endpoint_answers_2xx() {
    let server = TestServer::start("200 OK");

    let output = run_health_check(&[server.url()]);

    assert_exit_code(&output, 0);
    let records = records(&output);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0]["kind"], "healthy");
    assert_eq!(records[0]["status"], 200);
    assert_eq!(records[0]["exit_code"], 0);
}

#[test]
fn it_should_exit_1_and_report_unhealthy_when_the_endpoint_answers_a_non_2xx_status() {
    let server = TestServer::start("503 Service Unavailable");

    let output = run_health_check(&[server.url()]);

    assert_exit_code(&output, 1);
    let records = records(&output);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0]["kind"], "unhealthy");
    assert_eq!(records[0]["status"], 503);
    assert_eq!(records[0]["exit_code"], 1);
}

#[test]
fn it_should_exit_1_and_report_unreachable_when_nothing_listens_on_the_port() {
    let (held_socket, dead_port) = hold_a_port_that_refuses_connections();

    let output = run_health_check(&[format!("http://127.0.0.1:{dead_port}/health_check")]);

    drop(held_socket);
    assert_exit_code(&output, 1);
    let records = records(&output);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0]["kind"], "unreachable");
    assert!(records[0]["message"].is_string());
    assert_eq!(records[0]["exit_code"], 1);
}

#[test]
fn it_should_exit_2_with_a_usage_error_record_when_the_url_argument_is_missing() {
    let output = run_health_check::<&str>(&[]);

    assert_exit_code(&output, 2);
    let records = records(&output);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0]["kind"], "usage_error");
    assert_eq!(records[0]["exit_code"], 2);
}

#[test]
fn it_should_exit_2_with_a_usage_error_record_when_there_are_extra_arguments() {
    let output = run_health_check(&["http://127.0.0.1:1/health_check", "extra"]);

    assert_exit_code(&output, 2);
    let records = records(&output);
    assert_eq!(records[0]["kind"], "usage_error");
}

#[test]
fn it_should_exit_2_with_a_usage_error_record_when_the_argument_is_not_a_url() {
    let output = run_health_check(&["not-a-url-path-secret"]);

    assert_exit_code(&output, 2);
    let records = records(&output);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0]["kind"], "usage_error");
    assert_no_credentials(&output);
}

#[test]
fn it_should_exit_2_with_a_usage_error_record_when_the_url_scheme_is_not_http_or_https() {
    for url in ["ftp://127.0.0.1/health_check", "mailto:probe@example.com"] {
        let output = run_health_check(&[url]);

        assert_exit_code(&output, 2);
        let records = records(&output);
        assert_eq!(records.len(), 1, "one record for `{url}`");
        assert_eq!(records[0]["kind"], "usage_error", "the kind for `{url}`");
    }
}

#[cfg(unix)]
#[test]
fn it_should_exit_2_with_a_usage_error_record_when_the_argument_is_not_valid_utf8() {
    use std::os::unix::ffi::OsStrExt as _;

    let output = run_health_check(&[OsStr::from_bytes(b"http://127.0.0.1/\xff")]);

    assert_exit_code(&output, 2);
    let records = records(&output);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0]["kind"], "usage_error");
}

#[test]
fn it_should_not_expose_url_credentials_when_the_endpoint_is_healthy() {
    let server = TestServer::start("200 OK");

    let output = run_health_check(&[credential_url(server.address.port())]);

    assert_exit_code(&output, 0);
    assert_eq!(records(&output)[0]["kind"], "healthy");
    assert_no_credentials(&output);
}

#[test]
fn it_should_not_expose_url_credentials_when_the_endpoint_is_unhealthy() {
    let server = TestServer::start("503 Service Unavailable");

    let output = run_health_check(&[credential_url(server.address.port())]);

    assert_exit_code(&output, 1);
    assert_eq!(records(&output)[0]["kind"], "unhealthy");
    assert_no_credentials(&output);
}

#[test]
fn it_should_not_expose_url_credentials_when_the_endpoint_is_unreachable() {
    let (held_socket, dead_port) = hold_a_port_that_refuses_connections();

    let output = run_health_check(&[credential_url(dead_port)]);

    drop(held_socket);
    assert_exit_code(&output, 1);
    assert_eq!(records(&output)[0]["kind"], "unreachable");
    assert_no_credentials(&output);
}
