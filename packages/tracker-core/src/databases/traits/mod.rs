//! Narrow context traits and the aggregate [`Database`] supertrait.
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
