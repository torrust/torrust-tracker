use std::net::{IpAddr, SocketAddr};
use serde::{Deserialize};
use crate::InfoHash;
use crate::torrust_http_tracker::Bytes;

#[derive(Deserialize)]
pub struct AnnounceRequestQuery {
    pub downloaded: Bytes,
    pub uploaded: Bytes,
    pub key: String,
    pub peer_id: String,
    pub port: u16,
    pub left: Bytes,
    pub event: Option<String>,
    pub compact: Option<u8>,
}

pub struct AnnounceRequest {
    pub info_hash: InfoHash,
    pub peer_addr: SocketAddr,
    pub forwarded_ip: Option<IpAddr>,
    pub downloaded: Bytes,
    pub uploaded: Bytes,
    pub peer_id: String,
    pub port: u16,
    pub left: Bytes,
    pub event: Option<String>,
    pub compact: Option<u8>,
}

pub struct ScrapeRequest {
    pub info_hashes: Vec<InfoHash>,
}
