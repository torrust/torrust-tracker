use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;

use aquatic_udp_protocol::AnnounceEvent;
use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::peer::{self};
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use super::peer_list::PeerList;

/// A data structure containing all the information about a torrent in the tracker.
///
/// This is the tracker entry for a given torrent and contains the swarm data,
/// that's the list of all the peers trying to download the same torrent.
/// The tracker keeps one entry like this for every torrent.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Torrent {
    /// A network of peers that are all trying to download the torrent associated to this entry
    pub(crate) swarm: PeerList,

    /// The number of peers that have ever completed downloading the torrent associated to this entry
    pub(crate) downloaded: u32,
}

impl Torrent {
    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn get_swarm_metadata(&self) -> SwarmMetadata {
        let (seeders, leechers) = self.swarm.seeders_and_leechers();

        SwarmMetadata {
            downloaded: self.downloaded,
            complete: seeders as u32,
            incomplete: leechers as u32,
        }
    }

    #[must_use]
    pub fn meets_retaining_policy(&self, policy: &TrackerPolicy) -> bool {
        if policy.persistent_torrent_completed_stat && self.downloaded > 0 {
            return true;
        }

        if policy.remove_peerless_torrents && self.swarm.is_empty() {
            return false;
        }

        true
    }

    #[must_use]
    pub fn peers_is_empty(&self) -> bool {
        self.swarm.is_empty()
    }

    #[must_use]
    pub fn get_peers_len(&self) -> usize {
        self.swarm.len()
    }

    #[must_use]
    pub fn get_peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.swarm.get_all(limit)
    }

    #[must_use]
    pub fn get_peers_for_client(&self, client: &SocketAddr, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.swarm.get_peers_excluding_addr(client, limit)
    }

    pub fn upsert_peer(&mut self, peer: &peer::Peer) -> bool {
        let mut number_of_downloads_increased: bool = false;

        match peer::ReadInfo::get_event(peer) {
            AnnounceEvent::Stopped => {
                drop(self.swarm.remove(&peer::ReadInfo::get_id(peer)));
            }
            AnnounceEvent::Completed => {
                let previous = self.swarm.upsert(Arc::new(*peer));
                // Don't count if peer was not previously known and not already completed.
                if previous.is_some_and(|p| p.event != AnnounceEvent::Completed) {
                    self.downloaded += 1;
                    number_of_downloads_increased = true;
                }
            }
            _ => {
                // `Started` event (first announced event) or
                // `None` event (announcements done at regular intervals).
                drop(self.swarm.upsert(Arc::new(*peer)));
            }
        }

        number_of_downloads_increased
    }

    pub fn remove_inactive_peers(&mut self, current_cutoff: DurationSinceUnixEpoch) {
        self.swarm.remove_inactive_peers(current_cutoff);
    }
}
