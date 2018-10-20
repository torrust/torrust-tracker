use std;

use server::Events;

pub enum TrackerMode {

    /// In static mode torrents are tracked only if they were added ahead of time.
    StaticMode,

    /// In dynamic mode, torrents are tracked being added ahead of time.
    DynamicMode,

    /// Tracker will only serve authenticated peers.
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

pub trait HexConv {
    fn to_hex(&self) -> String;
    fn from_hex(hex: &str) -> Option<InfoHash>;
}

impl HexConv for InfoHash {
    fn to_hex(&self) -> String {
        const HEX: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
        let data = self;
        let mut res = String::with_capacity(data.len() * 2);
        for b in data {
            res.push(HEX[((b >> 4) & 0x0fu8) as usize]);
            res.push(HEX[(b & 0x0fu8) as usize]);
        }
        return res;
    }

    fn from_hex(hex: &str) -> Option<InfoHash> {
        let mut res : InfoHash = [0u8; 20];
        if hex.len() != 2 * res.len() {
            return None;
        }

        let mut tmp = 0;
        let mut idx = 0;

        for ch in hex.chars() {
            if idx % 2 == 1 {
                tmp <<= 4;
            }

            let mut num = ch as u32;
            if ch >= '0' && ch <= '9' {
                num -= '0' as u32;
            } else if ch >= 'a' && ch <= 'f' {
                num -= 'a' as u32;
                num += 10;
            } else if ch >= 'A' && ch <= 'F' {
                num -= 'A' as u32;
                num += 10;
            } else {
                return None;
            }

            tmp |= num & 0x0f;

            if idx % 2 == 1 {
                res[(idx - 1) / 2] = tmp as u8;
                tmp = 0;
            }

            idx += 1;
        }

        return Some(res);
    }
}

pub struct TorrentEntry {
    is_flagged: bool,
    peers: std::collections::BTreeMap<PeerId, TorrentPeer>,

    completed: u32,
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
    pub fn new() -> TorrentTracker {
        TorrentTracker{
            mode: TrackerMode::DynamicMode,
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
            std::collections::btree_map::Entry::Occupied(entry) => {
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

    #[test]
    fn test_ih2hex() {
        let ih: InfoHash = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 0xa,
            0xb, 0xc, 0xd, 0xe, 0xf, 0x10, 0x11, 0x12, 0x13, 0xff];
        assert!(ih.to_hex() == "0102030405060708090a0b0c0d0e0f10111213ff");
    }

    #[test]
    fn test_hex2ih() {
        let ih = InfoHash::from_hex("0102030405060708090a0b0c0d0e0f10111213ff").unwrap();
        assert_eq!(ih, [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 0xa,
            0xb, 0xc, 0xd, 0xe, 0xf, 0x10, 0x11, 0x12, 0x13, 0xff]);
    }
}