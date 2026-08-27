use torrust_tracker_configuration::v3_0_0::core::Core;
use torrust_tracker_configuration::v3_0_0::database::Database;
use torrust_tracker_core::databases::setup::initialize_database;

use super::{ActiveDatabase, BenchmarkResource};

pub(super) async fn initialize() -> ActiveDatabase {
    let sqlite_db_path = std::env::temp_dir().join(format!(
        "torrust-tracker-core-benchmark-{}.sqlite3",
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()
    ));
    let sqlite_db_path_as_string = sqlite_db_path.to_string_lossy().to_string();
    let config = Core {
        database: Some(Database::Sqlite3 {
            path: sqlite_db_path_as_string,
        }),
        ..Default::default()
    };

    let database = initialize_database(&config).await;

    ActiveDatabase {
        database: Some(database),
        resource: Some(BenchmarkResource::Sqlite(sqlite_db_path)),
    }
}
