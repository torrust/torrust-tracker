use std::net::SocketAddr;
use std::sync::Arc;

use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch};
use torrust_tracker_torrent_repository::{entry, LockTrackedTorrent, TrackedTorrentHandle};

#[derive(Debug, Clone)]
pub(crate) enum Torrent {
    Single(entry::torrent::TrackedTorrent),
    MutexStd(TrackedTorrentHandle),
}

impl Torrent {
    pub(crate) fn get_stats(&self) -> SwarmMetadata {
        match self {
            Torrent::Single(entry) => entry.get_swarm_metadata(),
            Torrent::MutexStd(entry) => entry.lock_or_panic().get_swarm_metadata(),
        }
    }

    pub(crate) fn meets_retaining_policy(&self, policy: &TrackerPolicy) -> bool {
        match self {
            Torrent::Single(entry) => entry.meets_retaining_policy(policy),
            Torrent::MutexStd(entry) => entry.lock_or_panic().meets_retaining_policy(policy),
        }
    }

    pub(crate) fn peers_is_empty(&self) -> bool {
        match self {
            Torrent::Single(entry) => entry.swarm_is_empty(),
            Torrent::MutexStd(entry) => entry.lock_or_panic().swarm_is_empty(),
        }
    }

    pub(crate) fn get_peers_len(&self) -> usize {
        match self {
            Torrent::Single(entry) => entry.swarm_len(),
            Torrent::MutexStd(entry) => entry.lock_or_panic().swarm_len(),
        }
    }

    pub(crate) fn get_peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        match self {
            Torrent::Single(entry) => entry.swarm_peers(limit),
            Torrent::MutexStd(entry) => entry.lock_or_panic().swarm_peers(limit),
        }
    }

    pub(crate) fn get_peers_for_client(&self, client: &SocketAddr, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        match self {
            Torrent::Single(entry) => entry.get_peers_for_client(client, limit),
            Torrent::MutexStd(entry) => entry.lock_or_panic().get_peers_for_client(client, limit),
        }
    }

    pub(crate) fn upsert_peer(&mut self, peer: &peer::Peer) -> bool {
        match self {
            Torrent::Single(entry) => entry.handle_announcement(peer),
            Torrent::MutexStd(entry) => entry.lock_or_panic().handle_announcement(peer),
        }
    }

    pub(crate) fn remove_inactive_peers(&mut self, current_cutoff: DurationSinceUnixEpoch) {
        match self {
            Torrent::Single(entry) => entry.remove_inactive_peers(current_cutoff),
            Torrent::MutexStd(entry) => entry.lock_or_panic().remove_inactive_peers(current_cutoff),
        }
    }
}
