---
doc-type: coverage-evidence
issue: 2902
package: torrust-tracker-core
measured-commit: 5833cdae98c0cfe3c2523c7f17e33bcfd3d7292f
measured-utc: 2026-11-17
---

# Coverage Evidence - `torrust-tracker-core`

Baseline for the sub-issue spec of #2902. Measured on 2026-11-17 at `develop` commit `5833cdae9`
(full SHA `5833cdae98c0cfe3c2523c7f17e33bcfd3d7292f`)
(`Merge torrust/torrust-tracker#2900: chore(issues): [#2873] archive the closed spec and mark it DONE in EPIC #2003`),
with `cargo-llvm-cov 0.6.16` and stable Rust 1.99.1, from a clean workspace and a separate
`CARGO_LLVM_COV_TARGET_DIR`. Every figure is filtered to `packages/tracker-core/src/` (62 files, 51 with executable lines). The
totals include the in-file `#[cfg(test)]` modules (most unit tests live in the files they test), so
they are not production-only coverage; `tests/` is not counted. The package has no benches or examples.

```console
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-core --all-features --lib --json --output-path .tmp/cov/unit.json
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-core --all-features --test integration --json --output-path .tmp/cov/integration.json
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-core --all-features --json --output-path .tmp/cov/aggregate.json
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-core --lib --json --output-path .tmp/cov/unit-default.json
```

The container-gated row (T1b) was measured afterwards on the S0 branch at `690cfb5f5`
(`690cfb5f5d2dbd061fdfe47a9353be608544966a`), whose
`packages/` tree is identical to `5833cdae9` (`git diff --stat 5833cdae9 690cfb5f5 -- packages` prints
nothing), after `docker pull mysql:8.0` and `docker pull postgres:16` (the tests' default tags,
`mysql/mod.rs:115` and `postgres/mod.rs:102`), with the same separate target directory:

```console
cargo llvm-cov clean --workspace
TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST=true TORRUST_TRACKER_CORE_RUN_POSTGRES_DRIVER_TEST=true \
  cargo llvm-cov -p torrust-tracker-core --all-features --json --output-path .tmp/cov/container.json
```

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Unit-only (`--all-features --lib`) | 157 unit | 4,099 / 5,104 (80.31%) | 5,716 / 7,127 (80.20%) | 687 / 954 (72.01%) |
| Integration-only (`--all-features --test integration`) | 12 integration | 841 / 1,909 (44.05%) | 936 / 2,506 (37.35%) | 145 / 484 (29.96%) |
| Aggregate (`--all-features`, all targets) | 157 unit + 12 integration | 4,230 / 5,149 (82.15%) | 5,834 / 7,182 (81.23%) | 703 / 958 (73.38%) |
| Unit-only without the `db-compatibility-tests` feature (`--lib`) | 155 unit (the two driver tests are not compiled) | 4,085 / 4,780 (85.46%) | 5,700 / 6,691 (85.19%) | 683 / 916 (74.56%) |
| Container-gated aggregate (`--all-features`, both `TORRUST_TRACKER_CORE_RUN_*_DRIVER_TEST=true`, Docker `mysql:8.0` and `postgres:16`) | 157 unit + 12 integration, both driver suites run | 4,830 / 5,149 (93.80%) | 6,723 / 7,182 (93.61%) | 809 / 958 (84.45%) |

Reading the scopes:

- The integration-only figure is low because the 12 tests drive the package through its container
  and cover only the announce, scrape, persisted-downloads and shutdown paths. It is not evidence
  for any unit seam.
- The MySQL and PostgreSQL driver tests (`run_mysql_driver_tests`, `run_postgres_driver_tests`) are
  compiled by `--all-features` but **skip themselves and pass** unless
  `TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST` / `TORRUST_TRACKER_CORE_RUN_POSTGRES_DRIVER_TEST` is
  set, and they then need Docker and the `mysql` / `postgres` images. So the four default rows above do
  **not** include them: the two drivers (728 measured lines in ten files, 0-4% covered) are measured here as
  uncovered. The fifth row above measures them with the suites running (T1b, 2026-11-17, images
  `mysql:8.0` and `postgres:16`, the tests' defaults): the drivers' `mod.rs` files reach 95.8%, the
  schema migrators 83-88%, and each store 58-64%: the same failure branches (malformed row,
  failed query) that the SQLite stores leave uncovered. So the container-gated suite already
  protects the server drivers' happy path; the remaining gap there is the failure branches (F2).
- Aggregate and unit-only lines differ in five files only, all under `statistics/`
  (`event/handler.rs` 179 → 198 of 203 lines, `event/listener.rs` 357 → 419 of 421, `metrics.rs`
  11 → 20 of 45, `persisted/mod.rs` 0 → 26 of 32, `repository.rs` 86 → 101 of 149). That is the
  one area where the integration binary protects code no unit test reaches; the EPIC rule says it
  still needs focused unit tests (slice S1). The aggregate must not be used to claim unit coverage.
- The EPIC tables for the other packages use `--all-features --lib`; the same command is the
  unit-only baseline here (80.31%). The scope without the feature (85.46%) is the one that matches
  what a contributor's plain `cargo test -p torrust-tracker-core` runs.

## Per-File Inventory

Lines only; the integration-only and aggregate columns show where the integration binary adds
anything, and the last column is the container-gated run (T1b) with both driver suites running (a `-`
means the file is not reached).

| File | Unit-only lines | Integration-only lines | Aggregate lines | Container-gated aggregate lines |
| --- | ---: | ---: | ---: | ---: |
| `announce_handler.rs` | 400 / 401 (99.75%) | 80 / 92 (86.96%) | 400 / 401 (99.75%) | 400 / 401 (99.75%) |
| `authentication/handler.rs` | 371 / 371 (100.00%) | 6 / 90 (6.67%) | 371 / 371 (100.00%) | 371 / 371 (100.00%) |
| `authentication/key/mod.rs` | 77 / 77 (100.00%) | 0 / 31 (0.00%) | 77 / 77 (100.00%) | 77 / 77 (100.00%) |
| `authentication/key/peer_key.rs` | 100 / 101 (99.01%) | 0 / 45 (0.00%) | 100 / 101 (99.01%) | 100 / 101 (99.01%) |
| `authentication/key/repository/in_memory.rs` | 92 / 92 (100.00%) | 0 / 21 (0.00%) | 92 / 92 (100.00%) | 92 / 92 (100.00%) |
| `authentication/key/repository/persisted.rs` | 73 / 74 (98.65%) | 5 / 20 (25.00%) | 73 / 74 (98.65%) | 73 / 74 (98.65%) |
| `authentication/mod.rs` | 123 / 123 (100.00%) | - | 123 / 123 (100.00%) | 123 / 123 (100.00%) |
| `authentication/service.rs` | 144 / 144 (100.00%) | 6 / 30 (20.00%) | 144 / 144 (100.00%) | 144 / 144 (100.00%) |
| `container.rs` | 143 / 144 (99.31%) | 72 / 73 (98.63%) | 143 / 144 (99.31%) | 143 / 144 (99.31%) |
| `databases/driver/mod.rs` | 185 / 188 (98.40%) | - | 185 / 188 (98.40%) | 187 / 188 (99.47%) |
| `databases/driver/mysql/auth_key_store.rs` | 0 / 50 (0.00%) | 0 / 50 (0.00%) | 0 / 50 (0.00%) | 29 / 50 (58.00%) |
| `databases/driver/mysql/mod.rs` | 7 / 212 (3.30%) | 0 / 38 (0.00%) | 7 / 212 (3.30%) | 203 / 212 (95.75%) |
| `databases/driver/mysql/schema_migrator.rs` | 0 / 75 (0.00%) | 0 / 75 (0.00%) | 0 / 75 (0.00%) | 66 / 75 (88.00%) |
| `databases/driver/mysql/torrent_metrics_store.rs` | 0 / 45 (0.00%) | 0 / 45 (0.00%) | 0 / 45 (0.00%) | 29 / 45 (64.44%) |
| `databases/driver/mysql/whitelist_store.rs` | 0 / 28 (0.00%) | 0 / 28 (0.00%) | 0 / 28 (0.00%) | 17 / 28 (60.71%) |
| `databases/driver/postgres/auth_key_store.rs` | 0 / 50 (0.00%) | 0 / 50 (0.00%) | 0 / 50 (0.00%) | 29 / 50 (58.00%) |
| `databases/driver/postgres/mod.rs` | 7 / 189 (3.70%) | 0 / 39 (0.00%) | 7 / 189 (3.70%) | 181 / 189 (95.77%) |
| `databases/driver/postgres/schema_migrator.rs` | 0 / 6 (0.00%) | 0 / 6 (0.00%) | 0 / 6 (0.00%) | 5 / 6 (83.33%) |
| `databases/driver/postgres/torrent_metrics_store.rs` | 0 / 45 (0.00%) | 0 / 45 (0.00%) | 0 / 45 (0.00%) | 29 / 45 (64.44%) |
| `databases/driver/postgres/whitelist_store.rs` | 0 / 28 (0.00%) | 0 / 28 (0.00%) | 0 / 28 (0.00%) | 17 / 28 (60.71%) |
| `databases/driver/sqlite/auth_key_store.rs` | 29 / 50 (58.00%) | 0 / 50 (0.00%) | 29 / 50 (58.00%) | 29 / 50 (58.00%) |
| `databases/driver/sqlite/mod.rs` | 85 / 92 (92.39%) | 32 / 37 (86.49%) | 85 / 92 (92.39%) | 85 / 92 (92.39%) |
| `databases/driver/sqlite/schema_migrator.rs` | 145 / 155 (93.55%) | 22 / 73 (30.14%) | 145 / 155 (93.55%) | 145 / 155 (93.55%) |
| `databases/driver/sqlite/torrent_metrics_store.rs` | 29 / 45 (64.44%) | 29 / 45 (64.44%) | 29 / 45 (64.44%) | 29 / 45 (64.44%) |
| `databases/driver/sqlite/whitelist_store.rs` | 17 / 28 (60.71%) | 0 / 28 (0.00%) | 17 / 28 (60.71%) | 17 / 28 (60.71%) |
| `databases/error.rs` | 28 / 35 (80.00%) | 0 / 23 (0.00%) | 28 / 35 (80.00%) | 35 / 35 (100.00%) |
| `databases/setup.rs` | 34 / 44 (77.27%) | 18 / 29 (62.07%) | 34 / 44 (77.27%) | 34 / 44 (77.27%) |
| `databases/traits/whitelist.rs` | 2 / 2 (100.00%) | 0 / 2 (0.00%) | 2 / 2 (100.00%) | 2 / 2 (100.00%) |
| `error.rs` | 38 / 50 (76.00%) | 0 / 12 (0.00%) | 38 / 50 (76.00%) | 38 / 50 (76.00%) |
| `lib.rs` | 58 / 58 (100.00%) | - | 58 / 58 (100.00%) | 58 / 58 (100.00%) |
| `peer_tests.rs` | 30 / 30 (100.00%) | - | 30 / 30 (100.00%) | 30 / 30 (100.00%) |
| `scrape_handler.rs` | 190 / 190 (100.00%) | 43 / 44 (97.73%) | 190 / 190 (100.00%) | 190 / 190 (100.00%) |
| `statistics/event/handler.rs` | 179 / 203 (88.18%) | 92 / 99 (92.93%) | 198 / 203 (97.54%) | 198 / 203 (97.54%) |
| `statistics/event/listener.rs` | 357 / 406 (87.93%) | 156 / 168 (92.86%) | 419 / 421 (99.52%) | 419 / 421 (99.52%) |
| `statistics/metrics.rs` | 11 / 45 (24.44%) | 20 / 45 (44.44%) | 20 / 45 (44.44%) | 20 / 45 (44.44%) |
| `statistics/mod.rs` | 40 / 40 (100.00%) | 40 / 40 (100.00%) | 40 / 40 (100.00%) | 40 / 40 (100.00%) |
| `statistics/persisted/downloads.rs` | 77 / 77 (100.00%) | 39 / 39 (100.00%) | 77 / 77 (100.00%) | 77 / 77 (100.00%) |
| `statistics/persisted/mod.rs` | 0 / 11 (0.00%) | 26 / 32 (81.25%) | 26 / 32 (81.25%) | 26 / 32 (81.25%) |
| `statistics/repository.rs` | 86 / 140 (61.43%) | 60 / 115 (52.17%) | 101 / 149 (67.79%) | 101 / 149 (67.79%) |
| `test_helpers.rs` | 109 / 123 (88.62%) | - | 109 / 123 (88.62%) | 109 / 123 (88.62%) |
| `torrent/manager.rs` | 153 / 153 (100.00%) | 15 / 48 (31.25%) | 153 / 153 (100.00%) | 153 / 153 (100.00%) |
| `torrent/persisted_downloads.rs` | 22 / 22 (100.00%) | 22 / 22 (100.00%) | 22 / 22 (100.00%) | 22 / 22 (100.00%) |
| `torrent/repository/in_memory.rs` | 56 / 60 (93.33%) | 30 / 60 (50.00%) | 56 / 60 (93.33%) | 56 / 60 (93.33%) |
| `torrent/services.rs` | 267 / 267 (100.00%) | 0 / 1 (0.00%) | 267 / 267 (100.00%) | 267 / 267 (100.00%) |
| `whitelist/authorization.rs` | 74 / 74 (100.00%) | 12 / 16 (75.00%) | 74 / 74 (100.00%) | 74 / 74 (100.00%) |
| `whitelist/manager.rs` | 99 / 99 (100.00%) | 6 / 26 (23.08%) | 99 / 99 (100.00%) | 99 / 99 (100.00%) |
| `whitelist/mod.rs` | 18 / 18 (100.00%) | - | 18 / 18 (100.00%) | 18 / 18 (100.00%) |
| `whitelist/repository/in_memory.rs` | 51 / 51 (100.00%) | 0 / 17 (0.00%) | 51 / 51 (100.00%) | 51 / 51 (100.00%) |
| `whitelist/repository/persisted.rs` | 74 / 74 (100.00%) | 3 / 25 (12.00%) | 74 / 74 (100.00%) | 74 / 74 (100.00%) |
| `whitelist/setup.rs` | 7 / 7 (100.00%) | 7 / 7 (100.00%) | 7 / 7 (100.00%) | 7 / 7 (100.00%) |
| `whitelist/test_helpers.rs` | 12 / 12 (100.00%) | - | 12 / 12 (100.00%) | 12 / 12 (100.00%) |

## Uncovered Line Groups (unit-only, `--all-features --lib`)

Line numbers are those of `5833cdae9`. Each row covers one file, or one family of files with the same
class; where a file has a `#[cfg(test)]` module, the lines column labels its `production:` lines and
its `test module:` lines separately. A test-module line is never a decision to test. Classes: **3A** unit-testable now; **3B** needs a boundary
above a unit test (a container, a real database); **owned** by a collaborator or another issue;
**no decision** (logging-only code, code-region artifacts); **test code** (lines in a `#[cfg(test)]`
module or in test support). A
"hypothesis" is not a decision: each group is re-challenged per file in the decision records
(hard to test, owned elsewhere, or merely unselected?).

| File | Uncovered lines | What they are | Class | Hypothesis |
| --- | --- | --- | --- | --- |
| `databases/driver/mysql/*` (5 files), `postgres/*` (5 files) | 0-4% of 728 measured lines | The whole MySQL and PostgreSQL drivers: schema migrator, three stores, `mod.rs` pool code | 3B (container-gated) | With the suites running the files reach 58-96% (T1b). The suite exists (`run_tests`) and runs in the `db-compatibility` workflow; the uncovered rest is the failure branches (a malformed row needs a hand-written row). Candidates for 3A: error mapping with an unreachable endpoint (no container) |
| `databases/driver/sqlite/auth_key_store.rs` | 18-100, 124-126 | Row-parse failure branches (`MalformedDatabaseRecord` for a bad key or a negative `valid_until`), `sqlx` failures, `parse_valid_until` negative case | 3A | Test through a temp SQLite file with a malformed row inserted by raw `sqlx`; the same helper is copied in three drivers (finding F2) |
| `databases/driver/sqlite/torrent_metrics_store.rs` | 15-124 (16 of 45 lines uncovered) | Aggregate metric load/save error branches, `u32` conversion failure (`MalformedDatabaseRecord`) | 3A | Same technique |
| `databases/driver/sqlite/whitelist_store.rs` | 18-75 (11 of 28) | Row-parse and query failure branches | 3A | Same technique |
| `databases/driver/sqlite/mod.rs` | production: 54, 58, 60, 62, 79; test module: 103, 119 | Production: the error branches of the aggregate-metric helpers (`load_torrent_aggregate_metric`: query failure, `try_get` failure, the `u32` conversion to `MalformedDatabaseRecord`; `save_torrent_aggregate_metric`: query failure). Test module: lines 103 and 119 are `unreachable!` arms of the test helpers | 3A (production), test code (test module) | A row with an out-of-range value inserted by raw `sqlx`; a dropped table. `Sqlite::new` (lines 38-47) is fully covered |
| `databases/driver/sqlite/schema_migrator.rs` | production: 38, 56, 90, 103, 122-123, 136, 139, 152; test module: 275 | Production: `?` error regions of the legacy bootstrap and `drop_database_tables`. Test module: line 275 is an assertion | mostly no decision (production), test code (test module) | Legacy partial-state and migrated paths are already tested; challenge each `?` region |
| `databases/driver/mod.rs` | test module: 67-68, 70 | Retry loop of the test-only `create_database_tables` helper | test support | No-change |
| `databases/error.rs` | production: 140-147; test module: 160, 173 | Production: `From<(MigrateError, Driver)>` (the `MigrationError` mapping). Test module: lines 160 and 173 are assertions | 3A (production), test code (test module) | One test of the migration-error mapping; the `sqlx::Error` arms are already tested |
| `databases/setup.rs` | production: 114-128 | `initialize_database_from_configuration` for MySQL and PostgreSQL | 3B (success path) or 3A (unreachable endpoint) | Unreachable-endpoint error test is 3A; the success path is container-gated |
| `error.rs` | 47-49, 53-55, 59-61, 65-67 | The four `From` conversions into `TrackerCoreError` | 3A | Trivial but package-owned wiring; one table test |
| `statistics/repository.rs` | 104, 114-120, 139-146, 148-154, 157-158, 164-206 | `set_counter`, `set_gauge`, `increment_gauge`, `decrement_gauge` and their error-logging arms | 3A | Covered only by the integration binary today; direct unit tests (success and metric-type-mismatch error) |
| `statistics/metrics.rs` | 36-44, 49-57, 62-69, 74-81 | `Metrics::set_counter`, `set_gauge`, `increment_gauge`, `decrement_gauge` | 3A | Same |
| `statistics/persisted/mod.rs` | 21-25, 59-61, 65-67 | `load_persisted_metrics` (startup load into the in-memory counters) | 3A | Unit test with the `testing` feature's `TorrentMetricsStore` mock (ADR on test support) or a temp SQLite store |
| `statistics/persisted/downloads.rs` | production: 133 | Production: the `?` of `increase_global_downloads` when `load_global_downloads` fails | 3A (medium) | Failing-`Database` case |
| `statistics/event/listener.rs` | production: 19-83; test module: 283-284 | Production: the two spawning wrappers (`run_*_event_listener`) and the `*_unspawned` entry points. Test module: lines 283-284 belong to the test `ScriptedReceiver` | 3A (production), test code (test module) | Spawn, send an event, cancel, await the handle; drain behavior is owned by the `events` package |
| `statistics/event/handler.rs` | production: 19-58, 129 | Logging-only handlers (`handle_torrent_added`, `_removed`, `handle_peer_*`) and one error arm | no decision (logging-only arms), 3A (line 129) | The logging-only arms carry no state change; decide whether a log-assertion test (ADR on log identifiers) is worth it; line 129 is a metric-increment error arm |
| `statistics/mod.rs` | 49 | Production: the implicit `else` of `if tracker_usage_statistics_enabled` (the in-session counter is not described when usage statistics are disabled) | 3A | A metrics test with usage statistics disabled |
| `container.rs` | production: 86; test module: 193 | Production: the `Persistence` error mapping when the database cannot initialize. Test module: line 193 is an assertion | 3A (production), test code (test module) | Container built over an invalid SQLite path |
| `authentication/handler.rs` | production: 252, 280; test module: 424, 479, 493, 531, 604, 656, 690 | Production: the `?` regions of `remove_peer_key` and `load_peer_keys_from_database` (database failure). Test module: the other seven lines are assertions | 3A (production), test code (test module) | Fake the `Database` failing |
| `authentication/key/repository/persisted.rs` | production: 57, 71; test module: 96 | Production: the `?` regions of `remove` and `load_keys`. Test module: line 96 is an `unreachable!` arm of a test helper | 3A (production), test code (test module) | Same fake |
| `authentication/key/peer_key.rs` | production: 64 | The `_ => false` arm of `PeerKey`'s `PartialEq` (one key expiring, the other permanent) | 3A | One equality case |
| `authentication/key/mod.rs` | test module: 304 | An assertion | test code | No-change |
| `announce_handler.rs` | production: 184, 189; test module: 634 | Production: the `?` regions of the announce (whitelist failure, downloads load failure). Test module: line 634 is an assertion | 3A (production), test code (test module) | Whitelist and database failure cases may exist elsewhere; challenge |
| `scrape_handler.rs` | test module: 350 | Test module: an assertion | test code | No-change |
| `torrent/manager.rs` | production: 66 | Production: the `?` of `load_torrents_from_database` when the database fails | 3A (medium) | Failing-`Database` case |
| `torrent/repository/in_memory.rs` | 177-179 | `get_torrent_peers` delegating to the swarm registry | owned (swarm-coordination-registry) | A thin delegation test is cheap; decide per file |
| `whitelist/manager.rs` | production: 53, 66, 79 | The `?` regions of `add`, `remove` and `load` (database failure) | 3A | Same failing-`Database` fake |
| `whitelist/repository/persisted.rs` | production: 30, 36, 46, 52 | The `?` regions of the read and the write in `add` and `remove` | 3A | Same fake (finding F4 check) |
| `whitelist/authorization.rs` | test module: 139 | An assertion | test code | No-change |
| `test_helpers.rs` | 144-156, 209 | Test-support builder branch for persistent completed statistics | test support | Not package-owned behavior; no test by decision, unless a slice uses it |

Counts are by line class, not by decision: a "region" group can disappear when its neighbors are
tested. The grouping above is the T1 hypothesis the decision records revisit.

## Test-Technique Assessment (initial)

| Technique | Applicability | Initial decision |
| --- | --- | --- |
| Unit tests | The package is almost all in-process logic over traits | Primary level for every slice |
| Package-local integration (`tests/integration.rs`) | 12 tests over the container already exist | Keep; add only if a unit cannot reach the boundary (for example persistence plus listeners) |
| Container-gated database suite (MySQL, PostgreSQL) | Exists, skipped silently without the environment variables | Measured (T1b); change nothing without the maintainer's decision (Q2, Q3) |
| Runnable examples | None; `lib.rs` has doc examples | Out of scope |
| Property-based (`quickcheck`) | Candidates flagged below; the package has no property-test dependency | Per-seam decision in the records, with an `add-rust-dependency` justification |
| Mutation testing | Applicable after the last test increment | Bounded sample (T4) |
| Fuzzing | Not applicable (no parser of untrusted bytes in this package) | Not applicable |

Property-test candidates (the guidelines' two triggers):

- `authentication::key::Key` (`peer_key.rs`): `Key::new` / `value` and the random-key generator have a
  round-trip and length invariant (a generated key always parses; a parsed key always prints back to
  the same text; only 32 alphanumeric characters are accepted).
- `parse_valid_until` and the expiry arithmetic (`generate_expiring_key`, `verify_key_expiration`,
  `PeerKey::expiry_time`): bounds (negative stored timestamps, `u64` seconds beyond `i64`, lifetime
  overflow); the example-based tests cover the named boundaries, a property would cover the range.
- Persisted downloads (`u32` counters through `i64` columns): round-trip and saturation bounds.

Everything else (policy branches over two or three cases, delegation) stays example-based.
