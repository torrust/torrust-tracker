use serde::{Deserialize, Serialize};

use super::error::Error;
use super::mysql::Mysql;
use super::settings::Settings;
use super::sqlite::Sqlite;
use super::{Builder, Database};

#[derive(Default, Serialize, Deserialize, Hash, PartialEq, PartialOrd, Ord, Eq, Copy, Debug, Clone)]
pub enum Driver {
    #[default]
    Sqlite3,
    MySQL,
}

impl Driver {
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if unable to connect to the database.
    pub fn build(settings: &Settings) -> Result<Box<dyn Database>, Error> {
        let database = match settings.driver {
            Driver::Sqlite3 => Builder::<Sqlite>::build(settings),
            Driver::MySQL => Builder::<Mysql>::build(settings),
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
