//! # Torrust Tracker REST API Application
//!
//! `torrust-tracker-rest-api-application` contains the use-case services and
//! port traits for the Torrust Tracker REST API.
//!
//! This package owns:
//!
//! - Port traits (interfaces) such as `TorrentQueryPort`.
//! - Use-case services and orchestration logic.
//! - Mapping of domain errors to protocol-level error categories.
//!
//! This package does NOT own:
//!
//! - Axum server routing or middleware.
//! - Tracker internal database or domain logic.
//! - Protocol DTOs (those belong to `rest-api-protocol`).
pub mod v1;
