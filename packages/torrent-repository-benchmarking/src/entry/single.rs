use std::net::SocketAddr;
use std::sync::Arc;

use aquatic_udp_protocol::AnnounceEvent;
use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::peer::{self};
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use super::Entry;
use crate::EntrySingle;

impl Entry for EntrySingle {
    #[allow(clippy::cast_possible_truncation)]
    fn get_swarm_metadata(&self) -> SwarmMetadata {
        let (seeders, leechers) = self.swarm.seeders_and_leechers();

        SwarmMetadata {
            downloaded: self.downloaded,
            complete: seeders as u32,
            incomplete: leechers as u32,
        }
    }

    fn meets_retaining_policy(&self, policy: &TrackerPolicy) -> bool {
        if policy.persistent_torrent_completed_stat && self.downloaded > 0 {
            return true;
        }

        if policy.remove_peerless_torrents && self.swarm.is_empty() {
            return false;
        }

        true
    }

    fn peers_is_empty(&self) -> bool {
        self.swarm.is_empty()
    }

    fn get_peers_len(&self) -> usize {
        self.swarm.len()
    }

    fn get_peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.swarm.get_all(limit)
    }

    fn get_peers_for_client(&self, client: &SocketAddr, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.swarm.get_peers_excluding_addr(client, limit)
    }

    fn upsert_peer(&mut self, peer: &peer::Peer) -> bool {
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

    fn remove_inactive_peers(&mut self, current_cutoff: DurationSinceUnixEpoch) {
        self.swarm.remove_inactive_peers(current_cutoff);
    }
}
