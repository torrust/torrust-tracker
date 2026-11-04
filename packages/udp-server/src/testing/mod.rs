//! Test-only infrastructure for `udp-server`.
//!
//! This module provides convenience setup code (wiring containers, starting/stopping
//! the server) for integration tests in this crate and external consumers such as
//! `axum-health-check-api-server`.
//!
//! > **Note**: This module is compiled only with the non-default `testing` feature, which
//! > is test support outside the semver guarantees of this package. Enable it from a
//! > `[dev-dependencies]` entry only; the package's own targets enable it through a
//! > path-only self dev-dependency.

pub mod environment;
