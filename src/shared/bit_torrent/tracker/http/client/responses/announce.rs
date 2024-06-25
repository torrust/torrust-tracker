use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::body::Bytes;
use serde::{Deserialize, Serialize};
use torrust_tracker_primitives::peer;

use super::{Announce, BencodeParseError};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
pub(super) struct Response {
    pub complete: u32,
    pub incomplete: u32,
    pub interval: u32,
    #[serde(rename = "min interval")]
    pub min_interval: u32,
    pub peers: Vec<DictionaryPeer>, // Peers using IPV4 and IPV6
}

pub struct ResponseBuilder {
    response: Response,
}

impl ResponseBuilder {
    #[must_use]
    pub fn new(policy: &torrust_tracker_configuration::AnnouncePolicy) -> Self {
        Self {
            response: Response {
                interval: policy.interval,
                min_interval: policy.interval_min,
                ..Default::default()
            },
        }
    }

    #[must_use]
    pub fn with_complete(mut self, complete: u32) -> Self {
        self.response.complete = complete;
        self
    }

    #[must_use]
    pub fn with_peers(mut self, peers: Vec<DictionaryPeer>) -> Self {
        self.response.peers = peers;
        self
    }
}

impl TryFrom<&Bytes> for ResponseBuilder {
    type Error = BencodeParseError;

    /// # Errors
    ///
    /// Will return an error if the deserialized bencoded response can't not be converted into a valid response.
    fn try_from(value: &Bytes) -> Result<Self, Self::Error> {
        let response: Response = serde_bencode::from_bytes(value).map_err(|e| BencodeParseError::ParseSerdeBencodeError {
            data: value.to_vec(),
            err: e.into(),
        })?;

        Ok(Self { response })
    }
}

impl ResponseBuilder {
    #[must_use]
    pub fn build(self) -> Announce {
        self.response.into()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct DictionaryPeer {
    pub ip: String,
    #[serde(rename = "peer id")]
    #[serde(with = "serde_bytes")]
    pub peer_id: Vec<u8>,
    pub port: u16,
}

impl From<peer::Peer> for DictionaryPeer {
    fn from(peer: peer::Peer) -> Self {
        DictionaryPeer {
            peer_id: peer.peer_id.to_bytes().to_vec(),
            ip: peer.peer_addr.ip().to_string(),
            port: peer.peer_addr.port(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DeserializedCompact {
    pub complete: u32,
    pub incomplete: u32,
    pub interval: u32,
    #[serde(rename = "min interval")]
    pub min_interval: u32,
    #[serde(with = "serde_bytes")]
    pub peers: Vec<u8>,
}

impl DeserializedCompact {
    /// # Errors
    ///
    /// Will return an error if bytes can't be deserialized.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, serde_bencode::Error> {
        serde_bencode::from_bytes::<DeserializedCompact>(bytes)
    }
}

#[derive(Debug, PartialEq)]
pub struct Compact {
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
    #[must_use]
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
    /// # Panics
    ///
    /// Will panic if the provided socket address is a IPv6 IP address.
    /// It's not supported for compact peers.
    #[must_use]
    pub fn new(socket_addr: &SocketAddr) -> Self {
        match socket_addr.ip() {
            IpAddr::V4(ip) => Self {
                ip,
                port: socket_addr.port(),
            },
            IpAddr::V6(_ip) => panic!("IPV6 is not supported for compact peer"),
        }
    }

    #[must_use]
    pub fn new_from_bytes(bytes: &[u8]) -> Self {
        Self {
            ip: Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]),
            port: u16::from_be_bytes([bytes[4], bytes[5]]),
        }
    }
}

impl From<DeserializedCompact> for Compact {
    fn from(compact_announce: DeserializedCompact) -> Self {
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
