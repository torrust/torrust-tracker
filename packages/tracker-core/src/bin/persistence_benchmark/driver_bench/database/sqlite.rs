use bittorrent_tracker_core::databases::setup::initialize_database;
use torrust_tracker_configuration as configuration;

use super::{ActiveDatabase, BenchmarkResource};

pub(super) fn initialize() -> ActiveDatabase {
    let sqlite_db_path = std::env::temp_dir().join(format!(
        "torrust-tracker-core-benchmark-{}.sqlite3",
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()
    ));
    let sqlite_db_path_as_string = sqlite_db_path.to_string_lossy().to_string();
    let mut config = configuration::Core::default();
    config.database.driver = configuration::Driver::Sqlite3;
    config.database.path = sqlite_db_path_as_string;

    let database = initialize_database(&config);

    ActiveDatabase {
        database: Some(database),
        resource: Some(BenchmarkResource::Sqlite(sqlite_db_path)),
    }
}
