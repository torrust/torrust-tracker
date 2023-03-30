//! `Peer` and Peer `Id` API resources.
use serde::{Deserialize, Serialize};

use crate::tracker;

/// `Peer` API resource.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Peer {
    /// The peer's ID. See [`Id`](crate::servers::apis::v1::context::torrent::resources::peer::Id).
    pub peer_id: Id,
    /// The peer's socket address. For example: `192.168.1.88:17548`.
    pub peer_addr: String,
    /// The peer's last update time in milliseconds.
    #[deprecated(since = "2.0.0", note = "please use `updated_milliseconds_ago` instead")]
    pub updated: u128,
    /// The peer's last update time in milliseconds.
    pub updated_milliseconds_ago: u128,
    /// The peer's uploaded bytes.
    pub uploaded: i64,
    /// The peer's downloaded bytes.
    pub downloaded: i64,
    /// The peer's left bytes (pending to download).
    pub left: i64,
    /// The peer's event: `started`, `stopped`, `completed`.
    /// See [`AnnounceEventDef`](crate::shared::bit_torrent::common::AnnounceEventDef).
    pub event: String,
}

/// Peer `Id` API resource.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Id {
    /// The peer's ID in hex format. For example: `0x2d7142343431302d2a64465a3844484944704579`.
    pub id: Option<String>,
    /// The peer's client name. For example: `qBittorrent`.
    pub client: Option<String>,
}

impl From<tracker::peer::Id> for Id {
    fn from(peer_id: tracker::peer::Id) -> Self {
        Id {
            id: peer_id.to_hex_string(),
            client: peer_id.get_client_name().map(std::string::ToString::to_string),
        }
    }
}

impl From<tracker::peer::Peer> for Peer {
    #[allow(deprecated)]
    fn from(peer: tracker::peer::Peer) -> Self {
        Peer {
            peer_id: Id::from(peer.peer_id),
            peer_addr: peer.peer_addr.to_string(),
            updated: peer.updated.as_millis(),
            updated_milliseconds_ago: peer.updated.as_millis(),
            uploaded: peer.uploaded.0,
            downloaded: peer.downloaded.0,
            left: peer.left.0,
            event: format!("{:?}", peer.event),
        }
    }
}
