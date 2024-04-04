use std::collections::BTreeMap;
use std::sync::Arc;

use crossbeam_skiplist::SkipMap;
use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::info_hash::InfoHash;
use torrust_tracker_primitives::pagination::Pagination;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::torrent_metrics::TorrentsMetrics;
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch, PersistentTorrents};

use super::Repository;
use crate::entry::{Entry, EntrySync};
use crate::{EntryMutexStd, EntrySingle};

#[derive(Default, Debug)]
pub struct CrossbeamSkipList<T> {
    torrents: SkipMap<InfoHash, T>,
}

impl Repository<EntryMutexStd> for CrossbeamSkipList<EntryMutexStd>
where
    EntryMutexStd: EntrySync,
    EntrySingle: Entry,
{
    fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (bool, SwarmMetadata) {
        let entry = self.torrents.get_or_insert(*info_hash, Arc::default());
        entry.value().insert_or_update_peer_and_get_stats(peer)
    }

    fn get(&self, key: &InfoHash) -> Option<EntryMutexStd> {
        let maybe_entry = self.torrents.get(key);
        maybe_entry.map(|entry| entry.value().clone())
    }

    fn get_metrics(&self) -> TorrentsMetrics {
        let mut metrics = TorrentsMetrics::default();

        for entry in &self.torrents {
            let stats = entry.value().lock().expect("it should get a lock").get_stats();
            metrics.complete += u64::from(stats.complete);
            metrics.downloaded += u64::from(stats.downloaded);
            metrics.incomplete += u64::from(stats.incomplete);
            metrics.torrents += 1;
        }

        metrics
    }

    fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, EntryMutexStd)> {
        match pagination {
            Some(pagination) => self
                .torrents
                .iter()
                .skip(pagination.offset as usize)
                .take(pagination.limit as usize)
                .map(|entry| (*entry.key(), entry.value().clone()))
                .collect(),
            None => self
                .torrents
                .iter()
                .map(|entry| (*entry.key(), entry.value().clone()))
                .collect(),
        }
    }

    fn import_persistent(&self, persistent_torrents: &PersistentTorrents) {
        for (info_hash, completed) in persistent_torrents {
            if self.torrents.contains_key(info_hash) {
                continue;
            }

            let entry = EntryMutexStd::new(
                EntrySingle {
                    peers: BTreeMap::default(),
                    downloaded: *completed,
                }
                .into(),
            );

            // Since SkipMap is lock-free the torrent could have been inserted
            // after checking if it exists.
            self.torrents.get_or_insert(*info_hash, entry);
        }
    }

    fn remove(&self, key: &InfoHash) -> Option<EntryMutexStd> {
        self.torrents.remove(key).map(|entry| entry.value().clone())
    }

    fn remove_inactive_peers(&self, current_cutoff: DurationSinceUnixEpoch) {
        for entry in &self.torrents {
            entry.value().remove_inactive_peers(current_cutoff);
        }
    }

    fn remove_peerless_torrents(&self, policy: &TrackerPolicy) {
        for entry in &self.torrents {
            if entry.value().is_good(policy) {
                continue;
            }

            entry.remove();
        }
    }
}
