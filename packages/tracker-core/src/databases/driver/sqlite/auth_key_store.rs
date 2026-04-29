use std::panic::Location;

use r2d2_sqlite::rusqlite::params;
use r2d2_sqlite::rusqlite::types::Null;
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use super::{Sqlite, DRIVER};
use crate::authentication::{self, Key};
use crate::databases::error::Error;
use crate::databases::AuthKeyStore;

impl AuthKeyStore for Sqlite {
    fn load_keys(&self) -> Result<Vec<authentication::PeerKey>, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let mut stmt = conn.prepare("SELECT key, valid_until FROM keys")?;

        let raw: Vec<(String, Option<i64>)> = stmt
            .query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, Option<i64>>(1)?)))?
            .filter_map(std::result::Result::ok)
            .collect();

        raw.into_iter()
            .map(|(key, opt_valid_until)| {
                let key = key.parse::<Key>().map_err(|e| Error::MalformedDatabaseRecord {
                    message: e.to_string(),
                    driver: DRIVER,
                })?;
                Ok(match opt_valid_until {
                    Some(valid_until) => authentication::PeerKey {
                        key,
                        valid_until: Some(DurationSinceUnixEpoch::from_secs(valid_until.unsigned_abs())),
                    },
                    None => authentication::PeerKey { key, valid_until: None },
                })
            })
            .collect()
    }

    fn get_key_from_keys(&self, key: &Key) -> Result<Option<authentication::PeerKey>, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let mut stmt = conn.prepare("SELECT key, valid_until FROM keys WHERE key = ?")?;

        let mut rows = stmt.query([key.to_string()])?;

        let key = rows.next()?;

        let peer_key = key
            .map(|f| -> Result<authentication::PeerKey, Error> {
                let valid_until: Option<i64> = f.get(1).map_err(Error::from)?;
                let key: String = f.get(0).map_err(Error::from)?;
                let key = key.parse::<Key>().map_err(|e| Error::MalformedDatabaseRecord {
                    message: e.to_string(),
                    driver: DRIVER,
                })?;
                Ok(match valid_until {
                    Some(valid_until) => authentication::PeerKey {
                        key,
                        valid_until: Some(DurationSinceUnixEpoch::from_secs(valid_until.unsigned_abs())),
                    },
                    None => authentication::PeerKey { key, valid_until: None },
                })
            })
            .transpose()?;

        Ok(peer_key)
    }

    fn add_key_to_keys(&self, auth_key: &authentication::PeerKey) -> Result<usize, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let insert = match auth_key.valid_until {
            Some(valid_until) => conn.execute(
                "INSERT INTO keys (key, valid_until) VALUES (?1, ?2)",
                [auth_key.key.to_string(), valid_until.as_secs().to_string()],
            )?,
            None => conn.execute(
                "INSERT INTO keys (key, valid_until) VALUES (?1, ?2)",
                params![auth_key.key.to_string(), Null],
            )?,
        };

        if insert == 0 {
            Err(Error::InsertFailed {
                location: Location::caller(),
                driver: DRIVER,
            })
        } else {
            Ok(insert)
        }
    }

    fn remove_key_from_keys(&self, key: &Key) -> Result<usize, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let deleted = conn.execute("DELETE FROM keys WHERE key = ?", [key.to_string()])?;

        if deleted == 1 {
            // should only remove a single record.
            Ok(deleted)
        } else {
            Err(Error::DeleteFailed {
                location: Location::caller(),
                error_code: deleted,
                driver: DRIVER,
            })
        }
    }
}
