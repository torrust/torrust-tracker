---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/whitelist/repository/persisted.rs
status: done
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/whitelist/repository/persisted.rs
    - packages/tracker-core/src/whitelist/manager.rs
    - packages/tracker-core/src/whitelist/repository/in_memory.rs
    - packages/tracker-core/src/databases/traits/whitelist.rs
---

# Persisted Whitelist and Whitelist Manager Test Plan

Follow the shared [guidance](README.md). This plan covers ledger rows 35 and 36 together:
`packages/tracker-core/src/whitelist/repository/persisted.rs` (`DatabaseWhitelist`) and
`packages/tracker-core/src/whitelist/manager.rs` (`WhitelistManager`). They are one unit: the manager writes the
database first and memory second. It also settles finding F4.

## Current State

- **Unit-only coverage** (`--all-features --lib`, stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, S4 base
  `develop` at `39d6d7e1b`, measured 2026-11-25): `persisted.rs` 78 / 78 lines, 112 / 116 regions; `manager.rs` 95 / 95
  lines, 95 / 98 regions. The uncovered regions are the `?` error arms: four in `persisted.rs` (the check and the
  write of `add` and of `remove`) and three in `manager.rs` (one per method).
- **Existing tests:** `persisted.rs` 5 on a real SQLite database (add, remove, load, no duplicate, remove of an
  absent hash); `manager.rs` 3 (add, remove, load), all on a real database. None makes the store fail.
- **Module-owned decisions** (read from the code):
  - `DatabaseWhitelist::add` and `remove` are idempotent: they check first and write only if needed; a store
    failure of the check or the write is returned.
  - `load_from_database` returns the store's list or its error.
  - `WhitelistManager` writes the database **first**, then memory: a database failure leaves memory untouched;
    `load_whitelist_from_database` **replaces** the in-memory list, and leaves it untouched when the load fails.

## F4: the check-then-write of `add` and `remove`

Checked on 2026-11-25 with a throwaway test (SQLite on a temporary database, a multi-thread runtime, 200 rounds
of two concurrent `add` calls of the same info-hash, not committed): in **199 of 200 rounds** one call fails with
`UNIQUE constraint failed: whitelist.info_hash` (`InvalidQuery`) while the other returns `Ok`. So the idempotence of
`add` holds sequentially and breaks under concurrency; the effect is that one of two identical concurrent whitelist
requests reports a failure although the hash ends up whitelisted. It is a bug (low severity), not a documentation
gap. It is handled by its own issue and session (proposed to the maintainer); this plan adds **no test that pins the
racing behavior**, and a regression test follows once the fix merges.

## Maintainer Decision

Plan approved on 2026-11-25. F4: confirmed as a bug, handled by its own issue and session; no test pins the racing behavior here.

## Test Design

Tests join each file's `tests` module, with a `MockWhitelistStore` (the `mockall` mock of the store trait, as the
other mock-based tests use) so each failure is injected at one call, with `.times(1)` or `.never()` on the others.

**`persisted.rs`:**

| Test | Arrange | Act | Assert |
| --- | --- | --- | --- |
| a failed check stops `add` before any write | the store's check fails; the write is `.never()` | `add` | the error is returned |
| a failed write is returned by `add` | the check says no; the write fails | `add` | the error is returned |
| a failed check stops `remove` before any delete | the check fails; the delete is `.never()` | `remove` | the error is returned |
| a failed delete is returned by `remove` | the check says yes; the delete fails | `remove` | the error is returned |
| a failed load is returned | `load_whitelist` fails | `load_from_database` | the error is returned |

**`manager.rs`** (a manager over a `DatabaseWhitelist` on the mock and an `InMemoryWhitelist` the test keeps):

| Test | Arrange | Act | Assert |
| --- | --- | --- | --- |
| a failed persistence leaves memory without the hash on `add` | the store fails the write | `add_torrent_to_whitelist` | the error is returned; memory does not contain the hash |
| a failed persistence keeps the hash in memory on `remove` | memory holds the hash; the store fails the delete | `remove_torrent_from_whitelist` | the error is returned; memory still contains the hash |
| a failed load keeps the in-memory list | memory holds a hash; the store fails the load | `load_whitelist_from_database` | the error is returned; memory still contains the hash |
| a load replaces the in-memory list | memory holds a stale hash; the store returns another | `load_whitelist_from_database` | `Ok`; the stale hash is gone and the stored one is present |

## Property-Test Decision

No `quickcheck`: error injection at one call, and fixed hashes.

## Named Mutations

| ID | File | Mutation | Expected killer |
| --- | --- | --- | --- |
| Q1 | `persisted.rs` | `add` treats a failed check as "not whitelisted" | the failed-check `add` test |
| Q2 | `persisted.rs` | `add` ignores a failed write | the failed-write `add` test |
| Q3 | `persisted.rs` | `remove` treats a failed check as "whitelisted" | the failed-check `remove` test |
| Q4 | `persisted.rs` | `remove` ignores a failed delete | the failed-delete `remove` test |
| Q5 | `persisted.rs` | `load_from_database` returns an empty list on error | the failed-load test |
| R1 | `manager.rs` | `add` writes memory before the database | the failed `add` test |
| R2 | `manager.rs` | `add` ignores a failed persistence | the failed `add` test |
| R3 | `manager.rs` | `remove` removes from memory before the database | the failed `remove` test |
| R4 | `manager.rs` | `remove` ignores a failed persistence | the failed `remove` test |
| R5 | `manager.rs` | `load` clears memory before loading | the failed-load manager test |
| R6 | `manager.rs` | `load` does not clear memory first | the replace test |
| R7 | `manager.rs` | `load` ignores a failed load and clears memory | the failed-load manager test |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| W1 | DONE | The nine tests. | They pass; the seven error arms run. |
| W2 | DONE | Run Q1-Q5 and R1-R7. | Each killed by its expected test. |
| W3 | DONE | Record the results, F4 and the ledger; ask for the completed-file review. | This record and the ledger. |

## Prose-First AAA Review

Temporary prose, written from this plan's Test Design, then compared with the code.

| Test | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_not_write_when_the_check_before_adding_fails` | A store whose check fails and whose write must never run. | Add a hash. | The check's error is returned. | Matches the code. |
| `it_should_return_the_error_when_the_write_of_an_add_fails` | A store whose check says "not whitelisted" and whose write fails. | Add a hash. | The write's error is returned. | Matches the code. |
| `it_should_not_delete_when_the_check_before_removing_fails` | A store whose check fails and whose delete must never run. | Remove a hash. | The check's error is returned. | Matches the code. |
| `it_should_return_the_error_when_the_delete_of_a_remove_fails` | A store whose check says "whitelisted" and whose delete fails. | Remove a hash. | The delete's error is returned. | Matches the code. |
| `it_should_return_the_error_when_loading_the_whitelist_fails` | A store whose load fails. | Load from the database. | The load's error is returned. | Matches the code. |
| `it_should_not_add_the_torrent_to_memory_when_persisting_it_fails` | A manager over a store whose write fails, and an empty memory. | Add a torrent. | The error is returned and memory has no hash. | Matches the code. |
| `it_should_keep_the_torrent_in_memory_when_removing_it_from_the_database_fails` | A store whose delete fails, and a hash in memory. | Remove the torrent. | The error is returned and the hash is still in memory. | Matches the code. |
| `it_should_keep_the_in_memory_whitelist_when_loading_it_from_the_database_fails` | A store whose load fails, and a hash in memory. | Load the whitelist. | The error is returned and the hash is still in memory. | Matches the code. |
| `it_should_replace_the_in_memory_whitelist_with_the_one_in_the_database` | A store that returns one hash, and another hash in memory. | Load the whitelist. | The load succeeds, the stale hash is gone and the stored one is present. | Matches the code. |

Smells found: none. The "never" expectations on the second call of each pair make an extra store call fail the test.

## Results

- **Coverage** (stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, `--all-features --lib`, clean coverage target
  directory, 2026-11-25): `persisted.rs` 78 / 78 lines to 193 / 193, regions 112 / 116 to 220 / 220;
  `manager.rs` 95 / 95 to 212 / 212, regions 95 / 98 to 211 / 211 (the counts include the test modules). All
  seven error arms run. The package's unit-only lines 5,557 / 6,306 (88.12%), regions 6,084 / 6,970 (87.29%),
  functions 978 / 1,170 (83.59%).
- **Tests:** 9 added (5 in `persisted.rs`, 4 in `manager.rs`): 321 to 330 unit tests.
- **Mutations** (each applied alone and reverted, the files compared with their copies afterwards):

  | ID | Result |
  | --- | --- |
  | Q1 | Killed by the failed-check `add` test |
  | Q2 | Killed by the failed-write `add` test and the manager's failed `add` test |
  | Q3 | Killed by the failed-check `remove` test |
  | Q4 | Killed by the failed-delete `remove` test and the manager's failed `remove` test |
  | Q5 | Killed by the failed-load tests of both files |
  | R1 | Killed by the manager's failed `add` test |
  | R2 | Killed by the manager's failed `add` test |
  | R3 | Killed by the manager's failed `remove` test |
  | R4 | Killed by the manager's failed `remove` test |
  | R5 | Killed by the manager's failed-load test |
  | R6 | Killed by the replace test (the first draft, removing the `clear`, left it unused and did not compile; rewritten to clear only when the new list is empty) |
  | R7 | Killed by the manager's failed-load test |

- **F4:** confirmed as a bug on 2026-11-25 (199 of 200 rounds of two concurrent adds of one hash fail with a unique-constraint error); recorded in the spec and handed to its own issue.
- **Review outcome:** the maintainer approved the completed files on 2026-11-25.
- **Validation:** `cargo clippy -p torrust-tracker-core --all-targets --all-features` (only the existing
  `axum::serve` warning), `cargo +nightly fmt --all -- --check` and `cargo machete --with-metadata` pass.
