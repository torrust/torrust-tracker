//! Database driver types.
//!
//! This module defines the [`Driver`] enum which identifies the database
//! management system used by the tracker. It is a cross-cutting domain
//! concept shared by configuration deserialization, database initialization,
//! and CLI tooling.

use std::str::FromStr;

use derive_more::Display;
use serde::{Deserialize, Serialize};

/// The database management system used by the tracker.
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Display, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Driver {
    /// The `Sqlite3` database driver.
    Sqlite3,
    /// The `MySQL` database driver.
    MySQL,
    /// The `PostgreSQL` database driver.
    PostgreSQL,
}

impl Driver {
    /// Returns the stable lowercase identifier used by CLI and reports.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sqlite3 => "sqlite3",
            Self::MySQL => "mysql",
            Self::PostgreSQL => "postgresql",
        }
    }
}

impl FromStr for Driver {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "sqlite3" => Ok(Self::Sqlite3),
            "mysql" => Ok(Self::MySQL),
            "postgresql" => Ok(Self::PostgreSQL),
            _ => Err("driver must be one of: sqlite3, mysql, postgresql".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Driver;

    #[test]
    fn it_should_display_sqlite3() {
        assert_eq!(Driver::Sqlite3.to_string(), "Sqlite3");
    }

    #[test]
    fn it_should_display_mysql() {
        assert_eq!(Driver::MySQL.to_string(), "MySQL");
    }

    #[test]
    fn it_should_display_postgresql() {
        assert_eq!(Driver::PostgreSQL.to_string(), "PostgreSQL");
    }

    #[test]
    fn it_should_return_as_str_sqlite3() {
        assert_eq!(Driver::Sqlite3.as_str(), "sqlite3");
    }

    #[test]
    fn it_should_return_as_str_mysql() {
        assert_eq!(Driver::MySQL.as_str(), "mysql");
    }

    #[test]
    fn it_should_return_as_str_postgresql() {
        assert_eq!(Driver::PostgreSQL.as_str(), "postgresql");
    }

    #[test]
    fn it_should_parse_sqlite3() {
        let driver: Result<Driver, _> = "sqlite3".parse();
        assert_eq!(driver.unwrap(), Driver::Sqlite3);
    }

    #[test]
    fn it_should_parse_mysql() {
        let driver: Result<Driver, _> = "mysql".parse();
        assert_eq!(driver.unwrap(), Driver::MySQL);
    }

    #[test]
    fn it_should_parse_postgresql() {
        let driver: Result<Driver, _> = "postgresql".parse();
        assert_eq!(driver.unwrap(), Driver::PostgreSQL);
    }

    #[test]
    fn it_should_fail_parsing_invalid_driver() {
        let driver: Result<Driver, _> = "invalid".parse();
        assert!(driver.is_err());
    }

    #[test]
    fn it_should_serialize_sqlite3_to_lowercase() {
        let serialized = serde_json::to_string(&Driver::Sqlite3).unwrap();
        assert_eq!(serialized, "\"sqlite3\"");
    }

    #[test]
    fn it_should_serialize_mysql_to_lowercase() {
        let serialized = serde_json::to_string(&Driver::MySQL).unwrap();
        assert_eq!(serialized, "\"mysql\"");
    }

    #[test]
    fn it_should_serialize_postgresql_to_lowercase() {
        let serialized = serde_json::to_string(&Driver::PostgreSQL).unwrap();
        assert_eq!(serialized, "\"postgresql\"");
    }
}
