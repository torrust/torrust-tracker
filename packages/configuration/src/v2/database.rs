use serde::{Deserialize, Serialize};
use torrust_tracker_primitives::DatabaseDriver;

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
    /// example: `root:password@localhost:3306/torrust`.
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
}
