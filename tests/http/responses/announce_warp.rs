/// todo: this mod should be removed when we remove the Warp implementation for the HTTP tracker.
use serde::{self, Deserialize, Serialize};
use torrust_tracker::tracker::peer::Peer;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct WarpAnnounce {
    pub complete: u32,
    pub incomplete: u32,
    pub interval: u32,
    #[serde(rename = "min interval")]
    pub min_interval: u32,
    pub peers: Vec<WarpDictionaryPeer>, // Peers using IPV4
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct WarpDictionaryPeer {
    pub ip: String,
    pub peer_id: String,
    pub port: u16,
}

impl From<Peer> for WarpDictionaryPeer {
    fn from(peer: Peer) -> Self {
        Self {
            peer_id: peer.peer_id.to_string(),
            ip: peer.peer_addr.ip().to_string(),
            port: peer.peer_addr.port(),
        }
    }
}
