use std::sync::Arc;

use torrust_tracker::core::Tracker;
use torrust_tracker::servers::apis::server;

pub mod connection_info;
pub mod environment;
pub mod v1;

pub type Started = environment::Environment<server::Running>;

/// It forces a database error by dropping all tables.
/// That makes any query fail.
/// code-review:
/// Alternatively we could:
/// - Inject a database mock in the future.
/// - Inject directly the database reference passed to the Tracker type.
pub fn force_database_error(tracker: &Arc<Tracker>) {
    tracker.drop_database_tables().unwrap();
}
