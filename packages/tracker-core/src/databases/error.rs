//! Database errors.
//!
//! This module defines the [`Error`] enum used to represent errors that occur
//! during database operations. These errors encapsulate issues such as missing
//! query results, malformed queries, connection failures, and connection pool
//! creation errors. Each error variant includes contextual information such as
//! the associated database driver and, when applicable, the source error.
//!
//! External errors from the `sqlx` database library are converted into this
//! error type using the provided `From` implementations. No variant records a
//! source location (see
//! `docs/adrs/20261118045443_errors_do_not_record_source_locations.md`).
use std::sync::Arc;

use sqlx::Error as SqlxError;
use sqlx::migrate::MigrateError;
use torrust_tracker_primitives::Driver;

/// A shareable source error, so that [`Error`] stays `Clone`.
type DynError = Arc<dyn std::error::Error + Send + Sync>;

/// Database error type that encapsulates various failures encountered during
/// database operations.
#[derive(thiserror::Error, Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    /// Indicates that a query unexpectedly returned no rows.
    ///
    /// This error variant is used when a query that is expected to return a
    /// result does not.
    #[error("The {driver} query unexpectedly returned nothing: {source}")]
    QueryReturnedNoRows { source: DynError, driver: Driver },

    /// Indicates that the query was malformed.
    ///
    /// This error variant is used when the SQL query itself is invalid or
    /// improperly formatted.
    #[error("The {driver} query was malformed: {source}")]
    InvalidQuery { source: DynError, driver: Driver },

    /// Indicates a failure to insert a record into the database.
    ///
    /// This error is raised when an insertion operation fails.
    #[error("Unable to insert record into {driver} database")]
    InsertFailed { driver: Driver },

    /// Indicates a failure to update a record into the database.
    ///
    /// This error is raised when an insertion operation fails.
    #[error("Unable to update record into {driver} database")]
    UpdateFailed { driver: Driver },

    /// Indicates a failure to delete a record from the database.
    ///
    /// This error includes an error code that may be returned by the database
    /// driver.
    #[error("Failed to remove record from {driver} database, error-code: {error_code}")]
    DeleteFailed { error_code: usize, driver: Driver },

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
    ConnectionError { source: DynError, driver: Driver },

    /// Indicates a failure while applying schema migrations.
    ///
    /// This error variant wraps `sqlx::migrate::MigrateError`, raised by
    /// `MIGRATOR.run()` (or by the helpers used to bootstrap the
    /// `_sqlx_migrations` tracking table on legacy databases).
    #[error("Failed to apply {driver} schema migrations: {source}")]
    MigrationError { source: DynError, driver: Driver },

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
    fn from(value: (SqlxError, Driver)) -> Self {
        let (err, driver) = value;

        match err {
            SqlxError::RowNotFound => Self::QueryReturnedNoRows {
                source: Arc::new(SqlxError::RowNotFound),
                driver,
            },
            SqlxError::Io(_)
            | SqlxError::Tls(_)
            | SqlxError::PoolTimedOut
            | SqlxError::PoolClosed
            | SqlxError::WorkerCrashed
            | SqlxError::Configuration(_) => Self::ConnectionError {
                source: Arc::new(err),
                driver,
            },
            _ => Self::InvalidQuery {
                source: Arc::new(err),
                driver,
            },
        }
    }
}

impl From<(MigrateError, Driver)> for Error {
    fn from(value: (MigrateError, Driver)) -> Self {
        let (err, driver) = value;

        Self::MigrationError {
            source: Arc::new(err),
            driver,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use rstest::rstest;
    use sqlx::Error as SqlxError;
    use sqlx::migrate::MigrateError;
    use torrust_tracker_primitives::Driver;

    use crate::databases::error::Error;

    /// The name of the variant of `error`, to compare a conversion with a plain string.
    fn variant(error: &Error) -> &'static str {
        match error {
            Error::QueryReturnedNoRows { .. } => "QueryReturnedNoRows",
            Error::InvalidQuery { .. } => "InvalidQuery",
            Error::ConnectionError { .. } => "ConnectionError",
            Error::MigrationError { .. } => "MigrationError",
            _ => "another variant",
        }
    }

    /// The driver a converted error carries.
    fn driver(error: &Error) -> &Driver {
        match error {
            Error::QueryReturnedNoRows { driver, .. }
            | Error::InvalidQuery { driver, .. }
            | Error::ConnectionError { driver, .. }
            | Error::MigrationError { driver, .. } => driver,
            other => panic!("expected a converted error, got {other:?}"),
        }
    }

    #[rstest]
    #[case::row_not_found(SqlxError::RowNotFound, "QueryReturnedNoRows")]
    #[case::io(SqlxError::Io(io::Error::from(io::ErrorKind::ConnectionRefused)), "ConnectionError")]
    #[case::tls(SqlxError::Tls("a tls failure".into()), "ConnectionError")]
    #[case::pool_timed_out(SqlxError::PoolTimedOut, "ConnectionError")]
    #[case::pool_closed(SqlxError::PoolClosed, "ConnectionError")]
    #[case::worker_crashed(SqlxError::WorkerCrashed, "ConnectionError")]
    #[case::configuration(SqlxError::Configuration("a bad option".into()), "ConnectionError")]
    #[case::column_not_found(SqlxError::ColumnNotFound("a_column".to_string()), "InvalidQuery")]
    #[case::decode(SqlxError::Decode("a bad value".into()), "InvalidQuery")]
    #[case::protocol(SqlxError::Protocol("a bad packet".to_string()), "InvalidQuery")]
    fn it_should_classify_each_sqlx_error(#[case] error: SqlxError, #[case] expected: &str) {
        let converted: Error = (error, Driver::Sqlite3).into();

        assert_eq!(variant(&converted), expected);
    }

    #[test]
    fn it_should_classify_a_migration_error() {
        let converted: Error = (MigrateError::VersionMissing(1), Driver::Sqlite3).into();

        assert_eq!(variant(&converted), "MigrationError");
    }

    #[rstest]
    #[case::a_missing_row(Error::from((SqlxError::RowNotFound, Driver::PostgreSQL)))]
    #[case::a_connection_failure(Error::from((SqlxError::PoolClosed, Driver::PostgreSQL)))]
    #[case::an_invalid_query(Error::from((SqlxError::Protocol("a bad packet".to_string()), Driver::PostgreSQL)))]
    #[case::a_migration_failure(Error::from((MigrateError::VersionMissing(1), Driver::PostgreSQL)))]
    fn it_should_keep_the_driver_it_was_converted_with(#[case] converted: Error) {
        assert_eq!(driver(&converted), &Driver::PostgreSQL);
    }

    #[test]
    fn it_should_render_a_sqlx_error_with_its_own_text_and_no_source_path() {
        let err: Error = (sqlx::Error::RowNotFound, Driver::Sqlite3).into();

        assert_eq!(
            err.to_string(),
            "The Sqlite3 query unexpectedly returned nothing: no rows returned by a query that expected to return at least one row"
        );
    }

    #[test]
    fn it_should_render_a_failed_insert_naming_the_driver() {
        let err = Error::InsertFailed { driver: Driver::MySQL };

        assert_eq!(err.to_string(), "Unable to insert record into MySQL database");
    }

    #[test]
    fn it_should_render_a_failed_update_naming_the_driver() {
        let err = Error::UpdateFailed {
            driver: Driver::PostgreSQL,
        };

        assert_eq!(err.to_string(), "Unable to update record into PostgreSQL database");
    }

    #[test]
    fn it_should_render_a_failed_delete_naming_the_driver_and_the_rows_removed() {
        let err = Error::DeleteFailed {
            error_code: 0,
            driver: Driver::Sqlite3,
        };

        assert_eq!(
            err.to_string(),
            "Failed to remove record from Sqlite3 database, error-code: 0"
        );
    }
}
