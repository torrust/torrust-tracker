use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use std::net::IpAddr;
use serde::{Serialize};
use warp::http::Response;

#[derive(Serialize)]
pub struct Peer {
    pub peer_id: String,
    pub ip: IpAddr,
    pub port: u16,
}

#[derive(Serialize)]
pub struct AnnounceResponse {
    pub interval: u32,
    //pub tracker_id: String,
    pub complete: u32,
    pub incomplete: u32,
    pub peers: Vec<Peer>
}

impl AnnounceResponse {
    pub fn write(&self) -> String {
        serde_bencode::to_string(&self).unwrap()
    }

    pub fn write_compact(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut peers_v4: Vec<u8> = Vec::new();
        let mut peers_v6: Vec<u8> = Vec::new();

        for peer in &self.peers {
            match peer.ip {
                IpAddr::V4(ip) => {
                    peers_v4.write(&u32::from(ip).to_be_bytes())?;
                    peers_v4.write(&peer.port.to_be_bytes())?;
                }
                IpAddr::V6(ip) => {
                    peers_v6.write(&u128::from(ip).to_be_bytes())?;
                    peers_v6.write(&peer.port.to_be_bytes())?;
                }
            }
        }

        let mut bytes: Vec<u8> = Vec::new();
        bytes.write(b"d8:intervali")?;
        bytes.write(&self.interval.to_string().as_bytes())?;
        bytes.write(b"e8:completei")?;
        bytes.write(&self.complete.to_string().as_bytes())?;
        bytes.write(b"e10:incompletei")?;
        bytes.write(&self.incomplete.to_string().as_bytes())?;
        bytes.write(b"e5:peers")?;
        bytes.write(&peers_v4.len().to_string().as_bytes())?;
        bytes.write(b":")?;
        bytes.write(peers_v4.as_slice())?;
        bytes.write(b"e6:peers6")?;
        bytes.write(&peers_v6.len().to_string().as_bytes())?;
        bytes.write(b":")?;
        bytes.write(peers_v6.as_slice())?;
        bytes.write(b"e")?;

        Ok(bytes)
    }
}

#[derive(Serialize)]
pub struct ScrapeResponseEntry {
    pub complete: u32,
    pub downloaded: u32,
    pub incomplete: u32,
}

#[derive(Serialize)]
pub struct ScrapeResponse {
    pub files: HashMap<String, ScrapeResponseEntry>
}

impl ScrapeResponse {
    pub fn write(&self) -> String {
        serde_bencode::to_string(&self).unwrap()
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub failure_reason: String
}

impl warp::Reply for ErrorResponse {
    fn into_response(self) -> warp::reply::Response {
        Response::new(format!("{}", serde_bencode::to_string(&self).unwrap()).into())
    }
}
