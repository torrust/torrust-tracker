# Subissue Draft for #1525-04b: Migrate Consumers to Narrow Persistence Traits

## Goal

Replace every use of `Arc<Box<dyn Database>>` in production and test code with
the specific narrow trait the consumer actually needs (`AuthKeyStore`,
`TorrentMetricsStore`, `WhitelistStore`, or `SchemaMigrator`). After this
subissue the `Database` aggregate supertrait becomes a purely internal
compile-time guard that is no longer part of the public surface of
`tracker-core`.

## Background

Subissue #1525-04 (GitHub [#1713](https://github.com/torrust/torrust-tracker/issues/1713))
introduced the four narrow traits and kept `Database` as an aggregate supertrait
so that consumer call sites did not need to change.

Now that the structural split is in place, this subissue wires consumers to the
narrow traits they actually need. No upcasting is required: the factory will
construct the concrete driver (`Sqlite`, `Mysql`) and coerce it directly into
each narrow `Arc<dyn XxxStore>`. Coercion from a sized type to a trait object is
available on all Rust versions.

## Proposed Branch

- `1525-04b-migrate-consumers-to-narrow-traits`

## Current State

All consumers depend on `Arc<Box<dyn Database>>` for everything, regardless of
which methods they actually call:

| Consumer                                           | Methods actually used                                       |
| -------------------------------------------------- | ----------------------------------------------------------- |
| `DatabaseKeyRepository`                            | `AuthKeyStore` methods only                                 |
| `DatabaseDownloadsMetricRepository`                | `TorrentMetricsStore` methods only                          |
| `whitelist::setup::initialize_whitelist_manager`   | `WhitelistStore` methods only                               |
| `databases::driver::build` / `initialize_database` | `SchemaMigrator::create_database_tables` only               |
| `bin/persistence_benchmark`                        | All four concerns — uses `Database` as a convenience bundle |
| `container::TrackerCoreContainer`                  | Holds the database and fans it out to the above             |

## Target State

```text
TrackerCoreContainer
  database_stores:        DatabaseStores   ← replaces Arc<Box<dyn Database>>
  ...rest of fields unchanged...
```

`DatabaseStores` is a plain struct holding one `Arc<dyn XxxStore>` per context.
The container stores it as one named field; individual services are wired at
construction time by passing the relevant field (e.g.
`database_stores.auth_key_store.clone()`) to each service constructor. Services
themselves never see `DatabaseStores` — they receive only the narrow trait they
need.

The factory (`databases::driver::build` / `initialize_database`) constructs the
concrete driver once and produces four `Arc<dyn XxxStore>` coercions from it:

```rust
pub struct DatabaseStores {
    pub schema_migrator:       Arc<dyn SchemaMigrator>,
    pub torrent_metrics_store: Arc<dyn TorrentMetricsStore>,
    pub whitelist_store:       Arc<dyn WhitelistStore>,
    pub auth_key_store:        Arc<dyn AuthKeyStore>,
}

pub fn initialize_database(config: &Core) -> DatabaseStores {
    match config.database.driver {
        Driver::Sqlite3 => {
            let db = Arc::new(Sqlite::new(&config.database.path).expect("..."));
            db.create_database_tables().expect("...");
            DatabaseStores {
                schema_migrator:       db.clone(),
                torrent_metrics_store: db.clone(),
                whitelist_store:       db.clone(),
                auth_key_store:        db,
            }
        }
        Driver::MySQL => { /* same pattern */ }
    }
}
```

## Tasks

### 1) Introduce `DatabaseStores`

Add a plain struct `databases::setup::DatabaseStores` holding one `Arc<dyn XxxStore>`
per narrow trait. No `Arc<Box<dyn Database>>`.

### 2) Update `initialize_database`

Change the return type from `Arc<Box<dyn Database>>` to `DatabaseStores`.
Build the concrete driver, call `create_database_tables`, then produce the four
coercions.

### 3) Update `TrackerCoreContainer`

- Replace `pub database: Arc<Box<dyn Database>>` with `pub database_stores: DatabaseStores`.
- Update `initialize_from` to call `initialize_database` (which now returns
  `DatabaseStores`) and fan the narrow stores out to each service constructor:

  ```rust
  let db = initialize_database(core_config);
  let whitelist_manager = initialize_whitelist_manager(db.whitelist_store.clone(), ...);
  let db_key_repository = Arc::new(DatabaseKeyRepository::new(db.auth_key_store.clone()));
  let db_downloads = Arc::new(DatabaseDownloadsMetricRepository::new(db.torrent_metrics_store.clone()));
  // ... store the struct itself so callers can still access it if needed
  Self { database_stores: db, ... }
  ```

### 4) Update individual consumers

- `DatabaseKeyRepository::new` — accept `Arc<dyn AuthKeyStore>` instead of
  `Arc<Box<dyn Database>>`.
- `DatabaseDownloadsMetricRepository::new` — accept `Arc<dyn TorrentMetricsStore>`.
- `whitelist::setup::initialize_whitelist_manager` — accept `Arc<dyn WhitelistStore>`.

### 5) Update tests in `authentication/handler.rs`

Replace `Arc<Box<dyn Database>>` wiring with `MockAuthKeyStore` injected
directly as `Arc<dyn AuthKeyStore>`.

### 6) Update `axum-rest-tracker-api-server` test helper

`packages/axum-rest-tracker-api-server/tests/server/mod.rs::force_database_error`
currently receives `&Arc<Box<dyn Database>>`. Update to the narrow trait(s) it
actually exercises.

### 7) Update benchmark binary

`bin/persistence_benchmark/driver_bench/` passes `&dyn Database` to operations
that each touch only one concern. Update each operation function to accept the
narrow trait it needs:

- `operations/torrent.rs` → `&dyn TorrentMetricsStore`
- `operations/whitelist.rs` → `&dyn WhitelistStore`
- `operations/keys.rs` → `&dyn AuthKeyStore`
- `database/mod.rs::reset_database` → `&dyn SchemaMigrator`

### 8) Make `Database` private

Once no production or test code outside `databases/` uses `Database`, stop
re-exporting it from `databases/mod.rs`. Keep it accessible inside
`databases/traits/database.rs` for driver authors.

## Out of Scope

- Async trait methods. That is subissue #1525-05.
- Schema migrations. That is subissue #1525-06.
- PostgreSQL support. That is subissue #1525-08.

## Acceptance Criteria

- [ ] `Arc<Box<dyn Database>>` appears only inside `databases/` (driver + traits).
- [ ] Each consumer holds only the narrow trait(s) it uses.
- [ ] `Database` is no longer re-exported from `databases/mod.rs`.
- [ ] Tests in `authentication/handler.rs` use `MockAuthKeyStore` directly.
- [ ] `force_database_error` helper in `axum-rest-tracker-api-server` is updated.
- [ ] Benchmark operations accept narrow traits.
- [ ] `cargo test --workspace --all-targets` passes.
- [ ] `linter all` exits with code `0`.

## References

- EPIC: #1525
- GitHub Issue: #1715
- Predecessor: [docs/issues/1713-1525-04-split-persistence-traits.md](1713-1525-04-split-persistence-traits.md)
- ADR: [docs/adrs/20260429000000_keep_database_as_aggregate_supertrait.md](../adrs/20260429000000_keep_database_as_aggregate_supertrait.md)
- Successor: [docs/issues/1525-05-migrate-sqlite-and-mysql-to-sqlx.md](1525-05-migrate-sqlite-and-mysql-to-sqlx.md)
