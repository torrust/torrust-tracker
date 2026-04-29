//! This module provides functionality for setting up databases.
//!
//! For the persistence trait boundary and wiring rationale, see ADR
//! [`20260429000000_keep_database_as_aggregate_supertrait`](../../../docs/adrs/20260429000000_keep_database_as_aggregate_supertrait.md).
use std::sync::Arc;

use torrust_tracker_configuration::Core;

use super::driver::mysql::Mysql;
use super::driver::sqlite::Sqlite;
use super::driver::Driver;
use super::traits::{AuthKeyStore, SchemaMigrator, TorrentMetricsStore, WhitelistStore};

/// A bundle of narrow-trait store references, one per persistence context.
///
/// The factory (`initialize_database`) constructs the concrete driver once and
/// coerces it into each narrow `Arc<dyn XxxStore>`.  Individual services are
/// wired at construction time by passing the relevant field
/// (e.g. `database_stores.auth_key_store.clone()`) to each constructor.
/// Services themselves never hold a `DatabaseStores`; they only see the narrow
/// trait they need.
pub struct DatabaseStores {
    /// Schema lifecycle: create / drop tables.
    pub schema_migrator: Arc<dyn SchemaMigrator>,
    /// Per-torrent and global download counters.
    pub torrent_metrics_store: Arc<dyn TorrentMetricsStore>,
    /// Torrent infohash whitelist.
    pub whitelist_store: Arc<dyn WhitelistStore>,
    /// Authentication key persistence.
    pub auth_key_store: Arc<dyn AuthKeyStore>,
}

/// Initializes and returns a [`DatabaseStores`] bundle based on the provided
/// configuration.
///
/// This function creates a new database driver according to the settings
/// defined in the [`Core`] configuration. It selects the appropriate driver
/// (either `Sqlite3` or `MySQL`) as specified in `config.database.driver` and
/// attempts to build the database connection using the path defined in
/// `config.database.path`.
///
/// The concrete driver is constructed once and coerced into four narrow
/// `Arc<dyn XxxStore>` references, one for each persistence context.
///
/// # Panics
///
/// This function will panic if the database cannot be initialized (i.e., if the
/// driver fails to build the connection). This is enforced by the use of
/// [`expect`](std::result::Result::expect) in the implementation.
///
/// # Example
///
/// ```rust,no_run
/// use torrust_tracker_configuration::Core;
/// use bittorrent_tracker_core::databases::setup::initialize_database;
///
/// // Create a default configuration (ensure it is properly set up for your environment)
/// let config = Core::default();
///
/// // Initialize the database; this will panic if initialization fails.
/// let stores = initialize_database(&config);
/// ```
#[must_use]
pub fn initialize_database(config: &Core) -> DatabaseStores {
    let driver = match config.database.driver {
        torrust_tracker_configuration::Driver::Sqlite3 => Driver::Sqlite3,
        torrust_tracker_configuration::Driver::MySQL => Driver::MySQL,
    };

    match driver {
        Driver::Sqlite3 => {
            let db = Arc::new(Sqlite::new(&config.database.path).expect("Database driver build failed."));
            db.create_database_tables().expect("Could not create database tables.");
            DatabaseStores {
                schema_migrator: db.clone(),
                torrent_metrics_store: db.clone(),
                whitelist_store: db.clone(),
                auth_key_store: db,
            }
        }
        Driver::MySQL => {
            let db = Arc::new(Mysql::new(&config.database.path).expect("Database driver build failed."));
            db.create_database_tables().expect("Could not create database tables.");
            DatabaseStores {
                schema_migrator: db.clone(),
                torrent_metrics_store: db.clone(),
                whitelist_store: db.clone(),
                auth_key_store: db,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::initialize_database;
    use crate::test_helpers::tests::ephemeral_configuration;

    #[test]
    fn it_should_initialize_the_sqlite_database() {
        let config = ephemeral_configuration();
        let _database = initialize_database(&config);
    }
}
