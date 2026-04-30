//! The [`SchemaMigrator`] trait — schema management context.
use async_trait::async_trait;
use mockall::automock;

use super::super::error::Error;

/// Trait covering schema lifecycle operations for a database driver.
///
/// Implementors are responsible for creating and dropping the full set of
/// database tables used by the tracker.
#[async_trait]
#[automock]
pub trait SchemaMigrator: Sync + Send {
    /// Creates the necessary database tables.
    ///
    /// The SQL queries for table creation are hardcoded in the trait implementation.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the tables cannot be created.
    async fn create_database_tables(&self) -> Result<(), Error>;

    /// Drops the database tables.
    ///
    /// This operation removes the persistent schema.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the tables cannot be dropped.
    async fn drop_database_tables(&self) -> Result<(), Error>;
}
