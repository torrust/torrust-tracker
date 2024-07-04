use serde::{Deserialize, Serialize};
use torrust_tracker_primitives::DatabaseDriver;
use url::Url;

#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Database {
    // Database configuration
    /// Database driver. Possible values are: `Sqlite3`, and `MySQL`.
    #[serde(default = "Database::default_driver")]
    pub driver: DatabaseDriver,

    /// Database connection string. The format depends on the database driver.
    /// For `Sqlite3`, the format is `path/to/database.db`, for example:
    /// `./storage/tracker/lib/database/sqlite3.db`.
    /// For `Mysql`, the format is `mysql://db_user:db_user_password:port/db_name`, for
    /// example: `mysql://root:password@localhost:3306/torrust`.
    #[serde(default = "Database::default_path")]
    pub path: String,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            driver: Self::default_driver(),
            path: Self::default_path(),
        }
    }
}

impl Database {
    fn default_driver() -> DatabaseDriver {
        DatabaseDriver::Sqlite3
    }

    fn default_path() -> String {
        String::from("./storage/tracker/lib/database/sqlite3.db")
    }

    /// Masks secrets in the configuration.
    ///
    /// # Panics
    ///
    /// Will panic if the database path for `MySQL` is not a valid URL.
    pub fn mask_secrets(&mut self) {
        match self.driver {
            DatabaseDriver::Sqlite3 => {
                // Nothing to mask
            }
            DatabaseDriver::MySQL => {
                let mut url = Url::parse(&self.path).expect("path for MySQL driver should be a valid URL");
                url.set_password(Some("***")).expect("url password should be changed");
                self.path = url.to_string();
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use torrust_tracker_primitives::DatabaseDriver;

    use super::Database;

    #[test]
    fn it_should_allow_masking_the_mysql_user_password() {
        let mut database = Database {
            driver: DatabaseDriver::MySQL,
            path: "mysql://root:password@localhost:3306/torrust".to_string(),
        };

        database.mask_secrets();

        assert_eq!(database.path, "mysql://root:***@localhost:3306/torrust".to_string());
    }
}
