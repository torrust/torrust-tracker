//! The [`AsyncSchemaMigrator`] trait — schema management context.
use async_trait::async_trait;

use crate::databases::error::Error;

/// Trait covering async schema lifecycle operations for a database driver.
#[async_trait]
pub trait AsyncSchemaMigrator: Send + Sync {
    /// Creates the necessary database tables.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the tables cannot be created.
    async fn create_database_tables(&self) -> Result<(), Error>;

    /// Drops the database tables.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the tables cannot be dropped.
    async fn drop_database_tables(&self) -> Result<(), Error>;
}
