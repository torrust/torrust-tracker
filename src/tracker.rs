use crate::server::{AnnounceEvent};
use log::{error, trace};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;
use tokio::io::AsyncBufReadExt;
use tokio::sync::RwLock;
use crate::common::{NumberOfBytes, InfoHash};
use super::common::*;

const TWO_HOURS: std::time::Duration = std::time::Duration::from_secs(3600 * 2);

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
    ip: std::net::SocketAddr,
    #[serde(serialize_with = "ser_instant")]
    updated: std::time::Instant,
    uploaded: NumberOfBytes,
    downloaded: NumberOfBytes,
    left: NumberOfBytes,
    event: AnnounceEvent,
}

impl TorrentPeer {
    fn is_seeder(&self) -> bool { self.left.0 == 0 && self.event != AnnounceEvent::Stopped }

    fn is_leecher(&self) -> bool {
        self.left.0 > 0 && self.event != AnnounceEvent::Stopped
    }

    fn is_stopped(&self) -> bool {
        self.event == AnnounceEvent::Stopped
    }

    fn is_completed(&self) -> bool {
        self.event == AnnounceEvent::Completed
    }
}

fn ser_instant<S: serde::Serializer>(inst: &std::time::Instant, ser: S) -> Result<S::Ok, S::Error> {
    ser.serialize_u64(inst.elapsed().as_millis() as u64)
}

struct InfoHashVisitor;

impl<'v> serde::de::Visitor<'v> for InfoHashVisitor {
    type Value = InfoHash;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a 40 character long hash")
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        if v.len() != 40 {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &"expected a 40 character long string",
            ));
        }

        let mut res = InfoHash { 0: [0u8; 20] };

        if let Err(_) = binascii::hex2bin(v.as_bytes(), &mut res.0) {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &"expected a hexadecimal string",
            ));
        } else {
            return Ok(res);
        }
    }
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

    pub fn update_peer(
        &mut self,
        peer_id: &PeerId,
        torrent_peer: TorrentPeer
    ) {
        let is_seeder = torrent_peer.is_seeder().clone();
        let is_completed = torrent_peer.is_completed().clone();

        let torrent_peer_prev = self.peers.insert(
            peer_id.clone(),
            torrent_peer
        );

        match torrent_peer_prev {
            None => {
                self.update_stats(is_seeder, is_completed, false, false);
            }
            Some(torrent_peer_prev) => {
                self.update_stats(is_seeder, is_completed, torrent_peer_prev.is_seeder(), torrent_peer_prev.is_completed());
            }
        }
    }

    pub fn get_peers(&self, remote_addr: &std::net::SocketAddr) -> Vec<std::net::SocketAddr> {
        let mut list = Vec::new();
        for (_, peer) in self
            .peers
            .iter()
            .filter(|e| e.1.ip.is_ipv4() == remote_addr.is_ipv4())
            .take(74)
        {
            if peer.ip == *remote_addr {
                continue;
            }

            list.push(peer.ip);
        }
        list
    }

    pub fn get_peers_iter(&self) -> impl Iterator<Item = (&PeerId, &TorrentPeer)> {
        self.peers.iter()
    }

    pub fn update_stats(&mut self, is_seeder: bool, is_completed: bool, was_seeder: bool, was_completed: bool) {
        if is_seeder && !was_seeder {
            self.seeders += 1;
        } else if was_seeder && !is_seeder {
            self.seeders -= 1;
        }

        // don't double count completed events for one peer
        if is_completed && !was_completed {
            self.completed += 1;
        }
    }

    pub fn get_stats(&self) -> (u32, u32, u32) {
        let leechers = (self.peers.len() as u32) - self.seeders;
        (self.seeders, self.completed, leechers)
    }

    pub fn is_flagged(&self) -> bool {
        self.is_flagged
    }
}

struct TorrentDatabase {
    torrent_peers: tokio::sync::RwLock<std::collections::BTreeMap<InfoHash, TorrentEntry>>,
}

impl Default for TorrentDatabase {
    fn default() -> Self {
        TorrentDatabase {
            torrent_peers: tokio::sync::RwLock::new(std::collections::BTreeMap::new()),
        }
    }
}

pub struct TorrentTracker {
    mode: TrackerMode,
    database: TorrentDatabase,
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

impl TorrentTracker {
    pub fn new(mode: TrackerMode) -> TorrentTracker {
        TorrentTracker {
            mode,
            database: TorrentDatabase {
                torrent_peers: RwLock::new(std::collections::BTreeMap::new()),
            },
        }
    }

    pub async fn load_database<R: tokio::io::AsyncRead + Unpin>(
        mode: TrackerMode, reader: &mut R,
    ) -> Result<TorrentTracker, std::io::Error> {
        let reader = tokio::io::BufReader::new(reader);
        let reader = async_compression::tokio::bufread::BzDecoder::new(reader);
        let reader = tokio::io::BufReader::new(reader);

        let res = TorrentTracker::new(mode);
        let mut db = res.database.torrent_peers.write().await;

        let mut records = reader.lines();
        loop {
            let line = match records.next_line().await {
                Ok(Some(v)) => v,
                Ok(None) => break,
                Err(ref err) => {
                    error!("failed to read lines! {}", err);
                    continue;
                },
            };
            let row: DatabaseRow = match serde_json::from_str(&line) {
                Ok(v) => v,
                Err(err) => {
                    error!("failed to parse json: {}", err);
                    continue;
                }
            };
            let entry = row.entry.into_owned();
            let infohash = row.info_hash;
            db.insert(infohash, entry);
        }

        trace!("loaded {} entries from database", db.len());

        drop(db);

        Ok(res)
    }

    /// Adding torrents is not relevant to dynamic trackers.
    pub async fn add_torrent(&self, info_hash: &InfoHash) -> Result<(), ()> {
        let mut write_lock = self.database.torrent_peers.write().await;
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
        use std::collections::btree_map::Entry;
        let mut entry_lock = self.database.torrent_peers.write().await;
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
        if let Some(entry) = self.database.torrent_peers.write().await.get_mut(info_hash) {
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
        remote_addr: &std::net::SocketAddr
    ) -> Option<Vec<std::net::SocketAddr>> {
        let read_lock = self.database.torrent_peers.read().await;
        match read_lock.get(info_hash) {
            None => {
                None
            }
            Some(entry) => {
                Some(entry.get_peers(remote_addr))
            }
        }
    }

    pub async fn update_torrent_and_get_stats(
        &self,
        remote_address: &std::net::SocketAddr,
        info_hash: &InfoHash,
        peer_id: &PeerId,
        uploaded: &NumberOfBytes,
        downloaded: &NumberOfBytes,
        left: &NumberOfBytes,
        event: &AnnounceEvent,
    ) -> Result<TorrentStats, TorrentError> {
        use std::collections::btree_map::Entry;
        let mut torrent_peers = self.database.torrent_peers.write().await;

        let torrent_entry = match torrent_peers.entry(info_hash.clone()) {
            Entry::Vacant(vacant) => {
                // todo: support multiple tracker modes
                match self.mode {
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
                torrent_entry.update_peer(
                    peer_id,
                    TorrentPeer {
                        ip: remote_address.clone(),
                        updated: std::time::Instant::now(),
                        uploaded: uploaded.clone(),
                        downloaded: downloaded.clone(),
                        left: left.clone(),
                        event: event.clone(),
                    }
                );

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

    pub(crate) async fn get_database<'a>(&'a self) -> tokio::sync::RwLockReadGuard<'a, BTreeMap<InfoHash, TorrentEntry>> {
        self.database.torrent_peers.read().await
    }

    pub async fn save_database<W: tokio::io::AsyncWrite + Unpin>(&self, w: W) -> Result<(), std::io::Error> {
        use tokio::io::AsyncWriteExt;

        let mut writer = async_compression::tokio::write::BzEncoder::new(w);

        let db_lock = self.database.torrent_peers.read().await;

        let db: &BTreeMap<InfoHash, TorrentEntry> = &*db_lock;
        let mut tmp = Vec::with_capacity(4096);

        for row in db {
            let entry = DatabaseRow {
                info_hash: row.0.clone(),
                entry: Cow::Borrowed(row.1),
            };
            tmp.clear();
            if let Err(err) = serde_json::to_writer(&mut tmp, &entry) {
                error!("failed to serialize: {}", err);
                continue;
            };
            tmp.push(b'\n');
            writer.write_all(&tmp).await?;
        }
        writer.flush().await?;
        Ok(())
    }

    async fn cleanup(&self) {
        let mut lock = self.database.torrent_peers.write().await;
        let db: &mut BTreeMap<InfoHash, TorrentEntry> = &mut *lock;
        let mut torrents_to_remove = Vec::new();

        for (k, v) in db.iter_mut() {
            // timed-out peers..
            {
                let mut peers_to_remove = Vec::new();
                let torrent_peers = &mut v.peers;

                for (peer_id, state) in torrent_peers.iter() {
                    if state.updated.elapsed() > TWO_HOURS {
                        // over 2 hours past since last update...
                        peers_to_remove.push(*peer_id);
                    }
                }

                for peer_id in peers_to_remove.iter() {
                    torrent_peers.remove(peer_id);
                }
            }

            if self.mode == TrackerMode::DynamicMode {
                // peer-less torrents..
                if v.peers.len() == 0 && !v.is_flagged() {
                    torrents_to_remove.push(k.clone());
                }
            }
        }

        for info_hash in torrents_to_remove {
            db.remove(&info_hash);
        }
    }

    pub async fn periodic_task(&self, db_path: &str) {
        // cleanup db
        self.cleanup().await;

        // save journal db.
        let mut journal_path = std::path::PathBuf::from(db_path);

        let mut filename = String::from(journal_path.file_name().unwrap().to_str().unwrap());
        filename.push_str("-journal");

        journal_path.set_file_name(filename.as_str());
        let jp_str = journal_path.as_path().to_str().unwrap();

        // scope to make sure backup file is dropped/closed.
        {
            let mut file = match tokio::fs::File::create(jp_str).await {
                Err(err) => {
                    error!("failed to open file '{}': {}", db_path, err);
                    return;
                }
                Ok(v) => v,
            };
            trace!("writing database to {}", jp_str);
            if let Err(err) = self.save_database(&mut file).await {
                error!("failed saving database. {}", err);
                return;
            }
        }

        // overwrite previous db
        trace!("renaming '{}' to '{}'", jp_str, db_path);
        if let Err(err) = tokio::fs::rename(jp_str, db_path).await {
            error!("failed to move db backup. {}", err);
        }
    }
}
