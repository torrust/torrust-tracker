//! API resources for the authentication key context.
//!
//! These types define the serialization contract for the `/api/v1/keys`
//! endpoint responses.
use std::fmt;

use serde::{Deserialize, Serialize};

/// A resource that represents an authentication key.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct AuthKey {
    /// The authentication key.
    pub key: String,
    /// The timestamp when the key will expire.
    #[deprecated(since = "3.0.0", note = "please use `expiry_time` instead")]
    pub valid_until: Option<u64>,
    /// The ISO 8601 timestamp when the key will expire.
    pub expiry_time: Option<String>,
}

/// Errors that can occur during auth key operations.
///
/// These correspond to the variants of `tracker_core::error::PeerKeyError`
/// but are protocol-level types without tracker-core dependencies.
#[derive(Debug)]
pub enum AuthKeyError {
    /// The provided duration overflows.
    DurationOverflow { seconds_valid: u64 },
    /// The provided key is invalid.
    InvalidKey { key: String, reason: String },
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
            Self::InvalidKey { key, reason } => {
                write!(f, "invalid key: \"{key}\", {reason}")
            }
            Self::DisabledByConfiguration { capability } => {
                write!(f, "{capability} capability is disabled by configuration")
            }
            Self::Database(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for AuthKeyError {}
