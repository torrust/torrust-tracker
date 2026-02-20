//! Authentication keys for private trackers.
//!
//! This module defines the types and functionality for managing authentication
//! keys used by the tracker. These keys, represented by the `Key` and `PeerKey`
//!  types, are essential for authenticating peers in private tracker
//! environments.
//!
//! A `Key` is a 32-character alphanumeric token, while a `PeerKey` couples a
//! `Key` with an optional expiration timestamp. If the expiration is set (via
//! `valid_until`), the key will become invalid after that time.
use std::str::FromStr;
use std::time::Duration;

use derive_more::Display;
use rand::distr::Alphanumeric;
use rand::{rng, RngExt};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use torrust_tracker_clock::conv::convert_from_timestamp_to_datetime_utc;
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use super::AUTH_KEY_LENGTH;

/// A peer authentication key with an optional expiration time.
///
/// A `PeerKey` associates a generated `Key` (a 32-character alphanumeric string)
/// with an optional expiration timestamp (`valid_until`). If `valid_until` is
/// `None`, the key is considered permanent.
///
/// # Example
///
/// ```rust
/// use std::time::Duration;
/// use bittorrent_tracker_core::authentication::key::peer_key::{Key, PeerKey};
///
/// let expiring_key = PeerKey {
///     key: Key::random(),
///     valid_until: Some(Duration::from_secs(3600)), // Expires in 1 hour
/// };
///
/// let permanent_key = PeerKey {
///     key: Key::random(),
///     valid_until: None,
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PeerKey {
    /// A 32-character authentication key. For example: `YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ`
    pub key: Key,

    /// An optional expiration timestamp. If set, the key becomes invalid after
    /// this time. A value of `None` indicates a permanent key.
    pub valid_until: Option<DurationSinceUnixEpoch>,
}

impl PartialEq for PeerKey {
    fn eq(&self, other: &Self) -> bool {
        // When comparing two PeerKeys, ignore fractions of seconds since only
        // whole seconds are stored in the database.
        self.key == other.key
            && match (&self.valid_until, &other.valid_until) {
                (Some(a), Some(b)) => a.as_secs() == b.as_secs(),
                (None, None) => true,
                _ => false,
            }
    }
}

impl Eq for PeerKey {}

impl std::fmt::Display for PeerKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.expiry_time() {
            Some(expire_time) => write!(f, "key: `{}`, valid until `{}`", self.key, expire_time),
            None => write!(f, "key: `{}`, permanent", self.key),
        }
    }
}

impl PeerKey {
    #[must_use]
    pub fn key(&self) -> Key {
        self.key.clone()
    }

    /// Computes and returns the expiration time as a UTC `DateTime`, if one
    /// exists.
    ///
    /// The returned time is derived from the stored seconds since the Unix
    /// epoch. Note that any fractional seconds are discarded since only whole
    /// seconds are stored in the database.
    ///
    /// # Panics
    ///
    /// Panics if the key's timestamp overflows the internal `i64` type (this is
    ///  extremely unlikely, happening roughly 292.5 billion years from now).
    #[must_use]
    pub fn expiry_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        // We remove the fractions of seconds because we only store the seconds
        // in the database.
        self.valid_until
            .map(|valid_until| convert_from_timestamp_to_datetime_utc(Duration::from_secs(valid_until.as_secs())))
    }
}

/// A token used for authentication.
///
/// The `Key` type encapsulates a 32-character string that must consist solely
/// of ASCII alphanumeric characters (0-9, a-z, A-Z). This key is used by the
/// tracker to authenticate peers.
///
/// # Examples
///
/// Creating a key from a valid string:
///
/// ```
/// use bittorrent_tracker_core::authentication::key::peer_key::Key;
/// let key = Key::new("YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ").unwrap();
/// ```
///
/// Generating a random key:
///
/// ```
/// use bittorrent_tracker_core::authentication::key::peer_key::Key;
/// let random_key = Key::random();
/// ```
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Display, Hash)]
pub struct Key(String);

impl Key {
    /// Constructs a new `Key` from the given string.
    ///
    /// # Errors
    ///
    /// Returns a `ParseKeyError` if:
    ///
    /// - The input string does not have exactly 32 characters.
    /// - The input string contains characters that are not ASCII alphanumeric.
    pub fn new(value: &str) -> Result<Self, ParseKeyError> {
        if value.len() != AUTH_KEY_LENGTH {
            return Err(ParseKeyError::InvalidKeyLength);
        }

        if !value.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(ParseKeyError::InvalidChars);
        }

        Ok(Self(value.to_owned()))
    }

    /// Generates a new random authentication key.
    ///
    /// The random key is generated by sampling 32 ASCII alphanumeric characters.
    ///
    /// # Panics
    ///
    /// Panics if the random number generator fails to produce a valid key
    /// (extremely unlikely).
    pub fn random() -> Self {
        let random_id: String = rng()
            .sample_iter(&Alphanumeric)
            .take(AUTH_KEY_LENGTH)
            .map(char::from)
            .collect();
        random_id.parse::<Key>().expect("Failed to generate a valid random key")
    }

    #[must_use]
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Errors that can occur when parsing a string into a `Key`.
///
/// # Examples
///
/// ```rust
/// use bittorrent_tracker_core::authentication::Key;
/// use std::str::FromStr;
///
/// let key_string = "YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ";
/// let key = Key::from_str(key_string);
///
/// assert!(key.is_ok());
/// assert_eq!(key.unwrap().to_string(), key_string);
/// ```
///
/// If the string does not contains a valid key, the parser function will return
/// this error.
#[derive(Debug, Error)]
pub enum ParseKeyError {
    /// The provided key does not have exactly 32 characters.
    #[error("Invalid key length. Key must be have 32 chars")]
    InvalidKeyLength,

    /// The provided key contains invalid characters. Only ASCII alphanumeric
    /// characters are allowed.
    #[error("Invalid chars for key. Key can only alphanumeric chars (0-9, a-z, A-Z)")]
    InvalidChars,
}

impl FromStr for Key {
    type Err = ParseKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Key::new(s)?;
        Ok(Self(s.to_string()))
    }
}

#[cfg(test)]
mod tests {

    mod key {
        use std::str::FromStr;

        use crate::authentication::Key;

        #[test]
        fn should_be_parsed_from_an_string() {
            let key_string = "YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ";
            let key = Key::from_str(key_string);

            assert!(key.is_ok());
            assert_eq!(key.unwrap().to_string(), key_string);
        }

        #[test]
        fn should_be_generated_randomly() {
            let _key = Key::random();
        }

        #[test]
        fn length_should_be_32() {
            let key = Key::new("");
            assert!(key.is_err());

            let string_longer_than_32 = "012345678901234567890123456789012"; // DevSkim: ignore  DS173237
            let key = Key::new(string_longer_than_32);
            assert!(key.is_err());
        }

        #[test]
        fn should_only_include_alphanumeric_chars() {
            let key = Key::new("%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%");
            assert!(key.is_err());
        }

        #[test]
        fn should_return_a_reference_to_the_inner_string() {
            let key = Key::new("YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ").unwrap(); // DevSkim: ignore  DS173237

            assert_eq!(key.value(), "YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ"); // DevSkim: ignore  DS173237
        }
    }

    mod peer_key {

        use std::time::Duration;

        use crate::authentication::key::peer_key::{Key, PeerKey};

        #[test]
        fn could_have_an_expiration_time() {
            let expiring_key = PeerKey {
                key: Key::random(),
                valid_until: Some(Duration::from_secs(100)),
            };

            assert_eq!(expiring_key.expiry_time().unwrap().to_string(), "1970-01-01 00:01:40 UTC");
        }

        #[test]
        fn could_be_permanent() {
            let permanent_key = PeerKey {
                key: Key::random(),
                valid_until: None,
            };

            assert_eq!(permanent_key.expiry_time(), None);
        }

        mod expiring {
            use std::time::Duration;

            use crate::authentication::key::peer_key::{Key, PeerKey};

            #[test]
            fn should_be_displayed_when_it_is_expiring() {
                let expiring_key = PeerKey {
                    key: Key::random(),
                    valid_until: Some(Duration::from_secs(100)),
                };

                assert_eq!(
                    expiring_key.to_string(),
                    format!("key: `{}`, valid until `1970-01-01 00:01:40 UTC`", expiring_key.key) // cspell:disable-line
                );
            }
        }

        mod permanent {

            use crate::authentication::key::peer_key::{Key, PeerKey};

            #[test]
            fn should_be_displayed_when_it_is_permanent() {
                let permanent_key = PeerKey {
                    key: Key::random(),
                    valid_until: None,
                };

                assert_eq!(
                    permanent_key.to_string(),
                    format!("key: `{}`, permanent", permanent_key.key) // cspell:disable-line
                );
            }
        }
    }
}
