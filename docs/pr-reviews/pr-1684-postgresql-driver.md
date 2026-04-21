# PR #1684 Review: PostgreSQL Driver Support

## Summary

This review covers both the original PR #1684 and its follow-up rework in PR #1695
(`DamnCrab:codex/pg-adaptation-rework`). PR #1684 was rejected due to a merge-blocking counter
overflow bug and an architectural concern about per-query OS thread spawning. The rework in PR #1695
addresses all four issues raised in that review. This document records the original findings and
documents how each was resolved.

---

## Issues Found in PR #1684

### 1. Counter Overflow Risk — was MERGE BLOCKER — ✅ Resolved in PR #1695

**Original finding:** The PR #1684 PostgreSQL driver stored download counters in signed 32-bit
`INTEGER` columns and converted through `i32` with `.unwrap()` calls. Any counter exceeding
2,147,483,647 would panic at runtime.

**Root cause:** The `r2d2_postgres` crate does not auto-downcast integer types, so the code had to
explicitly specify `i32`. Combined with a 32-bit `INTEGER` column, this created a hard overflow at
2 billion downloads.

**Resolution in rework:** The rework migrated to `sqlx`, which supports async-native PostgreSQL
access and handles type mapping directly. Counters are now stored and read as `i64`, with explicit
error-propagating conversions via `.map_err()` instead of `.unwrap()`. Relevant locations in
[packages/tracker-core/src/databases/driver/postgres.rs](../../packages/tracker-core/src/databases/driver/postgres.rs):

- Line 73: `fn decode_counter_i64(&self, value: i64) -> Result<...>` — converts `i64` to
  `NumberOfDownloads` with error propagation
- Line 77: `fn encode_counter(&self, value: NumberOfDownloads) -> Result<i64, Error>` — converts
  `NumberOfDownloads` to `i64` with error propagation
- Line 153: counter columns read as `i64` via `row.try_get("completed")`

No panicking `.unwrap()` calls remain on counter conversion paths.

### 2. Per-Query OS Thread Spawning — was Medium Priority — ✅ Resolved in PR #1695

**Original finding:** PR #1684 spawned and joined a fresh OS thread for every database operation
to work around a nested Tokio runtime conflict introduced by the `r2d2_postgres` / sync `postgres`
crate. This was not free: each query created an OS thread, allocated a stack, and paid context
switch overhead. Under high load this would have been measurable.

**Resolution in rework:** The rework replaced `r2d2_postgres` with `sqlx`, which is async-native.
The driver now runs directly on the existing Tokio runtime with no thread spawning. The connection
pool is managed by `sqlx`'s built-in async pool, making the PostgreSQL driver consistent with how
the SQLite and MySQL drivers will be migrated in subissue #1525-03.

### 3. Missing PostgreSQL Configuration File — was Medium Priority — ✅ Resolved in PR #1695

**Original finding:** No default PostgreSQL configuration file was provided in the deployment
directory, unlike the existing `tracker.container.mysql.toml`.

**Resolution in rework:**
`share/default/config/tracker.container.postgresql.toml` was added, parallel to the MySQL
container configuration.

### 4. Missing Database Migrations — was Medium Priority — ✅ Resolved in PR #1695

**Original finding:** No migration infrastructure existed. Adding a new backend with schema
requirements without migrations made schema evolution risky.

**Resolution in rework:** A full migrations directory was introduced at
`packages/tracker-core/migrations/` with subdirectories for each backend
(`mysql/`, `postgresql/`, `sqlite/`). Migrations are applied automatically when a database-backed
store is first used. Timestamps are shared across backends to keep schema evolution aligned.

---

## Validation Results (PR #1684)

✅ **Passed locally:**

- `cargo test -p bittorrent-tracker-core databases::error::tests`
- `TORRUST_TRACKER_CORE_RUN_POSTGRES_DRIVER_TEST=true cargo test -p bittorrent-tracker-core run_postgres_driver_tests`
- `cargo test -p torrust-tracker-configuration database`

✅ **Integration test:** PostgreSQL driver ran successfully against a real container in
testcontainers.

✅ **Configuration handling:** PostgreSQL URL masking and schema registration work correctly.

---

## Positive Aspects (PR #1684)

- Configuration and driver wiring were clean
- Error handling integration (`GenericConnectionError` variant) was appropriate
- Tests were comprehensive with both container and local database options
- Documentation was clear about runtime constraints

---

## Original Recommendation

PR #1684 was rejected. The main reason was architectural: the per-query OS thread spawning model
was below the minimum performance bar for merge, independent of the overflow bug.

The recommendation was to revisit PostgreSQL support alongside or after the persistence redesign
tracked in [issue #1525](https://github.com/torrust/torrust-tracker/issues/1525). PR #1695 follows
exactly that direction: it reworks the full persistence layer and delivers PostgreSQL on top of the
new async `sqlx` substrate in a single coherent redesign.

---

## Remaining Issues in PR #1695

Running `linter all` against the rework branch reveals failures that must be addressed before
PR #1695 can merge.

### Clippy errors (37)

Affected files (all in `packages/tracker-core`):

- `src/databases/driver/mysql.rs` — `unused_self`, `needless_pass_by_value`, casting `u64` to
  `usize`
- `src/databases/driver/postgres.rs` — same categories
- `src/databases/driver/sqlite.rs` — same categories
- `src/databases/mod.rs` — `all fields have the same postfix: keys`
- `src/torrent/services.rs` — `useless conversion to the same type: u64`
- `packages/torrent-repository-benchmarking/tests/repository/mod.rs` — casting truncation

The project's `.cargo/config.toml` sets `-D warnings` globally, so these are hard errors that
prevent compilation.

### Spell-checking failures

New words introduced by the rework are not yet in `project-words.txt`:
`sqlx`, `Sqlx`, `isready`, `getpid`, `qbittorrent`, `urandom`, `savepath`.
