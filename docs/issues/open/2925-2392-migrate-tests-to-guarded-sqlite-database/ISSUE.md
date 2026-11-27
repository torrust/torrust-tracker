---
schema-version: 1
doc-type: issue
issue-type: bug
status: in-progress
priority: p2
epic: 2392
github-issue: 2925
spec-path: docs/issues/open/2925-2392-migrate-tests-to-guarded-sqlite-database/ISSUE.md
branch: "2925-migrate-tests-to-guarded-sqlite-database"
related-pr: null
last-updated-utc: "2026-11-27 11:13"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - write-unit-test
  related-artifacts:
    - .claude/skills/fix-bug/SKILL.md
    - .claude/skills/create-issue/SKILL.md
    - docs/issues/open/2392-test-isolation/EPIC.md
    - docs/issues/closed/2907-2392-remove-leaked-test-temporary-files/ISSUE.md
    - docs/issues/closed/2924-2392-sqlite-test-database-guard/ISSUE.md
    - packages/test-helpers/src/configuration.rs
---

<!-- skill-link: create-issue -->

# Issue #2925 - Move Tests to the Guard-Owned SQLite Database and Default to No Database

Parent EPIC: #2392 - Test Isolation (`docs/issues/open/2392-test-isolation/EPIC.md`)

## Goal

No test leaves a SQLite database behind: neither a `data_*.db` file nor a guard directory with
its `data.db`. Tests that need persistence use the guard-owned SQLite
database of #2924 (`docs/issues/closed/2924-2392-sqlite-test-database-guard/ISSUE.md`),
and the `ephemeral*()` configurations no longer configure a database. After this issue, a full
test run leaves no SQLite database in a fresh temporary directory; with #2907 also done, the
directory is empty.

## Background

Every `ephemeral*()` configuration of `torrust-tracker-test-helpers` sets a SQLite database at
`<temp dir>/data_<16 random characters>.db`, and nothing removes the file. #2907's T1 on `develop`
`61e3fedcf` counted 301 such files after one full run; about 256 come from these configurations
(#2907 T2 experiment, recorded in the #2924 spec's Background).

The maintainer decided (EPIC Decisions, 2026-11-17) that `ephemeral()` may default to no
database: `core.database` is an `Option`, and a tracker runs without persistence. Tests that need
persistence opt in through the factory of #2924 and keep its guard.

PR #2923 (#2902, open on 2026-11-17) fixes the leak inside `packages/tracker-core` with that
package's own guard, so this issue skips that package's tests once it merges.

Facts from the #2907 session, measured before PR #2923 and to be rechecked in T1:

- About 278 call sites of the `ephemeral*()` factories in 6 packages plus `src/`:
  `axum-rest-api-server` 75, `axum-http-server` 70, `http-core` 56, `udp-server` 23,
  `tracker-core` 14, `src/` 14, `axum-health-check-api-server` 9.
- With no database in `ephemeral()`, 100 tests failed, in these test binaries:
  `torrust-tracker --lib` (3, `app::tests`), `axum-http-server --lib` and `--test integration`,
  `axum-rest-api-server --lib` and `--test integration` (36 in `server::v1::contract`),
  `udp-server --lib`, `http-core --lib` and its `http_tracker_core_benchmark` bench, and
  `tracker-core --lib`.
- Some of them may fail only because a helper unwraps the persistence configuration, not because
  the test needs a database.

## Scope

### In Scope

- Classify the tests that fail without a database: the test needs persistence, or only a helper
  assumes it.
- Move the tests that need persistence to the factory of #2924, one pull request per
  package.
- Make helpers that only assume persistence work without it.
- Then remove the database from `ephemeral()` and the functions built on it, and remove
  `ephemeral_sqlite_database()`.
- Record the recheck that a fresh `TMPDIR` holds no SQLite database file and no database
  directory after the full suite and, with #2907 done, is empty.

### Out of Scope

- The guard type and the factory (#2924).
- The TOML and TLS leaks (#2907).
- Deleting the files already leaked on any machine.

## Architectural Decisions

- Related ADRs: none.
- ADRs to create: none expected.
- Decided by the maintainer (EPIC Decisions, 2026-11-17): no database by default; persistence is
  opt-in through a factory that returns a guard the test keeps.
- Consequence to record in T1: about 180 tests stop exercising the SQLite start-up path. List
  which tests still cover it, so the coverage that remains is a decision and not an accident.
- Decided in T1 with the maintainer's approval (record: https://github.com/torrust/torrust-tracker/pull/3067#issuecomment-7197208715): the `ephemeral*()`
  names stay, except `ephemeral_with_no_services()`, which becomes `ephemeral_without_services()`
  in T4; the pull-request order of the packages and the helper pattern are in the T1 decisions
  below. The coverage list (AC4) was approved too (see its section).

## Design and Ownership Review

- Ownership: each migrated test, or the fixture struct it uses, binds the guard for the whole
  test body, and drops it after the tracker.
- Restart tests (`packages/tracker-core/tests/integration.rs`) share one guard between their two
  environments.
- Failure path: a test that panics still drops the guard.
- Deadline: not applicable, the change awaits no readiness operation.
- Design-review checkpoint: after the first package's pull request, review the migration pattern
  with the maintainer before starting the next package.

### Test Development Loop

Follow the progressive test-development loop of the `write-unit-test` skill
(`.claude/skills/write-unit-test/SKILL.md`). A migrated test keeps its Arrange-Act-Assert shape:
the guard is incidental mechanics and belongs in the fixture unless persistence is the test's
causal initial state.

## Bug-Fix Process

Follow `.claude/skills/fix-bug/SKILL.md`:

1. Real-artifact reproduction: the full suite with a fresh `TMPDIR`, counting `data_*.db` (V1).
2. Regression boundary: the end-to-end count per package after its pull request, and the final
   full-suite count. The guard's own unit tests belong to #2924.
3. Like-for-like recheck of step 1 after the last pull request (V2).

## Regression Test Strategy

The full-suite count is the regression check. Add no test that scans the temporary directory
from inside the suite: it would race with other test processes on the same machine.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Reproduce and classify | Before-count (V1); table of failing tests with their class; package order, naming and remaining SQLite start-up coverage approved by the maintainer. |
| T2 | IN_PROGRESS | Migrate one package | First pull request (`udp-server`); migration pattern reviewed before T3. |
| T3 | TODO | Migrate the remaining packages | One pull request per package; each records its package count. |
| T4 | TODO | Default to no database | `ephemeral*()` set no database; `ephemeral_sqlite_database()` removed; `ephemeral_with_no_services()` renamed to `ephemeral_without_services()` in every caller. |
| T5 | TODO | Recheck | Fresh `TMPDIR` has no SQLite database file or directory after the suite (V2); `linter all` passes. |

## T1 Findings and Proposals

Measured on `develop` `f5329f758`, 2026-11-26 (details and the full list of failing tests in
`manual-verification-evidence.md`).

- **Before (V1):** 249 `data_*.db` files and no other entry in a fresh `TMPDIR` after the full suite.
- **Classification (V1b):** with `ephemeral()` set to no database, 77 tests in 7 test binaries
  and the `http_tracker_core_benchmark` bench fail. 48 tests need persistence; 29 only run a helper
  that unwraps the database configuration. `tracker-core` does not fail (its own guard wrappers).
  The 100 failures of the #2907 count dropped mostly because `tracker-core` is already migrated;
  the `axum-rest-api-server` count also moved (36 in `server::v1::contract` then, 31 now), because
  its test files changed between the two counts.

  | Package (test binary) | Failing | Need persistence | Helper only | Cause of the helper-only ones |
  | --- | --- | --- | --- | --- |
  | `udp-server` (`--lib`) | 25 | 1 (`scrape` persisted downloads) | 24 | `initialize_core_tracker_services` calls `initialize_database` |
  | `axum-http-server` (`--lib`) | 5 | 0 | 5 | the same helper in `v1/handlers/announce.rs` |
  | `axum-http-server` (`--test integration`) | 9 | 9 (private, listed, private and listed) | 0 | |
  | `axum-rest-api-server` (`--lib`) | 2 | 2 (routes that need keys or whitelist persistence) | 0 | |
  | `axum-rest-api-server` (`--test integration`) | 31 | 31 (`auth_key`, `whitelist`, one persisted metric) | 0 | |
  | `http-core` (`--lib`) | 2 | 2 (`HttpTrackerCoreContainer::initialize` composes persistence) | 0 | |
  | `http-core` (bench, 1 bench, not a test) | 1 bench | 1 bench (`benches/helpers/util.rs` initializes the database) | 0 | |
  | `torrust-tracker` (`--lib`) | 3 | 3 (two `app` tests set `persistent_torrent_completed_stat`; the third uses `ephemeral_private()` and expects persistence) | 0 | |
  | Total | 77 tests and 1 bench | 48 tests and 1 bench | 29 | |

- **`http-core` leaks:** the 6 files of the #2924 recheck come from the two container tests (1
  each) and from the benchmark (4); no other `http-core` test leaks one.
- **Not owned by this issue:** `tracker-core` keeps its own `EphemeralConfiguration` wrappers and
  no issue owns adopting the shared type there. Reported to the EPIC coordinator.

### Decisions approved by the maintainer

Record of the approvals, with the maintainer's words: https://github.com/torrust/torrust-tracker/pull/3067#issuecomment-7197208715.

- **Pull-request order (approved by the maintainer, 2026-11-26):** `udp-server` first (it shows both patterns: a helper that stops
  unwrapping, and one test that opts in), then `http-core`, `axum-http-server`,
  `axum-rest-api-server`, `torrust-tracker` (root `src/`), then the T4 default change. The
  design-review checkpoint of the spec happens after the `udp-server` pull request.
- **Naming (approved by the maintainer, 2026-11-26):** keep the `ephemeral*()` names, because
  without a database they describe the configuration more accurately (OS-assigned ports, nothing
  survives the test). Rename only `ephemeral_with_no_services()` to `ephemeral_without_services()`,
  to match `ephemeral_without_reverse_proxy()`; the rename goes in the T4 pull request.
- **Helper pattern (approved by the maintainer, 2026-11-26):** a helper that only assumed persistence initializes it only when
  `core.database` is set, so it works before and after T4. A test that needs persistence calls
  `ephemeral_with_sqlite_database()` (or a wrapper that takes the config) and keeps the guard in
  its fixture struct for the whole test body.

### SQLite start-up coverage that remains (AC4) - approved by the maintainer, 2026-11-26

After T4, only these tests open a SQLite database (derived from a search for `Database::Sqlite3`,
`ephemeral_with_sqlite_database` and a `sqlite3` driver in test configurations on `develop`
`f5329f758`; each package pull request re-checks its own part):

- the 48 migrated tests listed in the evidence appendix, which use the guard-owned database;
- the `tracker-core` tests and its restart tests (`packages/tracker-core/tests/integration.rs`),
  which use that package's own guard;
- tests that already set their own database: the `src/bootstrap/jobs/http_tracker.rs` test that
  builds its own directory, the `hung-component-tracker` integration tests, which write
  `driver = "sqlite3"` in their configuration file, and the main-application integration tests,
  which start the full application over a `driver = "sqlite3"` database in their own workspace
  (`tests/persistence/scrape_after_restart.rs`, which restarts on that database,
  `tests/persistence/completed_downloads_on_shutdown.rs`, `tests/common/configuration.rs`,
  `tests/metrics/fixed_ports.rs`, `tests/scaffold.rs`,
  `tests/lifecycle/fixture_shutdown_assertion.rs` and
  `tests/banning/udp_shared_connection_id_error_limit.rs`), and the main-application test
  binaries that start the tracker as a child process through the harness
  `tests/common/native_tracker/command.rs`, which also sets `driver = "sqlite3"`
  (`cli-configuration`, `lifecycle-shutdown-policy`, `lifecycle-signals` and
  `metrics-statistics-disabled-logs`).

These do not open a database, so they are not coverage: the `torrust-tracker-test-helpers`
factory tests (they check the guard directory and the configured path only), the
`src/bootstrap/persistence.rs` tests (they validate configuration requirements) and the
`src/container.rs` tests (they start from `Configuration::default()`, whose database is `None`).
The `http-core` container test that points at a missing directory
(`it_should_panic_when_the_configured_database_cannot_be_initialized`) covers the initialization
failure path, not an open.

The tests that stop opening a database are every test that used `ephemeral*()` without needing
persistence, in these packages: `udp-server` (the 24 helper-only tests and the rest of its
handler and server tests), `axum-http-server` (the 5 helper-only handler tests, which include
private and listed modes, and the public-mode contract tests), `axum-health-check-api-server`,
`axum-rest-api-server` (the `torrent`, `stats` and `health_check` contract contexts and the tests
built on `api_container`, which compose the core over `core_config.database`), the root `torrust-tracker` tests that use
`ephemeral*()`. The coverage of the production start-up path with a real database therefore
rests mainly on the main-application integration tests, which are the nearest coverage of it, and
on the other groups above. The maintainer approved this list (record: https://github.com/torrust/torrust-tracker/pull/3067#issuecomment-7198218832).

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Evidence and classification | Documentation-only commit after maintainer review. |
| T2-T3 | One package's tests and helpers | One pull request per package; commit after focused validation and review. |
| T4 | The default change and the removed helper | Own pull request, after every package is migrated. |
| T5 | Recheck evidence | Documentation-only commit. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2392-migrate-tests-to-guarded-sqlite-database/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-17 18:03 UTC - Claude Code (Claude Fable 5.2), EPIC #2392 coordinator - Drafted from the
  split of #2907 that the maintainer decided on 2026-11-17, using the #2907 session's T1 and T2
  findings.
- 2026-11-17 18:10 UTC - Claude Code (Claude Fable 5.2), EPIC #2392 coordinator - Noted that
  PR #2923 (#2902) fixes `tracker-core`, so T1 rechecks the counts and this issue skips that
  package.
- 2026-11-17 18:39 UTC - Claude Code (Claude Fable 5.2), EPIC #2392 coordinator - The maintainer approved
  the draft. Created GitHub issue #2925 and moved the spec from `docs/issues/drafts/` to
  `docs/issues/open/`. The package order, naming and remaining SQLite coverage stay open until T1.
- 2026-11-17 21:12 UTC - Claude Code (Claude Fable 5.2), EPIC #2392 coordinator - Review of PR #2926:
  the goal, AC1, T5 and V2 now also exclude a leftover guard directory, because the guard keeps
  the database as `data.db` inside a directory and a check for `data_*.db` alone would miss it.

- 2026-11-26 08:35 UTC - Claude Code (Claude Sonnet 5.5), #2925 implementer - T1: recorded V1 (249 files),
  the no-database classification (77 failing tests, 48 need persistence) and the per-test
  attribution of the 6 `http-core` files. Proposed the package order and kept names. Waiting for
  the maintainer's approval (relayed by the EPIC coordinator) before T2.
- 2026-11-26 13:59 UTC - Claude Code (Claude Sonnet 5.5), #2925 implementer - The maintainer, in this
  session, agreed with the problem and the solution and approved the naming (record: https://github.com/torrust/torrust-tracker/pull/3067#issuecomment-7197208715): keep `ephemeral*()`,
  rename only `ephemeral_with_no_services()` to `ephemeral_without_services()` in the T4 pull
  request. The package order and the helper pattern were explained to him in the same exchange.
- 2026-11-26 14:09 UTC - Claude Code (Claude Sonnet 5.5), #2925 implementer - The maintainer approved the
  package order and the helper pattern ("Approve both", relayed by the EPIC coordinator). T1 is
  done; T2 starts with `udp-server` after this pull request merges.
- 2026-11-26 16:05 UTC - Claude Code (Claude Sonnet 5.5), #2925 implementer - Correction of the entry above after
  review: T1 is not done. Its expected output includes the maintainer's approval of the remaining
  SQLite start-up coverage (AC4), the list was wrong (three groups never open a database) and is
  corrected now, and no approval of it exists yet. T1 is back to `IN_PROGRESS`.
- 2026-11-26 16:12 UTC - Claude Code (Claude Sonnet 5.5), #2925 implementer - Correction of the
  2026-11-26 08:35 UTC entry above after review: that stamp is an estimate and is later than commit
  `docs(issues): [#2925] record the T1 reproduction and the classification of tests that need
  persistence` (2026-11-26T08:09:21Z), which carries the entry, so the work it records was done
  before 08:09 UTC. The entry is not rewritten.
- 2026-11-26 16:53 UTC - Claude Code (Claude Sonnet 5.5), #2925 implementer - The maintainer approved the
  corrected coverage list (AC4) ("Approve it", relayed by the EPIC coordinator; record: https://github.com/torrust/torrust-tracker/pull/3067#issuecomment-7198218832).
  The list he approved was the summary in that record; the main-application integration tests
  named in the spec widen it. All T1 outputs are approved: T1 is `DONE`, and T2 starts with
  `udp-server` after this pull request merges.
- 2026-11-26 18:50 UTC - Claude Code (Claude Sonnet 5.5), #2925 implementer - T2 started with `udp-server`
  (pull request pending). Pattern: the test helper `initialize_core_tracker_services` initializes
  persistence only when `core.database` is set (it was unconditional); the one test that needs
  persistence (`public_with_persistent_completed_statistics` in `handlers/scrape.rs`) takes its
  configuration from `ephemeral_with_sqlite_database()` and hands the guard to the services struct
  (`CoreTrackerServices.sqlite_database_guard`, declared last so it drops after the stores).
  Evidence in `manual-verification-evidence.md`, section "T2: udp-server".
- 2026-11-26 19:04 UTC - Claude Code (Claude Sonnet 5.5), #2925 implementer - Correction of the entry above
  after a review of PR 3082: the guard is not part of `CoreTrackerServices`. A fixture drops its
  fields in declaration order, and `CoreTrackerServices` drops before `CoreUdpTrackerServices`,
  which also holds the database. The guard now lives in the test fixture (`Tracker` in
  `handlers/scrape.rs`), declared after both service bundles. The migration pattern for the next
  packages is: the fixture struct owns the guard as its last field.
- 2026-11-27 11:13 UTC - Claude Code (Claude Sonnet 5.5), #2925 implementer - Correction after review of PR 3082:
  `public_with_persistent_completed_statistics` is the constructor of the `Tracker` fixture, not
  a test. The test that needs persistence is
  `it_should_return_the_persisted_downloads_of_torrents_absent_from_memory_in_request_order`. The
  T2 evidence was rerun at the final head with stamps (evidence file, section "T2: udp-server").

## Acceptance Criteria

- [ ] AC1: After the full test suite with a fresh `TMPDIR`, no SQLite database remains:
  no `data_*.db` file and no leftover guard directory, with or without its `data.db` file.
- [ ] AC2: The `ephemeral*()` configurations set no database, `ephemeral_sqlite_database()`
  no longer exists, and `ephemeral_with_no_services()` is renamed `ephemeral_without_services()`.
- [ ] AC3: Every test that needs persistence gets its database from the guard-owned factory.
- [ ] AC4: The spec lists the tests that still cover the SQLite start-up path.
- [ ] AC5: With #2907 done, the fresh `TMPDIR` is empty after the suite.
- [ ] `linter all` exits with code `0`

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| V1 | Reproduce | `TMPDIR=$(mktemp -d) cargo +stable test --tests --benches --examples --workspace --all-targets --all-features`, then count the files | Leaked files counted | DONE (249) | `manual-verification-evidence.md` |
| V2 | Recheck | Same command after T4 | No `data_*.db` file and no leftover guard directory, with or without its `data.db` file; no entry at all once #2907 is done | TODO | `manual-verification-evidence.md` |

Evidence policy: every recorded command result names the toolchain that produced it, and the
evidence file is created from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | V2 |
| AC2 | TODO | Code review |
| AC3 | TODO | T1 classification table, updated per pull request |
| AC4 | TODO | Spec section added in T1 |
| AC5 | TODO | V2 |

## Risks and Trade-offs

- **Lost coverage.** Tests that used a database only by default stop exercising SQLite start-up.
  T1 makes the remaining coverage explicit.
- **Many pull requests.** One per package keeps each reviewable; the cost is a longer delivery.
  The old and new ways coexist until T4.
- **Hidden dependence on persistence.** A helper may unwrap the database configuration. T1's
  classification finds these before any default changes.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If the work invalidates an assumption of this spec, changes its design materially, or finds
  something reusable, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this folder; otherwise add a progress-log
  entry that says why none was needed.
- An independent reviewer that receives this folder-style specification records its result in
  `agent-review-reports.md` (`docs/templates/AGENT-REVIEW-REPORTS.md`).

## References

- Parent EPIC: #2392
- Depends on: #2924, `docs/issues/closed/2924-2392-sqlite-test-database-guard/ISSUE.md`
- Found by: #2907 (T1 and T2), #2394 finding F2
- Related: PR #2923 (#2902), the same fix inside `tracker-core`
