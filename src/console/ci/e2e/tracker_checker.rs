use std::io::{self, Read};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};

/// Absolute deadline for building `tracker_client`.
const BUILD_DEADLINE: Duration = Duration::from_secs(15 * 60);

/// Absolute deadline for the checker run itself (the network timeout of the
/// checker is shorter; this only guarantees that a hung child cannot hang the
/// runner). The binary is built before, so this does not include the build.
const CHECKER_DEADLINE: Duration = Duration::from_secs(5 * 60);

/// How long the output readers may keep running after the child has exited or
/// was killed. It bounds the drain when something else keeps a pipe open.
const READER_GRACE: Duration = Duration::from_secs(5);

/// How often the wait loop polls the child.
const POLL_INTERVAL: Duration = Duration::from_millis(100);

/// The arguments of the command that builds the checker (after `cargo`).
const BUILD_ARGS: [&str; 5] = ["build", "-p", "torrust-tracker-client", "--bin", "tracker_client"];

/// The arguments of the command that runs the checker (after `cargo`).
const CHECKER_ARGS: [&str; 7] = [
    "run",
    "-p",
    "torrust-tracker-client",
    "--bin",
    "tracker_client",
    "--",
    "check",
];

/// Runs the Tracker Checker (`tracker_client check`).
///
/// The binary is built first, under its own deadline, so the deadline of the
/// check does not include a cold build. The configuration is passed to the
/// child in `TORRUST_CHECKER_CONFIG`. The child's stdout and stderr are
/// captured (not inherited) and logged, so the runner's own stdout is never
/// written by the child.
///
/// # Errors
///
/// Will return an error if the Tracker Checker cannot be built or launched,
/// exits with a non-zero status or does not finish before its deadline.
pub fn run(config_content: &str) -> io::Result<()> {
    run_command("cargo", &BUILD_ARGS, None, BUILD_DEADLINE, READER_GRACE)?;
    run_command("cargo", &CHECKER_ARGS, Some(config_content), CHECKER_DEADLINE, READER_GRACE)
}

/// What a finished child left behind.
struct Captured {
    /// The exit status, or the error that ended the wait (for example the deadline).
    status: io::Result<ExitStatus>,
    stdout: String,
    stderr: String,
}

fn run_command(
    program: &str,
    args: &[&str],
    config_content: Option<&str>,
    deadline: Duration,
    reader_grace: Duration,
) -> io::Result<()> {
    tracing::info!(
        "Running: {}{program} {}",
        if config_content.is_some() {
            "TORRUST_CHECKER_CONFIG=[config] "
        } else {
            ""
        },
        args.join(" ")
    );
    if let Some(config) = config_content {
        tracing::info!("Tracker Checker config:\n{config}");
    }

    let captured = execute(program, args, config_content, deadline, reader_grace)?;

    log_output("stdout", &captured.stdout);
    log_output("stderr", &captured.stderr);

    let status = captured.status?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!("`{program} {}` failed: {status}", args.join(" "))))
    }
}

/// Spawns the child with piped output, drains both pipes and waits for it.
///
/// The child is always killed (when still running) and reaped before this
/// returns, on every path including a panic. The captured output is returned
/// even when the wait ended with an error (the deadline), so it can be logged. Both pipes are read on their own
/// threads, so a full pipe cannot block the child, and the draining is bounded
/// by `reader_grace` after the child ends, so a pipe kept open by something
/// else cannot hold this function past its deadline.
///
/// Assumption: the child has no descendant that outlives it. On Unix `cargo run`
/// replaces itself with the binary it runs, so the process killed at the
/// deadline is the checker itself. A descendant that kept a pipe open would only
/// cost the grace period and part of the output, not a hang.
fn execute(
    program: &str,
    args: &[&str],
    config_content: Option<&str>,
    deadline: Duration,
    reader_grace: Duration,
) -> io::Result<Captured> {
    let mut command = Command::new(program);
    command
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(config) = config_content {
        command.env("TORRUST_CHECKER_CONFIG", config);
    }

    let mut child = ChildGuard(command.spawn()?);

    let stdout = child.0.stdout.take().map(spawn_reader);
    let stderr = child.0.stderr.take().map(spawn_reader);

    let wait_result = wait_with_deadline(&mut child.0, deadline);

    // One window for both pipes: the drain after the child ends is bounded by `reader_grace`.
    let end = Instant::now() + reader_grace;
    let stdout = collect(stdout, end);
    let stderr = collect(stderr, end);

    Ok(Captured {
        status: wait_result,
        stdout,
        stderr,
    })
}

/// Kills and reaps the child on drop (a no-op when it already exited).
struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        if matches!(self.0.try_wait(), Ok(None)) {
            drop(self.0.kill());
        }
        drop(self.0.wait());
    }
}

fn log_output(stream: &str, content: &str) {
    if !content.is_empty() {
        tracing::info!("{stream}:\n{content}");
    }
}

/// Reads the pipe in chunks and sends each one, so whatever was read before a
/// deadline is kept.
fn spawn_reader<R: Read + Send + 'static>(mut pipe: R) -> Receiver<Vec<u8>> {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let mut buffer = [0_u8; 8192];
        loop {
            match pipe.read(&mut buffer) {
                Ok(0) | Err(_) => break,
                Ok(read) => {
                    if sender.send(buffer[..read].to_vec()).is_err() {
                        break;
                    }
                }
            }
        }
    });

    receiver
}

/// Collects the chunks until the pipe closes or the `end` instant has passed.
fn collect(receiver: Option<Receiver<Vec<u8>>>, end: Instant) -> String {
    let Some(receiver) = receiver else {
        return String::new();
    };

    let mut bytes = Vec::new();

    loop {
        let remaining = end.saturating_duration_since(Instant::now());
        match receiver.recv_timeout(remaining) {
            Ok(chunk) => bytes.extend_from_slice(&chunk),
            Err(RecvTimeoutError::Timeout) => {
                tracing::warn!("Output of the child was still open at the end of the drain window; it was cut");
                break;
            }
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }

    String::from_utf8_lossy(&bytes).into_owned()
}

fn wait_with_deadline(child: &mut Child, deadline: Duration) -> io::Result<ExitStatus> {
    let start = Instant::now();

    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(status);
        }

        if start.elapsed() >= deadline {
            drop(child.kill());
            drop(child.wait());
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                format!("the child did not finish within {deadline:?}"),
            ));
        }

        thread::sleep(POLL_INTERVAL);
    }
}

#[cfg(all(test, unix))]
mod tests {
    use std::io::ErrorKind;
    use std::time::{Duration, Instant};

    use super::{BUILD_ARGS, CHECKER_ARGS, execute, run_command};

    const GRACE: Duration = Duration::from_secs(5);

    #[test]
    fn it_should_launch_the_unified_tracker_client_check_command() {
        // Arrange
        let expected = "run -p torrust-tracker-client --bin tracker_client -- check";

        // Act
        let command_line = CHECKER_ARGS.join(" ");

        // Assert
        assert_eq!(command_line, expected);
    }

    #[test]
    fn it_should_build_the_same_binary_before_running_it() {
        // Arrange
        let expected = "build -p torrust-tracker-client --bin tracker_client";

        // Act
        let command_line = BUILD_ARGS.join(" ");

        // Assert
        assert_eq!(command_line, expected);
    }

    #[test]
    fn it_should_pass_the_configuration_in_the_checker_config_env_var() {
        // Arrange
        let config = r#"{"a":1}"#;
        let script = r#"printf '%s' "$TORRUST_CHECKER_CONFIG""#;

        // Act
        let captured = execute("sh", &["-c", script], Some(config), Duration::from_secs(10), GRACE).expect("it should run");

        // Assert
        assert_eq!(captured.stdout, config);
    }

    #[test]
    fn it_should_capture_stdout_and_stderr_separately() {
        // Arrange
        let script = "echo out; echo err >&2";

        // Act
        let captured = execute("sh", &["-c", script], None, Duration::from_secs(10), GRACE).expect("it should run");

        // Assert
        assert_eq!(captured.stdout, "out\n");
        assert_eq!(captured.stderr, "err\n");
    }

    #[test]
    fn it_should_report_a_non_zero_exit_status_as_an_error() {
        // Arrange
        let args = ["-c", "exit 1"];

        // Act
        let result = run_command("sh", &args, None, Duration::from_secs(10), GRACE);

        // Assert
        let error = result.expect_err("a non-zero exit should be an error");
        assert_eq!(error.kind(), ErrorKind::Other);
    }

    #[test]
    fn it_should_capture_everything_when_the_child_writes_more_than_a_pipe_buffer_to_both_streams() {
        // Arrange
        let script = "head -c 300000 /dev/zero | tr '\\0' x; head -c 300000 /dev/zero | tr '\\0' y >&2";

        // Act
        let captured = execute("sh", &["-c", script], None, Duration::from_secs(30), GRACE).expect("it should run");

        // Assert
        assert_eq!(captured.stdout, "x".repeat(300_000));
        assert_eq!(captured.stderr, "y".repeat(300_000));
    }

    #[test]
    fn it_should_kill_the_child_and_return_promptly_when_the_deadline_expires() {
        // Arrange
        let start = Instant::now();

        // Act
        let result = execute("sh", &["-c", "exec sleep 60"], None, Duration::from_millis(300), GRACE);

        // Assert: the child was killed (a surviving `sleep 60` would be waited out).
        let error = result
            .expect("it should run")
            .status
            .expect_err("a hung child should time out");
        assert_eq!(error.kind(), ErrorKind::TimedOut);
        assert!(start.elapsed() < Duration::from_secs(20), "took {:?}", start.elapsed());
    }

    #[test]
    fn it_should_return_the_output_written_before_the_deadline_expired() {
        // Arrange: the child writes a line and then hangs.
        let script = "echo before; echo err_before >&2; exec sleep 60";

        // Act
        let captured = execute("sh", &["-c", script], None, Duration::from_millis(500), GRACE).expect("it should run");

        // Assert
        assert_eq!(
            captured.status.expect_err("a hung child should time out").kind(),
            ErrorKind::TimedOut
        );
        assert_eq!(captured.stdout, "before\n");
        assert_eq!(captured.stderr, "err_before\n");
    }

    #[test]
    fn it_should_share_one_drain_window_between_both_pipes() {
        // Arrange: the child exits at once while a descendant holds both pipes for 8 s.
        let script = "sleep 8 &";
        let start = Instant::now();

        // Act
        let captured =
            execute("sh", &["-c", script], None, Duration::from_secs(10), Duration::from_secs(1)).expect("it should run");

        // Assert: one 1 s window, not two.
        assert!(captured.status.is_ok());
        assert!(start.elapsed() < Duration::from_millis(1800), "took {:?}", start.elapsed());
    }

    #[test]
    fn it_should_not_wait_for_a_descendant_that_keeps_the_pipes_open() {
        // Arrange: the main process hangs and a descendant holds both pipes for 8 s.
        let script = "sleep 8 & exec sleep 60";
        let start = Instant::now();

        // Act
        let result = execute(
            "sh",
            &["-c", script],
            None,
            Duration::from_millis(300),
            Duration::from_millis(500),
        );

        // Assert: the drain is cut after the grace period, long before the 8 s of the descendant.
        let error = result
            .expect("it should run")
            .status
            .expect_err("a hung child should time out");
        assert_eq!(error.kind(), ErrorKind::TimedOut);
        assert!(start.elapsed() < Duration::from_secs(5), "took {:?}", start.elapsed());
    }

    #[test]
    fn it_should_keep_the_output_read_before_a_descendant_held_the_pipe_open() {
        // Arrange: the child exits at once, a descendant keeps stdout open for 8 s.
        let script = "echo first; sleep 8 &";
        let start = Instant::now();

        // Act
        let captured = execute(
            "sh",
            &["-c", script],
            None,
            Duration::from_secs(10),
            Duration::from_millis(500),
        )
        .expect("it should run");

        // Assert
        assert_eq!(captured.stdout, "first\n");
        assert!(start.elapsed() < Duration::from_secs(5), "took {:?}", start.elapsed());
    }

    #[test]
    fn it_should_fail_when_the_program_cannot_be_launched() {
        // Arrange
        let program = "/nonexistent/program";

        // Act
        let result = run_command(program, &[], None, Duration::from_secs(1), GRACE);

        // Assert
        let error = result.expect_err("a missing program should be an error");
        assert_eq!(error.kind(), ErrorKind::NotFound);
    }
}
