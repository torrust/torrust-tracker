//! Owns the running tracker child from startup through readiness, shutdown, or drop cleanup.
//!
//! The root keeps one absolute startup deadline and reaps its child before releasing the
//! collaborator-owned workspace. It must not implement failed starts, configuration rendering,
//! output capture, or health probing.

mod command;
// The lifecycle-signals binary has no failed-start scenarios.
#[allow(dead_code)]
pub mod failed_start;
mod health;
mod output;

// The lifecycle-signals binary does not configure alternative sources.
#[allow(unused_imports)]
pub mod configuration {
    //! Exposes only the running fixture surface needed by executable configuration scenarios.

    pub use super::command::NativeTrackerConfigurationSources;
}

use std::net::SocketAddr;
use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::time::Duration;

use nix::sys::signal::{Signal, kill};
use nix::unistd::Pid;
use tokio::process::Child;
use tokio::sync::oneshot;
use torrust_tracker_axum_health_check_api_server::resources::Status;

use self::command::{NativeTrackerConfigurationSources, NativeTrackerWorkspace, tracker_command};
use self::health::{HealthCheckClient, HealthCheckProbe, HealthCheckProbeError, parse_health_check_address};
use self::output::TrackerOutputCapture;

const STARTUP_DEADLINE: Duration = Duration::from_secs(10);
const SHUTDOWN_DEADLINE: Duration = Duration::from_secs(30);
const RETRY_INTERVAL: Duration = Duration::from_millis(50);
const SIGNAL_HANDLERS_READY_MESSAGE: &str = "Tracker shutdown signal handlers installed.";

/// A running tracker executable isolated in a temporary workspace.
pub struct NativeTracker {
    child: Option<Child>,
    output: Option<TrackerOutputCapture>,
    workspace: Option<NativeTrackerWorkspace>,
    health_check_client: Option<HealthCheckClient>,
    drop_cleanup_complete: Option<oneshot::Sender<Result<i32, String>>>,
    drop_cleanup_observer: Option<oneshot::Receiver<Result<i32, String>>>,
}

impl NativeTracker {
    /// Spawns the Cargo-built tracker binary with an isolated CLI configuration and port-zero bindings.
    pub fn start() -> Self {
        let workspace = NativeTrackerWorkspace::new();
        Self::start_in_workspace(workspace)
    }

    /// Spawns a tracker child with fixture-owned CLI and optional environment base sources.
    // This shared module is also compiled by signal-only test binaries.
    #[allow(dead_code)]
    pub fn start_with_configuration_sources(sources: NativeTrackerConfigurationSources) -> Self {
        let workspace = NativeTrackerWorkspace::with_configuration_sources(sources);
        Self::start_in_workspace(workspace)
    }

    fn start_in_workspace(workspace: NativeTrackerWorkspace) -> Self {
        let mut command = tracker_command(
            workspace.configuration_path(),
            workspace.environment_configuration_path(),
            workspace.environment_configuration_toml(),
            workspace.health_check_api_bind_address_override(),
        );

        let mut child = command.spawn().expect("spawn Cargo-built tracker executable");
        let stdout = child.stdout.take().expect("tracker child stdout is piped");
        let stderr = child.stderr.take().expect("tracker child stderr is piped");
        let output = TrackerOutputCapture::new(stdout, stderr);
        let (drop_cleanup_complete, drop_cleanup_observer) = oneshot::channel();

        Self {
            child: Some(child),
            output: Some(output),
            workspace: Some(workspace),
            health_check_client: None,
            drop_cleanup_complete: Some(drop_cleanup_complete),
            drop_cleanup_observer: Some(drop_cleanup_observer),
        }
    }

    /// Waits until the tracker is healthy and its executable-boundary signal handlers are installed.
    pub async fn wait_until_ready(&mut self) -> Result<(), String> {
        let deadline = tokio::time::Instant::now() + STARTUP_DEADLINE;

        loop {
            if self.readiness_is_satisfied(deadline).await? {
                return Ok(());
            }
            self.fail_if_child_exited().await?;
            if tokio::time::Instant::now() >= deadline {
                return Err(self.startup_timeout_failure().await);
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

    /// Returns the health-check address discovered while waiting for readiness.
    pub fn health_check_address(&self) -> Result<SocketAddr, String> {
        self.health_check_client
            .as_ref()
            .map(|client| client.address)
            .ok_or_else(|| Self::failure_message_sync("tracker health-check address is unavailable before readiness"))
    }

    /// Returns the CLI-selected configuration path owned by this fixture.
    pub fn configuration_path(&self) -> Result<PathBuf, String> {
        self.workspace
            .as_ref()
            .map(|workspace| workspace.configuration_path().to_path_buf())
            .ok_or_else(|| Self::failure_message_sync("tracker workspace is unavailable after shutdown"))
    }

    /// Returns the isolated storage path owned by this fixture.
    pub fn storage_path(&self) -> Result<PathBuf, String> {
        self.workspace
            .as_ref()
            .map(|workspace| workspace.storage_path().to_path_buf())
            .ok_or_else(|| Self::failure_message_sync("tracker workspace is unavailable after shutdown"))
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

        let mut output_capture = self
            .output
            .take()
            .expect("tracker output capture must be available before shutdown");
        output_capture.wait_for_readers().await;
        let output = output_capture.contents().await;
        exit_result
            .map(|_| output.clone())
            .map_err(|message| format!("{message}\ntracker output:\n{output}"))
    }

    /// Delivers SIGTERM to the child and waits for its graceful shutdown.
    // Configuration tests use this convenience; signal tests verify delivery directly.
    #[allow(dead_code)]
    pub async fn gracefully_shutdown(self) -> Result<String, String> {
        let pid = self.pid()?;
        kill(
            Pid::from_raw(
                i32::try_from(pid).map_err(|error| Self::failure_message_sync(&format!("convert tracker PID: {error}")))?,
            ),
            Signal::SIGTERM,
        )
        .map_err(|error| Self::failure_message_sync(&format!("deliver SIGTERM to tracker child: {error}")))?;
        self.shutdown().await
    }

    /// Returns an observer for the signal that terminated the reaped drop-path child.
    pub const fn take_drop_cleanup_observer(&mut self) -> oneshot::Receiver<Result<i32, String>> {
        self.drop_cleanup_observer
            .take()
            .expect("drop cleanup observer must be taken at most once")
    }

    fn failure_message_sync(message: &str) -> String {
        format!("{message}\ntracker output is being drained concurrently")
    }

    async fn discover_health_check_client(&mut self) {
        if self.health_check_client.is_none() {
            self.health_check_client = self
                .output_ref()
                .contents()
                .await
                .lines()
                .find_map(parse_health_check_address)
                .map(HealthCheckClient::new);
        }
    }

    async fn signal_handlers_are_installed(&self) -> bool {
        self.output_ref().contents().await.contains(SIGNAL_HANDLERS_READY_MESSAGE)
    }

    async fn readiness_is_satisfied(&mut self, deadline: tokio::time::Instant) -> Result<bool, String> {
        self.discover_health_check_client().await;

        match &self.health_check_client {
            Some(client) => match client.probe(deadline).await {
                Ok(HealthCheckProbe::Unavailable) => Ok(false),
                Ok(HealthCheckProbe::Report(report)) if report.status == Status::Ok => {
                    Ok(self.signal_handlers_are_installed().await)
                }
                Ok(HealthCheckProbe::Report(report)) => {
                    self.fail_if_startup_deadline_reached(
                        deadline,
                        &format!(
                            "health endpoint {} reported {:?}: {}",
                            client.address, report.status, report.message
                        ),
                    )
                    .await
                }
                Err(HealthCheckProbeError::TimedOut) => Err(self.startup_timeout_failure().await),
                Err(HealthCheckProbeError::UnexpectedHttpStatus(status)) => {
                    self.fail_if_startup_deadline_reached(
                        deadline,
                        &format!("health endpoint {} returned HTTP {status}", client.address),
                    )
                    .await
                }
                Err(HealthCheckProbeError::InvalidReport(error)) => {
                    self.fail_if_startup_deadline_reached(
                        deadline,
                        &format!("health endpoint {} returned an invalid report: {error}", client.address),
                    )
                    .await
                }
            },
            None => Ok(false),
        }
    }

    async fn fail_if_startup_deadline_reached(&self, deadline: tokio::time::Instant, message: &str) -> Result<bool, String> {
        if tokio::time::Instant::now() >= deadline {
            Err(self.failure_message(message).await)
        } else {
            Ok(false)
        }
    }

    async fn fail_if_child_exited(&mut self) -> Result<(), String> {
        let status = self
            .child_mut()
            .try_wait()
            .map_err(|error| Self::failure_message_sync(&format!("check tracker child status: {error}")))?;

        match status {
            Some(status) => Err(self
                .failure_message(&format!("tracker exited before readiness with {status}"))
                .await),
            None => Ok(()),
        }
    }

    async fn startup_timeout_failure(&self) -> String {
        self.failure_message("timed out waiting for health-check startup log, Status::Ok, and installed signal handlers")
            .await
    }

    async fn failure_message(&self, message: &str) -> String {
        format!("{message}\ntracker output:\n{}", self.output_ref().contents().await)
    }

    const fn child_ref(&self) -> &Child {
        self.child.as_ref().expect("tracker child must be available")
    }

    const fn child_mut(&mut self) -> &mut Child {
        self.child.as_mut().expect("tracker child must be available")
    }

    const fn output_ref(&self) -> &TrackerOutputCapture {
        self.output.as_ref().expect("tracker output capture must be available")
    }
}

impl Drop for NativeTracker {
    fn drop(&mut self) {
        let Some(mut child) = self.child.take() else {
            return;
        };
        let workspace = self.workspace.take();
        let output = self.output.take();
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
            let output = if let Some(mut output) = output {
                output.wait_for_readers().await;
                output.contents().await
            } else {
                String::new()
            };
            drop(workspace);
            if let Some(cleanup_complete) = cleanup_complete {
                drop(cleanup_complete.send(cleanup_result.map_err(|message| format!("{message}\ntracker output:\n{output}"))));
            }
        }));
    }
}
