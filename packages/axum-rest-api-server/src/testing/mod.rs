//! Test-only infrastructure for `axum-rest-api-server`.
//!
//! This module provides convenience setup code (wiring containers, starting/stopping
//! the server) for integration tests in this crate and external consumers such as
//! `axum-health-check-api-server`.
//!
//! It is **not** compiled into production builds.
//!
//! > **Note**: The UDP dependencies (`udp-server`, `udp-tracker-core`) are still
//! > needed at runtime because the production handlers in this crate reference
//! > their types directly. Full demotion to dev-dependencies requires the
//! > prerequisite decoupling in `rest-api-core` first.

pub mod environment;
