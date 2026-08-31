//! API resources for the whitelist context.
//!
//! Most whitelist responses reuse the [`ActionStatus`] enum from
//! `rest-api-protocol::v1::responses`. This module defines the specific
//! error type for whitelist command failures.
use std::fmt;

/// Errors that can occur during whitelist operations.
///
/// This type is used in the port trait's return type so that
/// the application layer and Axum handlers can handle errors
/// without depending on `tracker-core` database error types.
#[derive(Debug)]
pub enum WhitelistError {
    /// The listed-tracker capability is disabled by configuration.
    DisabledByConfiguration { capability: &'static str },
    /// A database error occurred during the whitelist operation.
    Database(String),
}

impl fmt::Display for WhitelistError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WhitelistError::DisabledByConfiguration { capability } => {
                write!(f, "{capability} capability is disabled by configuration")
            }
            // Forward the inner message as-is to preserve the original
            // error response format from the previous direct-to-WhitelistManager
            // wiring (the Axum handlers format via `{e}`).
            WhitelistError::Database(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for WhitelistError {}
