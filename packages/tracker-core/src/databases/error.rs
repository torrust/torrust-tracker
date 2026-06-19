//! Database errors.
//!
//! This module defines the [`Error`] enum used to represent errors that occur
//! during database operations. These errors encapsulate issues such as missing
//! query results, malformed queries, connection failures, and connection pool
//! creation errors. Each error variant includes contextual information such as
//! the associated database driver and, when applicable, the source error.
//!
//! External errors from the `sqlx` database library are converted into this
//! error type using the provided `From` implementations.
use std::panic::Location;
use std::sync::Arc;

use sqlx::Error as SqlxError;
use sqlx::migrate::MigrateError;
use torrust_located_error::{DynError, LocatedError};
use torrust_tracker_primitives::Driver;

/// Database error type that encapsulates various failures encountered during
/// database operations.
#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    /// Indicates that a query unexpectedly returned no rows.
    ///
    /// This error variant is used when a query that is expected to return a
    /// result does not.
    #[error("The {driver} query unexpectedly returned nothing: {source}")]
    QueryReturnedNoRows {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
        driver: Driver,
    },

    /// Indicates that the query was malformed.
    ///
    /// This error variant is used when the SQL query itself is invalid or
    /// improperly formatted.
    #[error("The {driver} query was malformed: {source}")]
    InvalidQuery {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
        driver: Driver,
    },

    /// Indicates a failure to insert a record into the database.
    ///
    /// This error is raised when an insertion operation fails.
    #[error("Unable to insert record into {driver} database, {location}")]
    InsertFailed {
        location: &'static Location<'static>,
        driver: Driver,
    },

    /// Indicates a failure to update a record into the database.
    ///
    /// This error is raised when an insertion operation fails.
    #[error("Unable to update record into {driver} database, {location}")]
    UpdateFailed {
        location: &'static Location<'static>,
        driver: Driver,
    },

    /// Indicates a failure to delete a record from the database.
    ///
    /// This error includes an error code that may be returned by the database
    /// driver.
    #[error("Failed to remove record from {driver} database, error-code: {error_code}, {location}")]
    DeleteFailed {
        location: &'static Location<'static>,
        error_code: usize,
        driver: Driver,
    },

    /// Indicates that a row read from the database contains a malformed value
    /// (e.g., a corrupt or manually-edited `info_hash` or key string that
    /// cannot be parsed into the expected domain type).
    #[error("Malformed {driver} database record: {message}")]
    MalformedDatabaseRecord { message: String, driver: Driver },

    /// Indicates a failure to connect to the database.
    ///
    /// This error variant wraps connection-related errors, such as pool
    /// timeouts, TLS failures, or invalid URL errors.
    #[error("Failed to connect to {driver} database: {source}")]
    ConnectionError {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
        driver: Driver,
    },

    /// Indicates a failure while applying schema migrations.
    ///
    /// This error variant wraps `sqlx::migrate::MigrateError`, raised by
    /// `MIGRATOR.run()` (or by the helpers used to bootstrap the
    /// `_sqlx_migrations` tracking table on legacy databases).
    #[error("Failed to apply {driver} schema migrations: {source}")]
    MigrationError {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
        driver: Driver,
    },

    /// Indicates that a pre-v4 database is in a partially-migrated state and
    /// cannot be auto-bootstrapped into the `sqlx` migration system.
    ///
    /// Raised by the legacy-bootstrap path of `create_database_tables()` when
    /// some — but not all — of the expected legacy tables are present and the
    /// `_sqlx_migrations` table does not yet exist. The fix is to apply the
    /// missing manual migrations before upgrading.
    #[error("Cannot upgrade {driver} database: {reason}")]
    LegacyDatabaseNotMigrated { reason: String, driver: Driver },
}

impl From<(SqlxError, Driver)> for Error {
    #[track_caller]
    fn from(value: (SqlxError, Driver)) -> Self {
        let (err, driver) = value;

        match err {
            SqlxError::RowNotFound => Self::QueryReturnedNoRows {
                source: (Arc::new(SqlxError::RowNotFound) as DynError).into(),
                driver,
            },
            SqlxError::Io(_)
            | SqlxError::Tls(_)
            | SqlxError::PoolTimedOut
            | SqlxError::PoolClosed
            | SqlxError::WorkerCrashed
            | SqlxError::Configuration(_) => Self::ConnectionError {
                source: (Arc::new(err) as DynError).into(),
                driver,
            },
            _ => Self::InvalidQuery {
                source: (Arc::new(err) as DynError).into(),
                driver,
            },
        }
    }
}

impl From<(MigrateError, Driver)> for Error {
    #[track_caller]
    fn from(value: (MigrateError, Driver)) -> Self {
        let (err, driver) = value;

        Self::MigrationError {
            source: (Arc::new(err) as DynError).into(),
            driver,
        }
    }
}

#[cfg(test)]
mod tests {
    use torrust_tracker_primitives::Driver;

    use crate::databases::error::Error;

    #[test]
    fn it_should_build_a_database_error_from_a_sqlx_row_not_found_error() {
        let err: Error = (sqlx::Error::RowNotFound, Driver::Sqlite3).into();

        assert!(matches!(err, Error::QueryReturnedNoRows { .. }));
    }

    #[test]
    fn it_should_build_a_database_error_from_a_sqlx_io_error() {
        use std::io;

        let err: Error = (
            sqlx::Error::Io(io::Error::from(io::ErrorKind::ConnectionRefused)),
            Driver::MySQL,
        )
            .into();

        assert!(matches!(err, Error::ConnectionError { .. }));
    }
}
