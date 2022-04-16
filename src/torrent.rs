use std::borrow::Cow;
use std::net::{IpAddr, SocketAddr};

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use serde::{Deserialize, Serialize};

use crate::{InfoHash, MAX_SCRAPE_TORRENTS, PeerId};
use crate::common::{AnnounceEventDef, NumberOfBytesDef};
use crate::torrust_http_tracker::AnnounceRequest;

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
            event: announce_request.event,
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
            event,
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

    pub(crate) fn is_seeder(&self) -> bool { self.left.0 <= 0 && self.event != AnnounceEvent::Stopped }

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
    pub(crate) peers: std::collections::BTreeMap<PeerId, TorrentPeer>,
    pub(crate) completed: u32,
    #[serde(skip)]
    pub(crate) seeders: u32,
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
                            if self.seeders != 0 {
                                self.seeders -= 1;
                            }
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
