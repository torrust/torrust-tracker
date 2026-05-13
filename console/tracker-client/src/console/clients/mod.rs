//! Console clients.
//!
//! `unified` contains the in-progress single-binary implementation for issue #1771.
//! Legacy modules remain available during the deprecation window and are intentionally
//! kept separate so old binaries can stay frozen until the scheduled cleanup removal.
pub mod checker;
pub mod http;
pub mod udp;
pub mod unified;
