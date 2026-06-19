//! Test-only infrastructure for `axum-rest-api-server`.
//!
//! This module provides convenience setup code (wiring containers, starting/stopping
//! the server) for integration tests in this crate and external consumers such as
//! `axum-health-check-api-server`.
//!
//! > **Note**: Like `tracker-core::test_helpers`, this module is exported unconditionally
//! > from `lib.rs` so that external test packages can import it. It is primarily intended
//! > for test use, but is compiled in all build profiles.
//!
//! > **Note**: The UDP dependencies (`udp-server`, `udp-core`) are still
//! > needed at runtime because the production handlers in this crate reference
//! > their types directly. Full demotion to dev-dependencies requires the
//! > prerequisite decoupling in `rest-api-core` first.

pub mod environment;
