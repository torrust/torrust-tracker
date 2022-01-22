use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;
use tokio::sync::RwLock;
use crate::common::{NumberOfBytes, InfoHash};
use super::common::*;
use std::net::{SocketAddr, IpAddr};
use crate::{Configuration, http_server, key_manager, udp_server};
use std::collections::btree_map::Entry;
use crate::database::SqliteDatabase;
use std::sync::Arc;
use log::debug;
use crate::key_manager::{AuthKey};
use r2d2_sqlite::rusqlite;

const TWO_HOURS: std::time::Duration = std::time::Duration::from_secs(3600 * 2);
const FIVE_MINUTES: std::time::Duration = std::time::Duration::from_secs(300);

#[derive(Serialize, Deserialize, Clone, PartialEq)]
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

#[derive(PartialEq, Eq, Debug, Clone, Serialize)]
pub struct TorrentPeer {
    #[serde(skip)]
    pub peer_id: PeerId,
    #[serde(rename = "ip")]
    pub peer_addr: SocketAddr,
    #[serde(serialize_with = "ser_instant")]
    pub updated: std::time::Instant,
    pub uploaded: NumberOfBytes,
    pub downloaded: NumberOfBytes,
    pub left: NumberOfBytes,
    pub event: AnnounceEvent,
}

impl TorrentPeer {
    pub fn from_udp_announce_request(announce_request: &udp_server::AnnounceRequest, remote_addr: SocketAddr, peer_addr: Option<IpAddr>) -> Self {
        // Potentially substitute localhost IP with external IP
        let peer_addr = match peer_addr {
            None => SocketAddr::new(IpAddr::from(remote_addr.ip()), announce_request.port.0),
            Some(peer_addr) => {
                if remote_addr.ip().is_loopback() {
                    SocketAddr::new(IpAddr::from(peer_addr), announce_request.port.0)
                } else {
                    SocketAddr::new(IpAddr::from(remote_addr.ip()), announce_request.port.0)
                }
            }
        };

        TorrentPeer {
            peer_id: announce_request.peer_id,
            peer_addr,
            updated: std::time::Instant::now(),
            uploaded: announce_request.bytes_uploaded,
            downloaded: announce_request.bytes_downloaded,
            left: announce_request.bytes_left,
            event: announce_request.event
        }
    }

    pub fn from_http_announce_request(announce_request: &http_server::AnnounceRequest, remote_addr: SocketAddr, peer_addr: Option<IpAddr>) -> Self {
        // Potentially substitute localhost IP with external IP
        let peer_addr = match peer_addr {
            None => SocketAddr::new(IpAddr::from(remote_addr.ip()), announce_request.port),
            Some(peer_addr) => {
                if remote_addr.ip().is_loopback() {
                    SocketAddr::new(IpAddr::from(peer_addr), announce_request.port)
                } else {
                    SocketAddr::new(IpAddr::from(remote_addr.ip()), announce_request.port)
                }
            }
        };

        let event: AnnounceEvent = if let Some(event) = &announce_request.event {
            match event.as_ref() {
                "started" => AnnounceEvent::Started,
                "stopped" => AnnounceEvent::Stopped,
                "completed" => AnnounceEvent::Completed,
                _ => AnnounceEvent::None
            }
        } else {
            AnnounceEvent::None
        };

        TorrentPeer {
            peer_id: PeerId::from(announce_request.peer_id.as_bytes()),
            peer_addr,
            updated: std::time::Instant::now(),
            uploaded: announce_request.uploaded,
            downloaded: announce_request.downloaded,
            left: announce_request.left,
            event
        }
    }

    fn is_seeder(&self) -> bool { self.left.0 <= 0 && self.event != AnnounceEvent::Stopped }

    fn is_completed(&self) -> bool {
        self.event == AnnounceEvent::Completed
    }
}

fn ser_instant<S: serde::Serializer>(inst: &std::time::Instant, ser: S) -> Result<S::Ok, S::Error> {
    ser.serialize_u64(inst.elapsed().as_millis() as u64)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TorrentEntry {
    #[serde(skip)]
    peers: std::collections::BTreeMap<PeerId, TorrentPeer>,
    completed: u32,
    #[serde(skip)]
    seeders: u32,
}

impl TorrentEntry {
    pub fn new() -> TorrentEntry {
        TorrentEntry {
            peers: std::collections::BTreeMap::new(),
            completed: 0,
            seeders: 0,
        }
    }

    pub fn update_peer(&mut self, peer: &TorrentPeer) {
        match peer.event {
            AnnounceEvent::Stopped => {
                let peer_old = self.peers.remove(&peer.peer_id);
                self.update_torrent_stats_with_peer(peer, peer_old);
            }
            _ => {
                let peer_old = self.peers.insert(peer.peer_id, peer.clone());
                self.update_torrent_stats_with_peer(peer, peer_old);
            }
        }
    }

    pub fn get_peers(&self, remote_addr: &std::net::SocketAddr) -> Vec<TorrentPeer> {
        let mut list = Vec::new();
        for (_, peer) in self
            .peers
            .iter()
            .filter(|e| e.1.peer_addr.is_ipv4())
            .take(MAX_SCRAPE_TORRENTS as usize)
        {

            // skip ip address of client
            if peer.peer_addr == *remote_addr {
                //continue;
            }

            list.push(peer.clone());
        }
        list
    }

    pub fn get_peers_iter(&self) -> impl Iterator<Item = (&PeerId, &TorrentPeer)> {
        self.peers.iter()
    }

    pub fn update_torrent_stats_with_peer(&mut self, peer: &TorrentPeer, peer_old: Option<TorrentPeer>) {
        match peer_old {
            None => {
                if peer.is_seeder() {
                    self.seeders += 1;
                }

                if peer.is_completed() {
                    self.completed += 1;
                }
            }
            Some(peer_old) => {
                match peer.event {
                    AnnounceEvent::None => {
                        if peer.is_seeder() && !peer_old.is_seeder() {
                            self.seeders += 1;
                        }
                    }
                    AnnounceEvent::Completed => {
                        if peer.is_seeder() && !peer_old.is_seeder() {
                            self.seeders += 1;
                        }

                        // don't double count completed
                        if !peer_old.is_completed() {
                            self.completed += 1;
                        }
                    }
                    AnnounceEvent::Stopped => {
                        if peer_old.is_seeder() {
                            self.seeders -= 1;
                        }
                    }
                    // impossible, started should be the first time a peer announces itself
                    AnnounceEvent::Started => {}
                }
            }
        }
    }

    pub fn get_stats(&self) -> (u32, u32, u32) {
        let leechers: u32 = if self.seeders < (self.peers.len() as u32) {
            (self.peers.len() as u32) - self.seeders
        } else {
            0
        };

        (self.seeders, self.completed, leechers)
    }
}

#[derive(Serialize, Deserialize)]
struct DatabaseRow<'a> {
    info_hash: InfoHash,
    entry: Cow<'a, TorrentEntry>,
}

#[derive(Debug)]
pub struct TorrentStats {
    pub completed: u32,
    pub seeders: u32,
    pub leechers: u32,
}

#[derive(Debug)]
pub enum TorrentError {
    TorrentNotWhitelisted,
    PeerNotAuthenticated,
    PeerKeyNotValid,
}

pub struct TorrentTracker {
    pub config: Arc<Configuration>,
    torrents: tokio::sync::RwLock<std::collections::BTreeMap<InfoHash, TorrentEntry>>,
    database: SqliteDatabase,
}

impl TorrentTracker {
    pub fn new(config: Arc<Configuration>) -> TorrentTracker {
        let database = SqliteDatabase::new(&config.db_path).unwrap_or_else(|error| {
            panic!("Could not create SQLite database. Reason: {}", error)
        });

        TorrentTracker {
            config,
            torrents: RwLock::new(std::collections::BTreeMap::new()),
            database,
        }
    }

    pub async fn generate_auth_key(&self, seconds_valid: u64) -> Result<AuthKey, rusqlite::Error> {
        let auth_key = key_manager::generate_auth_key(seconds_valid);

        // add key to database
        if let Err(error) = self.database.add_key_to_keys(&auth_key).await { return Err(error) }

        Ok(auth_key)
    }

    pub async fn remove_auth_key(&self, key: String) -> Result<usize, rusqlite::Error> {
        self.database.remove_key_from_keys(key).await
    }

    pub async fn verify_auth_key(&self, auth_key: &AuthKey) -> Result<(), key_manager::Error> {
        let db_key = self.database.get_key_from_keys(&auth_key.key).await?;
        key_manager::verify_auth_key(&db_key)
    }

    pub async fn authenticate_request(&self, info_hash: &InfoHash, key: &Option<AuthKey>) -> Result<(), TorrentError> {
        match self.config.mode {
            TrackerMode::PublicMode => Ok(()),
            TrackerMode::ListedMode => {
                if !self.is_info_hash_whitelisted(info_hash).await {
                    return Err(TorrentError::TorrentNotWhitelisted)
                }

                Ok(())
            }
            TrackerMode::PrivateMode => {
                match key {
                    Some(key) => {
                        if self.verify_auth_key(key).await.is_err() {
                            return Err(TorrentError::PeerKeyNotValid)
                        }

                        Ok(())
                    }
                    None => {
                        return Err(TorrentError::PeerNotAuthenticated)
                    }
                }
            }
            TrackerMode::PrivateListedMode => {
                match key {
                    Some(key) => {
                        if self.verify_auth_key(key).await.is_err() {
                            return Err(TorrentError::PeerKeyNotValid)
                        }

                        if !self.is_info_hash_whitelisted(info_hash).await {
                            return Err(TorrentError::TorrentNotWhitelisted)
                        }

                        Ok(())
                    }
                    None => {
                        return Err(TorrentError::PeerNotAuthenticated)
                    }
                }
            }
        }
    }

    // Adding torrents is not relevant to public trackers.
    pub async fn add_torrent_to_whitelist(&self, info_hash: &InfoHash) -> Result<usize, rusqlite::Error> {
        self.database.add_info_hash_to_whitelist(info_hash.clone()).await
    }

    // Removing torrents is not relevant to public trackers.
    pub async fn remove_torrent_from_whitelist(&self, info_hash: &InfoHash) -> Result<usize, rusqlite::Error> {
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
        peer_addr: &std::net::SocketAddr
    ) -> Option<Vec<TorrentPeer>> {
        let read_lock = self.torrents.read().await;
        match read_lock.get(info_hash) {
            None => {
                None
            }
            Some(entry) => {
                Some(entry.get_peers(peer_addr))
            }
        }
    }

    pub async fn update_torrent_with_peer_and_get_stats(&self, info_hash: &InfoHash, peer: &TorrentPeer) -> Result<TorrentStats, TorrentError> {
        let mut torrents = self.torrents.write().await;

        let torrent_entry = match torrents.entry(info_hash.clone()) {
            Entry::Vacant(vacant) => {
                Ok(vacant.insert(TorrentEntry::new()))
            }
            Entry::Occupied(entry) => {
                Ok(entry.into_mut())
            }
        };

        match torrent_entry {
            Ok(torrent_entry) => {
                torrent_entry.update_peer(peer);

                let (seeders, completed, leechers) = torrent_entry.get_stats();

                Ok(TorrentStats {
                    seeders,
                    leechers,
                    completed,
                })
            }
            Err(e) => Err(e)
        }
    }

    pub async fn get_torrents(&self) -> tokio::sync::RwLockReadGuard<'_, BTreeMap<InfoHash, TorrentEntry>> {
        self.torrents.read().await
    }

    // remove torrents without peers
    pub async fn cleanup_torrents(&self) {
        debug!("Cleaning torrents..");
        let mut lock = self.torrents.write().await;
        let db: &mut BTreeMap<InfoHash, TorrentEntry> = &mut *lock;
        let mut torrents_to_remove = Vec::new();

        for (k, torrent_entry) in db.iter_mut() {
            // timed-out peers..
            {
                let mut peers_to_remove = Vec::new();
                let torrent_peers = &mut torrent_entry.peers;

                for (peer_id, peer) in torrent_peers.iter() {
                    if peer.is_seeder() {
                        if peer.updated.elapsed() > FIVE_MINUTES {
                            // remove seeders after 5 minutes since last update...
                            peers_to_remove.push(*peer_id);
                            torrent_entry.seeders -= 1;
                        }
                    } else if peer.updated.elapsed() > TWO_HOURS {
                        // remove peers after 2 hours since last update...
                        peers_to_remove.push(*peer_id);
                    }
                }

                for peer_id in peers_to_remove.iter() {
                    torrent_peers.remove(peer_id);
                }
            }

            if self.config.mode.clone() == TrackerMode::PublicMode {
                // peer-less torrents..
                if torrent_entry.peers.len() == 0 {
                    torrents_to_remove.push(k.clone());
                }
            }
        }

        for info_hash in torrents_to_remove {
            db.remove(&info_hash);
        }
    }
}
