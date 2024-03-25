use std::sync::{Arc, RwLock};

use super::EntryMutexStd;
use crate::core::peer;
use crate::core::torrent::{Entry, SwarmStats};
use crate::shared::bit_torrent::info_hash::InfoHash;

pub trait RepositorySync<T>: Default {
    fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (SwarmStats, bool);

    fn get_torrents<'a>(&'a self) -> std::sync::RwLockReadGuard<'a, std::collections::BTreeMap<InfoHash, T>>
    where
        std::collections::BTreeMap<InfoHash, T>: 'a;

    fn get_torrents_mut<'a>(&'a self) -> std::sync::RwLockWriteGuard<'a, std::collections::BTreeMap<InfoHash, T>>
    where
        std::collections::BTreeMap<InfoHash, T>: 'a;
}

pub struct RepositoryStdRwLock<T> {
    torrents: std::sync::RwLock<std::collections::BTreeMap<InfoHash, T>>,
}

impl RepositorySync<EntryMutexStd> for RepositoryStdRwLock<EntryMutexStd> {
    fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (SwarmStats, bool) {
        let maybe_existing_torrent_entry = self.get_torrents().get(info_hash).cloned();

        let torrent_entry: Arc<std::sync::Mutex<Entry>> = if let Some(existing_torrent_entry) = maybe_existing_torrent_entry {
            existing_torrent_entry
        } else {
            let mut torrents_lock = self.get_torrents_mut();
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

    fn get_torrents<'a>(&'a self) -> std::sync::RwLockReadGuard<'a, std::collections::BTreeMap<InfoHash, EntryMutexStd>>
    where
        std::collections::BTreeMap<InfoHash, EntryMutexStd>: 'a,
    {
        self.torrents.read().expect("unable to get torrent list")
    }

    fn get_torrents_mut<'a>(&'a self) -> std::sync::RwLockWriteGuard<'a, std::collections::BTreeMap<InfoHash, EntryMutexStd>>
    where
        std::collections::BTreeMap<InfoHash, EntryMutexStd>: 'a,
    {
        self.torrents.write().expect("unable to get writable torrent list")
    }
}

impl Default for RepositoryStdRwLock<EntryMutexStd> {
    fn default() -> Self {
        Self {
            torrents: RwLock::default(),
        }
    }
}

impl RepositorySync<Entry> for RepositoryStdRwLock<Entry> {
    fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (SwarmStats, bool) {
        let mut torrents = self.torrents.write().unwrap();

        let torrent_entry = match torrents.entry(*info_hash) {
            std::collections::btree_map::Entry::Vacant(vacant) => vacant.insert(Entry::new()),
            std::collections::btree_map::Entry::Occupied(entry) => entry.into_mut(),
        };

        let stats_updated = torrent_entry.insert_or_update_peer(peer);
        let stats = torrent_entry.get_stats();

        (
            SwarmStats {
                downloaded: stats.1,
                complete: stats.0,
                incomplete: stats.2,
            },
            stats_updated,
        )
    }

    fn get_torrents<'a>(&'a self) -> std::sync::RwLockReadGuard<'a, std::collections::BTreeMap<InfoHash, Entry>>
    where
        std::collections::BTreeMap<InfoHash, Entry>: 'a,
    {
        self.torrents.read().expect("unable to get torrent list")
    }

    fn get_torrents_mut<'a>(&'a self) -> std::sync::RwLockWriteGuard<'a, std::collections::BTreeMap<InfoHash, Entry>>
    where
        std::collections::BTreeMap<InfoHash, Entry>: 'a,
    {
        self.torrents.write().expect("unable to get writable torrent list")
    }
}

impl Default for RepositoryStdRwLock<Entry> {
    fn default() -> Self {
        Self {
            torrents: RwLock::default(),
        }
    }
}
