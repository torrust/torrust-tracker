//! Version 1 of the Torrust Tracker REST API contract.
//!
//! This module defines the wire-format DTOs and protocol semantics for the v1
//! REST API. These types are transport-agnostic: they can be serialized/deserialized
//! without any Axum or HTTP server dependency.
//!
//! # Type ownership
//!
//! - Request/response DTOs belong here.
//! - Error schemas and status codes belong here.
//! - `From` conversions from domain types belong in the runtime adapter layer,
//!   not in this package.
pub mod resources;
pub mod responses;
