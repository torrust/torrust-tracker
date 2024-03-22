use std::sync::Arc;

use crate::core::peer::Peer;
use crate::core::torrent::{Entry, SwarmStats};
use crate::core::{peer, TORRENT_PEERS_LIMIT};
use crate::shared::bit_torrent::info_hash::InfoHash;

pub trait Repository {
    fn new() -> Self;
    fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (SwarmStats, bool, Vec<Peer>);
}

pub trait TRepositoryAsync {
    fn new() -> Self;
    fn announce(
        &self,
        info_hash: &InfoHash,
        peer: &peer::Peer,
    ) -> impl std::future::Future<Output = (SwarmStats, bool, Vec<Peer>)> + Send;
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

    fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (SwarmStats, bool, Vec<Peer>) {
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

        let (stats, stats_updated, peers) = {
            let mut torrent_entry_lock = torrent_entry.lock().unwrap();
            let stats_updated = torrent_entry_lock.insert_or_update_peer(peer);
            let stats = torrent_entry_lock.get_stats();
            let peers: Vec<Peer> = torrent_entry_lock
                .get_peers_for_peer(peer, TORRENT_PEERS_LIMIT)
                .into_iter()
                .copied()
                .collect();

            (stats, stats_updated, peers)
        };

        (
            SwarmStats {
                downloaded: stats.1,
                complete: stats.0,
                incomplete: stats.2,
            },
            stats_updated,
            peers,
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

    fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (SwarmStats, bool, Vec<Peer>) {
        let mut torrents = self.torrents.write().unwrap();

        let torrent_entry = match torrents.entry(*info_hash) {
            std::collections::btree_map::Entry::Vacant(vacant) => vacant.insert(Entry::new()),
            std::collections::btree_map::Entry::Occupied(entry) => entry.into_mut(),
        };

        let stats_updated = torrent_entry.insert_or_update_peer(peer);
        let stats = torrent_entry.get_stats();
        let peers: Vec<Peer> = torrent_entry
            .get_peers_for_peer(peer, TORRENT_PEERS_LIMIT)
            .into_iter()
            .copied()
            .collect();

        (
            SwarmStats {
                downloaded: stats.1,
                complete: stats.0,
                incomplete: stats.2,
            },
            stats_updated,
            peers,
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

    async fn announce(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (SwarmStats, bool, Vec<Peer>) {
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

        let (stats, stats_updated, peers) = {
            let mut torrent_entry_lock = torrent_entry.lock().await;
            let stats_updated = torrent_entry_lock.insert_or_update_peer(peer);
            let stats = torrent_entry_lock.get_stats();
            let peers: Vec<Peer> = torrent_entry_lock
                .get_peers_for_peer(peer, TORRENT_PEERS_LIMIT)
                .into_iter()
                .copied()
                .collect();

            (stats, stats_updated, peers)
        };

        (
            SwarmStats {
                downloaded: stats.1,
                complete: stats.0,
                incomplete: stats.2,
            },
            stats_updated,
            peers,
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

    async fn announce(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (SwarmStats, bool, Vec<Peer>) {
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

        let (stats, stats_updated, peers) = {
            let mut torrent_entry_lock = torrent_entry.lock().unwrap();
            let stats_updated = torrent_entry_lock.insert_or_update_peer(peer);
            let stats = torrent_entry_lock.get_stats();
            let peers: Vec<Peer> = torrent_entry_lock
                .get_peers_for_peer(peer, TORRENT_PEERS_LIMIT)
                .into_iter()
                .copied()
                .collect();

            (stats, stats_updated, peers)
        };

        (
            SwarmStats {
                downloaded: stats.1,
                complete: stats.0,
                incomplete: stats.2,
            },
            stats_updated,
            peers,
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

    async fn announce(&self, info_hash: &InfoHash, peer: &peer::Peer) -> (SwarmStats, bool, Vec<Peer>) {
        let stats_updated = {
            let mut torrents_write_lock = self.torrents.write().await;
            let torrent_entry = torrents_write_lock.entry(*info_hash).or_insert(Entry::new());
            torrent_entry.insert_or_update_peer(peer)
        };

        let (stats, peers) = {
            let torrents_read_lock = self.torrents.read().await;

            match torrents_read_lock.get(info_hash) {
                None => (SwarmStats::zeroed(), vec![]),
                Some(entry) => {
                    let stats = entry.get_swarm_metadata();
                    let peers: Vec<Peer> = entry
                        .get_peers_for_peer(peer, TORRENT_PEERS_LIMIT)
                        .into_iter()
                        .copied()
                        .collect();

                    (stats, peers)
                }
            }
        };

        (stats, stats_updated, peers)
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
