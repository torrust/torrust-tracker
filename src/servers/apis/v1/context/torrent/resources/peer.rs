//! `Peer` and Peer `Id` API resources.
use derive_more::From;
use serde::{Deserialize, Serialize};
use torrust_tracker_primitives::peer;

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
    /// See [`AnnounceEvent`](aquatic_udp_protocol::AnnounceEvent).
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

impl From<peer::Id> for Id {
    fn from(peer_id: peer::Id) -> Self {
        Id {
            id: peer_id.to_hex_string(),
            client: peer_id.get_client_name(),
        }
    }
}

impl From<peer::Peer> for Peer {
    fn from(value: peer::Peer) -> Self {
        #[allow(deprecated)]
        Peer {
            peer_id: Id::from(value.peer_id),
            peer_addr: value.peer_addr.to_string(),
            updated: value.updated.as_millis(),
            updated_milliseconds_ago: value.updated.as_millis(),
            uploaded: value.uploaded.0,
            downloaded: value.downloaded.0,
            left: value.left.0,
            event: format!("{:?}", value.event),
        }
    }
}

#[derive(From, PartialEq, Default)]
pub struct Vector(pub Vec<Peer>);

impl FromIterator<peer::Peer> for Vector {
    fn from_iter<T: IntoIterator<Item = peer::Peer>>(iter: T) -> Self {
        let mut peers = Vector::default();

        for i in iter {
            peers.0.push(i.into());
        }
        peers
    }
}
