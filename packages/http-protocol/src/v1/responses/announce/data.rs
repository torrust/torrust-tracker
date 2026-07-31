//! DTO (Data Transfer Object) types for the HTTP tracker announce response.
//!
//! These are transport-agnostic types describing *what* data goes in the response,
//! without any encoding logic. They use domain-friendly types (`PeerId`, `SocketAddr`).
use std::net::SocketAddr;

use derive_more::Constructor;
use torrust_peer_id::PeerId;

// Protocol-local announce response DTOs intentionally duplicate some domain
// field shapes. This keeps protocol crates decoupled from tracker domain types
// and centralizes conversions in boundary adapters.
// adr: docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md
#[derive(Clone, Debug, PartialEq, Constructor, Default)]
pub struct AnnounceData {
    pub peers: Vec<Peer>,
    pub stats: SwarmMetadata,
    pub policy: AnnouncePolicy,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Constructor)]
pub struct AnnouncePolicy {
    pub interval: u32,
    pub interval_min: u32,
}

impl Default for AnnouncePolicy {
    fn default() -> Self {
        Self {
            interval: 120,
            interval_min: 120,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SwarmMetadata {
    pub complete: u32,
    pub downloaded: u32,
    pub incomplete: u32,
}

impl SwarmMetadata {
    #[must_use]
    pub const fn new(complete: u32, downloaded: u32, incomplete: u32) -> Self {
        Self {
            complete,
            downloaded,
            incomplete,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Peer {
    pub peer_id: PeerId,
    pub peer_addr: PeerAddress,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PeerAddress {
    Clearnet(SocketAddr),
    I2p {
        destination: String,
        destination_hash: [u8; 32],
        port: u16,
    },
}
