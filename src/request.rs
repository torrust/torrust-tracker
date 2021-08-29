use crate::common::*;
use std::net::Ipv4Addr;
use std::io;
use std::io::{Cursor, Read};
use byteorder::{NetworkEndian, ReadBytesExt};
use std::convert::TryInto;
use serde::{Deserialize, Serialize};

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
