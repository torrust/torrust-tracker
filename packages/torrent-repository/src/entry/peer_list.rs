//! A peer list.
use std::net::SocketAddr;
use std::sync::Arc;

use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch};

// code-review: the current implementation uses the peer Id as the ``BTreeMap``
// key. That would allow adding two identical peers except for the Id.
// For example, two peers with the same socket address but a different peer Id
// would be allowed. That would lead to duplicated peers in the tracker responses.

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

    pub fn upsert(&mut self, value: Arc<peer::Peer>) -> Option<Arc<peer::Peer>> {
        self.peers.insert(value.peer_id, value)
    }

    pub fn remove(&mut self, key: &peer::Id) -> Option<Arc<peer::Peer>> {
        self.peers.remove(key)
    }

    pub fn remove_inactive_peers(&mut self, current_cutoff: DurationSinceUnixEpoch) {
        self.peers
            .retain(|_, peer| peer::ReadInfo::get_updated(peer) > current_cutoff);
    }

    #[must_use]
    pub fn get(&self, peer_id: &peer::Id) -> Option<&Arc<peer::Peer>> {
        self.peers.get(peer_id)
    }

    #[must_use]
    pub fn get_all(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        match limit {
            Some(limit) => self.peers.values().take(limit).cloned().collect(),
            None => self.peers.values().cloned().collect(),
        }
    }

    #[must_use]
    pub fn seeders_and_leechers(&self) -> (usize, usize) {
        let seeders = self.peers.values().filter(|peer| peer.is_seeder()).count();
        let leechers = self.len() - seeders;

        (seeders, leechers)
    }

    #[must_use]
    pub fn get_peers_excluding_addr(&self, peer_addr: &SocketAddr, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        match limit {
            Some(limit) => self
                .peers
                .values()
                // Take peers which are not the client peer
                .filter(|peer| peer::ReadInfo::get_address(peer.as_ref()) != *peer_addr)
                // Limit the number of peers on the result
                .take(limit)
                .cloned()
                .collect(),
            None => self
                .peers
                .values()
                // Take peers which are not the client peer
                .filter(|peer| peer::ReadInfo::get_address(peer.as_ref()) != *peer_addr)
                .cloned()
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {

    mod it_should {
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};
        use std::sync::Arc;

        use torrust_tracker_primitives::peer::fixture::PeerBuilder;
        use torrust_tracker_primitives::peer::{self};
        use torrust_tracker_primitives::DurationSinceUnixEpoch;

        use crate::entry::peer_list::PeerList;

        #[test]
        fn be_empty_when_no_peers_have_been_inserted() {
            let peer_list = PeerList::default();

            assert!(peer_list.is_empty());
        }

        #[test]
        fn have_zero_length_when_no_peers_have_been_inserted() {
            let peer_list = PeerList::default();

            assert_eq!(peer_list.len(), 0);
        }

        #[test]
        fn allow_inserting_a_new_peer() {
            let mut peer_list = PeerList::default();

            let peer = PeerBuilder::default().build();

            assert_eq!(peer_list.upsert(peer.into()), None);
        }

        #[test]
        fn allow_updating_a_preexisting_peer() {
            let mut peer_list = PeerList::default();

            let peer = PeerBuilder::default().build();

            peer_list.upsert(peer.into());

            assert_eq!(peer_list.upsert(peer.into()), Some(Arc::new(peer)));
        }

        #[test]
        fn allow_getting_all_peers() {
            let mut peer_list = PeerList::default();

            let peer = PeerBuilder::default().build();

            peer_list.upsert(peer.into());

            assert_eq!(peer_list.get_all(None), [Arc::new(peer)]);
        }

        #[test]
        fn allow_getting_one_peer_by_id() {
            let mut peer_list = PeerList::default();

            let peer = PeerBuilder::default().build();

            peer_list.upsert(peer.into());

            assert_eq!(peer_list.get(&peer.peer_id), Some(Arc::new(peer)).as_ref());
        }

        #[test]
        fn increase_the_number_of_peers_after_inserting_a_new_one() {
            let mut peer_list = PeerList::default();

            let peer = PeerBuilder::default().build();

            peer_list.upsert(peer.into());

            assert_eq!(peer_list.len(), 1);
        }

        #[test]
        fn decrease_the_number_of_peers_after_removing_one() {
            let mut peer_list = PeerList::default();

            let peer = PeerBuilder::default().build();

            peer_list.upsert(peer.into());

            peer_list.remove(&peer.peer_id);

            assert!(peer_list.is_empty());
        }

        #[test]
        fn allow_removing_an_existing_peer() {
            let mut peer_list = PeerList::default();

            let peer = PeerBuilder::default().build();

            peer_list.upsert(peer.into());

            peer_list.remove(&peer.peer_id);

            assert_eq!(peer_list.get(&peer.peer_id), None);
        }

        #[test]
        fn allow_getting_all_peers_excluding_peers_with_a_given_address() {
            let mut peer_list = PeerList::default();

            let peer1 = PeerBuilder::default()
                .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                .with_peer_addr(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6969))
                .build();
            peer_list.upsert(peer1.into());

            let peer2 = PeerBuilder::default()
                .with_peer_id(&peer::Id(*b"-qB00000000000000002"))
                .with_peer_addr(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), 6969))
                .build();
            peer_list.upsert(peer2.into());

            assert_eq!(peer_list.get_peers_excluding_addr(&peer2.peer_addr, None), [Arc::new(peer1)]);
        }

        #[test]
        fn return_the_number_of_seeders_in_the_list() {
            let mut peer_list = PeerList::default();

            let seeder = PeerBuilder::seeder().build();
            let leecher = PeerBuilder::leecher().build();

            peer_list.upsert(seeder.into());
            peer_list.upsert(leecher.into());

            let (seeders, _leechers) = peer_list.seeders_and_leechers();

            assert_eq!(seeders, 1);
        }

        #[test]
        fn return_the_number_of_leechers_in_the_list() {
            let mut peer_list = PeerList::default();

            let seeder = PeerBuilder::seeder().build();
            let leecher = PeerBuilder::leecher().build();

            peer_list.upsert(seeder.into());
            peer_list.upsert(leecher.into());

            let (_seeders, leechers) = peer_list.seeders_and_leechers();

            assert_eq!(leechers, 1);
        }

        #[test]
        fn remove_inactive_peers() {
            let mut peer_list = PeerList::default();
            let one_second = DurationSinceUnixEpoch::new(1, 0);

            // Insert the peer
            let last_update_time = DurationSinceUnixEpoch::new(1_669_397_478_934, 0);
            let peer = PeerBuilder::default().last_updated_on(last_update_time).build();
            peer_list.upsert(peer.into());

            // Remove peers not updated since one second after inserting the peer
            peer_list.remove_inactive_peers(last_update_time + one_second);

            assert_eq!(peer_list.len(), 0);
        }

        #[test]
        fn not_remove_active_peers() {
            let mut peer_list = PeerList::default();
            let one_second = DurationSinceUnixEpoch::new(1, 0);

            // Insert the peer
            let last_update_time = DurationSinceUnixEpoch::new(1_669_397_478_934, 0);
            let peer = PeerBuilder::default().last_updated_on(last_update_time).build();
            peer_list.upsert(peer.into());

            // Remove peers not updated since one second before inserting the peer.
            peer_list.remove_inactive_peers(last_update_time - one_second);

            assert_eq!(peer_list.len(), 1);
        }

        #[test]
        fn allow_inserting_two_identical_peers_except_for_the_id() {
            let mut peer_list = PeerList::default();

            let peer1 = PeerBuilder::default()
                .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                .build();
            peer_list.upsert(peer1.into());

            let peer2 = PeerBuilder::default()
                .with_peer_id(&peer::Id(*b"-qB00000000000000002"))
                .build();
            peer_list.upsert(peer2.into());

            assert_eq!(peer_list.len(), 2);
        }
    }
}
