//! Client-side announce response deserialization types.
//!
//! These types are the reverse of the DTO layer — they deserialize bencoded
//! announce responses from the wire. Use wire-friendly types (`Vec<u8>`, `String`).

use serde::{Deserialize, Serialize};

/// Non-compact announce response (BEP 3 dictionary format).
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DeserializedNormal {
    pub complete: u32,
    pub incomplete: u32,
    pub interval: u32,
    #[serde(rename = "min interval")]
    pub min_interval: u32,
    pub peers: Vec<DictionaryPeer>,
}

/// A peer in dictionary format (BEP 3).
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DictionaryPeer {
    pub ip: String,
    #[serde(rename = "peer id")]
    #[serde(with = "serde_bytes")]
    pub peer_id: Vec<u8>,
    pub port: u16,
}

/// Raw compact announce response (BEP 23) from serde deserialization.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DeserializedCompact {
    pub complete: u32,
    pub incomplete: u32,
    pub interval: u32,
    #[serde(rename = "min interval")]
    pub min_interval: u32,
    #[serde(with = "serde_bytes")]
    pub peers: Vec<u8>,
    /// IPv6 compact peer list (BEP 7). Raw bytes from deserialization.
    #[serde(default)]
    #[serde(with = "serde_bytes")]
    pub peers6: Vec<u8>,
}

impl DeserializedCompact {
    /// # Errors
    ///
    /// Will return an error if bytes can't be deserialized.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, serde_bencode::Error> {
        serde_bencode::from_bytes::<DeserializedCompact>(bytes)
    }
}

/// Parsed compact announce response with peer entries extracted.
#[derive(Debug, PartialEq)]
pub struct DeserializedCompactParsed {
    pub complete: u32,
    pub incomplete: u32,
    pub interval: u32,
    pub min_interval: u32,
    pub peers: CompactPeerList,
}

pub use crate::v1::responses::announce::encoding::CompactPeer;

/// A list of compact peer entries.
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

impl From<DeserializedCompact> for DeserializedCompactParsed {
    fn from(compact_announce: DeserializedCompact) -> Self {
        let mut peers = vec![];

        #[allow(clippy::chunks_exact_to_as_chunks, clippy::explicit_iter_loop)]
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
