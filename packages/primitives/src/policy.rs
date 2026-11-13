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
#[allow(
    clippy::redundant_field_names,
    reason = "derive_more::Constructor emits field initializers on this MSRV-compatible version"
)]
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
    ///
    /// It also selects the meaning of the `downloaded` count in announce and
    /// scrape responses. When enabled, it is the persisted lifetime count. When
    /// disabled, it counts completions registered while the torrent's swarm
    /// has been in memory in this process, and it is lost when peerless
    /// cleanup (`remove_peerless_torrents`) removes the swarm. A scrape reports
    /// the persisted count even for a torrent no peer has announced since a
    /// restart.
    // issue: #2406
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
    const fn default_max_peer_timeout() -> u32 {
        900
    }

    const fn default_persistent_torrent_completed_stat() -> bool {
        false
    }

    const fn default_remove_peerless_torrents() -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::policy::TrackerPolicy;

    #[test]
    fn it_should_default_to_the_documented_values() {
        let policy = TrackerPolicy::default();

        assert_eq!(policy, TrackerPolicy::new(900, false, true));
    }

    #[test]
    fn it_should_use_the_default_of_every_field_missing_from_the_json() {
        let policy: TrackerPolicy = serde_json::from_str("{}").expect("an empty object should deserialize");

        assert_eq!(policy, TrackerPolicy::new(900, false, true));
    }

    #[test]
    fn it_should_keep_the_given_peer_timeout_and_default_the_rest() {
        let policy: TrackerPolicy =
            serde_json::from_str(r#"{"max_peer_timeout": 60}"#).expect("a partial object should deserialize");

        assert_eq!(policy, TrackerPolicy::new(60, false, true));
    }

    #[test]
    fn it_should_keep_each_flag_in_its_own_field() {
        let policy: TrackerPolicy =
            serde_json::from_str(r#"{"persistent_torrent_completed_stat": true, "remove_peerless_torrents": false}"#)
                .expect("an object with both flags should deserialize");

        assert_eq!(policy, TrackerPolicy::new(900, true, false));
    }

    #[test]
    fn it_should_serialize_to_the_three_configuration_keys() {
        let policy = TrackerPolicy::new(60, true, false);

        let json = serde_json::to_string(&policy).expect("it should serialize");

        assert_eq!(
            json,
            r#"{"max_peer_timeout":60,"persistent_torrent_completed_stat":true,"remove_peerless_torrents":false}"#
        );
    }
}
