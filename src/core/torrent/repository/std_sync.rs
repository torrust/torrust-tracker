use std::collections::BTreeMap;
use std::sync::Arc;

use futures::executor::block_on;
use futures::future::join_all;

use super::{Repository, UpdateTorrentAsync, UpdateTorrentSync};
use crate::core::databases::PersistentTorrents;
use crate::core::services::torrent::Pagination;
use crate::core::torrent::entry::{Entry, ReadInfo, Update, UpdateAsync, UpdateSync};
use crate::core::torrent::{entry, SwarmMetadata};
use crate::core::{peer, TorrentsMetrics};
use crate::shared::bit_torrent::info_hash::InfoHash;

#[derive(Default)]
pub struct RepositoryStdRwLock<T> {
    torrents: std::sync::RwLock<std::collections::BTreeMap<InfoHash, T>>,
}

impl RepositoryStdRwLock<entry::MutexTokio> {
    fn get_torrents<'a>(&'a self) -> std::sync::RwLockReadGuard<'a, std::collections::BTreeMap<InfoHash, entry::MutexTokio>>
    where
        std::collections::BTreeMap<InfoHash, entry::MutexTokio>: 'a,
    {
        self.torrents.read().expect("unable to get torrent list")
    }

    fn get_torrents_mut<'a>(&'a self) -> std::sync::RwLockWriteGuard<'a, std::collections::BTreeMap<InfoHash, entry::MutexTokio>>
    where
        std::collections::BTreeMap<InfoHash, entry::MutexTokio>: 'a,
    {
        self.torrents.write().expect("unable to get writable torrent list")
    }
}

impl UpdateTorrentAsync for RepositoryStdRwLock<entry::MutexTokio> {
    async fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (bool, SwarmMetadata) {
        let maybe_existing_torrent_entry = self.get_torrents().get(info_hash).cloned();

        let torrent_entry = if let Some(existing_torrent_entry) = maybe_existing_torrent_entry {
            existing_torrent_entry
        } else {
            let mut torrents_lock = self.get_torrents_mut();
            let entry = torrents_lock.entry(*info_hash).or_insert(Arc::default());
            entry.clone()
        };

        torrent_entry.insert_or_update_peer_and_get_stats(peer).await
    }
}
impl Repository<entry::MutexTokio> for RepositoryStdRwLock<entry::MutexTokio> {
    async fn get(&self, key: &InfoHash) -> Option<entry::MutexTokio> {
        let db = self.get_torrents();
        db.get(key).cloned()
    }

    async fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, entry::MutexTokio)> {
        let db = self.get_torrents();

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
        let db = self.get_torrents();
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

        block_on(join_all(futures));

        *metrics.blocking_lock_owned()
    }

    async fn import_persistent(&self, persistent_torrents: &PersistentTorrents) {
        let mut db = self.get_torrents_mut();

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
        let mut db = self.get_torrents_mut();
        db.remove(key)
    }

    async fn remove_inactive_peers(&self, max_peer_timeout: u32) {
        let db = self.get_torrents();

        let futures = db.values().map(|e| {
            let entry = e.clone();
            tokio::spawn(async move { entry.lock().await.remove_inactive_peers(max_peer_timeout) })
        });

        block_on(join_all(futures));
    }

    async fn remove_peerless_torrents(&self, policy: &crate::core::TrackerPolicy) {
        let mut db = self.get_torrents_mut();

        db.retain(|_, e| e.blocking_lock().is_not_zombie(policy));
    }
}

impl RepositoryStdRwLock<entry::MutexStd> {
    fn get_torrents<'a>(&'a self) -> std::sync::RwLockReadGuard<'a, std::collections::BTreeMap<InfoHash, entry::MutexStd>>
    where
        std::collections::BTreeMap<InfoHash, entry::MutexStd>: 'a,
    {
        self.torrents.read().expect("unable to get torrent list")
    }

    fn get_torrents_mut<'a>(&'a self) -> std::sync::RwLockWriteGuard<'a, std::collections::BTreeMap<InfoHash, entry::MutexStd>>
    where
        std::collections::BTreeMap<InfoHash, entry::MutexStd>: 'a,
    {
        self.torrents.write().expect("unable to get writable torrent list")
    }
}

impl UpdateTorrentSync for RepositoryStdRwLock<entry::MutexStd> {
    fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (bool, SwarmMetadata) {
        let maybe_existing_torrent_entry = self.get_torrents().get(info_hash).cloned();

        let torrent_entry: Arc<std::sync::Mutex<Entry>> = if let Some(existing_torrent_entry) = maybe_existing_torrent_entry {
            existing_torrent_entry
        } else {
            let mut torrents_lock = self.get_torrents_mut();
            let entry = torrents_lock
                .entry(*info_hash)
                .or_insert(Arc::new(std::sync::Mutex::new(Entry::default())));
            entry.clone()
        };

        torrent_entry.insert_or_update_peer_and_get_stats(peer)
    }
}
impl Repository<entry::MutexStd> for RepositoryStdRwLock<entry::MutexStd> {
    async fn get(&self, key: &InfoHash) -> Option<entry::MutexStd> {
        let db = self.get_torrents();
        db.get(key).cloned()
    }

    async fn get_metrics(&self) -> TorrentsMetrics {
        let db = self.get_torrents();
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

        block_on(join_all(futures));

        *metrics.blocking_lock_owned()
    }

    async fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, entry::MutexStd)> {
        let db = self.get_torrents();

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

    async fn import_persistent(&self, persistent_torrents: &PersistentTorrents) {
        let mut torrents = self.get_torrents_mut();

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
        let mut db = self.get_torrents_mut();
        db.remove(key)
    }

    async fn remove_inactive_peers(&self, max_peer_timeout: u32) {
        let db = self.get_torrents();

        let futures = db.values().map(|e| {
            let entry = e.clone();
            tokio::spawn(async move {
                entry
                    .lock()
                    .expect("it should get lock for entry")
                    .remove_inactive_peers(max_peer_timeout);
            })
        });

        block_on(join_all(futures));
    }

    async fn remove_peerless_torrents(&self, policy: &crate::core::TrackerPolicy) {
        let mut db = self.get_torrents_mut();

        db.retain(|_, e| e.lock().expect("it should lock entry").is_not_zombie(policy));
    }
}

impl RepositoryStdRwLock<Entry> {
    fn get_torrents<'a>(&'a self) -> std::sync::RwLockReadGuard<'a, std::collections::BTreeMap<InfoHash, Entry>>
    where
        std::collections::BTreeMap<InfoHash, Entry>: 'a,
    {
        self.torrents.read().expect("it should get the read lock")
    }

    fn get_torrents_mut<'a>(&'a self) -> std::sync::RwLockWriteGuard<'a, std::collections::BTreeMap<InfoHash, Entry>>
    where
        std::collections::BTreeMap<InfoHash, Entry>: 'a,
    {
        self.torrents.write().expect("it should get the write lock")
    }
}

impl UpdateTorrentSync for RepositoryStdRwLock<Entry> {
    fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (bool, SwarmMetadata) {
        let mut torrents = self.torrents.write().unwrap();

        let torrent_entry = match torrents.entry(*info_hash) {
            std::collections::btree_map::Entry::Vacant(vacant) => vacant.insert(Entry::default()),
            std::collections::btree_map::Entry::Occupied(entry) => entry.into_mut(),
        };

        torrent_entry.insert_or_update_peer_and_get_stats(peer)
    }
}
impl Repository<Entry> for RepositoryStdRwLock<Entry> {
    async fn get(&self, key: &InfoHash) -> Option<Entry> {
        let db = self.get_torrents();
        db.get(key).cloned()
    }

    async fn get_metrics(&self) -> TorrentsMetrics {
        let db = self.get_torrents();
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

        block_on(join_all(futures));

        *metrics.blocking_lock_owned()
    }

    async fn get_paginated(&self, pagination: Option<&Pagination>) -> Vec<(InfoHash, Entry)> {
        let db = self.get_torrents();

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

    async fn import_persistent(&self, persistent_torrents: &PersistentTorrents) {
        let mut torrents = self.get_torrents_mut();

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
        let mut db = self.get_torrents_mut();
        db.remove(key)
    }

    async fn remove_inactive_peers(&self, max_peer_timeout: u32) {
        let mut db = self.get_torrents_mut();

        drop(db.values_mut().map(|e| e.remove_inactive_peers(max_peer_timeout)));
    }

    async fn remove_peerless_torrents(&self, policy: &crate::core::TrackerPolicy) {
        let mut db = self.get_torrents_mut();

        db.retain(|_, e| e.is_not_zombie(policy));
    }
}
