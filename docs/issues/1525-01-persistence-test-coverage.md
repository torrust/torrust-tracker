# Subissue Draft for #1525-01: Add DB Compatibility Matrix

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

- `run-db-compatibility-matrix.sh`:
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
  current hardcoded default (e.g. `8.0`), preserving existing behavior. The matrix script sets
  this variable explicitly for each version in the loop, so unset means "run as today" and the
  matrix just expands that into multiple combinations.

- Add `contrib/dev-tools/qa/run-db-compatibility-matrix.sh` modeled after the PR prototype:
  - `set -euo pipefail`
  - define default version sets from env vars:
    - `MYSQL_VERSIONS` defaulting to at least `8.0 8.4`
    - `POSTGRES_VERSIONS` reserved for subissue #1525-08
  - run pre-checks once (`cargo check --workspace --all-targets`)
  - run protocol/configuration tests once
  - run SQLite driver tests once
  - loop MySQL versions: `docker pull mysql:<version>`, then run MySQL driver tests with
    `TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST=1` and
    `TORRUST_TRACKER_CORE_MYSQL_DRIVER_IMAGE_TAG=<version>`
  - print a clear heading for each backend/version before executing tests
  - fail fast on first failure with the failing backend/version visible in logs
  - keep script complexity intentionally low; avoid re-implementing test logic already in test
    functions
- Replace the current single MySQL `database` step in `.github/workflows/testing.yaml` with
  execution of the new script.

Acceptance criteria:

- [ ] DB image version injection is supported via `TORRUST_TRACKER_CORE_MYSQL_DRIVER_IMAGE_TAG`
      (and a reserved `POSTGRES` equivalent for subissue #1525-08).
- [ ] `contrib/dev-tools/qa/run-db-compatibility-matrix.sh` exists and runs successfully.
- [ ] The script exercises SQLite and at least two MySQL versions by default.
- [ ] Failures identify the backend/version combination that broke.
- [ ] The `database` job step in `.github/workflows/testing.yaml` runs the matrix script instead
      of a single-version MySQL command.
- [ ] The script structure allows PostgreSQL to be added in subissue #1525-08 without a redesign.
- [ ] Tests do not hard-code host ports; `testcontainers` assigns random ports automatically.
- [ ] All containers started by tests are removed unconditionally on test completion or failure.

### 2) Document the workflow

Steps:

- Document the local invocation command for the matrix script.
- Document that the CI `database` step runs the same script.

Acceptance criteria:

- [ ] The matrix script is documented and runnable without ad hoc manual steps.

## Out of Scope

- qBittorrent end-to-end testing (covered by subissue #1525-02).
- Adding PostgreSQL support itself.
- Refactoring the production persistence interfaces.
- Performance benchmarking, before/after comparison, and benchmark reporting.

## Definition of Done

- [ ] `cargo test --workspace --all-targets` passes.
- [ ] `linter all` exits with code `0`.
- [ ] The matrix script has been executed successfully in a clean environment; a passing run log
      is included in the PR description.

## References

- EPIC: #1525
- Reference PR: #1695
- Reference implementation branch: `josecelano:pr-1684-review` — see EPIC for checkout
  instructions (`docs/issues/1525-overhaul-persistence.md`)
- Reference script: `contrib/dev-tools/qa/run-db-compatibility-matrix.sh`
