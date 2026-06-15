//! Test-only infrastructure for `axum-http-server`.
//!
//! This module provides convenience setup code (wiring containers, starting/stopping
//! the server) for integration tests in this crate and external consumers such as
//! `axum-health-check-api-server`.
//!
//! It is **not** compiled into production builds.

pub mod environment;
