use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::Arc;

use log::info;
use tokio::sync::{RwLock, RwLockReadGuard};
use tokio::sync::mpsc::error::SendError;

use crate::Configuration;
use crate::protocol::common::InfoHash;
use crate::databases::database::Database;
use crate::databases::database;
use crate::mode::TrackerMode;
use crate::peer::TorrentPeer;
use crate::tracker::key::AuthKey;
use crate::tracker::key::Error::KeyInvalid;
use crate::statistics::{StatsTracker, TrackerStatistics, TrackerStatisticsEvent};
use crate::tracker::key;
use crate::tracker::torrent::{TorrentEntry, TorrentError, TorrentStats};

pub struct TorrentTracker {
    pub config: Arc<Configuration>,
    mode: TrackerMode,
    torrents: RwLock<std::collections::BTreeMap<InfoHash, TorrentEntry>>,
    updates: RwLock<std::collections::HashMap<InfoHash, u32>>,
    shadow: RwLock<std::collections::HashMap<InfoHash, u32>>,
    stats_tracker: StatsTracker,
    database: Box<dyn Database>
}

impl TorrentTracker {
    pub fn new(config: Arc<Configuration>) -> Result<TorrentTracker, r2d2::Error> {
        let database = database::connect_database(&config.db_driver, &config.db_path)?;
        let mut stats_tracker = StatsTracker::new();

        // starts a thread for updating tracker stats
        if config.statistics { stats_tracker.run_worker(); }

        Ok(TorrentTracker {
            config: config.clone(),
            mode: config.mode,
            torrents: RwLock::new(std::collections::BTreeMap::new()),
            updates: RwLock::new(std::collections::HashMap::new()),
            shadow: RwLock::new(std::collections::HashMap::new()),
            stats_tracker,
            database
        })
    }

    pub fn is_public(&self) -> bool {
        self.mode == TrackerMode::Public
    }

    pub fn is_private(&self) -> bool {
        self.mode == TrackerMode::Private || self.mode == TrackerMode::PrivateListed
    }

    pub fn is_whitelisted(&self) -> bool {
        self.mode == TrackerMode::Listed || self.mode == TrackerMode::PrivateListed
    }

    pub async fn generate_auth_key(&self, seconds_valid: u64) -> Result<AuthKey, database::Error> {
        let auth_key = key::generate_auth_key(seconds_valid);

        // add key to database
        if let Err(error) = self.database.add_key_to_keys(&auth_key).await { return Err(error); }

        Ok(auth_key)
    }

    pub async fn remove_auth_key(&self, key: String) -> Result<usize, database::Error> {
        self.database.remove_key_from_keys(key).await
    }

    pub async fn verify_auth_key(&self, auth_key: &AuthKey) -> Result<(), key::Error> {
        let db_key = self.database.get_key_from_keys(&auth_key.key).await.map_err(|_| KeyInvalid)?;
        key::verify_auth_key(&db_key)
    }

    pub async fn authenticate_request(&self, info_hash: &InfoHash, key: &Option<AuthKey>) -> Result<(), TorrentError> {
        // no authentication needed in public mode
        if self.is_public() { return Ok(()); }

        // check if auth_key is set and valid
        if self.is_private() {
            match key {
                Some(key) => {
                    if self.verify_auth_key(key).await.is_err() {
                        return Err(TorrentError::PeerKeyNotValid);
                    }
                }
                None => {
                    return Err(TorrentError::PeerNotAuthenticated);
                }
            }
        }

        // check if info_hash is whitelisted
        if self.is_whitelisted() {
            if self.is_info_hash_whitelisted(info_hash).await == false {
                return Err(TorrentError::TorrentNotWhitelisted);
            }
        }

        Ok(())
    }

    // Loading the torrents from database into memory
    pub async fn load_persistent_torrents(&self) -> Result<(), database::Error> {
        let persistent_torrents = self.database.load_persistent_torrents().await?;
        let mut torrents = self.torrents.write().await;

        for (info_hash, completed) in persistent_torrents {
            // Skip if torrent entry already exists
            if torrents.contains_key(&info_hash) { continue; }

            let torrent_entry = TorrentEntry {
                peers: Default::default(),
                completed,
            };

            torrents.insert(info_hash.clone(), torrent_entry);
        }

        Ok(())
    }

    // Saving the torrents from memory
    pub async fn save_torrents(&self) -> Result<(), database::Error> {
        let torrents = self.torrents.read().await;
        self.database.save_persistent_torrent_data(&*torrents).await
    }

    // Adding torrents is not relevant to public trackers.
    pub async fn add_torrent_to_whitelist(&self, info_hash: &InfoHash) -> Result<usize, database::Error> {
        self.database.add_info_hash_to_whitelist(info_hash.clone()).await
    }

    // Removing torrents is not relevant to public trackers.
    pub async fn remove_torrent_from_whitelist(&self, info_hash: &InfoHash) -> Result<usize, database::Error> {
        self.database.remove_info_hash_from_whitelist(info_hash.clone()).await
    }

    pub async fn is_info_hash_whitelisted(&self, info_hash: &InfoHash) -> bool {
        match self.database.get_info_hash_from_whitelist(&info_hash.to_string()).await {
            Ok(_) => true,
            Err(_) => false
        }
    }


    pub async fn get_torrent_peers(&self, info_hash: &InfoHash, client_addr: &SocketAddr, ) -> Vec<TorrentPeer> {
        let read_lock = self.torrents.read().await;

        match read_lock.get(info_hash) {
            None => vec![],
            Some(entry) => {
                entry.get_peers(Some(client_addr)).into_iter().cloned().collect()
            }
        }
    }

    pub async fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &TorrentPeer) -> TorrentStats {
        let mut torrents = self.torrents.write().await;

        let torrent_entry = match torrents.entry(info_hash.clone()) {
            Entry::Vacant(vacant) => {
                vacant.insert(TorrentEntry::new())
            }
            Entry::Occupied(entry) => {
                entry.into_mut()
            }
        };

        torrent_entry.update_peer(peer);

        let (seeders, completed, leechers) = torrent_entry.get_stats();

        if self.config.persistence {
            let mut updates = self.updates.write().await;
            if updates.contains_key(info_hash) {
                updates.remove(info_hash);
            }
            updates.insert(*info_hash, completed);
            drop(updates);
        }

        TorrentStats {
            seeders,
            leechers,
            completed,
        }
    }

    pub async fn get_torrents(&self) -> RwLockReadGuard<'_, BTreeMap<InfoHash, TorrentEntry>> {
        self.torrents.read().await
    }

    pub async fn get_stats(&self) -> RwLockReadGuard<'_, TrackerStatistics> {
        self.stats_tracker.get_stats().await
    }

    pub async fn send_stats_event(&self, event: TrackerStatisticsEvent) -> Option<Result<(), SendError<TrackerStatisticsEvent>>> {
        self.stats_tracker.send_event(event).await
    }

    pub async fn post_log(&self) {
        let torrents = self.torrents.read().await;
        let torrents_size = torrents.len();
        drop(torrents);
        let updates = self.updates.read().await;
        let updates_size = updates.len();
        drop(updates);
        let shadow = self.shadow.read().await;
        let shadow_size = shadow.len();
        drop(shadow);
        info!("-=[ Stats ]=- | Torrents: {} | Updates: {} | Shadow: {}", torrents_size, updates_size, shadow_size);
    }

    // todo: refactor
    // remove torrents without peers if enabled, and defragment memory
    pub async fn cleanup_torrents(&self) {
        let lock = self.torrents.write().await;

        // First we create a mapping of all the torrent hashes in a vector, and we use this to iterate through the btreemap.
        // Every hash we have handled, we remove from the btreemap completely, and push it to the top.
        let mut torrent_hashes: Vec<InfoHash> = Vec::new();
        for (k, _torrent_entry) in lock.iter() {
            torrent_hashes.push(k.clone());
        }

        drop(lock);

        // Let's iterate through all torrents, and parse.
        for hash in torrent_hashes.iter() {
            let mut torrent = TorrentEntry {
                peers: BTreeMap::new(),
                completed: 0,
            };

            let lock = self.torrents.write().await;
            let torrent_data = lock.get(hash).unwrap().clone();
            drop(lock);

            torrent.completed = torrent_data.completed.clone();
            for (peer_id, peer) in torrent_data.peers.iter() {
                if peer.updated.elapsed() > std::time::Duration::from_secs(self.config.peer_timeout as u64) {
                    continue;
                }
                torrent.peers.insert(peer_id.clone(), peer.clone());
            }
            let mut lock = self.torrents.write().await;
            lock.remove(hash);
            if self.config.mode.clone() == TrackerMode::Public && self.config.cleanup_peerless && !self.config.persistence {
                if torrent.peers.len() != 0 {
                    lock.insert(hash.clone(), torrent);
                }
            } else {
                lock.insert(hash.clone(), torrent);
            }
            drop(lock);
        }
    }

    // todo: refactor
    pub async fn periodic_saving(&self) {
        // Get a lock for writing
        // let mut shadow = self.shadow.write().await;

        // We will get the data and insert it into the shadow, while clearing updates.
        let mut updates = self.updates.write().await;
        let mut updates_cloned: std::collections::HashMap<InfoHash, u32> = std::collections::HashMap::new();
        // let mut torrent_hashes: Vec<InfoHash> = Vec::new();
        // Copying updates to updates_cloned
        for (k, completed) in updates.iter() {
            updates_cloned.insert(k.clone(), completed.clone());
        }
        updates.clear();
        drop(updates);

        // Copying updates_cloned into the shadow to overwrite
        for (k, completed) in updates_cloned.iter() {
            let mut shadows = self.shadow.write().await;
            if shadows.contains_key(k) {
                shadows.remove(k);
            }
            shadows.insert(k.clone(), completed.clone());
            drop(shadows);
        }
        drop(updates_cloned);

        // We updated the shadow data from the updates data, let's handle shadow data as expected.
        // Handle shadow_copy to be updated into SQL
        let mut shadow_copy: BTreeMap<InfoHash, TorrentEntry> = BTreeMap::new();
        let shadows = self.shadow.read().await;
        for (infohash, completed) in shadows.iter() {
            shadow_copy.insert(infohash.clone(), TorrentEntry {
                peers: Default::default(),
                completed: completed.clone(),
            });
        }
        drop(shadows);

        // We will now save the data from the shadow into the database.
        // This should not put any strain on the server itself, other then the harddisk/ssd.
        info!("Start saving shadow data into SQL...");
        let result = self.database.save_persistent_torrent_data(&shadow_copy).await;
        if result.is_ok() {
            info!("Done saving data to SQL and succeeded, emptying shadow...");
            let mut shadow = self.shadow.write().await;
            shadow.clear();
            drop(shadow);
        } else {
            info!("Done saving data to SQL and failed, not emptying shadow...");
        }
    }
}
