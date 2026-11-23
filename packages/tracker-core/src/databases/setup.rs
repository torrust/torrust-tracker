//! This module provides functionality for setting up databases.
//!
//! For the persistence trait boundary and wiring rationale, see ADR
//! [`20260429000000_keep_database_as_aggregate_supertrait`](../../../docs/adrs/20260429000000_keep_database_as_aggregate_supertrait.md).
use std::sync::Arc;

use torrust_tracker_configuration::v3_0_0::core::Core;
use torrust_tracker_configuration::v3_0_0::database::Database;

use super::driver::mysql::Mysql;
use super::driver::postgres::Postgres;
use super::driver::sqlite::Sqlite;
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

fn build_database_stores<T>(db: Arc<T>) -> DatabaseStores
where
    T: SchemaMigrator + TorrentMetricsStore + WhitelistStore + AuthKeyStore + Send + Sync + 'static,
{
    DatabaseStores {
        schema_migrator: db.clone(),
        torrent_metrics_store: db.clone(),
        whitelist_store: db.clone(),
        auth_key_store: db,
    }
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
/// In particular, schema initialization issues a query against the configured
/// database immediately after the driver is built. If the database service is
/// not yet ready to accept connections (for example, a freshly started `MySQL`
/// container that has not finished binding its TCP listener), the first query
/// can fail and this function will panic. The `sqlx` driver does not retry the
/// initial connection on its own, so callers are responsible for ensuring the
/// database is reachable before calling `initialize_database`.
///
/// Other panic causes include malformed connection URLs, authentication
/// failures, insufficient permissions to issue DDL, network errors, or any
/// other underlying `sqlx::Error` returned while creating the schema.
///
/// # Example
///
/// ```rust,no_run
/// use torrust_tracker_configuration::v3_0_0::core::Core;
/// use torrust_tracker_core::databases::setup::initialize_database;
///
/// // Create a default configuration (ensure it is properly set up for your environment)
/// let config = Core::default();
///
/// // Initialize the database; this will panic if initialization fails.
/// # async {
/// let stores = initialize_database(&config).await;
/// # };
/// ```
#[must_use]
pub async fn initialize_database(config: &Core) -> DatabaseStores {
    let database = config
        .database
        .as_ref()
        .expect("database configuration is required to initialize persistence");
    initialize_database_from_configuration(database)
        .await
        .expect("database initialization requested by this legacy convenience API must succeed")
}

/// Initializes and returns a [`DatabaseStores`] bundle for one selected
/// database configuration.
///
/// This factory is used at the optional persistence composition boundary, where
/// the selected database is already known to be present.
///
/// # Errors
///
/// Returns the typed driver or migration error when persistence cannot be
/// initialized.
pub async fn initialize_database_from_configuration(database: &Database) -> Result<DatabaseStores, super::error::Error> {
    match database {
        Database::Sqlite3 { path } => {
            let db = Arc::new(Sqlite::new(path)?);
            db.create_database_tables().await?;
            Ok(build_database_stores(db))
        }
        Database::MySQL(connection) => {
            let database_url = Database::MySQL(connection.clone()).connection_url();
            let db = Arc::new(Mysql::new(&database_url)?);
            db.create_database_tables().await?;
            Ok(build_database_stores(db))
        }
        Database::PostgreSQL(connection) => {
            let database_url = Database::PostgreSQL(connection.clone()).connection_url();
            let db = Arc::new(Postgres::new(&database_url)?);
            db.create_database_tables().await?;
            Ok(build_database_stores(db))
        }
    }
}

#[cfg(test)]
mod tests {
    use torrust_tracker_configuration::v3_0_0::core::Core;
    use torrust_tracker_configuration::v3_0_0::database::Database;

    use super::{initialize_database, initialize_database_from_configuration};
    use crate::authentication::{Key, PeerKey};
    use crate::test_helpers::tests::{ephemeral_configuration, sample_info_hash};

    /// A SQLite configuration whose database file sits in a directory that does not exist, so the
    /// database cannot be opened. The returned directory keeps the path unique to the test.
    fn sqlite_database_in_a_missing_directory() -> (Database, tempfile::TempDir) {
        let directory = tempfile::tempdir().expect("a temporary directory can be created");
        let path = directory.path().join("missing").join("data.db");

        (
            Database::Sqlite3 {
                path: path.to_str().expect("the temporary path is UTF-8").to_string(),
            },
            directory,
        )
    }

    #[tokio::test]
    async fn it_should_initialize_the_sqlite_database() {
        let ephemeral = ephemeral_configuration();
        let _database = initialize_database(&ephemeral.config).await;
    }

    #[tokio::test]
    async fn it_should_hand_the_same_migrated_database_to_all_four_stores() {
        // Arrange
        let ephemeral = ephemeral_configuration();
        let stores = initialize_database(&ephemeral.config).await;
        let key = PeerKey {
            key: Key::new("YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ").unwrap(),
            valid_until: None,
        };

        // Act: every store writes and reads back, so the tables exist
        stores
            .whitelist_store
            .add_info_hash_to_whitelist(sample_info_hash())
            .await
            .unwrap();
        stores.auth_key_store.add_key_to_keys(&key).await.unwrap();
        stores.torrent_metrics_store.save_global_downloads(5).await.unwrap();

        // Assert
        assert_eq!(
            stores.whitelist_store.load_whitelist().await.unwrap(),
            vec![sample_info_hash()]
        );
        assert_eq!(stores.auth_key_store.load_keys().await.unwrap(), vec![key]);
        assert_eq!(stores.torrent_metrics_store.load_global_downloads().await.unwrap(), Some(5));

        // Act: the schema migrator drops the tables of the database it shares with the others
        stores.schema_migrator.drop_database_tables().await.unwrap();

        // Assert
        assert!(
            stores.whitelist_store.load_whitelist().await.is_err(),
            "the schema migrator and the whitelist store must share one database"
        );
    }

    #[tokio::test]
    async fn it_should_return_an_error_instead_of_panicking_when_the_database_cannot_be_opened() {
        let (database, _directory) = sqlite_database_in_a_missing_directory();

        let result = initialize_database_from_configuration(&database).await;

        assert!(result.is_err(), "a database in a missing directory cannot be initialized");
    }

    #[tokio::test]
    #[should_panic(expected = "database configuration is required")]
    async fn the_legacy_factory_should_panic_when_the_configuration_has_no_database() {
        let config = Core {
            database: None,
            ..ephemeral_configuration().config
        };

        let _stores = initialize_database(&config).await;
    }

    #[tokio::test]
    #[should_panic(expected = "database initialization requested")]
    async fn the_legacy_factory_should_panic_when_the_database_cannot_be_opened() {
        let (database, _directory) = sqlite_database_in_a_missing_directory();
        let config = Core {
            database: Some(database),
            ..ephemeral_configuration().config
        };

        let _stores = initialize_database(&config).await;
    }
}
