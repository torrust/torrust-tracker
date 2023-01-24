use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Announce {
    pub complete: u32,
    pub incomplete: u32,
    pub interval: u32,
    #[serde(rename = "min interval")]
    pub min_interval: u32,
    pub peers: Vec<DictionaryPeer>, // Peers with IPV4
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DictionaryPeer {
    pub ip: String,
    pub peer_id: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CompactAnnounce {
    pub complete: u32,
    pub incomplete: u32,
    pub interval: u32,
    #[serde(rename = "min interval")]
    pub min_interval: u32,
    #[serde(with = "serde_bytes")]
    pub peers: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct DecodedCompactAnnounce {
    // code-review: there could be a way to deserialize this struct directly
    // by using serde instead of doing it manually. Or at least using a custom deserializer.
    pub complete: u32,
    pub incomplete: u32,
    pub interval: u32,
    pub min_interval: u32,
    pub peers: CompactPeerList,
}

#[derive(Debug, PartialEq)]
pub struct CompactPeerList {
    peers: Vec<CompactPeer>,
}

impl CompactPeerList {
    pub fn new(peers: Vec<CompactPeer>) -> Self {
        Self { peers }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompactPeer {
    ip: Ipv4Addr,
    port: u16,
}

impl CompactPeer {
    pub fn new(socket_addr: &SocketAddr) -> Self {
        match socket_addr.ip() {
            IpAddr::V4(ip) => Self {
                ip,
                port: socket_addr.port(),
            },
            IpAddr::V6(_ip) => panic!("IPV6 is not supported for compact peer"),
        }
    }

    pub fn new_from_bytes(bytes: &[u8]) -> Self {
        Self {
            ip: Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]),
            port: u16::from_be_bytes([bytes[4], bytes[5]]),
        }
    }
}

impl From<CompactAnnounce> for DecodedCompactAnnounce {
    fn from(compact_announce: CompactAnnounce) -> Self {
        let mut peers = vec![];

        for peer_bytes in compact_announce.peers.chunks_exact(6) {
            peers.push(CompactPeer::new_from_bytes(peer_bytes));
        }

        Self {
            complete: compact_announce.complete,
            incomplete: compact_announce.incomplete,
            interval: compact_announce.interval,
            min_interval: compact_announce.min_interval,
            peers: CompactPeerList::new(peers),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Error {
    #[serde(rename = "failure reason")]
    pub failure_reason: String,
}
