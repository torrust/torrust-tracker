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
//! use torrust_tracker_core::authentication;
//! use std::time::Duration;
//!
//! let expiring_key = authentication::key::generate_key(Some(Duration::new(9999, 0))).unwrap();
//!
//! // Later, verify that the key is still valid.
//! assert!(authentication::key::verify_key_expiration(&expiring_key).is_ok());
//! ```
//!
//! The core key types are defined as follows:
//!
//! ```rust
//! use torrust_tracker_core::authentication::Key;
//! use torrust_clock::DurationSinceUnixEpoch;
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

use std::sync::Arc;
use std::time::Duration;

use thiserror::Error;
use torrust_clock::DurationSinceUnixEpoch;
use torrust_clock::clock::Time;

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
    generate_key(None).expect("a permanent key has no lifetime to overflow")
}

/// It generates a new expiring random key [`PeerKey`].
#[cfg(test)]
#[must_use]
pub(crate) fn generate_expiring_key(lifetime: Duration) -> PeerKey {
    generate_key(Some(lifetime)).expect("the lifetime of a test key must not overflow the clock")
}

/// Generates a new random 32-character authentication key (`PeerKey`).
///
/// If a lifetime is provided, the generated key will expire after the specified
///  duration; otherwise, the key is permanent (i.e., it never expires).
///
/// # Errors
///
/// Returns [`DurationOverflow`] if adding the lifetime to the current time
/// overflows the clock. A permanent key (`None`) never fails.
///
/// # Arguments
///
/// * `lifetime`: An optional duration specifying how long the key is valid.
///   If `None`, the key is permanent.
///
/// # Examples
///
/// ```rust
/// use torrust_tracker_core::authentication::key;
/// use std::time::Duration;
///
/// // Generate an expiring key valid for 3600 seconds.
/// let expiring_key = key::generate_key(Some(Duration::from_secs(3600))).unwrap();
///
/// // Generate a permanent key.
/// let permanent_key = key::generate_key(None).unwrap();
///
/// // A lifetime that overflows the clock is an error, not a panic.
/// assert!(key::generate_key(Some(Duration::MAX)).is_err());
/// ```
pub fn generate_key(lifetime: Option<Duration>) -> Result<PeerKey, DurationOverflow> {
    let valid_until = match lifetime {
        Some(lifetime) => Some(CurrentClock::now_add(&lifetime).ok_or(DurationOverflow {
            seconds_valid: lifetime.as_secs(),
        })?),
        None => None,
    };

    // The key is a secret: nothing logs it, so the log says nothing about the generation.
    Ok(PeerKey {
        key: Key::random(),
        valid_until,
    })
}

/// Returned by [`generate_key`] when the lifetime added to the current time
/// overflows the clock.
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
#[error("Invalid peer key duration: {seconds_valid:?}, is not valid")]
pub struct DurationOverflow {
    /// The requested lifetime, in seconds.
    pub seconds_valid: u64,
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
/// use torrust_tracker_core::authentication::key;
/// use std::time::Duration;
///
/// let expiring_key = key::generate_key(Some(Duration::from_secs(100))).unwrap();
///
/// // If the key's expiration time has passed, the verification will fail.
/// assert!(key::verify_key_expiration(&expiring_key).is_ok());
/// ```
pub fn verify_key_expiration(auth_key: &PeerKey) -> Result<(), Error> {
    let current_time: DurationSinceUnixEpoch = CurrentClock::now();

    auth_key.valid_until.map_or_else(
        || Ok(()),
        |valid_until| {
            if valid_until < current_time {
                Err(Error::KeyExpired)
            } else {
                Ok(())
            }
        },
    )
}

/// Verification error. Error returned when an [`PeerKey`] cannot be
/// verified with the [`crate::authentication::key::verify_key_expiration`] function.
#[derive(Debug, Error, Clone)]
#[allow(dead_code)]
#[non_exhaustive]
pub enum Error {
    /// Wraps an underlying error encountered during key verification.
    #[error("Key could not be verified: {source}")]
    KeyVerificationError {
        source: Arc<dyn std::error::Error + Send + Sync>,
    },

    /// Indicates that the key is not registered. The error does not keep the
    /// key: it is the client's secret.
    #[error("Authentication key not registered")]
    UnableToReadKey,

    /// Indicates that the key has expired.
    #[error("Authentication key expired")]
    KeyExpired,

    /// Indicates that the required key for authentication was not provided.
    #[error("Missing authentication key: this tracker is private, use /announce/<key>")]
    MissingAuthKey,
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::KeyVerificationError { source: Arc::new(e) }
    }
}

#[cfg(test)]
mod tests {

    mod when_a_key_is_generated {
        use std::time::Duration;

        use torrust_tracker_test_helpers::logging::capture_logs_while;

        use crate::authentication::key::generate_key;

        #[test]
        fn it_should_not_write_an_expiring_key_to_the_log() {
            let (key, logs) = capture_logs_while(|| generate_key(Some(Duration::from_secs(60))).unwrap());

            assert!(!logs.contains(key.key.value()), "the log holds the generated key");
        }

        #[test]
        fn it_should_not_write_a_permanent_key_to_the_log() {
            let (key, logs) = capture_logs_while(|| generate_key(None).unwrap());

            assert!(!logs.contains(key.key.value()), "the log holds the generated key");
        }
    }

    mod when_the_lifetime_of_the_key_overflows_the_clock {
        use std::time::Duration;

        use crate::authentication::key::{DurationOverflow, generate_key};

        #[test]
        fn it_should_return_an_error_with_the_requested_seconds_instead_of_panicking() {
            let result = generate_key(Some(Duration::from_secs(u64::MAX)));

            assert_eq!(result.unwrap_err(), DurationOverflow { seconds_valid: u64::MAX });
        }

        #[test]
        fn it_should_state_the_requested_seconds_in_the_error_message() {
            let error = DurationOverflow { seconds_valid: 100 };

            assert_eq!(error.to_string(), "Invalid peer key duration: 100, is not valid");
        }
    }

    mod the_authentication_error {
        use rstest::rstest;

        use crate::authentication::key::Error;

        #[rstest]
        #[case::missing(
            Error::MissingAuthKey,
            "Missing authentication key: this tracker is private, use /announce/<key>"
        )]
        #[case::expired(Error::KeyExpired, "Authentication key expired")]
        fn it_should_tell_the_user_which_case_failed(#[case] error: Error, #[case] expected_message: &str) {
            assert_eq!(error.to_string(), expected_message);
        }
    }

    mod the_expiring_peer_key {

        use std::time::Duration;

        use torrust_clock::clock;
        use torrust_clock::clock::stopped::Stopped as _;

        use crate::authentication;

        #[test]
        fn should_be_displayed() {
            // Set the time to the current time.
            clock::Stopped::local_set_to_unix_epoch();

            let expiring_key = authentication::key::generate_key(Some(Duration::from_secs(0))).unwrap();

            assert_eq!(
                expiring_key.to_string(),
                "key: `[REDACTED]`, valid until `1970-01-01 00:00:00 UTC`" // cspell:disable-line
            );
        }

        #[test]
        fn should_be_generated_with_a_expiration_time() {
            let expiring_key = authentication::key::generate_key(Some(Duration::new(9999, 0))).unwrap();

            assert!(authentication::key::verify_key_expiration(&expiring_key).is_ok());
        }

        #[test]
        fn expiration_verification_should_fail_when_the_key_has_expired() {
            // Set the time to the current time.
            clock::Stopped::local_set_to_system_time_now();

            // Make key that is valid for 19 seconds.
            let expiring_key = authentication::key::generate_key(Some(Duration::from_secs(19))).unwrap();

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

        use torrust_clock::clock;
        use torrust_clock::clock::stopped::Stopped as _;

        use crate::authentication;

        #[test]
        fn should_be_displayed() {
            let permanent_key = authentication::key::generate_permanent_key();

            assert_eq!(
                permanent_key.to_string(),
                "key: `[REDACTED]`, permanent" // cspell:disable-line
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
            let err = sqlx::Error::RowNotFound;

            let err: key::Error = err.into();

            assert!(matches!(err, key::Error::KeyVerificationError { .. }));
        }

        #[test]
        fn should_render_the_database_error_text_and_no_source_path() {
            let err: key::Error = sqlx::Error::RowNotFound.into();

            assert_eq!(
                err.to_string(),
                "Key could not be verified: no rows returned by a query that expected to return at least one row"
            );
        }
    }

    mod the_expiration_of_a_key {
        use std::time::Duration;

        use quickcheck::TestResult;
        use quickcheck_macros::quickcheck;
        use rstest::rstest;
        use torrust_clock::clock;
        use torrust_clock::clock::stopped::Stopped as _;

        use crate::authentication::key::{Error, generate_key, verify_key_expiration};
        use crate::authentication::{Key, PeerKey};

        const EXPIRY: Duration = Duration::new(1_000, 500);

        fn key_expiring_at(valid_until: Duration) -> PeerKey {
            PeerKey {
                key: Key::random(),
                valid_until: Some(valid_until),
            }
        }

        #[rstest]
        #[case::one_nanosecond_before_the_expiry(Duration::new(1_000, 499), false)]
        #[case::exactly_at_the_expiry(Duration::new(1_000, 500), false)]
        #[case::one_nanosecond_after_the_expiry(Duration::new(1_000, 501), true)]
        fn it_should_expire_a_key_only_strictly_after_its_expiry_time(#[case] now: Duration, #[case] expired: bool) {
            // Arrange
            clock::Stopped::local_set(&now);
            let peer_key = key_expiring_at(EXPIRY);

            // Act
            let result = verify_key_expiration(&peer_key);

            // Assert
            assert_eq!(
                matches!(result, Err(Error::KeyExpired)),
                expired,
                "a key expiring at {EXPIRY:?} checked at {now:?} gave {result:?}"
            );
        }

        #[rstest]
        #[case::at_the_epoch(Duration::ZERO)]
        #[case::after_a_year(Duration::from_secs(365 * 24 * 60 * 60))]
        #[case::at_the_largest_time(Duration::new(u64::MAX, 0))]
        fn it_should_never_expire_a_permanent_key(#[case] now: Duration) {
            // Arrange
            clock::Stopped::local_set(&now);
            let permanent_key = generate_key(None).unwrap();

            // Act
            let result = verify_key_expiration(&permanent_key);

            // Assert
            assert!(result.is_ok(), "a permanent key checked at {now:?} gave {result:?}");
        }

        #[quickcheck]
        fn it_should_expire_a_key_exactly_when_its_expiry_is_before_now(
            expiry_seconds: u32,
            expiry_nanos: u32,
            now_seconds: u32,
            now_nanos: u32,
        ) -> TestResult {
            let expiry = Duration::new(u64::from(expiry_seconds), expiry_nanos % 1_000_000_000);
            let now = Duration::new(u64::from(now_seconds), now_nanos % 1_000_000_000);
            clock::Stopped::local_set(&now);

            let result = verify_key_expiration(&key_expiring_at(expiry));

            TestResult::from_bool(matches!(result, Err(Error::KeyExpired)) == (expiry < now))
        }

        #[rstest]
        #[case::a_whole_number_of_seconds(Some(Duration::from_secs(500)), Some(Duration::from_secs(1_500)))]
        #[case::with_nanoseconds(Some(Duration::new(500, 7)), Some(Duration::new(1_500, 7)))]
        #[case::no_lifetime(None, None)]
        fn it_should_generate_a_key_that_expires_at_now_plus_its_lifetime(
            #[case] lifetime: Option<Duration>,
            #[case] expected_valid_until: Option<Duration>,
        ) {
            // Arrange
            clock::Stopped::local_set(&Duration::from_secs(1_000));

            // Act
            let peer_key = generate_key(lifetime).unwrap();

            // Assert
            assert_eq!(
                peer_key.valid_until, expected_valid_until,
                "a key generated with lifetime {lifetime:?}"
            );
        }
    }
}
