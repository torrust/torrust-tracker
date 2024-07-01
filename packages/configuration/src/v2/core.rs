use serde::{Deserialize, Serialize};

use super::network::Network;
use crate::v2::database::Database;
use crate::{AnnouncePolicy, TrackerPolicy};

#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Core {
    // Announce policy configuration.
    #[serde(default = "Core::default_announce_policy")]
    pub announce_policy: AnnouncePolicy,

    // Database configuration.
    #[serde(default = "Core::default_database")]
    pub database: Database,

    /// Interval in seconds that the cleanup job will run to remove inactive
    /// peers from the torrent peer list.
    #[serde(default = "Core::default_inactive_peer_cleanup_interval")]
    pub inactive_peer_cleanup_interval: u64,

    // When `true` only approved torrents can be announced in the tracker.
    #[serde(default = "Core::default_listed")]
    pub listed: bool,

    // Network configuration.
    #[serde(default = "Core::default_network")]
    pub net: Network,

    // When `true` clients require a key to connect and use the tracker.
    #[serde(default = "Core::default_private")]
    pub private: bool,

    // Tracker policy configuration.
    #[serde(default = "Core::default_tracker_policy")]
    pub tracker_policy: TrackerPolicy,

    /// Weather the tracker should collect statistics about tracker usage.
    /// If enabled, the tracker will collect statistics like the number of
    /// connections handled, the number of announce requests handled, etc.
    /// Refer to the [`Tracker`](https://docs.rs/torrust-tracker) for more
    /// information about the collected metrics.
    #[serde(default = "Core::default_tracker_usage_statistics")]
    pub tracker_usage_statistics: bool,
}

impl Default for Core {
    fn default() -> Self {
        Self {
            announce_policy: Self::default_announce_policy(),
            database: Self::default_database(),
            inactive_peer_cleanup_interval: Self::default_inactive_peer_cleanup_interval(),
            listed: Self::default_listed(),
            net: Self::default_network(),
            private: Self::default_private(),
            tracker_policy: Self::default_tracker_policy(),
            tracker_usage_statistics: Self::default_tracker_usage_statistics(),
        }
    }
}

impl Core {
    fn default_announce_policy() -> AnnouncePolicy {
        AnnouncePolicy::default()
    }

    fn default_database() -> Database {
        Database::default()
    }

    fn default_inactive_peer_cleanup_interval() -> u64 {
        600
    }

    fn default_listed() -> bool {
        false
    }

    fn default_network() -> Network {
        Network::default()
    }

    fn default_private() -> bool {
        false
    }

    fn default_tracker_policy() -> TrackerPolicy {
        TrackerPolicy::default()
    }
    fn default_tracker_usage_statistics() -> bool {
        true
    }
}
