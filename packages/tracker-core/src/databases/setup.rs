//! This module provides functionality for setting up databases.
use torrust_tracker_configuration::Core;

use super::driver::{self, Driver};
use super::Persistence;

/// Initializes and returns persistence handles based on the provided configuration.
///
/// This function creates a new persistence backend according to the settings
/// defined in the [`Core`] configuration. The returned value groups the schema
/// migrator and the per-context stores.
///
/// # Panics
///
/// This function will panic if the database backend cannot be initialized.
#[must_use]
pub fn initialize_database(config: &Core) -> Persistence {
    let driver = match config.database.driver {
        torrust_tracker_configuration::Driver::Sqlite3 => Driver::Sqlite3,
        torrust_tracker_configuration::Driver::MySQL => Driver::MySQL,
        torrust_tracker_configuration::Driver::PostgreSQL => Driver::PostgreSQL,
    };

    driver::build(&driver, &config.database.path).expect("Database driver build failed.")
}

#[cfg(test)]
mod tests {
    use super::initialize_database;
    use crate::test_helpers::tests::ephemeral_configuration;

    #[test]
    fn it_should_initialize_the_sqlite_database() {
        let config = ephemeral_configuration();
        let _database = initialize_database(&config);
    }
}
