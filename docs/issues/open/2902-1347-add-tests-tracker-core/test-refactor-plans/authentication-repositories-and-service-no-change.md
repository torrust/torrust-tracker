---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/authentication/service.rs
status: done
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/authentication/service.rs
    - packages/tracker-core/src/authentication/key/repository/persisted.rs
    - packages/tracker-core/src/authentication/key/repository/in_memory.rs
    - packages/tracker-core/src/authentication/mod.rs
    - packages/tracker-core/src/authentication/key/repository/mod.rs
---

# Authentication Repositories, Service and Module No-Change Record

Follow the shared [guidance](README.md). This record covers ledger rows 31 to 34 together, as the spec allows for
no-change files: `key/repository/persisted.rs` (row 31), `key/repository/in_memory.rs` (row 32),
`service.rs` (row 33), and `authentication/mod.rs` with `key/repository/mod.rs` (row 34). Decision for all of
them: **no change**, after a mutation check of each file's logic.

## Current State

- **Unit-only coverage** (`--all-features --lib`, stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, S4 base
  `develop` at `39d6d7e1b`, after rows 28 to 30, measured 2026-11-25): `persisted.rs` 65 / 65 lines, 86 / 86 regions;
  `in_memory.rs` 91 / 91, 119 / 119; `service.rs` 172 / 172, 154 / 154; `authentication/mod.rs` 118 / 118, 141 / 141;
  `key/repository/mod.rs` has no executable line. At the S4 baseline, before row 28, `persisted.rs` had two
  uncovered regions (the `?` arms of `remove` and `load_keys`); the row 28 tests, which reach them through the handler
  with a failing store, cover them.
- **What the files are:**
  - `persisted.rs`: a pass-through to the `AuthKeyStore`: three methods that return the store's result.
  - `in_memory.rs`: a map of keys behind a lock: insert, remove, get, clear, reset.
  - `service.rs`: the authentication decision: a public tracker accepts every key; a private one needs the key in
    memory and, unless the configuration disables the check, not expired.
  - `authentication/mod.rs`: the `private` tracker tests of the service and handler together (the file is
    declarations and tests); `key/repository/mod.rs`: two module declarations.

## Mutation check

Each logic line was mutated alone and the whole unit suite run; the file was restored and compared with its copy.
A mutant that did not compile was rewritten.

| ID | File | Mutation | Result |
| --- | --- | --- | --- |
| P1 | `persisted.rs` | `add` ignores the store's error | Killed by 3 tests (the failing-add tests of the handler) |
| P2 | `persisted.rs` | `remove` ignores the store's error | Killed by the row 28 failing-remove test **alone** |
| P3 | `persisted.rs` | `load_keys` returns an empty list on error | Killed by the row 28 failing-load test **alone** |
| I1 | `in_memory.rs` | `insert` stores nothing | Killed by 9 tests |
| I2 | `in_memory.rs` | `remove` removes nothing | Killed by 2 tests |
| I3 | `in_memory.rs` | `get` finds nothing | Killed by 12 tests |
| I4 | `in_memory.rs` | `clear` clears nothing | Killed by `clear_all_peer_keys` |
| I5 | `in_memory.rs` | `reset_with` does not clear first | Killed by the row 28 replace test **alone** |
| I6 | `in_memory.rs` | `reset_with` inserts none of the new keys (first draft did not compile) | Killed by 3 tests |
| S1 | `service.rs` | `authenticate` checks the key only on a public tracker | Killed by 7 tests |
| S2 | `service.rs` | `tracker_is_private` is inverted | Killed by 7 tests |
| S3 | `service.rs` | an unregistered key is accepted | Killed by 3 tests |
| S4 | `service.rs` | the expiration check flag is inverted | Killed by 3 tests |
| S5 | `service.rs` | with no private-mode setting, expiry is not checked | Killed by 2 tests |
| S6 | `service.rs` | with the check enabled, expiry is not checked | Killed by 1 test |

## Decision

- **No change** for all four rows: every mutant is killed. The three that only a row 28 test kills (P2, P3, I5)
  show that the files' own tests did not pin those behaviors before S4; the handler tests now do, and a test of the
  pass-through or of `reset_with` in their own files would only repeat them.
- `authentication/mod.rs` is declarations and the combined service-and-handler tests; `key/repository/mod.rs` has
  no logic. Nothing to test.
- **Review of breadth (row 33):** the service tests cover the public tracker, an unregistered key, a registered key,
  an expired key with the check on, off and by default; no combination of the two configuration flags is untested.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| N1 | DONE | Run the 15 mutants. | The table above. |
| N2 | DONE | The maintainer approves the no-change decisions. | Approved on 2026-11-25; this record and the ledger. |

## Results

- **Decision:** the maintainer approved no change for rows 31 to 34 on 2026-11-25, with the reasons above.
- **Tests:** none added or changed.
