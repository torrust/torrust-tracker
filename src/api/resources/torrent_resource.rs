use serde::{Deserialize, Serialize};

use crate::PeerId;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TorrentResource {
    pub info_hash: String,
    pub seeders: u32,
    pub completed: u32,
    pub leechers: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peers: Option<Vec<TorrentPeerResource>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TorrentPeerResource {
    pub peer_id: PeerIdResource,
    pub peer_addr: String,
    pub updated: u128,
    pub uploaded: i64,
    pub downloaded: i64,
    pub left: i64,
    pub event: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PeerIdResource {
    pub id: Option<String>,
    pub client: Option<String>,
}

impl From<PeerId> for PeerIdResource {
    fn from(peer_id: PeerId) -> Self {
        PeerIdResource {
            id: peer_id.get_id(),
            client: peer_id.get_client_name().map(|client_name| client_name.to_string()),
        }
    }
}
