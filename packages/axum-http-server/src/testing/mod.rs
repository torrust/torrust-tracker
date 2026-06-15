//! Test-only infrastructure for `axum-http-server`.
//!
//! This module provides convenience setup code (wiring containers, starting/stopping
//! the server) for integration tests in this crate and external consumers such as
//! `axum-health-check-api-server`.
//!
//! > **Note**: Like `tracker-core::test_helpers`, this module is exported unconditionally
//! > from `lib.rs` so that external test packages can import it. It is primarily intended
//! > for test use, but is compiled in all build profiles.

pub mod environment;
