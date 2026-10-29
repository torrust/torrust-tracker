//! Narrow context traits and the aggregate [`Database`] supertrait.
//!
//! Design rationale and revisit criteria:
//! [`20260429000000_keep_database_as_aggregate_supertrait`](../../../../docs/adrs/20260429000000_keep_database_as_aggregate_supertrait.md).
pub mod auth_keys;
pub mod database;
pub mod schema;
pub mod torrent_metrics;
pub mod whitelist;

// Only `MockTorrentMetricsStore` is used by other packages' tests, so only it sits behind the `testing`
// feature. The other mocks are generated for this crate's own tests (`cfg(test)`).
pub use auth_keys::AuthKeyStore;
#[cfg(test)]
pub use auth_keys::MockAuthKeyStore;
pub use database::Database;
#[cfg(test)]
pub use schema::MockSchemaMigrator;
pub use schema::SchemaMigrator;
#[cfg(feature = "testing")]
pub use torrent_metrics::MockTorrentMetricsStore;
pub use torrent_metrics::TorrentMetricsStore;
#[cfg(test)]
pub use whitelist::MockWhitelistStore;
pub use whitelist::WhitelistStore;
