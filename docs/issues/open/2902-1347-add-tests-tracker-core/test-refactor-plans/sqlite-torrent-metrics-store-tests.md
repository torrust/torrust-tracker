---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/databases/driver/sqlite/torrent_metrics_store.rs
status: done
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/databases/driver/sqlite/torrent_metrics_store.rs
    - packages/tracker-core/src/databases/driver/sqlite/auth_key_store.rs
    - packages/tracker-core/src/databases/driver/sqlite/mod.rs
    - packages/tracker-core/src/databases/traits/torrent_metrics.rs
---

# SQLite Torrent Metrics Store File Test Plan

Follow the shared [guidance](README.md). This plan covers ledger row 9,
`packages/tracker-core/src/databases/driver/sqlite/torrent_metrics_store.rs`.

## Current State

- **Unit-only coverage:** 27 / 46 lines (58.70%), 28 / 60 regions, 11 / 23 functions, at the S2
  baseline (`develop` at `83f3ecbbc`, stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`,
  `--all-features --lib`, 2026-11-22). Uncovered lines:
  - 15-16: a column of a stored torrent that cannot be decoded (`torrent_downloads_from_row`);
  - 18-20: a stored `completed` outside `u32`; 25-27: a stored info-hash that does not parse;
  - 36, 53, 69, 94, 104 and 124: the query or execute failure of `load_all_torrents_downloads`,
    `load_torrents_downloads`, `load_torrent_downloads`, `save_torrent_downloads`,
    `increase_downloads_for_torrent` and `increase_global_downloads`;
  - 73 and 75-77: the single-torrent read of a column that cannot be decoded or of a `completed`
    outside `u32`.
- **Existing tests:** none in the file. The shared driver suite (`run_sqlite_driver_tests`) saves and
  loads torrents, loads all of them, loads a requested subset and more than one query chunk holds,
  and saves, loads and increases the global total: the happy path only.
- **Module-owned decisions:**
  - A stored `completed` outside `u32`, or an info-hash that does not parse, is
    `MalformedDatabaseRecord`; a column of the wrong type or a failed query is the mapped `sqlx`
    error (`InvalidQuery`).
  - `save_torrent_downloads` inserts or overwrites; `increase_downloads_for_torrent` and
    `increase_global_downloads` only add one to an existing row and create nothing (the downloads
    repository saves 1 first).
  - The count is stored as a signed 64-bit integer: the largest `u32` must round-trip.

## Maintainer Decision

Plan approved on 2026-11-23, including the shared fixture.

## Shared Fixture

Row 8 built a SQLite store over a `TempDir` database inside its own test module. Rows 9 to 11 need
the same, so its construction moves to `sqlite/mod.rs` as a `#[cfg(test)] mod fixture` with
`migrated_store()` (the store over a temporary database with its tables created, returned with the
`EphemeralConfiguration` that owns the directory) and `execute(&store, sql)` (raw `sqlx` on the
store's pool). Row 8's tests call it; their cases and assertions do not change. This is its own
`test` commit before this file's tests.

## Test Design

All tests use `migrated_store()` and keep the returned configuration alive. A bad database is built by
raw SQL (`Broken::Row` with the SQL literals of the info-hash and of `completed`, or
`Broken::TorrentsTableMissing`), as in row 8.

| Test | Cases | Assert |
| --- | --- | --- |
| `load_all_torrents_downloads` reading a bad row (`rstest`) | malformed: `completed` of `-1`, `completed` of `4294967296`, an info-hash text that does not parse; invalid query: `completed` of `'many'`, an info-hash that is a blob, a missing `torrents` table | `MalformedDatabaseRecord` for the first three, `InvalidQuery` for the others |
| `load_torrents_downloads` reading a bad row (`rstest`) | the requested info-hash stored with each bad `completed` (`-1`, `4294967296`, `'many'`), and a missing table | `MalformedDatabaseRecord`, `MalformedDatabaseRecord`, `InvalidQuery`, `InvalidQuery` |
| `load_torrent_downloads` reading a bad row (`rstest`) | the same four | the same variants |
| `save_torrent_downloads` twice | `1` then `7` for the same info-hash | the stored value is 7 (overwrites) |
| `save_torrent_downloads` at the limit | `u32::MAX` | it loads back as `u32::MAX` |
| `increase_downloads_for_torrent` on an absent torrent | no row | `Ok`, and no row is stored |
| `increase_global_downloads` with no global row | no row | `Ok`, and no global total is stored |
| the three writes on a missing table (`rstest`) | `save_torrent_downloads`, `increase_downloads_for_torrent`, `increase_global_downloads` | `InvalidQuery` |

A stored info-hash that does not parse (or is a blob) cannot be found by `load_torrents_downloads` or
`load_torrent_downloads`, which bind a valid hash: only `load_all_torrents_downloads` reads such a
row, so only it has those cases. Lines 75-77 (single read) and 18-20 (shared mapper) are reached by
the `completed` cases.

## Property-Test Decision

No `quickcheck`: the inputs are SQL rows and named boundaries; the sign and range rule of `u32` is
pinned by `-1`, `4294967296` and `u32::MAX`.

## Named Mutations

| ID | Mutation | Expected killer |
| --- | --- | --- |
| M1 | the mapper saturates a `completed` outside `u32` (`unwrap_or(u32::MAX)`) | the `-1` and `4294967296` cases of `load_all_torrents_downloads` |
| M2 | the mapper accepts an info-hash that does not parse (a fixed hash) | the info-hash case of `load_all_torrents_downloads` |
| M3 | `load_torrent_downloads` saturates a `completed` outside `u32` | the `-1` and `4294967296` cases of the single read |
| M4 | `load_all_torrents_downloads` skips a row it cannot map | the malformed cases of `load_all_torrents_downloads` |
| M5 | `load_torrents_downloads` skips a row it cannot map | the malformed cases of the batch read |
| M6 | the save does nothing when the row exists (`DO NOTHING`) | the save-twice test |
| M7 | `increase_downloads_for_torrent` creates the row when absent (upsert) | the absent-torrent test |
| M8 | `increase_global_downloads` creates the row when absent (upsert) | the no-global-row test |
| M9 | `save_torrent_downloads` ignores a failed statement | the missing-table cases |
| M10 | `increase_downloads_for_torrent` ignores a failed statement | the missing-table cases |
| M11 | `increase_global_downloads` ignores a failed statement | the missing-table cases |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| T1 | DONE | The shared fixture in `sqlite/mod.rs`; row 8's tests use it. | All tests pass unchanged. |
| T2 | DONE | The tests of this plan. | They pass; the failure lines run except any recorded exception. |
| T3 | DONE | Run M1-M11. | Each is killed by its expected test. |
| T4 | DONE | Record the results and ask for the completed-file review. | This record and the ledger. |

## Results

- **Coverage** (stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, `--all-features --lib`, clean coverage
  target directory, 2026-11-23): the file 27 / 46 lines to 99 / 99, regions 28 / 60 to 143 / 143,
  functions 11 / 23 to 35 / 35 (the counts include the test module). The package's unit-only lines
  4,747 / 5,563 (85.33%) to 4,823 / 5,620 (85.82%), regions 5,238 / 6,198 (84.51%), functions
  822 / 1,035 (79.42%). No line of the file is left uncovered.
- **Tests:** 21 added (cases included), none changed: 213 to 234 unit tests in the package. The
  shared fixture (`sqlite/mod.rs`, `migrated_store()` and `execute()`) was moved in its own commit,
  `test(tracker-core): [#2902] share the SQLite store test fixture`, and row 8's tests use it
  unchanged.
- **Mutations** (each applied alone and reverted, the file compared with its copy afterwards):

  | ID | Result |
  | --- | --- |
  | M1 | Killed by the `-1` and `4294967296` cases of the all-torrents and requested-torrents reads |
  | M2 | Killed by the info-hash that does not parse, in the all-torrents read |
  | M3 | Killed by the `-1` and `4294967296` cases of the single-torrent read |
  | M4 | Killed by all five bad-row cases of the all-torrents read |
  | M5 | Killed by the three bad-count cases of the requested-torrents read |
  | M6 | Killed by `it_should_overwrite_the_stored_downloads_when_a_torrent_is_saved_again` |
  | M7 | Killed by the absent-torrent test |
  | M8 | Killed by the no-global-row test |
  | M9 | Killed by `it_should_report_a_failed_write_as_an_invalid_query`, saving a torrent |
  | M10 | Killed by the same test, increasing a torrent |
  | M11 | Killed by the same test, increasing the global downloads |

- **Review outcome:** the maintainer approved the completed file on 2026-11-23.
- **Validation:** `cargo clippy -p torrust-tracker-core --all-targets --all-features` (only the
  existing `axum::serve` warning), `cargo +nightly fmt --all -- --check` and
  `cargo machete --with-metadata` pass.
