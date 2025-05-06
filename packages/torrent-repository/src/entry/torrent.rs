use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;

use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::peer::{self};
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use super::swarm::Swarm;

/// A data structure containing all the information about a torrent in the
/// tracker.
///
/// This is the tracker entry for a given torrent and contains the swarm data,
/// that's the list of all the peers trying to download the same torrent.
///
/// The tracker keeps one entry like this for every torrent.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TrackedTorrent {
    /// A network of peers that are all trying to download the torrent.
    swarm: Swarm,
}

impl TrackedTorrent {
    #[must_use]
    pub fn new(swarm: Swarm) -> Self {
        Self { swarm }
    }

    #[must_use]
    pub fn metadata(&self) -> SwarmMetadata {
        self.swarm.metadata()
    }

    #[must_use]
    pub fn meets_retaining_policy(&self, policy: &TrackerPolicy) -> bool {
        self.swarm.meets_retaining_policy(policy)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.swarm.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.swarm.len()
    }

    #[must_use]
    pub fn peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.swarm.peers(limit)
    }

    #[must_use]
    pub fn peers_excluding(&self, client: &SocketAddr, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.swarm.peers_excluding(client, limit)
    }

    pub fn handle_announcement(&mut self, peer: &peer::Peer) -> bool {
        self.swarm.handle_announcement(peer)
    }

    pub fn remove_inactive(&mut self, current_cutoff: DurationSinceUnixEpoch) {
        self.swarm.remove_inactive(current_cutoff);
    }
}
