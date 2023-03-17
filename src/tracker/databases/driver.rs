use torrust_tracker_primitives::DatabaseDriver;

use super::error::Error;
use super::mysql::Mysql;
use super::sqlite::Sqlite;
use super::{Builder, Database};

/// .
///
/// # Errors
///
/// This function will return an error if unable to connect to the database.
pub fn build(driver: &DatabaseDriver, db_path: &str) -> Result<Box<dyn Database>, Error> {
    let database = match driver {
        DatabaseDriver::Sqlite3 => Builder::<Sqlite>::build(db_path),
        DatabaseDriver::MySQL => Builder::<Mysql>::build(db_path),
    }?;

    database.create_database_tables().expect("Could not create database tables.");

    Ok(database)
}
