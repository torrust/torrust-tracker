use std::net::SocketAddr;
use std::sync::Arc;

use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::announce_event::AnnounceEvent;
use torrust_tracker_primitives::peer::{self};
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use super::Entry;
use crate::EntrySingle;

impl Entry for EntrySingle {
    #[allow(clippy::cast_possible_truncation)]
    fn get_swarm_metadata(&self) -> SwarmMetadata {
        let (seeders, leechers) = self.peers.seeders_and_leechers();

        SwarmMetadata {
            downloaded: self.downloaded,
            complete: seeders as u32,
            incomplete: leechers as u32,
        }
    }

    fn is_good(&self, policy: &TrackerPolicy) -> bool {
        if policy.persistent_torrent_completed_stat && self.downloaded > 0 {
            return true;
        }

        if policy.remove_peerless_torrents && self.peers.is_empty() {
            return false;
        }

        true
    }

    fn peers_is_empty(&self) -> bool {
        self.peers.is_empty()
    }

    fn get_peers_len(&self) -> usize {
        self.peers.len()
    }

    fn get_peers(&self, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.peers.get_peers(limit)
    }

    fn get_peers_for_client(&self, client: &SocketAddr, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        self.peers.get_peers_for_client(client, limit)
    }

    fn upsert_peer(&mut self, peer: &peer::Peer) -> bool {
        let mut downloaded_stats_updated: bool = false;

        match peer::ReadInfo::get_event(peer) {
            AnnounceEvent::Stopped => {
                drop(self.peers.remove(&peer::ReadInfo::get_id(peer)));
            }
            AnnounceEvent::Completed => {
                let previous = self.peers.insert(peer::ReadInfo::get_id(peer), Arc::new(*peer));
                // Don't count if peer was not previously known and not already completed.
                if previous.is_some_and(|p| p.event != AnnounceEvent::Completed) {
                    self.downloaded += 1;
                    downloaded_stats_updated = true;
                }
            }
            _ => {
                drop(self.peers.insert(peer::ReadInfo::get_id(peer), Arc::new(*peer)));
            }
        }

        downloaded_stats_updated
    }

    fn remove_inactive_peers(&mut self, current_cutoff: DurationSinceUnixEpoch) {
        self.peers
            .retain(|_, peer| peer::ReadInfo::get_updated(peer) > current_cutoff);
    }
}
