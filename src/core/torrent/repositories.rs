use std::collections::VecDeque;
use std::iter;
use std::mem::size_of;
use std::sync::{Arc, Mutex};

use dashmap::{DashMap, Map, SharedValue};

use crate::core::peer;
use crate::core::torrent::{Entry, SwarmStats};
use crate::shared::bit_torrent::info_hash::InfoHash;
use crate::shared::mem_size::{MemSize, POINTER_SIZE};

// todo: Make this a config option. Through env?
const MAX_MEMORY_LIMIT: Option<usize> = Some(8_000_000_000); // 8GB

const INFO_HASH_SIZE: usize = size_of::<InfoHash>();

/// Total memory impact of adding a new empty torrent ([torrent::Entry]) to a map.
const TORRENT_INSERTION_SIZE_COST: usize = 216;

/// Total memory impact of adding a new peer ([peer::Peer]) to a map.
const _PEER_INSERTION_SIZE_COST: usize = 132;

pub trait Repository {
    fn new() -> Self;

    /// Updates or inserts a torrent with a peer and returns the torrent statistics.
    fn upsert_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (SwarmStats, bool);
}

pub trait TRepositoryAsync {
    fn new() -> Self;

    /// Updates or inserts a torrent with a peer and returns the torrent statistics.
    fn upsert_torrent_with_peer_and_get_stats(
        &self,
        info_hash: &InfoHash,
        peer: &peer::Peer,
    ) -> impl std::future::Future<Output = (SwarmStats, bool)> + Send;
}

/// Structure that holds all torrents. Using `std::sync` locks.
pub struct Sync {
    torrents: std::sync::RwLock<std::collections::BTreeMap<InfoHash, Arc<std::sync::Mutex<Entry>>>>,
}

impl Sync {
    /// Returns the get torrents of this [`Sync`].
    ///
    /// # Panics
    ///
    /// Panics if unable to read the torrent.
    pub fn get_torrents(
        &self,
    ) -> std::sync::RwLockReadGuard<'_, std::collections::BTreeMap<InfoHash, Arc<std::sync::Mutex<Entry>>>> {
        self.torrents.read().expect("unable to get torrent list")
    }

    /// Returns the mutable get torrents of this [`Sync`].
    ///
    /// # Panics
    ///
    /// Panics if unable to write to the torrents list.
    pub fn get_torrents_mut(
        &self,
    ) -> std::sync::RwLockWriteGuard<'_, std::collections::BTreeMap<InfoHash, Arc<std::sync::Mutex<Entry>>>> {
        self.torrents.write().expect("unable to get writable torrent list")
    }
}

impl Repository for Sync {
    fn new() -> Self {
        Self {
            torrents: std::sync::RwLock::new(std::collections::BTreeMap::new()),
        }
    }

    fn upsert_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (SwarmStats, bool) {
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
}

/// Structure that holds all torrents. Using `std::sync` locks.
pub struct SyncSingle {
    torrents: std::sync::RwLock<std::collections::BTreeMap<InfoHash, Entry>>,
}

impl SyncSingle {
    /// Returns the get torrents of this [`SyncSingle`].
    ///
    /// # Panics
    ///
    /// Panics if unable to get torrent list.
    pub fn get_torrents(&self) -> std::sync::RwLockReadGuard<'_, std::collections::BTreeMap<InfoHash, Entry>> {
        self.torrents.read().expect("unable to get torrent list")
    }

    /// Returns the get torrents of this [`SyncSingle`].
    ///
    /// # Panics
    ///
    /// Panics if unable to get writable torrent list.
    pub fn get_torrents_mut(&self) -> std::sync::RwLockWriteGuard<'_, std::collections::BTreeMap<InfoHash, Entry>> {
        self.torrents.write().expect("unable to get writable torrent list")
    }
}

impl Repository for SyncSingle {
    fn new() -> Self {
        Self {
            torrents: std::sync::RwLock::new(std::collections::BTreeMap::new()),
        }
    }

    fn upsert_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (SwarmStats, bool) {
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
}

/// Structure that holds all torrents. Using `tokio::sync` locks.
#[allow(clippy::module_name_repetitions)]
pub struct RepositoryAsync {
    torrents: tokio::sync::RwLock<std::collections::BTreeMap<InfoHash, Arc<tokio::sync::Mutex<Entry>>>>,
}

impl TRepositoryAsync for RepositoryAsync {
    fn new() -> Self {
        Self {
            torrents: tokio::sync::RwLock::new(std::collections::BTreeMap::new()),
        }
    }

    async fn upsert_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (SwarmStats, bool) {
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

impl RepositoryAsync {
    pub async fn get_torrents(
        &self,
    ) -> tokio::sync::RwLockReadGuard<'_, std::collections::BTreeMap<InfoHash, Arc<tokio::sync::Mutex<Entry>>>> {
        self.torrents.read().await
    }

    pub async fn get_torrents_mut(
        &self,
    ) -> tokio::sync::RwLockWriteGuard<'_, std::collections::BTreeMap<InfoHash, Arc<tokio::sync::Mutex<Entry>>>> {
        self.torrents.write().await
    }
}

/// Structure that holds all torrents. Using a `tokio::sync` lock for the torrents map an`std::sync`nc lock for the inner torrent entry.
pub struct AsyncSync {
    torrents: tokio::sync::RwLock<std::collections::BTreeMap<InfoHash, Arc<std::sync::Mutex<Entry>>>>,
}

impl TRepositoryAsync for AsyncSync {
    fn new() -> Self {
        Self {
            torrents: tokio::sync::RwLock::new(std::collections::BTreeMap::new()),
        }
    }

    async fn upsert_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (SwarmStats, bool) {
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

impl AsyncSync {
    pub async fn get_torrents(
        &self,
    ) -> tokio::sync::RwLockReadGuard<'_, std::collections::BTreeMap<InfoHash, Arc<std::sync::Mutex<Entry>>>> {
        self.torrents.read().await
    }

    pub async fn get_torrents_mut(
        &self,
    ) -> tokio::sync::RwLockWriteGuard<'_, std::collections::BTreeMap<InfoHash, Arc<std::sync::Mutex<Entry>>>> {
        self.torrents.write().await
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct RepositoryAsyncSingle {
    torrents: tokio::sync::RwLock<std::collections::BTreeMap<InfoHash, Entry>>,
}

impl TRepositoryAsync for RepositoryAsyncSingle {
    fn new() -> Self {
        Self {
            torrents: tokio::sync::RwLock::new(std::collections::BTreeMap::new()),
        }
    }

    async fn upsert_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (SwarmStats, bool) {
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

impl RepositoryAsyncSingle {
    pub async fn get_torrents(&self) -> tokio::sync::RwLockReadGuard<'_, std::collections::BTreeMap<InfoHash, Entry>> {
        self.torrents.read().await
    }

    pub async fn get_torrents_mut(&self) -> tokio::sync::RwLockWriteGuard<'_, std::collections::BTreeMap<InfoHash, Entry>> {
        self.torrents.write().await
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct RepositoryDashmap {
    pub torrents: DashMap<InfoHash, Entry>,
    pub shard_priority_list: Vec<Mutex<VecDeque<InfoHash>>>,
    pub shard_locks: Vec<Mutex<()>>,
}

impl MemSize for RepositoryDashmap {
    fn get_mem_size(&self) -> usize {
        const MAP_SIZE: usize = size_of::<DashMap<InfoHash, Entry>>();

        let mut total_size_of_entries: usize = 0;

        for rm in self.torrents.iter() {
            // Add 2 times the POINTER_SIZE for a pointer to the key as String and value as Entry
            let entry_size = (2 * POINTER_SIZE) + INFO_HASH_SIZE + rm.value().get_mem_size();
            total_size_of_entries += entry_size;
        }

        MAP_SIZE + total_size_of_entries
    }
}

impl RepositoryDashmap {
    fn get_index_of_torrent_on_shard_priority_list(&self, shard_idx: usize, info_hash: &InfoHash) -> Option<usize> {
        let priority_list = unsafe { self.shard_priority_list.get_unchecked(shard_idx) }.lock().unwrap();

        let mut index = None;

        for (i, torrent) in priority_list.iter().enumerate() {
            if torrent == info_hash {
                index = Some(i);
            }
        }

        index
    }

    fn addshift_torrent_to_front_on_shard_priority_list(&self, shard_idx: usize, info_hash: &InfoHash) {
        let maybe_index = self.get_index_of_torrent_on_shard_priority_list(shard_idx, info_hash);

        let mut priority_list = self.shard_priority_list.get(shard_idx).unwrap().lock().unwrap();

        if let Some(index) = maybe_index {
            let _torrent = priority_list.remove(index);
        }

        priority_list.push_front(info_hash.to_owned());
    }
}

impl Repository for RepositoryDashmap {
    fn new() -> Self {
        let torrents = DashMap::new();

        // Keep a priority order per shard to prevent locking the entire map when checking and freeing memory.
        let shard_priority_list = iter::repeat_with(|| Mutex::new(VecDeque::new()))
            .take(torrents._shard_count())
            .collect();

        let shard_locks = iter::repeat_with(|| Mutex::new(())).take(torrents._shard_count()).collect();

        Self {
            torrents,
            shard_priority_list,
            shard_locks,
        }
    }

    fn upsert_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (SwarmStats, bool) {
        let hash = self.torrents.hash_usize(&info_hash);
        let shard_idx = self.torrents.determine_shard(hash);
        let mut shard = unsafe { self.torrents._yield_write_shard(shard_idx) };

        let mut torrent = shard.remove(info_hash).map(|v| v.into_inner()).unwrap_or_default();

        let stats_updated = torrent.insert_or_update_peer(peer);
        let stats = torrent.get_stats();

        let mut mem_size_shard: usize = 0;

        for torrent in shard.values() {
            mem_size_shard += (2 * POINTER_SIZE) + INFO_HASH_SIZE + torrent.get().get_mem_size();
        }

        let maybe_max_memory_available = MAX_MEMORY_LIMIT.map(|v| v / self.torrents._shard_count() - mem_size_shard);

        let memory_shortage = maybe_max_memory_available
            .map(|v| TORRENT_INSERTION_SIZE_COST.saturating_sub(v))
            .unwrap_or(0);

        if memory_shortage > 0 {
            let mut amount_freed: usize = 0;

            let mut priority_list = unsafe { self.shard_priority_list.get_unchecked(shard_idx) }.lock().unwrap();

            while amount_freed < memory_shortage && !priority_list.is_empty() {
                // Can safely unwrap as we check if the priority list is not empty
                let torrent_hash_to_be_removed = priority_list.pop_back().unwrap();

                if let Some(torrent) = shard.remove(&torrent_hash_to_be_removed) {
                    amount_freed += torrent.get().get_mem_size();
                }
            }
        }

        self.addshift_torrent_to_front_on_shard_priority_list(shard_idx, info_hash);

        shard
            .insert(info_hash.to_owned(), SharedValue::new(torrent))
            .map(|v| v.into_inner());

        drop(shard);

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

#[cfg(test)]
pub mod tests {
    use deepsize::DeepSizeOf;

    use crate::core::peer;
    use crate::core::torrent::repositories::{Repository, RepositoryDashmap};
    use crate::shared::bit_torrent::info_hash::InfoHash;
    use crate::shared::mem_size::MemSize;

    #[test]
    fn torrent_repository_should_have_runtime_memory_size_of() {
        let torrent_repository = RepositoryDashmap::new();

        let info_hash_1 = InfoHash([0u8; 20]);
        let info_hash_2 = InfoHash([1u8; 20]);

        let torrent_peer_1 = crate::core::torrent::tests::torrent_entry::TorrentPeerBuilder::default()
            .with_peer_id(peer::Id([0u8; 20]))
            .into();

        let torrent_peer_2 = crate::core::torrent::tests::torrent_entry::TorrentPeerBuilder::default()
            .with_peer_id(peer::Id([1u8; 20]))
            .into();

        assert_eq!(torrent_repository.get_mem_size(), 40);

        torrent_repository.upsert_torrent_with_peer_and_get_stats(&info_hash_1, &torrent_peer_1);

        assert_eq!(torrent_repository.get_mem_size(), 256);

        torrent_repository.upsert_torrent_with_peer_and_get_stats(&info_hash_2, &torrent_peer_2);

        assert_eq!(torrent_repository.get_mem_size(), 472);

        let torrent_entry_1 = torrent_repository.torrents.get(&info_hash_1).unwrap();

        assert_eq!(torrent_entry_1.get_mem_size(), 180);

        {
            let mut torrent_entry_2 = torrent_repository.torrents.get_mut(&info_hash_2).unwrap();

            assert_eq!(torrent_entry_2.get_mem_size(), 180);

            torrent_entry_2.insert_or_update_peer(&torrent_peer_1);

            assert_eq!(torrent_entry_2.get_mem_size(), 312);
        }

        assert_eq!(torrent_peer_1.deep_size_of(), 192);

        assert_eq!(torrent_repository.get_mem_size(), 604);

        torrent_repository.torrents.remove(&info_hash_2);

        assert_eq!(torrent_repository.get_mem_size(), 256);
    }

    #[test]
    fn torrent_should_have_priority_index_of_0() {
        let torrent_repository = RepositoryDashmap::new();

        let info_hash_1 = InfoHash([0u8; 20]);
        let info_hash_2 = InfoHash([1u8; 20]);

        let torrent_peer_1 = crate::core::torrent::tests::torrent_entry::TorrentPeerBuilder::default()
            .with_peer_id(peer::Id([0u8; 20]))
            .into();

        let torrent_peer_2 = crate::core::torrent::tests::torrent_entry::TorrentPeerBuilder::default()
            .with_peer_id(peer::Id([1u8; 20]))
            .into();

        torrent_repository.upsert_torrent_with_peer_and_get_stats(&info_hash_1, &torrent_peer_1);
        torrent_repository.upsert_torrent_with_peer_and_get_stats(&info_hash_2, &torrent_peer_2);

        let hash = torrent_repository.torrents.hash_usize(&info_hash_2);
        let shard_idx = torrent_repository.torrents.determine_shard(hash);

        let maybe_priority_idx = torrent_repository.get_index_of_torrent_on_shard_priority_list(shard_idx, &info_hash_2);

        assert_eq!(maybe_priority_idx, Some(0))
    }
}
