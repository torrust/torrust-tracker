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
    pub(crate) swarm: Swarm,

    /// The number of peers that have ever completed downloading the torrent.
    /// This value is can be persistent so it's loaded from the database when
    /// the tracker starts.
    pub(crate) downloaded: u32,
}

impl TrackedTorrent {
    #[must_use]
    pub fn get_swarm_metadata(&self) -> SwarmMetadata {
        let metadata = self.swarm.metadata();

        SwarmMetadata {
            downloaded: self.downloaded,
            complete: metadata.complete,
            incomplete: metadata.incomplete,
        }
    }

    /// Returns true if the torrents meets the retention policy, meaning that
    /// it should be kept in the tracker.
    #[must_use]
    pub fn meets_retaining_policy(&self, policy: &TrackerPolicy) -> bool {
        // code-review: why?
        if policy.persistent_torrent_completed_stat && self.downloaded > 0 {
            return true;
        }

        if policy.remove_peerless_torrents && self.swarm.is_empty() {
            return false;
        }

        true
    }

    #[must_use]
    pub fn swarm_is_empty(&self) -> bool {
        self.swarm.is_empty()
    }

    #[must_use]
    pub fn swarm_len(&self) -> usize {
        self.swarm.len()
    }

    #[must_use]
    pub fn swarm_peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.swarm.peers(limit)
    }

    #[must_use]
    pub fn get_peers_for_client(&self, client: &SocketAddr, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.swarm.peers_excluding(client, limit)
    }

    pub fn handle_announcement(&mut self, peer: &peer::Peer) -> bool {
        let downloads_increased = self.swarm.handle_announcement(peer);

        if downloads_increased {
            self.downloaded += 1;
        }

        downloads_increased
    }

    pub fn remove_inactive_peers(&mut self, current_cutoff: DurationSinceUnixEpoch) {
        self.swarm.remove_inactive(current_cutoff);
    }
}
