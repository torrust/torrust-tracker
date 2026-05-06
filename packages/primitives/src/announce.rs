//! Announce-related primitive types.

use std::sync::Arc;

use derive_more::derive::Constructor;
use torrust_tracker_configuration::AnnouncePolicy;

use crate::peer;
use crate::swarm_metadata::SwarmMetadata;

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
