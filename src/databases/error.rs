use std::panic::Location;
use std::sync::Arc;

use r2d2_mysql::mysql::UrlError;

use super::driver::Driver;
use crate::located_error::{Located, LocatedError};

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("The {driver} query unexpectedly returned nothing: {source}")]
    QueryReturnedNoRows {
        source: LocatedError<'static, dyn std::error::Error>,
        driver: Driver,
    },

    #[error("The {driver} query was malformed: {source}")]
    InvalidQuery {
        source: LocatedError<'static, dyn std::error::Error>,
        driver: Driver,
    },

    #[error("Unable to insert record into {driver} database, {location}")]
    InsertFailed {
        location: &'static Location<'static>,
        driver: Driver,
    },

    #[error("Failed to remove record from {driver} database, error-code: {error_code}, {location}")]
    DeleteFailed {
        location: &'static Location<'static>,
        error_code: usize,
        driver: Driver,
    },

    #[error("Failed to connect to {driver} database: {source}")]
    ConnectionError {
        source: LocatedError<'static, UrlError>,
        driver: Driver,
    },

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
                source: (Arc::new(err) as Arc<dyn std::error::Error>).into(),
                driver: Driver::Sqlite3,
            },
            _ => Error::InvalidQuery {
                source: (Arc::new(err) as Arc<dyn std::error::Error>).into(),
                driver: Driver::Sqlite3,
            },
        }
    }
}

impl From<r2d2_mysql::mysql::Error> for Error {
    #[track_caller]
    fn from(err: r2d2_mysql::mysql::Error) -> Self {
        let e: Arc<dyn std::error::Error> = Arc::new(err);
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
