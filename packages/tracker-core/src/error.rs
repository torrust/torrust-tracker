//! Core tracker errors.
//!
//! This module defines the error types used internally by the `BitTorrent`
//! tracker core.
//!
//! These errors encapsulate issues such as whitelisting violations, invalid
//! peer key data, and database persistence failures. Each error variant names
//! what failed and the offending input; none records a source location (see
//! `docs/adrs/20261118045443_errors_do_not_record_source_locations.md`).

use torrust_info_hash::InfoHash;

use super::authentication::key::ParseKeyError;
use super::databases;
use crate::authentication;

/// Wrapper for all errors returned by the tracker core.
///
/// This internal composition type is not an event payload: it can expose
/// implementation details and context that are unsuitable for a stable event
/// API. See the [general error-events
/// EPIC](../../../docs/issues/drafts/generalize-error-events/EPIC.md) before adding
/// error events derived from it.
#[derive(thiserror::Error, Debug, Clone)]
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
pub enum ScrapeError {
    /// Wraps errors related to torrent whitelisting.
    #[error("Whitelist error: {0}")]
    Whitelist(#[from] WhitelistError),

    /// Wraps errors related to database.
    #[error("Database error: {0}")]
    Database(#[from] databases::error::Error),
}

/// Errors related to torrent whitelisting.
///
/// This error is returned when an operation involves a torrent that is not
/// present in the whitelist.
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WhitelistError {
    /// Indicates that the torrent identified by `info_hash` is not whitelisted.
    #[error("The torrent: {info_hash}, is not whitelisted")]
    TorrentNotWhitelisted { info_hash: InfoHash },
}

/// Errors related to peer key operations.
///
/// This error type covers issues encountered during the handling of peer keys,
/// including validation of key durations, parsing errors, and database
/// persistence problems.
#[allow(
    clippy::module_name_repetitions,
    reason = "error type identifies the peer-key domain among tracker-core errors"
)]
#[derive(thiserror::Error, Debug, Clone)]
#[non_exhaustive]
pub enum PeerKeyError {
    /// Returned when the duration specified for the peer key exceeds the
    /// maximum.
    #[error("Invalid peer key duration: {seconds_valid:?}, is not valid")]
    DurationOverflow { seconds_valid: u64 },

    /// Returned when the provided peer key is invalid. It keeps the parse
    /// error, which states what is wrong, and never the key: a malformed key
    /// is still the user's secret.
    #[error("Invalid authentication key: {source}")]
    InvalidKey { source: ParseKeyError },

    /// Returned when persisting the peer key to the database fails.
    #[error("Can't persist key: {source}")]
    DatabaseError { source: databases::error::Error },
}

impl From<authentication::key::DurationOverflow> for PeerKeyError {
    fn from(overflow: authentication::key::DurationOverflow) -> Self {
        Self::DurationOverflow {
            seconds_valid: overflow.seconds_valid,
        }
    }
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
            };

            let err_msg = format!("{err}");

            assert_eq!(err_msg, format!("The torrent: {}, is not whitelisted", sample_info_hash()));
        }
    }

    mod peer_key_error {
        use torrust_tracker_primitives::Driver;

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
            let err = authentication::key::ParseKeyError::InvalidKeyLength { length: 14 };

            let err = PeerKeyError::InvalidKey { source: err };

            assert_eq!(err.to_string(), "Invalid authentication key: expected 32 characters, got 14");
        }

        #[test]
        fn persisting_into_database() {
            let err = databases::error::Error::InsertFailed { driver: Driver::Sqlite3 };

            let err = PeerKeyError::DatabaseError { source: err };

            let err_msg = format!("{err}");

            assert!(
                err_msg.contains(&"Can't persist key".to_string()),
                "Error message did not contain expected text: {err}"
            );
        }
    }

    mod tracker_core_error {
        use rstest::rstest;
        use torrust_tracker_primitives::Driver;

        use crate::error::{AnnounceError, ScrapeError, TrackerCoreError, WhitelistError};
        use crate::test_helpers::tests::sample_info_hash;
        use crate::{authentication, databases};

        fn a_database_error() -> databases::error::Error {
            databases::error::Error::InsertFailed { driver: Driver::Sqlite3 }
        }

        fn a_whitelist_error() -> WhitelistError {
            WhitelistError::TorrentNotWhitelisted {
                info_hash: sample_info_hash(),
            }
        }

        /// The name of the variant of `error`, to compare a conversion with a plain string.
        fn variant(error: &TrackerCoreError) -> &'static str {
            match error {
                TrackerCoreError::AnnounceError { .. } => "AnnounceError",
                TrackerCoreError::ScrapeError { .. } => "ScrapeError",
                TrackerCoreError::WhitelistError { .. } => "WhitelistError",
                TrackerCoreError::AuthenticationError { .. } => "AuthenticationError",
            }
        }

        #[rstest]
        #[case::announce(
            TrackerCoreError::from(AnnounceError::Database(a_database_error())),
            "AnnounceError",
            "Tracker core announce error: Database error: "
        )]
        #[case::scrape(
            TrackerCoreError::from(ScrapeError::Whitelist(a_whitelist_error())),
            "ScrapeError",
            "Tracker core scrape error: Whitelist error: "
        )]
        #[case::whitelist(
            TrackerCoreError::from(a_whitelist_error()),
            "WhitelistError",
            "Tracker core whitelist error: The torrent: "
        )]
        #[case::authentication(
            TrackerCoreError::from(authentication::key::Error::KeyExpired),
            "AuthenticationError",
            "Tracker core authentication error: Authentication key expired"
        )]
        fn it_should_wrap_each_error_in_its_own_variant_and_name_its_area(
            #[case] error: TrackerCoreError,
            #[case] expected_variant: &str,
            #[case] expected_message_start: &str,
        ) {
            let message = error.to_string();

            assert_eq!(variant(&error), expected_variant);
            assert!(
                message.starts_with(expected_message_start),
                "the message should start with {expected_message_start:?}, got: {message}"
            );
        }
    }
}
