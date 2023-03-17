use std::panic::Location;

use thiserror::Error;
use torrust_tracker_located_error::LocatedError;

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

    #[error("connection id could not be verified")]
    InvalidConnectionId { location: &'static Location<'static> },

    #[error("bad request: {source}")]
    BadRequest {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },
}
