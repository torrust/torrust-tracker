//! Native child-process fixture for tracker executable lifecycle scenarios.
//!
//! It owns one isolated tracker workspace, supplies its configuration through
//! the executable's CLI, drains the child's output while the tracker runs,
//! discovers the health endpoint from its startup log, and reaps the child
//! even when graceful shutdown exceeds the scenario deadline.

mod command;
mod health;
mod output;

use std::io::Write as _;
use std::net::SocketAddr;
use std::os::unix::fs::PermissionsExt as _;
use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::time::Duration;

use nix::sys::signal::{Signal, kill};
use nix::unistd::Pid;
use tokio::process::{Child, Command};
use tokio::sync::oneshot;
use torrust_tracker_axum_health_check_api_server::resources::Status;

pub use self::command::NativeTrackerConfigurationSources;
use self::command::{
    NativeTrackerWorkspace, configure_tracker_command, tracker_binary, tracker_command, write_configuration,
    write_configuration_in_directory,
};
use self::health::{HealthCheckClient, HealthCheckProbe, HealthCheckProbeError, parse_health_check_address};
use self::output::TrackerOutputCapture;

const STARTUP_DEADLINE: Duration = Duration::from_secs(10);
const SHUTDOWN_DEADLINE: Duration = Duration::from_secs(30);
// The signal-only test binary imports this shared fixture but has no failed-start scenarios.
#[allow(dead_code)]
const FAILURE_DEADLINE: Duration = Duration::from_secs(10);
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

/// Invalid CLI configuration sources supported by the failed-start fixture.
///
/// This deliberately exposes configuration cases rather than raw commands so
/// executable tests cannot bypass the fixture's environment and cleanup rules.
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum NativeTrackerInvalidCliSource {
    /// Omits the value following `--config-toml-path`.
    MissingOptionValue,
    /// Supplies an empty value for `--config-toml-path`.
    EmptyOptionValue,
    /// Supplies an absolute path that does not exist.
    MissingFile,
    /// Supplies a directory where a configuration file is required.
    Directory,
    /// Supplies an otherwise valid configuration file with malformed TOML appended.
    MalformedToml,
    /// Supplies `tracker.toml` from a child directory while it exists only in its parent.
    ParentOnlyRelativeFile,
}

/// Result of preparing an unreadable regular-file CLI source.
///
/// Permission bits are not enforced for privileged users or processes with
/// filesystem capabilities. The fixture probes that platform behavior before
/// spawning so the executable contract is only asserted when meaningful.
#[allow(dead_code)]
pub enum NativeTrackerUnreadableCliSource {
    /// The operating system denied a read, making the prepared startup attempt meaningful.
    Enforced(Box<NativeTrackerStartAttempt>),
    /// The current process can read mode-`000` files, so no startup attempt was prepared.
    NotEnforced { reason: String },
}

#[allow(dead_code)]
impl NativeTrackerUnreadableCliSource {
    /// Returns the prepared startup attempt when the platform enforces the unreadable mode.
    ///
    /// When it does not, the skip reason is reported on stderr and `None` is
    /// returned so the caller can end the test as an explicit skip.
    pub fn enforced_or_report_skip(self) -> Option<NativeTrackerStartAttempt> {
        match self {
            Self::Enforced(failed_start) => Some(*failed_start),
            Self::NotEnforced { reason } => {
                drop(writeln!(
                    std::io::stderr(),
                    "skipping unreadable regular-file assertion: {reason}"
                ));
                None
            }
        }
    }
}

/// A prepared tracker process startup that has not yet launched the executable.
#[allow(dead_code)]
pub struct NativeTrackerStartAttempt {
    command: Command,
    permission_restore: Option<NativeTrackerPermissionRestore>,
    workspace: Option<tempfile::TempDir>,
    source_path: Option<PathBuf>,
}

#[allow(dead_code)]
impl NativeTrackerStartAttempt {
    /// Prepares a tracker startup with a deliberately invalid CLI configuration source.
    pub fn with_invalid_cli_source(source: NativeTrackerInvalidCliSource) -> Self {
        let workspace = tempfile::tempdir().expect("create temporary invalid-source workspace");
        let (command, source_path) = invalid_source_command(&workspace, source);

        Self {
            command,
            permission_restore: None,
            workspace: Some(workspace),
            source_path,
        }
    }

    /// Prepares a valid regular configuration file that is unreadable by normal Unix permission checks.
    pub fn with_unreadable_regular_file() -> NativeTrackerUnreadableCliSource {
        let workspace = tempfile::tempdir().expect("create temporary unreadable-source workspace");
        let (source_path, _) = write_configuration(&workspace, "unreadable", 0);
        let original_mode = std::fs::metadata(&source_path)
            .expect("read fixture-owned configuration file metadata")
            .permissions()
            .mode();
        std::fs::set_permissions(&source_path, std::fs::Permissions::from_mode(0o000))
            .expect("make fixture-owned configuration file unreadable");
        let permission_restore = NativeTrackerPermissionRestore {
            path: source_path.clone(),
            mode: Some(original_mode),
        };

        match std::fs::read_to_string(&source_path) {
            Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
                NativeTrackerUnreadableCliSource::Enforced(Box::new(Self {
                    command: tracker_command(&source_path, None, None, None),
                    permission_restore: Some(permission_restore),
                    workspace: Some(workspace),
                    source_path: Some(source_path),
                }))
            }
            Ok(_) => NativeTrackerUnreadableCliSource::NotEnforced {
                reason: "the current process can read a mode-000 regular file (for example, it is privileged or has a filesystem capability)"
                    .to_owned(),
            },
            Err(error) => panic!("probe fixture-owned unreadable configuration file: {error}"),
        }
    }

    /// Launches the prepared tracker executable.
    pub fn start(mut self) -> NativeTrackerFailedStart {
        let mut child = self.command.spawn().expect("spawn Cargo-built tracker executable");
        let stdout = child.stdout.take().expect("tracker child stdout is piped");
        let stderr = child.stderr.take().expect("tracker child stderr is piped");

        NativeTrackerFailedStart {
            child: Some(child),
            output: Some(TrackerOutputCapture::new(stdout, stderr)),
            permission_restore: self.permission_restore.take(),
            workspace: self.workspace.take(),
            source_path: self.source_path.take(),
        }
    }
}

/// A tracker child process expected to fail before completing startup.
#[allow(dead_code)]
pub struct NativeTrackerFailedStart {
    child: Option<Child>,
    output: Option<TrackerOutputCapture>,
    // This must be dropped before the fixture workspace, so its file remains
    // present if explicit restoration was not possible.
    permission_restore: Option<NativeTrackerPermissionRestore>,
    workspace: Option<tempfile::TempDir>,
    source_path: Option<PathBuf>,
}

/// Restores the original Unix mode of a fixture-owned configuration file.
struct NativeTrackerPermissionRestore {
    path: PathBuf,
    mode: Option<u32>,
}

impl NativeTrackerPermissionRestore {
    fn restore(&mut self) -> std::io::Result<()> {
        let Some(mode) = self.mode else {
            return Ok(());
        };
        std::fs::set_permissions(&self.path, std::fs::Permissions::from_mode(mode))?;
        self.mode = None;
        Ok(())
    }
}

impl Drop for NativeTrackerPermissionRestore {
    fn drop(&mut self) {
        // Drop can run while another panic is unwinding and must never replace
        // that panic. A normal wait reports this error with child diagnostics.
        drop(self.restore());
    }
}

/// Stable evidence retained after a failed-start child has been reaped.
#[allow(dead_code)]
pub struct NativeTrackerFailedStartResult {
    exit_code: i32,
    output: String,
    source_path: Option<PathBuf>,
    source_mode: Option<u32>,
    _workspace: Option<tempfile::TempDir>,
}

/// Exit code `clap` uses when the command line itself is invalid.
#[allow(dead_code)]
const USAGE_ERROR_EXIT_CODE: i32 = 2;
/// Exit code the tracker uses when startup fails after argument parsing.
#[allow(dead_code)]
const STARTUP_FAILURE_EXIT_CODE: i32 = 1;

#[allow(dead_code)]
impl NativeTrackerFailedStartResult {
    /// Returns the process exit code captured after the child was reaped.
    pub const fn exit_code(&self) -> i32 {
        self.exit_code
    }

    /// Returns the combined stdout and stderr captured while the child ran.
    pub fn output(&self) -> &str {
        &self.output
    }

    /// Returns the fixture-owned source path while this result is retained.
    pub fn source_path(&self) -> Option<&std::path::Path> {
        self.source_path.as_deref()
    }

    /// Returns the original source permissions when the fixture changed them.
    pub const fn source_mode(&self) -> Option<u32> {
        self.source_mode
    }

    /// Asserts the child was rejected by argument parsing with the given usage diagnostic.
    pub fn assert_usage_error(&self, expected_diagnostic: &str) {
        self.assert_exit_code(USAGE_ERROR_EXIT_CODE);
        self.assert_output_contains(expected_diagnostic);
    }

    /// Asserts the child failed startup with the given diagnostic.
    pub fn assert_startup_failure(&self, expected_diagnostic: &str) {
        self.assert_exit_code(STARTUP_FAILURE_EXIT_CODE);
        self.assert_output_contains(expected_diagnostic);
    }

    /// Asserts the diagnostic names the fixture-owned source path.
    pub fn assert_diagnostic_names_source_path(&self) {
        let path = self
            .source_path()
            .expect("this failure source has no fixture-owned source path")
            .to_string_lossy()
            .into_owned();
        self.assert_output_contains(&path);
    }

    /// Asserts explicit configuration loading failed and identifies the supplied file.
    pub fn assert_explicit_configuration_file_load_failure(&self) {
        self.assert_startup_failure("Unable to load explicit configuration file");
        self.assert_diagnostic_names_source_path();
    }

    fn assert_exit_code(&self, expected: i32) {
        assert_eq!(
            self.exit_code, expected,
            "unexpected exit code\ntracker output:\n{}",
            self.output
        );
    }

    fn assert_output_contains(&self, expected_fragment: &str) {
        assert!(
            self.output.contains(expected_fragment),
            "tracker output does not contain {expected_fragment:?}\ntracker output:\n{}",
            self.output
        );
    }
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

#[allow(dead_code)]
impl NativeTrackerFailedStart {
    /// Returns the fixture-owned source path when the case has one.
    pub fn source_path(&self) -> Option<PathBuf> {
        self.source_path.clone()
    }

    /// Waits for the child process to exit and reaps it in the normal path.
    ///
    /// The initial wait, forced reaping, and output-reader completion are all
    /// deadline-bounded. On a timeout, this method force-kills and attempts to
    /// reap the child before returning diagnostics. `Drop` is only a best-effort
    /// fallback when an active Tokio runtime exists; it cannot guarantee async
    /// reaping.
    pub async fn wait_for_exit(mut self) -> Result<NativeTrackerFailedStartResult, String> {
        let mut child = self.child.take().expect("invalid-source tracker child must be available");
        let mut output_capture = self.output.take().expect("invalid-source output capture must be available");
        let status = match tokio::time::timeout(FAILURE_DEADLINE, child.wait()).await {
            Ok(Ok(status)) => Ok(status),
            Ok(Err(error)) => Err(format!("wait for invalid-source tracker child: {error}")),
            Err(_) => Self::kill_and_reap_after_timeout(&mut child).await,
        };
        let reader_result = Self::wait_for_output_readers(&mut output_capture).await;
        let output = output_capture.contents().await;
        let source_mode = self.permission_restore.as_ref().and_then(|restore| restore.mode);
        let restore_result = self
            .permission_restore
            .as_mut()
            .map_or(Ok(()), NativeTrackerPermissionRestore::restore)
            .map_err(|error| format!("restore fixture-owned configuration file permissions: {error}"));
        restore_result.map_err(|message| format!("{message}\ntracker output:\n{output}"))?;
        let status = status.map_err(|message| format!("{message}\ntracker output:\n{output}"))?;
        reader_result.map_err(|message| format!("{message}\ntracker output:\n{output}"))?;
        let exit_code = status
            .code()
            .ok_or_else(|| format!("invalid-source tracker child exited without a code: {status}\ntracker output:\n{output}"))?;

        Ok(NativeTrackerFailedStartResult {
            exit_code,
            output,
            source_path: self.source_path.take(),
            source_mode,
            _workspace: self.workspace.take(),
        })
    }

    async fn kill_and_reap_after_timeout(child: &mut Child) -> Result<std::process::ExitStatus, String> {
        let kill_result = child.start_kill();

        match tokio::time::timeout(FAILURE_DEADLINE, child.wait()).await {
            Ok(Ok(status)) => match kill_result {
                Ok(()) => Err(format!(
                    "invalid-source tracker child did not exit within {FAILURE_DEADLINE:?}; force-killed and reaped with {status}"
                )),
                Err(error) => Err(format!(
                    "invalid-source tracker child did not exit within {FAILURE_DEADLINE:?}; force-kill failed: {error}; reaped with {status}"
                )),
            },
            Ok(Err(error)) => Err(format!("reap force-killed invalid-source tracker child: {error}")),
            Err(_) => match kill_result {
                Ok(()) => Err(format!(
                    "invalid-source tracker child did not exit within {FAILURE_DEADLINE:?}, and did not reap within an additional {FAILURE_DEADLINE:?} after force-kill"
                )),
                Err(error) => Err(format!(
                    "invalid-source tracker child did not exit within {FAILURE_DEADLINE:?}; force-kill failed: {error}; and it did not reap within an additional {FAILURE_DEADLINE:?}"
                )),
            },
        }
    }

    async fn wait_for_output_readers(output_capture: &mut TrackerOutputCapture) -> Result<(), String> {
        tokio::time::timeout(FAILURE_DEADLINE, output_capture.wait_for_readers())
            .await
            .map_err(|_| format!("timed out waiting {FAILURE_DEADLINE:?} for invalid-source tracker output readers"))
    }
}

#[allow(dead_code)]
impl Drop for NativeTrackerFailedStart {
    fn drop(&mut self) {
        let Some(mut child) = self.child.take() else {
            return;
        };
        let output = self.output.take();
        // Restore synchronously before a runtime cleanup task can release the
        // workspace. Drop must not panic while unwinding, so this is best effort.
        if let Some(mut permission_restore) = self.permission_restore.take() {
            drop(permission_restore.restore());
        }

        // Without a runtime, synchronously kill and reap before field drop can
        // release the fixture workspace.
        let Ok(runtime) = tokio::runtime::Handle::try_current() else {
            drop(child.start_kill());
            let deadline = std::time::Instant::now() + FAILURE_DEADLINE;
            while std::time::Instant::now() < deadline {
                match child.try_wait() {
                    Ok(Some(_)) | Err(_) => break,
                    Ok(None) => std::thread::sleep(RETRY_INTERVAL),
                }
            }
            drop(output);
            return;
        };
        let workspace = self.workspace.take();
        drop(runtime.spawn(async move {
            drop(child.start_kill());
            drop(tokio::time::timeout(FAILURE_DEADLINE, child.wait()).await);
            if let Some(mut output) = output {
                drop(Self::wait_for_output_readers(&mut output).await);
            }
            drop(workspace);
        }));
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

#[allow(dead_code)]
fn invalid_source_command(workspace: &tempfile::TempDir, source: NativeTrackerInvalidCliSource) -> (Command, Option<PathBuf>) {
    let mut command = Command::new(tracker_binary());
    configure_tracker_command(&mut command);

    match source {
        NativeTrackerInvalidCliSource::MissingOptionValue => {
            command.arg("--config-toml-path");
            (command, None)
        }
        NativeTrackerInvalidCliSource::EmptyOptionValue => {
            command.arg("--config-toml-path").arg("");
            (command, None)
        }
        NativeTrackerInvalidCliSource::MissingFile => {
            let path = workspace.path().join("does-not-exist.toml");
            command.arg("--config-toml-path").arg(&path);
            (command, Some(path))
        }
        NativeTrackerInvalidCliSource::Directory => {
            let path = workspace.path().to_path_buf();
            command.arg("--config-toml-path").arg(&path);
            (command, Some(path))
        }
        NativeTrackerInvalidCliSource::MalformedToml => {
            let (path, _) = write_configuration(workspace, "malformed", 0);
            std::fs::write(
                &path,
                format!(
                    "{}\nmalformed_key = [",
                    std::fs::read_to_string(&path).expect("read configuration")
                ),
            )
            .expect("write malformed tracker configuration");
            command.arg("--config-toml-path").arg(&path);
            (command, Some(path))
        }
        NativeTrackerInvalidCliSource::ParentOnlyRelativeFile => {
            let parent = workspace.path().join("parent");
            let child = parent.join("child");
            std::fs::create_dir_all(&child).expect("create child working directory");
            let (parent_configuration, _) = write_configuration_in_directory(&parent, 0);
            assert_eq!(parent_configuration.file_name(), Some(std::ffi::OsStr::new("tracker.toml")));
            command.current_dir(child).arg("--config-toml-path").arg("tracker.toml");
            (command, Some(parent_configuration))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::os::unix::fs::PermissionsExt as _;
    use std::process::Stdio;

    use nix::sys::signal::kill;
    use nix::unistd::Pid;
    use tokio::process::Command;

    use super::{
        NativeTrackerFailedStart, NativeTrackerInvalidCliSource, NativeTrackerPermissionRestore, NativeTrackerStartAttempt,
        TrackerOutputCapture, invalid_source_command,
    };

    #[test]
    fn it_should_restore_permissions_when_the_restore_guard_is_dropped_without_a_tokio_runtime() {
        // Arrange
        let workspace = tempfile::tempdir().expect("create temporary permission-restore workspace");
        let path = workspace.path().join("configuration.toml");
        std::fs::write(&path, "configuration").expect("write fixture-owned configuration file");
        let original_mode = std::fs::metadata(&path)
            .expect("read fixture-owned configuration file metadata")
            .permissions()
            .mode();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o000))
            .expect("make fixture-owned configuration file unreadable");
        let restore = NativeTrackerPermissionRestore {
            path: path.clone(),
            mode: Some(original_mode),
        };

        // Act
        let thread_result = std::thread::spawn(move || drop(restore)).join();

        // Assert
        assert!(
            thread_result.is_ok(),
            "dropping the restoration guard outside Tokio must not panic"
        );
        assert_eq!(
            std::fs::metadata(path)
                .expect("read restored fixture-owned configuration file metadata")
                .permissions()
                .mode(),
            original_mode
        );
    }

    #[test]
    fn it_should_construct_a_parent_only_relative_source_from_the_child_working_directory() {
        // Arrange
        let workspace = tempfile::tempdir().expect("create temporary tracker workspace");

        // Act
        let (command, source_path) = invalid_source_command(&workspace, NativeTrackerInvalidCliSource::ParentOnlyRelativeFile);

        // Assert
        let source_path = source_path.expect("parent-only source should retain its parent file path");
        assert_eq!(source_path.file_name(), Some(OsStr::new("tracker.toml")));
        assert_eq!(
            source_path.parent().map(|parent| parent.join("child")).as_deref(),
            command.as_std().get_current_dir()
        );
        assert_eq!(command.as_std().get_args().last(), Some(OsStr::new("tracker.toml")));
    }

    #[tokio::test]
    async fn it_should_reap_a_failed_start_dropped_without_a_tokio_runtime() {
        // Arrange: spawning needs a runtime; the drop below happens outside one.
        let mut child = Command::new("sleep")
            .arg("60")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn long-running tracker substitute");
        let pid = Pid::from_raw(child.id().expect("child should have a PID").cast_signed());
        let stdout = child.stdout.take().expect("child stdout is piped");
        let stderr = child.stderr.take().expect("child stderr is piped");
        let failed_start = NativeTrackerFailedStart {
            child: Some(child),
            output: Some(TrackerOutputCapture::new(stdout, stderr)),
            permission_restore: None,
            workspace: Some(tempfile::tempdir().expect("create fixture workspace")),
            source_path: None,
        };

        // Act
        let result = std::thread::spawn(move || drop(failed_start)).join();

        // Assert
        assert!(result.is_ok(), "dropping a failed start outside Tokio must not panic");
        assert_eq!(kill(pid, None), Err(nix::errno::Errno::ESRCH));
    }

    #[tokio::test]
    async fn it_should_restore_the_unreadable_source_permissions_after_waiting_for_exit() {
        // Arrange
        let Some(start_attempt) = NativeTrackerStartAttempt::with_unreadable_regular_file().enforced_or_report_skip() else {
            return;
        };

        // Act
        let failure = start_attempt
            .start()
            .wait_for_exit()
            .await
            .expect("tracker should exit for an unreadable regular file");

        // Assert
        let restored_mode = std::fs::metadata(failure.source_path().expect("unreadable-file result should retain its path"))
            .expect("read restored unreadable-file metadata")
            .permissions()
            .mode();
        let original_mode = failure
            .source_mode()
            .expect("unreadable-file result should retain the original source mode");
        assert_eq!(restored_mode & 0o777, original_mode & 0o777);
    }
}
