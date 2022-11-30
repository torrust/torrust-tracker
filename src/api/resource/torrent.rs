use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Torrent {
    pub info_hash: String,
    pub seeders: u32,
    pub completed: u32,
    pub leechers: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peers: Option<Vec<super::peer::Peer>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ListItem {
    pub info_hash: String,
    pub seeders: u32,
    pub completed: u32,
    pub leechers: u32,
    // todo: this is always None. Remove field from endpoint?
    pub peers: Option<Vec<super::peer::Peer>>,
}
