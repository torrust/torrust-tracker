use std::net::SocketAddr;
use std::sync::Arc;

use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch};
use torrust_tracker_torrent_repository::entry::{Entry as _, EntryAsync as _, EntrySync as _};
use torrust_tracker_torrent_repository::{
    EntryMutexParkingLot, EntryMutexStd, EntryMutexTokio, EntryRwLockParkingLot, EntrySingle,
};

#[derive(Debug, Clone)]
pub(crate) enum Torrent {
    Single(EntrySingle),
    MutexStd(EntryMutexStd),
    MutexTokio(EntryMutexTokio),
    MutexParkingLot(EntryMutexParkingLot),
    RwLockParkingLot(EntryRwLockParkingLot),
}

impl Torrent {
    pub(crate) async fn get_stats(&self) -> SwarmMetadata {
        match self {
            Torrent::Single(entry) => entry.get_swarm_metadata(),
            Torrent::MutexStd(entry) => entry.get_swarm_metadata(),
            Torrent::MutexTokio(entry) => entry.clone().get_swarm_metadata().await,
            Torrent::MutexParkingLot(entry) => entry.clone().get_swarm_metadata(),
            Torrent::RwLockParkingLot(entry) => entry.clone().get_swarm_metadata(),
        }
    }

    pub(crate) async fn meets_retaining_policy(&self, policy: &TrackerPolicy) -> bool {
        match self {
            Torrent::Single(entry) => entry.meets_retaining_policy(policy),
            Torrent::MutexStd(entry) => entry.meets_retaining_policy(policy),
            Torrent::MutexTokio(entry) => entry.clone().meets_retaining_policy(policy).await,
            Torrent::MutexParkingLot(entry) => entry.meets_retaining_policy(policy),
            Torrent::RwLockParkingLot(entry) => entry.meets_retaining_policy(policy),
        }
    }

    pub(crate) async fn peers_is_empty(&self) -> bool {
        match self {
            Torrent::Single(entry) => entry.peers_is_empty(),
            Torrent::MutexStd(entry) => entry.peers_is_empty(),
            Torrent::MutexTokio(entry) => entry.clone().peers_is_empty().await,
            Torrent::MutexParkingLot(entry) => entry.peers_is_empty(),
            Torrent::RwLockParkingLot(entry) => entry.peers_is_empty(),
        }
    }

    pub(crate) async fn get_peers_len(&self) -> usize {
        match self {
            Torrent::Single(entry) => entry.get_peers_len(),
            Torrent::MutexStd(entry) => entry.get_peers_len(),
            Torrent::MutexTokio(entry) => entry.clone().get_peers_len().await,
            Torrent::MutexParkingLot(entry) => entry.get_peers_len(),
            Torrent::RwLockParkingLot(entry) => entry.get_peers_len(),
        }
    }

    pub(crate) async fn get_peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        match self {
            Torrent::Single(entry) => entry.get_peers(limit),
            Torrent::MutexStd(entry) => entry.get_peers(limit),
            Torrent::MutexTokio(entry) => entry.clone().get_peers(limit).await,
            Torrent::MutexParkingLot(entry) => entry.get_peers(limit),
            Torrent::RwLockParkingLot(entry) => entry.get_peers(limit),
        }
    }

    pub(crate) async fn get_peers_for_client(&self, client: &SocketAddr, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        match self {
            Torrent::Single(entry) => entry.get_peers_for_client(client, limit),
            Torrent::MutexStd(entry) => entry.get_peers_for_client(client, limit),
            Torrent::MutexTokio(entry) => entry.clone().get_peers_for_client(client, limit).await,
            Torrent::MutexParkingLot(entry) => entry.get_peers_for_client(client, limit),
            Torrent::RwLockParkingLot(entry) => entry.get_peers_for_client(client, limit),
        }
    }

    pub(crate) async fn upsert_peer(&mut self, peer: &peer::Peer) -> bool {
        match self {
            Torrent::Single(entry) => entry.upsert_peer(peer),
            Torrent::MutexStd(entry) => entry.upsert_peer(peer),
            Torrent::MutexTokio(entry) => entry.clone().upsert_peer(peer).await,
            Torrent::MutexParkingLot(entry) => entry.upsert_peer(peer),
            Torrent::RwLockParkingLot(entry) => entry.upsert_peer(peer),
        }
    }

    pub(crate) async fn remove_inactive_peers(&mut self, current_cutoff: DurationSinceUnixEpoch) {
        match self {
            Torrent::Single(entry) => entry.remove_inactive_peers(current_cutoff),
            Torrent::MutexStd(entry) => entry.remove_inactive_peers(current_cutoff),
            Torrent::MutexTokio(entry) => entry.clone().remove_inactive_peers(current_cutoff).await,
            Torrent::MutexParkingLot(entry) => entry.remove_inactive_peers(current_cutoff),
            Torrent::RwLockParkingLot(entry) => entry.remove_inactive_peers(current_cutoff),
        }
    }
}
