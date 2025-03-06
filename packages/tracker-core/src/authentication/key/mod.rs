//! Tracker authentication services and types.
//!
//! This module provides functions and data structures for handling tracker keys.
//! Tracker keys are tokens used to authenticate tracker clients when the
//! tracker is running in `private` mode.
//!
//! Authentication keys are used exclusively by HTTP trackers. Every key has an
//! expiration time, meaning that it is only valid for a predetermined period.
//! Once the expiration time is reached, an expiring key will be rejected.
//!
//! The primary key structure is [`PeerKey`], which couples a randomly generated
//!  [`Key`] (a 32-character alphanumeric string) with an optional expiration
//! timestamp.
//!
//! # Examples
//!
//! Generating a new key valid for `9999` seconds:
//!
//! ```rust
//! use bittorrent_tracker_core::authentication;
//! use std::time::Duration;
//!
//! let expiring_key = authentication::key::generate_key(Some(Duration::new(9999, 0)));
//!
//! // Later, verify that the key is still valid.
//! assert!(authentication::key::verify_key_expiration(&expiring_key).is_ok());
//! ```
//!
//! The core key types are defined as follows:
//!
//! ```rust
//! use bittorrent_tracker_core::authentication::Key;
//! use torrust_tracker_primitives::DurationSinceUnixEpoch;
//!
//! pub struct PeerKey {
//!     /// A random 32-character authentication token (e.g., `YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ`)
//!     pub key: Key,
//!
//!     /// The timestamp after which the key expires. If `None`, the key is permanent.
//!     pub valid_until: Option<DurationSinceUnixEpoch>,
//! }
//! ```
pub mod peer_key;
pub mod repository;

use std::panic::Location;
use std::sync::Arc;
use std::time::Duration;

use thiserror::Error;
use torrust_tracker_clock::clock::Time;
use torrust_tracker_located_error::{DynError, LocatedError};
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::CurrentClock;

pub type PeerKey = peer_key::PeerKey;
pub type Key = peer_key::Key;
pub type ParseKeyError = peer_key::ParseKeyError;

/// HTTP tracker authentication key length.
///
/// For more information see function [`generate_key`](crate::authentication::key::generate_key) to generate the
/// [`PeerKey`](crate::authentication::PeerKey).
pub(crate) const AUTH_KEY_LENGTH: usize = 32;

/// It generates a new permanent random key [`PeerKey`].
#[cfg(test)]
#[must_use]
pub(crate) fn generate_permanent_key() -> PeerKey {
    generate_key(None)
}

/// It generates a new expiring random key [`PeerKey`].
#[cfg(test)]
#[must_use]
pub(crate) fn generate_expiring_key(lifetime: Duration) -> PeerKey {
    generate_key(Some(lifetime))
}

/// Generates a new random 32-character authentication key (`PeerKey`).
///
/// If a lifetime is provided, the generated key will expire after the specified
///  duration; otherwise, the key is permanent (i.e., it never expires).
///
/// # Panics
///
/// Panics if the addition of the lifetime to the current time overflows
/// (an extremely unlikely event).
///
/// # Arguments
///
/// * `lifetime`: An optional duration specifying how long the key is valid.
///   If `None`, the key is permanent.
///
/// # Examples
///
/// ```rust
/// use bittorrent_tracker_core::authentication::key;
/// use std::time::Duration;
///
/// // Generate an expiring key valid for 3600 seconds.
/// let expiring_key = key::generate_key(Some(Duration::from_secs(3600)));
///
/// // Generate a permanent key.
/// let permanent_key = key::generate_key(None);
/// ```
#[must_use]
pub fn generate_key(lifetime: Option<Duration>) -> PeerKey {
    let random_key = Key::random();

    if let Some(lifetime) = lifetime {
        tracing::debug!("Generated key: {}, valid for: {:?} seconds", random_key, lifetime);

        PeerKey {
            key: random_key,
            valid_until: Some(CurrentClock::now_add(&lifetime).unwrap()),
        }
    } else {
        tracing::debug!("Generated key: {}, permanent", random_key);

        PeerKey {
            key: random_key,
            valid_until: None,
        }
    }
}

/// Verifies whether a given authentication key (`PeerKey`) is still valid.
///
/// For expiring keys, this function compares the key's expiration timestamp
/// against the current time. Permanent keys (with `None` as their expiration)
/// are always valid.
///
/// # Errors
///
/// Returns a verification error of type [`enum@Error`] if the key has expired.
///
/// # Examples
///
/// ```rust
/// use bittorrent_tracker_core::authentication::key;
/// use std::time::Duration;
///
/// let expiring_key = key::generate_key(Some(Duration::from_secs(100)));
///
/// // If the key's expiration time has passed, the verification will fail.
/// assert!(key::verify_key_expiration(&expiring_key).is_ok());
/// ```
pub fn verify_key_expiration(auth_key: &PeerKey) -> Result<(), Error> {
    let current_time: DurationSinceUnixEpoch = CurrentClock::now();

    match auth_key.valid_until {
        Some(valid_until) => {
            if valid_until < current_time {
                Err(Error::KeyExpired {
                    location: Location::caller(),
                })
            } else {
                Ok(())
            }
        }
        None => Ok(()), // Permanent key
    }
}

/// Verification error. Error returned when an [`PeerKey`] cannot be
/// verified with the [`crate::authentication::key::verify_key_expiration`] function.
#[derive(Debug, Error, Clone)]
#[allow(dead_code)]
pub enum Error {
    /// Wraps an underlying error encountered during key verification.
    #[error("Key could not be verified: {source}")]
    KeyVerificationError {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    /// Indicates that the key could not be read or found.
    #[error("Failed to read key: {key}, {location}")]
    UnableToReadKey {
        location: &'static Location<'static>,
        key: Box<Key>,
    },

    /// Indicates that the key has expired.
    #[error("Key has expired, {location}")]
    KeyExpired { location: &'static Location<'static> },

    /// Indicates that the required key for authentication was not provided.
    #[error("Missing authentication key, {location}")]
    MissingAuthKey { location: &'static Location<'static> },
}

impl From<r2d2_sqlite::rusqlite::Error> for Error {
    fn from(e: r2d2_sqlite::rusqlite::Error) -> Self {
        Error::KeyVerificationError {
            source: (Arc::new(e) as DynError).into(),
        }
    }
}

#[cfg(test)]
mod tests {

    mod the_expiring_peer_key {

        use std::time::Duration;

        use torrust_tracker_clock::clock;
        use torrust_tracker_clock::clock::stopped::Stopped as _;

        use crate::authentication;

        #[test]
        fn should_be_displayed() {
            // Set the time to the current time.
            clock::Stopped::local_set_to_unix_epoch();

            let expiring_key = authentication::key::generate_key(Some(Duration::from_secs(0)));

            assert_eq!(
                expiring_key.to_string(),
                format!("key: `{}`, valid until `1970-01-01 00:00:00 UTC`", expiring_key.key) // cspell:disable-line
            );
        }

        #[test]
        fn should_be_generated_with_a_expiration_time() {
            let expiring_key = authentication::key::generate_key(Some(Duration::new(9999, 0)));

            assert!(authentication::key::verify_key_expiration(&expiring_key).is_ok());
        }

        #[test]
        fn expiration_verification_should_fail_when_the_key_has_expired() {
            // Set the time to the current time.
            clock::Stopped::local_set_to_system_time_now();

            // Make key that is valid for 19 seconds.
            let expiring_key = authentication::key::generate_key(Some(Duration::from_secs(19)));

            // Mock the time has passed 10 sec.
            clock::Stopped::local_add(&Duration::from_secs(10)).unwrap();

            assert!(authentication::key::verify_key_expiration(&expiring_key).is_ok());

            // Mock the time has passed another 10 sec.
            clock::Stopped::local_add(&Duration::from_secs(10)).unwrap();

            assert!(authentication::key::verify_key_expiration(&expiring_key).is_err());
        }
    }

    mod the_permanent_peer_key {

        use std::time::Duration;

        use torrust_tracker_clock::clock;
        use torrust_tracker_clock::clock::stopped::Stopped as _;

        use crate::authentication;

        #[test]
        fn should_be_displayed() {
            // Set the time to the current time.
            clock::Stopped::local_set_to_unix_epoch();

            let expiring_key = authentication::key::generate_key(Some(Duration::from_secs(0)));

            assert_eq!(
                expiring_key.to_string(),
                format!("key: `{}`, valid until `1970-01-01 00:00:00 UTC`", expiring_key.key) // cspell:disable-line
            );
        }

        #[test]
        fn should_be_generated_without_expiration_time() {
            let expiring_key = authentication::key::generate_permanent_key();

            assert!(authentication::key::verify_key_expiration(&expiring_key).is_ok());
        }

        #[test]
        fn expiration_verification_should_always_succeed() {
            let expiring_key = authentication::key::generate_permanent_key();

            // Mock the time has passed 10 years.
            clock::Stopped::local_add(&Duration::from_secs(10 * 365 * 24 * 60 * 60)).unwrap();

            assert!(authentication::key::verify_key_expiration(&expiring_key).is_ok());
        }
    }

    mod the_key_verification_error {
        use crate::authentication::key;

        #[test]
        fn could_be_a_database_error() {
            let err = r2d2_sqlite::rusqlite::Error::InvalidQuery;

            let err: key::Error = err.into();

            assert!(matches!(err, key::Error::KeyVerificationError { .. }));
        }
    }
}
