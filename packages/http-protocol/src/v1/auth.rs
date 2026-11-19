use thiserror::Error;

/// Authentication error.
///
/// When the tracker is private, the authentication key is required in the URL
/// path. These are the possible errors that can occur when extracting the key
/// from the URL path.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// The key in the URL path is not a valid authentication key. The key is
    /// a secret, so the message names the expected format, not the key.
    #[error("Invalid format for authentication key param: expected 32 alphanumeric characters")]
    InvalidKeyFormat,

    /// The URL path has no key segment.
    #[error("Cannot extract authentication key param from URL path: expected /announce/<key> or /scrape/<key>")]
    CannotExtractKeyParam,
}
