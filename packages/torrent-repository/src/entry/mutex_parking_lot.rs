use std::net::SocketAddr;
use std::sync::Arc;

use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch};

use super::{Entry, EntrySync};
use crate::{EntryMutexParkingLot, EntrySingle};

impl EntrySync for EntryMutexParkingLot {
    fn get_swarm_metadata(&self) -> SwarmMetadata {
        self.lock().get_swarm_metadata()
    }

    fn is_good(&self, policy: &TrackerPolicy) -> bool {
        self.lock().is_good(policy)
    }

    fn peers_is_empty(&self) -> bool {
        self.lock().peers_is_empty()
    }

    fn get_peers_len(&self) -> usize {
        self.lock().get_peers_len()
    }

    fn get_peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.lock().get_peers(limit)
    }

    fn get_peers_for_client(&self, client: &SocketAddr, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.lock().get_peers_for_client(client, limit)
    }

    fn upsert_peer(&self, peer: &peer::Peer) -> bool {
        self.lock().upsert_peer(peer)
    }

    fn remove_inactive_peers(&self, current_cutoff: DurationSinceUnixEpoch) {
        self.lock().remove_inactive_peers(current_cutoff);
    }
}

impl From<EntrySingle> for EntryMutexParkingLot {
    fn from(entry: EntrySingle) -> Self {
        Arc::new(parking_lot::Mutex::new(entry))
    }
}
