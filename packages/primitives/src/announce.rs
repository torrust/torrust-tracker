//! Announce-related primitive types.

use std::sync::Arc;

use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};

use crate::announce_interval::AnnounceIntervalInSecs;
use crate::peer;
use crate::swarm_metadata::SwarmMetadata;

/// Announce policy
// `derive_more::Constructor` generates `field: field` initializers on this MSRV-compatible version.
// Nightly Clippy diagnoses that proc-macro expansion; remove this allowance once derive_more emits
// field-init shorthand.
#[allow(
    clippy::redundant_field_names,
    reason = "derive_more::Constructor emits field initializers on this MSRV-compatible version"
)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy, Constructor)]
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
    #[serde(default = "AnnouncePolicy::default_interval")]
    pub interval: AnnounceIntervalInSecs,

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
    #[serde(default = "AnnouncePolicy::default_interval_min")]
    pub interval_min: AnnounceIntervalInSecs,

    /// Maximum number of peers returned in a single announce response.
    ///
    /// When a client requests peers (via the `numwant` parameter or by
    /// omitting it), the tracker caps the response at this value. Clients
    /// requesting more peers than this limit will still receive at most
    /// `max_peers_per_announce` peers. Clients that omit `numwant` (asking
    /// for "as many as possible") also receive at most this many peers.
    ///
    /// Defaults to `74` (the standard `BitTorrent` peer-list size).
    #[serde(default = "AnnouncePolicy::default_max_peers_per_announce")]
    pub max_peers_per_announce: usize,
}

impl Default for AnnouncePolicy {
    fn default() -> Self {
        Self {
            interval: Self::default_interval(),
            interval_min: Self::default_interval_min(),
            max_peers_per_announce: Self::default_max_peers_per_announce(),
        }
    }
}

impl AnnouncePolicy {
    fn default_interval() -> AnnounceIntervalInSecs {
        AnnounceIntervalInSecs::from(120_u16)
    }

    fn default_interval_min() -> AnnounceIntervalInSecs {
        AnnounceIntervalInSecs::from(120_u16)
    }

    const fn default_max_peers_per_announce() -> usize {
        74
    }
}

/// Structure that holds the data returned by the `announce` request.
// `derive_more::Constructor` generates `field: field` initializers on this MSRV-compatible version.
// Nightly Clippy diagnoses that proc-macro expansion; remove this allowance once derive_more emits
// field-init shorthand.
#[allow(
    clippy::redundant_field_names,
    reason = "derive_more::Constructor emits field initializers on this MSRV-compatible version"
)]
#[derive(Clone, Debug, PartialEq, Eq, Constructor, Default)]
pub struct AnnounceData {
    /// The list of peers that are downloading the same torrent.
    /// It excludes the peer that made the request.
    pub peers: Vec<Arc<peer::Peer>>,
    /// Swarm statistics
    pub stats: SwarmMetadata,
    pub policy: AnnouncePolicy,
}

/// Intentional boundary duplication.
///
/// This domain type mirrors
/// protocol-level `AnnounceEvent` definitions in `udp-protocol` and
/// `http-protocol`, but is kept here so domain logic does not depend on
/// protocol wire formats.
// adr: docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum AnnounceEvent {
    Started,
    Stopped,
    Completed,
    None,
}

#[cfg(test)]
mod tests {
    mod announce_policy {
        use crate::announce::AnnouncePolicy;
        use crate::announce_interval::AnnounceIntervalInSecs;

        fn secs(value: u16) -> AnnounceIntervalInSecs {
            AnnounceIntervalInSecs::from(value)
        }

        #[test]
        fn it_should_default_to_the_documented_values() {
            let policy = AnnouncePolicy::default();

            assert_eq!(policy, AnnouncePolicy::new(secs(120), secs(120), 74));
        }

        #[test]
        fn it_should_use_the_default_of_every_field_missing_from_the_json() {
            let policy: AnnouncePolicy = serde_json::from_str("{}").expect("an empty object should deserialize");

            assert_eq!(policy, AnnouncePolicy::new(secs(120), secs(120), 74));
        }

        #[test]
        fn it_should_keep_the_given_field_and_default_the_others() {
            let policy: AnnouncePolicy =
                serde_json::from_str(r#"{"interval_min": 30}"#).expect("a partial object should deserialize");

            assert_eq!(policy, AnnouncePolicy::new(secs(120), secs(30), 74));
        }

        #[test]
        fn it_should_reject_an_interval_above_the_limit_of_every_delivery_protocol() {
            let result = serde_json::from_str::<AnnouncePolicy>(r#"{"interval": 2147483648}"#);

            assert!(
                result.is_err(),
                "an interval above i32::MAX should be rejected, got {result:?}"
            );
        }

        #[test]
        fn it_should_serialize_to_the_three_configuration_keys() {
            let policy = AnnouncePolicy::new(secs(60), secs(30), 10);

            let value = serde_json::to_value(policy).expect("it should serialize");

            assert_eq!(
                value,
                serde_json::json!({"interval": 60, "interval_min": 30, "max_peers_per_announce": 10})
            );
        }
    }

    mod announce_data {
        use crate::announce::{AnnounceData, AnnouncePolicy};
        use crate::announce_interval::AnnounceIntervalInSecs;
        use crate::swarm_metadata::SwarmMetadata;

        #[test]
        fn it_should_default_to_no_peers_empty_statistics_and_the_default_policy() {
            let data = AnnounceData::default();

            let default_interval = AnnounceIntervalInSecs::from(120_u16);
            let expected = AnnounceData::new(
                vec![],
                SwarmMetadata {
                    downloaded: 0,
                    complete: 0,
                    incomplete: 0,
                },
                AnnouncePolicy::new(default_interval, default_interval, 74),
            );
            assert_eq!(data, expected);
        }
    }
}
