//! Tracker policy types.
//!
//! This module contains the [`TrackerPolicy`] struct that governs
//! tracker-wide retention and cleanup behaviour.
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

/// Policy settings that control tracker-wide torrent and peer retention.
// `derive_more::Constructor` generates `field: field` initializers on this MSRV-compatible version.
// Nightly Clippy diagnoses that proc-macro expansion; remove this allowance once derive_more emits
// field-init shorthand.
#[allow(clippy::redundant_field_names)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Constructor)]
pub struct TrackerPolicy {
    // Cleanup job configuration
    /// Maximum time in seconds that a peer can be inactive before being
    /// considered an inactive peer. If a peer is inactive for more than this
    /// time, it will be removed from the torrent peer list.
    #[serde(default = "TrackerPolicy::default_max_peer_timeout")]
    pub max_peer_timeout: u32,

    /// If enabled the tracker will persist the number of completed downloads.
    /// That's how many times a torrent has been downloaded completely.
    #[serde(default = "TrackerPolicy::default_persistent_torrent_completed_stat")]
    pub persistent_torrent_completed_stat: bool,

    /// If enabled, the tracker will remove torrents that have no peers.
    /// The clean up torrent job runs every `inactive_peer_cleanup_interval`
    /// seconds and it removes inactive peers. Eventually, the peer list of a
    /// torrent could be empty and the torrent will be removed if this option is
    /// enabled.
    #[serde(default = "TrackerPolicy::default_remove_peerless_torrents")]
    pub remove_peerless_torrents: bool,
}

impl Default for TrackerPolicy {
    fn default() -> Self {
        Self {
            max_peer_timeout: Self::default_max_peer_timeout(),
            persistent_torrent_completed_stat: Self::default_persistent_torrent_completed_stat(),
            remove_peerless_torrents: Self::default_remove_peerless_torrents(),
        }
    }
}

impl TrackerPolicy {
    fn default_max_peer_timeout() -> u32 {
        900
    }

    fn default_persistent_torrent_completed_stat() -> bool {
        false
    }

    fn default_remove_peerless_torrents() -> bool {
        true
    }
}
