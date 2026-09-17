//! Materializes fixture workspaces and configuration, and constructs isolated child commands.
//!
//! This module does not own child processes or make running or failed-start lifecycle decisions.

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
pub(super) struct NativeTrackerWorkspace {
    _workspace: tempfile::TempDir,
    configuration_path: PathBuf,
    // The configuration binary does not inspect lifecycle fixture storage.
    #[allow(dead_code)]
    storage_path: PathBuf,
    environment_configuration_path: Option<PathBuf>,
    environment_configuration_toml: Option<String>,
    health_check_api_bind_address_override: Option<SocketAddr>,
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
    command
}

/// Applies the mandatory child isolation and output capture policy.
pub(super) fn configure_tracker_command(command: &mut Command) {
    command
        .env_remove("TORRUST_TRACKER_CONFIG_TOML")
        .env_remove("TORRUST_TRACKER_CONFIG_TOML_PATH")
        .env_remove("TORRUST_TRACKER_CONFIG_OVERRIDE_HEALTH_CHECK_API__BIND_ADDRESS")
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

pub(super) fn tracker_binary() -> PathBuf {
    std::env::var_os("NEXTEST_BIN_EXE_torrust-tracker")
        .or_else(|| std::env::var_os("CARGO_BIN_EXE_torrust-tracker"))
        .map_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_torrust-tracker")), PathBuf::from)
}

#[cfg(test)]
mod tests {
    use std::ffi::{OsStr, OsString};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::path::Path;

    use super::{tracker_command, write_configuration};

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
}
