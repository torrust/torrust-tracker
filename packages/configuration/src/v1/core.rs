use serde::{Deserialize, Serialize};
use torrust_tracker_primitives::TrackerMode;

use super::network::Network;
use crate::v1::database::Database;
use crate::{AnnouncePolicy, TrackerPolicy};

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

    /// Interval in seconds that the cleanup job will run to remove inactive
    /// peers from the torrent peer list.
    #[serde(default = "Core::default_inactive_peer_cleanup_interval")]
    pub inactive_peer_cleanup_interval: u64,

    // Tracker policy configuration.
    #[serde(default = "Core::default_tracker_policy")]
    pub tracker_policy: TrackerPolicy,

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
            tracker_usage_statistics: Self::default_tracker_usage_statistics(),
            inactive_peer_cleanup_interval: Self::default_inactive_peer_cleanup_interval(),
            tracker_policy: Self::default_tracker_policy(),
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

    fn default_inactive_peer_cleanup_interval() -> u64 {
        600
    }

    fn default_tracker_policy() -> TrackerPolicy {
        TrackerPolicy::default()
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
