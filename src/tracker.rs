use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;
use tokio::sync::RwLock;
use crate::common::{NumberOfBytes, InfoHash};
use super::common::*;
use std::net::{SocketAddr, IpAddr};
use crate::{AnnounceRequest, Configuration};
use std::collections::btree_map::Entry;
use crate::database::SqliteDatabase;
use std::sync::Arc;
use log::debug;

const TWO_HOURS: std::time::Duration = std::time::Duration::from_secs(3600 * 2);
const FIVE_MINUTES: std::time::Duration = std::time::Duration::from_secs(300);

#[derive(Deserialize, Clone, PartialEq)]
pub enum TrackerMode {
    /// In static mode torrents are tracked only if they were added ahead of time.
    #[serde(rename = "static")]
    StaticMode,

    /// In dynamic mode, torrents are tracked being added ahead of time.
    #[serde(rename = "dynamic")]
    DynamicMode,

    /// Tracker will only serve authenticated peers.
    #[serde(rename = "private")]
    PrivateMode,
}

#[derive(Clone, Serialize)]
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
    pub fn from_announce_request(announce_request: &AnnounceRequest, remote_addr: SocketAddr, peer_addr: Option<IpAddr>) -> Self {
        // Potentially substitute localhost IP with external IP
        let peer_addr = if remote_addr.ip().is_loopback() {
            SocketAddr::new(peer_addr.unwrap_or(IpAddr::from(remote_addr.ip())), announce_request.port.0)
        } else {
            SocketAddr::new(IpAddr::from(remote_addr.ip()), announce_request.port.0)
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
    is_flagged: bool,
    #[serde(skip)]
    peers: std::collections::BTreeMap<PeerId, TorrentPeer>,
    completed: u32,
    #[serde(skip)]
    seeders: u32,
}

impl TorrentEntry {
    pub fn new() -> TorrentEntry {
        TorrentEntry {
            is_flagged: false,
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

    pub fn get_peers(&self, remote_addr: &std::net::SocketAddr) -> Vec<std::net::SocketAddr> {
        let mut list = Vec::new();
        for (_, peer) in self
            .peers
            .iter()
            .filter(|e| e.1.peer_addr.is_ipv4() == remote_addr.is_ipv4())
            .take(MAX_SCRAPE_TORRENTS as usize)
        {
            if peer.peer_addr == *remote_addr {
                continue;
            }

            list.push(peer.peer_addr);
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
                    // impossible, started should be the first time a peer announces itself
                    AnnounceEvent::Started => {}
                    // impossible, peer should have been removed on this event
                    AnnounceEvent::Stopped => {}
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

    pub fn is_flagged(&self) -> bool {
        self.is_flagged
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
    TorrentFlagged,
    TorrentNotRegistered,
}

pub struct TorrentTracker {
    torrents: tokio::sync::RwLock<std::collections::BTreeMap<InfoHash, TorrentEntry>>,
    database: Arc<SqliteDatabase>,
    cfg: Arc<Configuration>,
}

impl TorrentTracker {
    pub fn new(cfg: Arc<Configuration>, database: Arc<SqliteDatabase>) -> TorrentTracker {
        TorrentTracker {
            torrents: RwLock::new(std::collections::BTreeMap::new()),
            database,
            cfg
        }
    }

    /// Adding torrents is not relevant to dynamic trackers.
    pub async fn add_torrent(&self, info_hash: &InfoHash) -> Result<(), ()> {
        let mut write_lock = self.torrents.write().await;
        match write_lock.entry(info_hash.clone()) {
            std::collections::btree_map::Entry::Vacant(ve) => {
                ve.insert(TorrentEntry::new());
                return Ok(());
            }
            std::collections::btree_map::Entry::Occupied(_entry) => {
                return Err(());
            }
        }
    }

    /// If the torrent is flagged, it will not be removed unless force is set to true.
    pub async fn remove_torrent(&self, info_hash: &InfoHash, force: bool) -> Result<(), ()> {
        let mut entry_lock = self.torrents.write().await;
        let torrent_entry = entry_lock.entry(info_hash.clone());
        match torrent_entry {
            Entry::Vacant(_) => {
                // no entry, nothing to do...
                return Err(());
            }
            Entry::Occupied(entry) => {
                if force || !entry.get().is_flagged() {
                    entry.remove();
                    return Ok(());
                }
                return Err(());
            }
        }
    }

    /// flagged torrents will result in a tracking error. This is to allow enforcement against piracy.
    pub async fn set_torrent_flag(&self, info_hash: &InfoHash, is_flagged: bool) -> bool {
        if let Some(entry) = self.torrents.write().await.get_mut(info_hash) {
            if is_flagged && !entry.is_flagged {
                // empty peer list.
                entry.peers.clear();
            }
            entry.is_flagged = is_flagged;
            true
        } else {
            false
        }
    }

    pub async fn get_torrent_peers(
        &self,
        info_hash: &InfoHash,
        peer_addr: &std::net::SocketAddr
    ) -> Option<Vec<std::net::SocketAddr>> {
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
                // todo: support multiple tracker modes
                match self.cfg.get_mode().clone() {
                    TrackerMode::DynamicMode => {
                        Ok(vacant.insert(TorrentEntry::new()))
                    },
                    _ => {
                        Err(TorrentError::TorrentNotRegistered)
                    }
                }
            }
            Entry::Occupied(entry) => {
                if entry.get().is_flagged() {
                    Err(TorrentError::TorrentFlagged)
                } else {
                    Ok(entry.into_mut())
                }
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

    pub(crate) async fn get_torrents<'a>(&'a self) -> tokio::sync::RwLockReadGuard<'a, BTreeMap<InfoHash, TorrentEntry>> {
        self.torrents.read().await
    }

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

            if self.cfg.get_mode().clone() == TrackerMode::DynamicMode {
                // peer-less torrents..
                if torrent_entry.peers.len() == 0 && !torrent_entry.is_flagged() {
                    torrents_to_remove.push(k.clone());
                }
            }
        }

        for info_hash in torrents_to_remove {
            db.remove(&info_hash);
        }
    }
}
