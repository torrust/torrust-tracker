use std::panic::Location;

use thiserror::Error;
use warp::reject::Reject;

use crate::located_error::LocatedError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("tracker server error: {source}")]
    TrackerError {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    #[error("internal server error: {message}, {location}")]
    InternalServer {
        location: &'static Location<'static>,
        message: String,
    },

    #[error("no valid infohashes found, {location}")]
    EmptyInfoHash { location: &'static Location<'static> },

    #[error("peer_id is either missing or invalid, {location}")]
    InvalidPeerId { location: &'static Location<'static> },

    #[error("could not find remote address: {message}, {location}")]
    AddressNotFound {
        location: &'static Location<'static>,
        message: String,
    },

    #[error("too many infohashes: {message}, {location}")]
    TwoManyInfoHashes {
        location: &'static Location<'static>,
        message: String,
    },
}

impl Reject for Error {}
