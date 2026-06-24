//! `Torrent` and `ListItem` API resources.
use serde::{Deserialize, Serialize};

use crate::v1::resources::peer::Peer;

/// `Torrent` API resource.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Torrent {
    /// The torrent's info hash v1.
    pub info_hash: String,
    /// The torrent's seeders counter. Active peers with a full copy of the
    /// torrent.
    pub seeders: u64,
    /// The torrent's completed counter. Peers that have ever completed the
    /// download.
    pub completed: u64,
    /// The torrent's leechers counter. Active peers that are downloading the
    /// torrent.
    pub leechers: u64,
    /// The torrent's peers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peers: Option<Vec<Peer>>,
}

/// `ListItem` API resource. A list item on a torrent list.
/// `ListItem` does not include a `peers` field.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ListItem {
    /// The torrent's info hash v1.
    pub info_hash: String,
    /// The torrent's seeders counter. Active peers with a full copy of the
    /// torrent.
    pub seeders: u64,
    /// The torrent's completed counter. Peers that have ever completed the
    /// download.
    pub completed: u64,
    /// The torrent's leechers counter. Active peers that are downloading the
    /// torrent.
    pub leechers: u64,
}
