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
use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use derive_more::Display;
use rand::distr::Alphanumeric;
use rand::{Rng, rng};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use torrust_clock::DurationSinceUnixEpoch;
use torrust_clock::conv::convert_from_timestamp_to_datetime_utc;

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
/// use torrust_tracker_core::authentication::key::peer_key::{Key, PeerKey};
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

/// Describes the key without printing it: the key is a secret, and nothing needs the whole
/// `PeerKey` as text. Code that hands the key to its owner reads `PeerKey::key` instead.
impl fmt::Display for PeerKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.expiry_time() {
            Some(expire_time) => write!(f, "key: `[REDACTED]`, valid until `{expire_time}`"),
            None => write!(f, "key: `[REDACTED]`, permanent"),
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
/// use torrust_tracker_core::authentication::key::peer_key::Key;
/// let key = Key::new("YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ").unwrap();
/// ```
///
/// Generating a random key:
///
/// ```
/// use torrust_tracker_core::authentication::key::peer_key::Key;
/// let random_key = Key::random();
/// ```
///
/// # Secrecy
///
/// A key is a secret. `Debug` redacts it (`Key([REDACTED])`), so a type that derives `Debug` and
/// holds a `Key` cannot print one by accident. `Display` and `Serialize` print the key and stay
/// only where it is deliberately handed to its owner or persisted: the REST reply that returns a
/// generated or added key, the database store, and the REST client building a key delete URL.
#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Display, Hash)]
pub struct Key(String);

impl fmt::Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Key([REDACTED])")
    }
}

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
        let length = value.chars().count();

        if length != AUTH_KEY_LENGTH {
            return Err(ParseKeyError::InvalidKeyLength { length });
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
        random_id.parse::<Self>().expect("Failed to generate a valid random key")
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
/// use torrust_tracker_core::authentication::Key;
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
/// this error. The error never holds the rejected string, because a malformed
/// key is still the user's secret: it states the received length or the
/// allowed characters instead.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseKeyError {
    /// The provided key does not have exactly 32 characters. It keeps the
    /// number of characters received, never the key: the key is a secret.
    #[error("expected 32 characters, got {length}")]
    InvalidKeyLength {
        /// The number of characters (not bytes) of the rejected key.
        length: usize,
    },

    /// The provided key contains invalid characters. Only ASCII alphanumeric
    /// characters are allowed.
    #[error("only the characters 0-9, a-z and A-Z are allowed")]
    InvalidChars,
}

impl FromStr for Key {
    type Err = ParseKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)?;
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
        fn it_should_be_redacted_in_its_debug_output() {
            let secret = "TestPeerKeyForDebugOutput0000001";
            let key = Key::new(secret).unwrap();

            let debug = format!("{key:?}");

            assert_eq!(debug, "Key([REDACTED])");
            assert!(!debug.contains(secret), "the Debug output holds the test secret");
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
        fn it_should_name_the_received_length_when_the_key_has_the_wrong_length() {
            let fourteen_characters = "ShortTestKey01";

            let error = Key::new(fourteen_characters).unwrap_err();

            assert_eq!(error.to_string(), "expected 32 characters, got 14");
        }

        #[test]
        fn it_should_count_characters_not_bytes_when_naming_the_received_length() {
            // 30 ASCII characters and one two-byte character: 31 characters, 32 bytes. Counting
            // bytes would accept the length and reject the characters instead.
            let thirty_one_characters = "TestPeerKeyWithAccent000000000é";

            let error = Key::new(thirty_one_characters).unwrap_err();

            assert_eq!(error.to_string(), "expected 32 characters, got 31");
        }

        #[test]
        fn it_should_reject_a_32_character_key_with_a_non_ascii_character_for_its_characters() {
            // 31 ASCII characters and one two-byte character: 32 characters, 33 bytes.
            let thirty_two_characters = "TestPeerKeyWithAccent0000000000é";

            let error = Key::new(thirty_two_characters).unwrap_err();

            assert_eq!(error.to_string(), "only the characters 0-9, a-z and A-Z are allowed");
        }

        #[test]
        fn it_should_name_the_allowed_characters_when_the_key_has_another_character() {
            let key_with_a_dash = "TestPeerKey-00000000000000000001";

            let error = Key::new(key_with_a_dash).unwrap_err();

            assert_eq!(error.to_string(), "only the characters 0-9, a-z and A-Z are allowed");
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

        #[test]
        fn it_should_not_print_its_key_in_debug_or_display_output() {
            let secret = "TestPeerKeyForPeerKeyOutput00001";
            let peer_key = PeerKey {
                key: Key::new(secret).unwrap(),
                valid_until: Some(Duration::from_secs(100)),
            };

            let debug = format!("{peer_key:?}");
            let display = peer_key.to_string();

            assert!(!debug.contains(secret), "the Debug output holds the test secret");
            assert!(!display.contains(secret), "the Display output holds the test secret");
            assert!(debug.contains("Key([REDACTED])"), "{debug}");
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
                    "key: `[REDACTED]`, valid until `1970-01-01 00:01:40 UTC`" // cspell:disable-line
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

                assert_eq!(permanent_key.to_string(), "key: `[REDACTED]`, permanent"); // cspell:disable-line
            }
        }
    }

    mod key_validation {
        use std::str::FromStr;

        use quickcheck::TestResult;
        use quickcheck_macros::quickcheck;
        use rstest::rstest;

        use crate::authentication::Key;
        use crate::authentication::key::peer_key::ParseKeyError;

        #[rstest]
        #[case::letters_and_digits("YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ", None)]
        #[case::only_digits("01234567890123456789012345678901", None)]
        #[case::one_character_short("YZSl4lMZupRuOpSRC3krIKR5BPB14nr", Some(ParseKeyError::InvalidKeyLength { length: 31 }))]
        #[case::one_character_long("YZSl4lMZupRuOpSRC3krIKR5BPB14nrJx", Some(ParseKeyError::InvalidKeyLength { length: 33 }))]
        #[case::empty("", Some(ParseKeyError::InvalidKeyLength { length: 0 }))]
        #[case::a_hyphen("YZSl4lMZupRuOpSRC3krIKR5BPB14n-J", Some(ParseKeyError::InvalidChars))]
        #[case::a_space("YZSl4lMZupRuOpSRC3krIKR5BPB14n J", Some(ParseKeyError::InvalidChars))]
        #[case::sixteen_two_byte_letters(
            "\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}",
            Some(ParseKeyError::InvalidKeyLength { length: 16 })
        )]
        #[case::thirty_two_two_byte_letters(
            "\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}",
            Some(ParseKeyError::InvalidChars)
        )]
        #[case::too_long_and_with_a_hyphen("YZSl4lMZupRuOpSRC3krIKR5BPB14n-Jx", Some(ParseKeyError::InvalidKeyLength { length: 33 }))]
        fn it_should_accept_only_thirty_two_ascii_alphanumeric_characters(
            #[case] input: &str,
            #[case] expected: Option<ParseKeyError>,
        ) {
            // Act
            let from_new = Key::new(input).err();
            let from_str = Key::from_str(input).err();

            // Assert
            assert_eq!(from_new, expected, "Key::new({input:?})");
            assert_eq!(from_str, expected, "Key::from_str({input:?}) should agree with Key::new");
        }

        #[quickcheck]
        #[expect(
            clippy::needless_pass_by_value,
            reason = "quickcheck passes each generated argument by value"
        )]
        fn it_should_accept_a_string_exactly_when_it_is_thirty_two_ascii_alphanumeric_characters(
            seed: Vec<u8>,
            extra: u8,
        ) -> TestResult {
            // Strings of 31 to 33 characters from an alphabet that is mostly valid, so the accepted and the
            // rejected cases both come up often.
            const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789- \u{e9}";
            if seed.is_empty() {
                return TestResult::discard();
            }
            let alphabet: Vec<char> = ALPHABET.chars().collect();
            let input: String = seed
                .iter()
                .cycle()
                .take(31 + usize::from(extra % 3))
                .map(|byte| alphabet[usize::from(*byte) % alphabet.len()])
                .collect();
            let is_valid = input.chars().count() == 32 && input.chars().all(|c| c.is_ascii_alphanumeric());

            TestResult::from_bool(Key::new(&input).map_or(!is_valid, |key| is_valid && key.value() == input))
        }

        #[test]
        fn it_should_generate_only_valid_random_keys() {
            for _ in 0..100 {
                // Act
                let key = Key::random();

                // Assert
                assert!(Key::new(key.value()).is_ok(), "a random key should be valid: {}", key.value());
            }
        }
    }

    mod peer_key_equality {
        use std::time::Duration;

        use rstest::rstest;

        use crate::authentication::{Key, PeerKey};

        const KEY: &str = "YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ";
        const OTHER_KEY: &str = "ZYSl4lMZupRuOpSRC3krIKR5BPB14nrJ";

        fn a_peer_key(key: &str, valid_until: Option<Duration>) -> PeerKey {
            PeerKey {
                key: key.parse::<Key>().unwrap(),
                valid_until,
            }
        }

        #[rstest]
        #[case::the_same_key_and_expiry(KEY, Some(Duration::new(100, 0)), KEY, Some(Duration::new(100, 0)), true)]
        #[case::the_same_second_with_other_nanoseconds(
            KEY,
            Some(Duration::new(100, 1)),
            KEY,
            Some(Duration::new(100, 999)),
            true
        )]
        #[case::another_second(KEY, Some(Duration::new(100, 0)), KEY, Some(Duration::new(101, 0)), false)]
        #[case::permanent_and_expiring(KEY, None, KEY, Some(Duration::new(100, 0)), false)]
        #[case::expiring_and_permanent(KEY, Some(Duration::new(100, 0)), KEY, None, false)]
        #[case::another_key(KEY, Some(Duration::new(100, 0)), OTHER_KEY, Some(Duration::new(100, 0)), false)]
        #[case::both_permanent(KEY, None, KEY, None, true)]
        fn it_should_compare_the_key_and_the_expiry_in_whole_seconds(
            #[case] key: &str,
            #[case] valid_until: Option<Duration>,
            #[case] other_key: &str,
            #[case] other_valid_until: Option<Duration>,
            #[case] expected: bool,
        ) {
            // Arrange
            let peer_key = a_peer_key(key, valid_until);
            let other = a_peer_key(other_key, other_valid_until);

            // Act
            let equal = peer_key == other;

            // Assert
            assert_eq!(equal, expected, "{peer_key} == {other}");
        }
    }
}
