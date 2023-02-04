pub mod auth;
pub mod peer;
pub mod services;
pub mod statistics;
pub mod torrent;

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::mpsc::error::SendError;
use tokio::sync::{RwLock, RwLockReadGuard};
use torrust_tracker_configuration::Configuration;
use torrust_tracker_primitives::TrackerMode;

use crate::databases::{self, Database};
use crate::protocol::info_hash::InfoHash;

pub struct Tracker {
    pub config: Arc<Configuration>,
    mode: TrackerMode,
    keys: RwLock<std::collections::HashMap<String, auth::Key>>,
    whitelist: RwLock<std::collections::HashSet<InfoHash>>,
    torrents: RwLock<std::collections::BTreeMap<InfoHash, torrent::Entry>>,
    stats_event_sender: Option<Box<dyn statistics::EventSender>>,
    stats_repository: statistics::Repo,
    pub database: Box<dyn Database>,
}

#[derive(Debug, PartialEq, Default)]
pub struct TorrentsMetrics {
    pub seeders: u64,
    pub completed: u64,
    pub leechers: u64,
    pub torrents: u64,
}

impl Tracker {
    /// # Errors
    ///
    /// Will return a `r2d2::Error` if unable to connect to database.
    pub fn new(
        config: &Arc<Configuration>,
        stats_event_sender: Option<Box<dyn statistics::EventSender>>,
        stats_repository: statistics::Repo,
    ) -> Result<Tracker, r2d2::Error> {
        let database = databases::connect(&config.db_driver, &config.db_path)?;

        Ok(Tracker {
            config: config.clone(),
            mode: config.mode,
            keys: RwLock::new(std::collections::HashMap::new()),
            whitelist: RwLock::new(std::collections::HashSet::new()),
            torrents: RwLock::new(std::collections::BTreeMap::new()),
            stats_event_sender,
            stats_repository,
            database,
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

    /// # Errors
    ///
    /// Will return a `database::Error` if unable to add the `auth_key` to the database.
    pub async fn generate_auth_key(&self, lifetime: Duration) -> Result<auth::Key, databases::error::Error> {
        let auth_key = auth::generate(lifetime);
        self.database.add_key_to_keys(&auth_key).await?;
        self.keys.write().await.insert(auth_key.key.clone(), auth_key.clone());
        Ok(auth_key)
    }

    /// # Errors
    ///
    /// Will return a `database::Error` if unable to remove the `key` to the database.
    pub async fn remove_auth_key(&self, key: &str) -> Result<(), databases::error::Error> {
        self.database.remove_key_from_keys(key).await?;
        self.keys.write().await.remove(key);
        Ok(())
    }

    /// # Errors
    ///
    /// Will return a `key::Error` if unable to get any `auth_key`.
    pub async fn verify_auth_key(&self, auth_key: &auth::Key) -> Result<(), auth::Error> {
        // todo: use auth::KeyId for the function argument `auth_key`
        match self.keys.read().await.get(&auth_key.key) {
            None => Err(auth::Error::KeyInvalid),
            Some(key) => auth::verify(key),
        }
    }

    /// # Errors
    ///
    /// Will return a `database::Error` if unable to `load_keys` from the database.
    pub async fn load_keys(&self) -> Result<(), databases::error::Error> {
        let keys_from_database = self.database.load_keys().await?;
        let mut keys = self.keys.write().await;

        keys.clear();

        for key in keys_from_database {
            keys.insert(key.key.clone(), key);
        }

        Ok(())
    }

    /// Adding torrents is not relevant to public trackers.
    ///
    /// # Errors
    ///
    /// Will return a `database::Error` if unable to add the `info_hash` into the whitelist database.
    pub async fn add_torrent_to_whitelist(&self, info_hash: &InfoHash) -> Result<(), databases::error::Error> {
        self.add_torrent_to_database_whitelist(info_hash).await?;
        self.add_torrent_to_memory_whitelist(info_hash).await;
        Ok(())
    }

    /// It adds a torrent to the whitelist if it has not been whitelisted previously
    async fn add_torrent_to_database_whitelist(&self, info_hash: &InfoHash) -> Result<(), databases::error::Error> {
        let is_whitelisted = self.database.is_info_hash_whitelisted(info_hash).await?;

        if is_whitelisted {
            return Ok(());
        }

        self.database.add_info_hash_to_whitelist(*info_hash).await?;

        Ok(())
    }

    pub async fn add_torrent_to_memory_whitelist(&self, info_hash: &InfoHash) -> bool {
        self.whitelist.write().await.insert(*info_hash)
    }

    /// Removing torrents is not relevant to public trackers.
    ///
    /// # Errors
    ///
    /// Will return a `database::Error` if unable to remove the `info_hash` from the whitelist database.
    pub async fn remove_torrent_from_whitelist(&self, info_hash: &InfoHash) -> Result<(), databases::error::Error> {
        self.remove_torrent_from_database_whitelist(info_hash).await?;
        self.remove_torrent_from_memory_whitelist(info_hash).await;
        Ok(())
    }

    /// # Errors
    ///
    /// Will return a `database::Error` if unable to remove the `info_hash` from the whitelist database.
    pub async fn remove_torrent_from_database_whitelist(&self, info_hash: &InfoHash) -> Result<(), databases::error::Error> {
        let is_whitelisted = self.database.is_info_hash_whitelisted(info_hash).await?;

        if !is_whitelisted {
            return Ok(());
        }

        self.database.remove_info_hash_from_whitelist(*info_hash).await?;

        Ok(())
    }

    pub async fn remove_torrent_from_memory_whitelist(&self, info_hash: &InfoHash) -> bool {
        self.whitelist.write().await.remove(info_hash)
    }

    pub async fn is_info_hash_whitelisted(&self, info_hash: &InfoHash) -> bool {
        self.whitelist.read().await.contains(info_hash)
    }

    /// # Errors
    ///
    /// Will return a `database::Error` if unable to load the list whitelisted `info_hash`s from the database.
    pub async fn load_whitelist(&self) -> Result<(), databases::error::Error> {
        let whitelisted_torrents_from_database = self.database.load_whitelist().await?;
        let mut whitelist = self.whitelist.write().await;

        whitelist.clear();

        for info_hash in whitelisted_torrents_from_database {
            let _ = whitelist.insert(info_hash);
        }

        Ok(())
    }

    /// # Errors
    ///
    /// Will return a `torrent::Error::PeerKeyNotValid` if the `key` is not valid.
    ///
    /// Will return a `torrent::Error::PeerNotAuthenticated` if the `key` is `None`.
    ///
    /// Will return a `torrent::Error::TorrentNotWhitelisted` if the the Tracker is in listed mode and the `info_hash` is not whitelisted.
    pub async fn authenticate_request(&self, info_hash: &InfoHash, key: &Option<auth::Key>) -> Result<(), torrent::Error> {
        // no authentication needed in public mode
        if self.is_public() {
            return Ok(());
        }

        // check if auth_key is set and valid
        if self.is_private() {
            match key {
                Some(key) => {
                    if self.verify_auth_key(key).await.is_err() {
                        return Err(torrent::Error::PeerKeyNotValid);
                    }
                }
                None => {
                    return Err(torrent::Error::PeerNotAuthenticated);
                }
            }
        }

        // check if info_hash is whitelisted
        if self.is_whitelisted() && !self.is_info_hash_whitelisted(info_hash).await {
            return Err(torrent::Error::TorrentNotWhitelisted);
        }

        Ok(())
    }

    /// Loading the torrents from database into memory
    ///
    /// # Errors
    ///
    /// Will return a `database::Error` if unable to load the list of `persistent_torrents` from the database.
    pub async fn load_persistent_torrents(&self) -> Result<(), databases::error::Error> {
        let persistent_torrents = self.database.load_persistent_torrents().await?;
        let mut torrents = self.torrents.write().await;

        for (info_hash, completed) in persistent_torrents {
            // Skip if torrent entry already exists
            if torrents.contains_key(&info_hash) {
                continue;
            }

            let torrent_entry = torrent::Entry {
                peers: BTreeMap::default(),
                completed,
            };

            torrents.insert(info_hash, torrent_entry);
        }

        Ok(())
    }

    /// Get all torrent peers for a given torrent filtering out the peer with the client address
    pub async fn get_torrent_peers(&self, info_hash: &InfoHash, client_addr: &SocketAddr) -> Vec<peer::Peer> {
        let read_lock = self.torrents.read().await;

        match read_lock.get(info_hash) {
            None => vec![],
            Some(entry) => entry.get_peers(Some(client_addr)).into_iter().copied().collect(),
        }
    }

    /// Get all torrent peers for a given torrent
    pub async fn get_all_torrent_peers(&self, info_hash: &InfoHash) -> Vec<peer::Peer> {
        let read_lock = self.torrents.read().await;

        match read_lock.get(info_hash) {
            None => vec![],
            Some(entry) => entry.get_peers(None).into_iter().copied().collect(),
        }
    }

    pub async fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &peer::Peer) -> torrent::SwamStats {
        let mut torrents = self.torrents.write().await;

        let torrent_entry = match torrents.entry(*info_hash) {
            Entry::Vacant(vacant) => vacant.insert(torrent::Entry::new()),
            Entry::Occupied(entry) => entry.into_mut(),
        };

        let stats_updated = torrent_entry.update_peer(peer);

        // todo: move this action to a separate worker
        if self.config.persistent_torrent_completed_stat && stats_updated {
            let _ = self
                .database
                .save_persistent_torrent(info_hash, torrent_entry.completed)
                .await;
        }

        let (seeders, completed, leechers) = torrent_entry.get_stats();

        torrent::SwamStats {
            completed,
            seeders,
            leechers,
        }
    }

    pub async fn get_torrents(&self) -> RwLockReadGuard<'_, BTreeMap<InfoHash, torrent::Entry>> {
        self.torrents.read().await
    }

    pub async fn get_torrents_metrics(&self) -> TorrentsMetrics {
        let mut torrents_metrics = TorrentsMetrics {
            seeders: 0,
            completed: 0,
            leechers: 0,
            torrents: 0,
        };

        let db = self.get_torrents().await;

        db.values().for_each(|torrent_entry| {
            let (seeders, completed, leechers) = torrent_entry.get_stats();
            torrents_metrics.seeders += u64::from(seeders);
            torrents_metrics.completed += u64::from(completed);
            torrents_metrics.leechers += u64::from(leechers);
            torrents_metrics.torrents += 1;
        });

        torrents_metrics
    }

    pub async fn get_stats(&self) -> RwLockReadGuard<'_, statistics::Metrics> {
        self.stats_repository.get_stats().await
    }

    pub async fn send_stats_event(&self, event: statistics::Event) -> Option<Result<(), SendError<statistics::Event>>> {
        match &self.stats_event_sender {
            None => None,
            Some(stats_event_sender) => stats_event_sender.send_event(event).await,
        }
    }

    // Remove inactive peers and (optionally) peerless torrents
    pub async fn cleanup_torrents(&self) {
        let mut torrents_lock = self.torrents.write().await;

        // If we don't need to remove torrents we will use the faster iter
        if self.config.remove_peerless_torrents {
            torrents_lock.retain(|_, torrent_entry| {
                torrent_entry.remove_inactive_peers(self.config.max_peer_timeout);

                if self.config.persistent_torrent_completed_stat {
                    torrent_entry.completed > 0 || !torrent_entry.peers.is_empty()
                } else {
                    !torrent_entry.peers.is_empty()
                }
            });
        } else {
            for (_, torrent_entry) in torrents_lock.iter_mut() {
                torrent_entry.remove_inactive_peers(self.config.max_peer_timeout);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use torrust_tracker_configuration::{Configuration, ephemeral_configuration};

    use super::statistics::Keeper;
    use super::{TorrentsMetrics, Tracker};

    pub fn tracker_configuration() -> Arc<Configuration> {
        Arc::new(ephemeral_configuration())
    }

    pub fn tracker_factory() -> Tracker {
        // code-review: the tracker initialization is duplicated in many places. Consider make this function public.

        // Configuration
        let configuration = tracker_configuration();

        // Initialize stats tracker
        let (stats_event_sender, stats_repository) = Keeper::new_active_instance();

        // Initialize Torrust tracker
        match Tracker::new(&configuration, Some(stats_event_sender), stats_repository) {
            Ok(tracker) => tracker,
            Err(error) => {
                panic!("{}", error)
            }
        }
    }

    #[tokio::test]
    async fn the_tracker_should_collect_torrent_metrics() {
        let tracker = tracker_factory();

        let torrents_metrics = tracker.get_torrents_metrics().await;

        assert_eq!(
            torrents_metrics,
            TorrentsMetrics {
                seeders: 0,
                completed: 0,
                leechers: 0,
                torrents: 0
            }
        );
    }
}
