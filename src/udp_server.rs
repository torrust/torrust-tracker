use log::{debug};
use std;
use std::convert::TryInto;
use std::io;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::io::{Cursor, Read};
use tokio::net::UdpSocket;
use byteorder::{NetworkEndian, ReadBytesExt};

use super::common::*;
use crate::response::*;
use crate::utils::get_connection_id;
use crate::tracker::TorrentTracker;
use crate::{TorrentPeer, TrackerMode, TorrentError};
use crate::key_manager::AuthKey;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Request {
    Connect(ConnectRequest),
    Announce(AnnounceRequest),
    Scrape(ScrapeRequest),
}

impl From<ConnectRequest> for Request {
    fn from(r: ConnectRequest) -> Self {
        Self::Connect(r)
    }
}

impl From<AnnounceRequest> for Request {
    fn from(r: AnnounceRequest) -> Self {
        Self::Announce(r)
    }
}

impl From<ScrapeRequest> for Request {
    fn from(r: ScrapeRequest) -> Self {
        Self::Scrape(r)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ConnectRequest {
    pub transaction_id: TransactionId,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AnnounceRequest {
    pub connection_id: ConnectionId,
    pub transaction_id: TransactionId,
    pub info_hash: InfoHash,
    pub peer_id: PeerId,
    pub bytes_downloaded: NumberOfBytes,
    pub bytes_uploaded: NumberOfBytes,
    pub bytes_left: NumberOfBytes,
    pub event: AnnounceEvent,
    pub ip_address: Option<Ipv4Addr>,
    pub key: PeerKey,
    pub peers_wanted: NumberOfPeers,
    pub port: Port,
    pub auth_key: Option<AuthKey>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ScrapeRequest {
    pub connection_id: ConnectionId,
    pub transaction_id: TransactionId,
    pub info_hashes: Vec<InfoHash>,
}

#[derive(Debug)]
pub struct RequestParseError {
    pub transaction_id: Option<TransactionId>,
    pub message: Option<String>,
    pub error: Option<io::Error>,
}

impl RequestParseError {
    pub fn new(err: io::Error, transaction_id: i32) -> Self {
        Self {
            transaction_id: Some(TransactionId(transaction_id)),
            message: None,
            error: Some(err),
        }
    }
    pub fn io(err: io::Error) -> Self {
        Self {
            transaction_id: None,
            message: None,
            error: Some(err),
        }
    }
    pub fn text(transaction_id: i32, message: &str) -> Self {
        Self {
            transaction_id: Some(TransactionId(transaction_id)),
            message: Some(message.to_string()),
            error: None,
        }
    }
}

impl Request {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, RequestParseError> {
        let mut cursor = Cursor::new(bytes);

        let connection_id = cursor
            .read_i64::<NetworkEndian>()
            .map_err(RequestParseError::io)?;
        let action = cursor
            .read_i32::<NetworkEndian>()
            .map_err(RequestParseError::io)?;
        let transaction_id = cursor
            .read_i32::<NetworkEndian>()
            .map_err(RequestParseError::io)?;



        match action {
            // Connect
            0 => {
                if connection_id == PROTOCOL_ID {
                    Ok((ConnectRequest {
                        transaction_id: TransactionId(transaction_id),
                    })
                        .into())
                } else {
                    Err(RequestParseError::text(
                        transaction_id,
                        "Protocol identifier missing",
                    ))
                }
            }

            // Announce
            1 => {
                let mut info_hash = [0; 20];
                let mut peer_id = [0; 20];
                let mut ip = [0; 4];

                cursor
                    .read_exact(&mut info_hash)
                    .map_err(|err| RequestParseError::new(err, transaction_id))?;
                cursor
                    .read_exact(&mut peer_id)
                    .map_err(|err| RequestParseError::new(err, transaction_id))?;

                let bytes_downloaded = cursor
                    .read_i64::<NetworkEndian>()
                    .map_err(|err| RequestParseError::new(err, transaction_id))?;
                let bytes_left = cursor
                    .read_i64::<NetworkEndian>()
                    .map_err(|err| RequestParseError::new(err, transaction_id))?;
                let bytes_uploaded = cursor
                    .read_i64::<NetworkEndian>()
                    .map_err(|err| RequestParseError::new(err, transaction_id))?;
                let event = cursor
                    .read_i32::<NetworkEndian>()
                    .map_err(|err| RequestParseError::new(err, transaction_id))?;

                cursor
                    .read_exact(&mut ip)
                    .map_err(|err| RequestParseError::new(err, transaction_id))?;

                let key = cursor
                    .read_u32::<NetworkEndian>()
                    .map_err(|err| RequestParseError::new(err, transaction_id))?;
                let peers_wanted = cursor
                    .read_i32::<NetworkEndian>()
                    .map_err(|err| RequestParseError::new(err, transaction_id))?;
                let port = cursor
                    .read_u16::<NetworkEndian>()
                    .map_err(|err| RequestParseError::new(err, transaction_id))?;

                // BEP 41: add auth key if available
                let auth_key: Option<AuthKey> = if bytes.len() > 98 + AUTH_KEY_LENGTH {
                    let mut key_buffer = [0; AUTH_KEY_LENGTH];
                    // key should be the last bytes
                    cursor.set_position((bytes.len() - AUTH_KEY_LENGTH) as u64);
                    if cursor.read_exact(&mut key_buffer).is_ok() {
                        debug!("AuthKey buffer: {:?}", key_buffer);
                        AuthKey::from_buffer(key_buffer)
                    } else {
                        None
                    }
                } else {
                    None
                };

                let opt_ip = if ip == [0; 4] {
                    None
                } else {
                    Some(Ipv4Addr::from(ip))
                };

                Ok((AnnounceRequest {
                    connection_id: ConnectionId(connection_id),
                    transaction_id: TransactionId(transaction_id),
                    info_hash: InfoHash(info_hash),
                    peer_id: PeerId(peer_id),
                    bytes_downloaded: NumberOfBytes(bytes_downloaded),
                    bytes_uploaded: NumberOfBytes(bytes_uploaded),
                    bytes_left: NumberOfBytes(bytes_left),
                    event: AnnounceEvent::from_i32(event),
                    ip_address: opt_ip,
                    key: PeerKey(key),
                    peers_wanted: NumberOfPeers(peers_wanted),
                    port: Port(port),
                    auth_key,
                })
                    .into())
            }

            // Scrape
            2 => {
                let position = cursor.position() as usize;
                let inner = cursor.into_inner();

                let info_hashes = (&inner[position..])
                    .chunks_exact(20)
                    .take(MAX_SCRAPE_TORRENTS as usize)
                    .map(|chunk| InfoHash(chunk.try_into().unwrap()))
                    .collect();

                Ok((ScrapeRequest {
                    connection_id: ConnectionId(connection_id),
                    transaction_id: TransactionId(transaction_id),
                    info_hashes,
                })
                    .into())
            }

            _ => Err(RequestParseError::text(transaction_id, "Invalid action")),
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

    pub async fn authenticate_announce_request(&self, announce_request: &AnnounceRequest) -> Result<(), TorrentError> {
        match self.tracker.config.mode {
            TrackerMode::PublicMode => Ok(()),
            TrackerMode::ListedMode => {
                if !self.tracker.is_info_hash_whitelisted(&announce_request.info_hash).await {
                    return Err(TorrentError::TorrentNotWhitelisted)
                }

                Ok(())
            }
            TrackerMode::PrivateMode => {
                match &announce_request.auth_key {
                    Some(auth_key) => {
                        if self.tracker.verify_auth_key(auth_key).await.is_err() {
                            return Err(TorrentError::PeerKeyNotValid)
                        }

                        Ok(())
                    }
                    None => {
                        return Err(TorrentError::PeerNotAuthenticated)
                    }
                }
            }
            TrackerMode::PrivateListedMode => {
                match &announce_request.auth_key {
                    Some(auth_key) => {
                        if self.tracker.verify_auth_key(auth_key).await.is_err() {
                            return Err(TorrentError::PeerKeyNotValid)
                        }

                        if !self.tracker.is_info_hash_whitelisted(&announce_request.info_hash).await {
                            return Err(TorrentError::TorrentNotWhitelisted)
                        }

                        Ok(())
                    }
                    None => {
                        return Err(TorrentError::PeerNotAuthenticated)
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
                        match self.tracker.authenticate_request(&r.info_hash, &r.auth_key).await {
                            Ok(()) => self.handle_announce(remote_addr, r).await,
                            Err(e) => {
                                match e {
                                    TorrentError::TorrentNotWhitelisted => {
                                        debug!("Info_hash not whitelisted.");
                                        self.send_error(remote_addr, &r.transaction_id, "torrent not whitelisted").await;
                                    }
                                    TorrentError::PeerKeyNotValid => {
                                        debug!("Peer key not valid.");
                                        self.send_error(remote_addr, &r.transaction_id, "peer key not valid").await;
                                    }
                                    TorrentError::PeerNotAuthenticated => {
                                        debug!("Peer not authenticated.");
                                        self.send_error(remote_addr, &r.transaction_id, "peer not authenticated").await;
                                    }
                                }
                            }
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

        let response = UdpResponse::from(UdpConnectionResponse {
            action: Actions::Connect,
            transaction_id: request.transaction_id,
            connection_id,
        });

        let _ = self.send_response(remote_addr, response).await;
    }

    async fn handle_announce(&self, remote_addr: SocketAddr, request: AnnounceRequest) {
        let peer = TorrentPeer::from_udp_announce_request(&request, remote_addr, self.tracker.config.get_ext_ip());

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

                let response = UdpResponse::from(UdpAnnounceResponse {
                    action: Actions::Announce,
                    transaction_id: request.transaction_id,
                    interval: self.tracker.config.udp_tracker.announce_interval,
                    leechers: torrent_stats.leechers,
                    seeders: torrent_stats.seeders,
                    peers,
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
        let mut scrape_response = UdpScrapeResponse {
            action: Actions::Scrape,
            transaction_id: request.transaction_id,
            torrent_stats: Vec::new(),
        };

        let db = self.tracker.get_torrents().await;

        for info_hash in request.info_hashes.iter() {
            let scrape_entry = match db.get(&info_hash) {
                Some(torrent_info) => {
                    let (seeders, completed, leechers) = torrent_info.get_stats();

                    UdpScrapeResponseEntry {
                        seeders: seeders as i32,
                        completed: completed as i32,
                        leechers: leechers as i32,
                    }
                }
                None => {
                    UdpScrapeResponseEntry {
                        seeders: 0,
                        completed: 0,
                        leechers: 0,
                    }
                }
            };

            scrape_response.torrent_stats.push(scrape_entry);
        }

        let response = UdpResponse::from(scrape_response);

        let _ = self.send_response(remote_addr, response).await;
    }

    async fn send_response(&self, remote_addr: SocketAddr, response: UdpResponse) -> Result<usize, ()> {
        debug!("sending response to: {:?}", &remote_addr);

        let buffer = vec![0u8; MAX_PACKET_SIZE];
        let mut cursor = Cursor::new(buffer);

        match response.write_to_bytes(&mut cursor) {
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
        let error_response = UdpErrorResponse {
            action: Actions::Error,
            transaction_id: transaction_id.clone(),
            message: error_msg.to_string(),
        };

        let response = UdpResponse::from(error_response);

        let _ = self.send_response(remote_addr, response).await;
    }
}
