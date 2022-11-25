use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct TorrentResource {
    pub info_hash: String,
    pub completed: u32,
    pub leechers: u32,
    pub peers: Vec<TorrentPeerResource>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct TorrentPeerResource {
    pub peer_id: PeerIdResource,
    pub peer_addr: String,
    pub updated: i64,
    pub uploaded: i64,
    pub downloaded: i64,
    pub left: i64,
    pub event: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct PeerIdResource {
    pub id: String,
    pub client: String,
}
