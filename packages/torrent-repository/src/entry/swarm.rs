//! A swarm is a collection of peers that are all trying to download the same
//! torrent.
use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::Arc;

use aquatic_udp_protocol::AnnounceEvent;
use torrust_tracker_primitives::peer::{self, Peer, PeerAnnouncement};
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::DurationSinceUnixEpoch;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Swarm {
    peers: BTreeMap<SocketAddr, Arc<PeerAnnouncement>>,
    metadata: SwarmMetadata,
}

impl Swarm {
    pub fn handle_announce(&mut self, incoming_announce: Arc<PeerAnnouncement>) -> Option<Arc<Peer>> {
        let is_now_seeder = incoming_announce.is_seeder();
        let has_completed = incoming_announce.event == AnnounceEvent::Completed;

        if let Some(old_announce) = self.peers.insert(incoming_announce.peer_addr, incoming_announce) {
            // A peer has been updated in the swarm.

            // Check if the peer has changed its from leecher to seeder or vice versa.
            if old_announce.is_seeder() != is_now_seeder {
                if is_now_seeder {
                    self.metadata.complete += 1;
                    self.metadata.incomplete -= 1;
                } else {
                    self.metadata.complete -= 1;
                    self.metadata.incomplete += 1;
                }
            }

            // Check if the peer has completed downloading the torrent.
            if has_completed && old_announce.event != AnnounceEvent::Completed {
                self.metadata.downloaded += 1;
            }

            Some(old_announce)
        } else {
            // A new peer has been added to the swarm.

            // Check if the peer is a seeder or a leecher.
            if is_now_seeder {
                self.metadata.complete += 1;
            } else {
                self.metadata.incomplete += 1;
            }

            // Check if the peer has completed downloading the torrent.
            if has_completed {
                // Don't increment `downloaded` here: we only count transitions
                // from a known peer
            }

            None
        }
    }

    pub fn remove(&mut self, peer_to_remove: &Peer) -> Option<Arc<Peer>> {
        match self.peers.remove(&peer_to_remove.peer_addr) {
            Some(old_peer) => {
                // A peer has been removed from the swarm.

                // Check if the peer was a seeder or a leecher.
                if old_peer.is_seeder() {
                    self.metadata.complete -= 1;
                } else {
                    self.metadata.incomplete -= 1;
                }

                Some(old_peer)
            }
            None => None,
        }
    }

    pub fn remove_inactive(&mut self, current_cutoff: DurationSinceUnixEpoch) {
        self.peers.retain(|_, peer| {
            let is_active = peer::ReadInfo::get_updated(peer) > current_cutoff;

            if !is_active {
                // Update the metadata when removing a peer.
                if peer.is_seeder() {
                    self.metadata.complete -= 1;
                } else {
                    self.metadata.incomplete -= 1;
                }
            }

            is_active
        });
    }

    #[must_use]
    pub fn get(&self, peer_addr: &SocketAddr) -> Option<&Arc<Peer>> {
        self.peers.get(peer_addr)
    }

    #[must_use]
    pub fn peers(&self, limit: Option<usize>) -> Vec<Arc<Peer>> {
        match limit {
            Some(limit) => self.peers.values().take(limit).cloned().collect(),
            None => self.peers.values().cloned().collect(),
        }
    }

    #[must_use]
    pub fn peers_excluding(&self, peer_addr: &SocketAddr, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
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

    #[must_use]
    pub fn metadata(&self) -> SwarmMetadata {
        self.metadata
    }

    /// Returns the number of seeders and leechers in the swarm.
    ///
    /// # Panics
    ///
    /// This function will panic if the `complete` or `incomplete` fields in the
    /// `metadata` field cannot be converted to `usize`.
    #[must_use]
    pub fn seeders_and_leechers(&self) -> (usize, usize) {
        let seeders = self
            .metadata
            .complete
            .try_into()
            .expect("Failed to convert 'complete' (seeders) count to usize");
        let leechers = self
            .metadata
            .incomplete
            .try_into()
            .expect("Failed to convert 'incomplete' (leechers) count to usize");

        (seeders, leechers)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.peers.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.peers.is_empty()
    }
}

#[cfg(test)]
mod tests {

    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;

    use aquatic_udp_protocol::PeerId;
    use torrust_tracker_primitives::peer::fixture::PeerBuilder;
    use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
    use torrust_tracker_primitives::DurationSinceUnixEpoch;

    use crate::entry::swarm::Swarm;

    #[test]
    fn it_should_be_empty_when_no_peers_have_been_inserted() {
        let swarm = Swarm::default();

        assert!(swarm.is_empty());
    }

    #[test]
    fn it_should_have_zero_length_when_no_peers_have_been_inserted() {
        let swarm = Swarm::default();

        assert_eq!(swarm.len(), 0);
    }

    #[test]
    fn it_should_allow_inserting_a_new_peer() {
        let mut swarm = Swarm::default();

        let peer = PeerBuilder::default().build();

        assert_eq!(swarm.handle_announce(peer.into()), None);
    }

    #[test]
    fn it_should_allow_updating_a_preexisting_peer() {
        let mut swarm = Swarm::default();

        let peer = PeerBuilder::default().build();

        swarm.handle_announce(peer.into());

        assert_eq!(swarm.handle_announce(peer.into()), Some(Arc::new(peer)));
    }

    #[test]
    fn it_should_allow_getting_all_peers() {
        let mut swarm = Swarm::default();

        let peer = PeerBuilder::default().build();

        swarm.handle_announce(peer.into());

        assert_eq!(swarm.peers(None), [Arc::new(peer)]);
    }

    #[test]
    fn it_should_allow_getting_one_peer_by_id() {
        let mut swarm = Swarm::default();

        let peer = PeerBuilder::default().build();

        swarm.handle_announce(peer.into());

        assert_eq!(swarm.get(&peer.peer_addr), Some(Arc::new(peer)).as_ref());
    }

    #[test]
    fn it_should_increase_the_number_of_peers_after_inserting_a_new_one() {
        let mut swarm = Swarm::default();

        let peer = PeerBuilder::default().build();

        swarm.handle_announce(peer.into());

        assert_eq!(swarm.len(), 1);
    }

    #[test]
    fn it_should_decrease_the_number_of_peers_after_removing_one() {
        let mut swarm = Swarm::default();

        let peer = PeerBuilder::default().build();

        swarm.handle_announce(peer.into());

        swarm.remove(&peer);

        assert!(swarm.is_empty());
    }

    #[test]
    fn it_should_allow_removing_an_existing_peer() {
        let mut swarm = Swarm::default();

        let peer = PeerBuilder::default().build();

        swarm.handle_announce(peer.into());

        let old = swarm.remove(&peer);

        assert_eq!(old, Some(Arc::new(peer)));
        assert_eq!(swarm.get(&peer.peer_addr), None);
    }

    #[test]
    fn it_should_allow_removing_a_non_existing_peer() {
        let mut swarm = Swarm::default();

        let peer = PeerBuilder::default().build();

        assert_eq!(swarm.remove(&peer), None);
    }

    #[test]
    fn it_should_allow_getting_all_peers_excluding_peers_with_a_given_address() {
        let mut swarm = Swarm::default();

        let peer1 = PeerBuilder::default()
            .with_peer_id(&PeerId(*b"-qB00000000000000001"))
            .with_peer_addr(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6969))
            .build();
        swarm.handle_announce(peer1.into());

        let peer2 = PeerBuilder::default()
            .with_peer_id(&PeerId(*b"-qB00000000000000002"))
            .with_peer_addr(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), 6969))
            .build();
        swarm.handle_announce(peer2.into());

        assert_eq!(swarm.peers_excluding(&peer2.peer_addr, None), [Arc::new(peer1)]);
    }

    #[test]
    fn it_should_remove_inactive_peers() {
        let mut swarm = Swarm::default();
        let one_second = DurationSinceUnixEpoch::new(1, 0);

        // Insert the peer
        let last_update_time = DurationSinceUnixEpoch::new(1_669_397_478_934, 0);
        let peer = PeerBuilder::default().last_updated_on(last_update_time).build();
        swarm.handle_announce(peer.into());

        // Remove peers not updated since one second after inserting the peer
        swarm.remove_inactive(last_update_time + one_second);

        assert_eq!(swarm.len(), 0);
    }

    #[test]
    fn it_should_not_remove_active_peers() {
        let mut swarm = Swarm::default();
        let one_second = DurationSinceUnixEpoch::new(1, 0);

        // Insert the peer
        let last_update_time = DurationSinceUnixEpoch::new(1_669_397_478_934, 0);
        let peer = PeerBuilder::default().last_updated_on(last_update_time).build();
        swarm.handle_announce(peer.into());

        // Remove peers not updated since one second before inserting the peer.
        swarm.remove_inactive(last_update_time - one_second);

        assert_eq!(swarm.len(), 1);
    }

    #[test]
    fn it_should_allow_inserting_two_identical_peers_except_for_the_socket_address() {
        let mut swarm = Swarm::default();

        let peer1 = PeerBuilder::default()
            .with_peer_addr(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6969))
            .build();
        swarm.handle_announce(peer1.into());

        let peer2 = PeerBuilder::default()
            .with_peer_addr(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), 6969))
            .build();
        swarm.handle_announce(peer2.into());

        assert_eq!(swarm.len(), 2);
    }

    #[test]
    fn it_should_not_allow_inserting_two_peers_with_different_peer_id_but_the_same_socket_address() {
        let mut swarm = Swarm::default();

        // When that happens the peer ID will be changed in the swarm.
        // In practice, it's like if the peer had changed its ID.

        let peer1 = PeerBuilder::default()
            .with_peer_id(&PeerId(*b"-qB00000000000000001"))
            .with_peer_addr(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6969))
            .build();
        swarm.handle_announce(peer1.into());

        let peer2 = PeerBuilder::default()
            .with_peer_id(&PeerId(*b"-qB00000000000000002"))
            .with_peer_addr(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6969))
            .build();
        swarm.handle_announce(peer2.into());

        assert_eq!(swarm.len(), 1);
    }

    #[test]
    fn it_should_return_the_metadata() {
        let mut swarm = Swarm::default();

        let seeder = PeerBuilder::seeder().build();
        let leecher = PeerBuilder::leecher().build();

        swarm.handle_announce(seeder.into());
        swarm.handle_announce(leecher.into());

        assert_eq!(
            swarm.metadata(),
            SwarmMetadata {
                downloaded: 0,
                complete: 1,
                incomplete: 1,
            }
        );
    }

    #[test]
    fn it_should_return_the_number_of_seeders_in_the_list() {
        let mut swarm = Swarm::default();

        let seeder = PeerBuilder::seeder().build();
        let leecher = PeerBuilder::leecher().build();

        swarm.handle_announce(seeder.into());
        swarm.handle_announce(leecher.into());

        let (seeders, _leechers) = swarm.seeders_and_leechers();

        assert_eq!(seeders, 1);
    }

    #[test]
    fn it_should_return_the_number_of_leechers_in_the_list() {
        let mut swarm = Swarm::default();

        let seeder = PeerBuilder::seeder().build();
        let leecher = PeerBuilder::leecher().build();

        swarm.handle_announce(seeder.into());
        swarm.handle_announce(leecher.into());

        let (_seeders, leechers) = swarm.seeders_and_leechers();

        assert_eq!(leechers, 1);
    }

    mod updating_the_swarm_metadata {

        mod when_a_new_peer_is_added {
            use torrust_tracker_primitives::peer::fixture::PeerBuilder;

            use crate::entry::swarm::Swarm;

            #[test]
            fn it_should_increase_the_number_of_leechers_if_the_new_peer_is_a_leecher_() {
                let mut swarm = Swarm::default();

                let leechers = swarm.metadata().leechers();

                let leecher = PeerBuilder::leecher().build();

                swarm.handle_announce(leecher.into());

                assert_eq!(swarm.metadata().leechers(), leechers + 1);
            }

            #[test]
            fn it_should_increase_the_number_of_seeders_if_the_new_peer_is_a_seeder() {
                let mut swarm = Swarm::default();

                let seeders = swarm.metadata().seeders();

                let seeder = PeerBuilder::seeder().build();

                swarm.handle_announce(seeder.into());

                assert_eq!(swarm.metadata().seeders(), seeders + 1);
            }

            #[test]
            fn it_should_not_increasing_the_number_of_downloads_if_the_new_peer_has_completed_downloading_as_it_was_not_previously_known(
            ) {
                let mut swarm = Swarm::default();

                let downloads = swarm.metadata().downloads();

                let seeder = PeerBuilder::seeder().build();

                swarm.handle_announce(seeder.into());

                assert_eq!(swarm.metadata().downloads(), downloads);
            }
        }

        mod when_a_peer_is_removed {
            use torrust_tracker_primitives::peer::fixture::PeerBuilder;

            use crate::entry::swarm::Swarm;

            #[test]
            fn it_should_decrease_the_number_of_leechers_if_the_removed_peer_was_a_leecher() {
                let mut swarm = Swarm::default();

                let leecher = PeerBuilder::leecher().build();

                swarm.handle_announce(leecher.into());

                let leechers = swarm.metadata().leechers();

                swarm.remove(&leecher);

                assert_eq!(swarm.metadata().leechers(), leechers - 1);
            }

            #[test]
            fn it_should_decrease_the_number_of_seeders_if_the_removed_peer_was_a_seeder() {
                let mut swarm = Swarm::default();

                let seeder = PeerBuilder::seeder().build();

                swarm.handle_announce(seeder.into());

                let seeders = swarm.metadata().seeders();

                swarm.remove(&seeder);

                assert_eq!(swarm.metadata().seeders(), seeders - 1);
            }
        }

        mod when_a_peer_is_removed_due_to_inactivity {
            use std::time::Duration;

            use torrust_tracker_primitives::peer::fixture::PeerBuilder;

            use crate::entry::swarm::Swarm;

            #[test]
            fn it_should_decrease_the_number_of_leechers_when_a_removed_peer_is_a_leecher() {
                let mut swarm = Swarm::default();

                let leecher = PeerBuilder::leecher().build();

                swarm.handle_announce(leecher.into());

                let leechers = swarm.metadata().leechers();

                swarm.remove_inactive(leecher.updated + Duration::from_secs(1));

                assert_eq!(swarm.metadata().leechers(), leechers - 1);
            }

            #[test]
            fn it_should_decrease_the_number_of_seeders_when_the_removed_peer_is_a_seeder() {
                let mut swarm = Swarm::default();

                let seeder = PeerBuilder::seeder().build();

                swarm.handle_announce(seeder.into());

                let seeders = swarm.metadata().seeders();

                swarm.remove_inactive(seeder.updated + Duration::from_secs(1));

                assert_eq!(swarm.metadata().seeders(), seeders - 1);
            }
        }

        mod for_changes_in_existing_peers {
            use aquatic_udp_protocol::NumberOfBytes;
            use torrust_tracker_primitives::peer::fixture::PeerBuilder;

            use crate::entry::swarm::Swarm;

            #[test]
            fn it_should_increase_seeders_and_decreasing_leechers_when_the_peer_changes_from_leecher_to_seeder_() {
                let mut swarm = Swarm::default();

                let mut peer = PeerBuilder::leecher().build();

                swarm.handle_announce(peer.into());

                let leechers = swarm.metadata().leechers();
                let seeders = swarm.metadata().seeders();

                peer.left = NumberOfBytes::new(0); // Convert to seeder

                swarm.handle_announce(peer.into());

                assert_eq!(swarm.metadata().seeders(), seeders + 1);
                assert_eq!(swarm.metadata().leechers(), leechers - 1);
            }

            #[test]
            fn it_should_increase_leechers_and_decreasing_seeders_when_the_peer_changes_from_seeder_to_leecher() {
                let mut swarm = Swarm::default();

                let mut peer = PeerBuilder::seeder().build();

                swarm.handle_announce(peer.into());

                let leechers = swarm.metadata().leechers();
                let seeders = swarm.metadata().seeders();

                peer.left = NumberOfBytes::new(10); // Convert to leecher

                swarm.handle_announce(peer.into());

                assert_eq!(swarm.metadata().leechers(), leechers + 1);
                assert_eq!(swarm.metadata().seeders(), seeders - 1);
            }

            #[test]
            fn it_should_increase_the_number_of_downloads_when_the_peer_announces_completed_downloading() {
                let mut swarm = Swarm::default();

                let mut peer = PeerBuilder::leecher().build();

                swarm.handle_announce(peer.into());

                let downloads = swarm.metadata().downloads();

                peer.event = aquatic_udp_protocol::AnnounceEvent::Completed;

                swarm.handle_announce(peer.into());

                assert_eq!(swarm.metadata().downloads(), downloads + 1);
            }

            #[test]
            fn it_should_not_increasing_the_number_of_downloads_when_the_peer_announces_completed_downloading_twice_() {
                let mut swarm = Swarm::default();

                let mut peer = PeerBuilder::leecher().build();

                swarm.handle_announce(peer.into());

                let downloads = swarm.metadata().downloads();

                peer.event = aquatic_udp_protocol::AnnounceEvent::Completed;

                swarm.handle_announce(peer.into());

                swarm.handle_announce(peer.into());

                assert_eq!(swarm.metadata().downloads(), downloads + 1);
            }
        }
    }
}
