use std::io::{self, Read};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

/// Absolute deadline for the whole checker run, including the `cargo run`
/// build. The checker has its own network timeout; this only guarantees that
/// a hung child cannot hang the runner.
const CHECKER_DEADLINE: Duration = Duration::from_secs(15 * 60);

/// How often the wait loop polls the child.
const POLL_INTERVAL: Duration = Duration::from_millis(100);

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
/// The configuration is passed to the child in `TORRUST_CHECKER_CONFIG`. The
/// child's stdout and stderr are captured (not inherited) and logged, so the
/// runner's own stdout is never written by the child.
///
/// # Errors
///
/// Will return an error if the Tracker Checker cannot be launched, exits with
/// a non-zero status or does not finish before the deadline.
pub fn run(config_content: &str) -> io::Result<()> {
    run_command("cargo", &CHECKER_ARGS, config_content, CHECKER_DEADLINE)
}

fn run_command(program: &str, args: &[&str], config_content: &str, deadline: Duration) -> io::Result<()> {
    tracing::info!(
        "Running Tracker Checker: TORRUST_CHECKER_CONFIG=[config] {program} {}",
        args.join(" ")
    );
    tracing::info!("Tracker Checker config:\n{config_content}");

    let mut child = Command::new(program)
        .env("TORRUST_CHECKER_CONFIG", config_content)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // The child is owned by this guard: whatever happens below (success, error
    // or panic), it is killed and reaped before `run_command` returns.
    let guard = ChildGuard { child: &mut child };

    let stdout_reader = guard.child.stdout.take().map(spawn_reader);
    let stderr_reader = guard.child.stderr.take().map(spawn_reader);

    let wait_result = wait_with_deadline(guard.child, deadline);

    // The pipes are closed when the child exits (or is killed), so the readers end.
    let stdout = join_reader(stdout_reader);
    let stderr = join_reader(stderr_reader);

    log_output("stdout", &stdout);
    log_output("stderr", &stderr);

    let status = wait_result?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!("Failed to run Tracker Checker: {status}")))
    }
}

/// Kills and reaps the child on drop (a no-op when it already exited).
struct ChildGuard<'a> {
    child: &'a mut Child,
}

impl Drop for ChildGuard<'_> {
    fn drop(&mut self) {
        if matches!(self.child.try_wait(), Ok(None)) {
            drop(self.child.kill());
        }
        drop(self.child.wait());
    }
}

fn log_output(stream: &str, content: &str) {
    if !content.is_empty() {
        tracing::info!("Tracker Checker {stream}:\n{content}");
    }
}

fn spawn_reader<R: Read + Send + 'static>(mut pipe: R) -> JoinHandle<String> {
    thread::spawn(move || {
        let mut bytes = Vec::new();
        drop(pipe.read_to_end(&mut bytes));
        String::from_utf8_lossy(&bytes).into_owned()
    })
}

fn join_reader(reader: Option<JoinHandle<String>>) -> String {
    reader.and_then(|handle| handle.join().ok()).unwrap_or_default()
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
                format!("Tracker Checker did not finish within {deadline:?}"),
            ));
        }

        thread::sleep(POLL_INTERVAL);
    }
}

#[cfg(all(test, unix))]
mod tests {
    use std::time::Duration;

    use super::{CHECKER_ARGS, run_command};

    #[test]
    fn it_should_launch_the_unified_tracker_client_check_command() {
        assert_eq!(
            CHECKER_ARGS.join(" "),
            "run -p torrust-tracker-client --bin tracker_client -- check"
        );
    }

    #[test]
    fn it_should_pass_the_configuration_in_the_checker_config_env_var() {
        let result = run_command(
            "sh",
            &["-c", r#"test "$TORRUST_CHECKER_CONFIG" = '{"a":1}'"#],
            r#"{"a":1}"#,
            Duration::from_secs(10),
        );

        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn it_should_fail_when_the_child_exits_with_a_non_zero_status() {
        let result = run_command("sh", &["-c", "exit 1"], "{}", Duration::from_secs(10));

        assert!(result.is_err());
    }

    #[test]
    fn it_should_not_block_when_the_child_writes_more_than_a_pipe_buffer_to_both_streams() {
        let result = run_command(
            "sh",
            &[
                "-c",
                "head -c 300000 /dev/zero | tr '\\0' x; head -c 300000 /dev/zero | tr '\\0' y >&2",
            ],
            "{}",
            Duration::from_secs(30),
        );

        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn it_should_kill_the_child_and_fail_when_the_deadline_expires() {
        let result = run_command("sh", &["-c", "exec sleep 60"], "{}", Duration::from_millis(300));

        let error = result.expect_err("a hung child should time out");
        assert_eq!(error.kind(), std::io::ErrorKind::TimedOut);
    }

    #[test]
    fn it_should_fail_when_the_program_cannot_be_launched() {
        let result = run_command("/nonexistent/program", &[], "{}", Duration::from_secs(1));

        assert!(result.is_err());
    }
}
