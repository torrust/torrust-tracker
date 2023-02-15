use std::fmt;
use std::panic::Location;
use std::str::FromStr;

use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::http::axum_implementation::query::{ParseQueryError, Query};
use crate::http::axum_implementation::responses;
use crate::http::percent_encoding::{percent_decode_info_hash, percent_decode_peer_id};
use crate::located_error::{Located, LocatedError};
use crate::protocol::info_hash::{ConversionError, InfoHash};
use crate::tracker::peer::{self, IdConversionError};

pub type NumberOfBytes = i64;

pub struct ExtractAnnounceRequest(pub Announce);

// Param names in the URL query
const INFO_HASH: &str = "info_hash";
const PEER_ID: &str = "peer_id";
const PORT: &str = "port";
const DOWNLOADED: &str = "downloaded";
const UPLOADED: &str = "uploaded";
const LEFT: &str = "left";
const EVENT: &str = "event";
const COMPACT: &str = "compact";

#[derive(Debug, PartialEq)]
pub struct Announce {
    // Mandatory params
    pub info_hash: InfoHash,
    pub peer_id: peer::Id,
    pub port: u16,
    // Optional params
    pub downloaded: Option<NumberOfBytes>,
    pub uploaded: Option<NumberOfBytes>,
    pub left: Option<NumberOfBytes>,
    pub event: Option<Event>,
    pub compact: Option<Compact>,
}

#[derive(PartialEq, Debug)]
pub enum Event {
    Started,
    Stopped,
    Completed,
}

impl FromStr for Event {
    type Err = ParseAnnounceQueryError;

    fn from_str(raw_param: &str) -> Result<Self, Self::Err> {
        match raw_param {
            "started" => Ok(Self::Started),
            "stopped" => Ok(Self::Stopped),
            "completed" => Ok(Self::Completed),
            _ => Err(ParseAnnounceQueryError::InvalidParam {
                param_name: EVENT.to_owned(),
                param_value: raw_param.to_owned(),
                location: Location::caller(),
            }),
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Event::Started => write!(f, "started"),
            Event::Stopped => write!(f, "stopped"),
            Event::Completed => write!(f, "completed"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Compact {
    Accepted = 1,
    NotAccepted = 0,
}

impl fmt::Display for Compact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

#[derive(Error, Debug)]
pub enum ParseAnnounceQueryError {
    #[error("missing param {param_name} in {location}")]
    MissingParam {
        location: &'static Location<'static>,
        param_name: String,
    },
    #[error("invalid param value {param_value} for {param_name} in {location}")]
    InvalidParam {
        param_name: String,
        param_value: String,
        location: &'static Location<'static>,
    },
    #[error("param value overflow {param_value} for {param_name} in {location}")]
    NumberOfBytesOverflow {
        param_name: String,
        param_value: String,
        location: &'static Location<'static>,
    },
    #[error("invalid param value {param_value} for {param_name} in {source}")]
    InvalidInfoHashParam {
        param_name: String,
        param_value: String,
        source: LocatedError<'static, ConversionError>,
    },
    #[error("invalid param value {param_value} for {param_name} in {source}")]
    InvalidPeerIdParam {
        param_name: String,
        param_value: String,
        source: LocatedError<'static, IdConversionError>,
    },
}

impl From<ParseQueryError> for responses::error::Error {
    fn from(err: ParseQueryError) -> Self {
        responses::error::Error {
            failure_reason: format!("Cannot parse query params: {err}"),
        }
    }
}

impl From<ParseAnnounceQueryError> for responses::error::Error {
    fn from(err: ParseAnnounceQueryError) -> Self {
        responses::error::Error {
            failure_reason: format!("Cannot parse query params for announce request: {err}"),
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
        })
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
        None => {
            return Err(ParseAnnounceQueryError::MissingParam {
                location: Location::caller(),
                param_name: INFO_HASH.to_owned(),
            })
        }
    }
}

fn extract_peer_id(query: &Query) -> Result<peer::Id, ParseAnnounceQueryError> {
    match query.get_param(PEER_ID) {
        Some(raw_param) => Ok(
            percent_decode_peer_id(&raw_param).map_err(|err| ParseAnnounceQueryError::InvalidPeerIdParam {
                param_name: PEER_ID.to_owned(),
                param_value: raw_param.clone(),
                source: Located(err).into(),
            })?,
        ),
        None => {
            return Err(ParseAnnounceQueryError::MissingParam {
                location: Location::caller(),
                param_name: PEER_ID.to_owned(),
            })
        }
    }
}

fn extract_port(query: &Query) -> Result<u16, ParseAnnounceQueryError> {
    match query.get_param(PORT) {
        Some(raw_param) => Ok(u16::from_str(&raw_param).map_err(|_e| ParseAnnounceQueryError::InvalidParam {
            param_name: PORT.to_owned(),
            param_value: raw_param.clone(),
            location: Location::caller(),
        })?),
        None => {
            return Err(ParseAnnounceQueryError::MissingParam {
                location: Location::caller(),
                param_name: PORT.to_owned(),
            })
        }
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

            Ok(Some(i64::try_from(number_of_bytes).map_err(|_e| {
                ParseAnnounceQueryError::NumberOfBytesOverflow {
                    param_name: param_name.to_owned(),
                    param_value: raw_param.clone(),
                    location: Location::caller(),
                }
            })?))
        }
        None => Ok(None),
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

#[async_trait]
impl<S> FromRequestParts<S> for ExtractAnnounceRequest
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let raw_query = parts.uri.query();

        if raw_query.is_none() {
            return Err(responses::error::Error {
                failure_reason: "missing query params for announce request".to_string(),
            }
            .into_response());
        }

        let query = raw_query.unwrap().parse::<Query>();

        if let Err(error) = query {
            return Err(responses::error::Error::from(error).into_response());
        }

        let announce_request = Announce::try_from(query.unwrap());

        if let Err(error) = announce_request {
            return Err(responses::error::Error::from(error).into_response());
        }

        Ok(ExtractAnnounceRequest(announce_request.unwrap()))
    }
}

#[cfg(test)]
mod tests {

    mod announce_request {

        use crate::http::axum_implementation::query::Query;
        use crate::http::axum_implementation::requests::announce::{
            Announce, Compact, Event, COMPACT, DOWNLOADED, EVENT, INFO_HASH, LEFT, PEER_ID, PORT, UPLOADED,
        };
        use crate::protocol::info_hash::InfoHash;
        use crate::tracker::peer;

        #[test]
        fn should_be_instantiated_from_the_url_query_with_only_the_mandatory_params() {
            let raw_query = Query::from(vec![
                (INFO_HASH, "%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0"),
                (PEER_ID, "-qB00000000000000001"),
                (PORT, "17548"),
            ])
            .to_string();

            let query = raw_query.parse::<Query>().unwrap();

            let announce_request = Announce::try_from(query).unwrap();

            assert_eq!(
                announce_request,
                Announce {
                    info_hash: "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0".parse::<InfoHash>().unwrap(),
                    peer_id: "-qB00000000000000001".parse::<peer::Id>().unwrap(),
                    port: 17548,
                    downloaded: None,
                    uploaded: None,
                    left: None,
                    event: None,
                    compact: None,
                }
            );
        }

        #[test]
        fn should_be_instantiated_from_the_url_query_params() {
            let raw_query = Query::from(vec![
                (INFO_HASH, "%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0"),
                (PEER_ID, "-qB00000000000000001"),
                (PORT, "17548"),
                (DOWNLOADED, "1"),
                (UPLOADED, "2"),
                (LEFT, "3"),
                (EVENT, "started"),
                (COMPACT, "0"),
            ])
            .to_string();

            let query = raw_query.parse::<Query>().unwrap();

            let announce_request = Announce::try_from(query).unwrap();

            assert_eq!(
                announce_request,
                Announce {
                    info_hash: "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0".parse::<InfoHash>().unwrap(),
                    peer_id: "-qB00000000000000001".parse::<peer::Id>().unwrap(),
                    port: 17548,
                    downloaded: Some(1),
                    uploaded: Some(2),
                    left: Some(3),
                    event: Some(Event::Started),
                    compact: Some(Compact::NotAccepted),
                }
            );
        }

        mod when_it_is_instantiated_from_the_url_query_params {

            use crate::http::axum_implementation::query::Query;
            use crate::http::axum_implementation::requests::announce::{
                Announce, COMPACT, DOWNLOADED, EVENT, INFO_HASH, LEFT, PEER_ID, PORT, UPLOADED,
            };

            #[test]
            fn it_should_fail_if_the_query_does_not_include_all_the_mandatory_params() {
                let raw_query_without_info_hash = "peer_id=-qB00000000000000001&port=17548";

                assert!(Announce::try_from(raw_query_without_info_hash.parse::<Query>().unwrap()).is_err());

                let raw_query_without_peer_id = "info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&port=17548";

                assert!(Announce::try_from(raw_query_without_peer_id.parse::<Query>().unwrap()).is_err());

                let raw_query_without_port =
                    "info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&peer_id=-qB00000000000000001";

                assert!(Announce::try_from(raw_query_without_port.parse::<Query>().unwrap()).is_err());
            }

            #[test]
            fn it_should_fail_if_the_info_hash_param_is_invalid() {
                let raw_query = Query::from(vec![
                    (INFO_HASH, "INVALID_INFO_HASH_VALUE"),
                    (PEER_ID, "-qB00000000000000001"),
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
                    (PEER_ID, "-qB00000000000000001"),
                    (PORT, "INVALID_PORT_VALUE"),
                ])
                .to_string();

                assert!(Announce::try_from(raw_query.parse::<Query>().unwrap()).is_err());
            }

            #[test]
            fn it_should_fail_if_the_downloaded_param_is_invalid() {
                let raw_query = Query::from(vec![
                    (INFO_HASH, "%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0"),
                    (PEER_ID, "-qB00000000000000001"),
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
                    (PEER_ID, "-qB00000000000000001"),
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
                    (PEER_ID, "-qB00000000000000001"),
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
                    (PEER_ID, "-qB00000000000000001"),
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
                    (PEER_ID, "-qB00000000000000001"),
                    (PORT, "17548"),
                    (COMPACT, "INVALID_COMPACT_VALUE"),
                ])
                .to_string();

                assert!(Announce::try_from(raw_query.parse::<Query>().unwrap()).is_err());
            }
        }
    }
}
