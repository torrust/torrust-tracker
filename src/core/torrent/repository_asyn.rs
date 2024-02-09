use std::sync::Arc;

use super::{EntryMutexStd, EntryMutexTokio, UpdateTorrentAsync};
use crate::core::peer;
use crate::core::torrent::{Entry, SwarmStats};
use crate::shared::bit_torrent::info_hash::InfoHash;

pub trait RepositoryAsync<T>: Default {
    fn get_torrents<'a>(
        &'a self,
    ) -> impl std::future::Future<Output = tokio::sync::RwLockReadGuard<'a, std::collections::BTreeMap<InfoHash, T>>> + Send
    where
        std::collections::BTreeMap<InfoHash, T>: 'a;

    fn get_torrents_mut<'a>(
        &'a self,
    ) -> impl std::future::Future<Output = tokio::sync::RwLockWriteGuard<'a, std::collections::BTreeMap<InfoHash, T>>> + Send
    where
        std::collections::BTreeMap<InfoHash, T>: 'a;
}

pub struct RepositoryTokioRwLock<T> {
    torrents: tokio::sync::RwLock<std::collections::BTreeMap<InfoHash, T>>,
}
impl UpdateTorrentAsync for RepositoryTokioRwLock<EntryMutexTokio> {
    async fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (SwarmStats, bool) {
        let maybe_existing_torrent_entry = self.get_torrents().await.get(info_hash).cloned();

        let torrent_entry: Arc<tokio::sync::Mutex<Entry>> = if let Some(existing_torrent_entry) = maybe_existing_torrent_entry {
            existing_torrent_entry
        } else {
            let mut torrents_lock = self.get_torrents_mut().await;
            let entry = torrents_lock
                .entry(*info_hash)
                .or_insert(Arc::new(tokio::sync::Mutex::new(Entry::new())));
            entry.clone()
        };

        let (stats, stats_updated) = {
            let mut torrent_entry_lock = torrent_entry.lock().await;
            let stats_updated = torrent_entry_lock.insert_or_update_peer(peer);
            let stats = torrent_entry_lock.get_stats();

            (stats, stats_updated)
        };

        (
            SwarmStats {
                downloaded: stats.1,
                complete: stats.0,
                incomplete: stats.2,
            },
            stats_updated,
        )
    }
}

impl RepositoryAsync<EntryMutexTokio> for RepositoryTokioRwLock<EntryMutexTokio> {
    async fn get_torrents<'a>(&'a self) -> tokio::sync::RwLockReadGuard<'a, std::collections::BTreeMap<InfoHash, EntryMutexTokio>>
    where
        std::collections::BTreeMap<InfoHash, EntryMutexTokio>: 'a,
    {
        self.torrents.read().await
    }

    async fn get_torrents_mut<'a>(
        &'a self,
    ) -> tokio::sync::RwLockWriteGuard<'a, std::collections::BTreeMap<InfoHash, EntryMutexTokio>>
    where
        std::collections::BTreeMap<InfoHash, EntryMutexTokio>: 'a,
    {
        self.torrents.write().await
    }
}

impl Default for RepositoryTokioRwLock<EntryMutexTokio> {
    fn default() -> Self {
        Self {
            torrents: tokio::sync::RwLock::default(),
        }
    }
}

impl UpdateTorrentAsync for RepositoryTokioRwLock<EntryMutexStd> {
    async fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (SwarmStats, bool) {
        let maybe_existing_torrent_entry = self.get_torrents().await.get(info_hash).cloned();

        let torrent_entry: Arc<std::sync::Mutex<Entry>> = if let Some(existing_torrent_entry) = maybe_existing_torrent_entry {
            existing_torrent_entry
        } else {
            let mut torrents_lock = self.get_torrents_mut().await;
            let entry = torrents_lock
                .entry(*info_hash)
                .or_insert(Arc::new(std::sync::Mutex::new(Entry::new())));
            entry.clone()
        };

        let (stats, stats_updated) = {
            let mut torrent_entry_lock = torrent_entry.lock().unwrap();
            let stats_updated = torrent_entry_lock.insert_or_update_peer(peer);
            let stats = torrent_entry_lock.get_stats();

            (stats, stats_updated)
        };

        (
            SwarmStats {
                downloaded: stats.1,
                complete: stats.0,
                incomplete: stats.2,
            },
            stats_updated,
        )
    }
}

impl RepositoryAsync<EntryMutexStd> for RepositoryTokioRwLock<EntryMutexStd> {
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

impl Default for RepositoryTokioRwLock<EntryMutexStd> {
    fn default() -> Self {
        Self {
            torrents: tokio::sync::RwLock::default(),
        }
    }
}

impl UpdateTorrentAsync for RepositoryTokioRwLock<Entry> {
    async fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (SwarmStats, bool) {
        let (stats, stats_updated) = {
            let mut torrents_lock = self.torrents.write().await;
            let torrent_entry = torrents_lock.entry(*info_hash).or_insert(Entry::new());
            let stats_updated = torrent_entry.insert_or_update_peer(peer);
            let stats = torrent_entry.get_stats();

            (stats, stats_updated)
        };

        (
            SwarmStats {
                downloaded: stats.1,
                complete: stats.0,
                incomplete: stats.2,
            },
            stats_updated,
        )
    }
}

impl RepositoryAsync<Entry> for RepositoryTokioRwLock<Entry> {
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

impl Default for RepositoryTokioRwLock<Entry> {
    fn default() -> Self {
        Self {
            torrents: tokio::sync::RwLock::default(),
        }
    }
}
