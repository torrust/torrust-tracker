---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/3052-whitelist-add-remove-race/ISSUE.md
last-updated-utc: "2026-11-25 17:29"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-11-25 (V1 run before 14:15)
- Artifact under test: `torrust/develop` at merge commit 11c54fb48 ("Merge torrust/torrust-tracker#3047: chore: update dependencies"), unmodified production code plus the temporary test below.
- Operating system / environment: Linux, stable Rust toolchain `rustc 1.100.0`, SQLite on a temporary database created by `ephemeral_configuration_for_listed_tracker()`.

## Verification Processes

### V1 - Concurrent identical add and remove on SQLite (before the fix)

- Goal: observe the wrong outcome (a failure for an idempotent operation).
- Initial state: empty whitelist, fresh temporary SQLite database.
- Status: `DONE`
- Outcome: **Reproduced** (the wrong outcome itself was observed).

#### Steps Performed

1. Temporarily added the test below to `packages/tracker-core/src/whitelist/repository/persisted.rs`.
2. Ran `cargo test -p torrust-tracker-core --lib repro_concurrent -- --nocapture`.
3. Reverted the test (`git checkout` of the file). This temporary test is a measuring instrument: it counts
   failures and prints them, and it never asserts, so it passes even when every call fails. It is not the
   maintained regression test. T3 writes a separate asserting test (both concurrent `add` calls return
   `Ok` and the hash is whitelisted afterwards; both concurrent `remove` calls return `Ok` and the hash
   is gone afterwards, checked in every round), and the red evidence comes from that version.

```rust
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn repro_concurrent_add_and_remove() {
    use std::sync::Arc;
    let ephemeral = ephemeral_configuration_for_listed_tracker();
    let whitelist = Arc::new(initialize_database_whitelist(&ephemeral.config).await);
    let infohash = sample_info_hash();
    let (mut add_fail, mut rem_fail) = (0, 0);
    let mut first_err = None;
    for _ in 0..200 {
        whitelist.remove(&infohash).await.unwrap();
        let (a, b) = (whitelist.clone(), whitelist.clone());
        let (ra, rb) = tokio::join!(
            tokio::spawn(async move { a.add(&infohash).await }),
            tokio::spawn(async move { b.add(&infohash).await })
        );
        for r in [ra.unwrap(), rb.unwrap()] {
            if let Err(e) = r { add_fail += 1; first_err.get_or_insert(format!("{e:?}")); }
        }
        let (a, b) = (whitelist.clone(), whitelist.clone());
        let (ra, rb) = tokio::join!(
            tokio::spawn(async move { a.remove(&infohash).await }),
            tokio::spawn(async move { b.remove(&infohash).await })
        );
        for r in [ra.unwrap(), rb.unwrap()] {
            if let Err(e) = r { rem_fail += 1; first_err.get_or_insert(format!("{e:?}")); }
        }
        whitelist.add(&infohash).await.unwrap();
    }
    println!("REPRO add_failures={add_fail}/200 remove_failures={rem_fail}/200 first_err={first_err:?}");
}
```

#### Observed Result

```text
REPRO add_failures=200/200 remove_failures=200/200 first_err=Some("InvalidQuery { source: Database(SqliteError { code: 2067, message: \"UNIQUE constraint failed: whitelist.info_hash\" }), driver: Sqlite3 }")
test whitelist::repository::persisted::tests::the_persisted_whitelist_repository::repro_concurrent_add_and_remove ... ok
```

(`add_failures` counts failed calls: one per round. The first error is the UNIQUE violation. The error variant of the failing
removes was not printed in this run (only the first error was kept); `DeleteFailed` is predicted from the
code and is to be captured in the red run of T3.)

#### Conclusion

Both concurrent identical `add` and concurrent identical `remove` fail in every round on SQLite.

#### After the fix (V1 rerun)

Artifact under test: the commit `fix(tracker-core): [#3052] make the whitelist add and remove atomic and idempotent` and the harness test commit after it; the temporary measuring test
of V1, unchanged, run once locally and then removed (not committed).
Command: `cargo test -p torrust-tracker-core --lib repro_concurrent -- --nocapture`.

```text
REPRO add_failures=0/200 remove_failures=0/200 first_err=None
test result: ok. 1 passed
```

The committed asserting tests of T3 (`should_not_fail_on_concurrent_identical_adds`,
`should_not_fail_on_concurrent_identical_removes`) pass on the fixed code: `cargo test -p torrust-tracker-core --lib persisted::tests`
reports `15 passed; 0 failed`.

### T3 - Red run of the asserting concurrency tests (SQLite, before the fix)

Artifact under test: `torrust/develop` at the merge of #3053 (spec) plus the two new tests in
`packages/tracker-core/src/whitelist/repository/persisted.rs`
(`should_not_fail_on_concurrent_identical_adds`, `should_not_fail_on_concurrent_identical_removes`).
Both fail in the first rounds on the unfixed code. Command:
`cargo test -p torrust-tracker-core --lib persisted::tests`. Result: `13 passed; 2 failed`.

```text
concurrent add failed in round 0: Err(InvalidQuery { source: Database(SqliteError { code: 2067, message: "UNIQUE constraint failed: whitelist.info_hash" }), driver: Sqlite3 })
concurrent remove failed in round 1: Err(DeleteFailed { error_code: 0, driver: Sqlite3 })
```

This confirms the predicted `DeleteFailed` for a concurrent `remove`, which V1 did not print.

#### Concurrent store-level harness cases (SQLite, before the fix)

Added to the shared harness in `packages/tracker-core/src/databases/driver/mod.rs`
(`it_should_not_fail_adding_the_same_infohash_concurrently`,
`it_should_not_fail_removing_the_same_infohash_concurrently`): two concurrent identical store calls
(`tokio::join!`), 50 rounds, both must return `Ok` and their row counts must sum to 1.
Command: `cargo test -p torrust-tracker-core --lib run_sqlite_driver_tests`. The suite is one test, so the
add case stops it first; the remove case was observed by keeping the add case unregistered for one local
run (not committed).

```text
add:    called `Result::unwrap()` on an `Err` value: InvalidQuery { source: Database(SqliteError { code: 2067, message: "UNIQUE constraint failed: whitelist.info_hash" }), driver: Sqlite3 }
remove: called `Result::unwrap()` on an `Err` value: DeleteFailed { error_code: 0, driver: Sqlite3 }
```

The red run on the MySQL and PostgreSQL Docker suites is recorded under V2.

### V2 - MySQL and PostgreSQL

Status: DONE (before-fix red recorded; after-fix green recorded below).

Reproduced at store level with the concurrent harness cases of T3 (two concurrent identical
`add_info_hash_to_whitelist` or `remove_info_hash_from_whitelist` calls on the Docker databases),
against `torrust/develop` at the merge of #3053 plus the two T3 test commits. The suites are one test
each, so the add case stops it first; the remove case was observed by keeping the add case unregistered
for one local run (not committed). Commands:
`TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST=true cargo test -p torrust-tracker-core --features db-compatibility-tests --lib run_mysql_driver_tests`
and the same with `POSTGRES`. Docker containers were started by the suites.

```text
MySQL add:        InvalidQuery { source: Database(MySqlDatabaseError { code: Some("23000"), number: 1062, message: "Duplicate entry '...' for key 'whitelist.info_hash'" }), driver: MySQL }
PostgreSQL add:   InvalidQuery { source: Database(PgDatabaseError { ... code: "23505", message: "duplicate key value violates unique constraint \"whitelist_info_hash_key\"" ... }), driver: PostgreSQL }
MySQL remove:     DeleteFailed { error_code: 0, driver: MySQL }
PostgreSQL remove: DeleteFailed { error_code: 0, driver: PostgreSQL }
```

At store level there is no read-then-write window: a second identical `INSERT` always violates the
unique constraint and a second `DELETE` always removes zero rows, so these cases fail in the first
iteration whatever the interleaving, as the replaced sequential tests did. The red run therefore shows
that all three drivers reject the duplicate write, which the fix changes. It does not show the
`DatabaseWhitelist` read-then-write race: that was observed on SQLite only (V1 and the `persisted.rs`
tests). The MySQL affected-rows behavior of `ON DUPLICATE KEY UPDATE` is checked after the fix.

Container images: the suites ran with their default tags, `mysql:8.0` and `postgres:16`
(overridable with `TORRUST_TRACKER_CORE_MYSQL_DRIVER_IMAGE_TAG` and
`TORRUST_TRACKER_CORE_POSTGRES_DRIVER_IMAGE_TAG`, unset). CI runs the same suites on MySQL 8.0 and 8.4
and PostgreSQL 14 to 17.

#### After the fix (V2 rerun)

Artifact under test: the commit `fix(tracker-core): [#3052] make the whitelist add and remove atomic and idempotent` and the harness test commit after it. Commands as above; the Docker
containers were started by the suites.

```text
MySQL:      run_mysql_driver_tests      ... test result: ok. 1 passed
PostgreSQL: run_postgres_driver_tests   ... test result: ok. 1 passed
SQLite:     run_sqlite_driver_tests     ... test result: ok. 1 passed
```

Finding during this run, MySQL affected rows: the first version of the concurrent add case asserted that
the two row counts sum to 1. On MySQL it failed with `left: 2, right: 1`: the driver connects with the
found-rows flag, so `ON DUPLICATE KEY UPDATE info_hash = info_hash` reports 1 row on a duplicate, not 0.
SQLite and PostgreSQL report 0. The removal count is exact on all three drivers. Decision: the add
count is not a contract (documented on `WhitelistStore::add_info_hash_to_whitelist`); the harness
asserts only that both adds succeed and the hash is listed. Removal counts (`1` then `0`) are asserted.

## Test intent (prose-first Arrange-Act-Assert)

The prose was written from the requirement before comparing it with the test code. Each comparison
found the code matching the prose.

- `persisted.rs` `should_not_fail_on_concurrent_identical_adds`. Arrange: an empty persisted whitelist on
  SQLite shared by two tasks, one info-hash. Act: both tasks `add` the hash at the same time, 200
  rounds, removing it after each round. Assert: both calls return `Ok` and the hash is loaded from the
  database.
- `persisted.rs` `should_not_fail_on_concurrent_identical_removes`. Arrange: the hash is listed at the
  start of each round. Act: two tasks `remove` it at the same time. Assert: both calls return `Ok` and the
  hash is no longer loaded.
- Harness `it_should_not_fail_adding_the_same_infohash_concurrently`. Arrange: a fresh random hash per
  round. Act: two concurrent store `add` calls. Assert: both succeed and the hash is listed.
- Harness `it_should_not_fail_removing_the_same_infohash_concurrently`. Arrange: a listed random hash.
  Act: two concurrent store `remove` calls. Assert: both succeed, the removed counts sum to 1, and the
  hash is gone.
- Harness `it_should_not_fail_adding_the_same_infohash_twice`. Arrange: a fresh hash. Act: add it twice.
  Assert: both succeed and the hash is listed.
- Harness `it_should_not_fail_removing_a_missing_infohash`. Arrange: a hash never added. Act: remove
  it. Assert: success with 0 rows removed.
- Harness `it_should_not_fail_removing_the_same_infohash_twice`. Arrange: a listed hash. Act: remove it
  twice. Assert: counts `1` then `0` and the hash is gone.

## Failures and Follow-up

None yet.
