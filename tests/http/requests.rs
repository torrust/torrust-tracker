use std::fmt;
use std::net::IpAddr;

use percent_encoding::NON_ALPHANUMERIC;
use serde_repr::Serialize_repr;

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
