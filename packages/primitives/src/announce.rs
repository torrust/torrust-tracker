//! Announce-related primitive types.

use std::sync::Arc;

use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};

use crate::peer;
use crate::swarm_metadata::SwarmMetadata;

/// Announce policy
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
    #[serde(default = "AnnouncePolicy::default_interval_min")]
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

/// Structure that holds the data returned by the `announce` request.
#[derive(Clone, Debug, PartialEq, Constructor, Default)]
pub struct AnnounceData {
    /// The list of peers that are downloading the same torrent.
    /// It excludes the peer that made the request.
    pub peers: Vec<Arc<peer::Peer>>,
    /// Swarm statistics
    pub stats: SwarmMetadata,
    pub policy: AnnouncePolicy,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum AnnounceEvent {
    Started,
    Stopped,
    Completed,
    None,
}
