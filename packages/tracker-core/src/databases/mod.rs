//! The persistence module.
//!
//! Persistence is implemented through four narrow context traits and an
//! aggregate supertrait:
//!
//! - [`SchemaMigrator`] — schema lifecycle (create / drop tables)
//! - [`TorrentMetricsStore`] — per-torrent and global download counters
//! - [`WhitelistStore`] — torrent infohash whitelist
//! - [`AuthKeyStore`] — authentication key persistence
//! - [`Database`] — aggregate supertrait; any type that implements all four
//!   narrow traits automatically satisfies `Database` via a blanket impl
//!
//! Design rationale: see ADR
//! [`20260429000000_keep_database_as_aggregate_supertrait`](../../../docs/adrs/20260429000000_keep_database_as_aggregate_supertrait.md).
//!
//! There are two implementations (two drivers):
//!
//! - **`MySQL`**
//! - **`Sqlite`**
//!
//! > **NOTICE**: There are no database migrations at this time. If schema
//! > changes occur, either migration functionality will be implemented or a
//! > script will be provided to migrate to the new schema.
//!
//! The persistent objects handled by this module include:
//!
//! - **Torrent metrics**: Metrics such as the number of completed downloads for
//!   each torrent.
//! - **Torrent whitelist**: A list of torrents (by infohash) that are allowed.
//! - **Authentication keys**: Expiring authentication keys used to secure
//!   access to private trackers.
//!
//! # Torrent Metrics
//!
//! | Field       | Sample data                                | Description                                                                 |
//! |-------------|--------------------------------------------|-----------------------------------------------------------------------------|
//! | `id`        | 1                                          | Auto-increment id                                                           |
//! | `info_hash` | `c1277613db1d28709b034a017ab2cae4be07ae10` | `BitTorrent` infohash V1                                                    |
//! | `completed` | 20                                         | The number of peers that have completed downloading the associated torrent. |
//!
//! > **NOTICE**: The peer list for a torrent is not persisted. Because peers re-announce at
//! > intervals, the peer list is regenerated periodically.
//!
//! # Torrent Whitelist
//!
//! | Field       | Sample data                                | Description                    |
//! |-------------|--------------------------------------------|--------------------------------|
//! | `id`        | 1                                          | Auto-increment id              |
//! | `info_hash` | `c1277613db1d28709b034a017ab2cae4be07ae10` | `BitTorrent` infohash V1       |
//!
//! # Authentication Keys
//!
//! | Field         | Sample data                        | Description                          |
//! |---------------|------------------------------------|--------------------------------------|
//! | `id`          | 1                                  | Auto-increment id                    |
//! | `key`         | `IrweYtVuQPGbG9Jzx1DihcPmJGGpVy82` | Authentication token (32 chars)      |
//! | `valid_until` | 1672419840                         | Timestamp indicating expiration time |
//!
//! > **NOTICE**: All authentication keys must have an expiration date.
pub mod driver;
pub mod error;
pub mod setup;
pub mod traits;

pub use traits::{
    AuthKeyStore, MockAuthKeyStore, MockSchemaMigrator, MockTorrentMetricsStore, MockWhitelistStore, SchemaMigrator,
    TorrentMetricsStore, WhitelistStore,
};
