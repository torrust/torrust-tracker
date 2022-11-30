use std::net::IpAddr;

use serde::Deserialize;

use crate::http::Bytes;
use crate::protocol::info_hash::InfoHash;
use crate::tracker::peer;

#[derive(Deserialize)]
pub struct AnnounceQuery {
    pub downloaded: Option<Bytes>,
    pub uploaded: Option<Bytes>,
    pub key: Option<String>,
    pub port: u16,
    pub left: Option<Bytes>,
    pub event: Option<String>,
    pub compact: Option<u8>,
}

#[derive(Debug)]
pub struct Announce {
    pub info_hash: InfoHash,
    pub peer_addr: IpAddr,
    pub downloaded: Bytes,
    pub uploaded: Bytes,
    pub peer_id: peer::Id,
    pub port: u16,
    pub left: Bytes,
    pub event: Option<String>,
    pub compact: Option<u8>,
}

pub struct Scrape {
    pub info_hashes: Vec<InfoHash>,
    pub peer_addr: IpAddr,
}
