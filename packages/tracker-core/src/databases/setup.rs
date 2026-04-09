//! This module provides functionality for setting up databases.
use std::sync::Arc;

use torrust_tracker_configuration::Core;

use super::driver::{self, Driver};
use super::Database;

/// Initializes and returns a database instance based on the provided configuration.
///
/// This function creates a new database instance according to the settings
/// defined in the [`Core`] configuration. It selects the appropriate driver
/// (either `Sqlite3` or `MySQL`) as specified in `config.database.driver` and
/// attempts to build the database connection using the path defined in
/// `config.database.path`.
///
/// The resulting database instance is wrapped in a shared pointer (`Arc`) to a
/// boxed trait object, allowing safe sharing of the database connection across
/// multiple threads.
///
/// # Panics
///
/// This function will panic if the database cannot be initialized (i.e., if the
///  driver fails to build the connection). This is enforced by the use of
/// [`expect`](std::result::Result::expect) in the implementation.
///
/// # Example
///
/// ```rust,no_run
/// use torrust_tracker_configuration::Core;
/// use bittorrent_tracker_core::databases::setup::initialize_database;
///
/// // Create a default configuration (ensure it is properly set up for your environment)
/// let config = Core::default();
///
/// // Initialize the database; this will panic if initialization fails.
/// let database = initialize_database(&config);
///
/// // The returned database instance can now be used for persistence operations.
/// ```
#[must_use]
pub fn initialize_database(config: &Core) -> Arc<Box<dyn Database>> {
    let driver = match config.database.driver {
        torrust_tracker_configuration::Driver::Sqlite3 => Driver::Sqlite3,
        torrust_tracker_configuration::Driver::MySQL => Driver::MySQL,
        torrust_tracker_configuration::Driver::PostgreSQL => Driver::PostgreSQL,
    };

    Arc::new(driver::build(&driver, &config.database.path).expect("Database driver build failed."))
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
