//! The [`Database`] aggregate supertrait — the full driver contract.
use super::auth_keys::AuthKeyStore;
use super::schema::SchemaMigrator;
use super::torrent_metrics::TorrentMetricsStore;
use super::whitelist::WhitelistStore;

/// The full database driver contract — **internal use only**.
///
/// A new database driver must implement all four supertrait bounds:
/// [`SchemaMigrator`], [`TorrentMetricsStore`], [`WhitelistStore`], and
/// [`AuthKeyStore`]. The blanket impl below means that any type satisfying all
/// four automatically satisfies `Database` — no separate
/// `impl Database for MyDriver {}` block is needed.
///
/// This trait is a compile-time completeness guard for driver authors. External
/// consumers (services, repositories, tests) should depend only on the narrow
/// trait they actually need (`AuthKeyStore`, `WhitelistStore`, etc.). Migration
/// of consumer wiring away from `Arc<Box<dyn Database>>` toward narrow trait
/// injection happens in subsequent subissues; it does not require trait-object
/// upcasting because the factory will coerce the concrete driver type directly
/// into each narrow trait object.
pub trait Database: Sync + Send + SchemaMigrator + TorrentMetricsStore + WhitelistStore + AuthKeyStore {}

impl<T> Database for T where T: Sync + Send + SchemaMigrator + TorrentMetricsStore + WhitelistStore + AuthKeyStore {}
