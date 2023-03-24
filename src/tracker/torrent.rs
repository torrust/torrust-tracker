//! Structs to store the swarm data.
//!
//! There are to main data structures:
//!
//! - A torrent [`Entry`](crate::tracker::torrent::Entry): it contains all the information stored by the tracker for one torrent.
//! - The [`SwarmMetadata`](crate::tracker::torrent::SwarmMetadata): it contains aggregate information that can me derived from the torrent entries.
//!
//! A "swarm" is a network of peers that are trying to download the same torrent.
//!
//! The torrent entry contains the "swarm" data, which is basically the list of peers in the swarm.
//! That's the most valuable information the peer want to get from the tracker, because it allows them to
//! start downloading torrent from those peers.
//!
//! > **NOTICE**: that both swarm data (torrent entries) and swarm metadata (aggregate counters) are related to only one torrent.
//!
//! The "swarm metadata" contains aggregate data derived from the torrent entries. There two types of data:
//!
//! - For **active peers**: metrics related to the current active peers in the swarm.
//! - **Historical data**: since the tracker started running.
//!
//! The tracker collects metrics for:
//!
//! - The number of peers that have completed downloading the torrent since the tracker started collecting metrics.
//! - The number of peers that have completed downloading the torrent and are still active, that means they are actively participating in the network,
//! by announcing themselves periodically to the tracker. Since they have completed downloading they have a full copy of the torrent data. Peers with a
//! full copy of the data are called "seeders".
//! - The number of peers that have NOT completed downloading the torrent and are still active, that means they are actively participating in the network.
//! Peer that don not have a full copy of the torrent data are called "leechers".
//!
//! > **NOTICE**: that both [`SwarmMetadata`](crate::tracker::torrent::SwarmMetadata) and [`SwarmStats`](crate::tracker::torrent::SwarmStats) contain the same information. [`SwarmMetadata`](crate::tracker::torrent::SwarmMetadata) is using the names used on [BEP 48: Tracker Protocol Extension: Scrape](https://www.bittorrent.org/beps/bep_0048.html).
use std::time::Duration;

use aquatic_udp_protocol::AnnounceEvent;
use serde::{Deserialize, Serialize};

use super::peer::{self, Peer};
use crate::shared::bit_torrent::common::MAX_SCRAPE_TORRENTS;
use crate::shared::clock::{Current, TimeNow};

/// A data structure containing all the information about a torrent in the tracker.
///
/// This is the tracker entry for a given torrent and contains the swarm data,
/// that's the list of all the peers trying to download the same torrent.
/// The tracker keeps one entry like this for every torrent.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Entry {
    /// The swarm: a network of peers that are all trying to download the torrent associated to this entry
    #[serde(skip)]
    pub peers: std::collections::BTreeMap<peer::Id, peer::Peer>,
    /// The number of peers that have ever completed downloading the torrent associated to this entry
    pub completed: u32,
}

/// Swarm statistics for one torrent.
/// Swarm metadata dictionary in the scrape response.
///
/// See [BEP 48: Tracker Protocol Extension: Scrape](https://www.bittorrent.org/beps/bep_0048.html)
#[derive(Debug, PartialEq, Default)]
pub struct SwarmMetadata {
    /// The number of peers that have ever completed downloading
    pub downloaded: u32,

    /// The number of active peers that have completed downloading (seeders)
    pub complete: u32,
    /// The number of active peers that have not completed downloading (leechers)
    pub incomplete: u32,
}

impl SwarmMetadata {
    #[must_use]
    pub fn zeroed() -> Self {
        Self::default()
    }
}

/// Swarm statistics for one torrent.
///
/// See [BEP 48: Tracker Protocol Extension: Scrape](https://www.bittorrent.org/beps/bep_0048.html)
#[derive(Debug, PartialEq, Default)]
pub struct SwarmStats {
    /// The number of peers that have ever completed downloading
    pub completed: u32,

    /// The number of active peers that have completed downloading (seeders)
    pub seeders: u32,
    /// The number of active peers that have not completed downloading (leechers)
    pub leechers: u32,
}

impl Entry {
    #[must_use]
    pub fn new() -> Entry {
        Entry {
            peers: std::collections::BTreeMap::new(),
            completed: 0,
        }
    }

    /// It updates a peer and returns true if the number of complete downloads have increased.
    ///
    /// The number of peers that have complete downloading is synchronously updated when peers are updated.
    /// That's the total torrent downloads counter.
    pub fn update_peer(&mut self, peer: &peer::Peer) -> bool {
        let mut did_torrent_stats_change: bool = false;

        match peer.event {
            AnnounceEvent::Stopped => {
                let _ = self.peers.remove(&peer.peer_id);
            }
            AnnounceEvent::Completed => {
                let peer_old = self.peers.insert(peer.peer_id, *peer);
                // Don't count if peer was not previously known
                if peer_old.is_some() {
                    self.completed += 1;
                    did_torrent_stats_change = true;
                }
            }
            _ => {
                let _ = self.peers.insert(peer.peer_id, *peer);
            }
        }

        did_torrent_stats_change
    }

    /// Get all swarm peers, limiting the result to the maximum number of scrape
    /// torrents.
    #[must_use]
    pub fn get_all_peers(&self) -> Vec<&peer::Peer> {
        self.peers.values().take(MAX_SCRAPE_TORRENTS as usize).collect()
    }

    /// It returns the list of peers for a given peer client, limiting the
    /// result to the maximum number of scrape torrents.
    ///
    /// It filters out the input peer, typically because we want to return this
    /// list of peers to that client peer.
    #[must_use]
    pub fn get_peers_for_peer(&self, client: &Peer) -> Vec<&peer::Peer> {
        self.peers
            .values()
            // Take peers which are not the client peer
            .filter(|peer| peer.peer_addr != client.peer_addr)
            // Limit the number of peers on the result
            .take(MAX_SCRAPE_TORRENTS as usize)
            .collect()
    }

    /// It returns the swarm metadata (statistics) as a tuple:
    ///
    /// `(seeders, completed, leechers)`
    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn get_stats(&self) -> (u32, u32, u32) {
        let seeders: u32 = self.peers.values().filter(|peer| peer.is_seeder()).count() as u32;
        let leechers: u32 = self.peers.len() as u32 - seeders;
        (seeders, self.completed, leechers)
    }

    /// It returns the swarm metadata (statistics) as an struct
    #[must_use]
    pub fn get_swarm_metadata(&self) -> SwarmMetadata {
        // code-review: consider using always this function instead of `get_stats`.
        let (seeders, completed, leechers) = self.get_stats();
        SwarmMetadata {
            complete: seeders,
            downloaded: completed,
            incomplete: leechers,
        }
    }

    /// It removes peer from the swarm that have not been updated for more than `max_peer_timeout` seconds
    pub fn remove_inactive_peers(&mut self, max_peer_timeout: u32) {
        let current_cutoff = Current::sub(&Duration::from_secs(u64::from(max_peer_timeout))).unwrap_or_default();
        self.peers.retain(|_, peer| peer.updated > current_cutoff);
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    mod torrent_entry {

        use std::net::{IpAddr, Ipv4Addr, SocketAddr};
        use std::ops::Sub;
        use std::time::Duration;

        use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};

        use crate::shared::clock::{Current, DurationSinceUnixEpoch, Stopped, StoppedTime, Time, Working};
        use crate::tracker::peer;
        use crate::tracker::torrent::Entry;

        struct TorrentPeerBuilder {
            peer: peer::Peer,
        }

        impl TorrentPeerBuilder {
            pub fn default() -> TorrentPeerBuilder {
                let default_peer = peer::Peer {
                    peer_id: peer::Id([0u8; 20]),
                    peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
                    updated: Current::now(),
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

            pub fn with_peer_id(mut self, peer_id: peer::Id) -> Self {
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

            pub fn into(self) -> peer::Peer {
                self.peer
            }
        }

        /// A torrent seeder is a peer with 0 bytes left to download which
        /// has not announced it has stopped
        fn a_torrent_seeder() -> peer::Peer {
            TorrentPeerBuilder::default()
                .with_number_of_bytes_left(0)
                .with_event_completed()
                .into()
        }

        /// A torrent leecher is a peer that is not a seeder.
        /// Leecher: left > 0 OR event = Stopped
        fn a_torrent_leecher() -> peer::Peer {
            TorrentPeerBuilder::default()
                .with_number_of_bytes_left(1)
                .with_event_completed()
                .into()
        }

        #[test]
        fn the_default_torrent_entry_should_contain_an_empty_list_of_peers() {
            let torrent_entry = Entry::new();

            assert_eq!(torrent_entry.get_all_peers().len(), 0);
        }

        #[test]
        fn a_new_peer_can_be_added_to_a_torrent_entry() {
            let mut torrent_entry = Entry::new();
            let torrent_peer = TorrentPeerBuilder::default().into();

            torrent_entry.update_peer(&torrent_peer); // Add the peer

            assert_eq!(*torrent_entry.get_all_peers()[0], torrent_peer);
            assert_eq!(torrent_entry.get_all_peers().len(), 1);
        }

        #[test]
        fn a_torrent_entry_should_contain_the_list_of_peers_that_were_added_to_the_torrent() {
            let mut torrent_entry = Entry::new();
            let torrent_peer = TorrentPeerBuilder::default().into();

            torrent_entry.update_peer(&torrent_peer); // Add the peer

            assert_eq!(torrent_entry.get_all_peers(), vec![&torrent_peer]);
        }

        #[test]
        fn a_peer_can_be_updated_in_a_torrent_entry() {
            let mut torrent_entry = Entry::new();
            let mut torrent_peer = TorrentPeerBuilder::default().into();
            torrent_entry.update_peer(&torrent_peer); // Add the peer

            torrent_peer.event = AnnounceEvent::Completed; // Update the peer
            torrent_entry.update_peer(&torrent_peer); // Update the peer in the torrent entry

            assert_eq!(torrent_entry.get_all_peers()[0].event, AnnounceEvent::Completed);
        }

        #[test]
        fn a_peer_should_be_removed_from_a_torrent_entry_when_the_peer_announces_it_has_stopped() {
            let mut torrent_entry = Entry::new();
            let mut torrent_peer = TorrentPeerBuilder::default().into();
            torrent_entry.update_peer(&torrent_peer); // Add the peer

            torrent_peer.event = AnnounceEvent::Stopped; // Update the peer
            torrent_entry.update_peer(&torrent_peer); // Update the peer in the torrent entry

            assert_eq!(torrent_entry.get_all_peers().len(), 0);
        }

        #[test]
        fn torrent_stats_change_when_a_previously_known_peer_announces_it_has_completed_the_torrent() {
            let mut torrent_entry = Entry::new();
            let mut torrent_peer = TorrentPeerBuilder::default().into();

            torrent_entry.update_peer(&torrent_peer); // Add the peer

            torrent_peer.event = AnnounceEvent::Completed; // Update the peer
            let stats_have_changed = torrent_entry.update_peer(&torrent_peer); // Update the peer in the torrent entry

            assert!(stats_have_changed);
        }

        #[test]
        fn torrent_stats_should_not_change_when_a_peer_announces_it_has_completed_the_torrent_if_it_is_the_first_announce_from_the_peer(
        ) {
            let mut torrent_entry = Entry::new();
            let torrent_peer_announcing_complete_event = TorrentPeerBuilder::default().with_event_completed().into();

            // Add a peer that did not exist before in the entry
            let torrent_stats_have_not_changed = !torrent_entry.update_peer(&torrent_peer_announcing_complete_event);

            assert!(torrent_stats_have_not_changed);
        }

        #[test]
        fn a_torrent_entry_should_return_the_list_of_peers_for_a_given_peer_filtering_out_the_client_that_is_making_the_request()
        {
            let mut torrent_entry = Entry::new();
            let peer_socket_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
            let torrent_peer = TorrentPeerBuilder::default().with_peer_address(peer_socket_address).into();
            torrent_entry.update_peer(&torrent_peer); // Add peer

            // Get peers excluding the one we have just added
            let peers = torrent_entry.get_peers_for_peer(&torrent_peer);

            assert_eq!(peers.len(), 0);
        }

        #[test]
        fn two_peers_with_the_same_ip_but_different_port_should_be_considered_different_peers() {
            let mut torrent_entry = Entry::new();

            let peer_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

            // Add peer 1
            let torrent_peer_1 = TorrentPeerBuilder::default()
                .with_peer_address(SocketAddr::new(peer_ip, 8080))
                .into();
            torrent_entry.update_peer(&torrent_peer_1);

            // Add peer 2
            let torrent_peer_2 = TorrentPeerBuilder::default()
                .with_peer_address(SocketAddr::new(peer_ip, 8081))
                .into();
            torrent_entry.update_peer(&torrent_peer_2);

            // Get peers for peer 1
            let peers = torrent_entry.get_peers_for_peer(&torrent_peer_1);

            // The peer 2 using the same IP but different port should be included
            assert_eq!(peers[0].peer_addr.ip(), Ipv4Addr::new(127, 0, 0, 1));
            assert_eq!(peers[0].peer_addr.port(), 8081);
        }

        fn peer_id_from_i32(number: i32) -> peer::Id {
            let peer_id = number.to_le_bytes();
            peer::Id([
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, peer_id[0], peer_id[1],
                peer_id[2], peer_id[3],
            ])
        }

        #[test]
        fn the_tracker_should_limit_the_list_of_peers_to_74_when_clients_scrape_torrents() {
            let mut torrent_entry = Entry::new();

            // We add one more peer than the scrape limit
            for peer_number in 1..=74 + 1 {
                let torrent_peer = TorrentPeerBuilder::default()
                    .with_peer_id(peer_id_from_i32(peer_number))
                    .into();
                torrent_entry.update_peer(&torrent_peer);
            }

            let peers = torrent_entry.get_all_peers();

            assert_eq!(peers.len(), 74);
        }

        #[test]
        fn torrent_stats_should_have_the_number_of_seeders_for_a_torrent() {
            let mut torrent_entry = Entry::new();
            let torrent_seeder = a_torrent_seeder();

            torrent_entry.update_peer(&torrent_seeder); // Add seeder

            assert_eq!(torrent_entry.get_stats().0, 1);
        }

        #[test]
        fn torrent_stats_should_have_the_number_of_leechers_for_a_torrent() {
            let mut torrent_entry = Entry::new();
            let torrent_leecher = a_torrent_leecher();

            torrent_entry.update_peer(&torrent_leecher); // Add leecher

            assert_eq!(torrent_entry.get_stats().2, 1);
        }

        #[test]
        fn torrent_stats_should_have_the_number_of_peers_that_having_announced_at_least_two_events_the_latest_one_is_the_completed_event(
        ) {
            let mut torrent_entry = Entry::new();
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
            let mut torrent_entry = Entry::new();
            let torrent_peer_announcing_complete_event = TorrentPeerBuilder::default().with_event_completed().into();

            // Announce "Completed" torrent download event.
            // It's the first event announced from this peer.
            torrent_entry.update_peer(&torrent_peer_announcing_complete_event); // Add the peer

            let number_of_peers_with_completed_torrent = torrent_entry.get_stats().1;

            assert_eq!(number_of_peers_with_completed_torrent, 0);
        }

        #[test]
        fn a_torrent_entry_should_remove_a_peer_not_updated_after_a_timeout_in_seconds() {
            let mut torrent_entry = Entry::new();

            let timeout = 120u32;

            let now = Working::now();
            Stopped::local_set(&now);

            let timeout_seconds_before_now = now.sub(Duration::from_secs(u64::from(timeout)));
            let inactive_peer = TorrentPeerBuilder::default()
                .updated_at(timeout_seconds_before_now.sub(Duration::from_secs(1)))
                .into();
            torrent_entry.update_peer(&inactive_peer); // Add the peer

            torrent_entry.remove_inactive_peers(timeout);

            assert_eq!(torrent_entry.peers.len(), 0);
        }
    }
}
