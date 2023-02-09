use std::panic::Location;
use std::str::FromStr;

use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use thiserror::Error;

use super::query::Query;
use crate::http::percent_encoding::{percent_decode_info_hash, percent_decode_peer_id};
use crate::protocol::info_hash::{ConversionError, InfoHash};
use crate::tracker::peer::{self, IdConversionError};

pub struct ExtractAnnounceParams(pub AnnounceParams);

#[derive(Debug, PartialEq)]
pub struct AnnounceParams {
    pub info_hash: InfoHash,
    pub peer_id: peer::Id,
    pub port: u16,
}

#[derive(Error, Debug)]
pub enum ParseAnnounceQueryError {
    #[error("missing infohash {location}")]
    MissingInfoHash { location: &'static Location<'static> },
    #[error("invalid infohash {location}")]
    InvalidInfoHash { location: &'static Location<'static> },
    #[error("missing peer id {location}")]
    MissingPeerId { location: &'static Location<'static> },
    #[error("invalid peer id {location}")]
    InvalidPeerId { location: &'static Location<'static> },
    #[error("missing port {location}")]
    MissingPort { location: &'static Location<'static> },
    #[error("invalid port {location}")]
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
        Self::InvalidPeerId {
            location: Location::caller(),
        }
    }
}

impl TryFrom<Query> for AnnounceParams {
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
impl<S> FromRequestParts<S> for ExtractAnnounceParams
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let raw_query = parts.uri.query();

        if raw_query.is_none() {
            return Err((StatusCode::BAD_REQUEST, "missing query params"));
        }

        let query = raw_query.unwrap().parse::<Query>();

        if query.is_err() {
            return Err((StatusCode::BAD_REQUEST, "can't parse query params"));
        }

        let announce_params = AnnounceParams::try_from(query.unwrap());

        if announce_params.is_err() {
            return Err((StatusCode::BAD_REQUEST, "can't parse query params for announce request"));
        }

        Ok(ExtractAnnounceParams(announce_params.unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::AnnounceParams;
    use crate::http::axum_implementation::query::Query;
    use crate::protocol::info_hash::InfoHash;
    use crate::tracker::peer;

    #[test]
    fn announce_request_params_should_be_extracted_from_url_query_params() {
        let raw_query = "info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&peer_id=-qB00000000000000001&port=17548";

        let query = raw_query.parse::<Query>().unwrap();

        let announce_params = AnnounceParams::try_from(query).unwrap();

        assert_eq!(
            announce_params,
            AnnounceParams {
                info_hash: "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0".parse::<InfoHash>().unwrap(),
                peer_id: "-qB00000000000000001".parse::<peer::Id>().unwrap(),
                port: 17548,
            }
        );
    }
}
