---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/databases/driver/mysql/mod.rs
status: done
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/databases/driver/mysql/mod.rs
    - packages/tracker-core/src/databases/driver/postgres/mod.rs
    - packages/tracker-core/src/databases/setup.rs
    - .github/workflows/db-compatibility.yaml
    - packages/tracker-core/README.md
    - packages/AGENTS.md
---

# Server Database Drivers Test Plan

Follow the shared [guidance](README.md). This plan covers ledger rows 18 to 27, slice S3: the MySQL
(`mysql/{mod,auth_key_store,schema_migrator,torrent_metrics_store,whitelist_store}.rs`) and PostgreSQL
(`postgres/...`, the same five files) drivers, plus the driver-test commands of Q2.

## Current State

- **Unit-only coverage** (`--all-features --lib`, stable Rust 1.100.0): `mysql/mod.rs` 7 / 211 lines,
  `postgres/mod.rs` 7 / 188, each store file 0 / 6 to 0 / 77. Almost everything is behind
  `#[cfg(all(test, feature = "db-compatibility-tests"))]` and the two `TORRUST_TRACKER_CORE_RUN_*_DRIVER_TEST`
  variables: one suite per driver (`run_mysql_driver_tests`, `run_postgres_driver_tests`) runs the shared
  `run_tests` harness against a container. `.github/workflows/db-compatibility.yaml` runs both on every
  change under `packages/tracker-core/`, against MySQL 8.0 and 8.4 and PostgreSQL 14 to 17.
- **Decision of Q3:** Docker-free tests only for what needs no server, kept only if fast and not flaky;
  the happy path stays with `db-compatibility.yaml`.
- **Measurement (2026-11-24, this machine):** `Mysql::new("mysql://u:p@127.0.0.1:1/db")` builds a lazy
  pool without connecting; the first store call then fails with `ConnectionError { source: PoolTimedOut }`
  **after 30.0 s**, the fixed `sqlx` acquire timeout, because the drivers build their pools with the
  default options. A malformed URL (`"not a url"`) fails at once with
  `ConnectionError { source: Configuration(RelativeUrlWithoutBase) }`. An unreachable-endpoint test
  therefore fails the "fast" condition of Q3, and a shorter timeout needs a production change, which the
  maintainer declined on 2026-11-24 (Q8 below).
- **The setup arms** (`setup.rs` lines 119-128): the URL is built by `Database::connection_url()` from
  typed fields and percent-encoded, so a configuration cannot produce a URL that fails to parse; the arms
  can only fail after the 30 s timeout. They stay uncovered by unit tests and are owned by the Docker suites.

## Maintainer Decision

Q8 answered on 2026-11-24 (the measurement above): **instant tests only**. Plan approved the same day.

## Test Design

One test module per driver, always compiled (`#[cfg(test)]`, no feature), next to the existing gated one.
Each has an `rstest` table over the malformed URLs and one test of the well-formed URL; all are `#[tokio::test]`, because building a lazy pool needs a Tokio context:

| Case | Input | Assert |
| --- | --- | --- |
| a malformed URL | `"not a url"` | `Err(ConnectionError { source: Configuration(..), driver })` with the driver's own `Driver` value |
| a port that is not a number | `mysql://user:password@127.0.0.1:port/db` (`postgresql://..` for PostgreSQL) | the same error and driver |
| a well-formed URL | `mysql://user:password@127.0.0.1:1/db` (the port is closed) | `Ok`: the pool is lazy and no connection is attempted |

## Rows 18-27 decisions

| Rows | Files | Decision |
| --- | --- | --- |
| 18 and 23 | `mysql/mod.rs`, `postgres/mod.rs` | the constructor table above; the rest of the file (aggregate-metric helpers and the gated suite) is no-change: it needs a server |
| 19-22 and 24-27 | the store and migrator files | no change: every executable line talks to the database, so a Docker-free test would wait 30 s for the pool timeout; `db-compatibility.yaml` owns them through the shared `run_tests` harness |

## Q2 documentation fix

`packages/tracker-core/README.md` (Testing) and `packages/AGENTS.md` (Testing Packages) give
`TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST=true cargo test ...`, which omits `--features
db-compatibility-tests` (without it the gated module is not compiled and nothing runs) and names only
MySQL. Both are corrected to the commands of the workflow, for both drivers. Documentation only.

## Property-Test Decision

No `quickcheck`: the inputs are three fixed URLs per driver.

## Named Mutations

| ID | Mutation | Expected killer |
| --- | --- | --- |
| M1 | `new` reports `Driver::Sqlite3` instead of its own driver on a parse error | the malformed-URL case |
| M2 | `new` ignores a parse error (`unwrap_or_default()` on the options) | the two malformed-URL cases |
| M3 | `new` returns `Err` for every URL | the well-formed-URL case |

Applied to both drivers; a mutant that does not compile is rewritten.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| D1 | DONE | The two constructor modules. | They pass in 0.00 s (6 tests). |
| D2 | DONE | Run M1-M3 on both drivers. | Each killed by its case. |
| D3 | DONE | Correct the README and `packages/AGENTS.md` commands. | `linter all` passes. |
| D4 | DONE | Record the results, the ledger rows and the coverage; ask for the completed-file review. | This record and the ledger. |

## Prose-First AAA Review

Temporary prose, written from this plan's Test Design, then compared with the code. The two drivers have the same three tests.

| Test | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_report_a_connection_error_of_its_own_driver_when_the_url_cannot_be_parsed` (cases) | A URL that cannot be parsed: not a URL at all, or a port that is not a number. | Build the driver from it. | The error is a connection error of the driver's own `Driver` value. | Matches the code. The Arrange is the case argument. The source error is not matched, because it is a shared `dyn Error`. |
| `it_should_build_a_lazy_pool_without_connecting_when_the_url_is_well_formed` | A well-formed URL whose port (1) nothing listens on. | Build the driver. | It is `Ok`: a constructor that connected would fail. | Matches the code. |

Smells found: none. A first draft had a "URL of another driver" case; it was wrong (the MySQL parser accepts a `postgresql://` URL and the PostgreSQL parser a `mysql://` one, so the constructor succeeded) and was replaced by the port case.

## Results

- **Coverage** (stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, `--all-features --lib`, clean coverage target directory, 2026-11-24): `mysql/mod.rs` 7 / 211 to 19 / 217 lines, `postgres/mod.rs` 7 / 188 to 19 / 194 (the counts include the test modules); the package's unit-only lines 5,152 / 5,913 (87.13%) to 5,176 / 5,925 (87.36%), regions 5,724 / 6,621 (86.45%), functions 909 / 1,101 (82.56%). The other eight files stay as they were.
- **Tests:** 6 added (two drivers times three): 281 to 287 unit tests in the package. Each runs in under a millisecond.
- **Mutations** (each applied alone to each driver and reverted, the file compared with its copy afterwards):

  | ID | MySQL | PostgreSQL |
  | --- | --- | --- |
  | M1 | Killed by both malformed-URL cases | Killed by both malformed-URL cases |
  | M2 | Killed by both malformed-URL cases | Killed by both malformed-URL cases |
  | M3 | Killed by the well-formed-URL test | Killed by the well-formed-URL test |

- **Docs:** the README and `packages/AGENTS.md` commands now name `--features db-compatibility-tests`, both drivers and both variables; the README says a skipped run reports as passed.
- **Review outcome:** the maintainer approved the completed work on 2026-11-24.
- **Validation:** `cargo clippy -p torrust-tracker-core --all-targets --all-features` (only the
  existing `axum::serve` warning), `cargo +nightly fmt --all -- --check` and
  `cargo machete --with-metadata` pass.
