//! Test-only infrastructure for `axum-rest-api-server`.
//!
//! This module provides convenience setup code (wiring containers, starting/stopping
//! the server) for integration tests in this crate and external consumers such as
//! `axum-health-check-api-server`.
//!
//! > **Note**: This module is compiled only with the non-default `testing` feature, which
//! > is test support outside the semver guarantees of this package. Inside this workspace the
//! > feature is enabled from `[dev-dependencies]` entries only (the `Shipped Features` check
//! > enforces it), so that it never reaches a shipped build; the package's own targets enable
//! > it through a path-only self dev-dependency. Anyone else who wants this environment in a
//! > build opts in knowingly and accepts that it can change in any release.

pub mod environment;
