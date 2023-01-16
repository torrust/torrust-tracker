use serde::{Deserialize, Serialize};

use crate::tracker;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Peer {
    pub peer_id: Id,
    pub peer_addr: String,
    #[deprecated(since = "2.0.0", note = "please use `updated_milliseconds_ago` instead")]
    pub updated: u128,
    pub updated_milliseconds_ago: u128,
    pub uploaded: i64,
    pub downloaded: i64,
    pub left: i64,
    pub event: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Id {
    pub id: Option<String>,
    pub client: Option<String>,
}

impl From<tracker::peer::Id> for Id {
    fn from(peer_id: tracker::peer::Id) -> Self {
        Id {
            id: peer_id.get_id(),
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
