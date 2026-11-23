---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/databases/driver/sqlite/whitelist_store.rs
status: done
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/databases/driver/sqlite/whitelist_store.rs
    - packages/tracker-core/src/databases/driver/sqlite/mod.rs
    - packages/tracker-core/src/databases/traits/whitelist.rs
---

# SQLite Whitelist Store File Test Plan

Follow the shared [guidance](README.md). This plan covers ledger row 10,
`packages/tracker-core/src/databases/driver/sqlite/whitelist_store.rs`.

## Current State

- **Unit-only coverage:** 17 / 31 lines (54.84%), 18 / 42 regions, 7 / 15 functions, at the S2
  baseline (`develop` at `83f3ecbbc`, stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`,
  `--all-features --lib`, 2026-11-22). Uncovered lines:
  - 18, 36 and 75: the query or execute failure of `load_whitelist`, `get_info_hash_from_whitelist`
    and `remove_info_hash_from_whitelist`;
  - 22 and 24-26: a stored info-hash that cannot be decoded, or does not parse, in `load_whitelist`;
  - 40 and 42-44: the same two failures in `get_info_hash_from_whitelist`;
  - 64-66: the `usize` conversion of the affected-row count.
- **Existing tests:** none in the file. The shared driver suite (`run_sqlite_driver_tests`) loads the
  whitelist, adds and gets hashes, removes one and fails to add the same one twice: the happy path
  and the duplicate.
- **Module-owned decisions:** a stored info-hash that does not parse is `MalformedDatabaseRecord`; a
  value that is not text, or a failed query, is the mapped `sqlx` error (`InvalidQuery`).

## Maintainer Decision

Plan approved on 2026-11-23.

## Test Design

All tests use the shared `migrated_store()` and `execute()` of `sqlite/mod.rs` (row 9) and keep the
returned configuration alive. A bad database is a stored row built by raw SQL
(`Broken::Row { info_hash_sql }`) or a dropped `whitelist` table (`Broken::WhitelistTableMissing`).

| Test | Cases | Assert |
| --- | --- | --- |
| `load_whitelist` reading a bad row (`rstest`) | `an_info_hash_that_does_not_parse` (`'nope'`) | `MalformedDatabaseRecord` |
| `load_whitelist` reading an unreadable row or failing (`rstest`) | `an_info_hash_that_is_not_text` (`X'00'`), `a_missing_whitelist_table` | `InvalidQuery` |
| `get_info_hash_from_whitelist` on a missing table | `DROP TABLE whitelist` | `InvalidQuery` |
| `remove_info_hash_from_whitelist` on a missing table | `DROP TABLE whitelist` | `InvalidQuery` |

**Not tested, no decision:**

- 40 and 42-44, the decode and parse of the row `get_info_hash_from_whitelist` finds: the lookup
  binds a valid info-hash's text, so a row that matches it always parses and is text (a blob or a
  different text never equals it).
- 64-66, `usize::try_from` of the affected-row count: it never exceeds one row on a 64-bit target.

## Property-Test Decision

No `quickcheck`: the inputs are SQL rows and one missing table.

## Named Mutations

| ID | Mutation | Expected killer |
| --- | --- | --- |
| M1 | `load_whitelist` skips a row it cannot read (`filter_map(.. .ok())`) | the malformed and the unreadable `load_whitelist` cases |
| M2 | `load_whitelist` accepts an info-hash that does not parse (a fixed hash) | the malformed `load_whitelist` case |
| M3 | `get_info_hash_from_whitelist` ignores a failed query (`unwrap_or(None)`) | the missing-table `get` test |
| M4 | `remove_info_hash_from_whitelist` ignores a failed statement (`unwrap_or(1)`) | the missing-table `remove` test |
| M5 | `add_info_hash_to_whitelist` ignores a failed statement (`unwrap_or(1)`) | the shared suite's duplicate test (`run_sqlite_driver_tests`) |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| W1 | DONE | The four tests. | They pass; the failure lines run except the recorded exceptions. |
| W2 | DONE | Run M1-M5. | Each is killed by its expected test. |
| W3 | DONE | Record the results and ask for the completed-file review. | This record and the ledger. |

## Results

- **Coverage** (stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, `--all-features --lib`, clean coverage
  target directory, 2026-11-23): the file 17 / 31 lines to 47 / 54, regions 18 / 42 to 66 / 77,
  functions 7 / 15 to 18 / 21 (the counts include the test module). The package's unit-only lines
  4,823 / 5,620 (85.82%) to 4,853 / 5,643 (86.00%), regions 5,286 / 6,233 (84.81%), functions
  833 / 1,041 (80.02%). Left uncovered, as planned: 40 and 42-44 (the decode and parse of the row
  `get_info_hash_from_whitelist` finds) and 64-66 (the `usize` conversion).
- **Tests:** 5 added (cases included), none changed: 234 to 239 unit tests in the package.
- **Mutations** (each applied alone and reverted, the file compared with its copy afterwards):

  | ID | Result |
  | --- | --- |
  | M1 | Killed by the malformed and the not-text `load_whitelist` cases |
  | M2 | Killed by the malformed `load_whitelist` case |
  | M3 | Killed by the failed `get` test |
  | M4 | Killed by the failed `remove` test |
  | M5 | Killed by the shared suite, `run_sqlite_driver_tests` (its duplicate-add test) |

- **Review outcome:** the maintainer approved the completed file on 2026-11-23.
- **Validation:** `cargo clippy -p torrust-tracker-core --all-targets --all-features` (only the
  existing `axum::serve` warning), `cargo +nightly fmt --all -- --check` and
  `cargo machete --with-metadata` pass.
