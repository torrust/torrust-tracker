//! Shared test utilities for integration tests.
//!
//! This module is shared across multiple integration-test binaries via
//! `mod common;`. Each top-level file under `tests/` is a separate Cargo
//! integration-test executable. Common helpers belong here rather than in
//! a top-level file, so all test binaries can reach them.
//!
//! # Architecture
//!
//! Each integration-test binary manages **one** tracker application instance
//! with a fixed initial configuration. Scenario functions run sequentially
//! against that instance. A different initial configuration belongs to
//! another top-level binary, which Cargo may run concurrently.
//!
//! See `docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md`
//! for the full decision record.
mod announce;
mod statistics;
mod workspace;

// Each integration-test binary compiles this module independently. Not all
// binaries call every re-exported function, so the compiler emits
// unused_imports warnings for the binaries that don't. The attributes
// suppress those per-binary false positives.
#[allow(unused_imports)]
pub use announce::{http_announce, udp_announce};
#[allow(unused_imports)]
pub use statistics::{PartialGlobalStatistics, get_tracker_statistics};
#[allow(unused_imports)]
pub use workspace::{
    EphemeralTrackerWorkspace, http_api_url, http_tracker_urls, start_tracker_with_config, udp_socket_addr, udp_tracker_urls,
};
