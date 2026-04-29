# Keep `Database` as an Aggregate Supertrait

## Description

The persistence layer used a single monolithic `Database` trait with 18 methods
spanning four distinct concerns: schema lifecycle, torrent metrics, whitelist
management, and authentication keys. Consumers that only needed one concern
(e.g. `DatabaseKeyRepository`) were forced to depend on the full 18-method
interface, making tests harder to write and clouding the intent of each service.

The question was how to split the trait while preserving a single, discoverable
contract that all database drivers must satisfy.

## Agreement

Split `Database` into four narrow context traits:

- `SchemaMigrator` — `create_database_tables`, `drop_database_tables`
- `TorrentMetricsStore` — load/save/increase per-torrent and global download counters (7 methods)
- `WhitelistStore` — load/get/add/remove infohash whitelist entries (4 required + 1 default method)
- `AuthKeyStore` — load/get/add/remove authentication keys (4 methods)

Keep `Database` as an **empty aggregate supertrait** with a blanket implementation:

```rust
pub trait Database: Sync + Send + SchemaMigrator + TorrentMetricsStore + WhitelistStore + AuthKeyStore {}

impl<T> Database for T where T: Sync + Send + SchemaMigrator + TorrentMetricsStore + WhitelistStore + AuthKeyStore {}
```

`Database` is a **private, internal compile-time contract** for driver
completeness only. External consumers (services, repositories, tests) will
progress toward using only the narrow traits they actually need. That migration
happens in future subissues and does not require changing any consumer in this
step.

### Alternatives Considered

**Independent traits only (no `Database` supertrait)** — Each driver would
implement four separate traits; consumers would receive `Arc<Box<dyn AuthKeyStore>>`
etc. instead of `Arc<Box<dyn Database>>`.

Rejected because:

1. There would be no single place to verify that a driver implements the
   complete persistence contract — the compiler can no longer catch a partially
   implemented driver as one unit.
2. Changing every call site (container wiring, factory, tests) all at once
   would turn this structural step into a much larger, riskier diff. The
   aggregate supertrait lets the split land cleanly first; consumer migration
   follows in subsequent subissues.

Note on trait-object upcasting: migrating consumers to narrow traits does **not**
require upcasting (`dyn Database` → `dyn WhitelistStore`). The factory will
construct the concrete driver type (e.g. `Arc<Sqlite>`) and coerce it directly
into each narrow trait object (`Arc<dyn WhitelistStore>`, etc.). Coercion from
a sized type to a trait object is available on all Rust versions; upcasting
between two trait objects would be a different story, but is not needed here.

### Consequences

#### Positive

- Each narrow trait expresses a single context; services and tests can depend
  only on the interface they actually need.
- `#[automock]` on each narrow trait generates focused mocks (`MockAuthKeyStore`
  etc.) instead of one 18-method mega-mock.
- The blanket impl makes it impossible to partially implement `Database`:
  the compiler enforces completeness of all four narrow traits together.

### Negative

- Tests that previously used `MockDatabase` must be updated to use the
  appropriate narrow mock (`MockWhitelistStore`, `MockAuthKeyStore`, etc.).
  This is actually simpler — each mock covers only the methods the test cares
  about — but it is a mechanical change across test files.
- `Database` will persist as long as `Arc<Box<dyn Database>>` wiring exists.
  That wiring will be replaced in subissue #1525-04b
  ([docs/issues/1715-1525-04b-migrate-consumers-to-narrow-traits.md](../issues/1715-1525-04b-migrate-consumers-to-narrow-traits.md))
  by a plain `DatabaseStores` struct (one `Arc<dyn XxxStore>` field per
  context). `TrackerCoreContainer` will hold `DatabaseStores` instead of
  `Arc<Box<dyn Database>>`; each service is wired at construction time by
  passing only the narrow store it needs. At that point `Database` can be
  made fully private or removed.

### Clarification And Revisit Criteria

For now, `TorrentMetricsStore` keeps both per-torrent downloads (stored in
`torrents`) and the global aggregate metric `TORRENTS_DOWNLOADS_TOTAL`
(stored in `torrent_aggregate_metrics`). This is intentional: in the current
domain model there is only one persisted per-torrent metric and one persisted
global metric, and they are strongly related.

There is no near-term plan to add more tables, fields, or persisted objects in
this area. Therefore, introducing another split (for example,
`TorrentAggregateMetricStore`) is deferred to avoid extra API churn without
clear short-term benefit.

This decision should be reconsidered if persistence scope changes, especially
if aggregate metrics grow and are no longer torrent-specific (for example,
global tracker metrics such as total unique peers that ever announced), or if
method count/responsibility in `TorrentMetricsStore` increases materially.

## Date

2026-04-29

## References

- Issue spec: [docs/issues/1713-1525-04-split-persistence-traits.md](../issues/1713-1525-04-split-persistence-traits.md)
- GitHub issue: <https://github.com/torrust/torrust-tracker/issues/1713>
- EPIC: [docs/issues/1525-overhaul-persistence.md](../issues/1525-overhaul-persistence.md)
