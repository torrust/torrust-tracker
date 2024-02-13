use std::collections::BTreeMap;
use std::sync::Arc;

use torrust_tracker_configuration::TrackerPolicy;
use torrust_tracker_primitives::info_hash::InfoHash;
use torrust_tracker_primitives::pagination::Pagination;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::torrent_metrics::TorrentsMetrics;
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch, PersistentTorrents};

use super::RepositoryAsync;
use crate::entry::{Entry, EntrySync};
use crate::{EntryMutexStd, EntrySingle, TorrentsRwLockTokioMutexStd};

impl TorrentsRwLockTokioMutexStd {
    async fn get_torrents<'a>(&'a self) -> tokio::sync::RwLockReadGuard<'a, std::collections::BTreeMap<InfoHash, EntryMutexStd>>
    where
        std::collections::BTreeMap<InfoHash, EntryMutexStd>: 'a,
    {
        self.torrents.read().await
    }

    async fn get_torrents_mut<'a>(
        &'a self,
    ) -> tokio::sync::RwLockWriteGuard<'a, std::collections::BTreeMap<InfoHash, EntryMutexStd>>
    where
        std::collections::BTreeMap<InfoHash, EntryMutexStd>: 'a,
    {
        self.torrents.write().await
    }
}

impl RepositoryAsync<EntryMutexStd> for TorrentsRwLockTokioMutexStd
where
    EntryMutexStd: EntrySync,
    EntrySingle: Entry,
{
    async fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (bool, SwarmMetadata) {
        let maybe_entry = self.get_torrents().await.get(info_hash).cloned();

        let entry = if let Some(entry) = maybe_entry {
            entry
        } else {
            let mut db = self.get_torrents_mut().await;
            let entry = db.entry(*info_hash).or_insert(Arc::default());
            entry.clone()
        };

        entry.insert_or_update_peer_and_get_stats(peer)
    }
    async fn get(&self, key: &InfoHash) -> Option<EntryMutexStd> {
        let db = self.get_torrents().await;
        db.get(key).cloned()
    }

    async fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, EntryMutexStd)> {
        let db = self.get_torrents().await;

        match pagination {
            Some(pagination) => db
                .iter()
                .skip(pagination.offset as usize)
                .take(pagination.limit as usize)
                .map(|(a, b)| (*a, b.clone()))
                .collect(),
            None => db.iter().map(|(a, b)| (*a, b.clone())).collect(),
        }
    }

    async fn get_metrics(&self) -> TorrentsMetrics {
        let mut metrics = TorrentsMetrics::default();

        for entry in self.get_torrents().await.values() {
            let stats = entry.get_stats();
            metrics.seeders += u64::from(stats.complete);
            metrics.completed += u64::from(stats.downloaded);
            metrics.leechers += u64::from(stats.incomplete);
            metrics.torrents += 1;
        }

        metrics
    }

    async fn import_persistent(&self, persistent_torrents: &PersistentTorrents) {
        let mut torrents = self.get_torrents_mut().await;

        for (info_hash, completed) in persistent_torrents {
            // Skip if torrent entry already exists
            if torrents.contains_key(info_hash) {
                continue;
            }

            let entry = EntryMutexStd::new(
                EntrySingle {
                    peers: BTreeMap::default(),
                    completed: *completed,
                }
                .into(),
            );

            torrents.insert(*info_hash, entry);
        }
    }

    async fn remove(&self, key: &InfoHash) -> Option<EntryMutexStd> {
        let mut db = self.get_torrents_mut().await;
        db.remove(key)
    }

    async fn remove_inactive_peers(&self, current_cutoff: DurationSinceUnixEpoch) {
        let db = self.get_torrents().await;
        let entries = db.values().cloned();

        for entry in entries {
            entry.remove_inactive_peers(current_cutoff);
        }
    }

    async fn remove_peerless_torrents(&self, policy: &TrackerPolicy) {
        let mut db = self.get_torrents_mut().await;

        db.retain(|_, e| e.lock().expect("it should lock entry").is_not_zombie(policy));
    }
}
