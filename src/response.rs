use std;
use std::io::{Write};
use std::net::{SocketAddr};
use byteorder::{NetworkEndian, WriteBytesExt};
use super::common::*;
use std::io;
use crate::TorrentPeer;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum UdpResponse {
    Connect(UdpConnectionResponse),
    Announce(UdpAnnounceResponse),
    Scrape(UdpScrapeResponse),
    Error(UdpErrorResponse),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UdpConnectionResponse {
    pub action: Actions,
    pub transaction_id: TransactionId,
    pub connection_id: ConnectionId,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UdpAnnounceResponse {
    pub action: Actions,
    pub transaction_id: TransactionId,
    pub interval: u32,
    pub leechers: u32,
    pub seeders: u32,
    pub peers: Vec<TorrentPeer>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UdpScrapeResponse {
    pub action: Actions,
    pub transaction_id: TransactionId,
    pub torrent_stats: Vec<UdpScrapeResponseEntry>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UdpScrapeResponseEntry {
    pub seeders: i32,
    pub completed: i32,
    pub leechers: i32,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UdpErrorResponse {
    pub action: Actions,
    pub transaction_id: TransactionId,
    pub message: String,
}

impl From<UdpConnectionResponse> for UdpResponse {
    fn from(r: UdpConnectionResponse) -> Self {
        Self::Connect(r)
    }
}

impl From<UdpAnnounceResponse> for UdpResponse {
    fn from(r: UdpAnnounceResponse) -> Self {
        Self::Announce(r)
    }
}

impl From<UdpScrapeResponse> for UdpResponse {
    fn from(r: UdpScrapeResponse) -> Self {
        Self::Scrape(r)
    }
}

impl From<UdpErrorResponse> for UdpResponse {
    fn from(r: UdpErrorResponse) -> Self {
        Self::Error(r)
    }
}

impl UdpResponse {
    pub fn write_to_bytes(self, bytes: &mut impl Write) -> Result<(), io::Error> {
        match self {
            UdpResponse::Connect(r) => {
                bytes.write_i32::<NetworkEndian>(0)?; // 0 = connect
                bytes.write_i32::<NetworkEndian>(r.transaction_id.0)?;
                bytes.write_i64::<NetworkEndian>(r.connection_id.0)?;
            },
            UdpResponse::Announce(r) => {
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
            UdpResponse::Scrape(r) => {
                bytes.write_i32::<NetworkEndian>(2)?; // 2 = scrape
                bytes.write_i32::<NetworkEndian>(r.transaction_id.0)?;

                for torrent_stat in r.torrent_stats {
                    bytes.write_i32::<NetworkEndian>(torrent_stat.seeders)?;
                    bytes.write_i32::<NetworkEndian>(torrent_stat.completed)?;
                    bytes.write_i32::<NetworkEndian>(torrent_stat.leechers)?;
                }
            },
            UdpResponse::Error(r) => {
                bytes.write_i32::<NetworkEndian>(3)?;
                bytes.write_i32::<NetworkEndian>(r.transaction_id.0)?;
                bytes.write_all(r.message.as_bytes())?;
            },
        }

        Ok(())
    }
}
