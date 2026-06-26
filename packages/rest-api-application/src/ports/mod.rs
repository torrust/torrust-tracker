//! Port traits for REST API use-cases.
//!
//! These traits define the boundary between the application layer and
//! the tracker-internal implementation. Implementations live in the
//! runtime adapter package.
pub mod auth_key;
pub mod torrent;
pub mod whitelist;
