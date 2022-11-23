use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

use aquatic_udp_protocol::AnnounceEvent;
use serde::{Deserialize, Serialize};

use super::peer::TorrentPeer;
use crate::protocol::clock::{DefaultClock, TimeNow};
use crate::protocol::common::{PeerId, MAX_SCRAPE_TORRENTS};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TorrentEntry {
    #[serde(skip)]
    pub peers: std::collections::BTreeMap<PeerId, TorrentPeer>,
    pub completed: u32,
}

impl TorrentEntry {
    pub fn new() -> TorrentEntry {
        TorrentEntry {
            peers: std::collections::BTreeMap::new(),
            completed: 0,
        }
    }

    // Update peer and return completed (times torrent has been downloaded)
    pub fn update_peer(&mut self, peer: &TorrentPeer) -> bool {
        let mut did_torrent_stats_change: bool = false;

        match peer.event {
            AnnounceEvent::Stopped => {
                let _ = self.peers.remove(&peer.peer_id);
            }
            AnnounceEvent::Completed => {
                let peer_old = self.peers.insert(peer.peer_id.clone(), peer.clone());
                // Don't count if peer was not previously known
                if peer_old.is_some() {
                    self.completed += 1;
                    did_torrent_stats_change = true;
                }
            }
            _ => {
                let _ = self.peers.insert(peer.peer_id.clone(), peer.clone());
            }
        }

        did_torrent_stats_change
    }

    pub fn get_peers(&self, client_addr: Option<&SocketAddr>) -> Vec<&TorrentPeer> {
        self.peers
            .values()
            .filter(|peer| match client_addr {
                // Don't filter on ip_version
                None => true,
                // Filter out different ip_version from remote_addr
                Some(remote_addr) => {
                    // Skip ip address of client
                    if peer.peer_addr.ip() == remote_addr.ip() {
                        return false;
                    }

                    match peer.peer_addr.ip() {
                        IpAddr::V4(_) => remote_addr.is_ipv4(),
                        IpAddr::V6(_) => remote_addr.is_ipv6(),
                    }
                }
            })
            .take(MAX_SCRAPE_TORRENTS as usize)
            .collect()
    }

    pub fn get_stats(&self) -> (u32, u32, u32) {
        let seeders: u32 = self.peers.values().filter(|peer| peer.is_seeder()).count() as u32;
        let leechers: u32 = self.peers.len() as u32 - seeders;
        (seeders, self.completed, leechers)
    }

    pub fn remove_inactive_peers(&mut self, max_peer_timeout: u32) {
        let current_cutoff = DefaultClock::sub(&Duration::from_secs(max_peer_timeout as u64)).unwrap_or_default();
        self.peers.retain(|_, peer| peer.updated > current_cutoff);
    }
}

impl Default for TorrentEntry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct TorrentStats {
    pub completed: u32,
    pub seeders: u32,
    pub leechers: u32,
}

#[derive(Debug)]
pub enum TorrentError {
    TorrentNotWhitelisted,
    PeerNotAuthenticated,
    PeerKeyNotValid,
    NoPeersFound,
    CouldNotSendResponse,
    InvalidInfoHash,
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::ops::Sub;
    use std::time::Duration;

    use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};

    use crate::protocol::clock::{DefaultClock, DurationSinceUnixEpoch, StoppedClock, StoppedTime, Time, WorkingClock};
    use crate::protocol::common::PeerId;
    use crate::tracker::peer::TorrentPeer;
    use crate::tracker::torrent::TorrentEntry;

    struct TorrentPeerBuilder {
        peer: TorrentPeer,
    }

    impl TorrentPeerBuilder {
        pub fn default() -> TorrentPeerBuilder {
            let default_peer = TorrentPeer {
                peer_id: PeerId([0u8; 20]),
                peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
                updated: DefaultClock::now(),
                uploaded: NumberOfBytes(0),
                downloaded: NumberOfBytes(0),
                left: NumberOfBytes(0),
                event: AnnounceEvent::Started,
            };
            TorrentPeerBuilder { peer: default_peer }
        }

        pub fn with_event_completed(mut self) -> Self {
            self.peer.event = AnnounceEvent::Completed;
            self
        }

        pub fn with_peer_address(mut self, peer_addr: SocketAddr) -> Self {
            self.peer.peer_addr = peer_addr;
            self
        }

        pub fn with_peer_id(mut self, peer_id: PeerId) -> Self {
            self.peer.peer_id = peer_id;
            self
        }

        pub fn with_number_of_bytes_left(mut self, left: i64) -> Self {
            self.peer.left = NumberOfBytes(left);
            self
        }

        pub fn updated_at(mut self, updated: DurationSinceUnixEpoch) -> Self {
            self.peer.updated = updated;
            self
        }

        pub fn into(self) -> TorrentPeer {
            self.peer
        }
    }

    /// A torrent seeder is a peer with 0 bytes left to download which
    /// has not announced it has stopped
    fn a_torrent_seeder() -> TorrentPeer {
        TorrentPeerBuilder::default()
            .with_number_of_bytes_left(0)
            .with_event_completed()
            .into()
    }

    /// A torrent leecher is a peer that is not a seeder.
    /// Leecher: left > 0 OR event = Stopped
    fn a_torrent_leecher() -> TorrentPeer {
        TorrentPeerBuilder::default()
            .with_number_of_bytes_left(1)
            .with_event_completed()
            .into()
    }

    #[test]
    fn the_default_torrent_entry_should_contain_an_empty_list_of_peers() {
        let torrent_entry = TorrentEntry::new();

        assert_eq!(torrent_entry.get_peers(None).len(), 0);
    }

    #[test]
    fn a_new_peer_can_be_added_to_a_torrent_entry() {
        let mut torrent_entry = TorrentEntry::new();
        let torrent_peer = TorrentPeerBuilder::default().into();

        torrent_entry.update_peer(&torrent_peer); // Add the peer

        assert_eq!(*torrent_entry.get_peers(None)[0], torrent_peer);
        assert_eq!(torrent_entry.get_peers(None).len(), 1);
    }

    #[test]
    fn a_torrent_entry_should_contain_the_list_of_peers_that_were_added_to_the_torrent() {
        let mut torrent_entry = TorrentEntry::new();
        let torrent_peer = TorrentPeerBuilder::default().into();

        torrent_entry.update_peer(&torrent_peer); // Add the peer

        assert_eq!(torrent_entry.get_peers(None), vec![&torrent_peer]);
    }

    #[test]
    fn a_peer_can_be_updated_in_a_torrent_entry() {
        let mut torrent_entry = TorrentEntry::new();
        let mut torrent_peer = TorrentPeerBuilder::default().into();
        torrent_entry.update_peer(&torrent_peer); // Add the peer

        torrent_peer.event = AnnounceEvent::Completed; // Update the peer
        torrent_entry.update_peer(&torrent_peer); // Update the peer in the torrent entry

        assert_eq!(torrent_entry.get_peers(None)[0].event, AnnounceEvent::Completed);
    }

    #[test]
    fn a_peer_should_be_removed_from_a_torrent_entry_when_the_peer_announces_it_has_stopped() {
        let mut torrent_entry = TorrentEntry::new();
        let mut torrent_peer = TorrentPeerBuilder::default().into();
        torrent_entry.update_peer(&torrent_peer); // Add the peer

        torrent_peer.event = AnnounceEvent::Stopped; // Update the peer
        torrent_entry.update_peer(&torrent_peer); // Update the peer in the torrent entry

        assert_eq!(torrent_entry.get_peers(None).len(), 0);
    }

    #[test]
    fn torrent_stats_change_when_a_previously_known_peer_announces_it_has_completed_the_torrent() {
        let mut torrent_entry = TorrentEntry::new();
        let mut torrent_peer = TorrentPeerBuilder::default().into();

        torrent_entry.update_peer(&torrent_peer); // Add the peer

        torrent_peer.event = AnnounceEvent::Completed; // Update the peer
        let stats_have_changed = torrent_entry.update_peer(&torrent_peer); // Update the peer in the torrent entry

        assert!(stats_have_changed);
    }

    #[test]
    fn torrent_stats_should_not_change_when_a_peer_announces_it_has_completed_the_torrent_if_it_is_the_first_announce_from_the_peer(
    ) {
        let mut torrent_entry = TorrentEntry::new();
        let torrent_peer_announcing_complete_event = TorrentPeerBuilder::default().with_event_completed().into();

        // Add a peer that did not exist before in the entry
        let torrent_stats_have_not_changed = !torrent_entry.update_peer(&torrent_peer_announcing_complete_event);

        assert!(torrent_stats_have_not_changed);
    }

    #[test]
    fn a_torrent_entry_could_filter_out_peers_with_a_given_socket_address() {
        let mut torrent_entry = TorrentEntry::new();
        let peer_socket_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let torrent_peer = TorrentPeerBuilder::default().with_peer_address(peer_socket_address).into();
        torrent_entry.update_peer(&torrent_peer); // Add peer

        // Get peers excluding the one we have just added
        let peers = torrent_entry.get_peers(Some(&peer_socket_address));

        assert_eq!(peers.len(), 0);
    }

    fn peer_id_from_i32(number: i32) -> PeerId {
        let peer_id = number.to_le_bytes();
        PeerId([
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, peer_id[0], peer_id[1], peer_id[2],
            peer_id[3],
        ])
    }

    #[test]
    fn the_tracker_should_limit_the_list_of_peers_to_74_when_clients_scrape_torrents() {
        let mut torrent_entry = TorrentEntry::new();

        // We add one more peer than the scrape limit
        for peer_number in 1..=74 + 1 {
            let torrent_peer = TorrentPeerBuilder::default()
                .with_peer_id(peer_id_from_i32(peer_number))
                .into();
            torrent_entry.update_peer(&torrent_peer);
        }

        let peers = torrent_entry.get_peers(None);

        assert_eq!(peers.len(), 74)
    }

    #[test]
    fn torrent_stats_should_have_the_number_of_seeders_for_a_torrent() {
        let mut torrent_entry = TorrentEntry::new();
        let torrent_seeder = a_torrent_seeder();

        torrent_entry.update_peer(&torrent_seeder); // Add seeder

        assert_eq!(torrent_entry.get_stats().0, 1);
    }

    #[test]
    fn torrent_stats_should_have_the_number_of_leechers_for_a_torrent() {
        let mut torrent_entry = TorrentEntry::new();
        let torrent_leecher = a_torrent_leecher();

        torrent_entry.update_peer(&torrent_leecher); // Add leecher

        assert_eq!(torrent_entry.get_stats().2, 1);
    }

    #[test]
    fn torrent_stats_should_have_the_number_of_peers_that_having_announced_at_least_two_events_the_latest_one_is_the_completed_event(
    ) {
        let mut torrent_entry = TorrentEntry::new();
        let mut torrent_peer = TorrentPeerBuilder::default().into();
        torrent_entry.update_peer(&torrent_peer); // Add the peer

        // Announce "Completed" torrent download event.
        torrent_peer.event = AnnounceEvent::Completed;
        torrent_entry.update_peer(&torrent_peer); // Update the peer

        let number_of_previously_known_peers_with_completed_torrent = torrent_entry.get_stats().1;

        assert_eq!(number_of_previously_known_peers_with_completed_torrent, 1);
    }

    #[test]
    fn torrent_stats_should_not_include_a_peer_in_the_completed_counter_if_the_peer_has_announced_only_one_event() {
        let mut torrent_entry = TorrentEntry::new();
        let torrent_peer_announcing_complete_event = TorrentPeerBuilder::default().with_event_completed().into();

        // Announce "Completed" torrent download event.
        // It's the first event announced from this peer.
        torrent_entry.update_peer(&torrent_peer_announcing_complete_event); // Add the peer

        let number_of_peers_with_completed_torrent = torrent_entry.get_stats().1;

        assert_eq!(number_of_peers_with_completed_torrent, 0);
    }

    #[test]
    fn a_torrent_entry_should_remove_a_peer_not_updated_after_a_timeout_in_seconds() {
        let mut torrent_entry = TorrentEntry::new();

        let timeout = 120u32;

        let now = WorkingClock::now();
        StoppedClock::local_set(&now);

        let timeout_seconds_before_now = now.sub(Duration::from_secs(timeout as u64));
        let inactive_peer = TorrentPeerBuilder::default()
            .updated_at(timeout_seconds_before_now.sub(Duration::from_secs(1)))
            .into();
        torrent_entry.update_peer(&inactive_peer); // Add the peer

        torrent_entry.remove_inactive_peers(timeout);

        assert_eq!(torrent_entry.peers.len(), 0);
    }
}
