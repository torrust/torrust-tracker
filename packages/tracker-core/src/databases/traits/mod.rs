//! Narrow context traits and the aggregate [`Database`] supertrait.
//!
//! Design rationale and revisit criteria:
//! [`20260429000000_keep_database_as_aggregate_supertrait`](../../../../docs/adrs/20260429000000_keep_database_as_aggregate_supertrait.md).
pub mod auth_keys;
pub mod database;
pub mod schema;
pub mod torrent_metrics;
pub mod whitelist;

pub use auth_keys::AuthKeyStore;
#[cfg(test)]
pub use auth_keys::MockAuthKeyStore;
pub use database::Database;
#[cfg(test)]
pub use schema::MockSchemaMigrator;
pub use schema::SchemaMigrator;
pub use torrent_metrics::{MockTorrentMetricsStore, TorrentMetricsStore};
#[cfg(test)]
pub use whitelist::MockWhitelistStore;
pub use whitelist::WhitelistStore;
