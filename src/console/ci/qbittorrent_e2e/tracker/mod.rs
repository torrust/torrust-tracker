//! Torrust Tracker feature module for the qBittorrent E2E tests.

// Individual struct/enum `pub(crate)` annotations are intentional documentation
// of visibility intent even though they are technically redundant (private module).
#![allow(
    clippy::redundant_pub_crate,
    reason = "explicit pub(crate) documents intended visibility during staged migration"
)]
mod client;
mod config_builder;

pub(crate) use client::TrackerApiClient;
pub(super) use config_builder::{DatabaseDriver, TrackerConfig, TrackerConfigBuilder};
