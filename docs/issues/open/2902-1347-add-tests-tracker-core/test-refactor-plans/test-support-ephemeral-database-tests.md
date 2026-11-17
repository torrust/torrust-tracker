---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/test_helpers.rs
status: done
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/test_helpers.rs
    - packages/tracker-core/tests/common/fixtures.rs
    - packages/test-helpers/src/configuration.rs
    - packages/axum-rest-api-server/src/test_support.rs
---

# Ephemeral Test Database File Test Plan

This plan follows the [Per-File Workflow](../ISSUE.md#per-file-workflow). It is the test-support
change that fixes finding F5 (Q7), and the SQLite half of the design-review checkpoint T2b (the
first `TempDir`-owned SQLite fixture). It changes test code only: no production line changes.

It was first developed on the S1 branch and split into its own pull request, #2923, on
2026-11-17, at the maintainer's request, because issue #2907 (EPIC #2392, test isolation) planned
the same fix for the shared `test-helpers` function and every package. #2923 covers `tracker-core`
only. The #2907 session confirmed on 2026-11-17 that `tracker-core`'s callers are out of its scope
and that the SQLite part moves to a new #2392 subissue, which reuses the shape below.

## Current State

- `torrust_tracker_test_helpers::configuration::ephemeral_sqlite_database()` returns a bare path
  under `env::temp_dir()`; nothing removes the file. The shared `ephemeral()`, `ephemeral_public()`,
  `ephemeral_private()` and `ephemeral_listed()` configurations use it too.
- Measured on 2026-11-17 (stable Rust 1.99.1): one `cargo test -p torrust-tracker-core
  --all-features` left 72 new `data_*.db` files of about 48 KB in `/tmp` (61 from the unit tests,
  11 from the integration tests). `/tmp` held 267,706 such files from earlier runs of all packages.
- Six sources in this package: (1) `src/test_helpers.rs` (`ephemeral_configuration` and
  `ephemeral_configuration_for_listed_tracker`); (2) `tests/common/fixtures.rs`; (3) to (5) private
  copies in the test modules of `databases/driver/sqlite/mod.rs`, `sqlite/schema_migrator.rs` and
  `authentication/key/repository/persisted.rs`; (6) the shared full configurations, used by the test
  modules of `lib.rs`, `announce_handler.rs`, `authentication/{mod,handler}.rs` and
  `whitelist/test_helpers.rs`. The sixth was found while implementing; `scrape_handler.rs` also
  uses a shared configuration but never opens its database, so it leaves no file.

## Maintainer Decisions

All on 2026-11-17:

- **Q7:** fix it in S1, in this package only; the shared `test-helpers` function stays for the
  other packages.
- **Shape:** a struct with a public `config` and a private `TempDir` guard, the pattern of
  `TestApiContainer` in `axum-rest-api-server` (#2637).
- **Scope:** every source (the maintainer chose "all five now" with the goal of 0 files after a
  run; the sixth source, found afterwards, follows the same decision).
- **Guard owner:** the test. A helper that opens a database takes `&Core` or `&Configuration`; the
  test creates the configuration in its Arrange and keeps it alive.

## Fixture Design

- `test_helpers::tests::EphemeralConfiguration { pub config: Core, _database_directory: TempDir }`,
  built by `ephemeral_configuration()` and `ephemeral_configuration_for_listed_tracker()`.
- `test_helpers::tests::EphemeralTrackerConfiguration { pub config: Configuration, .. }`, built by
  `ephemeral_tracker_configuration(config)`, which moves the database of a shared configuration
  into the directory. Test modules wrap it in a named fixture (`public_tracker()`,
  `listed_tracker()`, `private_tracker()`, `private_tracker_with_keys_expiration_check_disabled()`).
- `tests/common/fixtures.rs` has its own `EphemeralConfiguration` (the integration binary cannot
  see `#[cfg(test)]` items of the library).
- `schema_migrator.rs`'s `new_pool()` returns the `TempDir` with the pool and the path.
- The database file is `data.db` inside the directory. If the guard is dropped too early, the
  directory is gone and SQLite fails to open the file, so a misuse fails the test instead of
  leaking. Probe: with every caller dropping the guard at once (`ephemeral_configuration().config`),
  26 of the unit tests that open a database failed with `unable to open database file`; the change was reverted.
- **Dependency:** `tempfile = "3.27.0"` as a dev-dependency, the version the workspace already uses
  (root `Cargo.toml` and `axum-rest-api-server`), so `Cargo.lock` gains no new package.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| E1 | DONE | Shared unit-test fixtures and their callers (sources 1 and 6). | 157 unit tests pass. |
| E2 | DONE | Integration fixture (source 2). | 12 integration tests pass. |
| E3 | DONE | The three private copies (sources 3 to 5). | Unit tests pass. |
| E4 | DONE | Count the files a full run leaves. | Results below. |

## Results

- **Leak:** after the change, `cargo test -p torrust-tracker-core --all-features` (157 unit, 12
  integration and 18 other tests, all passing, on `develop` at the time of the split) left 0 new `data_*.db` files and 0 new temporary
  directories in `/tmp` (counted with `find /tmp -maxdepth 1 -newer <marker>`), against 72 files
  before.
- **Tests:** no test added or removed; the changed tests keep their assertions. 22 files changed,
  all test code, plus the dev-dependency.
- **Validation:** `cargo clippy -p torrust-tracker-core --all-targets --all-features` (stable; only
  the existing `axum::serve` warning of `clippy.toml`), `cargo +nightly fmt --all -- --check`
  (nightly), `git diff --check` and `cargo machete --with-metadata` pass.
- **Not done:** the 267,706 files already in `/tmp` are not removed by this change; they belong to
  the person running the tests.
- **Review outcome:** the maintainer approved the completed change on 2026-11-17, including the sixth source, and asked for this separate PR.
- **Commits:** `test(tracker-core): [#2902] keep each unit-test database in a temporary directory`, `test(tracker-core): [#2902] keep each integration-test database in a temporary directory` and `test(tracker-core): [#2902] remove the private temporary-database helpers`.
