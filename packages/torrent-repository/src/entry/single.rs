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
    fn get_stats(&self) -> SwarmMetadata {
        let complete: u32 = self.peers.values().filter(|peer| peer.is_seeder()).count() as u32;
        let incomplete: u32 = self.peers.len() as u32 - complete;

        SwarmMetadata {
            downloaded: self.completed,
            complete,
            incomplete,
        }
    }

    fn is_not_zombie(&self, policy: &TrackerPolicy) -> bool {
        if policy.persistent_torrent_completed_stat && self.completed > 0 {
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
        match limit {
            Some(limit) => self.peers.values().take(limit).cloned().collect(),
            None => self.peers.values().cloned().collect(),
        }
    }

    fn get_peers_for_peer(&self, client: &peer::Peer, limit: Option<usize>) -> Vec<Arc<peer::Peer>> {
        match limit {
            Some(limit) => self
                .peers
                .values()
                // Take peers which are not the client peer
                .filter(|peer| peer::ReadInfo::get_address(peer.as_ref()) != peer::ReadInfo::get_address(client))
                // Limit the number of peers on the result
                .take(limit)
                .cloned()
                .collect(),
            None => self
                .peers
                .values()
                // Take peers which are not the client peer
                .filter(|peer| peer::ReadInfo::get_address(peer.as_ref()) != peer::ReadInfo::get_address(client))
                .cloned()
                .collect(),
        }
    }

    fn insert_or_update_peer(&mut self, peer: &peer::Peer) -> bool {
        let mut did_torrent_stats_change: bool = false;

        match peer::ReadInfo::get_event(peer) {
            AnnounceEvent::Stopped => {
                drop(self.peers.remove(&peer::ReadInfo::get_id(peer)));
            }
            AnnounceEvent::Completed => {
                let peer_old = self.peers.insert(peer::ReadInfo::get_id(peer), Arc::new(*peer));
                // Don't count if peer was not previously known and not already completed.
                if peer_old.is_some_and(|p| p.event != AnnounceEvent::Completed) {
                    self.completed += 1;
                    did_torrent_stats_change = true;
                }
            }
            _ => {
                drop(self.peers.insert(peer::ReadInfo::get_id(peer), Arc::new(*peer)));
            }
        }

        did_torrent_stats_change
    }

    fn insert_or_update_peer_and_get_stats(&mut self, peer: &peer::Peer) -> (bool, SwarmMetadata) {
        let changed = self.insert_or_update_peer(peer);
        let stats = self.get_stats();
        (changed, stats)
    }

    fn remove_inactive_peers(&mut self, current_cutoff: DurationSinceUnixEpoch) {
        self.peers
            .retain(|_, peer| peer::ReadInfo::get_updated(peer) > current_cutoff);
    }
}
