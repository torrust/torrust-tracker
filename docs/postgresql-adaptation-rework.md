# PostgreSQL Adaptation Rework

## Summary

This rework does not continue the original PostgreSQL attempt on top of the
same synchronous database abstraction. It starts from `develop` and addresses
the architectural concerns that blocked the previous proposal:

- remove the synchronous PostgreSQL driver plus thread-per-operation workaround
- replace the monolithic database trait with narrower persistence stores
- move SQL backends to a shared async `sqlx` substrate
- treat migrations as the schema source of truth
- widen completed/download counters to `u64` internally
- keep BitTorrent protocol compatibility by clamping only at the protocol edge

The goal is to make PostgreSQL a normal backend, not a special case.

## Main Changes

### Persistence redesign

- split the old `Database` trait into:
  - `SchemaMigrator`
  - `TorrentMetricsStore`
  - `WhitelistStore`
  - `AuthKeyStore`
- update the tracker core container and persisted repositories to depend on the
  narrow store traits instead of a single all-purpose database object

### Async SQL backend

- replace the SQL drivers used by SQLite and MySQL with `sqlx`
- add a PostgreSQL backend implemented on the same async substrate
- remove the old synchronous PostgreSQL execution model that spawned threads to
  avoid blocking the Tokio runtime

### Schema and data model alignment

- make migrations the source of truth for SQL schema
- add PostgreSQL migrations
- add a widening migration for download counters
- normalize `InfoHash` storage across SQL backends

### BitTorrent protocol handling

- widen internal completed/download counters from `u32` to `u64`
- keep HTTP responses compatible
- clamp UDP scrape counters explicitly at the protocol boundary instead of
  relying on unchecked narrowing casts

### Configuration and tooling

- add PostgreSQL configuration support and a default PostgreSQL container config
- add compatibility and end-to-end QA scripts
- add a before/after benchmark harness

## Benchmark Snapshot

Benchmarks were run with release builds and the same black-box workload against
the old branch and this rework:

- tracker startup to health ready
- HTTP announce lifecycle (`started -> completed`)
- REST whitelist add/add-concurrent/reload
- REST auth key add/add-concurrent/reload

Environment:

- SQLite `3.51.3`
- MySQL `8.0`
- PostgreSQL `16`

### SQLite

- startup: effectively unchanged
- announce path: small improvement, about `+2%` to `+4%`
- whitelist/auth-key writes: roughly flat
- reload paths: slower, around `0.64x` to `0.67x` of the old throughput

### MySQL

- startup: slightly faster
- announce path: about `+4%` to `+15%`
- sequential whitelist/auth-key writes: about `+16%` to `+19%`
- reload paths: mildly slower, around `0.88x` to `0.90x`

### PostgreSQL

- startup: slightly faster
- announce path: effectively neutral to slightly better
- whitelist writes: up to `+61%`
- auth-key writes: up to `+30%`
- auth-key reload: about `+38%`

The main outcome is that PostgreSQL now performs competitively without relying
on the previously rejected synchronous thread-spawning execution model.

## Draft Reply To Maintainer

> I reworked this from `develop` instead of continuing the previous PostgreSQL
> branch on top of the same database abstraction.
>
> The main idea this time was to address the architectural concerns first, and
> then add PostgreSQL on top of the new persistence layer, rather than adding a
> PostgreSQL special case.
>
> Concretely, I made these changes:
>
> - replaced the monolithic database trait with narrower persistence stores for
>   torrent metrics, whitelist, auth keys, and schema migration
> - moved the SQL backends to a shared async `sqlx` substrate
> - removed the synchronous PostgreSQL driver and the thread-per-operation
>   workaround
> - made migrations the source of truth for schema and added PostgreSQL
>   migrations
> - widened completed/download counters to `u64` internally and clamped only at
>   the BitTorrent protocol boundary where 32-bit fields are still required
> - added PostgreSQL config support, integration coverage, and QA scripts
>
> I also ran a before/after comparison with the same black-box workloads against
> SQLite, MySQL, and PostgreSQL.
>
> High-level results:
>
> - SQLite is mostly neutral, except the reload paths are slower
> - MySQL is modestly better on announce and write-heavy paths, with slightly
>   slower reloads
> - PostgreSQL shows the clearest improvement: startup is slightly better,
>   announce performance is at least on par, and write-heavy persistence paths
>   are noticeably faster than the previous attempt
>
> For PostgreSQL specifically, the point of this rework was not only to make it
> work, but to make it fit the project without the previously rejected blocking
> execution model.
>
> Does this overall direction look acceptable? If so, I can either keep it as a
> single PR or split it into smaller reviewable pieces, depending on how you
> would prefer to review it.
