//! Core tracker errors.
//!
//! This module defines the error types used internally by the `BitTorrent`
//! tracker core.
//!
//! These errors encapsulate issues such as whitelisting violations, invalid
//! peer key data, and database persistence failures. Each error variant
//! includes contextual information (such as source code location) to facilitate
//!  debugging.
use std::panic::Location;

use bittorrent_primitives::info_hash::InfoHash;
use torrust_tracker_located_error::LocatedError;

use super::authentication::key::ParseKeyError;
use super::databases;
use crate::authentication;

/// Wrapper for all errors returned by the tracker core.
#[derive(thiserror::Error, Debug, Clone)]
pub enum TrackerCoreError {
    /// Error returned when there was an error with the tracker core announce handler.
    #[error("Tracker core announce error: {source}")]
    AnnounceError { source: AnnounceError },

    /// Error returned when there was an error with the tracker core scrape handler.
    #[error("Tracker core scrape error: {source}")]
    ScrapeError { source: ScrapeError },

    /// Error returned when there was an error with the tracker core whitelist.
    #[error("Tracker core whitelist error: {source}")]
    WhitelistError { source: WhitelistError },

    /// Error returned when there was an error with the authentication in the tracker core.
    #[error("Tracker core authentication error: {source}")]
    AuthenticationError { source: authentication::key::Error },
}

impl From<AnnounceError> for TrackerCoreError {
    fn from(announce_error: AnnounceError) -> Self {
        Self::AnnounceError { source: announce_error }
    }
}

impl From<ScrapeError> for TrackerCoreError {
    fn from(scrape_error: ScrapeError) -> Self {
        Self::ScrapeError { source: scrape_error }
    }
}

impl From<WhitelistError> for TrackerCoreError {
    fn from(whitelist_error: WhitelistError) -> Self {
        Self::WhitelistError { source: whitelist_error }
    }
}

impl From<authentication::key::Error> for TrackerCoreError {
    fn from(whitelist_error: authentication::key::Error) -> Self {
        Self::AuthenticationError { source: whitelist_error }
    }
}

/// Errors related to announce requests.
#[derive(thiserror::Error, Debug, Clone)]
pub enum AnnounceError {
    /// Wraps errors related to torrent whitelisting.
    #[error("Whitelist error: {0}")]
    Whitelist(#[from] WhitelistError),

    /// Wraps errors related to database.
    #[error("Database error: {0}")]
    Database(#[from] databases::error::Error),
}

/// Errors related to scrape requests.
#[derive(thiserror::Error, Debug, Clone)]
pub enum ScrapeError {
    /// Wraps errors related to torrent whitelisting.
    #[error("Whitelist error: {0}")]
    Whitelist(#[from] WhitelistError),
}

/// Errors related to torrent whitelisting.
///
/// This error is returned when an operation involves a torrent that is not
/// present in the whitelist.
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum WhitelistError {
    /// Indicates that the torrent identified by `info_hash` is not whitelisted.
    #[error("The torrent: {info_hash}, is not whitelisted, {location}")]
    TorrentNotWhitelisted {
        info_hash: InfoHash,
        location: &'static Location<'static>,
    },
}

/// Errors related to peer key operations.
///
/// This error type covers issues encountered during the handling of peer keys,
/// including validation of key durations, parsing errors, and database
/// persistence problems.
#[allow(clippy::module_name_repetitions)]
#[derive(thiserror::Error, Debug, Clone)]
pub enum PeerKeyError {
    /// Returned when the duration specified for the peer key exceeds the
    /// maximum.
    #[error("Invalid peer key duration: {seconds_valid:?}, is not valid")]
    DurationOverflow { seconds_valid: u64 },

    /// Returned when the provided peer key is invalid.
    #[error("Invalid key: {key}")]
    InvalidKey {
        key: String,
        source: LocatedError<'static, ParseKeyError>,
    },

    /// Returned when persisting the peer key to the database fails.
    #[error("Can't persist key: {source}")]
    DatabaseError {
        source: LocatedError<'static, databases::error::Error>,
    },
}

#[cfg(test)]
mod tests {

    mod whitelist_error {

        use crate::error::WhitelistError;
        use crate::test_helpers::tests::sample_info_hash;

        #[test]
        fn torrent_not_whitelisted() {
            let err = WhitelistError::TorrentNotWhitelisted {
                info_hash: sample_info_hash(),
                location: std::panic::Location::caller(),
            };

            let err_msg = format!("{err}");

            assert!(
                err_msg.contains(&format!("The torrent: {}, is not whitelisted", sample_info_hash())),
                "Error message did not contain expected text: {err_msg}"
            );
        }
    }

    mod peer_key_error {
        use torrust_tracker_located_error::Located;

        use crate::databases::driver::Driver;
        use crate::error::PeerKeyError;
        use crate::{authentication, databases};

        #[test]
        fn duration_overflow() {
            let seconds_valid = 100;

            let err = PeerKeyError::DurationOverflow { seconds_valid };

            let err_msg = format!("{err}");

            assert!(
                err_msg.contains(&format!("Invalid peer key duration: {seconds_valid}")),
                "Error message did not contain expected text: {err_msg}"
            );
        }

        #[test]
        fn parsing_from_string() {
            let err = authentication::key::ParseKeyError::InvalidKeyLength;

            let err = PeerKeyError::InvalidKey {
                key: "INVALID KEY".to_string(),
                source: Located(err).into(),
            };

            let err_msg = format!("{err}");

            assert!(
                err_msg.contains(&"Invalid key: INVALID KEY".to_string()),
                "Error message did not contain expected text: {err_msg}"
            );
        }

        #[test]
        fn persisting_into_database() {
            let err = databases::error::Error::InsertFailed {
                location: std::panic::Location::caller(),
                driver: Driver::Sqlite3,
            };

            let err = PeerKeyError::DatabaseError {
                source: Located(err).into(),
            };

            let err_msg = format!("{err}");

            assert!(
                err_msg.contains(&"Can't persist key".to_string()),
                "Error message did not contain expected text: {err}"
            );
        }
    }
}
