use std::net::SocketAddr;
use std::sync::Arc;

use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch};
use torrust_tracker_torrent_repository::entry::{Entry as _, EntryAsync as _, EntrySync as _};
use torrust_tracker_torrent_repository::{EntryMutexStd, EntryMutexTokio, EntrySingle};

#[derive(Debug, Clone)]
pub(crate) enum Torrent {
    Single(EntrySingle),
    MutexStd(EntryMutexStd),
    MutexTokio(EntryMutexTokio),
}

impl Torrent {
    pub(crate) async fn get_stats(&self) -> SwarmMetadata {
        match self {
            Torrent::Single(entry) => entry.get_stats(),
            Torrent::MutexStd(entry) => entry.get_stats(),
            Torrent::MutexTokio(entry) => entry.clone().get_stats().await,
        }
    }

    pub(crate) async fn is_good(&self, policy: &TrackerPolicy) -> bool {
        match self {
            Torrent::Single(entry) => entry.is_good(policy),
            Torrent::MutexStd(entry) => entry.is_good(policy),
            Torrent::MutexTokio(entry) => entry.clone().check_good(policy).await,
        }
    }

    pub(crate) async fn peers_is_empty(&self) -> bool {
        match self {
            Torrent::Single(entry) => entry.peers_is_empty(),
            Torrent::MutexStd(entry) => entry.peers_is_empty(),
            Torrent::MutexTokio(entry) => entry.clone().peers_is_empty().await,
        }
    }

    pub(crate) async fn get_peers_len(&self) -> usize {
        match self {
            Torrent::Single(entry) => entry.get_peers_len(),
            Torrent::MutexStd(entry) => entry.get_peers_len(),
            Torrent::MutexTokio(entry) => entry.clone().get_peers_len().await,
        }
    }

    pub(crate) async fn get_peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        match self {
            Torrent::Single(entry) => entry.get_peers(limit),
            Torrent::MutexStd(entry) => entry.get_peers(limit),
            Torrent::MutexTokio(entry) => entry.clone().get_peers(limit).await,
        }
    }

    pub(crate) async fn get_peers_for_client(&self, client: &SocketAddr, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        match self {
            Torrent::Single(entry) => entry.get_peers_for_client(client, limit),
            Torrent::MutexStd(entry) => entry.get_peers_for_client(client, limit),
            Torrent::MutexTokio(entry) => entry.clone().get_peers_for_client(client, limit).await,
        }
    }

    pub(crate) async fn insert_or_update_peer(&mut self, peer: &peer::Peer) -> bool {
        match self {
            Torrent::Single(entry) => entry.insert_or_update_peer(peer),
            Torrent::MutexStd(entry) => entry.insert_or_update_peer(peer),
            Torrent::MutexTokio(entry) => entry.clone().insert_or_update_peer(peer).await,
        }
    }

    pub(crate) async fn insert_or_update_peer_and_get_stats(&mut self, peer: &peer::Peer) -> (bool, SwarmMetadata) {
        match self {
            Torrent::Single(entry) => entry.insert_or_update_peer_and_get_stats(peer),
            Torrent::MutexStd(entry) => entry.insert_or_update_peer_and_get_stats(peer),
            Torrent::MutexTokio(entry) => entry.clone().insert_or_update_peer_and_get_stats(peer).await,
        }
    }

    pub(crate) async fn remove_inactive_peers(&mut self, current_cutoff: DurationSinceUnixEpoch) {
        match self {
            Torrent::Single(entry) => entry.remove_inactive_peers(current_cutoff),
            Torrent::MutexStd(entry) => entry.remove_inactive_peers(current_cutoff),
            Torrent::MutexTokio(entry) => entry.clone().remove_inactive_peers(current_cutoff).await,
        }
    }
}
