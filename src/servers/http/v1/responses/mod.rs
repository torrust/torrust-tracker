//! HTTP responses for the HTTP tracker.
//!
//! Refer to the generic [HTTP server documentation](crate::servers::http) for
//! more information about the HTTP tracker.
pub mod announce;
pub mod error;
pub mod scrape;

pub use announce::{Announce, Compact, Normal};

/// Trait that defines the Announce Response Format
pub trait Response: axum::response::IntoResponse {
    /// Returns the Body of the Announce Response
    ///
    /// # Errors
    ///
    /// If unable to generate the response, it will return an error.
    fn body(self) -> Result<Vec<u8>, error::Error>;
}
