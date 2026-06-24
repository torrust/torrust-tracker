//! `Peer` and Peer `Id` API resources.
use serde::{Deserialize, Serialize};

/// `Peer` API resource.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Peer {
    /// The peer's ID. See [`Id`].
    pub peer_id: Id,
    /// The peer's socket address. For example: `192.168.1.88:17548`.
    pub peer_addr: String,
    /// The peer's last update time as a Unix timestamp in milliseconds since epoch.
    #[deprecated(since = "2.0.0", note = "please use `updated_milliseconds_ago` instead")]
    pub updated: u128,
    /// Milliseconds since the peer's last update (relative to the response generation time).
    /// Note: despite the `_ago` suffix, this field is populated with the **absolute Unix timestamp**
    /// in milliseconds (the same value as the deprecated `updated` field), not a relative duration.
    /// The name is a historical artifact — see issue #1930 follow-up tasks for the planned rename.
    #[allow(clippy::doc_markdown)]
    pub updated_milliseconds_ago: u128,
    /// The peer's uploaded bytes.
    pub uploaded: i64,
    /// The peer's downloaded bytes.
    pub downloaded: i64,
    /// The peer's left bytes (pending to download).
    pub left: i64,
    /// The peer's event: `Started`, `Stopped`, `Completed`, `None` (`PascalCase`).
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
