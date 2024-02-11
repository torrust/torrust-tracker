use std::collections::BTreeMap;
use std::sync::Arc;

use futures::future::join_all;

use super::{Repository, UpdateTorrentAsync};
use crate::core::databases::PersistentTorrents;
use crate::core::services::torrent::Pagination;
use crate::core::torrent::entry::{self, ReadInfo, Update, UpdateAsync};
use crate::core::torrent::{SwarmMetadata, TorrentsRwLockTokioMutexTokio};
use crate::core::{peer, TorrentsMetrics, TrackerPolicy};
use crate::shared::bit_torrent::info_hash::InfoHash;

impl TorrentsRwLockTokioMutexTokio {
    async fn get_torrents<'a>(
        &'a self,
    ) -> tokio::sync::RwLockReadGuard<'a, std::collections::BTreeMap<InfoHash, entry::MutexTokio>>
    where
        std::collections::BTreeMap<InfoHash, entry::MutexTokio>: 'a,
    {
        self.torrents.read().await
    }

    async fn get_torrents_mut<'a>(
        &'a self,
    ) -> tokio::sync::RwLockWriteGuard<'a, std::collections::BTreeMap<InfoHash, entry::MutexTokio>>
    where
        std::collections::BTreeMap<InfoHash, entry::MutexTokio>: 'a,
    {
        self.torrents.write().await
    }
}

impl UpdateTorrentAsync for TorrentsRwLockTokioMutexTokio {
    async fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (bool, SwarmMetadata) {
        let maybe_entry = self.get_torrents().await.get(info_hash).cloned();

        let entry = if let Some(entry) = maybe_entry {
            entry
        } else {
            let mut db = self.get_torrents_mut().await;
            let entry = db.entry(*info_hash).or_insert(Arc::default());
            entry.clone()
        };

        entry.insert_or_update_peer_and_get_stats(peer).await
    }
}

impl UpdateTorrentAsync for Arc<TorrentsRwLockTokioMutexTokio> {
    async fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (bool, SwarmMetadata) {
        self.as_ref().update_torrent_with_peer_and_get_stats(info_hash, peer).await
    }
}

impl Repository<entry::MutexTokio> for TorrentsRwLockTokioMutexTokio {
    async fn get(&self, key: &InfoHash) -> Option<entry::MutexTokio> {
        let db = self.get_torrents().await;
        db.get(key).cloned()
    }

    async fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, entry::MutexTokio)> {
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
        let metrics: Arc<tokio::sync::Mutex<TorrentsMetrics>> = Arc::default();

        // todo:: replace with a ring buffer
        let mut handles = Vec::<tokio::task::JoinHandle<()>>::default();

        for e in self.get_torrents().await.values() {
            let entry = e.clone();
            let metrics = metrics.clone();
            handles.push(tokio::task::spawn(async move {
                let stats = entry.lock().await.get_stats();
                metrics.lock().await.seeders += u64::from(stats.complete);
                metrics.lock().await.completed += u64::from(stats.downloaded);
                metrics.lock().await.leechers += u64::from(stats.incomplete);
                metrics.lock().await.torrents += 1;
            }));
        }

        join_all(handles).await;

        *metrics.lock_owned().await
    }

    async fn import_persistent(&self, persistent_torrents: &PersistentTorrents) {
        let mut db = self.get_torrents_mut().await;

        for (info_hash, completed) in persistent_torrents {
            // Skip if torrent entry already exists
            if db.contains_key(info_hash) {
                continue;
            }

            let entry = entry::MutexTokio::new(
                entry::Single {
                    peers: BTreeMap::default(),
                    completed: *completed,
                }
                .into(),
            );

            db.insert(*info_hash, entry);
        }
    }

    async fn remove(&self, key: &InfoHash) -> Option<entry::MutexTokio> {
        let mut db = self.get_torrents_mut().await;
        db.remove(key)
    }

    async fn remove_inactive_peers(&self, max_peer_timeout: u32) {
        // todo:: replace with a ring buffer
        let mut handles = Vec::<tokio::task::JoinHandle<()>>::default();

        for e in self.get_torrents().await.values() {
            let entry = e.clone();
            handles.push(tokio::task::spawn(async move {
                entry.lock().await.remove_inactive_peers(max_peer_timeout);
            }));
        }

        join_all(handles).await;
    }

    async fn remove_peerless_torrents(&self, policy: &TrackerPolicy) {
        let mut db = self.get_torrents_mut().await;

        db.retain(|_, e| e.blocking_lock().is_not_zombie(policy));
    }
}
