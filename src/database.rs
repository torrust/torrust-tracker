use crate::InfoHash;
use log::debug;
use std::sync::Arc;
use r2d2_sqlite::{SqliteConnectionManager, rusqlite};
use r2d2::{Pool};
use rusqlite::params;

pub struct SqliteDatabase {
    pool: Arc<Pool<SqliteConnectionManager>>
}

impl SqliteDatabase {
    pub async fn new() -> Option<SqliteDatabase> {

        let sqlite_file = "whitelist.db";
        let sqlite_connection_manager = SqliteConnectionManager::file(sqlite_file);
        let sqlite_pool = r2d2::Pool::new(sqlite_connection_manager)
            .expect("Failed to create r2d2 SQLite connection pool");
        let pool_arc = Arc::new(sqlite_pool);

        Some(SqliteDatabase {
            pool: pool_arc.clone()
        })
    }

    pub fn create_database(&self) -> Result<usize, rusqlite::Error> {
        let conn = self.pool.get().unwrap();
        match conn.execute(
            "CREATE TABLE IF NOT EXISTS whitelist (
                    id integer PRIMARY KEY AUTOINCREMENT,
                    info_hash VARCHAR(20) NOT NULL UNIQUE
                    )", params![]
        ) {
            Ok(updated) => Ok(updated),
            Err(e) => {
                debug!("{:?}", e);
                Err(e)
            }
        }
    }

    pub async fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, rusqlite::Error> {
        let conn = self.pool.get().unwrap();
        match conn.execute("INSERT INTO whitelist (info_hash) VALUES (?)", &[info_hash.to_string()]) {
            Ok(updated) => Ok(updated),
            Err(e) => {
                debug!("{:?}", e);
                Err(e)
            }
        }
    }
}
