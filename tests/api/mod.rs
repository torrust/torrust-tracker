use std::sync::Arc;

use torrust_tracker::tracker::Tracker;

pub mod asserts;
pub mod client;
pub mod connection_info;
pub mod test_environment;
pub mod tests;

/// It forces a database error by dropping all tables.
/// That makes any query fail.
/// code-review: alternatively we could inject a database mock in the future.
pub fn force_database_error(tracker: &Arc<Tracker>) {
    tracker.database.drop_database_tables().unwrap();
}
