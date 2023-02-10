use serde::{Deserialize, Serialize};

use super::error::Error;
use super::mysql::Mysql;
use super::sqlite::Sqlite;
use super::{Builder, Database};

#[derive(Serialize, Deserialize, Hash, PartialEq, PartialOrd, Ord, Eq, Copy, Debug, Clone)]
pub enum Driver {
    Sqlite3,
    MySQL,
}

impl Driver {
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if unable to connect to the database.
    pub fn build(&self, db_path: &str) -> Result<Box<dyn Database>, Error> {
        let database = match self {
            Driver::Sqlite3 => Builder::<Sqlite>::build(db_path),
            Driver::MySQL => Builder::<Mysql>::build(db_path),
        }?;

        database.create_database_tables().expect("Could not create database tables.");

        Ok(database)
    }
}

impl std::fmt::Display for Driver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sqlite3 => write!(f, "sqllite3"),
            Self::MySQL => write!(f, "my_sql"),
        }
    }
}

impl Default for Driver {
    fn default() -> Self {
        Driver::Sqlite3
    }
}
