use std::io::{self, Read};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, TryRecvError};
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

/// The most bytes kept from each stream. It bounds the memory of a child that
/// writes without end and the time of the last, non-waiting step of the drain.
const MAX_CAPTURED_BYTES: usize = 8 * 1024 * 1024;

/// The most bytes of each stream logged when the run succeeded.
const MAX_SUCCESS_LOG_BYTES: usize = 64 * 1024;

/// How many discarded bytes pass between two probes of the receiver by a reader thread.
const DISCARD_PROBE_BYTES: usize = 1024 * 1024;

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

    // Full output (up to the capture cap) when the run failed; only a bounded excerpt when it succeeded.
    let succeeded = captured.status.as_ref().is_ok_and(ExitStatus::success);
    let limit = if succeeded {
        MAX_SUCCESS_LOG_BYTES
    } else {
        MAX_CAPTURED_BYTES
    };
    log_output("stdout", &captured.stdout, limit);
    log_output("stderr", &captured.stderr, limit);

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

fn log_output(stream: &str, content: &str, limit: usize) {
    if content.is_empty() {
        return;
    }

    let excerpt = excerpt(content, limit);
    if excerpt.len() < content.len() {
        tracing::info!("{stream} (first {} of {} bytes):\n{excerpt}", excerpt.len(), content.len());
    } else {
        tracing::info!("{stream}:\n{content}");
    }
}

/// The start of `content`, at most `limit` bytes long and cut on a character boundary.
fn excerpt(content: &str, limit: usize) -> &str {
    let end = (0..=limit.min(content.len()))
        .rev()
        .find(|end| content.is_char_boundary(*end))
        .unwrap_or(0);
    &content[..end]
}

/// Reads the pipe in chunks and sends each one, so whatever was read before a
/// deadline is kept.
///
/// At most `MAX_CAPTURED_BYTES` are queued. The reader goes on reading and
/// discarding the rest, so the child never blocks on a full pipe and a child
/// that writes without end cannot grow the memory.
fn spawn_reader<R: Read + Send + 'static>(mut pipe: R) -> Receiver<Vec<u8>> {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let mut buffer = [0_u8; 8192];
        let mut queued = 0_usize;
        let mut discarded_since_probe = 0_usize;
        loop {
            match pipe.read(&mut buffer) {
                Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
                Ok(0) | Err(_) => break,
                Ok(read) => {
                    let keep = read.min(MAX_CAPTURED_BYTES.saturating_sub(queued));
                    queued += keep;
                    // Past the cap nothing is sent, so probe the receiver with an empty chunk once per
                    // `DISCARD_PROBE_BYTES` discarded: when it is gone the thread ends and closes the pipe.
                    let probe = keep < read && {
                        discarded_since_probe += read - keep;
                        discarded_since_probe >= DISCARD_PROBE_BYTES
                    };
                    if probe {
                        discarded_since_probe = 0;
                    }
                    if (keep > 0 || probe) && sender.send(buffer[..keep].to_vec()).is_err() {
                        break;
                    }
                }
            }
        }
    });

    receiver
}

/// Receives the next chunk, waiting only until `end`; after `end` it takes a
/// chunk that is already queued and never waits.
fn receive(receiver: &Receiver<Vec<u8>>, end: Instant) -> Result<Vec<u8>, RecvTimeoutError> {
    let remaining = end
        .checked_duration_since(Instant::now())
        .filter(|remaining| !remaining.is_zero());

    remaining.map_or_else(
        || {
            receiver.try_recv().map_err(|error| match error {
                TryRecvError::Empty => RecvTimeoutError::Timeout,
                TryRecvError::Disconnected => RecvTimeoutError::Disconnected,
            })
        },
        |remaining| receiver.recv_timeout(remaining),
    )
}

/// Collects the chunks until the pipe closes or the `end` instant has passed.
///
/// After the window ends, chunks that were already read and queued are still
/// taken (without waiting), so output read before the end is not lost; at most
/// `MAX_CAPTURED_BYTES` are kept, which bounds both the time of this last step
/// and the memory of a child that writes without end.
fn collect(receiver: Option<Receiver<Vec<u8>>>, end: Instant) -> String {
    let Some(receiver) = receiver else {
        return String::new();
    };

    let mut bytes = Vec::new();

    while bytes.len() < MAX_CAPTURED_BYTES {
        let next = receive(&receiver, end);

        match next {
            Ok(chunk) => bytes.extend_from_slice(&chunk),
            Err(RecvTimeoutError::Timeout) => {
                tracing::warn!("Output of the child was still open at the end of the drain window; it was cut");
                break;
            }
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }

    if bytes.len() > MAX_CAPTURED_BYTES {
        tracing::warn!("Output of the child exceeded {MAX_CAPTURED_BYTES} bytes; it was cut");
        bytes.truncate(MAX_CAPTURED_BYTES);
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

    use super::{
        BUILD_ARGS, CHECKER_ARGS, DISCARD_PROBE_BYTES, MAX_CAPTURED_BYTES, collect, excerpt, execute, run_command, spawn_reader,
    };

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
    fn it_should_keep_the_chunks_already_queued_when_the_drain_window_ended() {
        // Arrange: a chunk was read and queued, and the window ended a second ago.
        let (sender, receiver) = std::sync::mpsc::channel();
        sender.send(b"queued".to_vec()).expect("the receiver is alive");
        let ended = Instant::now()
            .checked_sub(Duration::from_secs(1))
            .expect("the clock is past one second");

        // Act
        let collected = collect(Some(receiver), ended);

        // Assert
        assert_eq!(collected, "queued");
    }

    #[test]
    fn it_should_cut_the_output_at_the_byte_cap_even_when_more_is_queued() {
        // Arrange: more than the cap is queued and the window ended a second ago.
        let (sender, receiver) = std::sync::mpsc::channel();
        sender
            .send(vec![b'x'; MAX_CAPTURED_BYTES + 1000])
            .expect("the receiver is alive");
        let ended = Instant::now()
            .checked_sub(Duration::from_secs(1))
            .expect("the clock is past one second");

        // Act
        let collected = collect(Some(receiver), ended);

        // Assert
        assert_eq!(collected.len(), MAX_CAPTURED_BYTES);
    }

    #[test]
    fn it_should_queue_at_most_the_byte_cap_from_a_pipe_that_writes_more() {
        // Arrange: a pipe that holds more than the cap.
        let pipe = std::io::Cursor::new(vec![b'x'; MAX_CAPTURED_BYTES + 100_000]);

        // Act
        let receiver = spawn_reader(pipe);
        let queued: usize = receiver.iter().map(|chunk| chunk.len()).sum();

        // Assert: the reader read it all (the channel closed) but queued only the cap.
        assert_eq!(queued, MAX_CAPTURED_BYTES);
    }

    #[test]
    fn it_should_stop_reading_a_pipe_past_the_cap_when_the_receiver_is_gone() {
        // Arrange: a pipe that never ends, counting what the reader takes, and a receiver already dropped.
        struct Endless(std::sync::Arc<std::sync::atomic::AtomicUsize>);
        impl std::io::Read for Endless {
            fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
                self.0.fetch_add(buffer.len(), std::sync::atomic::Ordering::SeqCst);
                buffer.fill(b'x');
                Ok(buffer.len())
            }
        }
        let read = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let receiver = spawn_reader(Endless(read.clone()));

        // Act: let the reader pass the cap and discard some, then drop the receiver.
        let load = || read.load(std::sync::atomic::Ordering::SeqCst);
        let start = Instant::now();
        while load() < MAX_CAPTURED_BYTES + 3 * DISCARD_PROBE_BYTES && start.elapsed() < Duration::from_secs(20) {
            std::thread::sleep(Duration::from_millis(5));
        }
        let at_drop = load();
        drop(receiver);
        std::thread::sleep(Duration::from_millis(300));
        let after_drop = load();
        std::thread::sleep(Duration::from_millis(300));

        // Assert: it passed the cap, and after the drop it ended: the count stopped growing.
        assert!(
            at_drop >= MAX_CAPTURED_BYTES + 3 * DISCARD_PROBE_BYTES,
            "read only {at_drop} bytes"
        );
        assert_eq!(load(), after_drop);
    }

    #[test]
    fn it_should_retry_a_read_interrupted_by_a_signal_instead_of_ending_the_stream() {
        // Arrange: a pipe whose first read is interrupted, followed by data.
        struct InterruptedOnce {
            interrupted: bool,
            data: std::io::Cursor<Vec<u8>>,
        }
        impl std::io::Read for InterruptedOnce {
            fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
                if self.interrupted {
                    return self.data.read(buffer);
                }
                self.interrupted = true;
                Err(std::io::ErrorKind::Interrupted.into())
            }
        }
        let pipe = InterruptedOnce {
            interrupted: false,
            data: std::io::Cursor::new(b"after".to_vec()),
        };

        // Act
        let collected = collect(Some(spawn_reader(pipe)), Instant::now() + GRACE);

        // Assert
        assert_eq!(collected, "after");
    }

    #[test]
    fn it_should_cut_the_log_excerpt_on_a_character_boundary() {
        // Arrange: a three-byte character straddles the limit.
        let content = "ab\u{20ac}cd";

        // Act
        let cut = excerpt(content, 3);

        // Assert
        assert_eq!(cut, "ab");
    }

    #[test]
    fn it_should_keep_the_whole_content_when_it_is_within_the_log_limit() {
        // Arrange
        let content = "short";

        // Act
        let cut = excerpt(content, 64);

        // Assert
        assert_eq!(cut, content);
    }

    #[test]
    fn it_should_keep_the_output_of_the_second_pipe_when_the_first_one_uses_the_whole_window() {
        // Arrange: the child exits after writing to both pipes while a descendant holds them open.
        let script = "echo out; echo err >&2; sleep 8 &";

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
        assert_eq!(captured.stdout, "out\n");
        assert_eq!(captured.stderr, "err\n");
    }

    #[test]
    fn it_should_share_one_drain_window_between_both_pipes() {
        // Arrange: the child exits at once while a descendant holds both pipes for 8 s.
        let script = "sleep 8 &";
        let start = Instant::now();

        // Act
        let captured =
            execute("sh", &["-c", script], None, Duration::from_secs(10), Duration::from_secs(3)).expect("it should run");

        // Assert: one 3 s window (about 3.1 s), not two (about 6.1 s), with 1.9 s of slack for a loaded runner.
        assert!(captured.status.is_ok());
        assert!(start.elapsed() < Duration::from_secs(5), "took {:?}", start.elapsed());
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
