use std::sync::Arc;

use torrust_tracker::core::Tracker;
use torrust_tracker::servers::apis::server;

pub mod connection_info;
pub mod environment;
pub mod v1;

pub type Started = environment::Environment<server::Running>;

/// It forces a database error by dropping all tables.
/// That makes any query fail.
/// code-review: alternatively we could inject a database mock in the future.
pub fn force_database_error(tracker: &Arc<Tracker>) {
    tracker.database.drop_database_tables().unwrap();
}
