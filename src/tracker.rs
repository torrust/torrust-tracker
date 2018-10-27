use std;
use binascii;
use serde::{self, Serialize, Deserialize};

use server::Events;

#[derive(Deserialize, Clone)]
pub enum TrackerMode {

    /// In static mode torrents are tracked only if they were added ahead of time.
    #[serde(rename="static")]
    StaticMode,

    /// In dynamic mode, torrents are tracked being added ahead of time.
    #[serde(rename="dynamic")]
    DynamicMode,

    /// Tracker will only serve authenticated peers.
    #[serde(rename="private")]
    PrivateMode,
}

struct TorrentPeer {
    ip: std::net::SocketAddr,
    uploaded: u64,
    downloaded: u64,
    left: u64,
    event: Events,
    updated: std::time::SystemTime,
}

pub type PeerId = [u8; 20];
pub type InfoHash = [u8; 20];

#[derive(Serialize, Deserialize)]
pub struct TorrentEntry {
    is_flagged: bool,

    #[serde(skip)]
    peers: std::collections::BTreeMap<PeerId, TorrentPeer>,

    completed: u32,

    #[serde(skip)]
    seeders: u32,
}

impl TorrentEntry {
    pub fn new() -> TorrentEntry{
        TorrentEntry{
            is_flagged: false,
            peers: std::collections::BTreeMap::new(),
            completed: 0,
            seeders: 0,
        }
    }

    pub fn is_flagged(&self) -> bool {
        self.is_flagged
    }

    pub fn update_peer(&mut self, peer_id: &PeerId, remote_address: &std::net::SocketAddr, uploaded: u64, downloaded: u64, left: u64, event: Events) {
        let is_seeder = left == 0 && uploaded > 0;
        let mut was_seeder = false;
        let mut is_completed = left == 0 && (event as u32) == (Events::Complete as u32);
        if let Some(prev) = self.peers.insert(*peer_id, TorrentPeer{
            updated: std::time::SystemTime::now(),
            left,
            downloaded,
            uploaded,
            ip: *remote_address,
            event,
        }) {
            was_seeder = prev.left == 0 && prev.uploaded > 0;

            if is_completed && (prev.event as u32) == (Events::Complete as u32) {
                // don't update count again. a torrent should only be updated once per peer.
                is_completed = false;
            }
        }

        if is_seeder && !was_seeder {
            self.seeders += 1;
        } else if was_seeder && !is_seeder {
            self.seeders -= 1;
        }

        if is_completed {
            self.completed += 1;
        }
    }

    pub fn get_peers(&self, remote_addr: &std::net::SocketAddr) -> Vec<std::net::SocketAddr> {
        let mut list = Vec::new();
        for (_, peer) in self.peers.iter().filter(|e| e.1.ip.is_ipv4() == remote_addr.is_ipv4()).take(74) {
            if peer.ip == *remote_addr {
                continue;
            }

            list.push(peer.ip);
        }
        list
    }

    pub fn get_stats(&self) -> (u32, u32, u32) {
        let leechers = (self.peers.len() as u32) - self.seeders;
        (self.seeders, self.completed, leechers)
    }
}

struct TorrentDatabase {
    torrent_peers: std::sync::RwLock<std::collections::BTreeMap<InfoHash, TorrentEntry>>,
}

impl Default for TorrentDatabase {
    fn default() -> Self {
        TorrentDatabase{
            torrent_peers: std::sync::RwLock::new(std::collections::BTreeMap::new()),
        }
    }
}

impl Serialize for TorrentDatabase {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        use binascii::bin2hex;

        let db = match self.torrent_peers.read() {
            Ok(v) => v,
            Err(err) => {
                error!("Failed to lock database for reading. {}", err);
                panic!("fatal error. check logs.");
            }
        };
        let mut info_hash_buffer = [0u8; 40];

        let mut map = serializer.serialize_map(None)?;

        for entry in db.iter() {
            let info_hash = match bin2hex(entry.0, &mut info_hash_buffer) {
                Ok(v) => std::str::from_utf8(v).unwrap(),
                Err(err) => {
                    error!("Failed to serialize database key.");
                    panic!("fatal error. check logs.")
                }
            };
            map.serialize_entry(info_hash, entry.1)?;
        }

        map.end()
    }
}

pub struct TorrentTracker {
    mode: TrackerMode,
    database: TorrentDatabase,
}

pub enum TorrentStats {
    TorrentFlagged,
    TorrentNotRegistered,
    Stats{
        seeders: u32,
        leechers: u32,
        complete: u32,
    }
}

impl TorrentTracker {
    pub fn new(mode: TrackerMode) -> TorrentTracker {
        TorrentTracker{
            mode,
            database: TorrentDatabase{
                torrent_peers: std::sync::RwLock::new(std::collections::BTreeMap::new()),
            }
        }
    }

    /// Adding torrents is not relevant to dynamic trackers.
    pub fn add_torrent(&self, info_hash: &InfoHash) -> Result<(), ()> {
        let mut write_lock = self.database.torrent_peers.write().unwrap();
        match write_lock.entry(*info_hash) {
            std::collections::btree_map::Entry::Vacant(ve) => {
                ve.insert(TorrentEntry::new());
                return Ok(());
            },
            std::collections::btree_map::Entry::Occupied(_entry) => {
                return Err(());
            }
        }
    }

    /// If the torrent is flagged, it will not be removed unless force is set to true.
    pub fn remove_torrent(&self, info_hash: &InfoHash, force: bool) -> Result<(), ()> {
        use std::collections::btree_map::Entry;
        let mut entry_lock = self.database.torrent_peers.write().unwrap();
        let torrent_entry = entry_lock.entry(*info_hash);
        match torrent_entry {
            Entry::Vacant(_) => {
                // no entry, nothing to do...
                return Err(());
            },
            Entry::Occupied(entry) => {
                if force || !entry.get().is_flagged() {
                    entry.remove();
                    return Ok(());
                }
                return Err(());
            },
        }
    }

    /// flagged torrents will result in a tracking error. This is to allow enforcement against piracy.
    pub fn set_torrent_flag(&self, info_hash: &InfoHash, is_flagged: bool) {
        if let Some(entry) = self.database.torrent_peers.write().unwrap().get_mut(info_hash) {
            if is_flagged && !entry.is_flagged {
                // empty peer list.
                entry.peers.clear();
            }
            entry.is_flagged = is_flagged;
        }
    }

    pub fn get_torrent_peers(&self, info_hash: &InfoHash, remote_addr: &std::net::SocketAddr) -> Option<Vec<std::net::SocketAddr>> {
        let read_lock = self.database.torrent_peers.read().unwrap();
        match read_lock.get(info_hash) {
            None => {
                return None;
            }
            Some(entry) => {
                return Some(entry.get_peers(remote_addr));
            }
        };
    }

    pub fn update_torrent_and_get_stats(&self, info_hash: &InfoHash, peer_id: &PeerId, remote_address: &std::net::SocketAddr, uploaded: u64, downloaded: u64, left: u64, event: Events) -> TorrentStats {
        use std::collections::btree_map::Entry;
        let mut torrent_peers = self.database.torrent_peers.write().unwrap();
        let torrent_entry = match torrent_peers.entry(*info_hash) {
            Entry::Vacant(vacant) => {
                match self.mode {
                    TrackerMode::DynamicMode => {
                        vacant.insert(TorrentEntry::new())
                    },
                    _ => {
                        return TorrentStats::TorrentNotRegistered;
                    }
                }
            },
            Entry::Occupied(entry) => {
                if entry.get().is_flagged() {
                    return TorrentStats::TorrentFlagged;
                }
                entry.into_mut()
            },
        };

        torrent_entry.update_peer(peer_id, remote_address, uploaded, downloaded, left, event);

        let (seeders, complete, leechers) = torrent_entry.get_stats();

        return TorrentStats::Stats {
            seeders,
            leechers,
            complete,
        };
    }

    pub (crate) fn get_database(&self) -> std::sync::RwLockReadGuard<std::collections::BTreeMap<InfoHash, TorrentEntry>>{
        self.database.torrent_peers.read().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sync<T: Sync>() {}
    fn is_send<T: Send>() {}

    #[test]
    fn tracker_send() {
        is_send::<TorrentTracker>();
    }

    #[test]
    fn tracker_sync() {
        is_sync::<TorrentTracker>();
    }
}