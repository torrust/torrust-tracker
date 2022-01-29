use serde::{Deserialize};

#[derive(Deserialize)]
pub struct AnnounceRequest {
    pub downloaded: u32,
    pub uploaded: u32,
    pub key: String,
    pub peer_id: String,
    pub port: u16,
    pub info_hash: String,
    pub left: u32,
    pub event: Option<String>,
    pub compact: Option<u8>,
}
