//! Materializes fixture workspaces and configuration, and constructs isolated child commands.
//!
//! This module does not own child processes or make running or failed-start lifecycle decisions.

use std::ffi::OsString;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::process::Stdio;

use tokio::process::Command;

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

/// Shutdown budgets, in seconds, that a tracker child receives through its
/// `TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__*` environment variables.
///
/// Values are plain integers so scenarios can also pass values the tracker must reject.
#[allow(
    clippy::struct_field_names,
    reason = "the fields mirror the configuration keys, which share the `_in_secs` postfix"
)]
#[derive(Clone, Copy)]
pub struct NativeTrackerShutdownBudgets {
    pub deadline_in_secs: u32,
    pub http_drain_in_secs: u32,
    pub udp_request_drain_in_secs: u32,
    pub event_listener_drain_in_secs: u32,
}

/// Base configuration sources supplied to one tracker child.
///
/// The fixture writes all corresponding files into its temporary workspace.
/// Health-check ports and a child-only bind-address override are configurable
/// because executable configuration tests need no other child-process
/// configuration surface.
#[derive(Clone, Copy)]
pub struct NativeTrackerConfigurationSources {
    cli: u16,
    environment_path: Option<u16>,
    environment_toml: Option<u16>,
    health_check_api_bind_address_override: Option<SocketAddr>,
    shutdown_budgets: Option<NativeTrackerShutdownBudgets>,
}

impl NativeTrackerConfigurationSources {
    /// Creates sources whose CLI-selected configuration listens on `port`.
    pub const fn with_cli_health_check_port(port: u16) -> Self {
        Self {
            cli: port,
            environment_path: None,
            environment_toml: None,
            health_check_api_bind_address_override: None,
            shutdown_budgets: None,
        }
    }

    /// Adds child-only shutdown budgets; without them the tracker uses its defaults.
    // See the type-level allowance: only the shutdown-policy test binary uses it.
    #[allow(dead_code)]
    pub const fn with_shutdown_budgets(mut self, budgets: NativeTrackerShutdownBudgets) -> Self {
        self.shutdown_budgets = Some(budgets);
        self
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
pub(super) struct NativeTrackerWorkspace {
    _workspace: tempfile::TempDir,
    configuration_path: PathBuf,
    // The configuration binary does not inspect lifecycle fixture storage.
    #[allow(dead_code)]
    storage_path: PathBuf,
    environment_configuration_path: Option<PathBuf>,
    environment_configuration_toml: Option<String>,
    health_check_api_bind_address_override: Option<SocketAddr>,
    shutdown_budgets: Option<NativeTrackerShutdownBudgets>,
}

impl NativeTrackerWorkspace {
    // The configuration binary always supplies explicit source settings.
    #[allow(dead_code)]
    pub(super) fn new() -> Self {
        Self::with_configuration_sources(NativeTrackerConfigurationSources::with_cli_health_check_port(0))
    }

    pub(super) fn with_configuration_sources(sources: NativeTrackerConfigurationSources) -> Self {
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
            shutdown_budgets: sources.shutdown_budgets,
        }
    }

    pub(super) fn configuration_path(&self) -> &Path {
        &self.configuration_path
    }

    // The configuration binary does not inspect lifecycle fixture storage.
    #[allow(dead_code)]
    pub(super) fn storage_path(&self) -> &Path {
        &self.storage_path
    }

    pub(super) fn environment_configuration_path(&self) -> Option<&Path> {
        self.environment_configuration_path.as_deref()
    }

    pub(super) fn environment_configuration_toml(&self) -> Option<&str> {
        self.environment_configuration_toml.as_deref()
    }

    pub(super) const fn health_check_api_bind_address_override(&self) -> Option<SocketAddr> {
        self.health_check_api_bind_address_override
    }

    pub(super) const fn shutdown_budgets(&self) -> Option<NativeTrackerShutdownBudgets> {
        self.shutdown_budgets
    }
}

pub(super) fn write_configuration(workspace: &tempfile::TempDir, name: &str, health_check_port: u16) -> (PathBuf, PathBuf) {
    let storage_path = workspace.path().join(format!("{name}-storage"));
    let config_path = workspace.path().join(format!("{name}-tracker.toml"));
    write_configuration_at(&config_path, &storage_path, health_check_port);
    (config_path, storage_path)
}

/// Builds a child command whose base configuration is selected only by the CLI.
///
/// The two legacy base-source variables are explicitly removed so inherited
/// environment state cannot override or obscure a fixture's CLI-selected file.
pub(super) fn tracker_command(
    configuration_path: &Path,
    environment_configuration_path: Option<&Path>,
    environment_configuration_toml: Option<&str>,
    health_check_api_bind_address_override: Option<SocketAddr>,
    shutdown_budgets: Option<NativeTrackerShutdownBudgets>,
) -> Command {
    let mut command = Command::new(tracker_binary());
    configure_tracker_command(&mut command);
    command.arg("--config-toml-path").arg(configuration_path);
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
    if let Some(budgets) = shutdown_budgets {
        command
            .env(
                "TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__DEADLINE_IN_SECS",
                budgets.deadline_in_secs.to_string(),
            )
            .env(
                "TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__HTTP_DRAIN_IN_SECS",
                budgets.http_drain_in_secs.to_string(),
            )
            .env(
                "TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__UDP_REQUEST_DRAIN_IN_SECS",
                budgets.udp_request_drain_in_secs.to_string(),
            )
            .env(
                "TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__EVENT_LISTENER_DRAIN_IN_SECS",
                budgets.event_listener_drain_in_secs.to_string(),
            );
    }
    command
}

/// Applies the mandatory child isolation and output capture policy.
pub(super) fn configure_tracker_command(command: &mut Command) {
    command
        .env_remove("TORRUST_TRACKER_CONFIG_TOML")
        .env_remove("TORRUST_TRACKER_CONFIG_TOML_PATH")
        .env_remove("TORRUST_TRACKER_CONFIG_OVERRIDE_HEALTH_CHECK_API__BIND_ADDRESS")
        .env_remove("TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__DEADLINE_IN_SECS")
        .env_remove("TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__HTTP_DRAIN_IN_SECS")
        .env_remove("TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__UDP_REQUEST_DRAIN_IN_SECS")
        .env_remove("TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__EVENT_LISTENER_DRAIN_IN_SECS")
        // Normal fixture waits reap children. In a no-runtime drop path this
        // remains best-effort termination, not guaranteed asynchronous reaping.
        .kill_on_drop(true)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
}

pub(super) fn write_configuration_in_directory(directory: &Path, health_check_port: u16) -> (PathBuf, PathBuf) {
    let storage_path = directory.join("storage");
    let config_path = directory.join("tracker.toml");
    write_configuration_at(&config_path, &storage_path, health_check_port);
    (config_path, storage_path)
}

fn write_configuration_at(config_path: &Path, storage_path: &Path, health_check_port: u16) {
    std::fs::create_dir_all(storage_path).expect("create tracker storage directory");
    let config = CONFIGURATION
        .replace("{STORAGE_PATH}", &storage_path.to_string_lossy())
        .replace("{HEALTH_CHECK_PORT}", &health_check_port.to_string());
    std::fs::write(config_path, config).expect("write tracker configuration");
}

/// Environment variable naming the tracker executable to run instead of the one Cargo built.
///
/// The container's `test` stages set it to the separately built shipped binary, so the black-box
/// tests exercise the executable the image ships. It is read before the variables nextest and Cargo
/// set, which cannot carry an override.
const SHIPPED_BINARY_VARIABLE: &str = "TORRUST_TRACKER_BINARY";

pub(super) fn tracker_binary() -> PathBuf {
    let binary = resolve_tracker_binary(
        |name| std::env::var_os(name),
        PathBuf::from(env!("CARGO_BIN_EXE_torrust-tracker")),
    );

    anchored_at(binary, &std::env::current_dir().expect("read the test working directory"))
}

/// Makes a relative executable path that has a directory part independent of the child's working directory.
///
/// A child whose working directory is changed (`Command::current_dir`) looks a relative path up inside that
/// directory, so `./target/release/torrust-tracker` must be anchored at the test's working directory first. A bare
/// name has no directory part and keeps its `PATH` lookup.
fn anchored_at(binary: PathBuf, working_directory: &Path) -> PathBuf {
    if binary.is_relative() && binary.components().count() > 1 {
        working_directory.join(binary)
    } else {
        binary
    }
}

fn resolve_tracker_binary(variable: impl Fn(&str) -> Option<OsString>, compiled: PathBuf) -> PathBuf {
    variable(SHIPPED_BINARY_VARIABLE)
        .filter(|path| !path.is_empty())
        .or_else(|| variable("NEXTEST_BIN_EXE_torrust-tracker"))
        .or_else(|| variable("CARGO_BIN_EXE_torrust-tracker"))
        .map_or(compiled, PathBuf::from)
}

#[cfg(test)]
mod tests {
    use std::ffi::{OsStr, OsString};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::path::{Path, PathBuf};

    use super::{NativeTrackerShutdownBudgets, anchored_at, resolve_tracker_binary, tracker_command, write_configuration};

    fn lookup_in<'a>(variables: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<OsString> + 'a {
        move |name| {
            variables
                .iter()
                .find(|(variable, _)| *variable == name)
                .map(|(_, value)| OsString::from(*value))
        }
    }

    #[test]
    fn it_should_run_the_binary_named_by_the_shipped_binary_override_before_the_one_nextest_or_cargo_built() {
        // Arrange
        let variables = [
            ("TORRUST_TRACKER_BINARY", "/shipped/torrust-tracker"),
            ("NEXTEST_BIN_EXE_torrust-tracker", "/nextest/torrust-tracker"),
            ("CARGO_BIN_EXE_torrust-tracker", "/cargo/torrust-tracker"),
        ];

        // Act
        let binary = resolve_tracker_binary(lookup_in(&variables), PathBuf::from("/compiled/torrust-tracker"));

        // Assert
        assert_eq!(binary, PathBuf::from("/shipped/torrust-tracker"));
    }

    #[test]
    fn it_should_run_the_binary_nextest_built_when_no_override_is_set() {
        // Arrange
        let variables = [
            ("NEXTEST_BIN_EXE_torrust-tracker", "/nextest/torrust-tracker"),
            ("CARGO_BIN_EXE_torrust-tracker", "/cargo/torrust-tracker"),
        ];

        // Act
        let binary = resolve_tracker_binary(lookup_in(&variables), PathBuf::from("/compiled/torrust-tracker"));

        // Assert
        assert_eq!(binary, PathBuf::from("/nextest/torrust-tracker"));
    }

    #[test]
    fn it_should_anchor_a_relative_binary_path_at_the_test_working_directory() {
        // Arrange: a path with a directory part, such as `./target/release/torrust-tracker`; a child that changes
        // its working directory would otherwise look the executable up inside that directory.
        let binary = PathBuf::from("./target/release/torrust-tracker");

        // Act
        let anchored = anchored_at(binary, Path::new("/workspace"));

        // Assert
        assert_eq!(anchored, PathBuf::from("/workspace/./target/release/torrust-tracker"));
    }

    #[test]
    fn it_should_leave_an_absolute_binary_path_as_it_is() {
        // Arrange
        let binary = PathBuf::from("/shipped/torrust-tracker");

        // Act
        let anchored = anchored_at(binary, Path::new("/workspace"));

        // Assert
        assert_eq!(anchored, PathBuf::from("/shipped/torrust-tracker"));
    }

    #[test]
    fn it_should_leave_a_bare_binary_name_for_the_path_lookup() {
        // Arrange: no directory part, so the operating system finds the executable through `PATH`.
        let binary = PathBuf::from("torrust-tracker");

        // Act
        let anchored = anchored_at(binary, Path::new("/workspace"));

        // Assert
        assert_eq!(anchored, PathBuf::from("torrust-tracker"));
    }

    #[test]
    fn it_should_ignore_a_shipped_binary_override_that_is_empty() {
        // Arrange: the override is set to the empty string, as `TORRUST_TRACKER_BINARY= cargo test` does.
        let variables = [
            ("TORRUST_TRACKER_BINARY", ""),
            ("NEXTEST_BIN_EXE_torrust-tracker", "/nextest/torrust-tracker"),
        ];

        // Act
        let binary = resolve_tracker_binary(lookup_in(&variables), PathBuf::from("/compiled/torrust-tracker"));

        // Assert
        assert_eq!(binary, PathBuf::from("/nextest/torrust-tracker"));
    }

    #[test]
    fn it_should_run_the_binary_compiled_into_the_test_when_no_variable_is_set() {
        // Arrange
        let variables: [(&str, &str); 0] = [];

        // Act
        let binary = resolve_tracker_binary(lookup_in(&variables), PathBuf::from("/compiled/torrust-tracker"));

        // Assert
        assert_eq!(binary, PathBuf::from("/compiled/torrust-tracker"));
    }

    #[test]
    fn it_should_select_its_configuration_with_the_cli_and_remove_legacy_base_source_variables() {
        // Arrange
        let configuration_path = Path::new("/workspace/tracker.toml");

        // Act
        let command = tracker_command(configuration_path, None, None, None, None);
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
            "TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__DEADLINE_IN_SECS",
            "TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__HTTP_DRAIN_IN_SECS",
            "TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__UDP_REQUEST_DRAIN_IN_SECS",
            "TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__EVENT_LISTENER_DRAIN_IN_SECS",
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
        let command = tracker_command(configuration_path, None, None, Some(override_address), None);
        let environment = command.as_std().get_envs().collect::<Vec<_>>();

        // Assert
        assert!(environment.iter().any(|(name, value)| {
            *name == OsStr::new("TORRUST_TRACKER_CONFIG_OVERRIDE_HEALTH_CHECK_API__BIND_ADDRESS")
                && *value == Some(override_value.as_os_str())
        }));
    }

    #[test]
    fn it_should_set_one_child_only_override_per_shutdown_budget() {
        // Arrange
        let configuration_path = Path::new("/workspace/tracker.toml");
        let budgets = NativeTrackerShutdownBudgets {
            deadline_in_secs: 11,
            http_drain_in_secs: 12,
            udp_request_drain_in_secs: 13,
            event_listener_drain_in_secs: 14,
        };

        // Act
        let command = tracker_command(configuration_path, None, None, None, Some(budgets));
        let environment = command.as_std().get_envs().collect::<Vec<_>>();

        // Assert
        for (variable, expected) in [
            (
                "TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__DEADLINE_IN_SECS",
                "11",
            ),
            (
                "TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__HTTP_DRAIN_IN_SECS",
                "12",
            ),
            (
                "TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__UDP_REQUEST_DRAIN_IN_SECS",
                "13",
            ),
            (
                "TORRUST_TRACKER_CONFIG_OVERRIDE_APPLICATION__SHUTDOWN__EVENT_LISTENER_DRAIN_IN_SECS",
                "14",
            ),
        ] {
            assert!(
                environment
                    .iter()
                    .any(|(name, value)| *name == OsStr::new(variable) && *value == Some(OsStr::new(expected))),
                "command should set {variable} to {expected}"
            );
        }
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
}
