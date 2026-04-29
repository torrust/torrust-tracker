pub mod connection_info;
pub mod v1;

use std::sync::Arc;

use bittorrent_tracker_core::databases::SchemaMigrator;

/// It forces a database error by dropping all tables. That makes all queries
/// fail.
///
/// code-review:
///
/// Alternatively we could:
///
/// - Inject a database mock in the future.
/// - Inject directly the database reference passed to the Tracker type.
pub fn force_database_error(schema_migrator: &Arc<dyn SchemaMigrator>) {
    schema_migrator.drop_database_tables().unwrap();
}
