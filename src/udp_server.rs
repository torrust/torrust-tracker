use log::{debug};
use std;
use std::net::{SocketAddr};
use std::sync::Arc;
use std::io::{Cursor};
use tokio::net::UdpSocket;

use crate::config::Configuration;
use super::common::*;
use crate::response::*;
use crate::request::{Request, ConnectRequest, AnnounceRequest, ScrapeRequest};
use crate::utils::get_connection_id;
use crate::tracker::TorrentTracker;
use crate::{TorrentPeer, TrackerMode};

pub struct UDPServer {
    socket: UdpSocket,
    tracker: Arc<TorrentTracker>,
    config: Arc<Configuration>,
}

impl UDPServer {
    pub async fn new(config: Arc<Configuration>, tracker: Arc<TorrentTracker>) -> Result<UDPServer, std::io::Error> {
        let cfg = config.clone();
        let srv = UdpSocket::bind(cfg.get_udp_config().get_address()).await?;

        Ok(UDPServer {
            socket: srv,
            tracker,
            config: cfg,
        })
    }

    async fn authenticate_announce_request(&self, announce_request: &AnnounceRequest) -> Result<(), ()> {


        match self.config.get_mode() {
            TrackerMode::PublicMode => Ok(()),
            TrackerMode::ListedMode => {
                if !self.tracker.is_info_hash_whitelisted(&announce_request.info_hash).await {
                    debug!("Info_hash not whitelisted.");
                    // todo: send error message to client
                    return Err(())
                }

                Ok(())
            }
            TrackerMode::PrivateMode => {
                match &announce_request.auth_key {
                    Some(auth_key) => {
                        if !self.tracker.key_manager.verify_auth_key(auth_key).await {
                            debug!("Key not valid.");
                            // todo: send error message to client
                            return Err(())
                        }

                        Ok(())
                    }
                    None => {
                        debug!("No key supplied.");
                        // todo: send error message to client
                        Err(())
                    }
                }
            }
            TrackerMode::PrivateListedMode => {
                match &announce_request.auth_key {
                    Some(auth_key) => {
                        if !self.tracker.key_manager.verify_auth_key(auth_key).await {
                            debug!("Key not valid.");
                            // todo: send error message to client
                            return Err(())
                        }

                        if !self.tracker.is_info_hash_whitelisted(&announce_request.info_hash).await {
                            debug!("Info_hash not whitelisted.");
                            // todo: send error message to client
                            return Err(())
                        }

                        Ok(())
                    }
                    None => {
                        debug!("No key supplied.");
                        // todo: send error message to client
                        Err(())
                    }
                }
            }
        }
    }

    pub async fn accept_packets(self) -> Result<(), std::io::Error> {
        let tracker = Arc::new(self);

        loop {
            let mut packet = vec![0u8; MAX_PACKET_SIZE];
            let (size, remote_address) = tracker.socket.recv_from(packet.as_mut_slice()).await?;

            let tracker = tracker.clone();
            tokio::spawn(async move {
                debug!("Received {} bytes from {}", size, remote_address);
                tracker.handle_packet(remote_address, &packet[..size]).await;
            });
        }
    }

    async fn handle_packet(&self, remote_addr: SocketAddr, payload: &[u8]) {
        let request = Request::from_bytes(&payload[..payload.len()]);

        match request {
            Ok(request) => {
                debug!("New request: {:?}", request);

                // todo: check for expired connection_id
                match request {
                    Request::Connect(r) => self.handle_connect(remote_addr, r).await,
                    Request::Announce(r) => {
                        match self.authenticate_announce_request(&r).await {
                            Ok(()) => self.handle_announce(remote_addr, r).await,
                            Err(()) => self.send_error(remote_addr, &r.transaction_id, "unauthenticated peer").await
                        }
                    },
                    Request::Scrape(r) => self.handle_scrape(remote_addr, r).await
                }
            }
            Err(err) => {
                debug!("request_from_bytes error: {:?}", err);
            }
        }
    }

    async fn handle_connect(&self, remote_addr: SocketAddr, request: ConnectRequest) {
        let connection_id = get_connection_id(&remote_addr);

        let response = UDPResponse::from(UDPConnectionResponse {
            action: Actions::Connect,
            transaction_id: request.transaction_id,
            connection_id,
        });

        let _ = self.send_response(remote_addr, response).await;
    }

    async fn handle_announce(&self, remote_addr: SocketAddr, request: AnnounceRequest) {
        let peer = TorrentPeer::from_announce_request(&request, remote_addr, self.config.get_ext_ip());

        match self.tracker.update_torrent_with_peer_and_get_stats(&request.info_hash, &peer).await {
            Ok(torrent_stats) => {
                // get all peers excluding the client_addr
                let peers = match self.tracker.get_torrent_peers(&request.info_hash, &peer.peer_addr).await {
                    Some(v) => v,
                    None => {
                        debug!("announce: No peers found.");
                        return;
                    }
                };

                let response = UDPResponse::from(UDPAnnounceResponse {
                    action: Actions::Announce,
                    transaction_id: request.transaction_id,
                    interval: self.config.get_udp_config().get_announce_interval(),
                    leechers: torrent_stats.leechers,
                    seeders: torrent_stats.seeders,
                    peers: ResponsePeerList(peers),
                });

                let _ = self.send_response(remote_addr, response).await;
            }
            Err(e) => {
                debug!("{:?}", e);
                self.send_error(remote_addr, &request.transaction_id, "error adding torrent").await;
            }
        }
    }

    async fn handle_scrape(&self, remote_addr: SocketAddr, request: ScrapeRequest) {
        let mut scrape_response = UDPScrapeResponse {
            action: Actions::Scrape,
            transaction_id: request.transaction_id,
            torrent_stats: Vec::new(),
        };

        let db = self.tracker.get_torrents().await;

        for info_hash in request.info_hashes.iter() {
            let scrape_entry = match db.get(&info_hash) {
                Some(torrent_info) => {
                    let (seeders, completed, leechers) = torrent_info.get_stats();

                    UDPScrapeResponseEntry {
                        seeders: seeders as i32,
                        completed: completed as i32,
                        leechers: leechers as i32,
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

            scrape_response.torrent_stats.push(scrape_entry);
        }

        let response = UDPResponse::from(scrape_response);

        let _ = self.send_response(remote_addr, response).await;
    }

    async fn send_response(&self, remote_addr: SocketAddr, response: UDPResponse) -> Result<usize, ()> {
        debug!("sending response to: {:?}", &remote_addr);

        let buffer = vec![0u8; MAX_PACKET_SIZE];
        let mut cursor = Cursor::new(buffer);

        // todo: add proper error logging
        match response.write_to_bytes(&mut cursor) {
            Ok(..) => {
                let position = cursor.position() as usize;
                let inner = cursor.get_ref();

                debug!("{:?}", &inner[..position]);
                match self.send_packet(&remote_addr, &inner[..position]).await {
                    Ok(byte_size) => Ok(byte_size),
                    Err(e) => {
                        debug!("{:?}", e);
                        Err(())
                    }
                }
            }
            Err(..) => {
                debug!("could not write response to bytes.");
                Err(())
            }
        }
    }

    async fn send_packet(&self, remote_addr: &SocketAddr, payload: &[u8]) -> Result<usize, std::io::Error> {
        match self.socket.send_to(payload, remote_addr).await {
            Err(err) => {
                debug!("failed to send a packet: {}", err);
                Err(err)
            },
            Ok(sz) => Ok(sz),
        }
    }

    async fn send_error(&self, remote_addr: SocketAddr, transaction_id: &TransactionId, error_msg: &str) {
        // let mut payload_buffer = vec![0u8; MAX_PACKET_SIZE];
        // let mut payload = StackVec::from(&mut payload_buffer);
        //
        // if let Ok(_) = write_to_bytes(&mut payload, &UDPResponseHeader {
        //     transaction_id: transaction_id.clone(),
        //     action: Actions::Error,
        // }) {
        //     let msg_bytes = Vec::from(error_msg.as_bytes());
        //     payload.extend(msg_bytes);
        //
        //     let _ = self.send_packet(remote_addr, payload.as_slice()).await;
        // }
    }
}
