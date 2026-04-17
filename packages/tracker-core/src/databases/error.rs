//! Database errors.
use std::panic::Location;
use std::sync::Arc;

use sqlx::Error as SqlxError;
use torrust_tracker_located_error::{DynError, LocatedError};

use super::driver::Driver;

/// Database error type that encapsulates various failures encountered during
/// persistence operations.
#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    /// Indicates that a query unexpectedly returned no rows.
    #[error("The {driver} query unexpectedly returned nothing: {source}")]
    QueryReturnedNoRows {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
        driver: Driver,
    },

    /// Indicates that the query was malformed or its returned data could not be decoded.
    #[error("The {driver} query was malformed: {source}")]
    InvalidQuery {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
        driver: Driver,
    },

    /// Indicates a failure to insert a record into the database.
    #[error("Unable to insert record into {driver} database, {location}")]
    InsertFailed {
        location: &'static Location<'static>,
        driver: Driver,
    },

    /// Indicates a failure to update a record into the database.
    #[error("Unable to update record into {driver} database, {location}")]
    UpdateFailed {
        location: &'static Location<'static>,
        driver: Driver,
    },

    /// Indicates a failure to delete a record from the database.
    #[error("Failed to remove record from {driver} database, error-code: {error_code}, {location}")]
    DeleteFailed {
        location: &'static Location<'static>,
        error_code: usize,
        driver: Driver,
    },

    /// Indicates a failure to connect to the database.
    #[error("Failed to connect to {driver} database: {source}")]
    ConnectionError {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
        driver: Driver,
    },

    /// Indicates a failure while running migrations.
    #[error("Failed to run {driver} database migrations: {source}")]
    MigrationError {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
        driver: Driver,
    },
}

impl Error {
    /// Builds an invalid query error.
    #[track_caller]
    pub fn invalid_query<E>(driver: Driver, err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::InvalidQuery {
            source: (Arc::new(err) as DynError).into(),
            driver,
        }
    }

    /// Builds a query returned no rows error.
    #[track_caller]
    pub fn query_returned_no_rows<E>(driver: Driver, err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::QueryReturnedNoRows {
            source: (Arc::new(err) as DynError).into(),
            driver,
        }
    }

    /// Builds a connection error.
    #[track_caller]
    pub fn connection_error<E>(driver: Driver, err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::ConnectionError {
            source: (Arc::new(err) as DynError).into(),
            driver,
        }
    }

    /// Builds a migration error.
    #[track_caller]
    pub fn migration_error<E>(driver: Driver, err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::MigrationError {
            source: (Arc::new(err) as DynError).into(),
            driver,
        }
    }
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

impl From<(DynError, Driver)> for Error {
    #[track_caller]
    fn from(value: (DynError, Driver)) -> Self {
        let (err, driver) = value;

        Self::ConnectionError {
            source: err.into(),
            driver,
        }
    }
}

impl From<(LocatedError<'static, dyn std::error::Error + Send + Sync>, Driver)> for Error {
    #[track_caller]
    fn from(value: (LocatedError<'static, dyn std::error::Error + Send + Sync>, Driver)) -> Self {
        let (source, driver) = value;

        Self::ConnectionError { source, driver }
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::sync::Arc;

    use super::Error;
    use crate::databases::driver::Driver;
    use torrust_tracker_located_error::DynError;

    #[test]
    fn it_should_build_a_database_error_from_a_sqlx_row_not_found_error() {
        let err: Error = (sqlx::Error::RowNotFound, Driver::Sqlite3).into();

        assert!(matches!(err, Error::QueryReturnedNoRows { .. }));
    }

    #[test]
    fn it_should_build_a_database_error_from_a_sqlx_io_error() {
        let err: Error = (
            sqlx::Error::Io(io::Error::from(io::ErrorKind::ConnectionRefused)),
            Driver::MySQL,
        )
            .into();

        assert!(matches!(err, Error::ConnectionError { .. }));
    }

    #[test]
    fn it_should_build_a_database_error_from_a_dyn_connection_error() {
        let err: Error = (
            (Arc::new(io::Error::from(io::ErrorKind::TimedOut)) as DynError),
            Driver::PostgreSQL,
        )
            .into();

        assert!(matches!(err, Error::ConnectionError { .. }));
    }

    #[test]
    fn it_should_build_a_migration_error() {
        let err = Error::migration_error(Driver::Sqlite3, io::Error::from(io::ErrorKind::InvalidData));

        assert!(matches!(err, Error::MigrationError { .. }));
    }

    #[test]
    fn it_should_build_an_invalid_query_error() {
        let err = Error::invalid_query(Driver::MySQL, io::Error::from(io::ErrorKind::InvalidData));

        assert!(matches!(err, Error::InvalidQuery { .. }));
    }

    #[test]
    fn it_should_build_a_located_connection_error() {
        let err = Error::ConnectionError {
            source: (Arc::new(io::Error::from(io::ErrorKind::TimedOut)) as DynError).into(),
            driver: Driver::Sqlite3,
        };

        assert!(matches!(err, Error::ConnectionError { .. }));
    }
}
