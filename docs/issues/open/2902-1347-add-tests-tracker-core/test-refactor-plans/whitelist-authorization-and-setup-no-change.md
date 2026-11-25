---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/whitelist/authorization.rs
status: done
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/whitelist/authorization.rs
    - packages/tracker-core/src/whitelist/setup.rs
    - packages/tracker-core/src/whitelist/mod.rs
    - packages/tracker-core/src/whitelist/test_helpers.rs
    - packages/tracker-core/src/whitelist/repository/mod.rs
---

# Whitelist Authorization and Module Files No-Change Record

Follow the shared [guidance](README.md). This record covers ledger rows 37 and 39 together, as the spec allows for
no-change files: `whitelist/authorization.rs` (row 37) and `whitelist/{mod,setup,test_helpers}.rs` with
`whitelist/repository/mod.rs` (row 39). Decision: **no change**, after a mutation check. Row 38
(`repository/in_memory.rs`) is not in this record: its check found a gap, see [its plan](whitelist-in-memory-tests.md).

## Current State

- **Unit-only coverage** (`--all-features --lib`, stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, S4 base `develop`
  at `cad54cb9b`, measured 2026-11-25): `authorization.rs` 83 / 83 lines, 85 / 85 regions; `setup.rs` 7 / 7;
  `test_helpers.rs` 11 / 11; `mod.rs` 20 / 20 (declarations); `repository/mod.rs` has no executable line.
- **What the files are:** `authorization.rs`: an unlisted tracker authorizes every hash, a listed one only the
  hashes in the in-memory whitelist; `setup.rs`: wires the manager to the store and the shared in-memory
  whitelist; `test_helpers.rs`: test support (configurations and services); the others declarations.

## Mutation check

Each logic line was mutated alone and the whole unit suite run; the file was restored and compared with its copy.

| ID | File | Mutation | Result |
| --- | --- | --- | --- |
| A1 | `authorization.rs` | the unlisted check is inverted | Killed by 16 tests |
| A2 | `authorization.rs` | the whitelist membership check is inverted | Killed by 5 tests |
| A3 | `authorization.rs` | `is_listed` is inverted | Killed by 16 tests |
| T1 | `setup.rs` | the manager gets a new in-memory whitelist instead of the shared one | Killed by the authorization test that adds through the manager |

## Decision

- **No change**: every mutant is killed. `setup.rs` is covered through the shared test helpers; `test_helpers.rs`
  is test support, whose behavior every whitelist test exercises; `mod.rs` and `repository/mod.rs` are declarations.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| N1 | DONE | Run the four mutants. | The table above. |
| N2 | DONE | The maintainer approves the no-change decisions. | Approved on 2026-11-25; this record and the ledger. |

## Results

- **Decision:** the maintainer approved no change for rows 37 and 39 on 2026-11-25, with the reasons above.
- **Tests:** none added or changed.
