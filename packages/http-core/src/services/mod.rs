//! Application services for the HTTP tracker.
//!
//! These modules contain logic that is specific to the HTTP tracker. They do
//! not depend on the Axum web server and could be reused for other web
//! servers.
//!
//! Refer to [`torrust_tracker`](crate) documentation.
pub mod announce;
pub(crate) mod error_mapping;
pub mod scrape;
