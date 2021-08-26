use log::{debug, error, trace};
use std;
use std::io::{Write, Cursor, Read};
use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use std::sync::Arc;
use tokio::net::UdpSocket;

use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};

use bincode;
use serde::{Deserialize, Serialize};

use crate::config::Configuration;
use crate::stackvec::StackVec;
use crate::tracker;
use bincode::Options;

use super::common::*;
use std::convert::TryInto;
use std::io;
use warp::http::Response;

// maximum MTU is usually 1500, but our stack allows us to allocate the maximum - so why not?
const MAX_PACKET_SIZE: usize = 0xffff;

const MAX_SCRAPE_TORRENTS: u8 = 74;

// protocol contants
const PROTOCOL_ID: i64 = 4_497_486_125_440;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum AnnounceEvent {
    Started,
    Stopped,
    Completed,
    None,
}

impl AnnounceEvent {
    #[inline]
    pub fn from_i32(i: i32) -> Self {
        match i {
            1 => Self::Completed,
            2 => Self::Started,
            3 => Self::Stopped,
            _ => Self::None,
        }
    }

    #[inline]
    pub fn to_i32(&self) -> i32 {
        match self {
            AnnounceEvent::None => 0,
            AnnounceEvent::Completed => 1,
            AnnounceEvent::Started => 2,
            AnnounceEvent::Stopped => 3,
        }
    }
}

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
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ScrapeRequest {
    pub connection_id: ConnectionId,
    pub transaction_id: TransactionId,
    pub info_hashes: Vec<InfoHash>,
}

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
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
    pub fn write(self, bytes: &mut impl Write) -> Result<(), io::Error> {
        match self {
            Request::Connect(r) => {
                bytes.write_i64::<NetworkEndian>(PROTOCOL_ID)?;
                bytes.write_i32::<NetworkEndian>(0)?;
                bytes.write_i32::<NetworkEndian>(r.transaction_id.0)?;
            }

            Request::Announce(r) => {
                bytes.write_i64::<NetworkEndian>(r.connection_id.0)?;
                bytes.write_i32::<NetworkEndian>(1)?;
                bytes.write_i32::<NetworkEndian>(r.transaction_id.0)?;

                bytes.write_all(&r.info_hash.0)?;
                bytes.write_all(&r.peer_id.0)?;

                bytes.write_i64::<NetworkEndian>(r.bytes_downloaded.0)?;
                bytes.write_i64::<NetworkEndian>(r.bytes_left.0)?;
                bytes.write_i64::<NetworkEndian>(r.bytes_uploaded.0)?;

                bytes.write_i32::<NetworkEndian>(r.event.to_i32())?;

                // ignore type errors
                bytes.write_all(&r.ip_address.map_or([0; 4], |ip| ip.octets()))?;

                bytes.write_u32::<NetworkEndian>(r.key.0)?;
                bytes.write_i32::<NetworkEndian>(r.peers_wanted.0)?;
                bytes.write_u16::<NetworkEndian>(r.port.0)?;
            }

            Request::Scrape(r) => {
                bytes.write_i64::<NetworkEndian>(r.connection_id.0)?;
                bytes.write_i32::<NetworkEndian>(2)?;
                bytes.write_i32::<NetworkEndian>(r.transaction_id.0)?;

                for info_hash in r.info_hashes {
                    bytes.write_all(&info_hash.0)?;
                }
            }
        }

        Ok(())
    }

    pub fn from_bytes(bytes: &[u8], max_scrape_torrents: u8) -> Result<Self, RequestParseError> {
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
                })
                    .into())
            }

            // Scrape
            2 => {
                let position = cursor.position() as usize;
                let inner = cursor.into_inner();

                let info_hashes = (&inner[position..])
                    .chunks_exact(20)
                    .take(max_scrape_torrents as usize)
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

#[derive(Serialize, Deserialize, Debug)]
struct UDPRequestHeader {
    connection_id: u64,
    action: Actions,
    transaction_id: u32,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum UDPResponse {
    Connect(UDPConnectionResponse),
    Announce(UDPAnnounceResponse),
    Scrape(UDPScrapeResponseEntry),
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
                // if ip_version == IpVersion::IPv4 {
                //     bytes.write_i32::<NetworkEndian>(1)?; // 1 = announce
                // } else {
                //     bytes.write_i32::<NetworkEndian>(4)?;
                // }

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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UDPConnectionResponse {
    action: Actions,
    transaction_id: TransactionId,
    connection_id: ConnectionId,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UDPAnnounceResponse {
    action: Actions,
    transaction_id: TransactionId,
    interval: u32,
    leechers: u32,
    seeders: u32,
    peers: ResponsePeerList,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UDPScrapeResponseEntry {
    seeders: u32,
    completed: u32,
    leechers: u32,
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

pub struct UDPTracker {
    srv: tokio::net::UdpSocket,
    tracker: std::sync::Arc<tracker::TorrentTracker>,
    config: Arc<Configuration>,
}

impl UDPTracker {
    pub async fn new(
        config: Arc<Configuration>, tracker: std::sync::Arc<tracker::TorrentTracker>,
    ) -> Result<UDPTracker, std::io::Error> {
        let cfg = config.clone();

        let srv = UdpSocket::bind(cfg.get_udp_config().get_address()).await?;

        Ok(UDPTracker {
            srv,
            tracker,
            config: cfg,
        })
    }

    async fn handle_packet(&self, remote_address: SocketAddr, payload: &[u8]) {
        let request = Request::from_bytes(&payload[..payload.len()], MAX_SCRAPE_TORRENTS);

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
        let connection_id = self.get_connection_id(&remote_addr);

        let response = UDPResponse::from(UDPConnectionResponse {
            action: Actions::Connect,
            transaction_id: request.transaction_id,
            connection_id,
        });

        let _ = self.send_response(remote_addr, response).await;
    }

    async fn handle_announce(&self, remote_addr: SocketAddr, request: AnnounceRequest) {
        // todo: I have no idea yet why this is here
        if request.connection_id != self.get_connection_id(&remote_addr) {
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
                match self.srv.send_to(bytes.as_slice(), remote_addr).await {
                    Ok(sz) => Ok(sz),
                    Err(err) => {
                        debug!("failed to send a packet: {}", err);
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

    fn get_connection_id(&self, remote_address: &SocketAddr) -> ConnectionId {
        match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(duration) => ConnectionId(((duration.as_secs() / 3600) | ((remote_address.port() as u64) << 36)) as i64),
            Err(_) => ConnectionId(0x7FFFFFFFFFFFFFFF),
        }
    }

    async fn send_packet(&self, remote_addr: &SocketAddr, payload: &[u8]) -> Result<usize, std::io::Error> {
        match self.srv.send_to(payload, remote_addr).await {
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
            let (size, remote_address) = tracker.srv.recv_from(packet.as_mut_slice()).await?;

            let tracker = tracker.clone();
            tokio::spawn(async move {
                debug!("Received {} bytes from {}", size, remote_address);
                tracker.handle_packet(remote_address, &packet[..size]).await;
            });
        }
    }
}
