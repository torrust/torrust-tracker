use std::collections::BTreeMap;
use std::sync::Arc;

use futures::future::join_all;

use super::{Repository, UpdateTorrentAsync};
use crate::core::databases::PersistentTorrents;
use crate::core::services::torrent::Pagination;
use crate::core::torrent::entry::{Entry, ReadInfo, Update, UpdateAsync, UpdateSync};
use crate::core::torrent::{entry, SwarmMetadata};
use crate::core::{peer, TorrentsMetrics, TrackerPolicy};
use crate::shared::bit_torrent::info_hash::InfoHash;

#[derive(Default)]
pub struct RepositoryTokioRwLock<T: Default> {
    torrents: tokio::sync::RwLock<std::collections::BTreeMap<InfoHash, T>>,
}

impl RepositoryTokioRwLock<entry::MutexTokio> {
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

impl UpdateTorrentAsync for RepositoryTokioRwLock<entry::MutexTokio> {
    async fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (bool, SwarmMetadata) {
        let maybe_torrent;
        {
            let db = self.torrents.read().await;
            maybe_torrent = db.get(info_hash).cloned();
        }

        let torrent = if let Some(torrent) = maybe_torrent {
            torrent
        } else {
            let entry = entry::MutexTokio::default();
            let mut db = self.torrents.write().await;
            db.insert(*info_hash, entry.clone());
            entry
        };

        torrent.insert_or_update_peer_and_get_stats(peer).await
    }
}

impl Repository<entry::MutexTokio> for RepositoryTokioRwLock<entry::MutexTokio> {
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
        let db = self.get_torrents().await;
        let metrics: Arc<tokio::sync::Mutex<TorrentsMetrics>> = Arc::default();

        let futures = db.values().map(|e| {
            let metrics = metrics.clone();
            let entry = e.clone();

            tokio::spawn(async move {
                let stats = entry.lock().await.get_stats();
                metrics.lock().await.seeders += u64::from(stats.complete);
                metrics.lock().await.completed += u64::from(stats.downloaded);
                metrics.lock().await.leechers += u64::from(stats.incomplete);
                metrics.lock().await.torrents += 1;
            })
        });

        join_all(futures).await;

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
                Entry {
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
        let db = self.get_torrents().await;

        let futures = db.values().map(|e| {
            let entry = e.clone();
            tokio::spawn(async move { entry.lock().await.remove_inactive_peers(max_peer_timeout) })
        });

        join_all(futures).await;
    }

    async fn remove_peerless_torrents(&self, policy: &TrackerPolicy) {
        let mut db = self.get_torrents_mut().await;

        db.retain(|_, e| e.blocking_lock().is_not_zombie(policy));
    }
}

impl RepositoryTokioRwLock<entry::MutexStd> {
    async fn get_torrents<'a>(&'a self) -> tokio::sync::RwLockReadGuard<'a, std::collections::BTreeMap<InfoHash, entry::MutexStd>>
    where
        std::collections::BTreeMap<InfoHash, entry::MutexStd>: 'a,
    {
        self.torrents.read().await
    }

    async fn get_torrents_mut<'a>(
        &'a self,
    ) -> tokio::sync::RwLockWriteGuard<'a, std::collections::BTreeMap<InfoHash, entry::MutexStd>>
    where
        std::collections::BTreeMap<InfoHash, entry::MutexStd>: 'a,
    {
        self.torrents.write().await
    }
}

impl UpdateTorrentAsync for RepositoryTokioRwLock<entry::MutexStd> {
    async fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (bool, SwarmMetadata) {
        let maybe_torrent;
        {
            let db = self.torrents.read().await;
            maybe_torrent = db.get(info_hash).cloned();
        }

        let torrent = if let Some(torrent) = maybe_torrent {
            torrent
        } else {
            let entry = entry::MutexStd::default();
            let mut db = self.torrents.write().await;
            db.insert(*info_hash, entry.clone());
            entry
        };

        torrent.insert_or_update_peer_and_get_stats(peer)
    }
}

impl Repository<entry::MutexStd> for RepositoryTokioRwLock<entry::MutexStd> {
    async fn get(&self, key: &InfoHash) -> Option<entry::MutexStd> {
        let db = self.get_torrents().await;
        db.get(key).cloned()
    }

    async fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, entry::MutexStd)> {
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
        let db = self.get_torrents().await;
        let metrics: Arc<tokio::sync::Mutex<TorrentsMetrics>> = Arc::default();

        let futures = db.values().map(|e| {
            let metrics = metrics.clone();
            let entry = e.clone();

            tokio::spawn(async move {
                let stats = entry.lock().expect("it should lock the entry").get_stats();
                metrics.lock().await.seeders += u64::from(stats.complete);
                metrics.lock().await.completed += u64::from(stats.downloaded);
                metrics.lock().await.leechers += u64::from(stats.incomplete);
                metrics.lock().await.torrents += 1;
            })
        });

        join_all(futures).await;

        *metrics.lock_owned().await
    }

    async fn import_persistent(&self, persistent_torrents: &PersistentTorrents) {
        let mut torrents = self.get_torrents_mut().await;

        for (info_hash, completed) in persistent_torrents {
            // Skip if torrent entry already exists
            if torrents.contains_key(info_hash) {
                continue;
            }

            let entry = entry::MutexStd::new(
                Entry {
                    peers: BTreeMap::default(),
                    completed: *completed,
                }
                .into(),
            );

            torrents.insert(*info_hash, entry);
        }
    }

    async fn remove(&self, key: &InfoHash) -> Option<entry::MutexStd> {
        let mut db = self.get_torrents_mut().await;
        db.remove(key)
    }

    async fn remove_inactive_peers(&self, max_peer_timeout: u32) {
        let db = self.get_torrents().await;

        let futures = db.values().map(|e| {
            let entry = e.clone();
            tokio::spawn(async move {
                entry
                    .lock()
                    .expect("it should get lock for entry")
                    .remove_inactive_peers(max_peer_timeout);
            })
        });

        join_all(futures).await;
    }

    async fn remove_peerless_torrents(&self, policy: &TrackerPolicy) {
        let mut db = self.get_torrents_mut().await;

        db.retain(|_, e| e.lock().expect("it should lock entry").is_not_zombie(policy));
    }
}

impl RepositoryTokioRwLock<Entry> {
    async fn get_torrents<'a>(&'a self) -> tokio::sync::RwLockReadGuard<'a, std::collections::BTreeMap<InfoHash, Entry>>
    where
        std::collections::BTreeMap<InfoHash, Entry>: 'a,
    {
        self.torrents.read().await
    }

    async fn get_torrents_mut<'a>(&'a self) -> tokio::sync::RwLockWriteGuard<'a, std::collections::BTreeMap<InfoHash, Entry>>
    where
        std::collections::BTreeMap<InfoHash, Entry>: 'a,
    {
        self.torrents.write().await
    }
}

impl UpdateTorrentAsync for RepositoryTokioRwLock<Entry> {
    async fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (bool, SwarmMetadata) {
        let mut db = self.torrents.write().await;

        let torrent = db.entry(*info_hash).or_insert(Entry::default());

        torrent.insert_or_update_peer_and_get_stats(peer)
    }
}

impl Repository<Entry> for RepositoryTokioRwLock<Entry> {
    async fn get(&self, key: &InfoHash) -> Option<Entry> {
        let db = self.get_torrents().await;
        db.get(key).cloned()
    }

    async fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, Entry)> {
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
        let db = self.get_torrents().await;
        let metrics: Arc<tokio::sync::Mutex<TorrentsMetrics>> = Arc::default();

        let futures = db.values().map(|e| {
            let metrics = metrics.clone();
            let entry = e.clone();

            tokio::spawn(async move {
                let stats = entry.get_stats();
                metrics.lock().await.seeders += u64::from(stats.complete);
                metrics.lock().await.completed += u64::from(stats.downloaded);
                metrics.lock().await.leechers += u64::from(stats.incomplete);
                metrics.lock().await.torrents += 1;
            })
        });

        join_all(futures).await;

        *metrics.lock_owned().await
    }

    async fn import_persistent(&self, persistent_torrents: &PersistentTorrents) {
        let mut torrents = self.get_torrents_mut().await;

        for (info_hash, completed) in persistent_torrents {
            // Skip if torrent entry already exists
            if torrents.contains_key(info_hash) {
                continue;
            }

            let entry = Entry {
                peers: BTreeMap::default(),
                completed: *completed,
            };

            torrents.insert(*info_hash, entry);
        }
    }

    async fn remove(&self, key: &InfoHash) -> Option<Entry> {
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
