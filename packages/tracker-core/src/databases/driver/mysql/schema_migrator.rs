use async_trait::async_trait;
use sqlx::MySqlPool;
use sqlx::migrate::Migrate;

use super::{DRIVER, MIGRATOR, Mysql};
use crate::databases::SchemaMigrator;
use crate::databases::error::Error;

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
impl SchemaMigrator for Mysql {
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
            "DROP TABLE IF EXISTS `_sqlx_migrations`;",
            "DROP TABLE IF EXISTS `torrent_aggregate_metrics`;",
            "DROP TABLE IF EXISTS `whitelist`;",
            "DROP TABLE IF EXISTS `torrents`;",
            "DROP TABLE IF EXISTS `keys`;",
        ];

        for stmt in statements {
            ::sqlx::query(stmt).execute(&self.pool).await.map_err(|e| (e, DRIVER))?;
        }

        Ok(())
    }
}

/// Detect a pre-v4 `MySQL` database (user-managed schema, no
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
/// 4. Delete the legacy-bootstrap test paths in `mysql/mod.rs`.
async fn bootstrap_legacy_schema(pool: &MySqlPool) -> Result<(), Error> {
    let migrations_table_exists: bool = ::sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM information_schema.tables \
         WHERE table_schema = DATABASE() AND table_name = '_sqlx_migrations'",
    )
    .fetch_one(pool)
    .await
    .map_err(|e| (e, DRIVER))?
        > 0;

    if migrations_table_exists {
        return Ok(());
    }

    let placeholders = vec!["?"; LEGACY_TABLES.len()].join(", ");
    let count_query = format!(
        "SELECT COUNT(*) FROM information_schema.tables \
         WHERE table_schema = DATABASE() AND table_name IN ({placeholders})"
    );
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
