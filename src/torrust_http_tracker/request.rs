use std::net::SocketAddr;
use serde::{Deserialize};
use crate::InfoHash;

#[derive(Deserialize)]
pub struct AnnounceRequestQuery {
    pub downloaded: u32,
    pub uploaded: u32,
    pub key: String,
    pub peer_id: String,
    pub port: u16,
    pub left: u32,
    pub event: Option<String>,
    pub compact: Option<u8>,
}

pub struct AnnounceRequest {
    pub info_hash: InfoHash,
    pub peer_addr: SocketAddr,
    pub downloaded: u32,
    pub uploaded: u32,
    pub peer_id: String,
    pub port: u16,
    pub left: u32,
    pub event: Option<String>,
    pub compact: Option<u8>,
}

pub struct ScrapeRequest {
    pub info_hashes: Vec<InfoHash>,
}
