//! `Announce` request for the HTTP tracker.
//!
//! Data structures and logic for parsing and building the `announce` request.
//! This type is used both for server-side parsing (via `TryFrom<Query>`) and
//! client-side construction (via `AnnounceBuilder` + `Display`).
use std::fmt;
use std::net::IpAddr;
use std::panic::Location;
use std::str::FromStr;

use thiserror::Error;
use torrust_info_hash::InfoHash;
use torrust_located_error::{Located, LocatedError};
use torrust_peer_id::PeerId;

use crate::percent_encoding::{
    PeerIdConversionError, percent_decode_info_hash, percent_decode_peer_id, percent_encode_byte_array,
};
use crate::v1::query::{ParseQueryError, Query};
use crate::v1::responses;

// Query param names
const INFO_HASH: &str = "info_hash";
const PEER_ID: &str = "peer_id";
const PORT: &str = "port";
const DOWNLOADED: &str = "downloaded";
const UPLOADED: &str = "uploaded";
const LEFT: &str = "left";
const EVENT: &str = "event";
const COMPACT: &str = "compact";
const NUMWANT: &str = "numwant";
const PEER_ADDR: &str = "peer_addr";

// Intentionally protocol-local: this currently mirrors the UDP protocol
// `NumberOfBytes` concept and domain byte counters, but it is kept local so
// HTTP wire semantics can evolve independently without forcing cross-protocol
// or domain-wide refactors.
// adr: docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct NumberOfBytes(pub i64);

impl NumberOfBytes {
    #[must_use]
    pub const fn new(v: i64) -> Self {
        Self(v)
    }
}

/// The `Announce` request. Fields use protocol-local types after parsing the
/// query params of the request; boundary layers map them to domain types.
///
/// This type is used for both server-side parsing and client-side construction:
///
/// - **Server-side**: Parsed from incoming HTTP query strings via `TryFrom<Query>`.
/// - **Client-side**: Built via `AnnounceBuilder` and serialized to a URL query
///   string via `Display`.
///
/// ```rust
/// use torrust_tracker_http_protocol::v1::requests::announce::{Announce, Compact, Event};
/// use torrust_info_hash::InfoHash;
/// use torrust_peer_id::PeerId;
/// use torrust_tracker_http_protocol::v1::requests::announce::NumberOfBytes;
///
/// let request = Announce {
///     // Mandatory params
///     info_hash: "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0".parse::<InfoHash>().unwrap(),
///     peer_id: PeerId(*b"-RC3000-000000000001"),
///     port: 17548,
///     // Optional params
///     peer_addr: None,
///     downloaded: Some(NumberOfBytes::new(1)),
///     uploaded: Some(NumberOfBytes::new(1)),
///     left: Some(NumberOfBytes::new(1)),
///     event: Some(Event::Started),
///     compact: Some(Compact::NotAccepted),
///     numwant: Some(50)
/// };
/// ```
///
/// > **NOTICE**: The [BEP 03. The `BitTorrent` Protocol Specification](https://www.bittorrent.org/beps/bep_0003.html)
/// > specifies that only the peer `IP` and `event` are optional. However, the
/// > tracker defines default values for some of the mandatory params.
///
/// > **NOTICE**: The struct contains `peer_addr` as per BEP 3. The tracker
/// > implementation may choose to use it or derive the IP from the connection.
#[derive(Clone, Debug, PartialEq)]
pub struct Announce {
    // Mandatory params
    /// The `InfoHash` of the torrent.
    pub info_hash: InfoHash,

    /// The `PeerId` of the peer.
    pub peer_id: PeerId,

    /// The port of the peer.
    pub port: u16,

    // Optional params
    /// The peer IP address (BEP 3 `ip` parameter).
    pub peer_addr: Option<IpAddr>,

    /// The number of bytes downloaded by the peer.
    pub downloaded: Option<NumberOfBytes>,

    /// The number of bytes uploaded by the peer.
    pub uploaded: Option<NumberOfBytes>,

    /// The number of bytes left to download by the peer.
    pub left: Option<NumberOfBytes>,

    /// The event that the peer is reporting. It can be `Started`, `Stopped` or
    /// `Completed`.
    pub event: Option<Event>,

    /// Whether the response should be in compact mode or not.
    pub compact: Option<Compact>,

    /// Number of peers that the client would receive from the tracker. The
    /// value is permitted to be zero.
    pub numwant: Option<u32>,
}

/// Errors that can occur when parsing the `Announce` request.
///
/// The `info_hash` and `peer_id` query params are special because they contain
/// binary data. The `info_hash` is a 20-byte SHA1 hash and the `peer_id` is a
/// 20-byte array.
#[derive(Error, Debug)]
pub enum ParseAnnounceQueryError {
    /// A mandatory param is missing.
    #[error("missing query params for announce request in {location}")]
    MissingParams { location: &'static Location<'static> },
    #[error("missing param {param_name} in {location}")]
    MissingParam {
        location: &'static Location<'static>,
        param_name: String,
    },
    /// The param cannot be parsed into the domain type.
    #[error("invalid param value {param_value} for {param_name} in {location}")]
    InvalidParam {
        param_name: String,
        param_value: String,
        location: &'static Location<'static>,
    },
    /// The param value is out of range.
    #[error("param value overflow {param_value} for {param_name} in {location}")]
    NumberOfBytesOverflow {
        param_name: String,
        param_value: String,
        location: &'static Location<'static>,
    },
    /// The `info_hash` is invalid.
    #[error("invalid param value {param_value} for {param_name} in {source}")]
    InvalidInfoHashParam {
        param_name: String,
        param_value: String,
        source: LocatedError<'static, torrust_info_hash::ConversionError>,
    },
    /// The `peer_id` is invalid.
    #[error("invalid param value {param_value} for {param_name} in {source}")]
    InvalidPeerIdParam {
        param_name: String,
        param_value: String,
        source: LocatedError<'static, PeerIdConversionError>,
    },
}

/// The event that the peer is reporting: `started`, `completed` or `stopped`.
///
/// If the event is not present or empty that means that the peer is just
/// updating its status. It's one of the announcements done at regular intervals.
///
/// Refer to [BEP 03. The `BitTorrent Protocol` Specification](https://www.bittorrent.org/beps/bep_0003.html)
/// for more information.
#[derive(PartialEq, Debug, Clone)]
pub enum Event {
    /// Event sent when a download first begins.
    Started,

    /// Event sent when the downloader cease downloading.
    Stopped,

    /// Event sent when the download is complete.
    /// No `completed` is sent if the file was complete when started.
    Completed,

    /// It is the same as not being present. If not present, this is one of the
    /// announcements done at regular intervals.
    Empty,
}

impl FromStr for Event {
    type Err = ParseAnnounceQueryError;

    fn from_str(raw_param: &str) -> Result<Self, Self::Err> {
        match raw_param {
            "started" => Ok(Self::Started),
            "stopped" => Ok(Self::Stopped),
            "completed" => Ok(Self::Completed),
            "empty" => Ok(Self::Empty),
            _ => Err(ParseAnnounceQueryError::InvalidParam {
                param_name: EVENT.to_owned(),
                param_value: raw_param.to_owned(),
                location: Location::caller(),
            }),
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::Started => write!(f, "started"),
            Event::Stopped => write!(f, "stopped"),
            Event::Completed => write!(f, "completed"),
            Event::Empty => write!(f, "empty"),
        }
    }
}

/// Whether the `announce` response should be in compact mode or not.
///
/// Depending on the value of this param, the tracker will return a different
/// response:
///
/// - [`Normal`](crate::v1::responses::announce::Normal), i.e. a `non-compact` response.
/// - [`Compact`](crate::v1::responses::announce::Compact) response.
///
/// Refer to [BEP 23. Tracker Returns Compact Peer Lists](https://www.bittorrent.org/beps/bep_0023.html)
#[derive(Clone, Debug, PartialEq)]
pub enum Compact {
    /// The client advises the tracker that the client prefers compact format.
    Accepted = 1,
    /// The client advises the tracker that is prefers the original format
    /// described in [BEP 03. The BitTorrent Protocol Specification](https://www.bittorrent.org/beps/bep_0003.html)
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

impl FromStr for Compact {
    type Err = ParseAnnounceQueryError;

    fn from_str(raw_param: &str) -> Result<Self, Self::Err> {
        match raw_param {
            "1" => Ok(Self::Accepted),
            "0" => Ok(Self::NotAccepted),
            _ => Err(ParseAnnounceQueryError::InvalidParam {
                param_name: COMPACT.to_owned(),
                param_value: raw_param.to_owned(),
                location: Location::caller(),
            }),
        }
    }
}

impl From<ParseQueryError> for responses::error::Error {
    fn from(err: ParseQueryError) -> Self {
        responses::error::Error {
            failure_reason: format!("Bad request. Cannot parse query params: {err}"),
        }
    }
}

impl From<ParseAnnounceQueryError> for responses::error::Error {
    fn from(err: ParseAnnounceQueryError) -> Self {
        responses::error::Error {
            failure_reason: format!("Bad request. Cannot parse query params for announce request: {err}"),
        }
    }
}

impl TryFrom<Query> for Announce {
    type Error = ParseAnnounceQueryError;

    fn try_from(query: Query) -> Result<Self, Self::Error> {
        Ok(Self {
            info_hash: extract_info_hash(&query)?,
            peer_id: extract_peer_id(&query)?,
            port: extract_port(&query)?,
            downloaded: extract_downloaded(&query)?,
            uploaded: extract_uploaded(&query)?,
            left: extract_left(&query)?,
            event: extract_event(&query)?,
            compact: extract_compact(&query)?,
            numwant: extract_numwant(&query)?,
            peer_addr: extract_peer_addr(&query),
        })
    }
}

impl fmt::Display for Announce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut params = vec![];

        params.push(("info_hash", percent_encode_byte_array(&self.info_hash.bytes())));
        params.push(("peer_id", percent_encode_byte_array(&self.peer_id.0)));
        params.push(("port", self.port.to_string()));

        if let Some(peer_addr) = &self.peer_addr {
            params.push(("peer_addr", peer_addr.to_string()));
        }
        if let Some(downloaded) = self.downloaded {
            params.push(("downloaded", downloaded.0.to_string()));
        }
        if let Some(uploaded) = self.uploaded {
            params.push(("uploaded", uploaded.0.to_string()));
        }
        if let Some(left) = self.left {
            params.push(("left", left.0.to_string()));
        }
        if let Some(event) = &self.event {
            params.push(("event", event.to_string()));
        }
        if let Some(compact) = &self.compact {
            params.push(("compact", compact.to_string()));
        }
        if let Some(numwant) = self.numwant {
            params.push(("numwant", numwant.to_string()));
        }

        let query = params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<String>>()
            .join("&");

        write!(f, "{query}")
    }
}

/// Builder for constructing an [`Announce`] request for client-side use.
///
/// Provides ergonomic construction with sensible defaults. The resulting
/// [`Announce`] can be serialized to a URL query string via its `Display` impl.
///
/// ```rust
/// use std::net::{IpAddr, Ipv4Addr};
/// use std::str::FromStr;
/// use torrust_tracker_http_protocol::v1::requests::announce::{AnnounceBuilder, Event, Compact};
/// use torrust_info_hash::InfoHash;
///
/// let announce = AnnounceBuilder::default()
///     .with_info_hash(&InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap())
///     .query();
///
/// let query_string = announce.to_string();
/// ```
#[derive(Clone, Debug)]
pub struct AnnounceBuilder {
    announce: Announce,
}

impl Default for AnnounceBuilder {
    fn default() -> Self {
        Self::with_default_values()
    }
}

impl AnnounceBuilder {
    /// Creates a builder with default test values.
    ///
    /// # Panics
    ///
    /// Will panic if the default info-hash value is not a valid info-hash.
    #[must_use]
    pub fn with_default_values() -> AnnounceBuilder {
        let default_announce = Announce {
            info_hash: InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(), // DevSkim: ignore DS173237
            peer_id: PeerId(*b"-qB00000000000000001"),
            port: 17548,
            peer_addr: Some(IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 88))),
            downloaded: None,
            uploaded: None,
            left: None,
            event: Some(Event::Started),
            compact: Some(Compact::NotAccepted),
            numwant: None,
        };
        Self {
            announce: default_announce,
        }
    }

    #[must_use]
    pub fn with_info_hash(mut self, info_hash: &InfoHash) -> Self {
        self.announce.info_hash = *info_hash;
        self
    }

    #[must_use]
    pub fn with_peer_id(mut self, peer_id: &PeerId) -> Self {
        self.announce.peer_id = *peer_id;
        self
    }

    #[must_use]
    pub fn with_port(mut self, port: u16) -> Self {
        self.announce.port = port;
        self
    }

    #[must_use]
    pub fn with_peer_addr(mut self, peer_addr: IpAddr) -> Self {
        self.announce.peer_addr = Some(peer_addr);
        self
    }

    #[must_use]
    pub fn with_event(mut self, event: Event) -> Self {
        self.announce.event = Some(event);
        self
    }

    /// # Panics
    ///
    /// Panics if `downloaded` exceeds `i64::MAX`.
    #[must_use]
    pub fn with_downloaded(mut self, downloaded: u64) -> Self {
        self.announce.downloaded = Some(NumberOfBytes::new(
            i64::try_from(downloaded).expect("downloaded value fits in i64"),
        ));
        self
    }

    /// # Panics
    ///
    /// Panics if `uploaded` exceeds `i64::MAX`.
    #[must_use]
    pub fn with_uploaded(mut self, uploaded: u64) -> Self {
        self.announce.uploaded = Some(NumberOfBytes::new(
            i64::try_from(uploaded).expect("uploaded value fits in i64"),
        ));
        self
    }

    /// # Panics
    ///
    /// Panics if `left` exceeds `i64::MAX`.
    #[must_use]
    pub fn with_left(mut self, left: u64) -> Self {
        self.announce.left = Some(NumberOfBytes::new(i64::try_from(left).expect("left value fits in i64")));
        self
    }

    #[must_use]
    pub fn with_compact(mut self, compact: Compact) -> Self {
        self.announce.compact = Some(compact);
        self
    }

    #[must_use]
    pub fn without_compact(mut self) -> Self {
        self.announce.compact = None;
        self
    }

    #[must_use]
    pub fn with_numwant(mut self, numwant: u32) -> Self {
        self.announce.numwant = Some(numwant);
        self
    }

    /// Consumes the builder and returns the constructed [`Announce`].
    #[must_use]
    pub fn query(self) -> Announce {
        self.announce
    }
}

// Mandatory params

fn extract_info_hash(query: &Query) -> Result<InfoHash, ParseAnnounceQueryError> {
    match query.get_param(INFO_HASH) {
        Some(raw_param) => {
            Ok(
                percent_decode_info_hash(&raw_param).map_err(|err| ParseAnnounceQueryError::InvalidInfoHashParam {
                    param_name: INFO_HASH.to_owned(),
                    param_value: raw_param.clone(),
                    source: Located(err).into(),
                })?,
            )
        }
        None => Err(ParseAnnounceQueryError::MissingParam {
            location: Location::caller(),
            param_name: INFO_HASH.to_owned(),
        }),
    }
}

fn extract_peer_id(query: &Query) -> Result<PeerId, ParseAnnounceQueryError> {
    match query.get_param(PEER_ID) {
        Some(raw_param) => Ok(
            percent_decode_peer_id(&raw_param).map_err(|err| ParseAnnounceQueryError::InvalidPeerIdParam {
                param_name: PEER_ID.to_owned(),
                param_value: raw_param.clone(),
                source: Located(err).into(),
            })?,
        ),
        None => Err(ParseAnnounceQueryError::MissingParam {
            location: Location::caller(),
            param_name: PEER_ID.to_owned(),
        }),
    }
}

fn extract_port(query: &Query) -> Result<u16, ParseAnnounceQueryError> {
    match query.get_param(PORT) {
        Some(raw_param) => Ok(u16::from_str(&raw_param).map_err(|_e| ParseAnnounceQueryError::InvalidParam {
            param_name: PORT.to_owned(),
            param_value: raw_param.clone(),
            location: Location::caller(),
        })?),
        None => Err(ParseAnnounceQueryError::MissingParam {
            location: Location::caller(),
            param_name: PORT.to_owned(),
        }),
    }
}

// Optional params

fn extract_downloaded(query: &Query) -> Result<Option<NumberOfBytes>, ParseAnnounceQueryError> {
    extract_number_of_bytes_from_param(DOWNLOADED, query)
}

fn extract_uploaded(query: &Query) -> Result<Option<NumberOfBytes>, ParseAnnounceQueryError> {
    extract_number_of_bytes_from_param(UPLOADED, query)
}

fn extract_left(query: &Query) -> Result<Option<NumberOfBytes>, ParseAnnounceQueryError> {
    extract_number_of_bytes_from_param(LEFT, query)
}

fn extract_number_of_bytes_from_param(param_name: &str, query: &Query) -> Result<Option<NumberOfBytes>, ParseAnnounceQueryError> {
    match query.get_param(param_name) {
        Some(raw_param) => {
            let number_of_bytes = u64::from_str(&raw_param).map_err(|_e| ParseAnnounceQueryError::InvalidParam {
                param_name: param_name.to_owned(),
                param_value: raw_param.clone(),
                location: Location::caller(),
            })?;

            let number_of_bytes =
                i64::try_from(number_of_bytes).map_err(|_e| ParseAnnounceQueryError::NumberOfBytesOverflow {
                    param_name: param_name.to_owned(),
                    param_value: raw_param.clone(),
                    location: Location::caller(),
                })?;

            let number_of_bytes = NumberOfBytes::new(number_of_bytes);

            Ok(Some(number_of_bytes))
        }
        None => Ok(None),
    }
}

fn extract_peer_addr(query: &Query) -> Option<IpAddr> {
    match query.get_param(PEER_ADDR) {
        Some(raw_param) => IpAddr::from_str(&raw_param).ok(),
        None => None,
    }
}

fn extract_event(query: &Query) -> Result<Option<Event>, ParseAnnounceQueryError> {
    match query.get_param(EVENT) {
        Some(raw_param) => Ok(Some(Event::from_str(&raw_param)?)),
        None => Ok(None),
    }
}

fn extract_compact(query: &Query) -> Result<Option<Compact>, ParseAnnounceQueryError> {
    match query.get_param(COMPACT) {
        Some(raw_param) => Ok(Some(Compact::from_str(&raw_param)?)),
        None => Ok(None),
    }
}

fn extract_numwant(query: &Query) -> Result<Option<u32>, ParseAnnounceQueryError> {
    match query.get_param(NUMWANT) {
        Some(raw_param) => match u32::from_str(&raw_param) {
            Ok(numwant) => Ok(Some(numwant)),
            Err(_) => Err(ParseAnnounceQueryError::InvalidParam {
                param_name: NUMWANT.to_owned(),
                param_value: raw_param.clone(),
                location: Location::caller(),
            }),
        },
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {

    mod announce_request {

        use torrust_info_hash::InfoHash;
        use torrust_peer_id::PeerId;

        use crate::v1::query::Query;
        use crate::v1::requests::announce::{
            Announce, COMPACT, Compact, DOWNLOADED, EVENT, Event, INFO_HASH, LEFT, NUMWANT, NumberOfBytes, PEER_ID, PORT,
            UPLOADED,
        };

        #[test]
        fn should_be_instantiated_from_the_url_query_with_only_the_mandatory_params() {
            let raw_query = Query::from(vec![
                (INFO_HASH, "%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0"),
                (PEER_ID, "-RC3000-000000000001"),
                (PORT, "17548"),
            ])
            .to_string();

            let query = raw_query.parse::<Query>().unwrap();

            let announce_request = Announce::try_from(query).unwrap();

            assert_eq!(
                announce_request,
                Announce {
                    info_hash: "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0".parse::<InfoHash>().unwrap(), // DevSkim: ignore DS173237
                    peer_id: PeerId(*b"-RC3000-000000000001"),
                    port: 17548,
                    peer_addr: None,
                    downloaded: None,
                    uploaded: None,
                    left: None,
                    event: None,
                    compact: None,
                    numwant: None,
                }
            );
        }

        #[test]
        fn should_be_instantiated_from_the_url_query_params() {
            let raw_query = Query::from(vec![
                (INFO_HASH, "%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0"),
                (PEER_ID, "-RC3000-000000000001"),
                (PORT, "17548"),
                (DOWNLOADED, "1"),
                (UPLOADED, "2"),
                (LEFT, "3"),
                (EVENT, "started"),
                (COMPACT, "0"),
                (NUMWANT, "50"),
            ])
            .to_string();

            let query = raw_query.parse::<Query>().unwrap();

            let announce_request = Announce::try_from(query).unwrap();

            assert_eq!(
                announce_request,
                Announce {
                    info_hash: "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0".parse::<InfoHash>().unwrap(), // DevSkim: ignore DS173237
                    peer_id: PeerId(*b"-RC3000-000000000001"),
                    port: 17548,
                    peer_addr: None,
                    downloaded: Some(NumberOfBytes::new(1)),
                    uploaded: Some(NumberOfBytes::new(2)),
                    left: Some(NumberOfBytes::new(3)),
                    event: Some(Event::Started),
                    compact: Some(Compact::NotAccepted),
                    numwant: Some(50),
                }
            );
        }

        mod when_it_is_instantiated_from_the_url_query_params {

            use crate::v1::query::Query;
            use crate::v1::requests::announce::{
                Announce, COMPACT, DOWNLOADED, EVENT, INFO_HASH, LEFT, NUMWANT, PEER_ID, PORT, UPLOADED,
            };

            #[test]
            fn it_should_fail_if_the_query_does_not_include_all_the_mandatory_params() {
                let raw_query_without_info_hash = "peer_id=-RC3000-000000000001&port=17548";

                assert!(Announce::try_from(raw_query_without_info_hash.parse::<Query>().unwrap()).is_err());

                let raw_query_without_peer_id = "info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&port=17548";

                assert!(Announce::try_from(raw_query_without_peer_id.parse::<Query>().unwrap()).is_err());

                let raw_query_without_port =
                    "info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&peer_id=-RC3000-000000000001";

                assert!(Announce::try_from(raw_query_without_port.parse::<Query>().unwrap()).is_err());
            }

            #[test]
            fn it_should_fail_if_the_info_hash_param_is_invalid() {
                let raw_query = Query::from(vec![
                    (INFO_HASH, "INVALID_INFO_HASH_VALUE"),
                    (PEER_ID, "-RC3000-000000000001"),
                    (PORT, "17548"),
                ])
                .to_string();

                assert!(Announce::try_from(raw_query.parse::<Query>().unwrap()).is_err());
            }

            #[test]
            fn it_should_fail_if_the_peer_id_param_is_invalid() {
                let raw_query = Query::from(vec![
                    (INFO_HASH, "%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0"),
                    (PEER_ID, "INVALID_PEER_ID_VALUE"),
                    (PORT, "17548"),
                ])
                .to_string();

                assert!(Announce::try_from(raw_query.parse::<Query>().unwrap()).is_err());
            }

            #[test]
            fn it_should_fail_if_the_port_param_is_invalid() {
                let raw_query = Query::from(vec![
                    (INFO_HASH, "%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0"),
                    (PEER_ID, "-RC3000-000000000001"),
                    (PORT, "INVALID_PORT_VALUE"),
                ])
                .to_string();

                assert!(Announce::try_from(raw_query.parse::<Query>().unwrap()).is_err());
            }

            #[test]
            fn it_should_fail_if_the_downloaded_param_is_invalid() {
                let raw_query = Query::from(vec![
                    (INFO_HASH, "%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0"),
                    (PEER_ID, "-RC3000-000000000001"),
                    (PORT, "17548"),
                    (DOWNLOADED, "INVALID_DOWNLOADED_VALUE"),
                ])
                .to_string();

                assert!(Announce::try_from(raw_query.parse::<Query>().unwrap()).is_err());
            }

            #[test]
            fn it_should_fail_if_the_uploaded_param_is_invalid() {
                let raw_query = Query::from(vec![
                    (INFO_HASH, "%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0"),
                    (PEER_ID, "-RC3000-000000000001"),
                    (PORT, "17548"),
                    (UPLOADED, "INVALID_UPLOADED_VALUE"),
                ])
                .to_string();

                assert!(Announce::try_from(raw_query.parse::<Query>().unwrap()).is_err());
            }

            #[test]
            fn it_should_fail_if_the_left_param_is_invalid() {
                let raw_query = Query::from(vec![
                    (INFO_HASH, "%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0"),
                    (PEER_ID, "-RC3000-000000000001"),
                    (PORT, "17548"),
                    (LEFT, "INVALID_LEFT_VALUE"),
                ])
                .to_string();

                assert!(Announce::try_from(raw_query.parse::<Query>().unwrap()).is_err());
            }

            #[test]
            fn it_should_fail_if_the_event_param_is_invalid() {
                let raw_query = Query::from(vec![
                    (INFO_HASH, "%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0"),
                    (PEER_ID, "-RC3000-000000000001"),
                    (PORT, "17548"),
                    (EVENT, "INVALID_EVENT_VALUE"),
                ])
                .to_string();

                assert!(Announce::try_from(raw_query.parse::<Query>().unwrap()).is_err());
            }

            #[test]
            fn it_should_fail_if_the_compact_param_is_invalid() {
                let raw_query = Query::from(vec![
                    (INFO_HASH, "%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0"),
                    (PEER_ID, "-RC3000-000000000001"),
                    (PORT, "17548"),
                    (COMPACT, "INVALID_COMPACT_VALUE"),
                ])
                .to_string();

                assert!(Announce::try_from(raw_query.parse::<Query>().unwrap()).is_err());
            }

            #[test]
            fn it_should_fail_if_the_numwant_param_is_invalid() {
                let raw_query = Query::from(vec![
                    (INFO_HASH, "%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0"),
                    (PEER_ID, "-RC3000-000000000001"),
                    (PORT, "17548"),
                    (NUMWANT, "-1"),
                ])
                .to_string();

                assert!(Announce::try_from(raw_query.parse::<Query>().unwrap()).is_err());
            }
        }
    }
}
