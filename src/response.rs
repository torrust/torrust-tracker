use std;
use std::io::{Write};
use std::net::{SocketAddr, Ipv4Addr};
use byteorder::{NetworkEndian, WriteBytesExt};
use super::common::*;
use std::io;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum UDPResponse {
    Connect(UDPConnectionResponse),
    Announce(UDPAnnounceResponse),
    Scrape(UDPScrapeResponseEntry),
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
    pub peers: ResponsePeerList,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UDPScrapeResponseEntry {
    pub seeders: u32,
    pub completed: u32,
    pub leechers: u32,
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

impl From<UDPScrapeResponseEntry> for UDPResponse {
    fn from(r: UDPScrapeResponseEntry) -> Self {
        Self::Scrape(r)
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

                // Silently ignore peers with wrong IP version
                for peer in r.peers.0 {
                    match peer {
                        SocketAddr::V4(socket_addr) => {
                            if socket_addr.ip() == &Ipv4Addr::new(127, 0, 0, 1) {
                                bytes.write_all(&Ipv4Addr::new(192, 168, 2, 2).octets())?;
                            } else {
                                bytes.write_all(&socket_addr.ip().octets())?;
                            }
                            bytes.write_u16::<NetworkEndian>(peer.port())?;
                        }
                        SocketAddr::V6(socket_addr) => {
                            bytes.write_all(&socket_addr.ip().octets())?;
                            bytes.write_u16::<NetworkEndian>(peer.port())?;
                        }
                    }
                }
            },

            // todo: fix scrape response
            // UDPResponse::Scrape(r) => {
            //     bytes.write_i32::<NetworkEndian>(2)?;
            //     bytes.write_i32::<NetworkEndian>(r.transaction_id.0)?;
            //
            //     for torrent_stat in r.torrent_stats {
            //         bytes.write_i32::<NetworkEndian>(torrent_stat.seeders.0)?;
            //         bytes.write_i32::<NetworkEndian>(torrent_stat.completed.0)?;
            //         bytes.write_i32::<NetworkEndian>(torrent_stat.leechers.0)?;
            //     }
            // },
            // UDPResponse::Error(r) => {
            //     bytes.write_i32::<NetworkEndian>(3)?;
            //     bytes.write_i32::<NetworkEndian>(r.transaction_id.0)?;
            //
            //     bytes.write_all(r.message.as_bytes())?;
            // },
            _ => {}
        }

        Ok(())
    }
}
