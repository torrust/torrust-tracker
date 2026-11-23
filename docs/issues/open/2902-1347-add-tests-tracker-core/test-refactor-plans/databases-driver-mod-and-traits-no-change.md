---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/databases/driver/mod.rs
status: done
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/databases/driver/mod.rs
    - packages/tracker-core/src/databases/mod.rs
    - packages/tracker-core/src/databases/traits/mod.rs
    - packages/tracker-core/src/databases/traits/auth_keys.rs
    - packages/tracker-core/src/databases/traits/database.rs
    - packages/tracker-core/src/databases/traits/schema.rs
    - packages/tracker-core/src/databases/traits/torrent_metrics.rs
    - packages/tracker-core/src/databases/traits/whitelist.rs
---

# Database Driver Module and Traits No-Change Record

Follow the shared [guidance](README.md). This record covers ledger rows 16 and 17 together, as the
spec allows for no-change files: `databases/driver/mod.rs` (row 16) and `databases/mod.rs` with the
six files of `databases/traits/` (row 17). Decision for both: **no change**.

## Row 16: `databases/driver/mod.rs`

- **Unit-only coverage:** 186 / 189 lines at the S2 baseline; 214 / 219 now (stable Rust 1.100.0,
  `cargo-llvm-cov 0.9.1`, `--all-features --lib`, 2026-11-23). The file holds the shared constants,
  the two functions that [row 8](sqlite-auth-key-store-tests.md) extracted and tested
  (`parse_valid_until`, `peer_key_from_row`), and the test harness every driver runs
  (`run_tests`).
- **Uncovered lines, challenged** (line numbers at the S2 head, after the rebase on `develop`
  `f73a359f8`; lines 105-106 and 108 of the loop never run):

  | Lines | What they are | Decision |
  | --- | --- | --- |
  | 102-108 | the retry loop of the test-only `create_database_tables` helper: sleeps 3 seconds and tries again, up to 20 times, then returns "Database is not ready after retries." | No change. The loop waits for a MySQL or PostgreSQL container that is still starting; SQLite is ready at once, so the loop never repeats in a unit test. A test of it would sleep for up to a minute to check a helper of the tests themselves, and the Docker suites exercise the wait |
  | 409, 444 | the `panic!` arm and the `Err(_)` arm of the helpers of the row 8 tests | test code |

## Row 17: `databases/mod.rs` and `databases/traits/*`

- **Unit-only coverage:** `databases/mod.rs`, `traits/mod.rs`, `traits/auth_keys.rs`,
  `traits/database.rs`, `traits/schema.rs` and `traits/torrent_metrics.rs` have no executable line;
  `traits/whitelist.rs` has 2 / 2. At the baseline: 2 / 2, no entries; unchanged.
- **What the files are:** module declarations, the persistence traits and their documentation, and
  the `mockall` mocks (`MockTorrentMetricsStore`, `MockAuthKeyStore`, `MockSchemaMigrator`,
  `MockWhitelistStore`) that other tests of the package use.
- **Decision:** no change. There is no logic to test: a trait declaration has no body, and the mocks
  are generated code whose behavior the tests that use them (the handlers, managers and repositories
  of S4 and S5) already exercise, and would fail on if it broke. The one executable statement of
  `traits/whitelist.rs` runs.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| N1 | DONE | The maintainer approves both no-change decisions. | Approved on 2026-11-23; this record and the ledger. |

## Results

- **Decision:** the maintainer approved no change for rows 16 and 17 on 2026-11-23, with the reasons above.
- **Tests:** none added or changed.
