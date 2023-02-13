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
use crate::protocol::info_hash::{ConversionError, InfoHash};
use crate::tracker::peer::{self, IdConversionError};

pub struct ExtractAnnounceRequest(pub Announce);

#[derive(Debug, PartialEq)]
pub struct Announce {
    pub info_hash: InfoHash,
    pub peer_id: peer::Id,
    pub port: u16,
}

#[derive(Error, Debug)]
pub enum ParseAnnounceQueryError {
    #[error("missing info_hash param: {location}")]
    MissingInfoHash { location: &'static Location<'static> },
    #[error("invalid info_hash param: {location}")]
    InvalidInfoHash { location: &'static Location<'static> },
    #[error("missing peer_id param: {location}")]
    MissingPeerId { location: &'static Location<'static> },
    #[error("invalid peer_id param: {location}")]
    InvalidPeerId { location: &'static Location<'static> },
    #[error("missing port param: {location}")]
    MissingPort { location: &'static Location<'static> },
    #[error("invalid port param: {location}")]
    InvalidPort { location: &'static Location<'static> },
}

impl From<IdConversionError> for ParseAnnounceQueryError {
    #[track_caller]
    fn from(_err: IdConversionError) -> Self {
        Self::InvalidPeerId {
            location: Location::caller(),
        }
    }
}

impl From<ConversionError> for ParseAnnounceQueryError {
    #[track_caller]
    fn from(_err: ConversionError) -> Self {
        Self::InvalidInfoHash {
            location: Location::caller(),
        }
    }
}

impl From<ParseQueryError> for responses::error::Error {
    fn from(err: ParseQueryError) -> Self {
        responses::error::Error {
            // code-review: should we expose error location in public HTTP tracker API?
            // Error message example: "Cannot parse query params: invalid param a=b=c in src/http/axum_implementation/query.rs:50:27"
            failure_reason: format!("Cannot parse query params: {err}"),
        }
    }
}

impl From<ParseAnnounceQueryError> for responses::error::Error {
    fn from(err: ParseAnnounceQueryError) -> Self {
        responses::error::Error {
            // code-review: should we expose error location in public HTTP tracker API?
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
        })
    }
}

fn extract_info_hash(query: &Query) -> Result<InfoHash, ParseAnnounceQueryError> {
    match query.get_param("info_hash") {
        Some(raw_info_hash) => Ok(percent_decode_info_hash(&raw_info_hash)?),
        None => {
            return Err(ParseAnnounceQueryError::MissingInfoHash {
                location: Location::caller(),
            })
        }
    }
}

fn extract_peer_id(query: &Query) -> Result<peer::Id, ParseAnnounceQueryError> {
    match query.get_param("peer_id") {
        Some(raw_peer_id) => Ok(percent_decode_peer_id(&raw_peer_id)?),
        None => {
            return Err(ParseAnnounceQueryError::MissingPeerId {
                location: Location::caller(),
            })
        }
    }
}

fn extract_port(query: &Query) -> Result<u16, ParseAnnounceQueryError> {
    match query.get_param("port") {
        Some(raw_port) => Ok(u16::from_str(&raw_port).map_err(|_e| ParseAnnounceQueryError::InvalidPort {
            location: Location::caller(),
        })?),
        None => {
            return Err(ParseAnnounceQueryError::MissingPort {
                location: Location::caller(),
            })
        }
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
    use super::Announce;
    use crate::http::axum_implementation::query::Query;
    use crate::protocol::info_hash::InfoHash;
    use crate::tracker::peer;

    #[test]
    fn announce_request_should_be_extracted_from_url_query_params() {
        let raw_query = "info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&peer_id=-qB00000000000000001&port=17548";

        let query = raw_query.parse::<Query>().unwrap();

        let announce_request = Announce::try_from(query).unwrap();

        assert_eq!(
            announce_request,
            Announce {
                info_hash: "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0".parse::<InfoHash>().unwrap(),
                peer_id: "-qB00000000000000001".parse::<peer::Id>().unwrap(),
                port: 17548,
            }
        );
    }
}
