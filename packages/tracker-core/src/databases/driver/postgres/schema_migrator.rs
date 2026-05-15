use async_trait::async_trait;

use super::{DRIVER, MIGRATOR, Postgres};
use crate::databases::SchemaMigrator;
use crate::databases::error::Error;

#[async_trait]
impl SchemaMigrator for Postgres {
    async fn create_database_tables(&self) -> Result<(), Error> {
        // `PostgreSQL` has no pre-v4 databases, so we skip legacy bootstrap
        // and run the embedded migrator directly.
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
