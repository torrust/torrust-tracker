use std::collections::BTreeMap;
use std::sync::Arc;

use futures::future::join_all;

use super::{Repository, UpdateTorrentAsync};
use crate::core::databases::PersistentTorrents;
use crate::core::services::torrent::Pagination;
use crate::core::torrent::entry::{self, ReadInfo, Update};
use crate::core::torrent::{SwarmMetadata, TorrentsRwLockTokio};
use crate::core::{peer, TorrentsMetrics, TrackerPolicy};
use crate::shared::bit_torrent::info_hash::InfoHash;

impl TorrentsRwLockTokio {
    async fn get_torrents<'a>(&'a self) -> tokio::sync::RwLockReadGuard<'a, std::collections::BTreeMap<InfoHash, entry::Single>>
    where
        std::collections::BTreeMap<InfoHash, entry::Single>: 'a,
    {
        self.torrents.read().await
    }

    async fn get_torrents_mut<'a>(
        &'a self,
    ) -> tokio::sync::RwLockWriteGuard<'a, std::collections::BTreeMap<InfoHash, entry::Single>>
    where
        std::collections::BTreeMap<InfoHash, entry::Single>: 'a,
    {
        self.torrents.write().await
    }
}

impl UpdateTorrentAsync for TorrentsRwLockTokio {
    async fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (bool, SwarmMetadata) {
        let mut db = self.get_torrents_mut().await;

        let entry = db.entry(*info_hash).or_insert(entry::Single::default());

        entry.insert_or_update_peer_and_get_stats(peer)
    }
}

impl UpdateTorrentAsync for Arc<TorrentsRwLockTokio> {
    async fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (bool, SwarmMetadata) {
        self.as_ref().update_torrent_with_peer_and_get_stats(info_hash, peer).await
    }
}

impl Repository<entry::Single> for TorrentsRwLockTokio {
    async fn get(&self, key: &InfoHash) -> Option<entry::Single> {
        let db = self.get_torrents().await;
        db.get(key).cloned()
    }

    async fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, entry::Single)> {
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

        let mut handles = Vec::<tokio::task::JoinHandle<()>>::default();

        for e in self.get_torrents().await.values() {
            let entry = e.clone();
            let metrics = metrics.clone();
            handles.push(tokio::task::spawn(async move {
                let stats = entry.get_stats();
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
        let mut torrents = self.get_torrents_mut().await;

        for (info_hash, completed) in persistent_torrents {
            // Skip if torrent entry already exists
            if torrents.contains_key(info_hash) {
                continue;
            }

            let entry = entry::Single {
                peers: BTreeMap::default(),
                completed: *completed,
            };

            torrents.insert(*info_hash, entry);
        }
    }

    async fn remove(&self, key: &InfoHash) -> Option<entry::Single> {
        let mut db = self.get_torrents_mut().await;
        db.remove(key)
    }

    async fn remove_inactive_peers(&self, max_peer_timeout: u32) {
        let mut db = self.get_torrents_mut().await;

        drop(db.values_mut().map(|e| e.remove_inactive_peers(max_peer_timeout)));
    }

    async fn remove_peerless_torrents(&self, policy: &TrackerPolicy) {
        let mut db = self.get_torrents_mut().await;

        db.retain(|_, e| e.is_not_zombie(policy));
    }
}
