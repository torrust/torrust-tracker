//! Narrow context traits and the aggregate [`Database`] supertrait.
//!
//! Design rationale and revisit criteria:
//! [`20260429000000_keep_database_as_aggregate_supertrait`](../../../../docs/adrs/20260429000000_keep_database_as_aggregate_supertrait.md).
pub mod auth_keys;
pub mod database;
pub mod schema;
pub mod torrent_metrics;
pub mod whitelist;

pub use auth_keys::{AuthKeyStore, MockAuthKeyStore};
pub use database::Database;
pub use schema::{MockSchemaMigrator, SchemaMigrator};
pub use torrent_metrics::{MockTorrentMetricsStore, TorrentMetricsStore};
pub use whitelist::{MockWhitelistStore, WhitelistStore};
