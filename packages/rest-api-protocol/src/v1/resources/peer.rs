//! `Peer` and Peer `Id` API resources.
use serde::{Deserialize, Serialize};

/// `Peer` API resource.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Peer {
    /// The peer's ID. See [`Id`].
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
