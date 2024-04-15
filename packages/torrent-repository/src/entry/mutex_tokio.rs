use std::net::SocketAddr;
use std::sync::Arc;

use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch};

use super::{Entry, EntryAsync};
use crate::{BTreeMapPeerList, EntryMutexTokio, EntrySingle, SkipMapPeerList};

impl EntryAsync for EntryMutexTokio<BTreeMapPeerList> {
    async fn get_swarm_metadata(&self) -> SwarmMetadata {
        self.lock().await.get_swarm_metadata()
    }

    async fn check_good(self, policy: &TrackerPolicy) -> bool {
        self.lock().await.is_good(policy)
    }

    async fn peers_is_empty(&self) -> bool {
        self.lock().await.peers_is_empty()
    }

    async fn get_peers_len(&self) -> usize {
        self.lock().await.get_peers_len()
    }

    async fn get_peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.lock().await.get_peers(limit)
    }

    async fn get_peers_for_client(&self, client: &SocketAddr, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.lock().await.get_peers_for_client(client, limit)
    }

    async fn upsert_peer(self, peer: &peer::Peer) -> bool {
        self.lock().await.upsert_peer(peer)
    }

    async fn remove_inactive_peers(self, current_cutoff: DurationSinceUnixEpoch) {
        self.lock().await.remove_inactive_peers(current_cutoff);
    }
}

impl EntryAsync for EntryMutexTokio<SkipMapPeerList> {
    async fn get_swarm_metadata(&self) -> SwarmMetadata {
        self.lock().await.get_swarm_metadata()
    }

    async fn check_good(self, policy: &TrackerPolicy) -> bool {
        self.lock().await.is_good(policy)
    }

    async fn peers_is_empty(&self) -> bool {
        self.lock().await.peers_is_empty()
    }

    async fn get_peers_len(&self) -> usize {
        self.lock().await.get_peers_len()
    }

    async fn get_peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.lock().await.get_peers(limit)
    }

    async fn get_peers_for_client(&self, client: &SocketAddr, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.lock().await.get_peers_for_client(client, limit)
    }

    async fn upsert_peer(self, peer: &peer::Peer) -> bool {
        self.lock().await.upsert_peer(peer)
    }

    async fn remove_inactive_peers(self, current_cutoff: DurationSinceUnixEpoch) {
        self.lock().await.remove_inactive_peers(current_cutoff);
    }
}

impl<T> From<EntrySingle<T>> for EntryMutexTokio<T> {
    fn from(entry: EntrySingle<T>) -> Self {
        Arc::new(tokio::sync::Mutex::new(entry))
    }
}
