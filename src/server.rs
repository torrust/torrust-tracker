use log::{debug};
use std;
use std::net::{SocketAddr};
use std::sync::Arc;
use std::io::Error;
use std::future::Future;
use tokio::net::UdpSocket;

use crate::config::Configuration;
use crate::stackvec::StackVec;
use super::common::*;
use crate::response::*;
use crate::request::{Request, ConnectRequest, AnnounceRequest, ScrapeRequest};
use crate::utils::get_connection_id;
use crate::tracker::TorrentTracker;
use crate::tracker;

pub struct UDPTracker {
    socket: UdpSocket,
    tracker: Arc<TorrentTracker>,
    config: Arc<Configuration>,
}

impl UDPTracker {
    pub async fn new(config: Arc<Configuration>, tracker: Arc<TorrentTracker>) -> Result<UDPTracker, std::io::Error> {
        let cfg = config.clone();
        let srv = UdpSocket::bind(cfg.get_udp_config().get_address()).await?;

        Ok(UDPTracker {
            socket: srv,
            tracker,
            config: cfg,
        })
    }

    async fn handle_packet(&self, remote_address: SocketAddr, payload: &[u8]) {
        let request = Request::from_bytes(&payload[..payload.len()]);

        match request {
            Ok(request) => {
                debug!("New request: {:?}", request);
                match request {
                    Request::Connect(r) => self.handle_connect(remote_address, r).await,
                    Request::Announce(r) => self.handle_announce(remote_address, r).await,
                    Request::Scrape(r) => self.handle_scrape(remote_address, r).await
                }
            }
            Err(err) => {
                debug!("request_from_bytes error: {:?}", err);

                // if let Some(transaction_id) = err.transaction_id {
                //     let opt_message = if err.error.is_some() {
                //         Some("Parse error".to_string())
                //     } else if let Some(message) = err.message {
                //         Some(message)
                //     } else {
                //         None
                //     };
                //
                //     if let Some(message) = opt_message {
                //         let response = ErrorResponse {
                //             transaction_id,
                //             message,
                //         };
                //
                //         local_responses.push((response.into(), src));
                //     }
                // }
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
        // todo: I have no idea yet why this is here
        if request.connection_id != get_connection_id(&remote_addr) {
            debug!("announce: Unmatching connection_id.");
            return;
        }

        let client_addr = SocketAddr::new(remote_addr.ip(), request.port.0);

        match self
            .tracker
            .update_torrent_and_get_stats(
                &remote_addr,
                &request.info_hash,
                &request.peer_id,
                &request.bytes_uploaded,
                &request.bytes_downloaded,
                &request.bytes_left,
                &request.event,
            )
            .await
        {
            Ok(torrent_stats) => {
                // get all peers excluding the client_addr
                let peers = match self.tracker.get_torrent_peers(&request.info_hash, &client_addr).await {
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

                let _ = self.send_response(client_addr, response).await;
            }
            Err(e) => {
                match e {
                    tracker::TorrentError::TorrentFlagged => {
                        debug!("Torrent flagged.");
                        self.send_error(&client_addr, &request.transaction_id, "torrent flagged.").await;
                        return;
                    }
                    tracker::TorrentError::TorrentNotRegistered => {
                        debug!("Torrent not registered.");
                        self.send_error(&client_addr, &request.transaction_id, "torrent not registered.").await;
                        return;
                    }
                }
            }
        }
    }

    async fn handle_scrape(&self, remote_addr: SocketAddr, request: ScrapeRequest) {
        // if request.connection_id != self.get_connection_id(&remote_addr) {
        //     debug!("scrape: Unmatching connection_id.");
        //     return;
        // }
        //
        // let mut response_buffer = vec![0u8; MAX_PACKET_SIZE];
        // let mut response = StackVec::from(&mut response_buffer);
        //
        // if write_to_bytes(&mut response, &UDPResponseHeader {
        //     action: Actions::Scrape,
        //     transaction_id: request.transaction_id,
        // })
        // .is_err()
        // {
        //     // not much we can do...
        //     error!("failed to encode udp scrape response header.");
        //     return;
        // }
        //
        // // skip first 16 bytes for header...
        // let info_hash_array = &request.info_hashes;
        //
        // if info_hash_array.len() % 20 != 0 {
        //     trace!("received weird length for scrape info_hash array (!mod20).");
        // }
        //
        // {
        //     let db = self.tracker.get_database().await;
        //
        //     // for torrent_index in 0..MAX_SCRAPE {
        //     //     let info_hash_start = torrent_index * 20;
        //     //     let info_hash_end = (torrent_index + 1) * 20;
        //     //
        //     //     if info_hash_end > info_hash_array.len() {
        //     //         break;
        //     //     }
        //     //
        //     //     let info_hash = &info_hash_array[info_hash_start..info_hash_end];
        //     //     let ih = InfoHash::from(info_hash.0);
        //     //     let result = match db.get(&ih) {
        //     //         Some(torrent_info) => {
        //     //             let (seeders, completed, leechers) = torrent_info.get_stats();
        //     //
        //     //             UDPScrapeResponseEntry {
        //     //                 seeders,
        //     //                 completed,
        //     //                 leechers,
        //     //             }
        //     //         }
        //     //         None => {
        //     //             UDPScrapeResponseEntry {
        //     //                 seeders: 0,
        //     //                 completed: 0,
        //     //                 leechers: 0,
        //     //             }
        //     //         }
        //     //     };
        //     //
        //     //     if pack_into(&mut response, &result).is_err() {
        //     //         debug!("failed to encode scrape entry.");
        //     //         return;
        //     //     }
        //     // }
        // }
        //
        // // if sending fails, not much we can do...
        // let _ = self.send_packet(&remote_addr, &response.as_slice()).await;
    }

    async fn send_response(&self, remote_addr: SocketAddr, response: UDPResponse) -> Result<usize, ()> {
        println!("sending response to: {:?}", &remote_addr);

        let mut byte_buffer = vec![0u8; MAX_PACKET_SIZE];
        let mut bytes = StackVec::from(byte_buffer.as_mut_slice());

        // todo: add proper error logging
        match response.write_to_bytes(&mut bytes) {
            Ok(..) => {
                debug!("{:?}", &bytes.as_slice());
                match self.send_packet(&remote_addr, bytes.as_slice()).await {
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

    async fn send_error(&self, remote_addr: &SocketAddr, transaction_id: &TransactionId, error_msg: &str) {
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
}
