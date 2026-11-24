use thiserror::Error;

/// Authentication error.
///
/// When the tracker is private, the authentication key is required in the URL
/// path. These are the possible errors that can occur when extracting the key
/// from the URL path.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// The key in the URL path does not have 32 characters. The key is a
    /// secret, so the message states how many characters were received, not
    /// the key.
    #[error("Invalid authentication key: expected 32 characters, got {length}")]
    InvalidKeyLength {
        /// The number of characters of the rejected key.
        length: usize,
    },

    /// The key in the URL path has a character outside `0-9`, `a-z` and
    /// `A-Z`. The message states the rule, not the key.
    #[error("Invalid authentication key: only the characters 0-9, a-z and A-Z are allowed")]
    InvalidKeyChars,

    /// The URL path has no key segment.
    #[error("Cannot extract authentication key param from URL path: expected /announce/<key> or /scrape/<key>")]
    CannotExtractKeyParam,
}
