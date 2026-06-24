//! # Torrust Tracker REST API Protocol
//!
//! `torrust-tracker-rest-api-protocol` contains versioned contract artifacts
//! for the Torrust Tracker REST API.
//!
//! This package owns:
//!
//! - Versioned endpoint contract modules (`v1`, `v2`, ...).
//! - Request/response DTOs, error schemas, and status mapping contracts.
//! - Auth contract surface (transport-agnostic semantics).
//!
//! This package does NOT own:
//!
//! - Axum server routing or middleware.
//! - Tracker internal database or domain logic.
//! - Client transport, retries, or timeouts.
pub mod v1;
