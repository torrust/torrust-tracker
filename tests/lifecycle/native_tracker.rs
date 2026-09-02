//! Native child-process fixture for tracker executable lifecycle scenarios.
//!
//! It owns one isolated tracker workspace, drains the child's output while the
//! tracker runs, discovers the health endpoint from its startup log, and reaps
//! the child even when graceful shutdown exceeds the scenario deadline.

use std::net::SocketAddr;
use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncBufReadExt as _, AsyncRead, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::{Mutex, oneshot};
use tokio::task::JoinHandle;
use torrust_tracker_axum_health_check_api_server::resources::{Report, Status};

const STARTUP_DEADLINE: Duration = Duration::from_secs(10);
const SHUTDOWN_DEADLINE: Duration = Duration::from_secs(30);
const RETRY_INTERVAL: Duration = Duration::from_millis(50);
const HEALTH_CHECK_STARTUP_PREFIX: &str = "Started on: http://";
const HEALTH_CHECK_LOG_TARGET: &str = "HEALTH CHECK API";
const SIGNAL_HANDLERS_READY_MESSAGE: &str = "Tracker shutdown signal handlers installed.";

const CONFIGURATION: &str = r#"
[metadata]
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
path = "{STORAGE_PATH}/sqlite3.db"

[[http_trackers]]
bind_address = "127.0.0.1:0"
tracker_usage_statistics = false

[health_check_api]
bind_address = "127.0.0.1:0"
"#;

/// A running tracker executable isolated in a temporary workspace.
pub struct NativeTracker {
    child: Option<Child>,
    output: Arc<Mutex<String>>,
    output_readers: Vec<JoinHandle<()>>,
    _workspace: tempfile::TempDir,
    health_check_address: Option<SocketAddr>,
    drop_cleanup_complete: Option<oneshot::Sender<Result<i32, String>>>,
    drop_cleanup_observer: Option<oneshot::Receiver<Result<i32, String>>>,
}

impl NativeTracker {
    /// Spawns the Cargo-built tracker binary with an isolated port-zero configuration.
    pub fn start() -> Self {
        let workspace = tempfile::tempdir().expect("create temporary tracker workspace");
        let config_path = write_configuration(&workspace);
        let output = Arc::new(Mutex::new(String::new()));
        let mut command = Command::new(tracker_binary());
        command
            // Configure only this child process. `Command::env` does not
            // mutate the test process environment, so parallel fixtures each
            // retain their own temporary configuration path.
            .env("TORRUST_TRACKER_CONFIG_TOML_PATH", config_path)
            .env_remove("TORRUST_TRACKER_CONFIG_TOML")
            // `shutdown` reaps normal and expected-error paths. This kills a
            // panicking test's child so it cannot outlive its temporary workspace.
            .kill_on_drop(true)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = command.spawn().expect("spawn Cargo-built tracker executable");
        let stdout = child.stdout.take().expect("tracker child stdout is piped");
        let stderr = child.stderr.take().expect("tracker child stderr is piped");
        let (drop_cleanup_complete, drop_cleanup_observer) = oneshot::channel();

        Self {
            child: Some(child),
            output: Arc::clone(&output),
            output_readers: vec![
                tokio::spawn(drain_output(stdout, Arc::clone(&output))),
                tokio::spawn(drain_output(stderr, output)),
            ],
            _workspace: workspace,
            health_check_address: None,
            drop_cleanup_complete: Some(drop_cleanup_complete),
            drop_cleanup_observer: Some(drop_cleanup_observer),
        }
    }

    /// Waits until the tracker is healthy and its executable-boundary signal handlers are installed.
    pub async fn wait_until_ready(&mut self) -> Result<(), String> {
        let deadline = tokio::time::Instant::now() + STARTUP_DEADLINE;
        let client = reqwest::Client::new();

        loop {
            self.discover_health_check_address().await;

            if let Some(address) = self.health_check_address {
                let health_check_url = format!("http://{address}/health_check"); // DevSkim: ignore DS137138
                match client.get(health_check_url).send().await {
                    Ok(response) if response.status().is_success() => match response.json::<Report>().await {
                        Ok(report) if report.status == Status::Ok && self.signal_handlers_are_installed().await => {
                            return Ok(());
                        }
                        Ok(report) => {
                            if tokio::time::Instant::now() >= deadline {
                                return Err(self
                                    .failure_message(&format!(
                                        "health endpoint {address} reported {:?}: {}",
                                        report.status, report.message
                                    ))
                                    .await);
                            }
                        }
                        Err(error) if tokio::time::Instant::now() >= deadline => {
                            return Err(self
                                .failure_message(&format!("health endpoint {address} returned an invalid report: {error}"))
                                .await);
                        }
                        Err(_) => {}
                    },
                    Ok(response) if tokio::time::Instant::now() >= deadline => {
                        return Err(self
                            .failure_message(&format!("health endpoint {address} returned HTTP {}", response.status()))
                            .await);
                    }
                    Ok(_) | Err(_) => {}
                }
            }

            if let Some(status) = self
                .child_mut()
                .try_wait()
                .map_err(|error| Self::failure_message_sync(&format!("check tracker child status: {error}")))?
            {
                return Err(self
                    .failure_message(&format!("tracker exited before readiness with {status}"))
                    .await);
            }
            if tokio::time::Instant::now() >= deadline {
                return Err(self
                    .failure_message("timed out waiting for health-check startup log, Status::Ok, and installed signal handlers")
                    .await);
            }
            tokio::time::sleep(RETRY_INTERVAL).await;
        }
    }

    /// Returns the retained child's exact operating-system PID.
    pub fn pid(&self) -> Result<u32, String> {
        self.child_ref()
            .id()
            .ok_or_else(|| Self::failure_message_sync("tracker child exited before signal delivery"))
    }

    /// Waits for a graceful exit, force-killing and reaping only after its deadline.
    pub async fn shutdown(mut self) -> Result<String, String> {
        let mut child = self.child.take().expect("tracker child must be available before shutdown");
        let exit_result = match tokio::time::timeout(SHUTDOWN_DEADLINE, child.wait()).await {
            Ok(Ok(status)) => Ok(status),
            Ok(Err(error)) => Err(Self::failure_message_sync(&format!("wait for tracker child: {error}"))),
            Err(_) => {
                child
                    .start_kill()
                    .map_err(|error| Self::failure_message_sync(&format!("force-kill timed out tracker child: {error}")))?;
                let status = child
                    .wait()
                    .await
                    .map_err(|error| Self::failure_message_sync(&format!("reap force-killed tracker child: {error}")))?;
                Err(Self::failure_message_sync(&format!(
                    "tracker did not exit within {SHUTDOWN_DEADLINE:?}; force-killed with {status}"
                )))
            }
        };

        for reader in self.output_readers.drain(..) {
            reader.await.expect("output reader task must complete");
        }
        let output = self.output.lock().await.clone();
        exit_result.map(|_| output)
    }

    /// Returns an observer for the signal that terminated the reaped drop-path child.
    pub const fn take_drop_cleanup_observer(&mut self) -> oneshot::Receiver<Result<i32, String>> {
        self.drop_cleanup_observer
            .take()
            .expect("drop cleanup observer must be taken at most once")
    }

    async fn discover_health_check_address(&mut self) {
        if self.health_check_address.is_some() {
            return;
        }

        let output = self.output.lock().await;
        self.health_check_address = output.lines().find_map(parse_health_check_address);
    }

    async fn signal_handlers_are_installed(&self) -> bool {
        self.output.lock().await.contains(SIGNAL_HANDLERS_READY_MESSAGE)
    }

    async fn failure_message(&self, message: &str) -> String {
        format!("{message}\ntracker output:\n{}", self.output.lock().await)
    }

    fn failure_message_sync(message: &str) -> String {
        format!("{message}\ntracker output is being drained concurrently")
    }

    const fn child_ref(&self) -> &Child {
        self.child.as_ref().expect("tracker child must be available")
    }

    const fn child_mut(&mut self) -> &mut Child {
        self.child.as_mut().expect("tracker child must be available")
    }
}

impl Drop for NativeTracker {
    fn drop(&mut self) {
        let Some(mut child) = self.child.take() else {
            return;
        };
        let cleanup_complete = self.drop_cleanup_complete.take();

        // `shutdown` owns normal and expected-error teardown. On a panic,
        // kill and reap in the active runtime rather than leaving a zombie.
        drop(tokio::spawn(async move {
            let cleanup_result = match child.start_kill() {
                Ok(()) => match child.wait().await {
                    Ok(status) => status
                        .signal()
                        .ok_or_else(|| format!("dropped tracker child exited without a signal: {status}")),
                    Err(error) => Err(format!("reap force-killed tracker child: {error}")),
                },
                Err(error) => Err(format!("force-kill dropped tracker child: {error}")),
            };
            if let Some(cleanup_complete) = cleanup_complete {
                drop(cleanup_complete.send(cleanup_result));
            }
        }));
    }
}

async fn drain_output<R>(stream: R, output: Arc<Mutex<String>>)
where
    R: AsyncRead + Unpin,
{
    let mut lines = BufReader::new(stream).lines();
    while let Some(line) = lines.next_line().await.expect("read tracker child output") {
        let mut output = output.lock().await;
        output.push_str(&line);
        output.push('\n');
    }
}

fn parse_health_check_address(line: &str) -> Option<SocketAddr> {
    if !line.contains(HEALTH_CHECK_LOG_TARGET) {
        return None;
    }
    let address = line.split_once(HEALTH_CHECK_STARTUP_PREFIX)?.1;
    address.parse().ok()
}

fn write_configuration(workspace: &tempfile::TempDir) -> PathBuf {
    let storage_path = workspace.path().join("storage");
    std::fs::create_dir_all(&storage_path).expect("create tracker storage directory");
    let config_path = workspace.path().join("tracker.toml");
    let config = CONFIGURATION.replace("{STORAGE_PATH}", &storage_path.to_string_lossy());
    std::fs::write(&config_path, config).expect("write tracker configuration");
    config_path
}

fn tracker_binary() -> PathBuf {
    std::env::var_os("NEXTEST_BIN_EXE_torrust-tracker")
        .or_else(|| std::env::var_os("CARGO_BIN_EXE_torrust-tracker"))
        .map_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_torrust-tracker")), PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::parse_health_check_address;

    #[test]
    fn it_should_extract_the_assigned_health_check_address_from_its_startup_log() {
        // Arrange
        let line = "2026-09-02T10:20:22Z  INFO HEALTH CHECK API: Started on: http://127.0.0.1:43210";

        // Act
        let address = parse_health_check_address(line);

        // Assert
        assert_eq!(
            address.expect("health-check address should parse").to_string(),
            "127.0.0.1:43210"
        );
    }
}
