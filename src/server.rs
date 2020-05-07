use log::{debug, error, trace};
use std;
use std::io::Write;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;

use bincode;
use serde::{Deserialize, Serialize};

use crate::config::Configuration;
use crate::stackvec::StackVec;
use crate::tracker;

// maximum MTU is usually 1500, but our stack allows us to allocate the maximum - so why not?
const MAX_PACKET_SIZE: usize = 0xffff;

// protocol contants
const PROTOCOL_ID: u64 = 0x0000041727101980;

#[repr(u32)]
#[derive(Serialize, Deserialize)]
enum Actions {
    Connect = 0,
    Announce = 1,
    Scrape = 2,
    Error = 3,
}

#[repr(u32)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Events {
    None = 0,
    Complete = 1,
    Started = 2,
    Stopped = 3,
}

fn pack_into<T: Serialize, W: std::io::Write>(w: &mut W, data: &T) -> Result<(), ()> {
    let mut config = bincode::config();
    config.big_endian();

    match config.serialize_into(w, data) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

fn unpack<'a, T: Deserialize<'a>>(data: &'a [u8]) -> Option<T> {
    let mut bo = bincode::config();
    bo.big_endian();

    match bo.deserialize(data) {
        Ok(obj) => Some(obj),
        Err(_) => None,
    }
}

#[derive(Serialize, Deserialize)]
struct UDPRequestHeader {
    connection_id: u64,
    action: Actions,
    transaction_id: u32,
}

#[derive(Serialize, Deserialize)]
struct UDPResponseHeader {
    action: Actions,
    transaction_id: u32,
}

#[derive(Serialize, Deserialize)]
struct UDPConnectionResponse {
    header: UDPResponseHeader,
    connection_id: u64,
}

#[derive(Serialize, Deserialize)]
struct UDPAnnounceRequest {
    header: UDPRequestHeader,

    info_hash: [u8; 20],
    peer_id: [u8; 20],
    downloaded: u64,
    left: u64,
    uploaded: u64,
    event: Events,
    ip_address: u32,
    key: u32,
    num_want: i32,
    port: u16,
}

#[derive(Serialize, Deserialize)]
struct UDPAnnounceResponse {
    header: UDPResponseHeader,

    interval: u32,
    leechers: u32,
    seeders: u32,
}

#[derive(Serialize)]
struct UDPScrapeResponseEntry {
    seeders: u32,
    completed: u32,
    leechers: u32,
}

pub struct UDPTracker {
    server: UdpSocket,
    tracker: std::sync::Arc<tracker::TorrentTracker>,
    config: Arc<Configuration>,
}

impl UDPTracker {
    pub async fn new(
        config: Arc<Configuration>, tracker: std::sync::Arc<tracker::TorrentTracker>,
    ) -> Result<UDPTracker, std::io::Error> {
        let cfg = config.clone();

        let server = UdpSocket::bind(cfg.get_udp_config().get_address()).await?;

        Ok(UDPTracker {
            server,
            tracker,
            config: cfg,
        })
    }

    // TODO: remove `mut` once https://github.com/tokio-rs/tokio/issues/1624 is resolved
    async fn handle_packet(&mut self, remote_address: &SocketAddr, payload: &[u8]) {
        let header: UDPRequestHeader = match unpack(payload) {
            Some(val) => val,
            None => {
                trace!("failed to parse packet from {}", remote_address);
                return;
            }
        };

        match header.action {
            Actions::Connect => self.handle_connect(remote_address, &header, payload).await,
            Actions::Announce => self.handle_announce(remote_address, &header, payload).await,
            Actions::Scrape => self.handle_scrape(remote_address, &header, payload).await,
            _ => {
                trace!("invalid action from {}", remote_address);
                // someone is playing around... ignore request.
                return;
            }
        }
    }

    // TODO: remove `mut` once https://github.com/tokio-rs/tokio/issues/1624 is resolved
    async fn handle_connect(&mut self, remote_addr: &SocketAddr, header: &UDPRequestHeader, _payload: &[u8]) {
        if header.connection_id != PROTOCOL_ID {
            trace!("Bad protocol magic from {}", remote_addr);
            return;
        }

        // send response...
        let conn_id = self.get_connection_id(remote_addr);

        let response = UDPConnectionResponse {
            header: UDPResponseHeader {
                transaction_id: header.transaction_id,
                action: Actions::Connect,
            },
            connection_id: conn_id,
        };

        let mut payload_buffer = vec![0u8; MAX_PACKET_SIZE];
        let mut payload = StackVec::from(payload_buffer.as_mut_slice());

        if let Ok(_) = pack_into(&mut payload, &response) {
            let _ = self.send_packet(remote_addr, payload.as_slice()).await;
        }
    }

    // TODO: remove `mut` once https://github.com/tokio-rs/tokio/issues/1624 is resolved
    async fn handle_announce(&mut self, remote_addr: &SocketAddr, header: &UDPRequestHeader, payload: &[u8]) {
        if header.connection_id != self.get_connection_id(remote_addr) {
            return;
        }

        let packet: UDPAnnounceRequest = match unpack(payload) {
            Some(v) => v,
            None => {
                trace!("failed to unpack announce request from {}", remote_addr);
                return;
            }
        };

        if let Ok(_plen) = bincode::serialized_size(&packet) {
            let plen = _plen as usize;
            if payload.len() > plen {
                let bep41_payload = &payload[plen..];

                // TODO: process BEP0041 payload.
                trace!("BEP0041 payload of {} bytes from {}", bep41_payload.len(), remote_addr);
            }
        }

        if packet.ip_address != 0 {
            // TODO: allow configurability of ip address
            // for now, ignore request.
            trace!("announce request for other IP ignored. (from {})", remote_addr);
            return;
        }

        let client_addr = SocketAddr::new(remote_addr.ip(), packet.port);
        let info_hash = packet.info_hash.into();

        let peer_id: &tracker::PeerId = tracker::PeerId::from_array(&packet.peer_id);

        match self
            .tracker
            .update_torrent_and_get_stats(
                &info_hash,
                peer_id,
                &client_addr,
                packet.uploaded,
                packet.downloaded,
                packet.left,
                packet.event,
            )
            .await
        {
            tracker::TorrentStats::Stats {
                leechers,
                complete: _,
                seeders,
            } => {
                let peers = match self.tracker.get_torrent_peers(&info_hash, &client_addr).await {
                    Some(v) => v,
                    None => {
                        return;
                    }
                };

                let mut payload_buffer = vec![0u8; MAX_PACKET_SIZE];
                let mut payload = StackVec::from(&mut payload_buffer);

                match pack_into(&mut payload, &UDPAnnounceResponse {
                    header: UDPResponseHeader {
                        action: Actions::Announce,
                        transaction_id: packet.header.transaction_id,
                    },
                    seeders,
                    interval: self.config.get_udp_config().get_announce_interval(),
                    leechers,
                }) {
                    Ok(_) => {}
                    Err(_) => {
                        return;
                    }
                };

                for peer in peers {
                    match peer {
                        SocketAddr::V4(ipv4) => {
                            let _ = payload.write(&ipv4.ip().octets());
                        }
                        SocketAddr::V6(ipv6) => {
                            let _ = payload.write(&ipv6.ip().octets());
                        }
                    };

                    let port_hton = client_addr.port().to_be();
                    let _ = payload.write(&[(port_hton & 0xff) as u8, ((port_hton >> 8) & 0xff) as u8]);
                }

                let _ = self.send_packet(&client_addr, payload.as_slice()).await;
            }
            tracker::TorrentStats::TorrentFlagged => {
                self.send_error(&client_addr, &packet.header, "torrent flagged.").await;
                return;
            }
            tracker::TorrentStats::TorrentNotRegistered => {
                self.send_error(&client_addr, &packet.header, "torrent not registered.").await;
                return;
            }
        }
    }

    // TODO: remove `mut` once https://github.com/tokio-rs/tokio/issues/1624 is resolved
    async fn handle_scrape(&mut self, remote_addr: &SocketAddr, header: &UDPRequestHeader, payload: &[u8]) {
        if header.connection_id != self.get_connection_id(remote_addr) {
            return;
        }

        const MAX_SCRAPE: usize = 74;

        let mut response_buffer = [0u8; 8 + MAX_SCRAPE * 12];
        let mut response = StackVec::from(&mut response_buffer);

        if pack_into(&mut response, &UDPResponseHeader {
            action: Actions::Scrape,
            transaction_id: header.transaction_id,
        })
        .is_err()
        {
            // not much we can do...
            error!("failed to encode udp scrape response header.");
            return;
        }

        // skip first 16 bytes for header...
        let info_hash_array = &payload[16..];

        if info_hash_array.len() % 20 != 0 {
            trace!("received weird length for scrape info_hash array (!mod20).");
        }

        {
            let db = self.tracker.get_database().await;

            for torrent_index in 0..MAX_SCRAPE {
                let info_hash_start = torrent_index * 20;
                let info_hash_end = (torrent_index + 1) * 20;

                if info_hash_end > info_hash_array.len() {
                    break;
                }

                let info_hash = &info_hash_array[info_hash_start..info_hash_end];
                let ih = tracker::InfoHash::from(info_hash);
                let result = match db.get(&ih) {
                    Some(torrent_info) => {
                        let (seeders, completed, leechers) = torrent_info.get_stats();

                        UDPScrapeResponseEntry {
                            seeders,
                            completed,
                            leechers,
                        }
                    }
                    None => {
                        UDPScrapeResponseEntry {
                            seeders: 0,
                            completed: 0,
                            leechers: 0,
                        }
                    }
                };

                if pack_into(&mut response, &result).is_err() {
                    debug!("failed to encode scrape entry.");
                    return;
                }
            }
        }

        // if sending fails, not much we can do...
        let _ = self.send_packet(&remote_addr, &response.as_slice()).await;
    }

    fn get_connection_id(&self, remote_address: &SocketAddr) -> u64 {
        match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(duration) => (duration.as_secs() / 3600) | ((remote_address.port() as u64) << 36),
            Err(_) => 0x8000000000000000,
        }
    }

    // TODO: remove `mut` once https://github.com/tokio-rs/tokio/issues/1624 is resolved
    async fn send_packet(&mut self, remote_addr: &SocketAddr, payload: &[u8]) -> Result<usize, std::io::Error> {
        self.server.send_to(payload, remote_addr).await
    }

    // TODO: remove `mut` once https://github.com/tokio-rs/tokio/issues/1624 is resolved
    async fn send_error(&mut self, remote_addr: &SocketAddr, header: &UDPRequestHeader, error_msg: &str) {
        let mut payload_buffer = vec![0u8; MAX_PACKET_SIZE];
        let mut payload = StackVec::from(&mut payload_buffer);

        if let Ok(_) = pack_into(&mut payload, &UDPResponseHeader {
            transaction_id: header.transaction_id,
            action: Actions::Error,
        }) {
            let msg_bytes = Vec::from(error_msg.as_bytes());
            payload.extend(msg_bytes);

            let _ = self.send_packet(remote_addr, payload.as_slice()).await;
        }
    }

    // TODO: remove `mut` for `accept_packet`, and spawn once https://github.com/tokio-rs/tokio/issues/1624 is resolved
    pub async fn accept_packet(&mut self) -> Result<(), std::io::Error> {
        let mut packet = vec![0u8; MAX_PACKET_SIZE];
        let (size, remote_address) = self.server.recv_from(packet.as_mut_slice()).await?;

        // tokio::spawn(async {
        debug!("Received {} bytes from {}", size, remote_address);
        self.handle_packet(&remote_address, &packet[..size]).await;
        // });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pack() {
        let mystruct = super::UDPRequestHeader {
            connection_id: 200,
            action: super::Actions::Connect,
            transaction_id: 77771,
        };
        let mut buffer = [0u8; MAX_PACKET_SIZE];
        let mut payload = StackVec::from(&mut buffer);

        assert!(pack_into(&mut payload, &mystruct).is_ok());
        assert_eq!(payload.as_slice().len(), 16);
        assert_eq!(payload.as_slice(), &[0, 0, 0, 0, 0, 0, 0, 200u8, 0, 0, 0, 0, 0, 1, 47, 203]);
    }

    #[test]
    fn unpack() {
        let buf = [0u8, 0, 0, 0, 0, 0, 0, 200, 0, 0, 0, 1, 0, 1, 47, 203];
        match super::unpack(&buf) {
            Some(obj) => {
                let x: super::UDPResponseHeader = obj;
                println!("conn_id={}", x.action as u32);
            }
            None => {
                assert!(false);
            }
        }
    }
}
