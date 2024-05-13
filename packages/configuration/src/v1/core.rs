use std::net::{IpAddr, Ipv4Addr};

use serde::{Deserialize, Serialize};
use torrust_tracker_primitives::{DatabaseDriver, TrackerMode};

use crate::{AnnouncePolicy, LogLevel};

#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Core {
    /// Logging level. Possible values are: `Off`, `Error`, `Warn`, `Info`,
    /// `Debug` and `Trace`. Default is `Info`.
    #[serde(default = "Core::default_log_level")]
    pub log_level: Option<LogLevel>,
    /// Tracker mode. See [`TrackerMode`] for more information.
    #[serde(default = "Core::default_mode")]
    pub mode: TrackerMode,

    // Database configuration
    /// Database driver. Possible values are: `Sqlite3`, and `MySQL`.
    #[serde(default = "Core::default_db_driver")]
    pub db_driver: DatabaseDriver,
    /// Database connection string. The format depends on the database driver.
    /// For `Sqlite3`, the format is `path/to/database.db`, for example:
    /// `./storage/tracker/lib/database/sqlite3.db`.
    /// For `Mysql`, the format is `mysql://db_user:db_user_password:port/db_name`, for
    /// example: `root:password@localhost:3306/torrust`.
    #[serde(default = "Core::default_db_path")]
    pub db_path: String,

    /// See [`AnnouncePolicy::interval`]
    #[serde(default = "AnnouncePolicy::default_interval")]
    pub announce_interval: u32,

    /// See [`AnnouncePolicy::interval_min`]
    #[serde(default = "AnnouncePolicy::default_interval_min")]
    pub min_announce_interval: u32,
    /// Weather the tracker is behind a reverse proxy or not.
    /// If the tracker is behind a reverse proxy, the `X-Forwarded-For` header
    /// sent from the proxy will be used to get the client's IP address.
    #[serde(default = "Core::default_on_reverse_proxy")]
    pub on_reverse_proxy: bool,
    /// The external IP address of the tracker. If the client is using a
    /// loopback IP address, this IP address will be used instead. If the peer
    /// is using a loopback IP address, the tracker assumes that the peer is
    /// in the same network as the tracker and will use the tracker's IP
    /// address instead.
    #[serde(default = "Core::default_external_ip")]
    pub external_ip: Option<IpAddr>,
    /// Weather the tracker should collect statistics about tracker usage.
    /// If enabled, the tracker will collect statistics like the number of
    /// connections handled, the number of announce requests handled, etc.
    /// Refer to the [`Tracker`](https://docs.rs/torrust-tracker) for more
    /// information about the collected metrics.
    #[serde(default = "Core::default_tracker_usage_statistics")]
    pub tracker_usage_statistics: bool,
    /// If enabled the tracker will persist the number of completed downloads.
    /// That's how many times a torrent has been downloaded completely.
    #[serde(default = "Core::default_persistent_torrent_completed_stat")]
    pub persistent_torrent_completed_stat: bool,

    // Cleanup job configuration
    /// Maximum time in seconds that a peer can be inactive before being
    /// considered an inactive peer. If a peer is inactive for more than this
    /// time, it will be removed from the torrent peer list.
    #[serde(default = "Core::default_max_peer_timeout")]
    pub max_peer_timeout: u32,
    /// Interval in seconds that the cleanup job will run to remove inactive
    /// peers from the torrent peer list.
    #[serde(default = "Core::default_inactive_peer_cleanup_interval")]
    pub inactive_peer_cleanup_interval: u64,
    /// If enabled, the tracker will remove torrents that have no peers.
    /// The clean up torrent job runs every `inactive_peer_cleanup_interval`
    /// seconds and it removes inactive peers. Eventually, the peer list of a
    /// torrent could be empty and the torrent will be removed if this option is
    /// enabled.
    #[serde(default = "Core::default_remove_peerless_torrents")]
    pub remove_peerless_torrents: bool,
}

impl Default for Core {
    fn default() -> Self {
        let announce_policy = AnnouncePolicy::default();

        Self {
            log_level: Self::default_log_level(),
            mode: Self::default_mode(),
            db_driver: Self::default_db_driver(),
            db_path: Self::default_db_path(),
            announce_interval: announce_policy.interval,
            min_announce_interval: announce_policy.interval_min,
            max_peer_timeout: Self::default_max_peer_timeout(),
            on_reverse_proxy: Self::default_on_reverse_proxy(),
            external_ip: Self::default_external_ip(),
            tracker_usage_statistics: Self::default_tracker_usage_statistics(),
            persistent_torrent_completed_stat: Self::default_persistent_torrent_completed_stat(),
            inactive_peer_cleanup_interval: Self::default_inactive_peer_cleanup_interval(),
            remove_peerless_torrents: Self::default_remove_peerless_torrents(),
        }
    }
}

impl Core {
    #[allow(clippy::unnecessary_wraps)]
    fn default_log_level() -> Option<LogLevel> {
        Some(LogLevel::Info)
    }

    fn default_mode() -> TrackerMode {
        TrackerMode::Public
    }

    fn default_db_driver() -> DatabaseDriver {
        DatabaseDriver::Sqlite3
    }

    fn default_db_path() -> String {
        String::from("./storage/tracker/lib/database/sqlite3.db")
    }

    fn default_on_reverse_proxy() -> bool {
        false
    }

    #[allow(clippy::unnecessary_wraps)]
    fn default_external_ip() -> Option<IpAddr> {
        Some(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)))
    }

    fn default_tracker_usage_statistics() -> bool {
        true
    }

    fn default_persistent_torrent_completed_stat() -> bool {
        false
    }

    fn default_max_peer_timeout() -> u32 {
        900
    }

    fn default_inactive_peer_cleanup_interval() -> u64 {
        600
    }

    fn default_remove_peerless_torrents() -> bool {
        true
    }
}
