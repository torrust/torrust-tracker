use std;
use std::io::{Write};
use std::net::{SocketAddr};
use byteorder::{NetworkEndian, WriteBytesExt};
use super::common::*;
use std::io;
use crate::TorrentPeer;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum UDPResponse {
    Connect(UDPConnectionResponse),
    Announce(UDPAnnounceResponse),
    Scrape(UDPScrapeResponse),
    Error(UDPErrorResponse),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UDPConnectionResponse {
    pub action: Actions,
    pub transaction_id: TransactionId,
    pub connection_id: ConnectionId,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UDPAnnounceResponse {
    pub action: Actions,
    pub transaction_id: TransactionId,
    pub interval: u32,
    pub leechers: u32,
    pub seeders: u32,
    pub peers: Vec<TorrentPeer>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UDPScrapeResponse {
    pub action: Actions,
    pub transaction_id: TransactionId,
    pub torrent_stats: Vec<UDPScrapeResponseEntry>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UDPScrapeResponseEntry {
    pub seeders: i32,
    pub completed: i32,
    pub leechers: i32,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UDPErrorResponse {
    pub action: Actions,
    pub transaction_id: TransactionId,
    pub message: String,
}

impl From<UDPConnectionResponse> for UDPResponse {
    fn from(r: UDPConnectionResponse) -> Self {
        Self::Connect(r)
    }
}

impl From<UDPAnnounceResponse> for UDPResponse {
    fn from(r: UDPAnnounceResponse) -> Self {
        Self::Announce(r)
    }
}

impl From<UDPScrapeResponse> for UDPResponse {
    fn from(r: UDPScrapeResponse) -> Self {
        Self::Scrape(r)
    }
}

impl From<UDPErrorResponse> for UDPResponse {
    fn from(r: UDPErrorResponse) -> Self {
        Self::Error(r)
    }
}

impl UDPResponse {
    pub fn write_to_bytes(self, bytes: &mut impl Write) -> Result<(), io::Error> {
        match self {
            UDPResponse::Connect(r) => {
                bytes.write_i32::<NetworkEndian>(0)?; // 0 = connect
                bytes.write_i32::<NetworkEndian>(r.transaction_id.0)?;
                bytes.write_i64::<NetworkEndian>(r.connection_id.0)?;
            },
            UDPResponse::Announce(r) => {
                bytes.write_i32::<NetworkEndian>(1)?; // 1 = announce
                bytes.write_i32::<NetworkEndian>(r.transaction_id.0)?;
                bytes.write_u32::<NetworkEndian>(r.interval)?;
                bytes.write_u32::<NetworkEndian>(r.leechers)?;
                bytes.write_u32::<NetworkEndian>(r.seeders)?;

                for peer in r.peers {
                    match peer.peer_addr {
                        SocketAddr::V4(socket_addr) => {
                            bytes.write_all(&socket_addr.ip().octets())?;
                            bytes.write_u16::<NetworkEndian>(socket_addr.port())?;
                        }
                        SocketAddr::V6(socket_addr) => {
                            bytes.write_all(&socket_addr.ip().octets())?;
                            bytes.write_u16::<NetworkEndian>(socket_addr.port())?;
                        }
                    }
                }
            },
            UDPResponse::Scrape(r) => {
                bytes.write_i32::<NetworkEndian>(2)?; // 2 = scrape
                bytes.write_i32::<NetworkEndian>(r.transaction_id.0)?;

                for torrent_stat in r.torrent_stats {
                    bytes.write_i32::<NetworkEndian>(torrent_stat.seeders)?;
                    bytes.write_i32::<NetworkEndian>(torrent_stat.completed)?;
                    bytes.write_i32::<NetworkEndian>(torrent_stat.leechers)?;
                }
            },
            UDPResponse::Error(r) => {
                bytes.write_i32::<NetworkEndian>(3)?;
                bytes.write_i32::<NetworkEndian>(r.transaction_id.0)?;
                bytes.write_all(r.message.as_bytes())?;
            },
        }

        Ok(())
    }
}
