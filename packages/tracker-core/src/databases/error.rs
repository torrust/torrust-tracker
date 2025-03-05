//! Database errors.
//!
//! This module defines the [`Error`] enum used to represent errors that occur
//! during database operations. These errors encapsulate issues such as missing
//! query results, malformed queries, connection failures, and connection pool
//! creation errors. Each error variant includes contextual information such as
//! the associated database driver and, when applicable, the source error.
//!
//! External errors from database libraries (e.g., `rusqlite`, `mysql`) are
//! converted into this error type using the provided `From` implementations.
use std::panic::Location;
use std::sync::Arc;

use r2d2_mysql::mysql::UrlError;
use torrust_tracker_located_error::{DynError, Located, LocatedError};

use super::driver::Driver;

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

    /// Indicates a failure to connect to the database.
    ///
    /// This error variant wraps connection-related errors, such as those caused by an invalid URL.
    #[error("Failed to connect to {driver} database: {source}")]
    ConnectionError {
        source: LocatedError<'static, UrlError>,
        driver: Driver,
    },

    /// Indicates a failure to create a connection pool.
    ///
    /// This error variant is used when the connection pool creation (using r2d2) fails.
    #[error("Failed to create r2d2 {driver} connection pool: {source}")]
    ConnectionPool {
        source: LocatedError<'static, r2d2::Error>,
        driver: Driver,
    },
}

impl From<r2d2_sqlite::rusqlite::Error> for Error {
    #[track_caller]
    fn from(err: r2d2_sqlite::rusqlite::Error) -> Self {
        match err {
            r2d2_sqlite::rusqlite::Error::QueryReturnedNoRows => Error::QueryReturnedNoRows {
                source: (Arc::new(err) as DynError).into(),
                driver: Driver::Sqlite3,
            },
            _ => Error::InvalidQuery {
                source: (Arc::new(err) as DynError).into(),
                driver: Driver::Sqlite3,
            },
        }
    }
}

impl From<r2d2_mysql::mysql::Error> for Error {
    #[track_caller]
    fn from(err: r2d2_mysql::mysql::Error) -> Self {
        let e: DynError = Arc::new(err);
        Error::InvalidQuery {
            source: e.into(),
            driver: Driver::MySQL,
        }
    }
}

impl From<UrlError> for Error {
    #[track_caller]
    fn from(err: UrlError) -> Self {
        Self::ConnectionError {
            source: Located(err).into(),
            driver: Driver::MySQL,
        }
    }
}

impl From<(r2d2::Error, Driver)> for Error {
    #[track_caller]
    fn from(e: (r2d2::Error, Driver)) -> Self {
        let (err, driver) = e;
        Self::ConnectionPool {
            source: Located(err).into(),
            driver,
        }
    }
}

#[cfg(test)]
mod tests {
    use r2d2_mysql::mysql;

    use crate::databases::error::Error;

    #[test]
    fn it_should_build_a_database_error_from_a_rusqlite_error() {
        let err: Error = r2d2_sqlite::rusqlite::Error::InvalidQuery.into();

        assert!(matches!(err, Error::InvalidQuery { .. }));
    }

    #[test]
    fn it_should_build_an_specific_database_error_from_a_no_rows_returned_rusqlite_error() {
        let err: Error = r2d2_sqlite::rusqlite::Error::QueryReturnedNoRows.into();

        assert!(matches!(err, Error::QueryReturnedNoRows { .. }));
    }

    #[test]
    fn it_should_build_a_database_error_from_a_mysql_error() {
        let url_err = mysql::error::UrlError::BadUrl;
        let err: Error = r2d2_mysql::mysql::Error::UrlError(url_err).into();

        assert!(matches!(err, Error::InvalidQuery { .. }));
    }

    #[test]
    fn it_should_build_a_database_error_from_a_mysql_url_error() {
        let err: Error = mysql::error::UrlError::BadUrl.into();

        assert!(matches!(err, Error::ConnectionError { .. }));
    }
}
