use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::sync::{RwLock, RwLockReadGuard};
use tokio::sync::mpsc::error::SendError;

use crate::Configuration;
use crate::protocol::common::InfoHash;
use crate::databases::database::Database;
use crate::databases::database;
use crate::mode::TrackerMode;
use crate::peer::TorrentPeer;
use crate::tracker::key::AuthKey;
use crate::statistics::{StatsTracker, TrackerStatistics, TrackerStatisticsEvent};
use crate::tracker::key;
use crate::tracker::torrent::{TorrentEntry, TorrentError, TorrentStats};

pub struct TorrentTracker {
    pub config: Arc<Configuration>,
    mode: TrackerMode,
    keys: RwLock<std::collections::HashMap<String, AuthKey>>,
    whitelist: RwLock<std::collections::HashSet<InfoHash>>,
    torrents: RwLock<std::collections::BTreeMap<InfoHash, TorrentEntry>>,
    stats_tracker: StatsTracker,
    database: Box<dyn Database>
}

impl TorrentTracker {
    pub fn new(config: Arc<Configuration>) -> Result<TorrentTracker, r2d2::Error> {
        let database = database::connect_database(&config.db_driver, &config.db_path)?;
        let mut stats_tracker = StatsTracker::new();

        // starts a thread for updating tracker stats
        if config.tracker_usage_statistics { stats_tracker.run_worker(); }

        Ok(TorrentTracker {
            config: config.clone(),
            mode: config.mode,
            keys: RwLock::new(std::collections::HashMap::new()),
            whitelist: RwLock::new(std::collections::HashSet::new()),
            torrents: RwLock::new(std::collections::BTreeMap::new()),
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

        // Add key to database
        if let Err(error) = self.database.add_key_to_keys(&auth_key).await { return Err(error); }

        // Add key to in-memory database
        self.keys.write().await.insert(auth_key.key.clone(), auth_key.clone());

        Ok(auth_key)
    }

    pub async fn remove_auth_key(&self, key: &str) -> Result<usize, database::Error> {
        self.database.remove_key_from_keys(&key).await?;

        // Remove key from in-memory database
        self.keys.write().await.remove(key);

        Ok(1)
    }

    pub async fn verify_auth_key(&self, auth_key: &AuthKey) -> Result<(), key::Error> {
        let keys_lock = self.keys.read().await;

        if let Some(key) = keys_lock.get(&auth_key.key) {
            key::verify_auth_key(key)
        } else {
            Err(key::Error::KeyInvalid)
        }
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

        // todo: speed this up
        // check if info_hash is whitelisted
        if self.is_whitelisted() {
            if !self.is_info_hash_whitelisted(info_hash).await {
                return Err(TorrentError::TorrentNotWhitelisted);
            }
        }

        Ok(())
    }

    pub async fn load_keys(&self) -> Result<(), database::Error> {
        let keys_from_database = self.database.load_keys().await?;
        let mut keys = self.keys.write().await;

        keys.clear();

        for key in keys_from_database {
            let _ = keys.insert(key.key.clone(), key);
        }

        Ok(())
    }

    pub async fn load_whitelist(&self) -> Result<(), database::Error> {
        let whitelisted_torrents_from_database = self.database.load_whitelist().await?;
        let mut whitelist = self.whitelist.write().await;

        whitelist.clear();

        for info_hash in whitelisted_torrents_from_database {
            let _ = whitelist.insert(info_hash);
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

        let stats_updated = torrent_entry.update_peer(peer);

        // todo: move this action to a separate worker
        if self.config.persistent_torrent_completed_stat && stats_updated {
            let _ = self.database.save_persistent_torrent(&info_hash, torrent_entry.completed).await;
        }

        let (seeders, completed, leechers) = torrent_entry.get_stats();

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

    // Remove inactive peers and (optionally) peerless torrents
    pub async fn cleanup_torrents(&self) {
        let mut torrents_lock = self.torrents.write().await;

        // If we don't need to remove torrents we will use the faster iter
        if self.config.remove_peerless_torrents {
            torrents_lock.retain(|_, torrent_entry| {
                torrent_entry.remove_inactive_peers(self.config.max_peer_timeout);

                match self.config.persistent_torrent_completed_stat {
                    true => { torrent_entry.completed > 0 || torrent_entry.peers.len() > 0 }
                    false => { torrent_entry.peers.len() > 0 }
                }
            });
        } else {
            for (_, torrent_entry) in torrents_lock.iter_mut() {
                torrent_entry.remove_inactive_peers(self.config.max_peer_timeout);
            }
        }
    }
}
