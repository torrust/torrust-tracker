use serde::{Deserialize, Serialize};
use serde;
use std::collections::BTreeMap;
use tokio::sync::{RwLock, RwLockReadGuard};
use crate::common::{InfoHash};
use std::net::{SocketAddr};
use crate::{Configuration, database, key_manager};
use std::collections::btree_map::Entry;
use std::sync::Arc;
use log::info;
use crate::key_manager::AuthKey;
use crate::database::{Database};
use crate::key_manager::Error::KeyInvalid;
use crate::torrent::{TorrentEntry, TorrentError, TorrentPeer, TorrentStats};
use crate::tracker_stats::{StatsTracker, TrackerStats};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum TrackerMode {
    // Will track every new info hash and serve every peer.
    #[serde(rename = "public")]
    PublicMode,

    // Will only track whitelisted info hashes.
    #[serde(rename = "listed")]
    ListedMode,

    // Will only serve authenticated peers
    #[serde(rename = "private")]
    PrivateMode,

    // Will only track whitelisted info hashes and serve authenticated peers
    #[serde(rename = "private_listed")]
    PrivateListedMode,
}

pub struct TorrentTracker {
    pub config: Arc<Configuration>,
    torrents: tokio::sync::RwLock<std::collections::BTreeMap<InfoHash, TorrentEntry>>,
    database: Box<dyn Database>,
    pub stats_tracker: StatsTracker
}

impl TorrentTracker {
    pub fn new(config: Arc<Configuration>) -> Result<TorrentTracker, r2d2::Error> {
        let database = database::connect_database(&config.db_driver, &config.db_path)?;
        let mut stats_tracker = StatsTracker::new();

        stats_tracker.run_worker();

        Ok(TorrentTracker {
            config,
            torrents: RwLock::new(std::collections::BTreeMap::new()),
            database,
            stats_tracker
        })
    }

    pub fn is_public(&self) -> bool {
        self.config.mode == TrackerMode::PublicMode
    }

    pub fn is_private(&self) -> bool {
        self.config.mode == TrackerMode::PrivateMode || self.config.mode == TrackerMode::PrivateListedMode
    }

    pub fn is_whitelisted(&self) -> bool {
        self.config.mode == TrackerMode::ListedMode || self.config.mode == TrackerMode::PrivateListedMode
    }

    pub async fn generate_auth_key(&self, seconds_valid: u64) -> Result<AuthKey, database::Error> {
        let auth_key = key_manager::generate_auth_key(seconds_valid);

        // add key to database
        if let Err(error) = self.database.add_key_to_keys(&auth_key).await { return Err(error) }

        Ok(auth_key)
    }

    pub async fn remove_auth_key(&self, key: String) -> Result<usize, database::Error> {
        self.database.remove_key_from_keys(key).await
    }

    pub async fn verify_auth_key(&self, auth_key: &AuthKey) -> Result<(), key_manager::Error> {
        let db_key = self.database.get_key_from_keys(&auth_key.key).await.map_err(|_| KeyInvalid)?;
        key_manager::verify_auth_key(&db_key)
    }

    pub async fn authenticate_request(&self, info_hash: &InfoHash, key: &Option<AuthKey>) -> Result<(), TorrentError> {
        // no authentication needed in public mode
        if self.is_public() { return Ok(()) }

        // check if auth_key is set and valid
        if self.is_private() {
            match key {
                Some(key) => {
                    if self.verify_auth_key(key).await.is_err() {
                        return Err(TorrentError::PeerKeyNotValid)
                    }
                }
                None => {
                    return Err(TorrentError::PeerNotAuthenticated)
                }
            }
        }

        // check if info_hash is whitelisted
        if self.is_whitelisted() {
            if self.is_info_hash_whitelisted(info_hash).await == false {
                return Err(TorrentError::TorrentNotWhitelisted)
            }
        }

        Ok(())
    }

    // Loading the torrents into memory
    pub async fn load_torrents(&self) -> Result<(), database::Error> {
        let torrents = self.database.load_persistent_torrent_data().await?;

        for torrent in torrents {
            let _ = self.add_torrent(torrent.0, 0, torrent.1, 0).await;
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


    pub async fn get_torrent_peers(
        &self,
        info_hash: &InfoHash,
        peer_addr: &SocketAddr
    ) -> Vec<TorrentPeer> {
        let read_lock = self.torrents.read().await;
        match read_lock.get(info_hash) {
            None => vec![],
            Some(entry) => {
                entry.get_peers(Some(peer_addr))
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

        TorrentStats {
            seeders,
            leechers,
            completed,
        }
    }

    pub async fn add_torrent(&self, info_hash: InfoHash, seeders: u32, completed: u32, leechers: u32) -> TorrentStats {
        let mut torrents = self.torrents.write().await;

        if !torrents.contains_key(&info_hash) {
            let torrent_entry = TorrentEntry {
                peers: Default::default(),
                completed,
                seeders
            };
            torrents.insert(info_hash.clone(), torrent_entry);
        }

        TorrentStats {
            seeders,
            completed,
            leechers,
        }
    }

    pub async fn get_torrents(&self) -> RwLockReadGuard<'_, BTreeMap<InfoHash, TorrentEntry>> {
        self.torrents.read().await
    }

    pub async fn get_stats(&self) -> RwLockReadGuard<'_, TrackerStats> {
        self.stats_tracker.get_stats().await
    }

    // remove torrents without peers if enabled, and defragment memory
    pub async fn cleanup_torrents(&self) {
        info!("Cleaning torrents...");
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
            let mut torrent = TorrentEntry{
                peers: BTreeMap::new(),
                completed: 0,
                seeders: 0
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
                if peer.is_seeder() {
                    torrent.seeders += 1;
                }
            }
            let mut lock = self.torrents.write().await;
            lock.remove(hash);
            if self.config.mode.clone() == TrackerMode::PublicMode && self.config.cleanup_peerless && !self.config.persistence {
                if torrent.peers.len() != 0 {
                    lock.insert(hash.clone(), torrent);
                }
            } else {
                lock.insert(hash.clone(), torrent);
            }
            drop(lock);
        }
        info!("Torrents cleaned up.");
    }
}
