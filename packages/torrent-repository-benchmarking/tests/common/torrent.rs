use std::net::SocketAddr;
use std::sync::Arc;

use torrust_clock::DurationSinceUnixEpoch;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::{TrackerPolicy, peer};
use torrust_tracker_torrent_repository_benchmarking::entry::{Entry as _, EntryAsync as _, EntrySync as _};
use torrust_tracker_torrent_repository_benchmarking::{
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
            Self::Single(entry) => entry.get_swarm_metadata(),
            Self::MutexStd(entry) => entry.get_swarm_metadata(),
            Self::MutexTokio(entry) => entry.clone().get_swarm_metadata().await,
            Self::MutexParkingLot(entry) => entry.clone().get_swarm_metadata(),
            Self::RwLockParkingLot(entry) => entry.clone().get_swarm_metadata(),
        }
    }

    pub(crate) async fn meets_retaining_policy(&self, policy: &TrackerPolicy) -> bool {
        match self {
            Self::Single(entry) => entry.meets_retaining_policy(policy),
            Self::MutexStd(entry) => entry.meets_retaining_policy(policy),
            Self::MutexTokio(entry) => entry.clone().meets_retaining_policy(policy).await,
            Self::MutexParkingLot(entry) => entry.meets_retaining_policy(policy),
            Self::RwLockParkingLot(entry) => entry.meets_retaining_policy(policy),
        }
    }

    pub(crate) async fn peers_is_empty(&self) -> bool {
        match self {
            Self::Single(entry) => entry.peers_is_empty(),
            Self::MutexStd(entry) => entry.peers_is_empty(),
            Self::MutexTokio(entry) => entry.clone().peers_is_empty().await,
            Self::MutexParkingLot(entry) => entry.peers_is_empty(),
            Self::RwLockParkingLot(entry) => entry.peers_is_empty(),
        }
    }

    pub(crate) async fn get_peers_len(&self) -> usize {
        match self {
            Self::Single(entry) => entry.get_peers_len(),
            Self::MutexStd(entry) => entry.get_peers_len(),
            Self::MutexTokio(entry) => entry.clone().get_peers_len().await,
            Self::MutexParkingLot(entry) => entry.get_peers_len(),
            Self::RwLockParkingLot(entry) => entry.get_peers_len(),
        }
    }

    pub(crate) async fn get_peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        match self {
            Self::Single(entry) => entry.get_peers(limit),
            Self::MutexStd(entry) => entry.get_peers(limit),
            Self::MutexTokio(entry) => entry.clone().get_peers(limit).await,
            Self::MutexParkingLot(entry) => entry.get_peers(limit),
            Self::RwLockParkingLot(entry) => entry.get_peers(limit),
        }
    }

    pub(crate) async fn get_peers_for_client(&self, client: &SocketAddr, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        match self {
            Self::Single(entry) => entry.get_peers_for_client(client, limit),
            Self::MutexStd(entry) => entry.get_peers_for_client(client, limit),
            Self::MutexTokio(entry) => entry.clone().get_peers_for_client(client, limit).await,
            Self::MutexParkingLot(entry) => entry.get_peers_for_client(client, limit),
            Self::RwLockParkingLot(entry) => entry.get_peers_for_client(client, limit),
        }
    }

    pub(crate) async fn upsert_peer(&mut self, peer: &peer::Peer) -> bool {
        match self {
            Self::Single(entry) => entry.upsert_peer(peer),
            Self::MutexStd(entry) => entry.upsert_peer(peer),
            Self::MutexTokio(entry) => entry.clone().upsert_peer(peer).await,
            Self::MutexParkingLot(entry) => entry.upsert_peer(peer),
            Self::RwLockParkingLot(entry) => entry.upsert_peer(peer),
        }
    }

    pub(crate) async fn remove_inactive_peers(&mut self, current_cutoff: DurationSinceUnixEpoch) {
        match self {
            Self::Single(entry) => entry.remove_inactive_peers(current_cutoff),
            Self::MutexStd(entry) => entry.remove_inactive_peers(current_cutoff),
            Self::MutexTokio(entry) => entry.clone().remove_inactive_peers(current_cutoff).await,
            Self::MutexParkingLot(entry) => entry.remove_inactive_peers(current_cutoff),
            Self::RwLockParkingLot(entry) => entry.remove_inactive_peers(current_cutoff),
        }
    }
}
