use super::auth_keys::AsyncAuthKeyStore;
use super::schema::AsyncSchemaMigrator;
use super::torrent_metrics::AsyncTorrentMetricsStore;
use super::whitelist::AsyncWhitelistStore;

/// The full async database driver contract for the parallel sqlx module.
///
/// A temporary aggregate supertrait used during the migration window where
/// sync and async driver stacks coexist.
pub trait AsyncDatabase:
    Send + Sync + AsyncSchemaMigrator + AsyncTorrentMetricsStore + AsyncWhitelistStore + AsyncAuthKeyStore
{
}

impl<T> AsyncDatabase for T where
    T: Send + Sync + AsyncSchemaMigrator + AsyncTorrentMetricsStore + AsyncWhitelistStore + AsyncAuthKeyStore
{
}
