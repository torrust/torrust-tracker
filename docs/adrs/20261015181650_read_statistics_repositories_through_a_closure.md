---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - docs/issues/open/2536-1669-hide-statistics-repository-read-lock/ISSUE.md
    - docs/issues/closed/2517-1669-encapsulate-statistics-repository-state/ISSUE.md
    - docs/adrs/20260629000000_adopt_independent_package_versioning.md
    - packages/udp-core/src/statistics/repository.rs
    - packages/udp-server/src/statistics/repository.rs
    - packages/http-core/src/statistics/repository.rs
    - packages/tracker-core/src/statistics/repository.rs
    - packages/swarm-coordination-registry/src/statistics/repository.rs
---

<!-- skill-link: create-adr -->

# Read the Statistics Repositories Through a Closure, Not a Lock Guard

## Scope

Repository-wide. The decision fixes the public read API of five statistics repositories in five
workspace packages (`udp-core`, `udp-server`, `http-core`, `tracker-core`, and
`swarm-coordination-registry`) and of the `Metrics` types they hold. The callers include
`rest-api-runtime-adapter` and the test environments of other packages, so it is a multi-package
contract and belongs in `docs/adrs/`.

## Description

Each statistics repository keeps its `Metrics` behind a `tokio::sync::RwLock`. Issue #2517 made the
lock field private, but each repository still returned the lock's read guard
(`RwLockReadGuard<'_, Metrics>`) from `get_stats()` or `get_metrics()`, and each `Metrics` type had a
public `metric_collection` field. That had three costs:

- The lock type was public API. Replacing the tokio lock (with a `std` or `parking_lot` lock, or a
  lock-free store) would have been a breaking change in five crates.
- A caller could hold the guard as long as it liked, including across an `.await`, which blocks
  every writer (the statistics event listeners) for that time.
- The layout of `Metrics` was public API, because its collection was a public field.

None of the five crates is published yet. Under the independent versioning policy in
[Adopt Independent Package Versioning](20260629000000_adopt_independent_package_versioning.md), a
public API change after the first publish is a breaking release, so the cheapest moment to fix this
is before it.

## Agreement

A statistics repository exposes no lock type and no metric-collection field in its public read API.
Callers read through one closure-based method with the same name in every repository, and through a
read-only accessor on `Metrics`:

```rust
impl Repository {
    /// Runs `read` on the current metrics while holding the read lock.
    pub async fn read_metrics<R>(&self, read: impl FnOnce(&Metrics) -> R) -> R {
        let metrics = self.stats.read().await;
        read(&metrics)
    }
}

impl Metrics {
    #[must_use]
    pub const fn metric_collection(&self) -> &MetricCollection {
        &self.metric_collection
    }
}
```

The `Metrics.metric_collection` field is `pub(crate)`. Each repository pins this with a
`compile_fail,E0616` doctest for the field and a compiling doctest for the new API.

The rule applies to the five existing repositories and to any new statistics repository.

- **The closure is synchronous.** A caller cannot hold the lock across an `.await`.
- **Callers keep the closure short.** The read lock is held while the closure runs, and writers
  wait. The closure must not block the thread (blocking I/O, sleeping, or long computation) and must
  not call `read_metrics` on the same repository, which could deadlock. The type system does not
  enforce this; `read_metrics()`'s rustdoc states it.
- **One acquisition reads consistent values.** A caller that needs several values that one
  repository serves together, such as the REST `Stats` with about twenty values from the UDP server
  repository, reads them in one closure. A caller that needs values from several repositories
  takes one acquisition per repository.
- **Accepted exception: `tracker-core`'s per-value accessors.** `tracker-core`'s repository also
  keeps `get_torrents_downloads_total()`, `get_torrents_downloads_in_session_total()` and
  `get_torrents_downloads_persisted_total()`. Each is one `read_metrics()` call, so the REST `Stats`
  reads the three with three acquisitions. This decision does not change them: they were already
  per-value accessors, no statistics field combines the three, and they expose no lock or field.
  A caller that must read them consistently with each other uses `read_metrics()` directly.
- **Debug logging stays lazy.** An event handler that logs the metrics reads inside the
  `tracing::debug!` arguments, so the lock is taken only when `debug` is enabled:

  ```rust
  tracing::debug!("stats: {}", stats_repository.read_metrics(|metrics| format!("{metrics:?}")).await);
  ```

### Alternatives Considered

- **Return the read guard (the previous design).** Rejected: it makes the lock type public API and
  lets a caller hold the lock across an `.await`.
- **One accessor method per value as the way to read several values together.** Rejected: each
  call takes the lock again, so several values read together may not be consistent with each
  other. The `tracker-core` accessors above are the existing exception.
- **Return an owned snapshot of `Metrics`.** Rejected: it clones the whole collection on every
  read, which the #2517 spec already ruled out for event paths.

### Consequences

- The lock implementation and the `Metrics` layout can change without a breaking release.
- A closure cannot return a borrow of `Metrics` or `.await`. A future caller that needs either
  copies what it needs inside the closure.
- The closure's blocking behaviour is a caller obligation that the compiler does not check.

## Date

2026-10-15

## References

- [#2536](https://github.com/torrust/torrust-tracker/issues/2536): hide the statistics
  repositories' read lock and metric collection field (this change)
- [#2517](https://github.com/torrust/torrust-tracker/issues/2517): made the repositories' lock
  field private; this decision is its recorded follow-up
- [#1669](https://github.com/torrust/torrust-tracker/issues/1669): the packages overhaul EPIC
- [Adopt Independent Package Versioning](20260629000000_adopt_independent_package_versioning.md)
