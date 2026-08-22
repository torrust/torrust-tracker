//! Configuration data structures for [Torrust Tracker](https://docs.rs/torrust-tracker).
//!
//! This module contains the configuration data structures for the
//! Torrust Tracker, which is a `BitTorrent` tracker server.
//!
//! The current schema version is `v3_0_0` (in progress).
//! The previous version [`v2_0_0`] is kept for backward compatibility.
//! Global re-exports still point to `v2_0_0` and will be migrated to `v3_0_0`
//! in the final cleanup subissue (#1980) once all v3 changes are complete.
pub mod logging;
pub mod v2_0_0;
pub mod v3_0_0;
pub mod validator;

use std::collections::HashMap;
use std::env;
use std::sync::Arc;

use camino::Utf8PathBuf;
use derive_more::Display;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use thiserror::Error;
use torrust_located_error::{DynError, LocatedError};

// Environment variables

/// The whole `tracker.toml` file content. It has priority over the config file.
/// Even if the file is not on the default path.
const ENV_VAR_CONFIG_TOML: &str = "TORRUST_TRACKER_CONFIG_TOML";

/// The `tracker.toml` file location.
pub const ENV_VAR_CONFIG_TOML_PATH: &str = "TORRUST_TRACKER_CONFIG_TOML_PATH";

pub type Configuration = v2_0_0::Configuration;
pub type Core = v2_0_0::core::Core;
pub type Logging = v2_0_0::logging::Logging;
pub type HealthCheckApi = v2_0_0::health_check_api::HealthCheckApi;
pub type HttpApi = v2_0_0::tracker_api::HttpApi;
pub type HttpTracker = v2_0_0::http_tracker::HttpTracker;
pub type UdpTracker = v2_0_0::udp_tracker::UdpTracker;
pub type Database = v2_0_0::database::Database;
pub type Threshold = v2_0_0::logging::Threshold;

/// Named configuration API tokens, protected from accidental diagnostic exposure.
pub type AccessTokens = HashMap<String, SecretString>;

pub const LATEST_VERSION: &str = "2.0.0";

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
    pub fn with_schema_version(schema_version: Version) -> Self {
        Self {
            app: Self::default_app(),
            purpose: Self::default_purpose(),
            schema_version,
        }
    }

    fn default_app() -> App {
        App::TorrustTracker
    }

    fn default_purpose() -> Purpose {
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
    fn new(semver: &str) -> Self {
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
        let env_var_config_toml = ENV_VAR_CONFIG_TOML.to_string();
        let env_var_config_toml_path = ENV_VAR_CONFIG_TOML_PATH.to_string();

        let config_toml = if let Ok(config_toml) = env::var(env_var_config_toml) {
            println!("Loading extra configuration from environment variable:\n {config_toml}");
            Some(config_toml)
        } else {
            None
        };

        let config_toml_path = if let Ok(config_toml_path) = env::var(env_var_config_toml_path) {
            println!("Loading extra configuration from file: `{config_toml_path}` ...");
            config_toml_path
        } else {
            println!("Loading extra configuration from default configuration file: `{default_config_toml_path}` ...");
            default_config_toml_path
        };

        Ok(Self {
            config_toml,
            config_toml_path,
        })
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
