# Issue #1713 (Subissue of #1525-04): Split Persistence Traits by Context

## Goal

Decompose the monolithic `Database` trait into four focused context traits while
keeping `Database` as the unified driver contract, and write an ADR to record the
decision.

## Background

`packages/tracker-core/src/databases/mod.rs` defines a single `Database` trait with
19 methods covering four unrelated concerns: schema management, torrent metrics,
whitelist, and authentication keys. This makes the trait long and conflates distinct
responsibilities in one place.

Two options were considered:

1. **Replace `Database` with four independent traits** — consumers hold
   `Arc<dyn WhitelistStore>` etc. directly. Clean interface segregation, but it loses
   the single place that tells a new driver implementor exactly what to build, and it
   changes every consumer at once.

2. **Keep `Database` as an aggregate supertrait** (chosen) — the four narrow traits
   exist independently; `Database` is defined as:

   ```rust
   pub trait Database:
       Sync + Send + SchemaMigrator + TorrentMetricsStore + WhitelistStore + AuthKeyStore {}
   ```

   A blanket impl means any type that implements all four narrow traits automatically
   satisfies `Database`. Existing consumers (`Arc<Box<dyn Database>>`) are untouched.

This preserves both goals:

- **One place to discover the full driver contract**: `Database` and its four supertrait
  bounds tell a new implementor exactly what to write.
- **Compiler-enforced completeness**: adding a fifth supertrait later causes a compile
  error in every driver that does not yet implement it.
- **Interface segregation at the consumer level**: the four narrow traits can be used
  directly in tests (`MockWhitelistStore` etc.) and optionally as dependency types once
  the MSRV allows trait-object upcasting (stabilised in Rust 1.76; current MSRV is 1.72).

## Proposed Branch

- `1713-1525-04-split-persistence-traits`

## Current State

The starting point (before this subissue):

```text
packages/tracker-core/src/databases/
  mod.rs          ← Database trait (19 methods, all concerns in one block)
  driver/
    mod.rs
    sqlite.rs     ← impl Database for Sqlite { ... 19 methods ... }
    mysql.rs      ← impl Database for Mysql  { ... 19 methods ... }
  error.rs
  setup.rs
```

The four context groups already exist as doc-comment markers inside the trait
(`# Context: Schema`, `# Context: Torrent Metrics`, etc.) — this subissue makes those
boundaries structural.

## Target State

```text
packages/tracker-core/src/databases/
  mod.rs                ← module declarations, re-exports
  database.rs           ← Database aggregate trait + blanket impl
  schema.rs             ← SchemaMigrator trait
  torrent_metrics.rs    ← TorrentMetricsStore trait
  whitelist.rs          ← WhitelistStore trait
  auth_keys.rs          ← AuthKeyStore trait
  driver/
    mod.rs
    sqlite.rs           ← impl SchemaMigrator + TorrentMetricsStore
                           + WhitelistStore + AuthKeyStore for Sqlite
    mysql.rs            ← same for Mysql
  error.rs
  setup.rs
```

## Tasks

### 1) Write the ADR

Create `docs/adrs/<timestamp>_keep_database_as_aggregate_supertrait.md` recording:

- The problem (19-method monolith, unclear per-context boundaries).
- The two options considered (independent traits vs. aggregate supertrait).
- The decision and rationale (aggregate supertrait — see Background above).
- The known constraint: trait-object upcasting from `dyn Database` to a narrow
  `dyn XxxStore` requires Rust ≥ 1.76; the MSRV today is 1.72, so consumer wiring
  stays as `Arc<Box<dyn Database>>` for now.

Add a row to `docs/adrs/index.md`.

### 2) Introduce the four narrow traits

Create one file per trait. Each file contains only that trait's methods, moved verbatim
from `Database` (doc-comments included), plus `#[automock]` for mockall.

**`databases/schema.rs`** — `SchemaMigrator`:

```rust
#[automock]
pub trait SchemaMigrator: Sync + Send {
    fn create_database_tables(&self) -> Result<(), Error>;
    fn drop_database_tables(&self) -> Result<(), Error>;
}
```

**`databases/torrent_metrics.rs`** — `TorrentMetricsStore`:

```rust
#[automock]
pub trait TorrentMetricsStore: Sync + Send {
    fn load_all_torrents_downloads(&self) -> Result<NumberOfDownloadsBTreeMap, Error>;
    fn load_torrent_downloads(&self, info_hash: &InfoHash) -> Result<Option<NumberOfDownloads>, Error>;
    fn save_torrent_downloads(&self, info_hash: &InfoHash, downloaded: NumberOfDownloads) -> Result<(), Error>;
    fn increase_downloads_for_torrent(&self, info_hash: &InfoHash) -> Result<(), Error>;
    fn load_global_downloads(&self) -> Result<Option<NumberOfDownloads>, Error>;
    fn save_global_downloads(&self, downloaded: NumberOfDownloads) -> Result<(), Error>;
    fn increase_global_downloads(&self) -> Result<(), Error>;
}
```

**`databases/whitelist.rs`** — `WhitelistStore`:

```rust
#[automock]
pub trait WhitelistStore: Sync + Send {
    fn load_whitelist(&self) -> Result<Vec<InfoHash>, Error>;
    fn get_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<Option<InfoHash>, Error>;
    fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error>;
    fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error>;
    fn is_info_hash_whitelisted(&self, info_hash: InfoHash) -> Result<bool, Error> {
        Ok(self.get_info_hash_from_whitelist(info_hash)?.is_some())
    }
}
```

**`databases/auth_keys.rs`** — `AuthKeyStore`:

```rust
#[automock]
pub trait AuthKeyStore: Sync + Send {
    fn load_keys(&self) -> Result<Vec<authentication::PeerKey>, Error>;
    fn get_key_from_keys(&self, key: &Key) -> Result<Option<authentication::PeerKey>, Error>;
    fn add_key_to_keys(&self, auth_key: &authentication::PeerKey) -> Result<usize, Error>;
    fn remove_key_from_keys(&self, key: &Key) -> Result<usize, Error>;
}
```

### 3) Introduce the `Database` aggregate trait

Create `databases/database.rs`:

```rust
use super::{AuthKeyStore, SchemaMigrator, TorrentMetricsStore, WhitelistStore};

/// The full driver contract.
///
/// A new database driver must implement all four supertrait bounds. The blanket
/// impl below means that any type satisfying all four automatically satisfies
/// `Database` — no separate `impl Database for MyDriver {}` is needed.
///
/// `Arc<Box<dyn Database>>` continues to be the wiring type used by driver
/// setup and consumer repositories. Direct use of the narrow traits as
/// dependency types will become practical once the MSRV reaches 1.76
/// (trait-object upcasting).
pub trait Database:
    Sync + Send + SchemaMigrator + TorrentMetricsStore + WhitelistStore + AuthKeyStore
{
}

impl<T> Database for T where
    T: Sync + Send + SchemaMigrator + TorrentMetricsStore + WhitelistStore + AuthKeyStore
{
}
```

Remove the `#[automock]` from the old `Database` trait definition — mocking now happens
through the four narrow traits.

### 4) Update the drivers

In `driver/sqlite.rs` and `driver/mysql.rs`:

- Remove `impl Database for <Driver> { ... }` (the blanket impl replaces it).
- Add four separate `impl` blocks — one per narrow trait — containing the same method
  bodies that were previously in the single `impl Database` block.
- No logic changes. This is a mechanical redistribution of existing code.

Example structure after the change:

```rust
impl SchemaMigrator for Sqlite {
    fn create_database_tables(&self) -> Result<(), Error> { ... }
    fn drop_database_tables(&self) -> Result<(), Error> { ... }
}

impl TorrentMetricsStore for Sqlite {
    fn load_all_torrents_downloads(&self) -> Result<NumberOfDownloadsBTreeMap, Error> { ... }
    // ... remaining 6 methods
}

impl WhitelistStore for Sqlite {
    // ... 5 methods
}

impl AuthKeyStore for Sqlite {
    // ... 4 methods
}
```

If the driver file becomes unwieldy, the four `impl` blocks can be moved into a
`driver/sqlite/` submodule — but that is optional and not required by this subissue.

### 5) Update `mod.rs`

- Declare the four new submodules.
- Re-export the traits and the `MockXxx` types so existing `use
crate::databases::Database` imports continue to work.
- Remove the method bodies and imports that were previously inlined in `mod.rs`.

After the change, `mod.rs` should be a thin index:

```rust
pub mod auth_keys;
pub mod database;
pub mod driver;
pub mod error;
pub mod schema;
pub mod setup;
pub mod torrent_metrics;
pub mod whitelist;

pub use auth_keys::{AuthKeyStore, MockAuthKeyStore};
pub use database::Database;
pub use schema::{MockSchemaMigrator, SchemaMigrator};
pub use torrent_metrics::{MockTorrentMetricsStore, TorrentMetricsStore};
pub use whitelist::{MockWhitelistStore, WhitelistStore};
```

## Out of Scope

- Changing consumer wiring from `Arc<Box<dyn Database>>` to narrow trait objects.
  That is blocked by the MSRV constraint and is deferred.
- Async trait methods. That is subissue #1525-05.
- Schema migrations. That is subissue #1525-06.
- PostgreSQL support. That is subissue #1525-08.

## Acceptance Criteria

- [ ] ADR is written and added to `docs/adrs/index.md`.
- [ ] Four narrow traits exist in separate files under `databases/`.
- [ ] `Database` is an empty aggregate supertrait with a blanket impl.
- [ ] Both drivers (`Sqlite`, `Mysql`) compile through the blanket impl with no manual
      `impl Database for <Driver>` block.
- [ ] No existing consumer file (`persisted.rs`, `downloads.rs`, etc.) is changed.
- [ ] `#[automock]` is on the four narrow traits; `MockDatabase` is removed.
- [ ] No behavior change — existing tests pass without modification.
- [ ] Persistence benchmarking (see subissue #1525-03) shows no regression against the
      committed baseline.
- [ ] `cargo test --workspace --all-targets` passes.
- [ ] `linter all` exits with code `0`.

## References

- EPIC: #1525
- Reference PR: #1695
- Reference implementation branch: `josecelano:pr-1684-review` — see EPIC for checkout
  instructions (`docs/issues/1525-overhaul-persistence.md`)
- `packages/tracker-core/src/databases/mod.rs` — current monolithic `Database` trait
- `packages/tracker-core/src/whitelist/repository/persisted.rs` — example consumer
- `packages/tracker-core/src/statistics/persisted/downloads.rs` — example consumer
- `packages/tracker-core/src/authentication/key/repository/persisted.rs` — example consumer
