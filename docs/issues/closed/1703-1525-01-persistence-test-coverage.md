---
doc-type: issue
issue-type: task
status: done
priority: p1
github-issue: 1703
spec-path: docs/issues/closed/1703-1525-01-persistence-test-coverage.md
branch: 1703-1525-01-persistence-test-coverage
related-pr: null
last-updated-utc: null
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/README.md
    - docs/issues/closed/1525-overhaul-persistence.md
    - packages/tracker-core/
---

# Subissue #1703 (Draft for #1525-01): Add DB Compatibility Matrix

- Issue: https://github.com/torrust/torrust-tracker/issues/1703

## Goal

Establish a compatibility matrix that exercises persistence-layer tests across supported database
versions before any refactoring begins.

## Why First

The later refactors change persistence architecture, async behavior, schema setup, and backend
implementations. Running the tests against multiple database versions first gives a baseline to
detect regressions early and narrows review scope to behavior rather than guesswork.

## Scope

- Bash is acceptable for low-complexity orchestration.
- Focus only on the database compatibility matrix; end-to-end real-client testing is covered by
  subissue #1525-02.

## Testing Principles

The implementation must follow these quality rules for all new and modified tests.

- **Isolation**: Each test run must be independent. Tests that spin up database containers via
  `testcontainers` already get their own ephemeral container; the bash matrix script achieves
  isolation by running one matrix cell at a time in a fresh process, each with an exclusively
  allocated container.
- **Independent system resources**: Tests must not hard-code host ports. `testcontainers` binds
  containers to random free host ports automatically — do not override this with fixed bindings.
  Temporary files or directories, if needed, must be created under a `tempfile`-managed path so
  they are always removed on exit.
- **Cleanup**: After each test (success or failure) all containers, volumes, and temporary files
  must be released. `testcontainers` handles containers automatically when the handle is dropped;
  ensure `Drop` is not suppressed.
- **Behavior, not implementation**: Tests must assert observable outcomes (e.g. the driver
  correctly inserts and retrieves a torrent entry) rather than internal state (e.g. a specific SQL
  query was issued).
- **Verified before done**: No test is considered complete until it has been executed and passes
  in a clean environment. Include confirmation of a passing run in the PR description.

## Reference QA Workflow

The PR #1695 review branch includes a QA script that defines the expected behavior:

- `database-compatibility` job in `.github/workflows/testing.yaml`:
  executes a compatibility matrix across SQLite, multiple MySQL versions, and multiple PostgreSQL
  versions.

This should be treated as a reference prototype, not a production artifact. The goal is to
re-implement it in a form that integrates with the repository's normal test strategy.

## Dependency Note

PostgreSQL is not implemented yet, so this subissue cannot require successful execution against
PostgreSQL. The structure should make it easy to add PostgreSQL combinations in subissue
`#1525-08` once the driver exists.

## Proposed Branch

- `1525-01-db-compatibility-matrix`

## Tasks

### 1) Port the compatibility matrix workflow

Add a low-complexity bash compatibility-matrix runner that exercises persistence-related tests
across supported database versions.

Tests to orchestrate:

- `cargo check --workspace --all-targets`
- configuration coverage for PostgreSQL connection settings
- large-download counter saturation tests in the HTTP protocol layer
- large-download counter saturation tests in the UDP protocol layer
- SQLite driver tests
- MySQL driver tests across selected MySQL versions

Note: PostgreSQL version-matrix execution is deferred to subissue #1525-08, once the
PostgreSQL driver exists.

Steps:

- Modify current DB driver tests so the DB image version can be injected through environment
  variables:
  - MySQL: `TORRUST_TRACKER_CORE_MYSQL_DRIVER_IMAGE_TAG`
  - PostgreSQL (reserved for subissue #1525-08): `TORRUST_TRACKER_CORE_POSTGRES_DRIVER_IMAGE_TAG`

  When `TORRUST_TRACKER_CORE_MYSQL_DRIVER_IMAGE_TAG` is not set, the test falls back to the
  current hardcoded default (e.g. `8.0`), preserving existing behavior. The CI matrix job sets
  this variable explicitly for each version in the loop, so unset means "run as today" and the
  matrix just expands that into multiple combinations.

- Add a dedicated `database-compatibility` workflow job (between unit and e2e) with matrix values for MySQL versions:
  - include matrix values for at least `8.0` and `8.4`
  - run `cargo test -p bittorrent-tracker-core --features db-compatibility-tests run_mysql_driver_tests -- --nocapture`
  - set `TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST=true`
  - set `TORRUST_TRACKER_CORE_MYSQL_DRIVER_IMAGE_TAG=<version>`
  - keep the test logic in Rust; use workflow matrix for version fan-out
- Replace the current single MySQL `database` step in `.github/workflows/testing.yaml` with a
  dedicated `database-compatibility` job.

Acceptance criteria:

- [ ] DB image version injection is supported via `TORRUST_TRACKER_CORE_MYSQL_DRIVER_IMAGE_TAG`
      (and a reserved `POSTGRES` equivalent for subissue #1525-08).
- [ ] `database-compatibility` workflow job runs successfully for each configured MySQL version.
- [ ] The workflow matrix exercises at least two MySQL versions by default.
- [ ] Failures identify the backend/version combination that broke.
- [ ] The dedicated `database-compatibility` job in `.github/workflows/testing.yaml` replaces the
      old single-version MySQL command.
- [ ] The workflow matrix structure allows PostgreSQL to be added in subissue #1525-08 without a
      redesign.
- [ ] Tests do not hard-code host ports; `testcontainers` assigns random ports automatically.
- [ ] All containers started by tests are removed unconditionally on test completion or failure.

### 2) Document the workflow

Steps:

- Document the local invocation command for the compatibility test using explicit feature + env
  vars.
- Document that CI runs the same test through the `database-compatibility` workflow job matrix.

Acceptance criteria:

- [ ] The compatibility test command is documented and runnable without ad hoc manual steps.

## Out of Scope

- qBittorrent end-to-end testing (covered by subissue #1525-02).
- Adding PostgreSQL support itself.
- Refactoring the production persistence interfaces.
- Performance benchmarking, before/after comparison, and benchmark reporting.

## Definition of Done

- [ ] `cargo test --workspace --all-targets` passes.
- [ ] `linter all` exits with code `0`.
- [ ] The `database-compatibility` workflow job has been executed successfully in a clean
      environment; a passing run log is included in the PR description.

## References

- EPIC: #1525
- Reference PR: #1695
- Reference implementation branch: `josecelano:pr-1684-review` — see EPIC for checkout
  instructions (`docs/issues/1525-overhaul-persistence.md`)
- Reference job: `.github/workflows/testing.yaml` `database-compatibility`
