pub mod connection_info;
pub mod v1;

use bittorrent_tracker_core::databases::Persistence;

/// It forces a database error by dropping all tables. That makes all queries
/// fail.
///
/// code-review:
///
/// Alternatively we could:
///
/// - Inject a database mock in the future.
/// - Inject directly the database reference passed to the Tracker type.
pub async fn force_database_error(tracker: &Persistence) {
    tracker.schema_migrator().drop_database_tables().await.unwrap();
}
