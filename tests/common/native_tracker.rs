//! Native child-process fixture for tracker executable lifecycle scenarios.
//!
//! It owns one isolated tracker workspace, supplies its configuration through
//! the executable's CLI, drains the child's output while the tracker runs,
//! discovers the health endpoint from its startup log, and reaps the child
//! even when graceful shutdown exceeds the scenario deadline.

use std::net::SocketAddr;
use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use nix::sys::signal::{Signal, kill};
use nix::unistd::Pid;
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
bind_address = "127.0.0.1:{HEALTH_CHECK_PORT}"
"#;

/// A running tracker executable isolated in a temporary workspace.
pub struct NativeTracker {
    child: Option<Child>,
    output: Option<TrackerOutputCapture>,
    workspace: Option<NativeTrackerWorkspace>,
    health_check_client: Option<HealthCheckClient>,
    drop_cleanup_complete: Option<oneshot::Sender<Result<i32, String>>>,
    drop_cleanup_observer: Option<oneshot::Receiver<Result<i32, String>>>,
}

/// Base configuration sources supplied to one tracker child.
///
/// The fixture writes all corresponding files into its temporary workspace.
/// Health-check ports and a child-only bind-address override are configurable
/// because executable configuration tests need no other child-process
/// configuration surface.
#[derive(Clone, Copy)]
// This shared module is compiled by signal-only and configuration test binaries;
// the former does not use configuration-specific source builders.
#[allow(dead_code)]
pub struct NativeTrackerConfigurationSources {
    cli: u16,
    environment_path: Option<u16>,
    environment_toml: Option<u16>,
    health_check_api_bind_address_override: Option<SocketAddr>,
}

impl NativeTrackerConfigurationSources {
    /// Creates sources whose CLI-selected configuration listens on `port`.
    pub const fn with_cli_health_check_port(port: u16) -> Self {
        Self {
            cli: port,
            environment_path: None,
            environment_toml: None,
            health_check_api_bind_address_override: None,
        }
    }

    /// Adds a child-only environment path source with its own health-check port.
    // See the type-level allowance: signal-only test binaries do not use it.
    #[allow(dead_code)]
    pub const fn with_environment_path_health_check_port(mut self, port: u16) -> Self {
        self.environment_path = Some(port);
        self
    }

    /// Adds a child-only complete-TOML environment source with its own health-check port.
    // See the type-level allowance: signal-only test binaries do not use it.
    #[allow(dead_code)]
    pub const fn with_environment_toml_health_check_port(mut self, port: u16) -> Self {
        self.environment_toml = Some(port);
        self
    }

    /// Adds a child-only health-check API bind-address override.
    // See the type-level allowance: signal-only test binaries do not use it.
    #[allow(dead_code)]
    pub const fn with_health_check_api_bind_address_override(mut self, address: SocketAddr) -> Self {
        self.health_check_api_bind_address_override = Some(address);
        self
    }
}

/// An isolated workspace and configuration for one tracker child process.
struct NativeTrackerWorkspace {
    _workspace: tempfile::TempDir,
    configuration_path: PathBuf,
    storage_path: PathBuf,
    environment_configuration_path: Option<PathBuf>,
    environment_configuration_toml: Option<String>,
    health_check_api_bind_address_override: Option<SocketAddr>,
}

impl NativeTrackerWorkspace {
    fn new() -> Self {
        Self::with_configuration_sources(NativeTrackerConfigurationSources::with_cli_health_check_port(0))
    }

    fn with_configuration_sources(sources: NativeTrackerConfigurationSources) -> Self {
        let workspace = tempfile::tempdir().expect("create temporary tracker workspace");
        let (configuration_path, storage_path) = write_configuration(&workspace, "cli", sources.cli);
        let environment_configuration_path = sources
            .environment_path
            .map(|port| write_configuration(&workspace, "environment-path", port).0);
        let environment_configuration_toml = sources.environment_toml.map(|port| {
            let (configuration_path, _) = write_configuration(&workspace, "environment-toml", port);
            std::fs::read_to_string(configuration_path).expect("read environment TOML configuration")
        });

        Self {
            _workspace: workspace,
            configuration_path,
            storage_path,
            environment_configuration_path,
            environment_configuration_toml,
            health_check_api_bind_address_override: sources.health_check_api_bind_address_override,
        }
    }

    fn configuration_path(&self) -> &std::path::Path {
        &self.configuration_path
    }

    fn storage_path(&self) -> &std::path::Path {
        &self.storage_path
    }

    fn environment_configuration_path(&self) -> Option<&std::path::Path> {
        self.environment_configuration_path.as_deref()
    }

    fn environment_configuration_toml(&self) -> Option<&str> {
        self.environment_configuration_toml.as_deref()
    }

    const fn health_check_api_bind_address_override(&self) -> Option<SocketAddr> {
        self.health_check_api_bind_address_override
    }
}

/// Concurrently drains and retains a tracker child's output for readiness and diagnostics.
struct TrackerOutputCapture {
    output: Arc<Mutex<String>>,
    readers: Vec<JoinHandle<()>>,
}

impl TrackerOutputCapture {
    fn new<R, S>(stdout: R, stderr: S) -> Self
    where
        R: AsyncRead + Unpin + Send + 'static,
        S: AsyncRead + Unpin + Send + 'static,
    {
        let output = Arc::new(Mutex::new(String::new()));

        Self {
            readers: vec![
                tokio::spawn(drain_output(stdout, Arc::clone(&output))),
                tokio::spawn(drain_output(stderr, Arc::clone(&output))),
            ],
            output,
        }
    }

    async fn wait_for_readers(&mut self) {
        for reader in self.readers.drain(..) {
            reader.await.expect("output reader task must complete");
        }
    }

    async fn contents(&self) -> String {
        self.output.lock().await.clone()
    }
}

/// A deadline-bounded client for the tracker health-check endpoint.
struct HealthCheckClient {
    address: SocketAddr,
    client: reqwest::Client,
}

impl HealthCheckClient {
    fn new(address: SocketAddr) -> Self {
        Self {
            address,
            client: reqwest::Client::new(),
        }
    }

    async fn probe(&self, deadline: tokio::time::Instant) -> Result<HealthCheckProbe, HealthCheckProbeError> {
        let health_check_url = format!("http://{}/health_check", self.address); // DevSkim: ignore DS137138
        let response = match tokio::time::timeout_at(deadline, self.client.get(health_check_url).send()).await {
            Ok(Ok(response)) => response,
            Ok(Err(_)) => return Ok(HealthCheckProbe::Unavailable),
            Err(_) => return Err(HealthCheckProbeError::TimedOut),
        };

        if !response.status().is_success() {
            return Err(HealthCheckProbeError::UnexpectedHttpStatus(response.status()));
        }

        let report = match tokio::time::timeout_at(deadline, response.json::<Report>()).await {
            Ok(Ok(report)) => report,
            Ok(Err(error)) => return Err(HealthCheckProbeError::InvalidReport(error.to_string())),
            Err(_) => return Err(HealthCheckProbeError::TimedOut),
        };

        Ok(HealthCheckProbe::Report(report))
    }
}

enum HealthCheckProbe {
    Unavailable,
    Report(Report),
}

enum HealthCheckProbeError {
    TimedOut,
    UnexpectedHttpStatus(reqwest::StatusCode),
    InvalidReport(String),
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

fn write_configuration(workspace: &tempfile::TempDir, name: &str, health_check_port: u16) -> (PathBuf, PathBuf) {
    let storage_path = workspace.path().join(format!("{name}-storage"));
    std::fs::create_dir_all(&storage_path).expect("create tracker storage directory");
    let config_path = workspace.path().join(format!("{name}-tracker.toml"));
    let config = CONFIGURATION
        .replace("{STORAGE_PATH}", &storage_path.to_string_lossy())
        .replace("{HEALTH_CHECK_PORT}", &health_check_port.to_string());
    std::fs::write(&config_path, config).expect("write tracker configuration");
    (config_path, storage_path)
}

/// Builds a child command whose base configuration is selected only by the CLI.
///
/// The two legacy base-source variables are explicitly removed so inherited
/// environment state cannot override or obscure a fixture's CLI-selected file.
fn tracker_command(
    configuration_path: &std::path::Path,
    environment_configuration_path: Option<&std::path::Path>,
    environment_configuration_toml: Option<&str>,
    health_check_api_bind_address_override: Option<SocketAddr>,
) -> Command {
    let mut command = Command::new(tracker_binary());
    command
        .arg("--config-toml-path")
        .arg(configuration_path)
        .env_remove("TORRUST_TRACKER_CONFIG_TOML")
        .env_remove("TORRUST_TRACKER_CONFIG_TOML_PATH")
        .env_remove("TORRUST_TRACKER_CONFIG_OVERRIDE_HEALTH_CHECK_API__BIND_ADDRESS")
        // `shutdown` reaps normal and expected-error paths. This kills a
        // panicking test's child so it cannot outlive its temporary workspace.
        .kill_on_drop(true)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(path) = environment_configuration_path {
        command.env("TORRUST_TRACKER_CONFIG_TOML_PATH", path);
    }
    if let Some(toml) = environment_configuration_toml {
        command.env("TORRUST_TRACKER_CONFIG_TOML", toml);
    }
    if let Some(address) = health_check_api_bind_address_override {
        command.env(
            "TORRUST_TRACKER_CONFIG_OVERRIDE_HEALTH_CHECK_API__BIND_ADDRESS",
            address.to_string(),
        );
    }
    command
}

fn tracker_binary() -> PathBuf {
    std::env::var_os("NEXTEST_BIN_EXE_torrust-tracker")
        .or_else(|| std::env::var_os("CARGO_BIN_EXE_torrust-tracker"))
        .map_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_torrust-tracker")), PathBuf::from)
}

#[cfg(test)]
mod tests {
    use std::ffi::{OsStr, OsString};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::path::Path;

    use super::{parse_health_check_address, tracker_command, write_configuration};

    #[test]
    fn it_should_select_its_configuration_with_the_cli_and_remove_legacy_base_source_variables() {
        // Arrange
        let configuration_path = Path::new("/workspace/tracker.toml");

        // Act
        let command = tracker_command(configuration_path, None, None, None);
        let arguments = command.as_std().get_args().collect::<Vec<_>>();
        let environment = command.as_std().get_envs().collect::<Vec<_>>();

        // Assert
        assert_eq!(
            arguments,
            vec![OsStr::new("--config-toml-path"), configuration_path.as_os_str()]
        );
        for variable in [
            "TORRUST_TRACKER_CONFIG_TOML",
            "TORRUST_TRACKER_CONFIG_TOML_PATH",
            "TORRUST_TRACKER_CONFIG_OVERRIDE_HEALTH_CHECK_API__BIND_ADDRESS",
        ] {
            assert!(
                environment
                    .iter()
                    .any(|(name, value)| *name == OsStr::new(variable) && value.is_none()),
                "command should remove inherited {variable}"
            );
        }
    }

    #[test]
    fn it_should_set_the_child_only_health_check_bind_address_override_after_removing_the_inherited_value() {
        // Arrange
        let configuration_path = Path::new("/workspace/tracker.toml");
        let override_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 43156);
        let override_value = OsString::from(override_address.to_string());

        // Act
        let command = tracker_command(configuration_path, None, None, Some(override_address));
        let environment = command.as_std().get_envs().collect::<Vec<_>>();

        // Assert
        assert!(environment.iter().any(|(name, value)| {
            *name == OsStr::new("TORRUST_TRACKER_CONFIG_OVERRIDE_HEALTH_CHECK_API__BIND_ADDRESS")
                && *value == Some(override_value.as_os_str())
        }));
    }

    #[test]
    fn it_should_write_a_port_zero_configuration_with_workspace_local_sqlite_storage() {
        // Arrange
        let workspace = tempfile::tempdir().expect("create temporary tracker workspace");

        // Act
        let (config_path, storage_path) = write_configuration(&workspace, "test", 0);
        let configuration = std::fs::read_to_string(&config_path).expect("read tracker configuration");

        // Assert
        assert!(
            config_path.starts_with(workspace.path()),
            "configuration path should be workspace-local"
        );
        assert!(storage_path.is_dir(), "tracker storage directory should be created");
        assert!(
            configuration.contains(&format!("path = \"{}/sqlite3.db\"", storage_path.to_string_lossy())),
            "configuration should use workspace-local SQLite storage"
        );
        assert!(
            configuration.contains("bind_address = \"127.0.0.1:0\""),
            "configuration should use port-zero listener bindings"
        );
        assert_eq!(
            configuration.matches("bind_address = \"127.0.0.1:0\"").count(),
            2,
            "HTTP tracker and health-check API should both use port-zero bindings"
        );
    }

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

    #[test]
    fn it_should_reject_non_health_check_startup_logs() {
        // Arrange
        let lines = [
            "2026-09-02T10:20:22Z  INFO HTTP TRACKER: Started on: http://127.0.0.1:43210",
            "2026-09-02T10:20:22Z  INFO HEALTH CHECK API: Listening on: http://127.0.0.1:43210",
            "2026-09-02T10:20:22Z  INFO HEALTH CHECK API: Started on: http://not-an-address", // DevSkim: ignore DS137138
        ];

        // Act and Assert
        for line in lines {
            assert_eq!(
                parse_health_check_address(line),
                None,
                "line should not provide a health-check address: {line}"
            );
        }
    }
}
