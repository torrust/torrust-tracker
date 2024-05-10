use std::net::{IpAddr, Ipv4Addr};

use serde::{Deserialize, Serialize};
use torrust_tracker_primitives::{DatabaseDriver, TrackerMode};

use crate::{AnnouncePolicy, LogLevel};

#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Core {
    /// Logging level. Possible values are: `Off`, `Error`, `Warn`, `Info`,
    /// `Debug` and `Trace`. Default is `Info`.
    pub log_level: Option<LogLevel>,
    /// Tracker mode. See [`TrackerMode`] for more information.
    pub mode: TrackerMode,

    // Database configuration
    /// Database driver. Possible values are: `Sqlite3`, and `MySQL`.
    pub db_driver: DatabaseDriver,
    /// Database connection string. The format depends on the database driver.
    /// For `Sqlite3`, the format is `path/to/database.db`, for example:
    /// `./storage/tracker/lib/database/sqlite3.db`.
    /// For `Mysql`, the format is `mysql://db_user:db_user_password:port/db_name`, for
    /// example: `root:password@localhost:3306/torrust`.
    pub db_path: String,

    /// See [`AnnouncePolicy::interval`]
    pub announce_interval: u32,

    /// See [`AnnouncePolicy::interval_min`]
    pub min_announce_interval: u32,
    /// Weather the tracker is behind a reverse proxy or not.
    /// If the tracker is behind a reverse proxy, the `X-Forwarded-For` header
    /// sent from the proxy will be used to get the client's IP address.
    pub on_reverse_proxy: bool,
    /// The external IP address of the tracker. If the client is using a
    /// loopback IP address, this IP address will be used instead. If the peer
    /// is using a loopback IP address, the tracker assumes that the peer is
    /// in the same network as the tracker and will use the tracker's IP
    /// address instead.
    pub external_ip: Option<IpAddr>,
    /// Weather the tracker should collect statistics about tracker usage.
    /// If enabled, the tracker will collect statistics like the number of
    /// connections handled, the number of announce requests handled, etc.
    /// Refer to the [`Tracker`](https://docs.rs/torrust-tracker) for more
    /// information about the collected metrics.
    pub tracker_usage_statistics: bool,
    /// If enabled the tracker will persist the number of completed downloads.
    /// That's how many times a torrent has been downloaded completely.
    pub persistent_torrent_completed_stat: bool,

    // Cleanup job configuration
    /// Maximum time in seconds that a peer can be inactive before being
    /// considered an inactive peer. If a peer is inactive for more than this
    /// time, it will be removed from the torrent peer list.
    pub max_peer_timeout: u32,
    /// Interval in seconds that the cleanup job will run to remove inactive
    /// peers from the torrent peer list.
    pub inactive_peer_cleanup_interval: u64,
    /// If enabled, the tracker will remove torrents that have no peers.
    /// The clean up torrent job runs every `inactive_peer_cleanup_interval`
    /// seconds and it removes inactive peers. Eventually, the peer list of a
    /// torrent could be empty and the torrent will be removed if this option is
    /// enabled.
    pub remove_peerless_torrents: bool,
}

impl Default for Core {
    fn default() -> Self {
        let announce_policy = AnnouncePolicy::default();

        Self {
            log_level: Some(LogLevel::Info),
            mode: TrackerMode::Public,
            db_driver: DatabaseDriver::Sqlite3,
            db_path: String::from("./storage/tracker/lib/database/sqlite3.db"),
            announce_interval: announce_policy.interval,
            min_announce_interval: announce_policy.interval_min,
            max_peer_timeout: 900,
            on_reverse_proxy: false,
            external_ip: Some(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
            tracker_usage_statistics: true,
            persistent_torrent_completed_stat: false,
            inactive_peer_cleanup_interval: 600,
            remove_peerless_torrents: true,
        }
    }
}
