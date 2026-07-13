//! `Announce` request builder for the HTTP tracker.
//!
//! Types for building announce request URLs to send to an HTTP tracker.
use std::fmt;
use std::net::IpAddr;
use std::str::FromStr;

use torrust_info_hash::InfoHash;
use torrust_peer_id::PeerId;

pub use super::announce::{Compact, Event};
use crate::percent_encoding::percent_encode_byte_array;

/// The announce request query string builder.
pub struct Query {
    pub info_hash: InfoHash,
    pub peer_addr: IpAddr,
    pub downloaded: BaseTenASCII,
    pub uploaded: BaseTenASCII,
    pub peer_id: PeerId,
    pub port: PortNumber,
    pub left: BaseTenASCII,
    pub event: Option<Event>,
    pub compact: Option<Compact>,
    pub numwant: Option<u32>,
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Query {
    /// It builds the URL query component for the announce request.
    #[must_use]
    pub fn build(&self) -> String {
        self.params().to_string()
    }

    #[must_use]
    pub fn params(&self) -> QueryParams {
        QueryParams::from(self)
    }
}

pub type BaseTenASCII = u64;
pub type PortNumber = u16;

/// Builder for constructing an announce `Query`.
pub struct QueryBuilder {
    announce_query: Query,
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::with_default_values()
    }
}

impl QueryBuilder {
    /// # Panics
    ///
    /// Will panic if the default info-hash value is not a valid info-hash.
    #[must_use]
    pub fn with_default_values() -> QueryBuilder {
        let default_announce_query = Query {
            info_hash: InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(), // DevSkim: ignore DS173237
            peer_addr: IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 88)),
            downloaded: 0,
            uploaded: 0,
            peer_id: PeerId(*b"-qB00000000000000001"),
            port: 17548,
            left: 0,
            event: Some(Event::Started),
            compact: Some(Compact::NotAccepted),
            numwant: None,
        };
        Self {
            announce_query: default_announce_query,
        }
    }

    #[must_use]
    pub fn with_info_hash(mut self, info_hash: &InfoHash) -> Self {
        self.announce_query.info_hash = *info_hash;
        self
    }

    #[must_use]
    pub fn with_peer_id(mut self, peer_id: &PeerId) -> Self {
        self.announce_query.peer_id = *peer_id;
        self
    }

    #[must_use]
    pub fn with_event(mut self, event: Event) -> Self {
        self.announce_query.event = Some(event);
        self
    }

    #[must_use]
    pub fn with_uploaded(mut self, uploaded: BaseTenASCII) -> Self {
        self.announce_query.uploaded = uploaded;
        self
    }

    #[must_use]
    pub fn with_downloaded(mut self, downloaded: BaseTenASCII) -> Self {
        self.announce_query.downloaded = downloaded;
        self
    }

    #[must_use]
    pub fn with_left(mut self, left: BaseTenASCII) -> Self {
        self.announce_query.left = left;
        self
    }

    #[must_use]
    pub fn with_port(mut self, port: PortNumber) -> Self {
        self.announce_query.port = port;
        self
    }

    #[must_use]
    pub fn with_compact(mut self, compact: Compact) -> Self {
        self.announce_query.compact = Some(compact);
        self
    }

    #[must_use]
    pub fn with_peer_addr(mut self, peer_addr: &IpAddr) -> Self {
        self.announce_query.peer_addr = *peer_addr;
        self
    }

    #[must_use]
    pub fn without_compact(mut self) -> Self {
        self.announce_query.compact = None;
        self
    }

    #[must_use]
    pub fn query(self) -> Query {
        self.announce_query
    }
}

/// It contains all the GET parameters that can be used in a HTTP Announce request.
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
    pub numwant: Option<String>,
}

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
        if let Some(numwant) = &self.numwant {
            params.push(("numwant", numwant));
        }

        let query = params
            .iter()
            .map(|param| format!("{}={}", param.0, param.1))
            .collect::<Vec<String>>()
            .join("&");

        write!(f, "{query}")
    }
}

impl QueryParams {
    pub fn from(announce_query: &Query) -> Self {
        let event = announce_query.event.as_ref().map(std::string::ToString::to_string);
        let compact = announce_query.compact.as_ref().map(std::string::ToString::to_string);
        let numwant = announce_query.numwant.map(|numwant| numwant.to_string());

        Self {
            info_hash: Some(percent_encode_byte_array(&announce_query.info_hash.bytes())),
            peer_addr: Some(announce_query.peer_addr.to_string()),
            downloaded: Some(announce_query.downloaded.to_string()),
            uploaded: Some(announce_query.uploaded.to_string()),
            peer_id: Some(percent_encode_byte_array(&announce_query.peer_id.0)),
            port: Some(announce_query.port.to_string()),
            left: Some(announce_query.left.to_string()),
            event,
            compact,
            numwant,
        }
    }

    pub fn remove_optional_params(&mut self) {
        self.peer_addr = None;
        self.downloaded = None;
        self.uploaded = None;
        self.left = None;
        self.event = None;
        self.compact = None;
        self.numwant = None;
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
            "numwant" => self.numwant = Some(param_value.to_string()),
            &_ => panic!("Invalid param name for announce query"),
        }
    }
}
