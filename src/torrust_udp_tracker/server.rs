use log::debug;
use std;
use std::net::SocketAddr;
use std::sync::Arc;
use std::io::Cursor;
use aquatic_udp_protocol::{AnnounceInterval, AnnounceRequest, AnnounceResponse, ConnectRequest, ConnectResponse, ErrorResponse, IpVersion, NumberOfDownloads, NumberOfPeers, Port, Request, Response, ResponsePeer, ScrapeRequest, ScrapeResponse, TorrentScrapeStatistics, TransactionId};
use tokio::net::UdpSocket;

use crate::common::*;
use crate::utils::get_connection_id;
use crate::tracker::TorrentTracker;
use crate::{InfoHash, TorrentError, TorrentPeer};

struct RequestError {
    error: TorrentError,
    transaction_id: TransactionId
}

struct AnnounceRequestWrapper {
    announce_request: AnnounceRequest,
    info_hash: InfoHash,
}

impl AnnounceRequestWrapper {
    pub fn new(announce_request: AnnounceRequest) -> Self {
        AnnounceRequestWrapper {
            announce_request: announce_request.clone(),
            info_hash: InfoHash(announce_request.info_hash.0)
        }
    }
}

pub struct UdpServer {
    socket: UdpSocket,
    tracker: Arc<TorrentTracker>,
}

impl UdpServer {
    pub async fn new(tracker: Arc<TorrentTracker>) -> Result<UdpServer, std::io::Error> {
        let srv = UdpSocket::bind(&tracker.config.udp_tracker.bind_address).await?;

        Ok(UdpServer {
            socket: srv,
            tracker,
        })
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
        let request = Request::from_bytes(&payload[..payload.len()], MAX_SCRAPE_TORRENTS);

        match request {
            Ok(request) => {
                debug!("New request: {:?}", request);
                self.handle_request(request, remote_addr).await;
            }
            Err(err) => {
                debug!("request_from_bytes error: {:?}", err);
            }
        }
    }

    async fn handle_request(&self, request: Request, remote_addr: SocketAddr) {
        // todo: check for expired connection_id
        let request_result = match request {
            Request::Connect(connect_request) => {
                self.handle_connect(remote_addr, &connect_request).await
                    .map_err(|error| RequestError { error, transaction_id: connect_request.transaction_id })
            }
            Request::Announce(announce_request) => {
                self.handle_announce(remote_addr, &announce_request).await
                    .map_err(|error| RequestError { error, transaction_id: announce_request.transaction_id })
            }
            Request::Scrape(scrape_request) => {
                self.handle_scrape(&scrape_request).await
                    .map_err(|error| RequestError { error, transaction_id: scrape_request.transaction_id })
            }
        };

        match request_result {
            Ok(response) => {
                let _ = self.send_response(remote_addr, response).await;
            }
            Err(request_error) => {
                let _ = self.handle_error(request_error.error, remote_addr, request_error.transaction_id).await;
            }
        }
    }

    async fn handle_connect(&self, remote_addr: SocketAddr, request: &ConnectRequest) -> Result<Response, TorrentError> {
        let connection_id = get_connection_id(&remote_addr);

        let response = Response::from(ConnectResponse {
            transaction_id: request.transaction_id,
            connection_id,
        });

        Ok(response)
    }

    async fn handle_announce(&self, remote_addr: SocketAddr, announce_request: &AnnounceRequest) -> Result<Response, TorrentError> {
        let wrapped_announce_request = AnnounceRequestWrapper::new(announce_request.clone());
        self.tracker.authenticate_request(&wrapped_announce_request.info_hash, &None).await?;

        let peer = TorrentPeer::from_udp_announce_request(&wrapped_announce_request.announce_request, remote_addr, self.tracker.config.get_ext_ip());

        return match self.tracker.update_torrent_with_peer_and_get_stats(&wrapped_announce_request.info_hash, &peer).await {
            Ok(torrent_stats) => {
                // get all peers excluding the client_addr
                let peers = match self.tracker.get_torrent_peers(&wrapped_announce_request.info_hash, &peer.peer_addr).await {
                    Some(v) => v,
                    None => {
                        return Err(TorrentError::NoPeersFound);
                    }
                };

                let response = Response::from(AnnounceResponse {
                    transaction_id: wrapped_announce_request.announce_request.transaction_id,
                    announce_interval: AnnounceInterval(self.tracker.config.udp_tracker.announce_interval as i32),
                    leechers: NumberOfPeers(torrent_stats.leechers as i32),
                    seeders: NumberOfPeers(torrent_stats.seeders as i32),
                    peers: peers.iter().map(|peer|
                        ResponsePeer {
                            ip_address: peer.peer_addr.ip(),
                            port: Port(peer.peer_addr.port())
                        }).collect()
                });

                Ok(response)
            }
            Err(e) => Err(e)
        }
    }

    async fn handle_scrape(&self, request: &ScrapeRequest) -> Result<Response, TorrentError> {
        let db = self.tracker.get_torrents().await;

        let mut torrent_stats: Vec<TorrentScrapeStatistics> = Vec::new();

        for info_hash in request.info_hashes.iter() {
            let info_hash = InfoHash(info_hash.0);
            let scrape_entry = match db.get(&info_hash) {
                Some(torrent_info) => {
                    let (seeders, completed, leechers) = torrent_info.get_stats();

                    TorrentScrapeStatistics {
                        seeders: NumberOfPeers(seeders as i32),
                        completed: NumberOfDownloads(completed as i32),
                        leechers: NumberOfPeers(leechers as i32),
                    }
                }
                None => {
                    TorrentScrapeStatistics {
                        seeders: NumberOfPeers(0),
                        completed: NumberOfDownloads(0),
                        leechers: NumberOfPeers(0),
                    }
                }
            };

            torrent_stats.push(scrape_entry);
        }

        let response = Response::from(ScrapeResponse {
            transaction_id: request.transaction_id,
            torrent_stats
        });

        Ok(response)
    }

    async fn handle_error(&self, e: TorrentError, remote_addr: SocketAddr, tx_id: TransactionId) {
        let mut err_msg = "oops";

        match e {
            TorrentError::TorrentNotWhitelisted => {
                debug!("Info_hash not whitelisted.");
                err_msg = "info hash not whitelisted";
            }
            TorrentError::PeerKeyNotValid => {
                debug!("Peer key not valid.");
                err_msg = "peer key not valid";
            }
            TorrentError::PeerNotAuthenticated => {
                debug!("Peer not authenticated.");
                err_msg = "peer not authenticated";
            }
            TorrentError::NoPeersFound => {
                debug!("No peers found.");
                err_msg = "no peers found";
            }
            _ => {}
        }

        self.send_error(remote_addr, tx_id, err_msg).await;
    }

    async fn send_response(&self, remote_addr: SocketAddr, response: Response) -> Result<usize, ()> {
        debug!("sending response to: {:?}", &remote_addr);

        let buffer = vec![0u8; MAX_PACKET_SIZE];
        let mut cursor = Cursor::new(buffer);

        match response.write(&mut cursor, IpVersion::IPv4) {
            Ok(_) => {
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
            Err(_) => {
                debug!("could not write response to bytes.");
                Err(())
            }
        }
    }

    async fn send_error(&self, remote_addr: SocketAddr, transaction_id: TransactionId, error_msg: &str) {
        let response = Response::from(ErrorResponse {
            transaction_id,
            message: error_msg.to_string(),
        });

        let _ = self.send_response(remote_addr, response).await;
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
}
