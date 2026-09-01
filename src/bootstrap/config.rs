//! Initialize configuration from file or env var.
//!
//! All environment variables are prefixed with `TORRUST_TRACKER_`.

use torrust_tracker_configuration::Info;
use torrust_tracker_configuration::v3_0_0::Configuration;

/// Errors while reading the tracker configuration source.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(
        "Could not prepare the tracker configuration source. Check `TORRUST_TRACKER_CONFIG_TOML_PATH` or `TORRUST_TRACKER_CONFIG_TOML`: {source}"
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
pub fn initialize_configuration() -> Result<Configuration, Error> {
    let info = Info::new(DEFAULT_PATH_CONFIG.to_string()).map_err(|source| Error::Source { source })?;
    Configuration::load(&info).map_err(|source| Error::Load { source })
}

#[cfg(test)]
mod tests {
    use std::sync::{LazyLock, Mutex};

    use torrust_tracker_configuration::Info;
    use torrust_tracker_configuration::v3_0_0::Configuration;

    use super::{Error, initialize_configuration};

    static ENVIRONMENT_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

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
        initialize_configuration().expect("default configuration should load");
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
        let result = initialize_configuration();

        // Assert
        assert!(matches!(result, Err(Error::Load { .. })));
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
