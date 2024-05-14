//! Configuration data structures for [Torrust Tracker](https://docs.rs/torrust-tracker).
//!
//! This module contains the configuration data structures for the
//! Torrust Tracker, which is a `BitTorrent` tracker server.
//!
//! The current version for configuration is [`v1`].
pub mod v1;

use std::collections::HashMap;
use std::sync::Arc;
use std::{env, fs};

use camino::Utf8PathBuf;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString};
use thiserror::Error;
use torrust_tracker_located_error::{DynError, LocatedError};

/// The maximum number of returned peers for a torrent.
pub const TORRENT_PEERS_LIMIT: usize = 74;

pub type Configuration = v1::Configuration;
pub type UdpTracker = v1::udp_tracker::UdpTracker;
pub type HttpTracker = v1::http_tracker::HttpTracker;
pub type HttpApi = v1::tracker_api::HttpApi;
pub type HealthCheckApi = v1::health_check_api::HealthCheckApi;

pub type AccessTokens = HashMap<String, String>;

#[derive(Copy, Clone, Debug, PartialEq, Constructor)]
pub struct TrackerPolicy {
    pub remove_peerless_torrents: bool,
    pub max_peer_timeout: u32,
    pub persistent_torrent_completed_stat: bool,
}

/// Information required for loading config
#[derive(Debug, Default, Clone)]
pub struct Info {
    tracker_toml: String,
    api_admin_token: Option<String>,
}

impl Info {
    /// Build Configuration Info
    ///
    /// # Errors
    ///
    /// Will return `Err` if unable to obtain a configuration.
    ///
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(
        env_var_config: String,
        env_var_path_config: String,
        default_path_config: String,
        env_var_api_admin_token: String,
    ) -> Result<Self, Error> {
        let tracker_toml = if let Ok(tracker_toml) = env::var(&env_var_config) {
            println!("Loading configuration from env var {env_var_config} ...");

            tracker_toml
        } else {
            let config_path = if let Ok(config_path) = env::var(env_var_path_config) {
                println!("Loading configuration file: `{config_path}` ...");

                config_path
            } else {
                println!("Loading default configuration file: `{default_path_config}` ...");

                default_path_config
            };

            fs::read_to_string(config_path)
                .map_err(|e| Error::UnableToLoadFromConfigFile {
                    source: (Arc::new(e) as DynError).into(),
                })?
                .parse()
                .map_err(|_e: std::convert::Infallible| Error::Infallible)?
        };
        let api_admin_token = env::var(env_var_api_admin_token).ok();

        Ok(Self {
            tracker_toml,
            api_admin_token,
        })
    }
}

/// Announce policy
#[derive(PartialEq, Eq, Debug, Clone, Copy, Constructor)]
pub struct AnnouncePolicy {
    /// Interval in seconds that the client should wait between sending regular
    /// announce requests to the tracker.
    ///
    /// It's a **recommended** wait time between announcements.
    ///
    /// This is the standard amount of time that clients should wait between
    /// sending consecutive announcements to the tracker. This value is set by
    /// the tracker and is typically provided in the tracker's response to a
    /// client's initial request. It serves as a guideline for clients to know
    /// how often they should contact the tracker for updates on the peer list,
    /// while ensuring that the tracker is not overwhelmed with requests.
    pub interval: u32,

    /// Minimum announce interval. Clients must not reannounce more frequently
    /// than this.
    ///
    /// It establishes the shortest allowed wait time.
    ///
    /// This is an optional parameter in the protocol that the tracker may
    /// provide in its response. It sets a lower limit on the frequency at which
    /// clients are allowed to send announcements. Clients should respect this
    /// value to prevent sending too many requests in a short period, which
    /// could lead to excessive load on the tracker or even getting banned by
    /// the tracker for not adhering to the rules.
    pub interval_min: u32,
}

impl Default for AnnouncePolicy {
    fn default() -> Self {
        Self {
            interval: Self::default_interval(),
            interval_min: Self::default_interval_min(),
        }
    }
}

impl AnnouncePolicy {
    fn default_interval() -> u32 {
        120
    }

    fn default_interval_min() -> u32 {
        120
    }
}

/// Errors that can occur when loading the configuration.
#[derive(Error, Debug)]
pub enum Error {
    /// Unable to load the configuration from the environment variable.
    /// This error only occurs if there is no configuration file and the
    /// `TORRUST_TRACKER_CONFIG` environment variable is not set.
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
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(default = "TslConfig::default_ssl_cert_path")]
    pub ssl_cert_path: Option<Utf8PathBuf>,
    /// Path to the SSL key file.
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(default = "TslConfig::default_ssl_key_path")]
    pub ssl_key_path: Option<Utf8PathBuf>,
}

impl TslConfig {
    #[allow(clippy::unnecessary_wraps)]
    fn default_ssl_cert_path() -> Option<Utf8PathBuf> {
        Some(Utf8PathBuf::new())
    }

    #[allow(clippy::unnecessary_wraps)]
    fn default_ssl_key_path() -> Option<Utf8PathBuf> {
        Some(Utf8PathBuf::new())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    /// A level lower than all log levels.
    Off,
    /// Corresponds to the `Error` log level.
    Error,
    /// Corresponds to the `Warn` log level.
    Warn,
    /// Corresponds to the `Info` log level.
    Info,
    /// Corresponds to the `Debug` log level.
    Debug,
    /// Corresponds to the `Trace` log level.
    Trace,
}
