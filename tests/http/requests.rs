use std::fmt;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

use percent_encoding::NON_ALPHANUMERIC;
use serde_repr::Serialize_repr;
use torrust_tracker::protocol::info_hash::InfoHash;
use torrust_tracker::tracker::peer::Id;

pub struct AnnounceQuery {
    pub info_hash: ByteArray20,
    pub peer_addr: IpAddr,
    pub downloaded: BaseTenASCII,
    pub uploaded: BaseTenASCII,
    pub peer_id: ByteArray20,
    pub port: PortNumber,
    pub left: BaseTenASCII,
    pub event: Option<Event>,
    pub compact: Option<Compact>,
}

impl fmt::Display for AnnounceQuery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

/// HTTP Tracker Announce Request:
///
/// <https://wiki.theory.org/BitTorrentSpecification#Tracker_HTTP.2FHTTPS_Protocol>
///
/// Some parameters are not implemented yet.
impl AnnounceQuery {
    /// It builds the URL query component for the announce request.
    ///
    /// This custom URL query params encoding is needed because `reqwest` does not allow
    /// bytes arrays in query parameters. More info on this issue:
    ///
    /// <https://github.com/seanmonstar/reqwest/issues/1613>
    pub fn build(&self) -> String {
        let mut params = vec![
            (
                "info_hash",
                percent_encoding::percent_encode(&self.info_hash, NON_ALPHANUMERIC).to_string(),
            ),
            ("peer_addr", self.peer_addr.to_string()),
            ("downloaded", self.downloaded.to_string()),
            ("uploaded", self.uploaded.to_string()),
            (
                "peer_id",
                percent_encoding::percent_encode(&self.peer_id, NON_ALPHANUMERIC).to_string(),
            ),
            ("port", self.port.to_string()),
            ("left", self.left.to_string()),
        ];

        if let Some(event) = &self.event {
            params.push(("event", event.to_string()));
        }

        if let Some(compact) = &self.compact {
            params.push(("compact", compact.to_string()));
        }

        params
            .iter()
            .map(|param| format!("{}={}", param.0, param.1))
            .collect::<Vec<String>>()
            .join("&")
    }
}

pub type BaseTenASCII = u64;
pub type ByteArray20 = [u8; 20];
pub type PortNumber = u16;

pub enum Event {
    //tarted,
    //Stopped,
    Completed,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            //Event::Started => write!(f, "started"),
            //Event::Stopped => write!(f, "stopped"),
            Event::Completed => write!(f, "completed"),
        }
    }
}

#[derive(Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum Compact {
    //Accepted = 1,
    NotAccepted = 0,
}

impl fmt::Display for Compact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            //Compact::Accepted => write!(f, "1"),
            Compact::NotAccepted => write!(f, "0"),
        }
    }
}

pub struct AnnounceQueryBuilder {
    announce_query: AnnounceQuery,
}

impl AnnounceQueryBuilder {
    pub fn default() -> AnnounceQueryBuilder {
        let default_announce_query = AnnounceQuery {
            info_hash: InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap().0,
            peer_addr: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 88)),
            downloaded: 0,
            uploaded: 0,
            peer_id: Id(*b"-qB00000000000000001").0,
            port: 17548,
            left: 0,
            event: Some(Event::Completed),
            compact: Some(Compact::NotAccepted),
        };
        Self {
            announce_query: default_announce_query,
        }
    }

    pub fn with_info_hash(mut self, info_hash: &InfoHash) -> Self {
        self.announce_query.info_hash = info_hash.0;
        self
    }

    pub fn with_peer_id(mut self, peer_id: &Id) -> Self {
        self.announce_query.peer_id = peer_id.0;
        self
    }

    pub fn into(self) -> AnnounceQuery {
        self.announce_query
    }
}
