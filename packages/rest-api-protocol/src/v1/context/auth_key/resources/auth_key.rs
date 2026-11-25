//! API resources for the authentication key context.
//!
//! These types define the serialization contract for the `/api/v1/keys`
//! endpoint responses.
use std::fmt;

use serde::{Deserialize, Serialize};

/// A resource that represents an authentication key.
#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthKey {
    /// The authentication key.
    pub key: String,
    /// The timestamp when the key will expire.
    #[deprecated(since = "3.0.0", note = "please use `expiry_time` instead")]
    pub valid_until: Option<u64>,
    /// The ISO 8601 timestamp when the key will expire.
    pub expiry_time: Option<String>,
}

/// The key is a secret, so `Debug` redacts it. The resource is serialized with the key where a
/// reply deliberately hands a generated or added key to its owner.
#[allow(deprecated)]
impl fmt::Debug for AuthKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AuthKey")
            .field("key", &"[REDACTED]")
            .field("valid_until", &self.valid_until)
            .field("expiry_time", &self.expiry_time)
            .finish()
    }
}

/// Errors that can occur during auth key operations.
///
/// These correspond to the variants of `tracker_core::error::PeerKeyError`
/// but are protocol-level types without tracker-core dependencies.
#[derive(Debug)]
#[non_exhaustive]
pub enum AuthKeyError {
    /// The provided duration overflows.
    DurationOverflow { seconds_valid: u64 },
    /// The provided key is invalid. The error keeps the reason, which states
    /// what is wrong with the key (its length or its characters), and never
    /// the key: a malformed key is still the user's secret.
    InvalidKey { reason: String },
    /// The private-tracker capability is disabled by configuration.
    DisabledByConfiguration { capability: &'static str },
    /// A database error occurred during the auth key operation.
    Database(String),
}

impl fmt::Display for AuthKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DurationOverflow { seconds_valid } => {
                write!(f, "duration overflow: {seconds_valid}")
            }
            Self::InvalidKey { reason } => {
                write!(f, "invalid key: {reason}")
            }
            Self::DisabledByConfiguration { capability } => {
                write!(f, "{capability} capability is disabled by configuration")
            }
            Self::Database(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for AuthKeyError {}

#[cfg(test)]
mod tests {
    use super::AuthKey;

    #[test]
    #[allow(deprecated)]
    fn it_should_not_print_the_key_in_its_debug_output() {
        let secret = "TestPeerKeyInTheAuthKeyResource1";
        let resource = AuthKey {
            key: secret.to_string(),
            valid_until: Some(100),
            expiry_time: Some("1970-01-01 00:01:40 UTC".to_string()),
        };

        let debug = format!("{resource:?}");

        assert!(!debug.contains(secret), "the Debug output holds the test secret");
        assert!(
            debug.contains("expiry_time"),
            "the Debug output should still show the other fields: {debug}"
        );
    }
}
