use crate::InfoHash;
use serde::Serialize;
use std::str::FromStr;
use rusqlite::{Connection, Error};
use log::debug;

pub struct SqliteDatabase {
    conn: Connection
}

impl SqliteDatabase {
    pub async fn new() -> Option<SqliteDatabase> {
        match Connection::open("whitelist.db") {
            Ok(conn) => {
                Some(SqliteDatabase {
                    conn
                })
            }
            Err(e) => None
        }
    }

    pub fn create_database(&self) -> Result<usize, rusqlite::Error> {
        match self.conn.execute(
            "CREATE TABLE IF NOT EXISTS whitelist (
                    id integer PRIMARY KEY AUTO_INCREMENT,
                    info_hash VARCHAR(20) NOT NULL UNIQUE
                    )", []
        ) {
            Ok(updated) => Ok(updated),
            Err(e) => Err(e)
        }
    }

    pub async fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, rusqlite::Error> {
        match self.conn.execute("INSERT INTO whitelist (info_hash) VALUES (?)", [info_hash.to_string()]) {
            Ok(updated) => Ok(updated),
            Err(e) => Err(e)
        }
    }
}
