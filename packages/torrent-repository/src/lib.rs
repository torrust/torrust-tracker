use std::collections::BTreeMap;
use std::sync::Arc;

use crossbeam_skiplist::SkipMap;
use repository::dash_map_mutex_std::XacrimonDashMap;
use repository::rw_lock_std::RwLockStd;
use repository::rw_lock_tokio::RwLockTokio;
use repository::skip_map_mutex_std::CrossbeamSkipList;
use torrust_tracker_clock::clock;
use torrust_tracker_primitives::peer;

pub mod entry;
pub mod repository;

// Peer List

pub type BTreeMapPeerList = BTreeMap<peer::Id, Arc<peer::Peer>>;
pub type SkipMapPeerList = SkipMap<peer::Id, Arc<peer::Peer>>;

// Torrent Entry

pub type EntrySingle<T> = entry::Torrent<T>;
pub type EntryMutexStd<T> = Arc<std::sync::Mutex<EntrySingle<T>>>;
pub type EntryMutexTokio<T> = Arc<tokio::sync::Mutex<EntrySingle<T>>>;

// Repos

// Torrent repo and peer list: BTreeMap
pub type TorrentsRwLockStd = RwLockStd<EntrySingle<BTreeMapPeerList>>;
pub type TorrentsRwLockStdMutexStd = RwLockStd<EntryMutexStd<BTreeMapPeerList>>;
pub type TorrentsRwLockStdMutexTokio = RwLockStd<EntryMutexTokio<BTreeMapPeerList>>;
pub type TorrentsRwLockTokio = RwLockTokio<EntrySingle<BTreeMapPeerList>>;
pub type TorrentsRwLockTokioMutexStd = RwLockTokio<EntryMutexStd<BTreeMapPeerList>>;
pub type TorrentsRwLockTokioMutexTokio = RwLockTokio<EntryMutexTokio<BTreeMapPeerList>>;

// Torrent repo: SkipMap; Peer list: BTreeMap
pub type TorrentsSkipMapMutexStd = CrossbeamSkipList<EntryMutexStd<BTreeMapPeerList>>;

// Torrent repo: DashMap; Peer list: BTreeMap
pub type TorrentsDashMapMutexStd = XacrimonDashMap<EntryMutexStd<BTreeMapPeerList>>;

// Torrent repo and peer list: SkipMap
pub type TorrentsSkipMapMutexStdSkipMap = CrossbeamSkipList<EntryMutexStd<SkipMapPeerList>>;

/// This code needs to be copied into each crate.
/// Working version, for production.
#[cfg(not(test))]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Working;

/// Stopped version, for testing.
#[cfg(test)]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Stopped;
