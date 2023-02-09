use std::collections::HashMap;
use std::io::Write;
use std::net::IpAddr;

use serde::{self, Deserialize, Serialize};

use crate::protocol::info_hash::InfoHash;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Peer {
    pub peer_id: String,
    pub ip: IpAddr,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Announce {
    pub interval: u32,
    #[serde(rename = "min interval")]
    pub interval_min: u32,
    //pub tracker_id: String,
    pub complete: u32,
    pub incomplete: u32,
    pub peers: Vec<Peer>,
}

impl Announce {
    /// # Panics
    ///
    /// It would panic if the `Announce` struct would contain an inappropriate type.
    #[must_use]
    pub fn write(&self) -> String {
        serde_bencode::to_string(&self).unwrap()
    }

    /// # Errors
    ///
    /// Will return `Err` if internally interrupted.
    pub fn write_compact(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut peers_v4: Vec<u8> = Vec::new();
        let mut peers_v6: Vec<u8> = Vec::new();

        for peer in &self.peers {
            match peer.ip {
                IpAddr::V4(ip) => {
                    peers_v4.write_all(&u32::from(ip).to_be_bytes())?;
                    peers_v4.write_all(&peer.port.to_be_bytes())?;
                }
                IpAddr::V6(ip) => {
                    peers_v6.write_all(&u128::from(ip).to_be_bytes())?;
                    peers_v6.write_all(&peer.port.to_be_bytes())?;
                }
            }
        }

        let mut bytes: Vec<u8> = Vec::new();
        bytes.write_all(b"d8:intervali")?;
        bytes.write_all(self.interval.to_string().as_bytes())?;
        bytes.write_all(b"e12:min intervali")?;
        bytes.write_all(self.interval_min.to_string().as_bytes())?;
        bytes.write_all(b"e8:completei")?;
        bytes.write_all(self.complete.to_string().as_bytes())?;
        bytes.write_all(b"e10:incompletei")?;
        bytes.write_all(self.incomplete.to_string().as_bytes())?;
        bytes.write_all(b"e5:peers")?;
        bytes.write_all(peers_v4.len().to_string().as_bytes())?;
        bytes.write_all(b":")?;
        bytes.write_all(peers_v4.as_slice())?;
        bytes.write_all(b"e6:peers6")?;
        bytes.write_all(peers_v6.len().to_string().as_bytes())?;
        bytes.write_all(b":")?;
        bytes.write_all(peers_v6.as_slice())?;
        bytes.write_all(b"e")?;

        Ok(bytes)
    }
}

#[derive(Serialize)]
pub struct ScrapeEntry {
    pub complete: u32,
    pub downloaded: u32,
    pub incomplete: u32,
}

#[derive(Serialize)]
pub struct Scrape {
    pub files: HashMap<InfoHash, ScrapeEntry>,
}

impl Scrape {
    /// # Errors
    ///
    /// Will return `Err` if internally interrupted.
    pub fn write(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.write_all(b"d5:filesd")?;

        for (info_hash, scrape_response_entry) in &self.files {
            bytes.write_all(b"20:")?;
            bytes.write_all(&info_hash.0)?;
            bytes.write_all(b"d8:completei")?;
            bytes.write_all(scrape_response_entry.complete.to_string().as_bytes())?;
            bytes.write_all(b"e10:downloadedi")?;
            bytes.write_all(scrape_response_entry.downloaded.to_string().as_bytes())?;
            bytes.write_all(b"e10:incompletei")?;
            bytes.write_all(scrape_response_entry.incomplete.to_string().as_bytes())?;
            bytes.write_all(b"ee")?;
        }

        bytes.write_all(b"ee")?;

        Ok(bytes)
    }
}

#[derive(Serialize)]
pub struct Error {
    #[serde(rename = "failure reason")]
    pub failure_reason: String,
}

impl Error {
    /// # Panics
    ///
    /// It would panic if the `Error` struct would contain an inappropriate type.
    #[must_use]
    pub fn write(&self) -> String {
        serde_bencode::to_string(&self).unwrap()
    }
}
