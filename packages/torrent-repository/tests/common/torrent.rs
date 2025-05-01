use std::net::SocketAddr;
use std::sync::Arc;

use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch};
use torrust_tracker_torrent_repository::{entry, TorrentEntry};

#[derive(Debug, Clone)]
pub(crate) enum Torrent {
    Single(entry::torrent::Torrent),
    MutexStd(TorrentEntry),
}

impl Torrent {
    pub(crate) fn get_stats(&self) -> SwarmMetadata {
        match self {
            Torrent::Single(entry) => entry.get_swarm_metadata(),
            Torrent::MutexStd(entry) => entry
                .lock()
                .expect("can't acquire lock for torrent entry")
                .get_swarm_metadata(),
        }
    }

    pub(crate) fn meets_retaining_policy(&self, policy: &TrackerPolicy) -> bool {
        match self {
            Torrent::Single(entry) => entry.meets_retaining_policy(policy),
            Torrent::MutexStd(entry) => entry
                .lock()
                .expect("can't acquire lock for torrent entry")
                .meets_retaining_policy(policy),
        }
    }

    pub(crate) fn peers_is_empty(&self) -> bool {
        match self {
            Torrent::Single(entry) => entry.peers_is_empty(),
            Torrent::MutexStd(entry) => entry.lock().expect("can't acquire lock for torrent entry").peers_is_empty(),
        }
    }

    pub(crate) fn get_peers_len(&self) -> usize {
        match self {
            Torrent::Single(entry) => entry.get_peers_len(),
            Torrent::MutexStd(entry) => entry.lock().expect("can't acquire lock for torrent entry").get_peers_len(),
        }
    }

    pub(crate) fn get_peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        match self {
            Torrent::Single(entry) => entry.get_peers(limit),
            Torrent::MutexStd(entry) => entry.lock().expect("can't acquire lock for torrent entry").get_peers(limit),
        }
    }

    pub(crate) fn get_peers_for_client(&self, client: &SocketAddr, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        match self {
            Torrent::Single(entry) => entry.get_peers_for_client(client, limit),
            Torrent::MutexStd(entry) => entry
                .lock()
                .expect("can't acquire lock for torrent entry")
                .get_peers_for_client(client, limit),
        }
    }

    pub(crate) fn upsert_peer(&mut self, peer: &peer::Peer) -> bool {
        match self {
            Torrent::Single(entry) => entry.upsert_peer(peer),
            Torrent::MutexStd(entry) => entry.lock().expect("can't acquire lock for torrent entry").upsert_peer(peer),
        }
    }

    pub(crate) fn remove_inactive_peers(&mut self, current_cutoff: DurationSinceUnixEpoch) {
        match self {
            Torrent::Single(entry) => entry.remove_inactive_peers(current_cutoff),
            Torrent::MutexStd(entry) => entry
                .lock()
                .expect("can't acquire lock for torrent entry")
                .remove_inactive_peers(current_cutoff),
        }
    }
}
