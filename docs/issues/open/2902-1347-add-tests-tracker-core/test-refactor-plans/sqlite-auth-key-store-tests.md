---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/databases/driver/sqlite/auth_key_store.rs
status: done
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/databases/driver/sqlite/auth_key_store.rs
    - packages/tracker-core/src/databases/driver/mysql/auth_key_store.rs
    - packages/tracker-core/src/databases/driver/postgres/auth_key_store.rs
    - packages/tracker-core/src/databases/driver/mod.rs
    - packages/tracker-core/src/databases/error.rs
---

# SQLite Auth Key Store File Test Plan

Follow the shared [guidance](README.md). This plan covers ledger row 8,
`packages/tracker-core/src/databases/driver/sqlite/auth_key_store.rs`, the first file of S2, and the
extraction of `parse_valid_until` that the maintainer decided on Q4 (finding F2).

## Current State

- **Unit-only coverage:** 29 / 55 lines (52.73%), 29 / 71 regions, 8 / 21 functions, at the S2
  baseline (`develop` at `83f3ecbbc`, stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`,
  `--all-features --lib`, 2026-11-22). Uncovered lines:
  - 18, 43, 79 and 100: the query or execute failure of `load_keys`, `get_key_from_keys`,
    `add_key_to_keys` and `remove_key_from_keys`;
  - 22-23 and 47-48: a column that cannot be decoded (`try_get`);
  - 26-28 and 51-53: a stored key that does not parse;
  - 32 and 57: the stored `valid_until` that `parse_valid_until` rejects, and 124-126, its negative
    arm;
  - 68-70: an expiry that does not fit a signed 64-bit integer;
  - 89-91: the `usize` conversion of the affected-row count.
- **Existing tests:** none in the file. The shared driver suite (`run_sqlite_driver_tests`, through
  `databases/driver/mod.rs`) saves, loads and removes permanent and expiring keys on a real database:
  the happy path only.
- **The duplicated code:** `parse_valid_until` is byte-identical in the SQLite, MySQL and
  PostgreSQL `auth_key_store.rs`, apart from the `DRIVER` constant each file defines, and the
  key-and-expiry parse of `load_keys` and `get_key_from_keys` is repeated twice in each of them. A
  test of one copy protects none of the others, and the Docker-free tests cannot reach the server
  drivers.
- **Module-owned decisions:**
  - A stored key that does not parse, or a negative `valid_until`, is `MalformedDatabaseRecord`; a
    column of the wrong type or a failed query is the `sqlx` error mapped by `databases/error.rs`
    (`InvalidQuery` for everything but I/O, pool and configuration failures).
  - An expiry that does not fit `i64` is refused before anything is written.
  - Adding a key that already exists fails and keeps the first row; removing a key deletes exactly
    one row, or fails.

## Maintainer Decision

Plan approved on 2026-11-22, with the row mapping extracted too: Q4 left that choice to this plan,
and the maintainer chose to extract both the expiry parse and the row mapping.

## Production Change (Q4)

One refactor, before any test, in its own commit. `databases/driver/mod.rs` gets two `pub(super)`
functions and the six private copies go:

- `parse_valid_until(value: i64, driver: &Driver) -> Result<DurationSinceUnixEpoch, Error>`: the copy
  that is byte-identical in the three drivers, with the driver as an argument.
- `peer_key_from_row(key_value: &str, valid_until: Option<i64>, driver: &Driver) -> Result<PeerKey,
  Error>`: the key parse (`MalformedDatabaseRecord` when it fails) and the expiry parse, which
  `load_keys` and `get_key_from_keys` repeat in each of the three drivers.

Each driver keeps reading the two columns with `try_get` (the row types differ) and passes the values
and `&DRIVER` (`Driver` is `Clone`, not `Copy`, so the functions borrow it and clone it into the
error). Same errors, same messages, same driver in the error: no behavior change. The
SQLite call sites are protected by the store tests below; the MySQL and PostgreSQL call sites by the
compiler, the shared functions' tests and the `db-compatibility` workflow, which stores and loads
keys with and without an expiry.

## Test Design

All store tests build the driver over a `TempDir` SQLite database created with
`ephemeral_configuration()` (kept by the test), with the tables created by `create_database_tables()`
and malformed rows written with raw `sqlx` through the driver's pool.

| Test | Cases | Assert |
| --- | --- | --- |
| `parse_valid_until`, in `databases/driver/mod.rs` (`rstest`) | `0`, `1`, `i64::MAX` seconds; `-1` and `i64::MIN` | `Ok(from_secs(n))`; the negative values give `MalformedDatabaseRecord` carrying the given driver and the value in the message |
| `parse_valid_until`, property (`quickcheck`, Q5) | any `i64` | non-negative: `Ok` with `as_secs() == value`; negative: `Err` |
| `peer_key_from_row`, in `databases/driver/mod.rs` (`rstest`) | a valid key with no expiry; a valid key with an expiry; a short text; a text of 32 characters with a symbol; a valid key with `-1` | the first two give the key with the expected `valid_until`; the next two `MalformedDatabaseRecord` carrying the driver they were given; the last `MalformedDatabaseRecord` carrying it too |
| `load_keys` reading a bad row (`rstest`) | `malformed_key` (a short text), `negative_valid_until` (`-1`), `non_integer_valid_until` (`'soon'`), `key_that_is_not_text` (a blob), `keys_table_missing` (`DROP TABLE keys`) | the first two give `MalformedDatabaseRecord`, the others `InvalidQuery` |
| `get_key_from_keys` reading a bad row (`rstest`) | `negative_valid_until`, `non_integer_valid_until`, `keys_table_missing` | the same variants: the error of the shared mapping and of the query is returned, not swallowed |
| `add_key_to_keys` with an expiry beyond `i64` | `valid_until = u64::MAX` seconds | `MalformedDatabaseRecord`; no row stored |
| `add_key_to_keys` with an existing key | the same key twice | the second call is `InvalidQuery` (unique constraint); the first row stays |
| `remove_key_from_keys` on a missing table | `DROP TABLE keys` | `InvalidQuery` |

**Not tested, no decision:**

- 89-91, `usize::try_from` of the affected-row count: it never exceeds one row on a 64-bit target.
- A stored key that does not parse, or is not text, is not reachable through `get_key_from_keys`:
  the lookup binds a `Key` built by `Key::new`, which always prints as a valid key, and a stored
  value of another type never equals that text (line 37 at the S2 head, the `key` column decode, is the same case). The shared `peer_key_from_row`
  test covers the branch directly, from text, without any `Key` built by `Deserialize` (that path
  skips the validation, finding F3, still to confirm, and no test pins it).

## Named Mutations

| ID | Mutation | Expected killer |
| --- | --- | --- |
| M1 | `parse_valid_until` accepts a negative value (`unsigned_abs`) | the negative cases and the property |
| M2 | `parse_valid_until` reports `Driver::Sqlite3` whatever the caller passes | the shared function's driver check (a case with another driver) |
| M3 | `peer_key_from_row` ignores a rejected expiry (`.transpose().ok().flatten()`) | the `-1` case of `peer_key_from_row` and `load_keys` `negative_valid_until` |
| M3b | `peer_key_from_row` accepts a key that does not parse (replaced by a fixed valid key) | the two bad-key cases of `peer_key_from_row` and `load_keys` `malformed_key` |
| M4 | `load_keys` ignores an expiry column it cannot decode (`try_get(..).unwrap_or(None)`) | `load_keys` `non_integer_valid_until` |
| M4b | `get_key_from_keys` swallows a mapping error (an `or_else` that returns a fixed key) | `get_key_from_keys` `negative_valid_until` |
| M4c | `load_keys` ignores a key column it cannot decode (`try_get(..).unwrap_or_default()`) | `load_keys` `key_that_is_not_text` |
| M5 | `add_key_to_keys` clamps an expiry beyond `i64` (`unwrap_or(i64::MAX)`) | the beyond-`i64` test |
| M6 | `add_key_to_keys` ignores a failed insert (`unwrap_or(1)` on the affected rows) | the existing-key test |
| M7 | `remove_key_from_keys` ignores a failed delete | the missing-table removal test |

## Property-Test Decision

Per Q5, one property next to the examples, on the shared function: it has a numeric domain with a
sign rule. `quickcheck = { version = "1", default-features = false }` and `quickcheck_macros = "1"`
become dev-dependencies of the package, as in `primitives` and `udp-core`; both are in the lock file.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| A1 | DONE | The refactor commit: the shared `parse_valid_until` and `peer_key_from_row`. | Compiles, the suite passes, no copy is left in the three drivers. |
| A2 | DONE | The shared functions' tests and the two dev-dependencies. | They pass; the negative arm and the key parse run in the shared functions. |
| A3 | DONE | The store tests. | They pass; the failure lines of the four queries run, except the recorded exception. |
| A4 | DONE | Run M1-M7. | Each is killed by its expected test. |
| A5 | DONE | Record the results and ask for the completed-file review. | This record, the ledger and F2. |

## Results

- **Coverage** (stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, `--all-features --lib`, clean coverage
  target directory, 2026-11-22): `sqlite/auth_key_store.rs` 29 / 55 lines to 97 / 102, regions
  29 / 71 to 118 / 126, functions 8 / 21 to 26 / 28 (the counts include the 21 new test helpers and
  cases of the file's test module). `databases/driver/mod.rs` 186 / 189 to 214 / 219 with the two
  shared functions and their tests. The MySQL and PostgreSQL key stores shrink from 55 to 35
  measured lines each. The package's unit-only lines 4,651 / 5,526 (84.17%) to 4,747 / 5,563
  (85.33%), regions 5,116 / 6,108 (83.76%), functions 796 / 1,021 (77.96%). What stays uncovered in
  the key store (line numbers at the S2 head, after the rebase on `develop` `f73a359f8`): line 37
  (the `key` column decode of `get_key_from_keys`, unreachable as above), 68-71 (the `usize`
  conversion, unreachable on 64-bit) and the `unreachable!` arm of the shared fixture
  (`sqlite/mod.rs`, line 99); in `driver/mod.rs` the existing test-only retry helper (row 16) and two test arms.
- **Tests:** 22 added, none changed: 11 for the shared functions (5 cases of `parse_valid_until`,
  one property, 5 cases of `peer_key_from_row`) and 11 for the store (cases of the `load_keys` and
  `get_key_from_keys` reads, the refused expiry, the duplicate key and the failed delete); 213 unit
  tests in the package, from 191.
- **Container suites:** the refactor was run against local `mysql:8.0` and `postgres:16`
  (`TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST` and `..._POSTGRES_...` set,
  `--lib databases::driver`): 7 tests pass in 9.4 s, so the two server drivers' key stores work
  through the shared functions.
- **Mutations** (each applied alone and reverted, the files compared with their copies afterwards;
  a mutant that left a parameter unused was rewritten with `let _ = driver;`):

  | ID | Result |
  | --- | --- |
  | M1 | Killed by the negative cases of both shared functions, the property and the `load_keys` and `get_key_from_keys` negative-expiry cases |
  | M2 | Killed by the three shared cases that pass a driver other than SQLite |
  | M3 | Killed by the negative-expiry cases of `peer_key_from_row`, `load_keys` and `get_key_from_keys` |
  | M3b | Killed by the two bad-key cases of `peer_key_from_row` and by `load_keys` `a_key_that_does_not_parse` |
  | M4 | Killed by `load_keys` `a_valid_until_that_is_not_an_integer` |
  | M4b | Killed by `get_key_from_keys` `a_negative_valid_until` |
  | M4c | Killed by `load_keys` `a_key_that_is_not_text` |
  | M5 | Killed by the expiry-beyond-`i64` test |
  | M6 | Killed by the existing-key test |
  | M7 | Killed by the failed-delete test |

- **Review outcome:** the maintainer approved the completed file on 2026-11-22 (the two test commits and these results).
- **Validation:** `cargo clippy -p torrust-tracker-core --all-targets --all-features` (only the
  existing `axum::serve` warning), `cargo +nightly fmt --all -- --check` and
  `cargo machete --with-metadata` pass. `Cargo.lock` gains only the two dev-dependency edges of
  `quickcheck` and `quickcheck_macros`, packages it already holds.
