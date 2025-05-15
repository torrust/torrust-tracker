//! A swarm is a collection of peers that are all trying to download the same
//! torrent.
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::net::SocketAddr;
use std::sync::Arc;

use aquatic_udp_protocol::AnnounceEvent;
use bittorrent_primitives::info_hash::InfoHash;
use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::peer::{self, Peer, PeerAnnouncement};
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::event::sender::Sender;
use crate::event::Event;

#[derive(Clone)]
pub struct Swarm {
    info_hash: InfoHash,
    peers: BTreeMap<SocketAddr, Arc<PeerAnnouncement>>,
    metadata: SwarmMetadata,
    event_sender: Sender,
}

#[allow(clippy::missing_fields_in_debug)]
impl Debug for Swarm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Swarm")
            .field("peers", &self.peers)
            .field("metadata", &self.metadata)
            .finish()
    }
}

impl Hash for Swarm {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.peers.hash(state);
        self.metadata.hash(state);
    }
}

impl PartialEq for Swarm {
    fn eq(&self, other: &Self) -> bool {
        self.peers == other.peers && self.metadata == other.metadata
    }
}

impl Eq for Swarm {}

impl Swarm {
    #[must_use]
    pub fn new(info_hash: &InfoHash, downloaded: u32, event_sender: Sender) -> Self {
        Self {
            info_hash: *info_hash,
            peers: BTreeMap::new(),
            metadata: SwarmMetadata::new(downloaded, 0, 0),
            event_sender,
        }
    }

    pub async fn handle_announcement(&mut self, incoming_announce: &PeerAnnouncement) -> bool {
        let mut downloads_increased: bool = false;

        let _previous_peer = match peer::ReadInfo::get_event(incoming_announce) {
            AnnounceEvent::Started | AnnounceEvent::None | AnnounceEvent::Completed => {
                self.upsert_peer(Arc::new(*incoming_announce), &mut downloads_increased).await
            }
            AnnounceEvent::Stopped => self.remove(incoming_announce).await,
        };

        downloads_increased
    }

    async fn upsert_peer(
        &mut self,
        incoming_announce: Arc<PeerAnnouncement>,
        downloads_increased: &mut bool,
    ) -> Option<Arc<Peer>> {
        let announcement = incoming_announce.clone();

        if let Some(previous_announce) = self.peers.insert(incoming_announce.peer_addr, incoming_announce) {
            *downloads_increased = self.update_metadata(Some(&previous_announce), &announcement);

            self.trigger_peer_updated_event(&previous_announce, &announcement, *downloads_increased)
                .await;

            Some(previous_announce)
        } else {
            *downloads_increased = self.update_metadata(None, &announcement);

            self.trigger_peer_added_event(&announcement).await;

            None
        }
    }

    fn update_metadata(
        &mut self,
        opt_previous_announce: Option<&Arc<PeerAnnouncement>>,
        new_announce: &Arc<PeerAnnouncement>,
    ) -> bool {
        let mut downloads_increased = false;

        if let Some(previous_announce) = opt_previous_announce {
            if previous_announce.role() != new_announce.role() {
                if new_announce.is_seeder() {
                    self.metadata.complete += 1;
                    self.metadata.incomplete -= 1;
                } else {
                    self.metadata.complete -= 1;
                    self.metadata.incomplete += 1;
                }
            }

            if new_announce.is_completed() && !previous_announce.is_completed() {
                self.metadata.downloaded += 1;
                downloads_increased = true;
            }
        } else if new_announce.is_seeder() {
            self.metadata.complete += 1;
        } else {
            self.metadata.incomplete += 1;
        }

        downloads_increased
    }

    async fn trigger_peer_updated_event(
        &self,
        old_announce: &Arc<PeerAnnouncement>,
        new_announce: &Arc<PeerAnnouncement>,
        downloads_increased: bool,
    ) {
        if let Some(event_sender) = self.event_sender.as_deref() {
            event_sender
                .send(Event::PeerUpdated {
                    info_hash: self.info_hash,
                    old_peer: *old_announce.clone(),
                    new_peer: *new_announce.clone(),
                })
                .await;

            if downloads_increased {
                event_sender
                    .send(Event::PeerDownloadCompleted {
                        info_hash: self.info_hash,
                        peer: *new_announce.clone(),
                    })
                    .await;
            }
        }
    }

    async fn trigger_peer_added_event(&self, announcement: &Arc<PeerAnnouncement>) {
        if let Some(event_sender) = self.event_sender.as_deref() {
            event_sender
                .send(Event::PeerAdded {
                    info_hash: self.info_hash,
                    peer: *announcement.clone(),
                })
                .await;
        }
    }

    pub async fn remove(&mut self, peer_to_remove: &Peer) -> Option<Arc<Peer>> {
        match self.peers.remove(&peer_to_remove.peer_addr) {
            Some(old_peer) => {
                // A peer has been removed from the swarm.

                // Check if the peer was a seeder or a leecher.
                if old_peer.is_seeder() {
                    self.metadata.complete -= 1;
                } else {
                    self.metadata.incomplete -= 1;
                }

                if let Some(event_sender) = self.event_sender.as_deref() {
                    event_sender
                        .send(Event::PeerRemoved {
                            info_hash: self.info_hash,
                            peer: *old_peer.clone(),
                        })
                        .await;
                }

                Some(old_peer)
            }
            None => None,
        }
    }

    pub async fn remove_inactive(&mut self, current_cutoff: DurationSinceUnixEpoch) -> usize {
        let mut number_of_peers_removed = 0;
        let mut removed_peers = Vec::new();

        self.peers.retain(|_key, peer| {
            let is_active = peer::ReadInfo::get_updated(peer) > current_cutoff;

            if !is_active {
                // Update the metadata when removing a peer.
                if peer.is_seeder() {
                    self.metadata.complete -= 1;
                } else {
                    self.metadata.incomplete -= 1;
                }

                number_of_peers_removed += 1;

                if let Some(_event_sender) = self.event_sender.as_deref() {
                    // Events can not be trigger here because retain does not allow
                    // async closures.
                    removed_peers.push(*peer.clone());
                }
            }

            is_active
        });

        if let Some(event_sender) = self.event_sender.as_deref() {
            for peer in &removed_peers {
                event_sender
                    .send(Event::PeerRemoved {
                        info_hash: self.info_hash,
                        peer: *peer,
                    })
                    .await;
            }
        }

        number_of_peers_removed
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

    #[must_use]
    pub fn is_peerless(&self) -> bool {
        self.is_empty()
    }

    /// Returns true if the swarm meets the retention policy, meaning that
    /// it should be kept in the list of swarms.
    #[must_use]
    pub fn meets_retaining_policy(&self, policy: &TrackerPolicy) -> bool {
        !self.should_be_removed(policy)
    }

    fn should_be_removed(&self, policy: &TrackerPolicy) -> bool {
        // If the policy is to remove peerless torrents and the swarm is empty (no peers),
        (policy.remove_peerless_torrents && self.is_empty())
            // but not when the policy is to persist torrent stats and the 
            // torrent has been downloaded at least once.
            // (because the only way to store the counter is to keep the swarm in memory.
            // See https://github.com/torrust/torrust-tracker/issues/1502)
            && !(policy.persistent_torrent_completed_stat && self.metadata().downloaded > 0)
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

    use crate::swarm::Swarm;
    use crate::tests::sample_info_hash;

    #[test]
    fn it_should_allow_debugging() {
        let swarm = Swarm::new(&sample_info_hash(), 0, None);

        assert_eq!(
            format!("{swarm:?}"),
            "Swarm { peers: {}, metadata: SwarmMetadata { downloaded: 0, complete: 0, incomplete: 0 } }"
        );
    }

    #[test]
    fn it_should_be_empty_when_no_peers_have_been_inserted() {
        let swarm = Swarm::new(&sample_info_hash(), 0, None);

        assert!(swarm.is_empty());
    }

    #[test]
    fn it_should_have_zero_length_when_no_peers_have_been_inserted() {
        let swarm = Swarm::new(&sample_info_hash(), 0, None);

        assert_eq!(swarm.len(), 0);
    }

    #[tokio::test]
    async fn it_should_allow_inserting_a_new_peer() {
        let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
        let mut downloads_increased = false;

        let peer = PeerBuilder::default().build();

        assert_eq!(swarm.upsert_peer(peer.into(), &mut downloads_increased).await, None);
    }

    #[tokio::test]
    async fn it_should_allow_updating_a_preexisting_peer() {
        let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
        let mut downloads_increased = false;

        let peer = PeerBuilder::default().build();

        swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

        assert_eq!(
            swarm.upsert_peer(peer.into(), &mut downloads_increased).await,
            Some(Arc::new(peer))
        );
    }

    #[tokio::test]
    async fn it_should_allow_getting_all_peers() {
        let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
        let mut downloads_increased = false;

        let peer = PeerBuilder::default().build();

        swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

        assert_eq!(swarm.peers(None), [Arc::new(peer)]);
    }

    #[tokio::test]
    async fn it_should_allow_getting_one_peer_by_id() {
        let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
        let mut downloads_increased = false;

        let peer = PeerBuilder::default().build();

        swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

        assert_eq!(swarm.get(&peer.peer_addr), Some(Arc::new(peer)).as_ref());
    }

    #[tokio::test]
    async fn it_should_increase_the_number_of_peers_after_inserting_a_new_one() {
        let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
        let mut downloads_increased = false;

        let peer = PeerBuilder::default().build();

        swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

        assert_eq!(swarm.len(), 1);
    }

    #[tokio::test]
    async fn it_should_decrease_the_number_of_peers_after_removing_one() {
        let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
        let mut downloads_increased = false;

        let peer = PeerBuilder::default().build();

        swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

        swarm.remove(&peer).await;

        assert!(swarm.is_empty());
    }

    #[tokio::test]
    async fn it_should_allow_removing_an_existing_peer() {
        let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
        let mut downloads_increased = false;

        let peer = PeerBuilder::default().build();

        swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

        let old = swarm.remove(&peer).await;

        assert_eq!(old, Some(Arc::new(peer)));
        assert_eq!(swarm.get(&peer.peer_addr), None);
    }

    #[tokio::test]
    async fn it_should_allow_removing_a_non_existing_peer() {
        let mut swarm = Swarm::new(&sample_info_hash(), 0, None);

        let peer = PeerBuilder::default().build();

        assert_eq!(swarm.remove(&peer).await, None);
    }

    #[tokio::test]
    async fn it_should_allow_getting_all_peers_excluding_peers_with_a_given_address() {
        let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
        let mut downloads_increased = false;

        let peer1 = PeerBuilder::default()
            .with_peer_id(&PeerId(*b"-qB00000000000000001"))
            .with_peer_addr(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6969))
            .build();
        swarm.upsert_peer(peer1.into(), &mut downloads_increased).await;

        let peer2 = PeerBuilder::default()
            .with_peer_id(&PeerId(*b"-qB00000000000000002"))
            .with_peer_addr(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), 6969))
            .build();
        swarm.upsert_peer(peer2.into(), &mut downloads_increased).await;

        assert_eq!(swarm.peers_excluding(&peer2.peer_addr, None), [Arc::new(peer1)]);
    }

    #[tokio::test]
    async fn it_should_remove_inactive_peers() {
        let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
        let mut downloads_increased = false;
        let one_second = DurationSinceUnixEpoch::new(1, 0);

        // Insert the peer
        let last_update_time = DurationSinceUnixEpoch::new(1_669_397_478_934, 0);
        let peer = PeerBuilder::default().last_updated_on(last_update_time).build();
        swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

        // Remove peers not updated since one second after inserting the peer
        swarm.remove_inactive(last_update_time + one_second).await;

        assert_eq!(swarm.len(), 0);
    }

    #[tokio::test]
    async fn it_should_not_remove_active_peers() {
        let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
        let mut downloads_increased = false;
        let one_second = DurationSinceUnixEpoch::new(1, 0);

        // Insert the peer
        let last_update_time = DurationSinceUnixEpoch::new(1_669_397_478_934, 0);
        let peer = PeerBuilder::default().last_updated_on(last_update_time).build();
        swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

        // Remove peers not updated since one second before inserting the peer.
        swarm.remove_inactive(last_update_time - one_second).await;

        assert_eq!(swarm.len(), 1);
    }

    mod for_retaining_policy {

        use torrust_tracker_configuration::TrackerPolicy;
        use torrust_tracker_primitives::peer::fixture::PeerBuilder;

        use crate::tests::sample_info_hash;
        use crate::Swarm;

        fn empty_swarm() -> Swarm {
            Swarm::new(&sample_info_hash(), 0, None)
        }

        async fn not_empty_swarm() -> Swarm {
            let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
            swarm.upsert_peer(PeerBuilder::default().build().into(), &mut false).await;
            swarm
        }

        async fn not_empty_swarm_with_downloads() -> Swarm {
            let mut swarm = Swarm::new(&sample_info_hash(), 0, None);

            let mut peer = PeerBuilder::leecher().build();
            let mut downloads_increased = false;

            swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

            peer.event = aquatic_udp_protocol::AnnounceEvent::Completed;

            swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

            assert!(swarm.metadata().downloads() > 0);

            swarm
        }

        fn remove_peerless_torrents_policy() -> TrackerPolicy {
            TrackerPolicy {
                remove_peerless_torrents: true,
                ..Default::default()
            }
        }

        fn don_not_remove_peerless_torrents_policy() -> TrackerPolicy {
            TrackerPolicy {
                remove_peerless_torrents: false,
                ..Default::default()
            }
        }

        mod when_removing_peerless_torrents_is_enabled {

            use torrust_tracker_configuration::TrackerPolicy;

            use crate::swarm::tests::for_retaining_policy::{
                empty_swarm, not_empty_swarm, not_empty_swarm_with_downloads, remove_peerless_torrents_policy,
            };

            #[test]
            fn it_should_be_removed_if_the_swarm_is_empty() {
                assert!(empty_swarm().should_be_removed(&remove_peerless_torrents_policy()));
            }

            #[tokio::test]
            async fn it_should_not_be_removed_is_the_swarm_is_not_empty() {
                assert!(!not_empty_swarm().await.should_be_removed(&remove_peerless_torrents_policy()));
            }

            #[tokio::test]
            async fn it_should_not_be_removed_even_if_the_swarm_is_empty_if_we_need_to_track_stats_for_downloads_and_there_has_been_downloads(
            ) {
                let policy = TrackerPolicy {
                    remove_peerless_torrents: true,
                    persistent_torrent_completed_stat: true,
                    ..Default::default()
                };

                assert!(!not_empty_swarm_with_downloads().await.should_be_removed(&policy));
            }
        }

        mod when_removing_peerless_torrents_is_disabled {

            use crate::swarm::tests::for_retaining_policy::{
                don_not_remove_peerless_torrents_policy, empty_swarm, not_empty_swarm,
            };

            #[test]
            fn it_should_not_be_removed_even_if_the_swarm_is_empty() {
                assert!(!empty_swarm().should_be_removed(&don_not_remove_peerless_torrents_policy()));
            }

            #[tokio::test]
            async fn it_should_not_be_removed_is_the_swarm_is_not_empty() {
                assert!(!not_empty_swarm()
                    .await
                    .should_be_removed(&don_not_remove_peerless_torrents_policy()));
            }
        }
    }

    #[tokio::test]
    async fn it_should_allow_inserting_two_identical_peers_except_for_the_socket_address() {
        let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
        let mut downloads_increased = false;

        let peer1 = PeerBuilder::default()
            .with_peer_addr(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6969))
            .build();
        swarm.upsert_peer(peer1.into(), &mut downloads_increased).await;

        let peer2 = PeerBuilder::default()
            .with_peer_addr(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), 6969))
            .build();
        swarm.upsert_peer(peer2.into(), &mut downloads_increased).await;

        assert_eq!(swarm.len(), 2);
    }

    #[tokio::test]
    async fn it_should_not_allow_inserting_two_peers_with_different_peer_id_but_the_same_socket_address() {
        let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
        let mut downloads_increased = false;

        // When that happens the peer ID will be changed in the swarm.
        // In practice, it's like if the peer had changed its ID.

        let peer1 = PeerBuilder::default()
            .with_peer_id(&PeerId(*b"-qB00000000000000001"))
            .with_peer_addr(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6969))
            .build();
        swarm.upsert_peer(peer1.into(), &mut downloads_increased).await;

        let peer2 = PeerBuilder::default()
            .with_peer_id(&PeerId(*b"-qB00000000000000002"))
            .with_peer_addr(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6969))
            .build();
        swarm.upsert_peer(peer2.into(), &mut downloads_increased).await;

        assert_eq!(swarm.len(), 1);
    }

    #[tokio::test]
    async fn it_should_return_the_swarm_metadata() {
        let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
        let mut downloads_increased = false;

        let seeder = PeerBuilder::seeder().build();
        let leecher = PeerBuilder::leecher().build();

        swarm.upsert_peer(seeder.into(), &mut downloads_increased).await;
        swarm.upsert_peer(leecher.into(), &mut downloads_increased).await;

        assert_eq!(
            swarm.metadata(),
            SwarmMetadata {
                downloaded: 0,
                complete: 1,
                incomplete: 1,
            }
        );
    }

    #[tokio::test]
    async fn it_should_return_the_number_of_seeders_in_the_list() {
        let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
        let mut downloads_increased = false;

        let seeder = PeerBuilder::seeder().build();
        let leecher = PeerBuilder::leecher().build();

        swarm.upsert_peer(seeder.into(), &mut downloads_increased).await;
        swarm.upsert_peer(leecher.into(), &mut downloads_increased).await;

        let (seeders, _leechers) = swarm.seeders_and_leechers();

        assert_eq!(seeders, 1);
    }

    #[tokio::test]
    async fn it_should_return_the_number_of_leechers_in_the_list() {
        let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
        let mut downloads_increased = false;

        let seeder = PeerBuilder::seeder().build();
        let leecher = PeerBuilder::leecher().build();

        swarm.upsert_peer(seeder.into(), &mut downloads_increased).await;
        swarm.upsert_peer(leecher.into(), &mut downloads_increased).await;

        let (_seeders, leechers) = swarm.seeders_and_leechers();

        assert_eq!(leechers, 1);
    }

    #[tokio::test]
    async fn it_should_be_a_peerless_swarm_when_it_does_not_contain_any_peers() {
        let swarm = Swarm::new(&sample_info_hash(), 0, None);
        assert!(swarm.is_peerless());
    }

    mod updating_the_swarm_metadata {

        mod when_a_new_peer_is_added {
            use torrust_tracker_primitives::peer::fixture::PeerBuilder;

            use crate::swarm::Swarm;
            use crate::tests::sample_info_hash;

            #[tokio::test]
            async fn it_should_increase_the_number_of_leechers_if_the_new_peer_is_a_leecher_() {
                let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
                let mut downloads_increased = false;

                let leechers = swarm.metadata().leechers();

                let leecher = PeerBuilder::leecher().build();

                swarm.upsert_peer(leecher.into(), &mut downloads_increased).await;

                assert_eq!(swarm.metadata().leechers(), leechers + 1);
            }

            #[tokio::test]
            async fn it_should_increase_the_number_of_seeders_if_the_new_peer_is_a_seeder() {
                let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
                let mut downloads_increased = false;

                let seeders = swarm.metadata().seeders();

                let seeder = PeerBuilder::seeder().build();

                swarm.upsert_peer(seeder.into(), &mut downloads_increased).await;

                assert_eq!(swarm.metadata().seeders(), seeders + 1);
            }

            #[tokio::test]
            async fn it_should_not_increasing_the_number_of_downloads_if_the_new_peer_has_completed_downloading_as_it_was_not_previously_known(
            ) {
                let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
                let mut downloads_increased = false;

                let downloads = swarm.metadata().downloads();

                let seeder = PeerBuilder::seeder().build();

                swarm.upsert_peer(seeder.into(), &mut downloads_increased).await;

                assert_eq!(swarm.metadata().downloads(), downloads);
            }
        }

        mod when_a_peer_is_removed {
            use torrust_tracker_primitives::peer::fixture::PeerBuilder;

            use crate::swarm::Swarm;
            use crate::tests::sample_info_hash;

            #[tokio::test]
            async fn it_should_decrease_the_number_of_leechers_if_the_removed_peer_was_a_leecher() {
                let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
                let mut downloads_increased = false;

                let leecher = PeerBuilder::leecher().build();

                swarm.upsert_peer(leecher.into(), &mut downloads_increased).await;

                let leechers = swarm.metadata().leechers();

                swarm.remove(&leecher).await;

                assert_eq!(swarm.metadata().leechers(), leechers - 1);
            }

            #[tokio::test]
            async fn it_should_decrease_the_number_of_seeders_if_the_removed_peer_was_a_seeder() {
                let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
                let mut downloads_increased = false;

                let seeder = PeerBuilder::seeder().build();

                swarm.upsert_peer(seeder.into(), &mut downloads_increased).await;

                let seeders = swarm.metadata().seeders();

                swarm.remove(&seeder).await;

                assert_eq!(swarm.metadata().seeders(), seeders - 1);
            }
        }

        mod when_a_peer_is_removed_due_to_inactivity {
            use std::time::Duration;

            use torrust_tracker_primitives::peer::fixture::PeerBuilder;

            use crate::swarm::Swarm;
            use crate::tests::sample_info_hash;

            #[tokio::test]
            async fn it_should_decrease_the_number_of_leechers_when_a_removed_peer_is_a_leecher() {
                let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
                let mut downloads_increased = false;

                let leecher = PeerBuilder::leecher().build();

                swarm.upsert_peer(leecher.into(), &mut downloads_increased).await;

                let leechers = swarm.metadata().leechers();

                swarm.remove_inactive(leecher.updated + Duration::from_secs(1)).await;

                assert_eq!(swarm.metadata().leechers(), leechers - 1);
            }

            #[tokio::test]
            async fn it_should_decrease_the_number_of_seeders_when_the_removed_peer_is_a_seeder() {
                let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
                let mut downloads_increased = false;

                let seeder = PeerBuilder::seeder().build();

                swarm.upsert_peer(seeder.into(), &mut downloads_increased).await;

                let seeders = swarm.metadata().seeders();

                swarm.remove_inactive(seeder.updated + Duration::from_secs(1)).await;

                assert_eq!(swarm.metadata().seeders(), seeders - 1);
            }
        }

        mod for_changes_in_existing_peers {
            use aquatic_udp_protocol::NumberOfBytes;
            use torrust_tracker_primitives::peer::fixture::PeerBuilder;

            use crate::swarm::Swarm;
            use crate::tests::sample_info_hash;

            #[tokio::test]
            async fn it_should_increase_seeders_and_decreasing_leechers_when_the_peer_changes_from_leecher_to_seeder_() {
                let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
                let mut downloads_increased = false;

                let mut peer = PeerBuilder::leecher().build();

                swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

                let leechers = swarm.metadata().leechers();
                let seeders = swarm.metadata().seeders();

                peer.left = NumberOfBytes::new(0); // Convert to seeder

                swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

                assert_eq!(swarm.metadata().seeders(), seeders + 1);
                assert_eq!(swarm.metadata().leechers(), leechers - 1);
            }

            #[tokio::test]
            async fn it_should_increase_leechers_and_decreasing_seeders_when_the_peer_changes_from_seeder_to_leecher() {
                let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
                let mut downloads_increased = false;

                let mut peer = PeerBuilder::seeder().build();

                swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

                let leechers = swarm.metadata().leechers();
                let seeders = swarm.metadata().seeders();

                peer.left = NumberOfBytes::new(10); // Convert to leecher

                swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

                assert_eq!(swarm.metadata().leechers(), leechers + 1);
                assert_eq!(swarm.metadata().seeders(), seeders - 1);
            }

            #[tokio::test]
            async fn it_should_increase_the_number_of_downloads_when_the_peer_announces_completed_downloading() {
                let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
                let mut downloads_increased = false;

                let mut peer = PeerBuilder::leecher().build();

                swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

                let downloads = swarm.metadata().downloads();

                peer.event = aquatic_udp_protocol::AnnounceEvent::Completed;

                swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

                assert_eq!(swarm.metadata().downloads(), downloads + 1);
            }

            #[tokio::test]
            async fn it_should_not_increasing_the_number_of_downloads_when_the_peer_announces_completed_downloading_twice_() {
                let mut swarm = Swarm::new(&sample_info_hash(), 0, None);
                let mut downloads_increased = false;

                let mut peer = PeerBuilder::leecher().build();

                swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

                let downloads = swarm.metadata().downloads();

                peer.event = aquatic_udp_protocol::AnnounceEvent::Completed;

                swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

                swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

                assert_eq!(swarm.metadata().downloads(), downloads + 1);
            }
        }
    }

    mod triggering_events {

        use std::sync::Arc;

        use aquatic_udp_protocol::AnnounceEvent::Started;
        use torrust_tracker_primitives::peer::fixture::PeerBuilder;
        use torrust_tracker_primitives::DurationSinceUnixEpoch;

        use crate::event::sender::tests::{expect_event_sequence, MockEventSender};
        use crate::event::Event;
        use crate::swarm::Swarm;
        use crate::tests::sample_info_hash;

        #[tokio::test]
        async fn it_should_trigger_an_event_when_a_new_peer_is_added() {
            let info_hash = sample_info_hash();
            let peer = PeerBuilder::leecher().build();

            let mut event_sender_mock = MockEventSender::new();

            expect_event_sequence(&mut event_sender_mock, vec![Event::PeerAdded { info_hash, peer }]);

            let mut swarm = Swarm::new(&sample_info_hash(), 0, Some(Arc::new(event_sender_mock)));

            let mut downloads_increased = false;
            swarm.upsert_peer(peer.into(), &mut downloads_increased).await;
        }

        #[tokio::test]
        async fn it_should_trigger_an_event_when_a_peer_is_directly_removed() {
            let info_hash = sample_info_hash();
            let peer = PeerBuilder::leecher().build();

            let mut event_sender_mock = MockEventSender::new();

            expect_event_sequence(
                &mut event_sender_mock,
                vec![Event::PeerAdded { info_hash, peer }, Event::PeerRemoved { info_hash, peer }],
            );

            let mut swarm = Swarm::new(&info_hash, 0, Some(Arc::new(event_sender_mock)));

            // Insert the peer
            let mut downloads_increased = false;
            swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

            swarm.remove(&peer).await;
        }

        #[tokio::test]
        async fn it_should_trigger_an_event_when_a_peer_is_removed_due_to_inactivity() {
            let info_hash = sample_info_hash();
            let peer = PeerBuilder::leecher().build();

            let mut event_sender_mock = MockEventSender::new();

            expect_event_sequence(
                &mut event_sender_mock,
                vec![Event::PeerAdded { info_hash, peer }, Event::PeerRemoved { info_hash, peer }],
            );

            let mut swarm = Swarm::new(&info_hash, 0, Some(Arc::new(event_sender_mock)));

            // Insert the peer
            let mut downloads_increased = false;
            swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

            // Peers not updated after this time will be removed
            let current_cutoff = peer.updated + DurationSinceUnixEpoch::from_secs(1);

            swarm.remove_inactive(current_cutoff).await;
        }

        #[tokio::test]
        async fn it_should_trigger_an_event_when_a_peer_is_updated() {
            let info_hash = sample_info_hash();
            let peer = PeerBuilder::leecher().with_event(Started).build();

            let mut event_sender_mock = MockEventSender::new();

            expect_event_sequence(
                &mut event_sender_mock,
                vec![
                    Event::PeerAdded { info_hash, peer },
                    Event::PeerUpdated {
                        info_hash,
                        old_peer: peer,
                        new_peer: peer,
                    },
                ],
            );

            let mut swarm = Swarm::new(&info_hash, 0, Some(Arc::new(event_sender_mock)));

            // Insert the peer
            let mut downloads_increased = false;
            swarm.upsert_peer(peer.into(), &mut downloads_increased).await;

            // Update the peer
            swarm.upsert_peer(peer.into(), &mut downloads_increased).await;
        }

        #[tokio::test]
        async fn it_should_trigger_an_event_when_a_peer_completes_a_download() {
            let info_hash = sample_info_hash();
            let started_peer = PeerBuilder::leecher().with_event(Started).build();
            let completed_peer = started_peer.into_completed();

            let mut event_sender_mock = MockEventSender::new();

            expect_event_sequence(
                &mut event_sender_mock,
                vec![
                    Event::PeerAdded {
                        info_hash,
                        peer: started_peer,
                    },
                    Event::PeerUpdated {
                        info_hash,
                        old_peer: started_peer,
                        new_peer: completed_peer,
                    },
                    Event::PeerDownloadCompleted {
                        info_hash,
                        peer: completed_peer,
                    },
                ],
            );

            let mut swarm = Swarm::new(&info_hash, 0, Some(Arc::new(event_sender_mock)));

            // Insert the peer
            let mut downloads_increased = false;
            swarm.upsert_peer(started_peer.into(), &mut downloads_increased).await;

            // Announce as completed
            swarm.upsert_peer(completed_peer.into(), &mut downloads_increased).await;
        }
    }
}
