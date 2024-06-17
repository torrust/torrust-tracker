use serde::{Deserialize, Serialize};
use torrust_tracker_primitives::TrackerMode;

use super::network::Network;
use crate::v1::database::Database;
use crate::AnnouncePolicy;

#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Core {
    /// Tracker mode. See [`TrackerMode`] for more information.
    #[serde(default = "Core::default_mode")]
    pub mode: TrackerMode,

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

    // Announce policy configuration.
    #[serde(default = "Core::default_announce_policy")]
    pub announce_policy: AnnouncePolicy,

    // Database configuration.
    #[serde(default = "Core::default_database")]
    pub database: Database,

    // Network configuration.
    #[serde(default = "Core::default_network")]
    pub net: Network,
}

impl Default for Core {
    fn default() -> Self {
        Self {
            mode: Self::default_mode(),
            max_peer_timeout: Self::default_max_peer_timeout(),
            tracker_usage_statistics: Self::default_tracker_usage_statistics(),
            persistent_torrent_completed_stat: Self::default_persistent_torrent_completed_stat(),
            inactive_peer_cleanup_interval: Self::default_inactive_peer_cleanup_interval(),
            remove_peerless_torrents: Self::default_remove_peerless_torrents(),
            announce_policy: Self::default_announce_policy(),
            database: Self::default_database(),
            net: Self::default_network(),
        }
    }
}

impl Core {
    fn default_mode() -> TrackerMode {
        TrackerMode::Public
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

    fn default_announce_policy() -> AnnouncePolicy {
        AnnouncePolicy::default()
    }

    fn default_database() -> Database {
        Database::default()
    }

    fn default_network() -> Network {
        Network::default()
    }
}
