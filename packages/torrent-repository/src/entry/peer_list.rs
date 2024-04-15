use std::net::SocketAddr;
use std::sync::Arc;

use torrust_tracker_primitives::peer;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PeerList {
    peers: std::collections::BTreeMap<peer::Id, Arc<peer::Peer>>,
}

impl PeerList {
    #[must_use]
    pub fn len(&self) -> usize {
        self.peers.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.peers.is_empty()
    }

    pub fn insert(&mut self, key: peer::Id, value: Arc<peer::Peer>) -> Option<Arc<peer::Peer>> {
        self.peers.insert(key, value)
    }

    pub fn remove(&mut self, key: &peer::Id) -> Option<Arc<peer::Peer>> {
        self.peers.remove(key)
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&peer::Id, &mut Arc<peer::Peer>) -> bool,
    {
        self.peers.retain(f);
    }

    #[must_use]
    pub fn seeders_and_leechers(&self) -> (usize, usize) {
        let seeders = self.peers.values().filter(|peer| peer.is_seeder()).count();
        let leechers = self.len() - seeders;

        (seeders, leechers)
    }

    #[must_use]
    pub fn get_peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        match limit {
            Some(limit) => self.peers.values().take(limit).cloned().collect(),
            None => self.peers.values().cloned().collect(),
        }
    }

    #[must_use]
    pub fn get_peers_for_client(&self, client: &SocketAddr, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        match limit {
            Some(limit) => self
                .peers
                .values()
                // Take peers which are not the client peer
                .filter(|peer| peer::ReadInfo::get_address(peer.as_ref()) != *client)
                // Limit the number of peers on the result
                .take(limit)
                .cloned()
                .collect(),
            None => self
                .peers
                .values()
                // Take peers which are not the client peer
                .filter(|peer| peer::ReadInfo::get_address(peer.as_ref()) != *client)
                .cloned()
                .collect(),
        }
    }
}
