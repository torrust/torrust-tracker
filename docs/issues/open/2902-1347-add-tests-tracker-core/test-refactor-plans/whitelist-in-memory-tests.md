---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/whitelist/repository/in_memory.rs
status: done
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/whitelist/repository/in_memory.rs
    - packages/tracker-core/src/whitelist/manager.rs
---

# In-Memory Whitelist File Test Plan

Follow the shared [guidance](README.md). This plan covers ledger row 38,
`packages/tracker-core/src/whitelist/repository/in_memory.rs` (`InMemoryWhitelist`). The ledger said "no change";
the mutation check of the slice found otherwise.

## Current State

- **Unit-only coverage** (`--all-features --lib`, stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, S4 base `develop`
  at `cad54cb9b`, measured 2026-11-25): 51 / 51 lines, 72 / 72 regions.
- **Existing tests:** four: add then contains, remove, contains, clear. None reads the **return value** of `add`
  or `remove`.
- **Module-owned decisions** (documented on the methods): `add` returns `true` if the torrent was newly added and
  `false` if it was already there; `remove` returns `true` if the torrent was present and `false` if it was
  not found. No production code reads either value today (the manager ignores them; `load` binds the result to
  `let _: bool`), but both are public API of a published crate (`add` is `pub`).

## Mutation check that found the gap (2026-11-25)

| Mutation | Result |
| --- | --- |
| `add` returns `true` whatever it did (B1) | **Survived** every test of the package |
| `remove` returns `true` whatever it did (B2) | **Survived** every test of the package |
| `contains` always true or always false (B3, B4) | Killed (8 and 9 tests) |
| `clear` clears nothing (B5) | Killed by 2 tests |
| `add` stores nothing and returns `true` (B6) | Killed |

## Maintainer Decision

Plan approved on 2026-11-25.

## Test Design

One `rstest` table in the file's `tests` module, over the two return values:

| Case | Arrange | Act | Assert |
| --- | --- | --- | --- |
| add of a new hash | an empty whitelist | `add` | `true` |
| add of a hash already there | the hash is in the whitelist | `add` | `false`; the hash is still there |
| remove of a present hash | the hash is in the whitelist | `remove` | `true`; the hash is gone |
| remove of an absent hash | an empty whitelist | `remove` | `false` |

## Property-Test Decision

No `quickcheck`: a set with two operations, four cases.

## Named Mutations

| ID | Mutation | Expected killer |
| --- | --- | --- |
| B1 | `add` returns `true` whatever it did | the add-of-a-known-hash case |
| B2 | `remove` returns `true` whatever it did | the remove-of-an-absent-hash case |
| B7 | `add` returns `false` whatever it did | the add-of-a-new-hash case |
| B8 | `remove` returns `false` whatever it did | the remove-of-a-present-hash case |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| I1 | DONE | The table. | It passes. |
| I2 | DONE | Run B1, B2, B7 and B8. | Each killed by its case. |
| I3 | DONE | Record the results and ask for the completed-file review. | This record and the ledger. |

## Prose-First AAA Review

Temporary prose, written from this plan's Test Design, then compared with the code.

| Test | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| `adding_should_report_whether_the_hash_was_new` (cases) | An empty whitelist, with the hash added first in one case. | Add the hash. | The result is `true` for a new hash and `false` for a known one; the hash is whitelisted. | Matches the code. |
| `removing_should_report_whether_the_hash_was_present` (cases) | An empty whitelist, with the hash added first in one case. | Remove the hash. | The result is `true` for a present hash and `false` for an absent one; the hash is not whitelisted. | Matches the code. |

Smells found: none.

## Results

- **Coverage** (stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, `--all-features --lib`, clean coverage target
  directory, 2026-11-25): the file stays 51 / 51 lines and 72 / 72 regions: the new tests add no measured
  line, and line coverage never showed the gap. The package's unit-only lines 5,558 / 6,306 (88.14%), regions
  6,085 / 6,970 (87.30%), functions 978 / 1,170 (83.59%).
- **Tests:** 4 cases added: 330 to 334 unit tests.
- **Mutations** (each applied alone and reverted, the file compared with its copy afterwards): B1 and B2
  **survived every test of the package before this plan** and are now killed (B1 by the known-hash case, B2 by
  the absent-hash case); B7 and B8, the opposite constants, are killed by the new-hash and present-hash cases.
- **Review outcome:** the maintainer approved the completed file on 2026-11-25.
- **Validation:** `cargo clippy -p torrust-tracker-core --all-targets --all-features` (only the existing
  `axum::serve` warning), `cargo +nightly fmt --all -- --check` and `cargo machete --with-metadata` pass.
