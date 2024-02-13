use std::sync::Arc;

pub mod entry;
pub mod repository;

pub type EntrySingle = entry::Torrent;
pub type EntryMutexStd = Arc<std::sync::Mutex<entry::Torrent>>;
pub type EntryMutexTokio = Arc<tokio::sync::Mutex<entry::Torrent>>;

pub type TorrentsRwLockStd = repository::RwLockStd<EntrySingle>;
pub type TorrentsRwLockStdMutexStd = repository::RwLockStd<EntryMutexStd>;
pub type TorrentsRwLockStdMutexTokio = repository::RwLockStd<EntryMutexTokio>;
pub type TorrentsRwLockTokio = repository::RwLockTokio<EntrySingle>;
pub type TorrentsRwLockTokioMutexStd = repository::RwLockTokio<EntryMutexStd>;
pub type TorrentsRwLockTokioMutexTokio = repository::RwLockTokio<EntryMutexTokio>;
