use std::fmt;
use std::net::IpAddr;

use serde_repr::Serialize_repr;
use torrust_tracker_primitives::announce_event::AnnounceEvent;
use torrust_tracker_primitives::info_hash::InfoHash;
use torrust_tracker_primitives::peer;

use super::Announce;
use crate::shared::bit_torrent::tracker::http::{percent_encode_byte_array, ByteArray20};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(super) struct Query {
    pub info_hash: ByteArray20,
    pub peer_addr: Option<IpAddr>,
    pub downloaded: Option<BaseTenASCII>,
    pub uploaded: Option<BaseTenASCII>,
    pub peer_id: ByteArray20,
    pub port: PortNumber,
    pub left: Option<BaseTenASCII>,
    pub event: Option<AnnounceEvent>,
    pub compact: Option<Compact>,
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", QueryParams::from(self))
    }
}

pub type BaseTenASCII = u64;
pub type PortNumber = u16;

#[derive(Serialize_repr, PartialEq, Eq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum Compact {
    Accepted = 1,
    NotAccepted = 0,
}

impl fmt::Display for Compact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Compact::Accepted => write!(f, "1"),
            Compact::NotAccepted => write!(f, "0"),
        }
    }
}

#[derive(Debug)]
pub struct QueryBuilder {
    query: Query,
}

impl QueryBuilder {
    /// # Panics
    ///
    /// Will panic if the default info-hash value is not a valid info-hash.
    #[must_use]
    pub fn new(info_hash: InfoHash, peer_id: peer::Id, port: u16) -> QueryBuilder {
        Self {
            query: Query {
                info_hash: info_hash.0,
                peer_addr: None,
                downloaded: None,
                uploaded: None,
                peer_id: peer_id.0,
                port,
                left: None,
                event: None,
                compact: None,
            },
        }
    }

    #[must_use]
    pub fn with_event(mut self, event: AnnounceEvent) -> Self {
        self.query.event = Some(event);
        self
    }

    #[must_use]
    pub fn with_compact(mut self) -> Self {
        self.query.compact = Some(Compact::Accepted);
        self
    }

    #[must_use]
    pub fn without_compact(mut self) -> Self {
        self.query.compact = Some(Compact::NotAccepted);
        self
    }

    #[must_use]
    pub fn with_peer_addr(mut self, peer_addr: &IpAddr) -> Self {
        self.query.peer_addr = Some(*peer_addr);
        self
    }

    #[must_use]
    pub fn build(self) -> Announce {
        self.query.into()
    }
}

/// It contains all the GET parameters that can be used in a HTTP Announce request.
///
/// Sample Announce URL with all the GET parameters (mandatory and optional):
///
/// ```text
/// http://127.0.0.1:7070/announce?
///     info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22 (mandatory)
///     peer_addr=192.168.1.88
///     downloaded=0
///     uploaded=0
///     peer_id=%2DqB00000000000000000 (mandatory)
///     port=17548 (mandatory)
///     left=0
///     event=completed
///     compact=0
/// ```
pub struct QueryParams {
    pub info_hash: Option<String>,
    pub peer_addr: Option<String>,
    pub downloaded: Option<String>,
    pub uploaded: Option<String>,
    pub peer_id: Option<String>,
    pub port: Option<String>,
    pub left: Option<String>,
    pub event: Option<String>,
    pub compact: Option<String>,
}

/// It builds the URL query component for the announce request.
///
/// This custom URL query params encoding is needed because `reqwest` does not allow
/// bytes arrays in query parameters. More info on this issue:
///
/// <https://github.com/seanmonstar/reqwest/issues/1613>
impl std::fmt::Display for QueryParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = vec![];

        if let Some(info_hash) = &self.info_hash {
            params.push(("info_hash", info_hash));
        }
        if let Some(peer_addr) = &self.peer_addr {
            params.push(("peer_addr", peer_addr));
        }
        if let Some(downloaded) = &self.downloaded {
            params.push(("downloaded", downloaded));
        }
        if let Some(uploaded) = &self.uploaded {
            params.push(("uploaded", uploaded));
        }
        if let Some(peer_id) = &self.peer_id {
            params.push(("peer_id", peer_id));
        }
        if let Some(port) = &self.port {
            params.push(("port", port));
        }
        if let Some(left) = &self.left {
            params.push(("left", left));
        }
        if let Some(event) = &self.event {
            params.push(("event", event));
        }
        if let Some(compact) = &self.compact {
            params.push(("compact", compact));
        }

        let query = params
            .iter()
            .map(|param| format!("{}={}", param.0, param.1))
            .collect::<Vec<String>>()
            .join("&");

        write!(f, "{query}")
    }
}

impl From<&Announce> for QueryParams {
    fn from(value: &Announce) -> Self {
        let query: &Query = &Announce::into(*value);
        query.into()
    }
}

impl From<&Query> for QueryParams {
    fn from(value: &Query) -> Self {
        let query = value;

        Self {
            info_hash: Some(percent_encode_byte_array(&query.info_hash)),
            peer_addr: query.peer_addr.as_ref().map(std::string::ToString::to_string),
            downloaded: query.downloaded.as_ref().map(std::string::ToString::to_string),
            uploaded: query.uploaded.as_ref().map(std::string::ToString::to_string),
            peer_id: Some(percent_encode_byte_array(&query.peer_id)),
            port: Some(query.port.to_string()),
            left: query.left.as_ref().map(std::string::ToString::to_string),
            event: query.event.as_ref().map(std::string::ToString::to_string),
            compact: query.compact.as_ref().map(std::string::ToString::to_string),
        }
    }
}

impl QueryParams {
    pub fn remove_optional_params(&mut self) {
        // todo: make them optional with the Option<...> in the AnnounceQuery struct
        // if they are really optional. So that we can crete a minimal AnnounceQuery
        // instead of removing the optional params afterwards.
        //
        // The original specification on:
        // <https://www.bittorrent.org/beps/bep_0003.html>
        // says only `ip` and `event` are optional.
        //
        // On <https://wiki.theory.org/BitTorrentSpecification#Tracker_Request_Parameters>
        // says only `ip`, `numwant`, `key` and `trackerid` are optional.
        //
        // but the server is responding if all these params are not included.
        self.peer_addr = None;
        self.downloaded = None;
        self.uploaded = None;
        self.left = None;
        self.event = None;
        self.compact = None;
    }

    /// # Panics
    ///
    /// Will panic if invalid param name is provided.
    pub fn set(&mut self, param_name: &str, param_value: &str) {
        match param_name {
            "info_hash" => self.info_hash = Some(param_value.to_string()),
            "peer_addr" => self.peer_addr = Some(param_value.to_string()),
            "downloaded" => self.downloaded = Some(param_value.to_string()),
            "uploaded" => self.uploaded = Some(param_value.to_string()),
            "peer_id" => self.peer_id = Some(param_value.to_string()),
            "port" => self.port = Some(param_value.to_string()),
            "left" => self.left = Some(param_value.to_string()),
            "event" => self.event = Some(param_value.to_string()),
            "compact" => self.compact = Some(param_value.to_string()),
            &_ => panic!("Invalid param name for announce query"),
        }
    }
}
