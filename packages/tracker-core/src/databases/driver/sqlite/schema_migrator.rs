use async_trait::async_trait;
use sqlx::migrate::Migrate;
use sqlx::SqlitePool;

use super::{Sqlite, DRIVER, MIGRATOR};
use crate::databases::error::Error;
use crate::databases::SchemaMigrator;

/// The four tables created by the three pre-v4 manual migrations.
///
/// A legacy database has either zero of these tables (fresh install) or all
/// four (fully-migrated pre-v4). Any in-between state means the user did not
/// apply every required manual migration before upgrading and is rejected by
/// [`bootstrap_legacy_schema`].
///
/// # Legacy compatibility
///
/// This constant — together with [`LAST_LEGACY_MIGRATION_VERSION`] and the
/// [`bootstrap_legacy_schema`] free function — exists only to support
/// in-place upgrades from pre-v4 deployments that managed their schema
/// outside `sqlx::migrate!`. Once the project drops support for those
/// installations, this entire compatibility layer (constants, free function
/// and the `bootstrap_legacy_schema(...)` call inside `create_database_tables`)
/// can be removed, leaving a clean migrator-only implementation.
const LEGACY_TABLES: &[&str] = &["whitelist", "torrents", "keys", "torrent_aggregate_metrics"];

/// Highest timestamp among the three pre-v4 manual migrations. Migrations at
/// or below this version are fake-applied for legacy databases.
///
/// See the legacy-compatibility note on [`LEGACY_TABLES`] — this constant is
/// part of the same removable layer.
const LAST_LEGACY_MIGRATION_VERSION: i64 = 20_250_527_093_000;

#[async_trait]
impl SchemaMigrator for Sqlite {
    async fn create_database_tables(&self) -> Result<(), Error> {
        bootstrap_legacy_schema(&self.pool).await?;
        MIGRATOR.run(&self.pool).await.map_err(|e| (e, DRIVER))?;
        Ok(())
    }

    async fn drop_database_tables(&self) -> Result<(), Error> {
        // `IF EXISTS` keeps test teardown safe across partial schemas.
        // `_sqlx_migrations` is created by the embedded `sqlx` migrator and
        // must be dropped here so the next `create_database_tables()` call
        // re-applies every migration from a clean state.
        let statements = [
            "DROP TABLE IF EXISTS _sqlx_migrations;",
            "DROP TABLE IF EXISTS torrent_aggregate_metrics;",
            "DROP TABLE IF EXISTS whitelist;",
            "DROP TABLE IF EXISTS torrents;",
            "DROP TABLE IF EXISTS keys;",
        ];

        for stmt in statements {
            ::sqlx::query(stmt).execute(&self.pool).await.map_err(|e| (e, DRIVER))?;
        }

        Ok(())
    }
}

/// Detect a pre-v4 `SQLite` database (user-managed schema, no
/// `_sqlx_migrations` table) and seed the migration history so that
/// [`MIGRATOR.run()`] can continue with only the new migrations.
///
/// # Legacy compatibility
///
/// This function and its supporting constants ([`LEGACY_TABLES`],
/// [`LAST_LEGACY_MIGRATION_VERSION`]) exist only to make in-place upgrades
/// from pre-v4 deployments work transparently. Pre-v4 trackers managed their
/// schema with hand-written `CREATE TABLE` statements instead of
/// `sqlx::migrate!`, so on first start under v4 the database has the legacy
/// tables but no `_sqlx_migrations` row — running the migrator directly
/// would fail with "table already exists".
///
/// When the project drops support for upgrading from pre-v4 trackers, the
/// entire compatibility layer can be deleted in one change:
///
/// 1. Delete this function.
/// 2. Delete [`LEGACY_TABLES`] and [`LAST_LEGACY_MIGRATION_VERSION`].
/// 3. Remove the `bootstrap_legacy_schema(&self.pool).await?;` call from
///    [`SchemaMigrator::create_database_tables`].
/// 4. Delete the legacy-bootstrap tests in the `tests` submodule.
async fn bootstrap_legacy_schema(pool: &SqlitePool) -> Result<(), Error> {
    let migrations_table_exists: bool =
        ::sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = '_sqlx_migrations'")
            .fetch_one(pool)
            .await
            .map_err(|e| (e, DRIVER))?
            > 0;

    if migrations_table_exists {
        return Ok(());
    }

    let placeholders = vec!["?"; LEGACY_TABLES.len()].join(", ");
    let count_query = format!("SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name IN ({placeholders})");
    let mut count_stmt = ::sqlx::query_scalar::<_, i64>(&count_query);
    for table in LEGACY_TABLES {
        count_stmt = count_stmt.bind(*table);
    }
    let present_legacy_tables = usize::try_from(count_stmt.fetch_one(pool).await.map_err(|e| (e, DRIVER))?).unwrap_or(0);

    if present_legacy_tables == 0 {
        return Ok(());
    }

    if present_legacy_tables < LEGACY_TABLES.len() {
        return Err(Error::LegacyDatabaseNotMigrated {
            reason: format!(
                "expected all of [{}] to exist after the legacy manual migrations, found only {} of {} tables; \
                 apply every pre-v4 migration before upgrading",
                LEGACY_TABLES.join(", "),
                present_legacy_tables,
                LEGACY_TABLES.len()
            ),
            driver: DRIVER,
        });
    }

    let mut conn = pool.acquire().await.map_err(|e| (e, DRIVER))?;
    conn.ensure_migrations_table().await.map_err(|e| (e, DRIVER))?;
    drop(conn);

    for migration in MIGRATOR.iter() {
        let version: i64 = migration.version;
        if version > LAST_LEGACY_MIGRATION_VERSION {
            continue;
        }

        let already_recorded: bool = ::sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM _sqlx_migrations WHERE version = ?")
            .bind(version)
            .fetch_one(pool)
            .await
            .map_err(|e| (e, DRIVER))?
            > 0;
        if already_recorded {
            continue;
        }

        ::sqlx::query(
            "INSERT INTO _sqlx_migrations \
             (version, description, installed_on, success, checksum, execution_time) \
             VALUES (?, ?, CURRENT_TIMESTAMP, TRUE, ?, 0)",
        )
        .bind(version)
        .bind(migration.description.as_ref())
        .bind(migration.checksum.as_ref())
        .execute(pool)
        .await
        .map_err(|e| (e, DRIVER))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use ::sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use ::sqlx::SqlitePool;
    use torrust_tracker_test_helpers::configuration::ephemeral_sqlite_database;

    use super::{bootstrap_legacy_schema, LEGACY_TABLES};
    use crate::databases::driver::sqlite::Sqlite;
    use crate::databases::error::Error;
    use crate::databases::SchemaMigrator;

    /// Connect to a fresh on-disk ephemeral `SQLite` database. We use a real
    /// file (not `:memory:`) so the same connection pool used by `Sqlite`
    /// observes tables created via the helper pool below.
    ///
    /// Build the pool through [`SqliteConnectOptions::filename`] (mirroring
    /// `Sqlite::new`) so the filesystem path is handled by `sqlx` directly
    /// instead of being string-formatted into a `sqlite://` URL — that keeps
    /// non-UTF-8 and Windows paths working.
    async fn new_pool() -> (SqlitePool, PathBuf) {
        let path = ephemeral_sqlite_database();
        let options = SqliteConnectOptions::new().filename(&path).create_if_missing(true);
        let pool = SqlitePoolOptions::new()
            .connect_with(options)
            .await
            .expect("connect to sqlite");
        (pool, path)
    }

    fn driver(path: &std::path::Path) -> Sqlite {
        Sqlite::new(path.to_str().expect("ephemeral path is utf-8 in tests")).unwrap()
    }

    /// Recreate the schema produced by the three pre-v4 manual migrations.
    ///
    /// This raw DDL mirrors the cumulative state of
    /// `migrations/sqlite/2024073018*.sql` and
    /// `migrations/sqlite/20250527093000_*.sql` after they have been applied
    /// in order. We build it by hand so the legacy-bootstrap tests can
    /// build a database that looks exactly like a pre-v4 tracker on disk
    /// (legacy tables present, no `_sqlx_migrations` row).
    ///
    /// # Legacy compatibility
    ///
    /// Drop this helper at the same time as [`bootstrap_legacy_schema`] —
    /// see the legacy-compatibility note on that function.
    async fn create_legacy_pre_v4_schema(pool: &SqlitePool) {
        for stmt in [
            "CREATE TABLE whitelist (id INTEGER PRIMARY KEY AUTOINCREMENT, info_hash TEXT NOT NULL UNIQUE);",
            "CREATE TABLE torrents (id INTEGER PRIMARY KEY AUTOINCREMENT, info_hash TEXT NOT NULL UNIQUE, completed INTEGER DEFAULT 0 NOT NULL);",
            "CREATE TABLE keys (id INTEGER PRIMARY KEY AUTOINCREMENT, key TEXT NOT NULL UNIQUE, valid_until INTEGER);",
            "CREATE TABLE torrent_aggregate_metrics (id INTEGER PRIMARY KEY AUTOINCREMENT, metric_name TEXT NOT NULL UNIQUE, value INTEGER DEFAULT 0 NOT NULL);",
        ] {
            ::sqlx::query(stmt).execute(pool).await.unwrap();
        }
    }

    #[tokio::test]
    async fn bootstrap_legacy_schema_should_be_a_noop_on_a_fresh_database() {
        let (pool, _path) = new_pool().await;

        bootstrap_legacy_schema(&pool).await.expect("noop on empty db");

        // No `_sqlx_migrations` row should be inserted yet — the regular
        // migrator path will create the table when it runs.
        let count: i64 =
            ::sqlx::query_scalar("SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = '_sqlx_migrations'")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn bootstrap_legacy_schema_should_seed_history_when_all_legacy_tables_exist() {
        let (pool, path) = new_pool().await;

        create_legacy_pre_v4_schema(&pool).await;

        bootstrap_legacy_schema(&pool).await.expect("legacy bootstrap should succeed");

        let recorded: i64 = ::sqlx::query_scalar("SELECT COUNT(*) FROM _sqlx_migrations")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(recorded, 3, "all three legacy migrations should be fake-applied");

        // A subsequent full migrator run on the driver must be a no-op (no
        // checksum errors, no duplicate-table errors).
        let driver = driver(&path);
        driver
            .create_database_tables()
            .await
            .expect("migrator run should be a no-op after bootstrap");
    }

    #[tokio::test]
    async fn bootstrap_legacy_schema_should_reject_partial_legacy_state() {
        let (pool, _path) = new_pool().await;

        // Only two of the four legacy tables exist.
        ::sqlx::query("CREATE TABLE whitelist (id INTEGER PRIMARY KEY);")
            .execute(&pool)
            .await
            .unwrap();
        ::sqlx::query("CREATE TABLE torrents (id INTEGER PRIMARY KEY);")
            .execute(&pool)
            .await
            .unwrap();

        let err = bootstrap_legacy_schema(&pool).await.expect_err("partial state must fail");
        match err {
            Error::LegacyDatabaseNotMigrated { reason, .. } => {
                assert!(reason.contains("apply every pre-v4 migration"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
        // Sanity: list is referenced so that future schema changes update both
        // sides of the precondition.
        assert_eq!(LEGACY_TABLES.len(), 4);
    }
}
