use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Announce {
    pub complete: u32,
    pub incomplete: u32,
    pub interval: u32,
    #[serde(rename = "min interval")]
    pub min_interval: u32,
    pub peers: Vec<DictionaryPeer>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DictionaryPeer {
    pub ip: String,
    pub peer_id: String,
    pub port: u16,
}
