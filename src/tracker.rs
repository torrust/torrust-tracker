use serde::{Deserialize, Serialize};
use serde;
use std::borrow::Cow;
use std::collections::BTreeMap;
use tokio::sync::{RwLock, RwLockWriteGuard};
use crate::common::{AnnounceEventDef, InfoHash, NumberOfBytesDef, PeerId};
use std::net::{IpAddr, SocketAddr};
use crate::{Configuration, key_manager, MAX_SCRAPE_TORRENTS};
use std::collections::btree_map::Entry;
use crate::database::SqliteDatabase;
use std::sync::Arc;
use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use log::{debug};
use crate::key_manager::AuthKey;
use r2d2_sqlite::rusqlite;
use crate::torrust_http_tracker::AnnounceRequest;

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
    pub peer_id: PeerId,
    pub peer_addr: SocketAddr,
    #[serde(serialize_with = "ser_instant")]
    pub updated: std::time::Instant,
    #[serde(with = "NumberOfBytesDef")]
    pub uploaded: NumberOfBytes,
    #[serde(with = "NumberOfBytesDef")]
    pub downloaded: NumberOfBytes,
    #[serde(with = "NumberOfBytesDef")]
    pub left: NumberOfBytes,
    #[serde(with = "AnnounceEventDef")]
    pub event: AnnounceEvent,
}

impl TorrentPeer {
    pub fn from_udp_announce_request(announce_request: &aquatic_udp_protocol::AnnounceRequest, remote_ip: IpAddr, host_opt_ip: Option<IpAddr>) -> Self {
        let peer_addr = TorrentPeer::peer_addr_from_ip_and_port_and_opt_host_ip(remote_ip, host_opt_ip, announce_request.port.0);

        TorrentPeer {
            peer_id: PeerId(announce_request.peer_id.0),
            peer_addr,
            updated: std::time::Instant::now(),
            uploaded: announce_request.bytes_uploaded,
            downloaded: announce_request.bytes_downloaded,
            left: announce_request.bytes_left,
            event: announce_request.event
        }
    }

    pub fn from_http_announce_request(announce_request: &AnnounceRequest, remote_ip: IpAddr, host_opt_ip: Option<IpAddr>) -> Self {
        let peer_addr = TorrentPeer::peer_addr_from_ip_and_port_and_opt_host_ip(remote_ip, host_opt_ip, announce_request.port);

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
            peer_id: announce_request.peer_id.clone(),
            peer_addr,
            updated: std::time::Instant::now(),
            uploaded: NumberOfBytes(announce_request.uploaded as i64),
            downloaded: NumberOfBytes(announce_request.downloaded as i64),
            left: NumberOfBytes(announce_request.left as i64),
            event
        }
    }

    // potentially substitute localhost ip with external ip
    pub fn peer_addr_from_ip_and_port_and_opt_host_ip(remote_ip: IpAddr, host_opt_ip: Option<IpAddr>, port: u16) -> SocketAddr {
        if remote_ip.is_loopback() && host_opt_ip.is_some() {
            SocketAddr::new(host_opt_ip.unwrap(), port)
        } else {
            SocketAddr::new(remote_ip, port)
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
                let peer_old = self.peers.insert(peer.peer_id.clone(), peer.clone());
                self.update_torrent_stats_with_peer(peer, peer_old);
            }
        }
    }

    pub fn get_peers(&self, remote_addr: Option<&std::net::SocketAddr>) -> Vec<TorrentPeer> {
        let mut list = Vec::new();
        for (_, peer) in self
            .peers
            .iter()
            .filter(|e| match remote_addr {
                // don't filter on ip_version
                None => true,
                // filter out different ip_version from remote_addr
                Some(remote_address) => {
                    match e.1.peer_addr.ip() {
                        IpAddr::V4(_) => { remote_address.is_ipv4() }
                        IpAddr::V6(_) => { remote_address.is_ipv6() }
                    }
                }
            })
            .take(MAX_SCRAPE_TORRENTS as usize)
        {

            // skip ip address of client
            if let Some(remote_addr) = remote_addr {
                if peer.peer_addr == *remote_addr {
                    continue;
                }
            }

            list.push(peer.clone());
        }
        list
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
    NoPeersFound,
    CouldNotSendResponse,
    InvalidInfoHash,
}

#[derive(Debug)]
pub struct TrackerStats {
    pub tcp4_connections_handled: u64,
    pub tcp4_announces_handled: u64,
    pub tcp4_scrapes_handled: u64,
    pub tcp6_connections_handled: u64,
    pub tcp6_announces_handled: u64,
    pub tcp6_scrapes_handled: u64,
    pub udp4_connections_handled: u64,
    pub udp4_announces_handled: u64,
    pub udp4_scrapes_handled: u64,
    pub udp6_connections_handled: u64,
    pub udp6_announces_handled: u64,
    pub udp6_scrapes_handled: u64,
}

pub struct TorrentTracker {
    pub config: Arc<Configuration>,
    torrents: tokio::sync::RwLock<std::collections::BTreeMap<InfoHash, TorrentEntry>>,
    database: SqliteDatabase,
    stats: tokio::sync::RwLock<TrackerStats>,
}

impl TorrentTracker {
    pub fn new(config: Arc<Configuration>) -> Result<TorrentTracker, rusqlite::Error> {
        let database = SqliteDatabase::new(&config.db_path)?;

        Ok(TorrentTracker {
            config,
            torrents: RwLock::new(std::collections::BTreeMap::new()),
            database,
            stats: RwLock::new(TrackerStats {
                tcp4_connections_handled: 0,
                tcp4_announces_handled: 0,
                tcp4_scrapes_handled: 0,
                tcp6_connections_handled: 0,
                tcp6_announces_handled: 0,
                tcp6_scrapes_handled: 0,
                udp4_connections_handled: 0,
                udp4_announces_handled: 0,
                udp4_scrapes_handled: 0,
                udp6_connections_handled: 0,
                udp6_announces_handled: 0,
                udp6_scrapes_handled: 0,
            }),
        })
    }

    fn is_public(&self) -> bool {
        self.config.mode == TrackerMode::PublicMode
    }

    fn is_private(&self) -> bool {
        self.config.mode == TrackerMode::PrivateMode || self.config.mode == TrackerMode::PrivateListedMode
    }

    fn is_whitelisted(&self) -> bool {
        self.config.mode == TrackerMode::ListedMode || self.config.mode == TrackerMode::PrivateListedMode
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
    pub async fn load_torrents(&self) -> Result<(), rusqlite::Error> {
        let torrents = self.database.load_persistent_torrent_data().await?;

        for torrent in torrents {
            self.add_torrent(torrent.0, 0, torrent.1, 0).await;
        }

        Ok(())
    }

    // Saving the torrents from memory
    pub async fn save_torrents(&self) -> Result<(), rusqlite::Error> {
        let torrents = self.torrents.read().await;
        self.database.save_persistent_torrent_data(&*torrents).await
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

    pub async fn get_torrents(&self) -> tokio::sync::RwLockReadGuard<'_, BTreeMap<InfoHash, TorrentEntry>> {
        self.torrents.read().await
    }

    pub async fn set_stats(&self) -> RwLockWriteGuard<'_, TrackerStats> {
        self.stats.write().await
    }

    pub async fn get_stats(&self) -> tokio::sync::RwLockReadGuard<'_, TrackerStats> {
        self.stats.read().await
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
                        if peer.updated.elapsed() > std::time::Duration::from_secs(self.config.peer_timeout as u64) {
                            // remove seeders after 5 minutes since last update...
                            peers_to_remove.push(peer_id.clone());
                            torrent_entry.seeders -= 1;
                        }
                    } else if peer.updated.elapsed() > std::time::Duration::from_secs(self.config.peer_timeout as u64) {
                        // remove peers after 2 hours since last update...
                        peers_to_remove.push(peer_id.clone());
                    }
                }

                for peer_id in peers_to_remove.iter() {
                    torrent_peers.remove(peer_id);
                }
            }

            if self.config.mode.clone() == TrackerMode::PublicMode && self.config.cleanup_peerless && !self.config.persistence {
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
