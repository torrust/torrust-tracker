use std::panic::Location;

use thiserror::Error;

/// Authentication error.
///
/// When the tracker is private, the authentication key is required in the URL
/// path. These are the possible errors that can occur when extracting the key
/// from the URL path.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid format for authentication key param. Error in {location}")]
    InvalidKeyFormat { location: &'static Location<'static> },

    #[error("Cannot extract authentication key param from URL path. Error in {location}")]
    CannotExtractKeyParam { location: &'static Location<'static> },
}
