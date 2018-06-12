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

type PeerId = [u8; 20];
type InfoHash = [u8; 20];

pub struct TorrentEntry {
    is_flagged: bool,
    peers: std::collections::BTreeMap<PeerId, TorrentPeer>,

    completed: u32,
    seeders: u32,
}

impl TorrentEntry {
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
        for (_, peer) in self.peers.iter().filter(|e| e.1.ip.is_ipv4() == remote_addr.is_ipv4()) {
            if peer.ip == *remote_addr {
                continue;
            }

            list.push(peer.ip);

            if list.len() >= 74 {
                // 74 is maximum peers supported by the protocol.
                break;
            }
        }
        list
    }

    pub fn get_stats(&self) -> (u32, u32, u32) {
        let leechers = (self.peers.len() as u32) - self.seeders;
        (self.seeders, self.completed, leechers)
    }
}

struct TorrentDatabase {
    torrent_peers: std::collections::BTreeMap<InfoHash, TorrentEntry>,
}

pub struct TorrentTracker {
    mode: TrackerMode,
    database: TorrentDatabase,
}

impl TorrentTracker {
    pub fn new() -> TorrentTracker {
        TorrentTracker{
            mode: TrackerMode::DynamicMode,
            database: TorrentDatabase{
                torrent_peers: std::collections::BTreeMap::new(),
            }
        }
    }

    /// Adding torrents is not relevant to dynamic trackers.
    pub fn add_torrent(&mut self, info_hash: &InfoHash) {
        self.database.torrent_peers.entry(*info_hash).or_insert(TorrentEntry{
            is_flagged: false,
            peers: std::collections::BTreeMap::new(),
            seeders: 0,
            completed: 0,
        });
    }

    /// If the torrent is flagged, it will not be removed unless force is set to true.
    pub fn remove_torrent(&mut self, info_hash: &InfoHash, force: bool) {
        if !force {
            if let Some(entry) = self.database.torrent_peers.get(info_hash) {
                if entry.is_flagged {
                    // torrent is flagged, ignore request.
                    return;
                }
            } else {
                // torrent not found, no point looking for it again...
                return;
            }
        }
        self.database.torrent_peers.remove(info_hash);
    }

    /// flagged torrents will result in a tracking error. This is to allow enforcement against piracy.
    pub fn set_torrent_flag(&mut self, info_hash: &InfoHash, is_flagged: bool) {
        if let Some(mut entry) = self.database.torrent_peers.get_mut(info_hash) {
            if is_flagged && !entry.is_flagged {
                // empty peer list.
                entry.peers.clear();
            }
            entry.is_flagged = is_flagged;
        }
    }

    pub fn get_torrent<F, R>(&mut self, info_hash: &InfoHash, action: F) -> Option<R>
    where F: Fn(&mut TorrentEntry) -> R
    {
        if let Some(torrent_entry) = self.database.torrent_peers.get_mut(info_hash) {
            Some(action(torrent_entry))
        } else {
            match self.mode {
                TrackerMode::StaticMode => None,
                TrackerMode::PrivateMode => None,
                TrackerMode::DynamicMode => {
                    None
                }
            }
        }
    }

    pub fn cleanup(&mut self) {
        
    }
}
