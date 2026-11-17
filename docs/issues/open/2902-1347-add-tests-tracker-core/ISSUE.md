---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 1347
github-issue: 2902
spec-path: docs/issues/open/2902-1347-add-tests-tracker-core/ISSUE.md
branch: "2902-1347-add-tests-tracker-core-spec"
related-pr: null
last-updated-utc: "2026-11-17 18:04"
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - .claude/skills/create-issue/SKILL.md
    - .claude/skills/write-unit-test/SKILL.md
    - docs/issues/open/1347-overhaul-packages-testing/EPIC.md
    - docs/issues/open/1347-overhaul-packages-testing/subissue-spec-guidelines.md
    - docs/issues/closed/2824-1347-add-tests-primitives/ISSUE.md
    - docs/issues/closed/2637-1347-add-tests-axum-rest-api-server/ISSUE.md
    - docs/issues/closed/1349-1347-add-tests-http-core/ISSUE.md
    - docs/issues/closed/1348-1347-add-tests-udp-core/ISSUE.md
    - docs/issues/open/2902-1347-add-tests-tracker-core/coverage-evidence.md
    - packages/tracker-core
---

<!-- skill-link: create-issue -->

# Issue #2902 - Add Tests to the Tracker-Core Package

Parent EPIC: #1347 - Overhaul: Packages Testing

## Goal

Increase the maintainable, package-local test coverage of `torrust-tracker-core` by going over
every Rust source file in `packages/tracker-core/src/`. For each file: review the existing tests
against the `write-unit-test` skill, analyse coverage, add unit tests for the package-owned
behavior (the persistence failure paths, the statistics counters and listeners, and the failing
collaborator cases), and record the resulting per-file coverage. Behavior that needs a real
database server is assessed and owned explicitly, not left implicit.

Deliver the work as a short sequence of small pull requests, each merged before the next slice
starts.

## Background

`torrust-tracker-core` is the domain core of the tracker: the announce and scrape handlers, peer
authentication keys, the whitelist, torrent and swarm statistics, the in-memory and persisted
repositories, the database drivers (SQLite, MySQL, PostgreSQL) and the container that wires them.
It is 11,369 source lines in 62 files (51 with executable lines), with 157 unit tests, 12 integration tests
(`tests/integration.rs`) and a container-gated database suite. EPIC #1347 names it "the reference
for effective package-level test coverage" (see its Scope), and most of it is: 26 of the 51 files with
executable lines are at or above 98% unit-only coverage. The remaining gap is concentrated:

- The MySQL and PostgreSQL drivers (10 files, 728 measured lines) are 0-4% covered because their
  suite skips itself unless Docker and two environment variables are present.
- The SQLite stores and `databases/{error,setup}.rs` have their failure branches untested.
- `statistics/` (repository, metrics, persisted load, event listeners, handlers) is reached only by
  the integration binary.
- Many `?` regions (a failing `Database` or whitelist) are untested in the handlers and repositories.

Its unit-only lines (80.31%) are the lowest of the packages with a unit-only figure in the EPIC
coverage table: the latest figures there are `primitives` 94.07%, `axum-rest-api-server` 96.67%,
`udp-core` 98.04%, `udp-server` 98.18% and `http-core` 99.28%. Since the integration binary reaches only the `statistics/` files that no unit test
reaches, the aggregate and unit-only numbers differ in exactly that area.

## Baseline Coverage

Measured 2026-11-17 at `develop` commit `5833cdae9` with `cargo-llvm-cov 0.6.16` and stable Rust
1.99.1 from a clean workspace; totals include only `packages/tracker-core/src/` (in-file test
modules and `src/test_helpers.rs` included).

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Unit-only (`--all-features --lib`) | 157 unit | 4,099 / 5,104 (80.31%) | 5,716 / 7,127 (80.20%) | 687 / 954 (72.01%) |
| Integration-only (`--all-features --test integration`) | 12 integration | 841 / 1,909 (44.05%) | 936 / 2,506 (37.35%) | 145 / 484 (29.96%) |
| Aggregate (`--all-features`) | 157 unit + 12 integration | 4,230 / 5,149 (82.15%) | 5,834 / 7,182 (81.23%) | 703 / 958 (73.38%) |
| Unit-only without `db-compatibility-tests` (`--lib`) | 155 unit (the two driver tests are not compiled) | 4,085 / 4,780 (85.46%) | 5,700 / 6,691 (85.19%) | 683 / 916 (74.56%) |

With the MySQL and PostgreSQL suites actually running (Docker `mysql:8.0` and `postgres:16`, both
`TORRUST_TRACKER_CORE_RUN_*_DRIVER_TEST=true`, measured 2026-11-17 with the maintainer's permission)
the aggregate is lines 4,830 / 5,149 (93.80%), regions 93.61%, functions 84.45%: the server drivers
reach 58-96% per file, and what stays uncovered there are the failure branches (F2). The default
runs above do not include them (F1).
The per-file inventory and the uncovered-line groups are in [coverage-evidence.md](coverage-evidence.md).
Indirect coverage from other packages' tests is not accepted as evidence.

## Key Package Responsibilities

| Rank | Responsibility | Source modules | Why it matters |
| ---: | --- | --- | --- |
| 1 | Persistence boundary | `databases/` (traits, `error.rs`, `setup.rs`, SQLite, MySQL, PostgreSQL drivers) | Keys, the whitelist and the download counters survive restarts only through it; a malformed row, a negative timestamp or a broken connection must surface as a typed error, never as silent loss. |
| 2 | Statistics and startup load | `statistics/` (repository, metrics, persisted load, listeners, handlers) | Counters and gauges feed the `/stats` and metrics endpoints; `load_persisted_metrics` restores the downloads total at startup; listeners must drain on shutdown. |
| 3 | Authentication, whitelist, torrent state | `authentication/`, `whitelist/`, `torrent/` | Decide who may announce; every one of their repositories depends on a `Database` that can fail. Already well covered; the failure cases are the gap. |
| 4 | Announce and scrape | `announce_handler.rs`, `scrape_handler.rs` | The hot path of the tracker. 99-100% covered; confirm no decision is left to the integration binary. |
| 5 | Wiring | `container.rs`, `lib.rs`, `error.rs`, `test_helpers.rs`, `peer_tests.rs` | Composition and error conversions. |

## Coverage Objectives

EPIC #1347 sets qualitative, risk-based objectives, not a numeric target.

1. **Highest** - the SQLite persistence boundary: every store's parse and query failure branch, the
   `MalformedDatabaseRecord` mapping, `Sqlite::new` path handling, and the `sqlx` error to
   `databases::error::Error` mapping (including the migration error).
2. **High** - the statistics counters, gauges and the persisted-load path, and the listener
   wrappers (spawn, event handled, cancel); each must be protected by a unit test, not only the
   integration binary.
3. **Medium** - failing-`Database` propagation in the authentication, whitelist and announce paths
   (the `?` regions), the container's `Persistence` error mapping and the `TrackerCoreError`
   conversions.
4. **Assessed, owner recorded** - the MySQL and PostgreSQL drivers: unit tests only for what needs
   no server (error mapping against an unreachable endpoint, if non-flaky); the happy path stays
   with the `db-compatibility` workflow and its owner is recorded (questions Q1-Q3).
5. **Low** - logging-only handlers, test support and assertion-only lines.

The objective is met when every package-owned highest and high decision is protected by a unit
test, and every other uncovered group has a recorded owner-based reason, not when a percentage is
reached.

## Test-Technique Assessment

See the table in [coverage-evidence.md](coverage-evidence.md#test-technique-assessment-initial).
Unit tests are the primary level; the 12 existing integration tests stay as they are. Property-based
testing is assessed per seam: the candidates are the authentication `Key` round trip and the
expiry/`parse_valid_until` bounds (the package has no property-test dependency; `quickcheck` was
added to `primitives` and `udp-core` by their issues, following `add-rust-dependency`). A property
is added only if it protects a state that examples miss.

## Scope

### In Scope

- The complete per-file inventory and baseline ([coverage-evidence.md](coverage-evidence.md)).
- Process every file through the per-file workflow below. Each file gets one concise decision record
  (about 40 lines) in `test-refactor-plans/`; no-change files may share one record.
- Add focused, deterministic unit tests for package-owned decisions, using temp SQLite files, the
  `testing` feature's `TorrentMetricsStore` mock and small failing fakes of the persistence traits.
  Review existing tests against `write-unit-test` and refactor those that have smells only when a
  record says so.
- Record the container-gated measurement (T1b, done).
- Dev-dependencies a record justifies (`quickcheck`), following `add-rust-dependency` and
  `cargo machete --with-metadata`.
- Documentation-comment corrections in the files under test.
- Hand off production findings through the [findings procedure](#findings-during-implementation).
- A bounded mutation sample after the last test increment.
- Issue-local `lessons.md` for reusable lessons; promoting them elsewhere is a separate small PR.

### Out of Scope

- Production behavior changes in `packages/tracker-core/src/`, unless the maintainer decides a
  small defect is fixed here (it then goes to its own `fix` commit in the first test slice that
  needs it).
- Tests of other packages that use `tracker-core`; shared test-support reorganization; #2301 and
  #2361.
- Changing the container-gated MySQL/PostgreSQL suites or their CI workflow (assessed only; see Q2).
- Refactoring the three copies of `parse_valid_until` and the row mapping into one helper
  (finding F2).
- Coverage-percentage targets and tests added only to move line totals.
- Cross-cutting process, skill or pattern-catalog edits.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260429000000_keep_database_as_aggregate_supertrait.md` (the `Database`
  trait shape the failing fakes implement),
  `docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md` (the
  `testing` feature and the `TorrentMetricsStore` mock),
  `docs/adrs/20260826124959_use_explicit_identifiers_for_test_log_assertions.md` (any log
  assertion), `docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md` (unit tests run
  in the container build, so they must not need Docker) and
  `docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md` (where the
  container-gated suites run).
- No package-local ADRs. None to create; if a design question (Q2) needs a decision record, it goes
  to its own issue under `create-adr`.

## Design and Ownership Review

Questions each file's record must answer; the first four are already in the
[Findings Register](#findings-register):

- The database suites pass without running (F1): never claim their coverage from a green run.
- Three copies of the same row-to-key mapping (F2): a test of one copy does not protect the others.
- `Key` derives `Deserialize` without validation (F3): do not pin the accepted inputs of the derive.
- The persisted whitelist `add` and `remove` check then write (F4): do not pin the race.
- The swarm registry and the `events` package own the registry state and the drain behavior: this
  package tests its own wiring and statistics, not theirs.

## Asynchronous Work, Cleanup and Test Fixtures

S1 spawns and cancels the statistics listeners, S2, S4 and S5 open SQLite databases on disk, and
each slice may add a failing fake of the persistence traits, so the `create-issue` rules for
asynchronous I/O, resource cleanup and reusable fixtures apply. Each file's decision record that adds
such a test restates the parts it uses.

Responsibility and ownership map:

| Resource | Created by | Owner of normal cleanup | Owner of failure and drop-path cleanup |
| --- | --- | --- | --- |
| Spawned listener task (`run_*_event_listener`) | The test | The test: cancels the `CancellationToken`, then awaits the `JoinHandle` | The `#[tokio::test]` runtime: when a test panics, dropping the per-test runtime drops the task. The test never relies on a detached task finishing |
| Event sender and receiver | The test, through the swarm registry's channel | The test drops the sender after the last event | Dropped with the test's stack frame |
| SQLite database file (new tests) | The test, in a `tempfile::TempDir` it owns | `TempDir` drop at the end of the test | `TempDir` drop during unwinding after a failed assertion |
| SQLite database file (existing tests) | `test_helpers::tests::ephemeral_configuration`, `ephemeral_configuration_for_listed_tracker` and `ephemeral_tracker_configuration`, and `tests/common/fixtures.rs` (since the F5 fix) | The test: the returned value's private `TempDir` removes the directory when the test drops it | `TempDir` drop during unwinding; an early drop makes SQLite fail to open the file, so misuse fails the test (F5, fixed) |
| Failing fake of a persistence trait | The slice's test module | Dropped with the test | Dropped with the test; holds no global or shared state |
| MySQL and PostgreSQL containers | `testcontainers` in the existing suites | `testcontainers` (unchanged by this issue) | `testcontainers` (unchanged) |
| Drain of queued events after cancellation | `torrust-tracker-events` (`drain`) | The `events` package; this package tests only that its listeners call it | The `events` package |

Lifetime invariants:

- Normal path: when a test returns, its listener task has finished (the `JoinHandle` was awaited), its
  temporary directory is removed and no fake is shared with another test.
- Failure and drop path: a failed assertion unwinds through the `TempDir` (directory removed) and
  ends the per-test runtime (task dropped); no test leaves a running task or a new file behind.
- New tests do not call `ephemeral_sqlite_database()` without a cleanup owner.

Deadlines: every awaited operation that waits on another task (awaiting a listener's `JoinHandle`,
waiting until an event has been handled, a drain) is wrapped in `tokio::time::timeout` with an
absolute deadline (5 seconds unless the record justifies another value) and fails with a message
naming the operation. Database calls on a local temporary SQLite file use the same bound when they
can wait on the pool. No test sleeps for a fixed time to wait for a result.

Design-review checkpoint (T2b): after the first passing vertical slice, meaning the first listener
test (spawn, handle one event, cancel, join under a deadline) and the first `TempDir`-owned SQLite
test pass in S1, stop and ask the maintainer to review the fixture shape before any other file
reuses it.

## Bug-Fix Process

The issue as a whole is coverage and test-design work, not a known defect. A defect found is handed
off through the findings procedure and fixed in its own issue under the `fix-bug` skill, unless the
maintainer decides a small one is fixed here; then the first slice that needs it follows the
`fix-bug` workflow and records the reproduction, the red and green regression-test results and the
final recheck in `manual-verification-evidence.md`.

## Regression Test Strategy

For each increment choose the smallest deterministic test that protects the package-owned behavior
and record it in the decision record. Never pin behavior that a handed-off finding reports as
wrong without a recorded maintainer decision: leave it uncovered with the finding as owner until
the fix merges.

## Findings During Implementation

A finding that needs a production change is handled this way (same procedure as #1348, #1349, #2637
and #2824):

1. Stop the current increment at a clean point.
2. Write a draft bug spec under `docs/issues/drafts/<short-topic>/ISSUE.md` on its own branch cut
   from `develop` (never on a slice branch), with the `Bug-Fix Process` and `Regression Test
   Strategy` sections, and a git-ignored `.tmp/<issue>-<short-topic>/HANDOFF.md`.
3. Stop and tell the maintainer; he decides whether this work pauses.
4. Record a progress-log entry and a `lessons.md` row naming the draft or issue.
5. Resume after a pause: rebase, re-measure the affected rows, re-check their records.

Non-bug findings (dead code, misleading docs) need only a `lessons.md` row.

### Findings Register

Found while writing this specification (code reading and the coverage runs, 2026-11-17). Each is a
proposal for the maintainer unless its status says otherwise.

| ID | Finding | Evidence | Proposed owner | Status |
| --- | --- | --- | --- | --- |
| F1 | `run_mysql_driver_tests` and `run_postgres_driver_tests` return `Ok(())` immediately when `TORRUST_TRACKER_CORE_RUN_*_DRIVER_TEST` is unset, so `cargo test --all-features` reports them as passed without running anything, and the coverage runs count both drivers as 0-4% covered. A reader of a green run cannot tell. | `databases/driver/mysql/mod.rs:204-207`, `postgres/mod.rs:185-188`; the unit-only coverage log reports both tests `ok` in a 157-test run that finished in 0.13 s, and the run with the variables set took 9.32 s | A separate test-design issue: make the skip visible (`#[ignore = "..."]`, or a loud skip message) and keep the `db-compatibility` workflow as it is. Not part of this issue (Q2). | OPEN (proposed) |
| F2 | The row-to-`PeerKey` mapping and `parse_valid_until` (negative timestamp to `MalformedDatabaseRecord`) are copied in the SQLite, MySQL and PostgreSQL `auth_key_store.rs`; the `u32` conversion of the aggregate metric is copied likewise. A test of the SQLite copy protects none of the others, and Docker-less tests cannot reach the MySQL and PostgreSQL copies. | `parse_valid_until` is identical in the three files (MySQL and PostgreSQL lines 119-125, SQLite lines 122-128) | A refactor issue extracting one helper (Q4); until then each driver's copy is assessed in its own record. | OPEN (proposed) |
| F3 | `authentication::key::Key` derives `Deserialize` over its inner `String`, so deserializing a key skips the 32-character, ASCII-alphanumeric validation of `Key::new`. To confirm in the `peer_key.rs` record: where (if anywhere) a `Key` is deserialized from untrusted input (REST path and JSON are the candidates). | `authentication/key/peer_key.rs:127` | Confirm first; if reachable, a bug issue under `fix-bug`. | TO CONFIRM |
| F4 | `PersistedWhitelist::add` and `remove` read `is_info_hash_whitelisted` and then write, without a transaction; two concurrent adds of the same hash can reach the driver's duplicate-insert error that `it_should_fail_trying_to_add_the_same_infohash_twice` shows is an error. Probably benign (the in-memory whitelist is the read path), but the behavior is unspecified. | `whitelist/repository/persisted.rs:29-54` | Confirm with a throwaway test in the whitelist record; at most a documentation or small fix issue. | TO CONFIRM |
| F5 | `ephemeral_sqlite_database()` (`packages/test-helpers/src/configuration.rs:85`) returns a bare `PathBuf` under `env::temp_dir()` with no cleanup owner. In this package it is called directly by `src/test_helpers.rs`, three other source files (`databases/driver/sqlite/mod.rs`, `sqlite/schema_migrator.rs`, `authentication/key/repository/persisted.rs`) and `tests/common/fixtures.rs`, and indirectly through `test_helpers::ephemeral_configuration` by most test modules that open a database, so every run leaves `data_*.db` files behind, the leak #2637 fixed for its own package in its S5. | Code reading, 2026-11-17 (review finding F1 of #2903); `grep -rn ephemeral_sqlite_database packages/tracker-core` | Q7 answered: fixed in this package with a `TempDir` guard owned by the test, in its own PR (split from S1 because #2907 planned the shared fix, which #2924 and #2925 (EPIC #2392) now take for the shared helper and the other packages). New tests never use the helper without a cleanup owner. | FIXED ([plan](test-refactor-plans/test-support-ephemeral-database-tests.md)): 0 files after a run, 72 before |

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`, `DEFERRED`.

| ID | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | DONE | Coverage evidence and complete module inventory | [coverage-evidence.md](coverage-evidence.md): baseline in four scopes, per-file uncovered groups, property candidacy, initial hypothesis for the 62 files. Delivered with this spec (S0). |
| T1b | DONE | Measure the container-gated scope | The `--all-features` run with `TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST` and `TORRUST_TRACKER_CORE_RUN_POSTGRES_DRIVER_TEST` set (images pulled with the maintainer's permission, 2026-11-17): lines 93.80%. Recorded in `coverage-evidence.md`; informs the scope of S3. |
| T2 | TODO | Shared plan guidance | `test-refactor-plans/README.md`: two-phase sequence, guardrails, toolchain-pinned commands. First test slice (S1). |
| T2b | IN_PROGRESS | Design-review checkpoint | After the first listener test and the first `TempDir`-owned SQLite test pass in S1, the maintainer reviews the fixture shape (see [Asynchronous Work, Cleanup and Test Fixtures](#asynchronous-work-cleanup-and-test-fixtures)) before other files reuse it. The SQLite half is done: the fixture shape of the [temporary-database fix](test-refactor-plans/test-support-ephemeral-database-tests.md) was approved on 2026-11-17; the listener half comes with S1. |
| T3 | TODO | Process every ledger row through the per-file workflow | Each row ends in exactly one of: test added, no-change with reason, deferred with owner. |
| T4 | TODO | Bounded mutation assessment | `mutation-evidence.md`: configuration, survivors, reason for each unviable mutant. Last slice. |
| T5 | TODO | Reconcile evidence and progress state | Ledger, records, slice table, acceptance rows and both EPIC coverage tables agree; grep for `status: proposed` and stray `TODO` labels in finished records. |
| T6 | TODO | Final verification, acceptance review, retrospective | Automatic checks, manual verification, acceptance review, `implementation-retrospective.md` (or a log entry saying none was needed). |

After the last test-producing increment, stop for maintainer review before T4-T6 and the final PR.

## Pull-Request Slices

Each slice is one PR, merged before the next slice branch is cut from `develop`. The slicing below
is the proposal for the maintainer's approval (questions Q5, Q6):

| Slice | Content | Ledger rows | Branch | PR | Status |
| --- | --- | --- | --- | --- | --- |
| S0 | This specification, the T1 evidence, and the EPIC registration with the baseline | - | `2902-1347-add-tests-tracker-core-spec` | TBD | IN_PROGRESS |
| S1 | T2, then `statistics/`: `repository.rs`, `metrics.rs`, `persisted/mod.rs`, `persisted/downloads.rs`, `event/listener.rs`, `event/handler.rs`, `mod.rs` (the only area reached just by the integration binary) | 1-7 | `2902-1347-add-tests-tracker-core-statistics` | TBD | TODO |
| S2 | The SQLite persistence boundary: `sqlite/{auth_key_store,torrent_metrics_store,whitelist_store,mod,schema_migrator}.rs`, `databases/{error,setup}.rs`, `error.rs`, plus the `databases/traits` and `driver/mod.rs` no-change decisions | 8-17 | `2902-1347-add-tests-tracker-core-sqlite` | TBD | TODO |
| S3 | MySQL and PostgreSQL drivers: assess after T1b; Docker-free error-path tests if non-flaky, otherwise no-change rows with the `db-compatibility` workflow as owner | 18-27 | `2902-1347-add-tests-tracker-core-server-drivers` | TBD | TODO |
| S4 | Authentication and whitelist: failing-`Database` propagation, `Key` property decision (F3, F4 checks) | 28-39 | `2902-1347-add-tests-tracker-core-auth-whitelist` | TBD | TODO |
| S5 | `torrent/`, `announce_handler.rs`, `scrape_handler.rs`, `container.rs`, `lib.rs`, `test_helpers.rs`, `peer_tests.rs`, then T4-T6; updates the EPIC tables; the spec moves to `docs/issues/closed/` in a separate `cleanup-completed-issues` PR after the issue closes | 40-45 | `2902-1347-add-tests-tracker-core-final` | TBD | TODO |

Rules: S0 to S4 use `Related to #2902`; only S5 uses `Closes #2902`. Do not stack branches; keep
at most one implementation PR open. Titles follow the repository convention, for example
`test(tracker-core): [#2902] cover the statistics counters and gauges`. Each PR updates the
ledger, the slice table, the slice measurement in `coverage-evidence.md` and the EPIC "Latest"
values. The tracker-wide manual scenarios (M1-M3) and the mutation run happen once, in S5.
Re-measure the baseline at the start of every slice, in all four scopes.

## Commit Points

| Task | Change set | Policy |
| --- | --- | --- |
| Spec (S0) | This specification, evidence, EPIC registration | One signed `docs(issues)` commit after approval, plus follow-ups recording the PR number and review fixes. |
| T3 | One source file: its decision record, tests and recorded coverage | One signed commit per completed file after the completed-file review. Never mix two source files. |
| Slice close | Measurement, EPIC update, records | One documentation commit per slice. |
| Review round | `docs/pr-reviews/pr-<PR>-review/PR-REVIEW.md` and fixes | Audit commit separate from fix commits, as `process-pr-review` requires. |

Conventional Commit messages, for example `test(tracker-core): cover the statistics counters`; all
commits GPG signed. Cite commits in issue-local evidence by their unique subject, never by SHA,
until merged. If work stops mid-task, record `interruption-note.md` (next action, validation state,
uncommitted-work location) and remove it on resume.

## Per-File Workflow

T3 applies to every row, one file at a time, in the ledger order within a slice.

Each file has one decision record `test-refactor-plans/<file>.md` of about 40 lines: current state
(unit-only coverage, existing tests, hypothesis); review of existing tests (`clean` or `refactor`
with smells); uncovered lines grouped by behavior, each classified **unit**, **collaborator-owned**,
**finding**, **container-gated** or **unselected** (with the answer to "hard to test, owned
elsewhere, or merely unselected?"); the property-test decision; the Prose-First AAA review of
every added or changed test; and results (before/after, mutation proofs). No-change rows share one
record.

Two maintainer gates per file: approve the decision record before changing tests, then review the
completed result. Intra-file steps proceed without stopping unless the record flags a risk.
Validation per file: focused `cargo test -p torrust-tracker-core`,
`cargo +nightly fmt --all -- --check`, `git diff --check`, and the pre-commit gate; then re-run
unit-only coverage and record before and after.

## Source File Ledger

States: `PENDING`, `IN_PROGRESS`, `BLOCKED`, `DONE`. Unit-only lines are the baseline measurement
(`--all-features --lib`). The rows cover every one of the 62 `.rs` files under `packages/tracker-core/src/`, each once
(declaration-only files are grouped with their neighbors, so there are fewer rows than files);
rows are ordered by slice, then by risk.

| Order | Source module | Slice | Unit-only lines | Initial hypothesis | State |
| ---: | --- | --- | ---: | --- | --- |
| 1 | `statistics/repository.rs` | S1 | 86 / 140 | Test candidate (four counter/gauge methods and error arms) | PENDING |
| 2 | `statistics/metrics.rs` | S1 | 11 / 45 | Test candidate | PENDING |
| 3 | `statistics/persisted/mod.rs` | S1 | 0 / 11 | Test candidate (`load_persisted_metrics`) | PENDING |
| 4 | `statistics/event/listener.rs` | S1 | 357 / 406 | Test candidate (spawn wrappers) | PENDING |
| 5 | `statistics/event/handler.rs` | S1 | 179 / 203 | Mostly logging-only; one error arm | PENDING |
| 6 | `statistics/persisted/downloads.rs` | S1 | 77 / 77 | Failing-`Database` case (line 133, `increase_global_downloads`) | PENDING |
| 7 | `statistics/mod.rs`, `statistics/event/mod.rs` | S1 | 40 / 40, no entries | Test candidate (`statistics/mod.rs:49`, usage statistics disabled); `event/mod.rs` declarations only | PENDING |
| 8 | `databases/driver/sqlite/auth_key_store.rs` | S2 | 29 / 50 | Test candidate | PENDING |
| 9 | `databases/driver/sqlite/torrent_metrics_store.rs` | S2 | 29 / 45 | Test candidate | PENDING |
| 10 | `databases/driver/sqlite/whitelist_store.rs` | S2 | 17 / 28 | Test candidate | PENDING |
| 11 | `databases/driver/sqlite/mod.rs` | S2 | 85 / 92 | Test candidate (aggregate-metric error branches) | PENDING |
| 12 | `databases/driver/sqlite/schema_migrator.rs` | S2 | 145 / 155 | No change likely; challenge each `?` | PENDING |
| 13 | `databases/error.rs` | S2 | 28 / 35 | Test candidate (migration-error mapping) | PENDING |
| 14 | `databases/setup.rs` | S2 | 34 / 44 | Test candidate (SQLite arm and unreachable endpoint); server arms to S3 | PENDING |
| 15 | `error.rs` | S2 | 38 / 50 | Test candidate (`From` conversions) | PENDING |
| 16 | `databases/driver/mod.rs` | S2 | 185 / 188 | No change (test-only retry helper) | PENDING |
| 17 | `databases/mod.rs`, `databases/traits/*` (6 files) | S2 | 2 / 2, no entries | No change (declarations and the mock) | PENDING |
| 18-22 | `databases/driver/mysql/{mod,auth_key_store,schema_migrator,torrent_metrics_store,whitelist_store}.rs` | S3 | 7 / 212, 0 / 50, 0 / 75, 0 / 45, 0 / 28 | Container-gated; assess after T1b | PENDING |
| 23-27 | `databases/driver/postgres/{mod,auth_key_store,schema_migrator,torrent_metrics_store,whitelist_store}.rs` | S3 | 7 / 189, 0 / 50, 0 / 6, 0 / 45, 0 / 28 | Container-gated; assess after T1b | PENDING |
| 28 | `authentication/handler.rs` | S4 | 371 / 371 | Failing-`Database` cases (two production `?` regions; seven assertion lines) | PENDING |
| 29 | `authentication/key/peer_key.rs` | S4 | 100 / 101 | `PartialEq` mixed-expiry arm; property candidate (`Key`); finding F3 check | PENDING |
| 30 | `authentication/key/mod.rs` | S4 | 77 / 77 | Property candidate (expiry bounds) | PENDING |
| 31 | `authentication/key/repository/persisted.rs` | S4 | 73 / 74 | Failing-`Database` cases | PENDING |
| 32 | `authentication/key/repository/in_memory.rs` | S4 | 92 / 92 | No change | PENDING |
| 33 | `authentication/service.rs` | S4 | 144 / 144 | No change; review breadth | PENDING |
| 34 | `authentication/mod.rs`, `authentication/key/repository/mod.rs` | S4 | 123 / 123, no entries | No change | PENDING |
| 35 | `whitelist/repository/persisted.rs` | S4 | 74 / 74 | Failing-`Database` cases; finding F4 check | PENDING |
| 36 | `whitelist/manager.rs` | S4 | 99 / 99 | Failing-`Database` cases | PENDING |
| 37 | `whitelist/authorization.rs` | S4 | 74 / 74 | No change (one test assertion line) | PENDING |
| 38 | `whitelist/repository/in_memory.rs` | S4 | 51 / 51 | No change | PENDING |
| 39 | `whitelist/{mod,setup,test_helpers}.rs`, `whitelist/repository/mod.rs` | S4 | 18 / 18, 7 / 7, 12 / 12, no entries | No change | PENDING |
| 40 | `torrent/repository/in_memory.rs` | S5 | 56 / 60 | Delegation of `get_torrent_peers` (owned by the registry) | PENDING |
| 41 | `torrent/manager.rs`, `torrent/services.rs`, `torrent/persisted_downloads.rs`, `torrent/mod.rs`, `torrent/repository/mod.rs` | S5 | 153 / 153, 267 / 267, 22 / 22, no entries | `manager.rs`: failing-`Database` case (line 66); the others no change, review breadth | PENDING |
| 42 | `announce_handler.rs` | S5 | 400 / 401 | Challenge the two `?` regions (one assertion line) | PENDING |
| 43 | `scrape_handler.rs` | S5 | 190 / 190 | No change (one assertion line) | PENDING |
| 44 | `container.rs` | S5 | 143 / 144 | Test candidate (`Persistence` error mapping) | PENDING |
| 45 | `lib.rs`, `peer_tests.rs`, `test_helpers.rs` | S5 | 58 / 58, 30 / 30, 109 / 123 | No change; test support classified | PENDING |

The first slice's record checks the grouping against `find packages/tracker-core/src -name '*.rs'`.

## Progress Tracking

### Workflow Checkpoints

- [x] Read EPIC #1347, the subissue guidelines, the `create-issue` workflow and the closed specs of
      #1348, #1349, #2637 and #2824.
- [x] Clean baseline coverage measured in four scopes and the complete module inventory recorded (T1).
- [ ] Specification and slicing reviewed and approved by the maintainer.
- [x] GitHub issue #2902 created. Linking it as a subissue of #1347 needs the maintainer (the bot account gets 404 from the sub-issues API).
- [ ] Spec-only PR (S0) merged before implementation.
- [x] Container-gated scope measured (T1b) with the maintainer's permission.
- [ ] Every ledger row `DONE` with a decision record, merged through its slice.
- [ ] Maintainer review after the final test-producing increment.
- [ ] Automatic verification with toolchain-qualified evidence.
- [ ] Manual verification in `manual-verification-evidence.md`.
- [ ] Bounded mutation assessment in `mutation-evidence.md`.
- [ ] Independent reviewer reports in `agent-review-reports.md` (if used).
- [ ] Acceptance criteria and implementation retrospective.
- [ ] Issue closed (by the S5 PR); spec moved to `docs/issues/closed/` (separate PR after the close).

### Progress Log

- 2026-11-17 10:57 UTC - Claude Code (Sonnet 5.5) - Created issue #2902; measured the baseline at `5833cdae9` in four scopes; drafted this specification and the coverage evidence; registered the subissue in EPIC #1347 (row 14). Open questions Q1-Q6 are for the maintainer.
- 2026-11-17 12:16 UTC - Claude Code (Sonnet 5.5) - The maintainer allowed the Docker image pull (Q1); T1b measured the container-gated scope: lines 93.80% (server drivers 58-96% per file). No review of #2903 arrived within 60 minutes.
- 2026-11-17 12:45 UTC - Claude Code (Opus 5.5) - Correction of the 12:16 UTC entry above: its last sentence is wrong as written. The poller I started right after the PR opened (11:05:26 UTC) timed out after its 60-minute wait with no review, but Cameron (`da2ce7`) had submitted a review of `690cfb5f5` with findings F1-F6 at 12:08:23 UTC, before that entry was written, and the six threads had no reply. A second review of `2b0c3fe36` (F7-F10) followed at 12:28:40 UTC. The ten findings are processed in `docs/pr-reviews/pr-2903-review/PR-REVIEW.md` (review finding F7 of #2903).
- 2026-11-17 18:04 UTC - Claude Code (Opus 5.5) - F5 fixed for this package in its own PR, split from S1 at the maintainer's request because #2907 (EPIC #2392) plans the same fix for the shared helper: a full `cargo test -p torrust-tracker-core --all-features` leaves 0 `data_*.db` files (72 before). The maintainer approved the shape (a struct with a private `TempDir`, owned by the test), the scope (all six sources in the package) and the completed change. S1 continues on its own branch.

## Open Design Questions

Each has a recommendation; the maintainer decides.

| ID | Question | Recommendation |
| --- | --- | --- |
| Q1 | May the agent pull the `mysql` and `postgres` Docker images (hundreds of MB) to measure the container-gated scope (T1b)? | ANSWERED: yes (maintainer, 2026-11-17); measured. |
| Q2 | The two database suites skip themselves silently (F1). Is the visible-skip fix in scope here? | No: separate issue; this one only records the scope separately. |
| Q3 | Scope of S3 for the MySQL and PostgreSQL drivers? | Docker-free unit tests only for what needs no server (an unreachable endpoint gives `ConnectionError`), if they are not flaky or slow; the happy path stays with the `db-compatibility` workflow, recorded as owner. |
| Q4 | Extract the duplicated `parse_valid_until` and row mapping (F2)? | Separate refactor issue after this one; tests per driver meanwhile. |
| Q5 | Is `quickcheck` acceptable for `Key` and the expiry bounds, as in #2824 and #1348? | Decide in the `peer_key.rs` and `key/mod.rs` records; add only if examples leave a surviving mutant. |
| Q6 | Is the six-slice plan (S0-S5) right, or should S4 and S5 merge? | Keep six: S4 and S5 are mostly failing-fake cases and no-change decisions; merging them mixes a risky area with the closing tasks. |
| Q7 | The temporary SQLite files the existing tests leave behind (F5): fix them here or hand them off? | ANSWERED (maintainer, 2026-11-17): fix here, package-local, with the package's helpers returning a `TempDir` guard with the configuration; the shared `test-helpers` function and the other packages are #2924 and #2925. Delivered in its own PR, split from S1 on the maintainer's request. |

Which EPIC row is the "unit-only" baseline? Recommendation: `--all-features --lib` (80.31%), the same
command as the other packages, with the container-gated figure reported separately in
`coverage-evidence.md` and not in the EPIC tables.

## Acceptance Criteria

- [ ] AC1: Complete per-file inventory with unit-only coverage, property candidacy and hypothesis.
- [ ] AC2: Every row ends in test-added, no-change-with-reason, or deferred-with-owner, with its
      decision record and recorded coverage.
- [ ] AC3: Clean, reproducible coverage evidence in all measured scopes with toolchain-qualified
      results.
- [ ] AC4: Every highest and high objective is protected by a unit test or has a recorded owner.
- [ ] AC5: No-change and deferral decisions enumerate their uncovered lines with per-group reasons.
- [ ] AC6: Each added or changed test has a recorded prose-first AAA review.
- [ ] AC7: Bounded mutation assessment completed, with no score target or CI gate.
- [ ] AC8: Reconciliation confirms records, evidence, slice table and both EPIC tables agree.
- [ ] AC9: Every finding needing a production change has an owner, a log entry and a `lessons.md`
      row, and no test pins its reported behavior without a recorded maintainer decision.
- [ ] AC10: Merged through the planned slices; only the last PR closes the issue.
- [ ] AC11: `cargo +nightly fmt --all -- --check`, `linter all` and
      `cargo test -p torrust-tracker-core` pass.
- [ ] AC12: Manual verification executed and recorded.

## Verification Plan

### Automatic Checks

- `cargo llvm-cov clean --workspace`, then the four commands of
  [coverage-evidence.md](coverage-evidence.md) (stable Rust), each writing to git-ignored `.tmp/`
- `cargo test -p torrust-tracker-core` (stable Rust)
- `cargo +nightly fmt --all -- --check` (nightly rustfmt; the bare stable command is not valid evidence)
- `cargo machete --with-metadata`
- `linter all`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=json`
- Pre-push checks before each slice PR.

Every evidence row names the toolchain that produced it.

### Manual Verification

Human-oriented interaction with the built tracker, run once in S5 with an isolated configuration
(automated tests never satisfy it):

| ID | Scenario | Steps | Expected | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Persisted downloads survive a restart | Start the tracker with SQLite persistence and the persistent completed statistic enabled; announce `completed` for a torrent; stop and start the tracker; scrape the torrent and read the stats endpoint. | The scrape `downloaded` count and the stats endpoint show the persisted value. | TODO | `manual-verification-evidence.md` (S5) |
| M2 | Private-mode keys | Start in private mode; create a key through the REST API; announce with the key and without it. | The announce with the key succeeds; without it, it is rejected with the authentication error. | TODO | `manual-verification-evidence.md` (S5) |
| M3 | Whitelist | Start in listed mode; announce an unlisted torrent; add it through the REST API; announce again. | The first announce is rejected as not whitelisted; the second succeeds. | TODO | `manual-verification-evidence.md` (S5) |

### Acceptance Verification

Reviewed against observed behavior after the last slice, before the issue is closed.

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1 | TODO | Inventory and T1 hypotheses in `coverage-evidence.md`. |
| AC2 | TODO | Ledger and `test-refactor-plans/`. |
| AC3 | TODO | `coverage-evidence.md`, final measurement. |
| AC4 | TODO | Records of the highest and high rows. |
| AC5 | TODO | No-change records. |
| AC6 | TODO | Prose-First AAA tables in the records. |
| AC7 | TODO | `mutation-evidence.md`. |
| AC8 | TODO | The T5 reconciliation. |
| AC9 | TODO | Findings Register and `lessons.md`. |
| AC10 | TODO | The merged slice PRs. |
| AC11 | TODO | Automatic checks of the last slice. |
| AC12 | TODO | `manual-verification-evidence.md`. |

## Risks and Trade-offs

- A green `cargo test --all-features` does not mean the MySQL and PostgreSQL suites ran (F1);
  mitigate by recording the container-gated scope separately and never claiming it from a green run.
- Types and `?` regions produce lines that count as uncovered but carry no decision; mitigate by
  testing behavior, not line totals.
- A test of a suspected defect would pin wrong behavior; mitigate with the findings procedure.
- Failing `Database` fakes can become large; mitigate by reusing the `testing` feature mock and one
  shared fake per slice, justified in its record.
- Tests that need a SQLite file can leak it: `ephemeral_sqlite_database()` has no cleanup owner
  (F5); mitigate by giving every new test a `tempfile::TempDir` it owns, added as a dev-dependency
  under `add-rust-dependency`.
- `develop` moves under the work; mitigate by re-measuring at every slice start.

## Implementation Completion Review

After implementation, compare the result with this specification and create
`implementation-retrospective.md` if there were material discoveries or deviations; otherwise add a
progress-log entry saying none was needed.

## References

- Parent EPIC: https://github.com/torrust/torrust-tracker/issues/1347
- Template sub-issues: #2824, #2637, #1349, #1348 (`docs/issues/closed/`)
- Package: `packages/tracker-core/`
