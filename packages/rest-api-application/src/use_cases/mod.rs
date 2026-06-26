//! Use-case services for the REST API.
//!
//! Each service orchestrates business logic by calling port traits.
pub mod auth_key;
pub mod stats;
pub mod torrent;
pub mod whitelist;
