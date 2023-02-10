use std::panic::Location;
use std::path::Path;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::driver::Driver::{self, Sqlite3};
use super::driver::{self};
use super::error::Error;
use super::{mysql, sqlite};

#[derive(Builder, Default, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Hash)]
#[builder(default, pattern = "immutable")]
pub struct Settings {
    #[builder(default = "driver::Driver::default()")]
    pub driver: driver::Driver,
    #[builder(default = "self.sql_lite_path_default()")]
    sql_lite_3_db_file_path: Option<Box<Path>>,
    my_sql_connection_url: Option<String>,
}

impl SettingsBuilder {
    // Private helper method that will set the default database path if the database is Sqlite.
    #[allow(clippy::unused_self)]
    fn sql_lite_path_default(&self) -> Option<Box<Path>> {
        if let Sqlite3 = driver::Driver::default() {
            Some(Path::new("data.db").into())
        } else {
            None
        }
    }
}

impl Settings {
    /// Returns the check of this [`Settings`].
    ///
    /// # Errors
    ///
    /// This function will return an error if unable to transform into a definite database setting.
    pub fn check(&self) -> Result<(), Error> {
        match self.driver {
            Driver::Sqlite3 => {
                sqlite::Settings::try_from(self)?;
            }
            Driver::MySQL => {
                mysql::Settings::try_from(self)?;
            }
        }

        Ok(())
    }

    pub fn get_sqlite_settings(&self) -> Result<sqlite::Settings, Error> {
        sqlite::Settings::try_from(self)
    }

    pub fn get_mysql_settings(&self) -> Result<mysql::Settings, Error> {
        mysql::Settings::try_from(self)
    }
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Hash)]
pub struct OldConfig {
    pub db_driver: driver::Driver,
    pub db_path: String,
}

impl TryFrom<&OldConfig> for Settings {
    type Error = Error;

    fn try_from(value: &OldConfig) -> Result<Self, Self::Error> {
        Ok(match value.db_driver {
            Driver::Sqlite3 => SettingsBuilder::default()
                .driver(Driver::Sqlite3)
                .sql_lite_3_db_file_path(Some(Path::new(&value.db_path).into()))
                .build()
                .unwrap(),
            Driver::MySQL => SettingsBuilder::default()
                .driver(Driver::MySQL)
                .my_sql_connection_url(Some(value.db_path.clone()))
                .build()
                .unwrap(),
        })
    }
}

impl TryFrom<&Settings> for sqlite::Settings {
    type Error = Error;

    fn try_from(value: &Settings) -> Result<Self, Self::Error> {
        Ok(Self {
            database_file_path: match value.driver {
                Driver::Sqlite3 => match &value.sql_lite_3_db_file_path {
                    Some(path) => path.clone(),
                    None => {
                        return Err(Error::MissingFelid {
                            location: Location::caller(),
                            felid: "sql_lite_3_db_file_path".to_string(),
                        })
                    }
                },
                driver => {
                    return Err(Error::WrongDriver {
                        location: Location::caller(),
                        expected: Driver::Sqlite3,
                        actual: driver,
                        settings: value.clone(),
                    })
                }
            },
        })
    }
}

impl TryFrom<&Settings> for mysql::Settings {
    type Error = Error;

    fn try_from(value: &Settings) -> Result<Self, Self::Error> {
        Ok(Self {
            connection_url: match value.driver {
                Driver::MySQL => match &value.my_sql_connection_url {
                    Some(url) => url.clone(),
                    None => {
                        return Err(Error::MissingFelid {
                            location: Location::caller(),
                            felid: "my_sql_connection_url".to_string(),
                        })
                    }
                },
                driver => {
                    return Err(Error::WrongDriver {
                        location: Location::caller(),
                        expected: Driver::MySQL,
                        actual: driver,
                        settings: value.clone(),
                    })
                }
            },
        })
    }
}
