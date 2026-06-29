//! # Torrust Tracker REST API Runtime Adapter
//!
//! `torrust-tracker-rest-api-runtime-adapter` provides tracker-specific
//! implementations of the application layer port traits.
//!
//! This package owns:
//!
//! - Adapter implementations for `TorrentQueryPort` and other ports.
//! - Conversion from domain types to protocol DTOs.
//! - Wiring tracker internals to the application layer.
//!
//! This package does NOT own:
//!
//! - Protocol DTOs (those belong to `rest-api-protocol`).
//! - Use-case services (those belong to `rest-api-application`).
//! - Axum server routing or middleware.
pub mod adapters;
pub mod container;
pub mod conversion;
