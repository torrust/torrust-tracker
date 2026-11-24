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
use std::borrow::Cow;
use std::sync::Arc;

use sqlx::Error as SqlxError;
use sqlx::error::ErrorKind;
use sqlx::migrate::MigrateError;
use torrust_tracker_primitives::Driver;

/// What is kept of an error a database server raised for a statement that binds a secret: the
/// kind and the code, never the message, which can hold the bound value.
#[derive(Debug, thiserror::Error)]
#[error("the database rejected the statement (kind: {kind}, code: {})", code.as_deref().unwrap_or("none"))]
struct StatementRejected {
    kind: String,
    code: Option<String>,
}

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

    /// Indicates that an authentication key to add is already stored.
    ///
    /// The message does not repeat what the database server answered: the server
    /// text of a duplicate entry holds the key, which is a secret.
    #[error("The {driver} database already holds this authentication key")]
    KeyAlreadyExists { driver: Driver },

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

impl Error {
    /// Converts the `sqlx` error of a statement that binds a secret (the `keys` table).
    ///
    /// The text a database server answers can hold the value that was bound: a duplicate key on
    /// `MySQL` is `Duplicate entry '<key>' for key 'keys.key'`. So the text is never kept. A
    /// unique-constraint violation, found by the driver's error kind and not by its text, becomes
    /// [`Error::KeyAlreadyExists`]; any other error the database raised keeps only its kind and
    /// its code. Errors that do not come from the database server (connection, pool, I/O) hold no
    /// bound value and convert as usual.
    pub(crate) fn from_statement_that_binds_a_secret(err: SqlxError, driver: Driver) -> Self {
        match err {
            SqlxError::Database(server_error) => {
                if matches!(server_error.kind(), ErrorKind::UniqueViolation) {
                    Self::KeyAlreadyExists { driver }
                } else {
                    Self::InvalidQuery {
                        source: Arc::new(StatementRejected {
                            kind: format!("{:?}", server_error.kind()),
                            code: server_error.code().map(Cow::into_owned),
                        }),
                        driver,
                    }
                }
            }
            other => Self::from((other, driver)),
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
    use std::borrow::Cow;
    use std::{fmt, io};

    use rstest::rstest;
    use sqlx::Error as SqlxError;
    use sqlx::migrate::MigrateError;
    use torrust_tracker_primitives::Driver;

    use crate::databases::error::Error;
    use crate::test_helpers::tests::assert_holds_no_secret;

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

    /// A database server error whose message holds the value that was written, like `MySQL`'s
    /// duplicate entry. Test values only.
    #[derive(Debug)]
    struct ServerError {
        message: String,
        unique_violation: bool,
        code: Option<&'static str>,
    }

    impl fmt::Display for ServerError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(&self.message)
        }
    }

    impl std::error::Error for ServerError {}

    impl sqlx::error::DatabaseError for ServerError {
        fn message(&self) -> &str {
            &self.message
        }

        fn code(&self) -> Option<Cow<'_, str>> {
            self.code.map(Cow::Borrowed)
        }

        fn as_error(&self) -> &(dyn std::error::Error + Send + Sync + 'static) {
            self
        }

        fn as_error_mut(&mut self) -> &mut (dyn std::error::Error + Send + Sync + 'static) {
            self
        }

        fn into_error(self: Box<Self>) -> Box<dyn std::error::Error + Send + Sync + 'static> {
            self
        }

        fn kind(&self) -> sqlx::error::ErrorKind {
            if self.unique_violation {
                sqlx::error::ErrorKind::UniqueViolation
            } else {
                sqlx::error::ErrorKind::CheckViolation
            }
        }
    }

    fn a_server_error(secret: &str, unique_violation: bool, code: Option<&'static str>) -> SqlxError {
        SqlxError::Database(Box::new(ServerError {
            message: format!("Duplicate entry '{secret}' for key 'keys.key'"),
            unique_violation,
            code,
        }))
    }

    #[test]
    fn it_should_turn_a_unique_violation_on_a_statement_that_binds_a_secret_into_a_key_already_exists_error() {
        // Arrange
        let secret = "TestPeerKeyInTheServerMessage001";
        let duplicate = a_server_error(secret, true, Some("23000"));

        // Act
        let converted = Error::from_statement_that_binds_a_secret(duplicate, Driver::MySQL);

        // Assert
        assert!(matches!(converted, Error::KeyAlreadyExists { .. }), "{converted:?}");
        assert_holds_no_secret(&converted, secret);
        assert_eq!(
            converted.to_string(),
            "The MySQL database already holds this authentication key"
        );
    }

    #[test]
    fn it_should_keep_only_the_kind_and_the_code_of_any_other_server_error_on_a_statement_that_binds_a_secret() {
        // Arrange
        let secret = "TestPeerKeyInTheOtherMessage0001";
        let other = a_server_error(secret, false, Some("HY000"));

        // Act
        let converted = Error::from_statement_that_binds_a_secret(other, Driver::MySQL);

        // Assert
        assert_holds_no_secret(&converted, secret);
        let shown = converted.to_string();
        assert!(
            shown.contains("CheckViolation") && shown.contains("HY000"),
            "the kind and the code should stay: {shown}"
        );
    }

    #[test]
    fn it_should_convert_an_error_that_does_not_come_from_the_server_as_usual_on_a_statement_that_binds_a_secret() {
        let converted = Error::from_statement_that_binds_a_secret(SqlxError::PoolClosed, Driver::PostgreSQL);

        assert_eq!(variant(&converted), "ConnectionError");
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
