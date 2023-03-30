//! Axum [`extractors`](axum::extract) for the HTTP server.
//!
//! This module contains the extractors used by the HTTP server to parse the
//! incoming requests.
pub mod announce_request;
pub mod authentication_key;
pub mod client_ip_sources;
pub mod scrape_request;
