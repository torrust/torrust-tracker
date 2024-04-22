//! Validated configuration for the tracker core.
//!
//! This configuration is a first level of validation that can be perform
//! statically without running the service.
use serde::{Deserialize, Serialize};
use thiserror::Error;
use torrust_tracker_primitives::{DatabaseDriver, TrackerMode};

use crate::Configuration;

/// Errors that can occur when validating the plain configuration.
#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
    /// Invalid bind address.
    #[error("Invalid log level, got: {log_level}")]
    InvalidLogLevel { log_level: String },
}

/// Configuration for each HTTP tracker.
#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Config {
    log_level: Option<String>, // todo: use enum
    mode: TrackerMode,
    db_driver: DatabaseDriver,
    db_path: String, // todo: use Path
    announce_interval: u32,
    min_announce_interval: u32,
    on_reverse_proxy: bool,
    external_ip: Option<String>, // todo: use IpAddr
    tracker_usage_statistics: bool,
    persistent_torrent_completed_stat: bool,
    max_peer_timeout: u32,
    inactive_peer_cleanup_interval: u64,
    remove_peerless_torrents: bool,
}

impl TryFrom<Configuration> for Config {
    type Error = ValidationError;

    fn try_from(config: Configuration) -> Result<Self, Self::Error> {
        // todo: validation

        Ok(Self {
            log_level: config.log_level,
            mode: config.mode,
            db_driver: config.db_driver,
            db_path: config.db_path,
            announce_interval: config.announce_interval,
            min_announce_interval: config.min_announce_interval,
            on_reverse_proxy: config.on_reverse_proxy,
            external_ip: config.external_ip,
            tracker_usage_statistics: config.tracker_usage_statistics,
            persistent_torrent_completed_stat: config.persistent_torrent_completed_stat,
            max_peer_timeout: config.max_peer_timeout,
            inactive_peer_cleanup_interval: config.inactive_peer_cleanup_interval,
            remove_peerless_torrents: config.remove_peerless_torrents,
        })
    }
}
