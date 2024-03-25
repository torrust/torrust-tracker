use std::net::SocketAddr;
use std::sync::Arc;

use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch};

use super::{Entry, EntrySync};
use crate::{EntryMutexStd, EntrySingle};

impl EntrySync for EntryMutexStd {
    fn get_stats(&self) -> SwarmMetadata {
        self.lock().expect("it should get a lock").get_stats()
    }

    fn is_good(&self, policy: &TrackerPolicy) -> bool {
        self.lock().expect("it should get a lock").is_good(policy)
    }

    fn peers_is_empty(&self) -> bool {
        self.lock().expect("it should get a lock").peers_is_empty()
    }

    fn get_peers_len(&self) -> usize {
        self.lock().expect("it should get a lock").get_peers_len()
    }

    fn get_peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.lock().expect("it should get lock").get_peers(limit)
    }

    fn get_peers_for_client(&self, client: &SocketAddr, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.lock().expect("it should get lock").get_peers_for_client(client, limit)
    }

    fn insert_or_update_peer(&self, peer: &peer::Peer) -> bool {
        self.lock().expect("it should lock the entry").insert_or_update_peer(peer)
    }

    fn insert_or_update_peer_and_get_stats(&self, peer: &peer::Peer) -> (bool, SwarmMetadata) {
        self.lock()
            .expect("it should lock the entry")
            .insert_or_update_peer_and_get_stats(peer)
    }

    fn remove_inactive_peers(&self, current_cutoff: DurationSinceUnixEpoch) {
        self.lock()
            .expect("it should lock the entry")
            .remove_inactive_peers(current_cutoff);
    }
}

impl From<EntrySingle> for EntryMutexStd {
    fn from(entry: EntrySingle) -> Self {
        Arc::new(std::sync::Mutex::new(entry))
    }
}
