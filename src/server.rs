use std;
use std::net::{SocketAddr, UdpSocket};
use std::io::Write;

use bincode;
use serde::{Serialize, Deserialize};

use tracker;
use stackvec::StackVec;

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

fn pack<T: Serialize>(data: &T) -> Option<Vec<u8>> {
    let mut bo = bincode::config();
    bo.big_endian();

    match bo.serialize(data) {
        Ok(v) => Some(v),
        Err(_) => None,
    }
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

pub struct UDPTracker {
    server: std::net::UdpSocket,
    tracker: std::sync::Arc<tracker::TorrentTracker>,
}

impl UDPTracker {
    pub fn new<T: std::net::ToSocketAddrs>(bind_address: T, tracker: std::sync::Arc<tracker::TorrentTracker>) -> Result<UDPTracker, std::io::Error> {
        let server = match UdpSocket::bind(bind_address) {
            Ok(s) => s,
            Err(e) => {
                return Err(e);
            }
        };

        Ok(UDPTracker{
            server,
            tracker,
        })
    }

    fn handle_packet(&self, remote_address: &SocketAddr, payload: &[u8]) {
        let header : UDPRequestHeader = match unpack(payload) {
            Some(val) => val,
            None => {
                return;
            }
        };

        match header.action {
            Actions::Connect => self.handle_connect(remote_address, &header, payload),
            Actions::Announce => self.handle_announce(remote_address, &header, payload),
            Actions::Scrape => self.handle_scrape(remote_address, &header, payload),
            _ => {
                // someone is playing around... ignore request.
                return;
            }
        }
    }

    fn handle_connect(&self, remote_addr: &SocketAddr, header: &UDPRequestHeader, _payload: &[u8]) {
        if header.connection_id != PROTOCOL_ID {
            return;
        }

        // send response...
        let conn_id = self.get_connection_id(remote_addr);

        let response = UDPConnectionResponse{
            header: UDPResponseHeader{
                transaction_id: header.transaction_id,
                action: Actions::Connect,
            },
            connection_id: conn_id,
        };

        let mut payload_buffer = [0u8; MAX_PACKET_SIZE];
        let mut payload = StackVec::from(&mut payload_buffer);

        if let Ok(_) = pack_into(&mut payload, &response) {
            let _ = self.send_packet(remote_addr, payload.as_slice());
        }
    }

    fn handle_announce(&self, remote_addr: &SocketAddr, header: &UDPRequestHeader, payload: &[u8]) {
        if header.connection_id != self.get_connection_id(remote_addr) {
            return;
        }

        let packet: UDPAnnounceRequest = match unpack(payload) {
            Some(v) => v,
            None => {
                return;
            }
        };

        let plen = bincode::serialized_size(&packet).unwrap() as usize;

        println!("payload len={}, announce len={}", payload.len(), plen);

        if payload.len() > plen {
            let bep41_payload = &payload[std::mem::size_of::<UDPAnnounceRequest>()..];
            println!("bep41: {:?}", bep41_payload);
        }

        if packet.ip_address != 0 {
            // TODO: allow configurability of ip address
            // for now, ignore request.
            return;
        }

        let client_addr = SocketAddr::new(remote_addr.ip(), packet.port);

        match self.tracker.update_torrent_and_get_stats(&packet.info_hash, &packet.peer_id, &client_addr, packet.uploaded, packet.downloaded, packet.left, packet.event) {
            tracker::TorrentStats::Stats {leechers, complete, seeders} => {
                let peers = match self.tracker.get_torrent_peers(&packet.info_hash, &client_addr) {
                    Some(v) => v,
                    None => {
                        return;
                    }
                };

                let mut payload_buffer = [0u8; MAX_PACKET_SIZE];
                let mut payload = StackVec::from(&mut payload_buffer);

                match pack_into(&mut payload,&UDPAnnounceResponse {
                    header: UDPResponseHeader {
                        action: Actions::Announce,
                        transaction_id: packet.header.transaction_id,
                    },
                    seeders,
                    interval: 20,
                    leechers,
                }) {
                    Ok(_) => {},
                    Err(_) => {
                        return;
                    }
                };

                for peer in peers {
                    match peer {
                        SocketAddr::V4(ipv4) => {
                            let _ = payload.write(&ipv4.ip().octets());
                        },
                        SocketAddr::V6(ipv6) => {
                            let _ = payload.write(&ipv6.ip().octets());
                        }
                    };

                    let port_hton = client_addr.port().to_be();
                    let _ = payload.write(&[(port_hton & 0xff) as u8, ((port_hton >> 8) & 0xff) as u8]);
                }

                let _ = self.send_packet(&client_addr, payload.as_slice());
            },
            tracker::TorrentStats::TorrentFlagged => {
                self.send_error(&client_addr, &packet.header, "torrent flagged.");
                return;
            },
            tracker::TorrentStats::TorrentNotRegistered => {
                self.send_error(&client_addr, &packet.header, "torrent not registered.");
                return;
            }
        }
    }

    fn handle_scrape(&self, remote_addr: &SocketAddr, header: &UDPRequestHeader, _payload: &[u8]) {
        if header.connection_id != self.get_connection_id(remote_addr) {
            return;
        }

        self.send_error(remote_addr, header, "scrape not yet implemented");
    }

    fn get_connection_id(&self, remote_address: &SocketAddr) -> u64 {
        match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(duration) => {
                (duration.as_secs() / 3600) | ((remote_address.port() as u64) << 36)
            },
            Err(_) => {
                0x8000000000000000
            }
        }
    }

    fn send_packet(&self, remote_addr: &SocketAddr, payload: &[u8]) -> Result<usize, std::io::Error> {
        self.server.send_to(payload, remote_addr)
    }

    fn send_error(&self, remote_addr: &SocketAddr, header: &UDPRequestHeader, error_msg: &str) {
        let mut payload_buffer = [0u8; MAX_PACKET_SIZE];
        let mut payload = StackVec::from(&mut payload_buffer);

        if let Ok(_) = pack_into(&mut payload, &UDPResponseHeader{
            transaction_id: header.transaction_id,
            action: Actions::Error,
        }) {
            let msg_bytes = Vec::from(error_msg.as_bytes());
            payload.extend(msg_bytes);

            let _ = self.send_packet(remote_addr, payload.as_slice());
        }
    }

    pub fn accept_packet(&self) -> Result<(), std::io::Error> {
        let mut packet = [0u8; MAX_PACKET_SIZE];
        match self.server.recv_from(&mut packet) {
            Ok((size, remote_address)) => {
                self.handle_packet(&remote_address, &packet[..size]);

                Ok(())
            },
            Err(e) => Err(e),
        }
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
        assert_eq!(payload.len(), 16);
        assert_eq!(payload.as_slice(), &[0, 0, 0, 0, 0, 0, 0, 200u8, 0, 0, 0, 0, 0, 1, 47, 203]);
    }

    #[test]
    fn unpack() {
        let buf = [0u8, 0, 0, 0, 0, 0, 0, 200, 0, 0, 0, 1, 0, 1, 47, 203];
        match super::unpack(&buf) {
            Some(obj) => {
                let x : super::UDPResponseHeader = obj;
                println!("conn_id={}", x.action as u32);
            },
            None => {
                assert!(false);
            }
        }
    }
}
