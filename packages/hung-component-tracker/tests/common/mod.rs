//! Owns one `hung-component-tracker` child process: its workspace, output, signal and reaping.
//!
//! It owns only incidental mechanics (configuration file, spawn, readiness wait, signal delivery,
//! bounded exit wait, kill and reap). It makes no assertion about the shutdown result: the tests do.

use std::io::{BufRead as _, BufReader, Read};
use std::path::Path;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use nix::sys::signal::{Signal, kill};
use nix::unistd::Pid;

/// The line the shared shutdown sequence logs once its signal handlers are installed.
const SIGNAL_HANDLERS_INSTALLED: &str = "Tracker shutdown signal handlers installed.";

/// The longest the child may take to start and install its signal handlers.
const STARTUP_TIMEOUT: Duration = Duration::from_secs(30);

/// How often the fixture checks the child's output and exit status.
const POLL_INTERVAL: Duration = Duration::from_millis(10);

/// Configuration variables a developer or CI may have exported; the child must not inherit them.
const CONFIGURATION_VARIABLE_PREFIX: &str = "TORRUST_TRACKER_CONFIG";

/// The `[application.shutdown]` budgets written into the child's configuration file, in seconds.
#[allow(
    clippy::struct_field_names,
    reason = "the fields mirror the configuration keys, which share the `_in_secs` postfix"
)]
#[derive(Clone, Copy, Debug)]
pub struct ShutdownBudgets {
    pub deadline_in_secs: u64,
    pub http_drain_in_secs: u64,
    pub udp_request_drain_in_secs: u64,
    pub event_listener_drain_in_secs: u64,
}

/// How the child ended, how long after the signal, and what it logged.
pub struct ChildExit {
    pub status: ExitStatus,
    pub elapsed_since_signal: Duration,
    /// The child's stdout and stderr, without the ANSI color escapes of the log formatter.
    pub output: String,
}

/// A running `hung-component-tracker` child in its own temporary workspace.
pub struct HungComponentTracker {
    child: Option<Child>,
    output: Arc<Mutex<String>>,
    readers: Vec<JoinHandle<()>>,
    _workspace: tempfile::TempDir,
}

impl HungComponentTracker {
    /// Writes a configuration with the given budgets and spawns the binary Cargo built for this crate.
    ///
    /// The configuration enables only the health check API, on an ephemeral port, and keeps the
    /// database in the temporary workspace.
    pub fn start(budgets: ShutdownBudgets) -> Self {
        let workspace = tempfile::tempdir().expect("create the temporary tracker workspace");
        let configuration_path = write_configuration(workspace.path(), budgets);

        let mut command = Command::new(env!("CARGO_BIN_EXE_hung-component-tracker"));
        for (name, _) in std::env::vars_os() {
            if name.to_string_lossy().starts_with(CONFIGURATION_VARIABLE_PREFIX) {
                command.env_remove(name);
            }
        }
        let mut child = command
            .arg("--config-toml-path")
            .arg(&configuration_path)
            .current_dir(workspace.path())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn the hung-component-tracker binary");

        let output = Arc::new(Mutex::new(String::new()));
        let readers = vec![
            spawn_reader(child.stdout.take().expect("child stdout is piped"), Arc::clone(&output)),
            spawn_reader(child.stderr.take().expect("child stderr is piped"), Arc::clone(&output)),
        ];

        Self {
            child: Some(child),
            output,
            readers,
            _workspace: workspace,
        }
    }

    /// Waits until the child logs that its shutdown signal handlers are installed.
    ///
    /// # Panics
    ///
    /// Panics with the child output when the child exits first or the startup timeout passes.
    pub fn wait_until_signal_handlers_installed(&mut self) {
        let deadline = Instant::now() + STARTUP_TIMEOUT;
        loop {
            if self.output_contains(SIGNAL_HANDLERS_INSTALLED) {
                return;
            }
            if let Some(status) = self.child_mut().try_wait().expect("check the child status") {
                panic!(
                    "the child exited with {status} before installing its signal handlers; output:\n{}",
                    self.output()
                );
            }
            assert!(
                Instant::now() < deadline,
                "the child did not install its signal handlers within {STARTUP_TIMEOUT:?}; output:\n{}",
                self.output()
            );
            std::thread::sleep(POLL_INTERVAL);
        }
    }

    /// Delivers `signal` to the child's exact PID and waits for it to exit, at most `timeout`.
    ///
    /// # Panics
    ///
    /// Panics with the child output when the child is still running after `timeout`; the child is
    /// killed and reaped first. Also panics when an output reader thread panicked, since the
    /// captured output is then incomplete.
    pub fn signal_and_wait_for_exit(mut self, signal: Signal, timeout: Duration) -> ChildExit {
        let pid = Pid::from_raw(i32::try_from(self.child_mut().id()).expect("the child PID fits an i32"));
        let signalled_at = Instant::now();
        kill(pid, signal).expect("deliver the signal to the child PID");

        let status = loop {
            if let Some(status) = self.child_mut().try_wait().expect("check the child status") {
                break status;
            }
            if signalled_at.elapsed() > timeout {
                self.kill_and_reap();
                self.join_readers();
                panic!(
                    "the child was still running {timeout:?} after {signal}; killed it; output:\n{}",
                    self.output()
                );
            }
            std::thread::sleep(POLL_INTERVAL);
        };
        let elapsed_since_signal = signalled_at.elapsed();
        self.child = None;
        self.join_readers();

        ChildExit {
            status,
            elapsed_since_signal,
            output: strip_ansi_escapes(&self.output()),
        }
    }

    fn output(&self) -> String {
        self.output.lock().expect("the output lock is not poisoned").clone()
    }

    /// Checks the captured output under its lock, without copying it on every poll.
    fn output_contains(&self, text: &str) -> bool {
        self.output.lock().expect("the output lock is not poisoned").contains(text)
    }

    const fn child_mut(&mut self) -> &mut Child {
        self.child.as_mut().expect("the child is running")
    }

    fn kill_and_reap(&mut self) {
        if let Some(mut child) = self.child.take() {
            // The child may have exited on its own between the last check and the kill.
            drop(child.kill());
            drop(child.wait());
        }
    }

    /// Joins the output reader threads and fails the test when one of them panicked, since the
    /// captured output would then be incomplete.
    fn join_readers(&mut self) {
        for reader in self.readers.drain(..) {
            assert!(
                reader.join().is_ok(),
                "an output reader thread panicked, so the captured child output is incomplete"
            );
        }
    }
}

impl Drop for HungComponentTracker {
    /// Kills and reaps a child still running when a test fails, so no process or port is left behind.
    fn drop(&mut self) {
        self.kill_and_reap();
        // This also runs while a failed assertion unwinds, so a reader panic must not panic again
        // here; the normal paths have already joined the readers with `join_readers`.
        for reader in self.readers.drain(..) {
            drop(reader.join());
        }
    }
}

fn write_configuration(workspace: &Path, budgets: ShutdownBudgets) -> std::path::PathBuf {
    let storage_path = workspace.join("storage");
    std::fs::create_dir_all(&storage_path).expect("create the tracker storage directory");
    let configuration = format!(
        r#"[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"

[core]
listed = false
private = false

[core.database]
driver = "sqlite3"
path = "{database_path}"

[health_check_api]
bind_address = "127.0.0.1:0"

[application.shutdown]
deadline_in_secs = {deadline}
http_drain_in_secs = {http_drain}
udp_request_drain_in_secs = {udp_request_drain}
event_listener_drain_in_secs = {event_listener_drain}
"#,
        database_path = storage_path.join("sqlite3.db").display(),
        deadline = budgets.deadline_in_secs,
        http_drain = budgets.http_drain_in_secs,
        udp_request_drain = budgets.udp_request_drain_in_secs,
        event_listener_drain = budgets.event_listener_drain_in_secs,
    );
    let configuration_path = workspace.join("tracker.toml");
    std::fs::write(&configuration_path, configuration).expect("write the tracker configuration");
    configuration_path
}

fn spawn_reader<R>(stream: R, output: Arc<Mutex<String>>) -> JoinHandle<()>
where
    R: Read + Send + 'static,
{
    std::thread::spawn(move || {
        for line in BufReader::new(stream).lines() {
            // A read error or a non-UTF-8 line panics, so `join_readers` reports the output as
            // incomplete instead of the test failing later on missing log lines.
            let line = line.expect("read a line of the child output");
            let mut output = output.lock().expect("the output lock is not poisoned");
            output.push_str(&line);
            output.push('\n');
        }
    })
}

/// Removes the ANSI color escapes the log formatter writes around field names, so assertions can
/// match `name=value`.
fn strip_ansi_escapes(text: &str) -> String {
    let mut plain = String::with_capacity(text.len());
    let mut characters = text.chars();
    while let Some(character) = characters.next() {
        if character == '\u{1b}' {
            // A color sequence is ESC, `[`, parameters, and a final letter such as `m`.
            for skipped in characters.by_ref() {
                if skipped.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            plain.push(character);
        }
    }
    plain
}
