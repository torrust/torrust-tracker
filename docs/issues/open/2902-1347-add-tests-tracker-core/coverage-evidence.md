---
doc-type: coverage-evidence
issue: 2902
package: torrust-tracker-core
measured-commit: 5833cdae98c0cfe3c2523c7f17e33bcfd3d7292f
measured-utc: 2026-11-18
---

# Coverage Evidence - `torrust-tracker-core`

Baseline for the sub-issue spec of #2902. Measured at `develop` commit `5833cdae9`
(full SHA `5833cdae98c0cfe3c2523c7f17e33bcfd3d7292f`)
(`Merge torrust/torrust-tracker#2900: chore(issues): [#2873] archive the closed spec and mark it DONE in EPIC #2003`),
with `cargo-llvm-cov 0.9.1` and stable Rust 1.100.0, from a clean workspace and a separate
`CARGO_LLVM_COV_TARGET_DIR`. Every figure is filtered to `packages/tracker-core/src/` (62 files, 50 with executable lines). The
totals include the helper functions of the in-file `#[cfg(test)]` modules (most unit tests live in the files they test), so
they are not production-only coverage; `tests/` is not counted. The package has no benches or examples.

**Re-measured on 2026-11-18.** The first measurement (2026-11-17) used `cargo-llvm-cov 0.6.16` and
Rust 1.99.1. After `stable` moved to Rust 1.100.0, `cargo-llvm-cov 0.6.16` reported 0% for every
file: Cargo 1.100.0 writes test binaries under `target/debug/build/<package>/<hash>/out/`, and
0.6.16 still read a stale binary from `target/debug/deps/`. At the maintainer's request every figure
of this issue was measured again with `cargo-llvm-cov 0.9.1` and Rust 1.100.0, from a clean
coverage target directory, at the same commits; the 1.99.1 figures are superseded and kept only in
the append-only progress logs. Two counting changes come with the new toolchain: `#[test]`
functions are not counted (so `peer_tests.rs`, which holds only tests, has no measured lines, and
test assertions no longer appear as uncovered lines), and each `tracing` macro adds a region for
its log-level check, which a test run with logging disabled leaves uncovered.

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

The container-gated row (T1b) is measured at the same commit, with the images `mysql:8.0` and
`postgres:16` (the tests' default tags, `mysql/mod.rs:115` and `postgres/mod.rs:102`; pulled with
the maintainer's permission on 2026-11-17 and reused from the local Docker cache) and the same
separate target directory:

```console
cargo llvm-cov clean --workspace
TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST=true TORRUST_TRACKER_CORE_RUN_POSTGRES_DRIVER_TEST=true \
  cargo llvm-cov -p torrust-tracker-core --all-features --json --output-path .tmp/cov/container.json
```

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Unit-only (`--all-features --lib`) | 157 unit | 4,009 / 5,054 (79.32%) | 4,338 / 5,518 (78.62%) | 686 / 953 (71.98%) |
| Integration-only (`--all-features --test integration`) | 12 integration | 827 / 1,931 (42.83%) | 712 / 2,033 (35.02%) | 145 / 484 (29.96%) |
| Aggregate (`--all-features`, all targets) | 157 unit + 12 integration | 4,138 / 5,099 (81.15%) | 4,433 / 5,570 (79.59%) | 702 / 957 (73.35%) |
| Unit-only without the `db-compatibility-tests` feature (`--lib`) | 155 unit (the two driver tests are not compiled) | 3,995 / 4,730 (84.46%) | 4,324 / 5,210 (82.99%) | 682 / 915 (74.54%) |
| Container-gated aggregate (`--all-features`, both `TORRUST_TRACKER_CORE_RUN_*_DRIVER_TEST=true`, Docker `mysql:8.0` and `postgres:16`) | 157 unit + 12 integration, both driver suites run | 4,725 / 5,099 (92.67%) | 4,979 / 5,570 (89.39%) | 808 / 957 (84.43%) |

Reading the scopes:

- The integration-only figure is low because the 12 tests drive the package through its container
  and cover only the announce, scrape, persisted-downloads and shutdown paths. It is not evidence
  for any unit seam.
- The MySQL and PostgreSQL driver tests (`run_mysql_driver_tests`, `run_postgres_driver_tests`) are
  compiled by `--all-features` but **skip themselves and pass** unless
  `TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST` / `TORRUST_TRACKER_CORE_RUN_POSTGRES_DRIVER_TEST` is
  set, and they then need Docker and the `mysql` / `postgres` images. So the four default rows above do
  **not** include them: the two drivers (746 measured lines in ten files, 0-4% covered) are measured here as
  uncovered. The fifth row above measures them with the suites running (T1b, images
  `mysql:8.0` and `postgres:16`, the tests' defaults): the drivers' `mod.rs` files reach 94%, the
  schema migrators 83-84%, and each store 53-59%: the same failure branches (malformed row,
  failed query) that the SQLite stores leave uncovered. So the container-gated suite already
  protects the server drivers' happy path; the remaining gap there is the failure branches (F2).
- Aggregate and unit-only lines differ in five files only, all under `statistics/`
  (`event/handler.rs` 175 / 196 → 191 / 196 lines, `event/listener.rs` 360 / 411 → 423 / 426,
  `metrics.rs` 11 → 20 of 45, `persisted/mod.rs` 0 / 11 → 26 / 32, `repository.rs` 85 / 139 → 100 / 148). That is the
  one area where the integration binary protects code no unit test reaches; the EPIC rule says it
  still needs focused unit tests (slice S1). The aggregate must not be used to claim unit coverage.
- The EPIC tables for the other packages use `--all-features --lib`; the same command is the
  unit-only baseline here (79.32%). The scope without the feature (84.46%) is the one that matches
  what a contributor's plain `cargo test -p torrust-tracker-core` runs.

## Per-File Inventory

Lines only; the integration-only and aggregate columns show where the integration binary adds
anything, and the last column is the container-gated run (T1b) with both driver suites running (a `-`
means the file is not reached).

| File | Unit-only lines | Integration-only lines | Aggregate lines | Container-gated aggregate lines |
| --- | ---: | ---: | ---: | ---: |
| `announce_handler.rs` | 388 / 389 (99.74%) | 80 / 92 (86.96%) | 388 / 389 (99.74%) | 388 / 389 (99.74%) |
| `authentication/handler.rs` | 366 / 366 (100.00%) | 6 / 90 (6.67%) | 366 / 366 (100.00%) | 366 / 366 (100.00%) |
| `authentication/key/mod.rs` | 77 / 77 (100.00%) | 0 / 31 (0.00%) | 77 / 77 (100.00%) | 77 / 77 (100.00%) |
| `authentication/key/peer_key.rs` | 99 / 100 (99.00%) | 0 / 44 (0.00%) | 99 / 100 (99.00%) | 99 / 100 (99.00%) |
| `authentication/key/repository/in_memory.rs` | 91 / 91 (100.00%) | 0 / 20 (0.00%) | 91 / 91 (100.00%) | 91 / 91 (100.00%) |
| `authentication/key/repository/persisted.rs` | 73 / 74 (98.65%) | 5 / 20 (25.00%) | 73 / 74 (98.65%) | 73 / 74 (98.65%) |
| `authentication/mod.rs` | 114 / 114 (100.00%) | - | 114 / 114 (100.00%) | 114 / 114 (100.00%) |
| `authentication/service.rs` | 143 / 143 (100.00%) | 6 / 29 (20.69%) | 143 / 143 (100.00%) | 143 / 143 (100.00%) |
| `container.rs` | 136 / 138 (98.55%) | 71 / 73 (97.26%) | 136 / 138 (98.55%) | 136 / 138 (98.55%) |
| `databases/driver/mod.rs` | 186 / 189 (98.41%) | - | 186 / 189 (98.41%) | 188 / 189 (99.47%) |
| `databases/driver/mysql/auth_key_store.rs` | 0 / 55 (0.00%) | 0 / 55 (0.00%) | 0 / 55 (0.00%) | 29 / 55 (52.73%) |
| `databases/driver/mysql/mod.rs` | 7 / 211 (3.32%) | 0 / 37 (0.00%) | 7 / 211 (3.32%) | 199 / 211 (94.31%) |
| `databases/driver/mysql/schema_migrator.rs` | 0 / 77 (0.00%) | 0 / 77 (0.00%) | 0 / 77 (0.00%) | 65 / 77 (84.42%) |
| `databases/driver/mysql/torrent_metrics_store.rs` | 0 / 46 (0.00%) | 0 / 46 (0.00%) | 0 / 46 (0.00%) | 27 / 46 (58.70%) |
| `databases/driver/mysql/whitelist_store.rs` | 0 / 31 (0.00%) | 0 / 31 (0.00%) | 0 / 31 (0.00%) | 17 / 31 (54.84%) |
| `databases/driver/postgres/auth_key_store.rs` | 0 / 55 (0.00%) | 0 / 55 (0.00%) | 0 / 55 (0.00%) | 29 / 55 (52.73%) |
| `databases/driver/postgres/mod.rs` | 7 / 188 (3.72%) | 0 / 38 (0.00%) | 7 / 188 (3.72%) | 177 / 188 (94.15%) |
| `databases/driver/postgres/schema_migrator.rs` | 0 / 6 (0.00%) | 0 / 6 (0.00%) | 0 / 6 (0.00%) | 5 / 6 (83.33%) |
| `databases/driver/postgres/torrent_metrics_store.rs` | 0 / 46 (0.00%) | 0 / 46 (0.00%) | 0 / 46 (0.00%) | 27 / 46 (58.70%) |
| `databases/driver/postgres/whitelist_store.rs` | 0 / 31 (0.00%) | 0 / 31 (0.00%) | 0 / 31 (0.00%) | 17 / 31 (54.84%) |
| `databases/driver/sqlite/auth_key_store.rs` | 29 / 55 (52.73%) | 0 / 55 (0.00%) | 29 / 55 (52.73%) | 29 / 55 (52.73%) |
| `databases/driver/sqlite/mod.rs` | 81 / 91 (89.01%) | 28 / 36 (77.78%) | 81 / 91 (89.01%) | 81 / 91 (89.01%) |
| `databases/driver/sqlite/schema_migrator.rs` | 136 / 149 (91.28%) | 20 / 73 (27.40%) | 136 / 149 (91.28%) | 136 / 149 (91.28%) |
| `databases/driver/sqlite/torrent_metrics_store.rs` | 27 / 46 (58.70%) | 27 / 46 (58.70%) | 27 / 46 (58.70%) | 27 / 46 (58.70%) |
| `databases/driver/sqlite/whitelist_store.rs` | 17 / 31 (54.84%) | 0 / 31 (0.00%) | 17 / 31 (54.84%) | 17 / 31 (54.84%) |
| `databases/error.rs` | 28 / 35 (80.00%) | 0 / 23 (0.00%) | 28 / 35 (80.00%) | 35 / 35 (100.00%) |
| `databases/setup.rs` | 33 / 41 (80.49%) | 17 / 26 (65.38%) | 33 / 41 (80.49%) | 33 / 41 (80.49%) |
| `databases/traits/whitelist.rs` | 2 / 2 (100.00%) | 0 / 2 (0.00%) | 2 / 2 (100.00%) | 2 / 2 (100.00%) |
| `error.rs` | 38 / 50 (76.00%) | 0 / 12 (0.00%) | 38 / 50 (76.00%) | 38 / 50 (76.00%) |
| `lib.rs` | 57 / 57 (100.00%) | - | 57 / 57 (100.00%) | 57 / 57 (100.00%) |
| `scrape_handler.rs` | 184 / 184 (100.00%) | 41 / 42 (97.62%) | 184 / 184 (100.00%) | 184 / 184 (100.00%) |
| `statistics/event/handler.rs` | 175 / 196 (89.29%) | 88 / 94 (93.62%) | 191 / 196 (97.45%) | 191 / 196 (97.45%) |
| `statistics/event/listener.rs` | 360 / 411 (87.59%) | 155 / 167 (92.81%) | 423 / 426 (99.30%) | 423 / 426 (99.30%) |
| `statistics/metrics.rs` | 11 / 45 (24.44%) | 20 / 45 (44.44%) | 20 / 45 (44.44%) | 20 / 45 (44.44%) |
| `statistics/mod.rs` | 42 / 43 (97.67%) | 42 / 43 (97.67%) | 42 / 43 (97.67%) | 42 / 43 (97.67%) |
| `statistics/persisted/downloads.rs` | 77 / 77 (100.00%) | 39 / 39 (100.00%) | 77 / 77 (100.00%) | 77 / 77 (100.00%) |
| `statistics/persisted/mod.rs` | 0 / 11 (0.00%) | 26 / 32 (81.25%) | 26 / 32 (81.25%) | 26 / 32 (81.25%) |
| `statistics/repository.rs` | 85 / 139 (61.15%) | 61 / 116 (52.59%) | 100 / 148 (67.57%) | 100 / 148 (67.57%) |
| `test_helpers.rs` | 110 / 124 (88.71%) | - | 110 / 124 (88.71%) | 110 / 124 (88.71%) |
| `torrent/manager.rs` | 150 / 156 (96.15%) | 15 / 54 (27.78%) | 150 / 156 (96.15%) | 150 / 156 (96.15%) |
| `torrent/persisted_downloads.rs` | 22 / 22 (100.00%) | 22 / 22 (100.00%) | 22 / 22 (100.00%) | 22 / 22 (100.00%) |
| `torrent/repository/in_memory.rs` | 56 / 60 (93.33%) | 30 / 60 (50.00%) | 56 / 60 (93.33%) | 56 / 60 (93.33%) |
| `torrent/services.rs` | 272 / 272 (100.00%) | 0 / 1 (0.00%) | 272 / 272 (100.00%) | 272 / 272 (100.00%) |
| `whitelist/authorization.rs` | 71 / 71 (100.00%) | 12 / 16 (75.00%) | 71 / 71 (100.00%) | 71 / 71 (100.00%) |
| `whitelist/manager.rs` | 97 / 97 (100.00%) | 6 / 26 (23.08%) | 97 / 97 (100.00%) | 97 / 97 (100.00%) |
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
**no decision** (logging-only code, logging-macro regions, code-region artifacts); **test code** (lines in a `#[cfg(test)]`
module or in test support). A
"hypothesis" is not a decision: each group is re-challenged per file in the decision records
(hard to test, owned elsewhere, or merely unselected?).

| File | Uncovered lines | What they are | Class | Hypothesis |
| --- | --- | --- | --- | --- |
| `databases/driver/mysql/*` (5 files), `postgres/*` (5 files) | 0-4% of 728 measured lines | The whole MySQL and PostgreSQL drivers: schema migrator, three stores, `mod.rs` pool code | 3B (container-gated) | With the suites running the files reach 58-96% (T1b). The suite exists (`run_tests`) and runs in the `db-compatibility` workflow; the uncovered rest is the failure branches (a malformed row needs a hand-written row). Candidates for 3A: error mapping with an unreachable endpoint (no container) |
| `databases/driver/sqlite/auth_key_store.rs` | 18-100, 124-126 | Row-parse failure branches (`MalformedDatabaseRecord` for a bad key or a negative `valid_until`), `sqlx` failures, `parse_valid_until` negative case | 3A | Test through a temp SQLite file with a malformed row inserted by raw `sqlx`; the same helper is copied in three drivers (finding F2) |
| `databases/driver/sqlite/torrent_metrics_store.rs` | 15-124 (16 of 45 lines uncovered) | Aggregate metric load/save error branches, `u32` conversion failure (`MalformedDatabaseRecord`) | 3A | Same technique |
| `databases/driver/sqlite/whitelist_store.rs` | 18-75 (11 of 28) | Row-parse and query failure branches | 3A | Same technique |
| `databases/driver/sqlite/mod.rs` | production: 54, 58, 60-62, 79; test module: 103, 119 | Production: the error branches of the aggregate-metric helpers (`load_torrent_aggregate_metric`: query failure, `try_get` failure, the `u32` conversion to `MalformedDatabaseRecord`; `save_torrent_aggregate_metric`: query failure). Test module: lines 103 and 119 are `unreachable!` arms of the test helpers | 3A (production), test code (test module) | A row with an out-of-range value inserted by raw `sqlx`; a dropped table. `Sqlite::new` (lines 38-47) is fully covered |
| `databases/driver/sqlite/schema_migrator.rs` | production: 38, 56, 90, 103, 122-123, 136, 139, 152; test module: 275 | Production: `?` error regions of the legacy bootstrap and `drop_database_tables`. Test module: line 275 is an assertion | mostly no decision (production), test code (test module) | Legacy partial-state and migrated paths are already tested; challenge each `?` region |
| `databases/driver/mod.rs` | test module: 67-68, 70 | Retry loop of the test-only `create_database_tables` helper | test support | No-change |
| `databases/error.rs` | production: 140-141, 143-147 | `From<(MigrateError, Driver)>` (the `MigrationError` mapping) | 3A | One test of the migration-error mapping; the `sqlx::Error` arms are already tested |
| `databases/setup.rs` | production: 114-128 | `initialize_database_from_configuration` for MySQL and PostgreSQL | 3B (success path) or 3A (unreachable endpoint) | Unreachable-endpoint error test is 3A; the success path is container-gated |
| `error.rs` | production: 47-49, 53-55, 59-61, 65-67 | The four `From` conversions into `TrackerCoreError` | 3A | Trivial but package-owned wiring; one table test |
| `statistics/repository.rs` | production: 104, 114-120, 139-146, 148-154, 157-158, 164-206 | `set_counter`, `set_gauge`, `increment_gauge`, `decrement_gauge` and their error-logging arms | 3A | Covered only by the integration binary today; direct unit tests (success and metric-type-mismatch error) |
| `statistics/metrics.rs` | 36-44, 49-57, 62-69, 74-81 | `Metrics::set_counter`, `set_gauge`, `increment_gauge`, `decrement_gauge` | 3A | Same |
| `statistics/persisted/mod.rs` | 21-25, 59-61, 65-67 | `load_persisted_metrics` (startup load into the in-memory counters) | 3A | Unit test with the `testing` feature's `TorrentMetricsStore` mock (ADR on test support) or a temp SQLite store |
| `statistics/persisted/downloads.rs` | production: 133 | Production: the `?` of `increase_global_downloads` when `load_global_downloads` fails | 3A (medium) | Failing-`Database` case |
| `statistics/event/listener.rs` | production: 19-83; logging-macro regions: 106, 134, 168, 186, 227, 231; test module: 283-284 | Production: the two spawning wrappers (`run_*_event_listener`) and the `*_unspawned` entry points. Test module: lines 283-284 belong to the test `ScriptedReceiver` | 3A (production), no decision (logging-macro regions), test code (test module) | Spawn, send an event, cancel, await the handle; drain behavior is owned by the `events` package |
| `statistics/event/handler.rs` | production: 19-58, 129; logging-macro regions: 66, 106, 108, 120 | Logging-only handlers (`handle_torrent_added`, `_removed`, `handle_peer_*`) and one error arm | no decision (logging-only arms and logging-macro regions), 3A (line 129) | The logging-only arms carry no state change; decide whether a log-assertion test (ADR on log identifiers) is worth it; line 129 is a metric-increment error arm |
| `statistics/mod.rs` | 49 | Production: the implicit `else` of `if tracker_usage_statistics_enabled` (the in-session counter is not described when usage statistics are disabled) | 3A | A metrics test with usage statistics disabled |
| `container.rs` | production: 86 | The `Persistence` error mapping when the database cannot initialize | 3A | Container built over an invalid SQLite path |
| `authentication/handler.rs` | production: 252, 280 | The `?` regions of `remove_peer_key` and `load_peer_keys_from_database` (database failure) | 3A | Fake the `Database` failing |
| `authentication/key/repository/persisted.rs` | production: 57, 71; test module: 96 | Production: the `?` regions of `remove` and `load_keys`. Test module: line 96 is an `unreachable!` arm of a test helper | 3A (production), test code (test module) | Same fake |
| `authentication/key/peer_key.rs` | production: 64 | The `_ => false` arm of `PeerKey`'s `PartialEq` (one key expiring, the other permanent) | 3A | One equality case |
| `authentication/key/mod.rs` | production: 113, 120 | The log-level regions of the two `tracing::debug!` calls in `generate_key` | no decision (logging-macro regions) | No-change |
| `announce_handler.rs` | production: 184, 189; test module: 634 | Production: the `?` regions of the announce (whitelist failure, downloads load failure). Test module: line 634 is an assertion | 3A (production), test code (test module) | Whitelist and database failure cases may exist elsewhere; challenge |
| `torrent/manager.rs` | production: 66; logging-macro regions: 120-123, 131-132 | Production: the `?` of `load_torrents_from_database` when the database fails | 3A (medium), no decision (logging-macro regions) | Failing-`Database` case |
| `torrent/repository/in_memory.rs` | 177-179 | `get_torrent_peers` delegating to the swarm registry | owned (swarm-coordination-registry) | A thin delegation test is cheap; decide per file |
| `whitelist/manager.rs` | production: 53, 66, 79 | The `?` regions of `add`, `remove` and `load` (database failure) | 3A | Same failing-`Database` fake |
| `whitelist/repository/persisted.rs` | production: 30, 36, 46, 52 | The `?` regions of the read and the write in `add` and `remove` | 3A | Same fake (finding F4 check) |
| `test_helpers.rs` | test module: 144-156, 209 | Test-support builder branch for persistent completed statistics | test support | Not package-owned behavior; no test by decision, unless a slice uses it |

Counts are by line class, not by decision: a "region" group can disappear when its neighbors are
tested. The grouping above is the T1 hypothesis the decision records revisit.

## Slice Measurements

Each slice closes with the four default scopes, measured at the slice's branch head and at the
`develop` commit it is based on, so the slice's own change is the difference between the two. Stable
Rust 1.100.0, `cargo-llvm-cov 0.9.1`, a clean coverage target directory, filtered to
`packages/tracker-core/src/`.

### S1 (`statistics/`)

Measured on 2026-11-21. Base: `develop` at `44ce734fe`, which also changed the integration tests (15
instead of 12). Head: the S1 branch at its commit
`docs(issues): [#2902] record the results of the statistics module tests` (the slice-close commit
changes documentation only).

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Unit-only, base | 157 unit | 4,032 / 5,075 (79.45%) | 4,352 / 5,530 (78.70%) | 677 / 944 (71.72%) |
| Unit-only, S1 | 180 unit | 4,739 / 5,614 (84.41%) | 5,051 / 6,113 (82.63%) | 771 / 1,015 (75.96%) |
| Integration-only, base and S1 | 15 integration | 827 / 1,683 (49.14%) | 712 / 1,711 (41.61%) | 145 / 442 (32.81%) |
| Aggregate, base | 157 unit + 15 integration | 4,161 / 5,120 (81.27%) | 4,447 / 5,582 (79.67%) | 693 / 948 (73.10%) |
| Aggregate, S1 | 180 unit + 15 integration | 4,739 / 5,614 (84.41%) | 5,051 / 6,113 (82.63%) | 771 / 1,015 (75.96%) |
| Unit-only without `db-compatibility-tests`, base | 155 unit | 4,018 / 4,751 (84.57%) | 4,338 / 5,222 (83.07%) | 673 / 906 (74.28%) |
| Unit-only without `db-compatibility-tests`, S1 | 178 unit | 4,725 / 5,290 (89.32%) | 5,037 / 5,805 (86.77%) | 767 / 977 (78.51%) |

At the S1 head the aggregate equals the unit-only figure line for line: the integration binary no
longer reaches any line of the package that the unit tests miss. Before S1 it added 129 lines, all
in the five `statistics/` files (see the baseline above). Every `statistics/` file is now fully
covered by lines; the regions left are the log-level checks of `tracing` calls, the unreachable
error arm of `load_persisted_metrics` (line 34) and test code. The per-file decisions are in the
[test-refactor plans](test-refactor-plans/README.md).

### S2 (the SQLite persistence boundary)

Measured on 2026-11-23. Base: `develop` at `f73a359f8`, which includes #2916 (the error texts of
`tracker-core` no longer name a source path) and its 10 new unit tests (191 to 201). Head: the S2 branch at its
commit `test(tracker-core): [#2902] adapt the tracker core error table to errors without source
locations` (the slice-close commit changes documentation only).

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Unit-only, base | 201 unit | 4,754 / 5,628 (84.47%) | 5,151 / 6,211 (82.93%) | 796 / 1,039 (76.61%) |
| Unit-only, S2 | 281 unit | 5,143 / 5,904 (87.11%) | 5,685 / 6,598 (86.16%) | 901 / 1,097 (82.13%) |
| Integration-only, base | 15 integration | 827 / 1,918 (43.12%) | 712 / 2,032 (35.04%) | 145 / 484 (29.96%) |
| Integration-only, S2 | 15 integration | 827 / 1,876 (44.08%) | 712 / 1,997 (35.65%) | 145 / 477 (30.40%) |
| Aggregate, base | 201 unit + 15 integration | 4,754 / 5,628 (84.47%) | 5,151 / 6,211 (82.93%) | 796 / 1,039 (76.61%) |
| Aggregate, S2 | 281 unit + 15 integration | 5,143 / 5,904 (87.11%) | 5,685 / 6,598 (86.16%) | 901 / 1,097 (82.13%) |
| Unit-only without `db-compatibility-tests`, base | 199 unit | 4,740 / 5,304 (89.37%) | 5,137 / 5,903 (87.02%) | 792 / 1,001 (79.12%) |
| Unit-only without `db-compatibility-tests`, S2 | 279 unit | 5,129 / 5,580 (91.92%) | 5,671 / 6,290 (90.16%) | 897 / 1,059 (84.70%) |

The aggregate still equals the unit-only figure line for line, as after S1. The integration-only
line count changes between base and S2 only because the library code it measures changed (the shared
key parsing moved into `databases/driver/mod.rs`); its tests did not. The MySQL and PostgreSQL
drivers (S3) are the largest uncovered area left in the default scopes; the SQLite persistence boundary
and the error conversions are covered, and the lines left in those files are recorded with their
reasons in the [test-refactor plans](test-refactor-plans/README.md).

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
