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
    last_connection_id: u64,
    uploaded: u64,
    downloaded: u64,
    left: u64,
    event: Events,
    updated: std::time::SystemTime,
}

type PeerId = [u8; 20];
type InfoHash = [u8; 20];

struct TorrentEntry {
    is_flagged: bool,
    peers: std::collections::BTreeMap<PeerId, TorrentPeer>,
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
        use std::collections::BTreeMap;
        self.database.torrent_peers.entry(*info_hash).or_insert(TorrentEntry{
            is_flagged: false,
            peers: std::collections::BTreeMap::new(),
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

    pub fn update_torrent_peer(&mut self, info_hash: &InfoHash, peer_id: &PeerId, remote_address: &std::net::SocketAddr, uploaded: u64, downloaded: u64, left: u64, event: Events) {
        if let Some(mut torrent_entry) = self.database.torrent_peers.get_mut(info_hash) {
            torrent_entry.peers.insert(*peer_id, TorrentPeer{
                updated: std::time::SystemTime::now(),
                left,
                downloaded,
                uploaded,
                ip: *remote_address,
                event,
                last_connection_id: 0,
            });
        }
    }

    /// returns a list of peers with the same type of address of the remote_addr (IP v4/v6)
    pub fn get_peers(&self, info_hash: &InfoHash, remote_addr: &std::net::SocketAddr) -> Vec<std::net::SocketAddr> {
        let mut list = Vec::new();
        if let Some(entry) = self.database.torrent_peers.get(info_hash) {
            for (_, peer) in entry.peers.iter().filter(|e| e.1.ip.is_ipv4() == remote_addr.is_ipv4()) {
                if peer.ip == *remote_addr {
                    continue;
                }

                list.push(peer.ip);

                if list.len() >= 74 {
                    // 74 is maximum peers supported by the protocol.
                    break;
                }
            }
        }
        list
    }

    pub fn get_stats(&self, info_hash: &InfoHash) -> Option<(i32, i32, i32)> {
        if let Some(torrent_entry) = self.database.torrent_peers.get(info_hash) {

            // TODO: store stats in temporary location...
            let mut seeders = 0;
            let mut leechers = 0;

            for (_, peer) in torrent_entry.peers.iter() {
                if peer.left == 0 {
                    seeders += 1;
                }
                else {
                    leechers += 1;
                }
            }

            Some((seeders, -1, leechers))
        } else {
            None
        }
    }

    pub fn cleanup(&mut self) {
        
    }
}
