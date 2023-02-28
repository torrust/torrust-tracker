pub mod driver;
pub mod error;
pub mod mysql;
pub mod sqlite;

use std::marker::PhantomData;

use async_trait::async_trait;

use self::error::Error;
use crate::protocol::info_hash::InfoHash;
use crate::tracker::auth;

pub(self) struct Builder<T>
where
    T: Database,
{
    phantom: PhantomData<T>,
}

impl<T> Builder<T>
where
    T: Database + 'static,
{
    /// .
    ///
    /// # Errors
    ///
    /// Will return `r2d2::Error` if `db_path` is not able to create a database.
    pub(self) fn build(db_path: &str) -> Result<Box<dyn Database>, Error> {
        Ok(Box::new(T::new(db_path)?))
    }
}

#[async_trait]
pub trait Database: Sync + Send {
    /// .
    ///
    /// # Errors
    ///
    /// Will return `r2d2::Error` if `db_path` is not able to create a database.
    fn new(db_path: &str) -> Result<Self, Error>
    where
        Self: std::marker::Sized;

    /// .
    ///
    /// # Errors
    ///
    /// Will return `Error` if unable to create own tables.
    fn create_database_tables(&self) -> Result<(), Error>;

    /// # Errors
    ///
    /// Will return `Err` if unable to drop tables.
    fn drop_database_tables(&self) -> Result<(), Error>;

    async fn load_persistent_torrents(&self) -> Result<Vec<(InfoHash, u32)>, Error>;

    async fn load_keys(&self) -> Result<Vec<auth::ExpiringKey>, Error>;

    async fn load_whitelist(&self) -> Result<Vec<InfoHash>, Error>;

    async fn save_persistent_torrent(&self, info_hash: &InfoHash, completed: u32) -> Result<(), Error>;

    // todo: replace type `&str` with `&InfoHash`
    async fn get_info_hash_from_whitelist(&self, info_hash: &str) -> Result<Option<InfoHash>, Error>;

    async fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error>;

    async fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error>;

    // todo: replace type `&str` with `&KeyId`
    async fn get_key_from_keys(&self, key: &str) -> Result<Option<auth::ExpiringKey>, Error>;

    async fn add_key_to_keys(&self, auth_key: &auth::ExpiringKey) -> Result<usize, Error>;

    // todo: replace type `&str` with `&KeyId`
    async fn remove_key_from_keys(&self, key: &str) -> Result<usize, Error>;

    async fn is_info_hash_whitelisted(&self, info_hash: &InfoHash) -> Result<bool, Error> {
        Ok(self
            .get_info_hash_from_whitelist(&info_hash.clone().to_string())
            .await?
            .is_some())
    }
}
