---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/authentication/handler.rs
status: done
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/authentication/handler.rs
    - packages/tracker-core/src/authentication/mod.rs
    - packages/tracker-core/src/authentication/key/repository/in_memory.rs
    - packages/tracker-core/src/authentication/key/repository/persisted.rs
---

# Authentication Keys Handler File Test Plan

Follow the shared [guidance](README.md). This plan covers ledger row 28,
`packages/tracker-core/src/authentication/handler.rs`: `KeysHandler`, which adds, removes and loads
authentication keys through the database repository and the in-memory repository.

## Current State

- **Unit-only coverage** (`--all-features --lib`, stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, S4 base
  `develop` at `41f294c9d`, measured 2026-11-25): 373 / 373 lines, 368 / 370
  regions, 63 / 63 functions. The two uncovered regions are the error arms of the `?` in
  `remove_peer_key` (line 243) and in `load_peer_keys_from_database` (line 271).
- **Existing tests:** 17 in the file: the four add paths (random or pre-generated, expiring or
  permanent), each with a failing-database case for the add paths, the invalid key, the duration
  overflow, and two tests that the error texts carry no source path. `authentication/mod.rs` has one
  happy-path test of `remove_peer_key` (the key no longer authenticates) and one of
  `load_peer_keys_from_database` (a key removed from memory comes back).
- **Module-owned decisions** (read from the code):
  - Every write goes to the database **first**, then to memory: a database failure leaves memory
    untouched (`add`, `remove`).
  - `load_peer_keys_from_database` **replaces** the in-memory keys with what the database holds
    (`reset_with`), and does not touch memory when the load fails.
  - `add_peer_key` maps a database failure to `PeerKeyError::DatabaseError`; `remove_peer_key` and
    `load_peer_keys_from_database` return the raw `databases::error::Error`.

The existing tests do not pin the two failure orderings, nor the replace semantics: the happy-path
load test would also pass if load only added keys.

## Test Design

Three tests in the handler's `tests` module, with a handler built from a mock `AuthKeyStore` and an
in-memory repository the test keeps (the field is private to the handler, but its tests module is a
descendant).

| Test | Arrange | Act | Assert |
| --- | --- | --- | --- |
| `it_should_keep_the_key_in_memory_when_removing_it_from_the_database_fails` | the in-memory repository holds a key; the mock `remove_key_from_keys` fails with `DeleteFailed` (once) | `remove_peer_key` | the error is `DeleteFailed` for the mock's driver; the key is still in memory |
| `it_should_keep_the_in_memory_keys_when_loading_from_the_database_fails` | the in-memory repository holds a key; the mock `load_keys` fails with a connection error (once) | `load_peer_keys_from_database` | the error is returned; the key is still in memory |
| `it_should_replace_the_in_memory_keys_with_the_keys_in_the_database` | memory holds a stale key the database does not; the mock `load_keys` returns another key | `load_peer_keys_from_database` | `Ok`; the stale key is gone and the database key is present |

## Property-Test Decision

No `quickcheck`: three fixed scenarios over one or two keys; the key validity rules belong to row 29.

## Named Mutations

| ID | Mutation | Expected killer |
| --- | --- | --- |
| M1 | `remove_peer_key` removes from memory before the database | the failing-remove test |
| M2 | `remove_peer_key` ignores the database error (the result is dropped with `drop(..)`, because `let _ =` on a `Result` is denied) | the failing-remove test |
| M3 | `load_peer_keys_from_database` resets memory (empty) before loading, then returns the error | the failing-load test |
| M4 | `load_peer_keys_from_database` ignores the database error and resets with an empty list | the failing-load test |
| M5 | `reset_with` replaced by inserting each loaded key (additive load) | the replace test |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| H1 | DONE | The three tests. | They pass; the two error arms run. |
| H2 | DONE | Run M1-M5. | Each is killed by its expected test. |
| H3 | DONE | Record the results and ask for the completed-file review. | This record and the ledger. |

## Maintainer Decision

Plan approved on 2026-11-25.

## Prose-First AAA Review

Temporary prose, written from this plan's Test Design, then compared with the code.

| Test | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_keep_the_key_in_memory_when_removing_it_from_the_database_fails` | A handler whose database store fails the removal once, and a key already in memory. | Remove the key through the handler. | The database error is returned with its driver, and the key is still in memory. | Matches the code. |
| `it_should_keep_the_in_memory_keys_when_loading_from_the_database_fails` | A handler whose store fails the load once, and a key in memory. | Load the keys from the database. | The error is returned and the key is still in memory. | Matches the code. |
| `it_should_replace_the_in_memory_keys_with_the_keys_in_the_database` | A store that returns one key, and a different key in memory. | Load the keys from the database. | The load succeeds, the stale key is gone and the stored key is present. | Matches the code. The Assert compares both lookups as one tuple. |

Smells found: none. Each test has AAA comments, one Act and a message on each assertion.

## Results

- **Coverage** (stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, `--all-features --lib`, clean coverage target
  directory, 2026-11-25): the file 373 / 373 lines to 466 / 466, regions 368 / 370 to 457 / 457,
  functions 63 / 63 to 76 / 76 (the counts include the test module). Both uncovered regions, the `?`
  error arms of lines 243 and 271, now run. The package's unit-only lines 5,186 / 5,935 (87.38%) to
  5,279 / 6,028 (87.57%), regions 5,815 / 6,708 (86.69%), functions 922 / 1,114 (82.76%).
- **Tests:** 3 added: 287 to 290 unit tests in the package.
- **Mutations** (each applied alone and reverted, the files compared with their copies afterwards):

  | ID | Result |
  | --- | --- |
  | M1 | Killed by the failing-remove test |
  | M2 | Killed by the failing-remove test |
  | M3 | Killed by the failing-load test |
  | M4 | Killed by the failing-load test |
  | M5 | Killed by the replace test alone: no other test of the package fails when `reset_with` stops clearing |

- **Review outcome:** pending.
- **Validation:** pending.
