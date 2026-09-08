//! Configuration data structures for [Torrust Tracker](https://docs.rs/torrust-tracker).
//!
//! This module contains the configuration data structures for the
//! Torrust Tracker, which is a `BitTorrent` tracker server.
//!
//! The current schema version is [`v3_0_0`].
//! The previous version [`v2_0_0`] is kept for backward compatibility.
//! Consumers must import schema types through an explicit versioned module.
pub mod v2_0_0;
pub mod v3_0_0;
pub mod validator;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use camino::Utf8PathBuf;
use derive_more::Display;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use thiserror::Error;
use torrust_located_error::{DynError, LocatedError};
use tracing::info;

// Environment variables

/// Complete TOML base source from the environment.
///
/// It has priority over [`ENV_VAR_CONFIG_TOML_PATH`] when no explicit file path
/// is supplied by the caller.
const ENV_VAR_CONFIG_TOML: &str = "TORRUST_TRACKER_CONFIG_TOML";

/// Legacy environment-selected TOML file location.
pub const ENV_VAR_CONFIG_TOML_PATH: &str = "TORRUST_TRACKER_CONFIG_TOML_PATH";

/// Named configuration API tokens, protected from accidental diagnostic exposure.
pub type AccessTokens = HashMap<String, SecretString>;

/// The most recent supported configuration schema version.
pub const LATEST_VERSION: &str = "3.0.0";

/// Info about the configuration specification.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Display, Clone)]
#[display("Metadata(app: {app}, purpose: {purpose}, schema_version: {schema_version})")]
pub struct Metadata {
    /// The application this configuration is valid for.
    #[serde(default = "Metadata::default_app")]
    app: App,

    /// The purpose of this parsed file.
    #[serde(default = "Metadata::default_purpose")]
    purpose: Purpose,

    /// The schema version for the configuration.
    #[serde(default = "Metadata::default_schema_version")]
    #[serde(flatten)]
    schema_version: Version,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            app: Self::default_app(),
            purpose: Self::default_purpose(),
            schema_version: Self::default_schema_version(),
        }
    }
}

impl Metadata {
    /// Creates a `Metadata` with a specific schema version, keeping other fields at their defaults.
    #[must_use]
    pub const fn with_schema_version(schema_version: Version) -> Self {
        Self {
            app: Self::default_app(),
            purpose: Self::default_purpose(),
            schema_version,
        }
    }

    const fn default_app() -> App {
        App::TorrustTracker
    }

    const fn default_purpose() -> Purpose {
        Purpose::Configuration
    }

    fn default_schema_version() -> Version {
        Version::latest()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Display, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum App {
    TorrustTracker,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Display, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Purpose {
    Configuration,
}

/// The configuration version.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Display, Clone)]
#[serde(rename_all = "lowercase")]
pub struct Version {
    #[serde(default = "Version::default_semver")]
    schema_version: String,
}

impl Default for Version {
    fn default() -> Self {
        Self {
            schema_version: Self::default_semver(),
        }
    }
}

impl Version {
    pub(crate) fn new(semver: &str) -> Self {
        Self {
            schema_version: semver.to_owned(),
        }
    }

    fn latest() -> Self {
        Self {
            schema_version: LATEST_VERSION.to_string(),
        }
    }

    fn default_semver() -> String {
        LATEST_VERSION.to_string()
    }
}

/// Information required for loading config
#[derive(Debug, Default, Clone)]
pub struct Info {
    config_toml: Option<String>,
    config_toml_path: String,
    explicit_config_toml_path: Option<PathBuf>,
}

/// The base source from which to load configuration data.
pub(crate) enum ConfigTomlSource<'a> {
    Inline(&'a str),
    Explicit { path: &'a Path, contents: &'a str },
    File(&'a str),
}

impl Info {
    /// Build Configuration Info
    ///
    /// # Errors
    ///
    /// Will return `Err` if unable to obtain a configuration.
    ///
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(default_config_toml_path: String) -> Result<Self, Error> {
        Self::new_with_explicit_config_toml_path(default_config_toml_path, None)
    }

    /// Builds configuration information from an optional explicit configuration-file path.
    ///
    /// An explicit path takes priority over all environment base sources. Its contents are read
    /// eagerly so the path is resolved exactly from the current working directory and no Figment
    /// parent-directory lookup occurs.
    ///
    /// # Errors
    ///
    /// Returns [`Error::UnableToLoadExplicitConfigFile`] if the explicit path cannot be read as a
    /// regular file.
    #[allow(clippy::needless_pass_by_value)]
    pub fn new_with_explicit_config_toml_path(
        default_config_toml_path: String,
        explicit_config_toml_path: Option<PathBuf>,
    ) -> Result<Self, Error> {
        match explicit_config_toml_path {
            Some(path) => Self::from_explicit_file(default_config_toml_path, path),
            None => Ok(Self::from_environment(default_config_toml_path)),
        }
    }

    fn from_explicit_file(default_config_toml_path: String, path: PathBuf) -> Result<Self, Error> {
        let config_toml = Self::read_explicit_config_toml_file(&path)?;
        info!(path = ?path, "Loading extra configuration from explicit configuration file");

        Ok(Self {
            config_toml: Some(config_toml),
            config_toml_path: default_config_toml_path,
            explicit_config_toml_path: Some(path),
        })
    }

    fn from_environment(default_config_toml_path: String) -> Self {
        let env_var_config_toml = ENV_VAR_CONFIG_TOML.to_string();
        let env_var_config_toml_path = ENV_VAR_CONFIG_TOML_PATH.to_string();

        let config_toml = env::var(env_var_config_toml).map_or(None, |config_toml| {
            info!(
                config_toml_bytes = config_toml.len(),
                "Loading extra configuration from environment variable"
            );
            Some(config_toml)
        });

        let config_toml_path = env::var(env_var_config_toml_path).map_or_else(
            |_| {
                info!("Loading extra configuration from default configuration file: `{default_config_toml_path}` ...");
                default_config_toml_path
            },
            |config_toml_path| {
                info!("Loading extra configuration from file: `{config_toml_path}` ...");
                config_toml_path
            },
        );

        // The path is irrelevant when inline configuration is selected. Keeping it empty also
        // distinguishes this legacy environment source from an explicit file source.
        match config_toml {
            None => Self {
                config_toml: None,
                config_toml_path,
                explicit_config_toml_path: None,
            },
            Some(config_toml) => Self {
                config_toml: Some(config_toml),
                config_toml_path,
                explicit_config_toml_path: None,
            },
        }
    }

    pub(crate) fn config_toml_source(&self) -> ConfigTomlSource<'_> {
        match (&self.explicit_config_toml_path, &self.config_toml) {
            (Some(path), Some(contents)) => ConfigTomlSource::Explicit { path, contents },
            (None, Some(config_toml)) => ConfigTomlSource::Inline(config_toml),
            (_, None) => ConfigTomlSource::File(&self.config_toml_path),
        }
    }

    pub(crate) fn attach_explicit_config_path(&self, source: Error) -> Error {
        match self.config_toml_source() {
            ConfigTomlSource::Explicit { path, .. } => Error::UnableToProcessExplicitConfigFile {
                path: path.to_path_buf(),
                source: (Arc::new(source) as DynError).into(),
            },
            ConfigTomlSource::Inline(_) | ConfigTomlSource::File(_) => source,
        }
    }

    fn read_explicit_config_toml_file(path: &PathBuf) -> Result<String, Error> {
        let metadata = fs::metadata(path).map_err(|source| Error::UnableToLoadExplicitConfigFile {
            path: path.clone(),
            source,
        })?;

        if !metadata.is_file() {
            return Err(Error::UnableToLoadExplicitConfigFile {
                path: path.clone(),
                source: io::Error::new(io::ErrorKind::InvalidInput, "path is not a regular file"),
            });
        }

        fs::read_to_string(path).map_err(|source| Error::UnableToLoadExplicitConfigFile {
            path: path.clone(),
            source,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::net::SocketAddr;
    use std::path::PathBuf;

    use figment::Jail;

    use super::{ENV_VAR_CONFIG_TOML, ENV_VAR_CONFIG_TOML_PATH, Error, Info};
    use crate::v3_0_0::Configuration;

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

    fn load_configuration(default_path: &str) -> Result<Configuration, Error> {
        let info = Info::new(default_path.to_owned())?;

        Configuration::load(&info)
    }

    fn load_configuration_with_explicit_path(path: PathBuf) -> Result<Configuration, Error> {
        let info = Info::new_with_explicit_config_toml_path("default.toml".to_owned(), Some(path))?;

        Configuration::load(&info)
    }

    fn health_check_address(port: u16) -> SocketAddr {
        format!("127.0.0.1:{port}")
            .parse()
            .expect("test health-check address should parse")
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn it_should_select_complete_toml_when_complete_toml_and_path_environment_sources_are_set() {
        Jail::expect_with(|jail| {
            // Arrange
            jail.clear_env();
            let path_configuration = configuration_with_health_check_port(41001);
            jail.create_file("path.toml", &path_configuration)?;
            jail.set_env(ENV_VAR_CONFIG_TOML, configuration_with_health_check_port(41002));
            jail.set_env(ENV_VAR_CONFIG_TOML_PATH, "path.toml");

            // Act
            let configuration = load_configuration("default.toml").expect("complete TOML source should load");

            // Assert
            assert_eq!(configuration.health_check_api.bind_address, health_check_address(41002));

            Ok(())
        });
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn it_should_select_complete_toml_when_only_complete_toml_environment_source_is_set() {
        Jail::expect_with(|jail| {
            // Arrange
            jail.clear_env();
            jail.set_env(ENV_VAR_CONFIG_TOML, configuration_with_health_check_port(41003));

            // Act
            let configuration = load_configuration("default.toml").expect("complete TOML source should load");

            // Assert
            assert_eq!(configuration.health_check_api.bind_address, health_check_address(41003));

            Ok(())
        });
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn it_should_select_the_path_file_when_only_path_environment_source_is_set() {
        Jail::expect_with(|jail| {
            // Arrange
            jail.clear_env();
            let path_configuration = configuration_with_health_check_port(41004);
            jail.create_file("path.toml", &path_configuration)?;
            jail.set_env(ENV_VAR_CONFIG_TOML_PATH, "path.toml");

            // Act
            let configuration = load_configuration("default.toml").expect("path environment source should load");

            // Assert
            assert_eq!(configuration.health_check_api.bind_address, health_check_address(41004));

            Ok(())
        });
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn it_should_select_the_given_default_file_when_no_environment_base_source_is_set() {
        Jail::expect_with(|jail| {
            // Arrange
            jail.clear_env();
            let default_configuration = configuration_with_health_check_port(41005);
            jail.create_file("default.toml", &default_configuration)?;

            // Act
            let configuration = load_configuration("default.toml").expect("default source should load");

            // Assert
            assert_eq!(configuration.health_check_api.bind_address, health_check_address(41005));

            Ok(())
        });
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn it_should_apply_an_environment_override_to_a_path_environment_source() {
        Jail::expect_with(|jail| {
            // Arrange
            jail.clear_env();
            let path_configuration = configuration_with_health_check_port(41006);
            jail.create_file("path.toml", &path_configuration)?;
            jail.set_env(ENV_VAR_CONFIG_TOML_PATH, "path.toml");
            jail.set_env(
                "TORRUST_TRACKER_CONFIG_OVERRIDE_HEALTH_CHECK_API__BIND_ADDRESS",
                "127.0.0.1:41007",
            );

            // Act
            let configuration = load_configuration("default.toml").expect("path environment source should load");

            // Assert
            assert_eq!(configuration.health_check_api.bind_address, health_check_address(41007));

            Ok(())
        });
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn it_should_report_the_first_mandatory_option_when_the_path_environment_file_is_missing() {
        Jail::expect_with(|jail| {
            // Arrange
            jail.clear_env();
            jail.set_env(ENV_VAR_CONFIG_TOML_PATH, "missing.toml");

            // Act
            let result = load_configuration("default.toml");

            // Assert
            assert!(matches!(
                result,
                Err(Error::MissingMandatoryOption { path }) if path == "metadata.schema_version"
            ));

            Ok(())
        });
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn it_should_search_parent_directories_for_a_relative_path_environment_source() {
        Jail::expect_with(|jail| {
            // Arrange
            jail.clear_env();
            let parent_configuration = configuration_with_health_check_port(41008);
            jail.create_file("tracker.toml", &parent_configuration)?;
            jail.create_dir("child")?;
            jail.change_dir("child")?;
            jail.set_env(ENV_VAR_CONFIG_TOML_PATH, "tracker.toml");

            // Act
            let configuration = load_configuration("default.toml").expect("parent-directory source should load");

            // Assert
            assert_eq!(configuration.health_check_api.bind_address, health_check_address(41008));

            Ok(())
        });
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn it_should_select_an_explicit_file_without_merging_complete_toml_or_path_environment_sources() {
        Jail::expect_with(|jail| {
            // Arrange
            jail.clear_env();
            jail.create_file("explicit.toml", &configuration_with_health_check_port(41009))?;
            jail.create_file(
                "path.toml",
                &format!(
                    "{}\n[[udp_trackers]]\nbind_address = \"127.0.0.1:41010\"",
                    configuration_with_health_check_port(41010)
                ),
            )?;
            jail.set_env(
                ENV_VAR_CONFIG_TOML,
                format!(
                    "{}\n[http_api]\nbind_address = \"127.0.0.1:41011\"",
                    configuration_with_health_check_port(41011)
                ),
            );
            jail.set_env(ENV_VAR_CONFIG_TOML_PATH, "path.toml");

            // Act
            let configuration =
                load_configuration_with_explicit_path(PathBuf::from("explicit.toml")).expect("explicit source should load");

            // Assert
            assert_eq!(configuration.health_check_api.bind_address, health_check_address(41009));
            assert!(
                configuration.udp_trackers.is_none(),
                "ignored environment path source must not be merged"
            );
            assert!(
                configuration.http_api.is_none(),
                "ignored complete TOML environment source must not be merged"
            );

            Ok(())
        });
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn it_should_select_an_explicit_file_when_no_environment_base_source_is_set() {
        Jail::expect_with(|jail| {
            // Arrange
            jail.clear_env();
            jail.create_file("explicit.toml", &configuration_with_health_check_port(41012))?;

            // Act
            let configuration =
                load_configuration_with_explicit_path(PathBuf::from("explicit.toml")).expect("explicit source should load");

            // Assert
            assert_eq!(configuration.health_check_api.bind_address, health_check_address(41012));

            Ok(())
        });
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn it_should_apply_an_environment_override_to_an_explicit_file() {
        Jail::expect_with(|jail| {
            // Arrange
            jail.clear_env();
            jail.create_file("explicit.toml", &configuration_with_health_check_port(41013))?;
            jail.set_env(
                "TORRUST_TRACKER_CONFIG_OVERRIDE_HEALTH_CHECK_API__BIND_ADDRESS",
                "127.0.0.1:41014",
            );

            // Act
            let configuration =
                load_configuration_with_explicit_path(PathBuf::from("explicit.toml")).expect("explicit source should load");

            // Assert
            assert_eq!(configuration.health_check_api.bind_address, health_check_address(41014));

            Ok(())
        });
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn it_should_require_each_mandatory_option_when_loading_an_explicit_file() {
        Jail::expect_with(|jail| {
            // Arrange
            jail.clear_env();
            let mandatory_options = [
                ("metadata.schema_version", "schema_version = \"3.0.0\""),
                ("logging.trace_filter", "trace_filter = \"info\""),
                ("core.private", "private = false"),
                ("core.listed", "listed = false"),
            ];

            for (mandatory_option, toml_entry) in mandatory_options {
                let configuration_without_mandatory_option = MANDATORY_CONFIGURATION.replace(toml_entry, "");
                jail.create_file("explicit.toml", &configuration_without_mandatory_option)?;

                // Act
                let result = load_configuration_with_explicit_path(PathBuf::from("explicit.toml"));

                // Assert
                assert!(matches!(
                    result,
                    Err(Error::UnableToProcessExplicitConfigFile {
                        source,
                        ..
                    }) if source.to_string().contains(&format!("Option path: {mandatory_option}"))
                ));
            }

            Ok(())
        });
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn it_should_apply_existing_defaults_when_only_mandatory_options_are_provided_by_an_explicit_file() {
        Jail::expect_with(|jail| {
            // Arrange
            jail.clear_env();
            jail.create_file("explicit.toml", MANDATORY_CONFIGURATION)?;

            // Act
            let configuration =
                load_configuration_with_explicit_path(PathBuf::from("explicit.toml")).expect("explicit source should load");

            // Assert
            assert_eq!(
                toml::to_string(&configuration).expect("loaded configuration should serialize"),
                toml::to_string(&Configuration::default()).expect("default configuration should serialize")
            );

            Ok(())
        });
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn it_should_return_a_path_specific_error_when_an_explicit_file_is_missing() {
        Jail::expect_with(|jail| {
            // Arrange
            jail.clear_env();
            let path = PathBuf::from("missing.toml");

            // Act
            let result = load_configuration_with_explicit_path(path.clone());

            // Assert
            assert!(matches!(
                result,
                Err(Error::UnableToLoadExplicitConfigFile {
                    path: error_path,
                    source,
                }) if error_path == path && source.kind() == io::ErrorKind::NotFound
            ));

            Ok(())
        });
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn it_should_return_a_path_specific_error_when_an_explicit_path_is_a_directory() {
        Jail::expect_with(|jail| {
            // Arrange
            jail.clear_env();
            jail.create_dir("configuration")?;
            let path = PathBuf::from("configuration");

            // Act
            let result = load_configuration_with_explicit_path(path.clone());

            // Assert
            assert!(matches!(
                result,
                Err(Error::UnableToLoadExplicitConfigFile {
                    path: error_path,
                    source,
                }) if error_path == path && source.kind() == io::ErrorKind::InvalidInput
            ));

            Ok(())
        });
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn it_should_not_search_parent_directories_for_a_relative_explicit_path() {
        Jail::expect_with(|jail| {
            // Arrange
            jail.clear_env();
            jail.create_file("tracker.toml", &configuration_with_health_check_port(41015))?;
            jail.create_dir("child")?;
            jail.change_dir("child")?;
            let path = PathBuf::from("tracker.toml");

            // Act
            let result = load_configuration_with_explicit_path(path.clone());

            // Assert
            assert!(matches!(result, Err(Error::UnableToLoadExplicitConfigFile { path: error_path, .. }) if error_path == path));

            Ok(())
        });
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn it_should_include_the_explicit_path_but_not_contents_when_explicit_toml_is_malformed() {
        Jail::expect_with(|jail| {
            // Arrange
            jail.clear_env();
            let path = PathBuf::from("malformed.toml");
            let malformed_content = "sensitive-malformed-content = [";
            jail.create_file(&path, malformed_content)?;

            // Act
            let error = load_configuration_with_explicit_path(path.clone()).expect_err("malformed explicit TOML should not load");

            // Assert
            let display = error.to_string();
            assert!(display.contains(path.to_str().expect("test path should be UTF-8")));
            assert!(!display.contains(malformed_content));

            Ok(())
        });
    }

    #[cfg(unix)]
    #[test]
    #[allow(clippy::result_large_err)]
    fn it_should_preserve_a_non_utf8_explicit_path_when_explicit_toml_is_malformed() {
        use std::os::unix::ffi::OsStringExt;

        Jail::expect_with(|jail| {
            // Arrange
            jail.clear_env();
            let path = PathBuf::from(std::ffi::OsString::from_vec(b"malformed-\xFF.toml".to_vec()));
            let malformed_content = "sensitive-malformed-content = [";
            jail.create_file(&path, malformed_content)?;

            // Act
            let error = load_configuration_with_explicit_path(path.clone()).expect_err("malformed explicit TOML should not load");

            // Assert
            assert!(
                matches!(error, Error::UnableToProcessExplicitConfigFile { path: ref error_path, .. } if error_path == &path)
            );
            assert!(!error.to_string().contains(malformed_content));

            Ok(())
        });
    }

    #[test]
    #[allow(clippy::result_large_err)]
    fn it_should_load_the_content_read_when_the_explicit_file_changes_after_info_is_created() {
        Jail::expect_with(|jail| {
            // Arrange
            jail.clear_env();
            let path = PathBuf::from("explicit.toml");
            let initially_read_content = configuration_with_health_check_port(41016);
            jail.create_file(&path, &initially_read_content)?;
            let info = Info::new_with_explicit_config_toml_path("default.toml".to_owned(), Some(path))
                .expect("explicit configuration file should be readable");
            jail.create_file("explicit.toml", &configuration_with_health_check_port(41017))?;

            // Act
            let configuration = Configuration::load(&info).expect("eagerly read explicit content should load");

            // Assert
            assert_eq!(configuration.health_check_api.bind_address, health_check_address(41016));

            Ok(())
        });
    }
}

/// Announce policy for the `BitTorrent` announce cycle.
///
/// **Deprecated**: import from [`torrust_tracker_primitives::AnnouncePolicy`] instead.
/// This re-export is kept for backwards compatibility and will be removed in a
/// future release. Removal is tracked as a follow-up cleanup subissue of EPIC
/// [#1669](https://github.com/torrust/torrust-tracker/issues/1669).
#[deprecated(
    since = "3.0.0-develop",
    note = "import `AnnouncePolicy` from `torrust_tracker_primitives` instead; \
            this re-export will be removed in a future release (see EPIC #1669)"
)]
pub use torrust_tracker_primitives::AnnouncePolicy;

/// Errors that can occur when loading the configuration.
#[derive(Error, Debug)]
pub enum Error {
    /// Unable to read an explicitly selected configuration file.
    #[error("Unable to load explicit configuration file `{path}`: {source}")]
    UnableToLoadExplicitConfigFile {
        /// The explicitly selected path that could not be read.
        path: PathBuf,
        /// The file-system failure.
        #[source]
        source: io::Error,
    },

    /// Unable to parse or extract an explicitly selected configuration file.
    #[error("Unable to process explicit configuration file `{path}`: {source}")]
    UnableToProcessExplicitConfigFile {
        /// The explicitly selected path whose contents could not be processed.
        path: PathBuf,
        /// The preserved configuration diagnostic.
        #[source]
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    /// Unable to load the configuration from the environment variable.
    /// This error only occurs if there is no configuration file and the
    /// `TORRUST_TRACKER_CONFIG_TOML` environment variable is not set.
    #[error("Unable to load from Environmental Variable: {source}")]
    UnableToLoadFromEnvironmentVariable {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    #[error("Unable to load from Config File: {source}")]
    UnableToLoadFromConfigFile {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    /// Unable to load the configuration from the configuration file.
    #[error("Failed processing the configuration: {source}")]
    ConfigError {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    #[error("The error for errors that can never happen.")]
    Infallible,

    #[error("Unsupported configuration version: {version}")]
    UnsupportedVersion { version: Version },

    #[error("Missing mandatory configuration option. Option path: {path}")]
    MissingMandatoryOption { path: String },
}

impl From<figment::Error> for Error {
    #[track_caller]
    fn from(err: figment::Error) -> Self {
        Self::ConfigError {
            source: (Arc::new(err) as DynError).into(),
        }
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Default)]
pub struct TslConfig {
    /// Path to the SSL certificate file.
    #[serde(default = "TslConfig::default_ssl_cert_path")]
    pub ssl_cert_path: Utf8PathBuf,

    /// Path to the SSL key file.
    #[serde(default = "TslConfig::default_ssl_key_path")]
    pub ssl_key_path: Utf8PathBuf,
}

impl TslConfig {
    #[allow(clippy::unnecessary_wraps)]
    fn default_ssl_cert_path() -> Utf8PathBuf {
        Utf8PathBuf::new()
    }

    #[allow(clippy::unnecessary_wraps)]
    fn default_ssl_key_path() -> Utf8PathBuf {
        Utf8PathBuf::new()
    }
}
