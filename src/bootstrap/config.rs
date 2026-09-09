//! Initialize configuration from file or env var.
//!
//! All environment variables are prefixed with `TORRUST_TRACKER_`.

use std::path::PathBuf;

use torrust_tracker_configuration::Info;
use torrust_tracker_configuration::v3_0_0::Configuration;

/// Errors while reading the tracker configuration source.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(
        "Could not prepare the tracker configuration source. Check `--config-toml-path`, `TORRUST_TRACKER_CONFIG_TOML_PATH`, or `TORRUST_TRACKER_CONFIG_TOML`: {source}"
    )]
    Source { source: torrust_tracker_configuration::Error },

    #[error("Could not load the tracker configuration. Fix the configured TOML source and try again: {source}")]
    Load { source: torrust_tracker_configuration::Error },
}

// skill-link: run-tracker-locally
pub const DEFAULT_PATH_CONFIG: &str = "./share/default/config/tracker.development.sqlite3.toml";

/// It loads the application configuration from the environment.
///
/// There are two methods to inject the configuration:
///
/// 1. By using a config file: `tracker.toml`.
/// 2. Environment variable: `TORRUST_TRACKER_CONFIG_TOML`. The variable contains the same contents as the `tracker.toml` file.
///
/// Environment variable has priority over the config file.
///
/// Refer to the [configuration documentation](https://docs.rs/torrust-tracker-configuration) for the configuration options.
///
/// # Errors
///
/// Returns source-preserving errors if the configuration source cannot be
/// prepared or parsed.
// issue: #2151
pub fn initialize_configuration(explicit_config_toml_path: Option<PathBuf>) -> Result<Configuration, Error> {
    let info = Info::new_with_explicit_config_toml_path(DEFAULT_PATH_CONFIG.to_string(), explicit_config_toml_path)
        .map_err(|source| Error::Source { source })?;
    Configuration::load(&info).map_err(|source| Error::Load { source })
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::net::SocketAddr;
    use std::sync::{LazyLock, Mutex};

    use torrust_tracker_configuration::Info;
    use torrust_tracker_configuration::v3_0_0::Configuration;

    use super::{Error, initialize_configuration};

    static ENVIRONMENT_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    const MANDATORY_CONFIGURATION: &str = r#"
        [metadata]
        schema_version = "3.0.0"

        [logging]
        trace_filter = "info"

        [core]
        listed = false
        private = false
    "#;

    fn configuration_with_health_check_port(port: u16) -> String {
        format!(
            r#"
                {MANDATORY_CONFIGURATION}

                [health_check_api]
                bind_address = "127.0.0.1:{port}"
            "#
        )
    }

    struct ConfigurationPathGuard {
        original_path: Option<std::ffi::OsString>,
        original_toml: Option<std::ffi::OsString>,
    }

    impl ConfigurationPathGuard {
        #[allow(unsafe_code)]
        fn replace(path: &std::path::Path) -> Self {
            let original_path = std::env::var_os(torrust_tracker_configuration::ENV_VAR_CONFIG_TOML_PATH);
            let original_toml = std::env::var_os("TORRUST_TRACKER_CONFIG_TOML");
            // SAFETY: `ENVIRONMENT_LOCK` serializes environment mutations in this test module.
            unsafe {
                std::env::remove_var("TORRUST_TRACKER_CONFIG_TOML");
                std::env::set_var(torrust_tracker_configuration::ENV_VAR_CONFIG_TOML_PATH, path);
            }

            Self {
                original_path,
                original_toml,
            }
        }

        #[allow(unsafe_code)]
        fn set_complete_toml(toml: String) {
            // SAFETY: `ENVIRONMENT_LOCK` serializes environment mutations in this test module.
            unsafe {
                std::env::set_var("TORRUST_TRACKER_CONFIG_TOML", toml);
            }
        }

        #[allow(unsafe_code)]
        fn remove_complete_toml() {
            // SAFETY: `ENVIRONMENT_LOCK` serializes environment mutations in this test module.
            unsafe {
                std::env::remove_var("TORRUST_TRACKER_CONFIG_TOML");
            }
        }

        #[allow(unsafe_code)]
        fn remove_path() {
            // SAFETY: `ENVIRONMENT_LOCK` serializes environment mutations in this test module.
            unsafe {
                std::env::remove_var(torrust_tracker_configuration::ENV_VAR_CONFIG_TOML_PATH);
            }
        }
    }

    impl Drop for ConfigurationPathGuard {
        #[allow(unsafe_code)]
        fn drop(&mut self) {
            // SAFETY: `ENVIRONMENT_LOCK` serializes environment mutations in this test module.
            unsafe {
                if let Some(path) = &self.original_path {
                    std::env::set_var(torrust_tracker_configuration::ENV_VAR_CONFIG_TOML_PATH, path);
                } else {
                    std::env::remove_var(torrust_tracker_configuration::ENV_VAR_CONFIG_TOML_PATH);
                }
                if let Some(toml) = &self.original_toml {
                    std::env::set_var("TORRUST_TRACKER_CONFIG_TOML", toml);
                } else {
                    std::env::remove_var("TORRUST_TRACKER_CONFIG_TOML");
                }
            }
        }
    }

    #[test]
    fn it_should_load_with_default_config() {
        // Arrange
        let _environment_lock = ENVIRONMENT_LOCK.lock().expect("lock environment access");

        // Act and assert
        initialize_configuration(None).expect("default configuration should load");
    }

    #[test]
    fn it_should_return_a_typed_load_error_when_the_configured_source_file_is_missing() {
        // Arrange
        let _environment_lock = ENVIRONMENT_LOCK.lock().expect("lock environment access");
        let missing_path = tempfile::tempdir()
            .expect("create temporary directory")
            .path()
            .join("missing-tracker-config.toml");
        let _path_guard = ConfigurationPathGuard::replace(&missing_path);

        // Act
        let result = initialize_configuration(None);

        // Assert
        assert!(matches!(result, Err(Error::Load { .. })));
    }

    #[test]
    fn it_should_name_the_cli_argument_when_an_explicit_configuration_source_cannot_be_prepared() {
        // Arrange
        let _environment_lock = ENVIRONMENT_LOCK.lock().expect("lock environment access");
        let missing_path = tempfile::tempdir()
            .expect("create temporary directory")
            .path()
            .join("missing-tracker-config.toml");

        // Act
        let error = initialize_configuration(Some(missing_path))
            .expect_err("missing explicit configuration source should fail before loading");

        // Assert
        assert!(error.to_string().contains("--config-toml-path"));
    }

    #[test]
    fn it_should_load_an_explicit_path_without_mutating_environment_sources() {
        // Arrange
        let _environment_lock = ENVIRONMENT_LOCK.lock().expect("lock environment access");
        let explicit_directory = tempfile::tempdir().expect("create explicit configuration directory");
        let explicit_path = explicit_directory.path().join("explicit.toml");
        fs::write(&explicit_path, configuration_with_health_check_port(42151)).expect("write explicit configuration");
        let original_toml = std::env::var_os("TORRUST_TRACKER_CONFIG_TOML");
        let original_path = std::env::var_os(torrust_tracker_configuration::ENV_VAR_CONFIG_TOML_PATH);

        // Act
        let configuration = initialize_configuration(Some(explicit_path)).expect("explicit configuration should load");

        // Assert
        assert_eq!(
            configuration.health_check_api.bind_address,
            "127.0.0.1:42151".parse::<SocketAddr>().unwrap()
        );
        assert_eq!(std::env::var_os("TORRUST_TRACKER_CONFIG_TOML"), original_toml);
        assert_eq!(
            std::env::var_os(torrust_tracker_configuration::ENV_VAR_CONFIG_TOML_PATH),
            original_path
        );
    }

    #[test]
    fn it_should_prefer_an_explicit_path_over_both_environment_base_sources() {
        // Arrange
        let _environment_lock = ENVIRONMENT_LOCK.lock().expect("lock environment access");
        let directory = tempfile::tempdir().expect("create configuration directory");
        let explicit_path = directory.path().join("explicit.toml");
        let environment_path = directory.path().join("environment.toml");
        fs::write(&explicit_path, configuration_with_health_check_port(42152)).expect("write explicit configuration");
        fs::write(&environment_path, configuration_with_health_check_port(42153)).expect("write environment path configuration");
        let _path_guard = ConfigurationPathGuard::replace(&environment_path);
        ConfigurationPathGuard::set_complete_toml(configuration_with_health_check_port(42154));

        // Act
        let configuration = initialize_configuration(Some(explicit_path)).expect("explicit configuration should load");

        // Assert
        assert_eq!(
            configuration.health_check_api.bind_address,
            "127.0.0.1:42152".parse::<SocketAddr>().unwrap()
        );
    }

    #[test]
    fn it_should_prefer_an_explicit_path_over_the_complete_toml_environment_source() {
        // Arrange
        let _environment_lock = ENVIRONMENT_LOCK.lock().expect("lock environment access");
        let directory = tempfile::tempdir().expect("create configuration directory");
        let explicit_path = directory.path().join("explicit.toml");
        fs::write(&explicit_path, configuration_with_health_check_port(42155)).expect("write explicit configuration");
        let _path_guard = ConfigurationPathGuard::replace(&directory.path().join("environment.toml"));
        ConfigurationPathGuard::remove_path();
        ConfigurationPathGuard::set_complete_toml(configuration_with_health_check_port(42156));

        // Act
        let configuration = initialize_configuration(Some(explicit_path)).expect("explicit configuration should load");

        // Assert
        assert_eq!(
            configuration.health_check_api.bind_address,
            "127.0.0.1:42155".parse::<SocketAddr>().unwrap()
        );
    }

    #[test]
    fn it_should_prefer_an_explicit_path_over_the_path_environment_source() {
        // Arrange
        let _environment_lock = ENVIRONMENT_LOCK.lock().expect("lock environment access");
        let directory = tempfile::tempdir().expect("create configuration directory");
        let explicit_path = directory.path().join("explicit.toml");
        let environment_path = directory.path().join("environment.toml");
        fs::write(&explicit_path, configuration_with_health_check_port(42157)).expect("write explicit configuration");
        fs::write(&environment_path, configuration_with_health_check_port(42158)).expect("write environment path configuration");
        let _path_guard = ConfigurationPathGuard::replace(&environment_path);
        ConfigurationPathGuard::remove_complete_toml();

        // Act
        let configuration = initialize_configuration(Some(explicit_path)).expect("explicit configuration should load");

        // Assert
        assert_eq!(
            configuration.health_check_api.bind_address,
            "127.0.0.1:42157".parse::<SocketAddr>().unwrap()
        );
    }

    #[test]
    fn it_should_load_every_shipped_configuration_template() {
        // Arrange
        let _environment_lock = ENVIRONMENT_LOCK.lock().expect("lock environment access");
        let templates = [
            "./share/default/config/tracker.container.mysql.toml",
            "./share/default/config/tracker.container.no-persistence.toml",
            "./share/default/config/tracker.container.postgresql.toml",
            "./share/default/config/tracker.container.sqlite3.toml",
            "./share/default/config/tracker.development.sqlite3.toml",
            "./share/default/config/tracker.e2e.container.sqlite3.toml",
            "./share/default/config/tracker.udp.benchmarking.toml",
        ];

        // Act and assert
        for template in templates {
            let info = Info::new(template.to_string()).expect("configuration source should be valid");

            Configuration::load(&info).unwrap_or_else(|error| panic!("template should load: {template}: {error}"));
        }
    }
}
