---
schema-version: 1
doc-type: issue
issue-type: bug
status: draft
priority: p2
epic: 2392
github-issue: null
spec-path: docs/issues/drafts/2392-migrate-tests-to-guarded-sqlite-database/ISSUE.md
branch: "{issue-number}-2392-migrate-tests-to-guarded-sqlite-database"
related-pr: null
last-updated-utc: "2026-11-17 18:03"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - write-unit-test
  related-artifacts:
    - .claude/skills/fix-bug/SKILL.md
    - .claude/skills/create-issue/SKILL.md
    - docs/issues/open/2392-test-isolation/EPIC.md
    - docs/issues/open/2907-2392-remove-leaked-test-temporary-files/ISSUE.md
    - docs/issues/drafts/2392-sqlite-test-database-guard/ISSUE.md
    - packages/test-helpers/src/configuration.rs
---

<!-- skill-link: create-issue -->

# Move Tests to the Guard-Owned SQLite Database and Default to No Database

Parent EPIC: #2392 - Test Isolation (`docs/issues/open/2392-test-isolation/EPIC.md`)

## Goal

No test leaves a `data_*.db` file behind. Tests that need persistence use the guard-owned SQLite
database of the foundation issue (`docs/issues/drafts/2392-sqlite-test-database-guard/ISSUE.md`),
and the `ephemeral*()` configurations no longer configure a database. After this issue, a full
test run leaves a fresh temporary directory empty.

## Background

Every `ephemeral*()` configuration of `torrust-tracker-test-helpers` sets a SQLite database at
`<temp dir>/data_<16 random characters>.db`, and nothing removes the file. #2907's T1 on `develop`
`61e3fedcf` counted 301 such files after one full run; about 256 come from these configurations
(#2907 T2 experiment, recorded in the foundation issue's Background).

The maintainer decided (EPIC Decisions, 2026-11-17) that `ephemeral()` may default to no
database: `core.database` is an `Option`, and a tracker runs without persistence. Tests that need
persistence opt in through the foundation issue's factory and keep its guard.

Facts from the #2907 session, to be rechecked in T1:

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
- Move the tests that need persistence to the foundation issue's factory, one pull request per
  package.
- Make helpers that only assume persistence work without it.
- Then remove the database from `ephemeral()` and the functions built on it, and remove
  `ephemeral_sqlite_database()`.
- Record the recheck that a fresh `TMPDIR` holds no `data_*.db` after the full suite and, with
  #2907 done, is empty.

### Out of Scope

- The guard type and the factory (foundation issue).
- The TOML and TLS leaks (#2907).
- Deleting the files already leaked on any machine.

## Architectural Decisions

- Related ADRs: none.
- ADRs to create: none expected.
- Decided by the maintainer (EPIC Decisions, 2026-11-17): no database by default; persistence is
  opt-in through a factory that returns a guard the test keeps.
- Consequence to record in T1: about 180 tests stop exercising the SQLite start-up path. List
  which tests still cover it, so the coverage that remains is a decision and not an accident.
- Open, decided in T1 with the maintainer's approval: whether the `ephemeral*()` functions are
  renamed when they lose the database, and the pull-request order of the packages.

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
   full-suite count. The guard's own unit tests belong to the foundation issue.
3. Like-for-like recheck of step 1 after the last pull request (V2).

## Regression Test Strategy

The full-suite count is the regression check. Add no test that scans the temporary directory
from inside the suite: it would race with other test processes on the same machine.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Reproduce and classify | Before-count (V1); table of failing tests with their class; package order, naming and remaining SQLite start-up coverage approved by the maintainer. |
| T2 | TODO | Migrate one package | First pull request; migration pattern reviewed before T3. |
| T3 | TODO | Migrate the remaining packages | One pull request per package; each records its package count. |
| T4 | TODO | Default to no database | `ephemeral*()` set no database; `ephemeral_sqlite_database()` removed. |
| T5 | TODO | Recheck | Fresh `TMPDIR` has no `data_*.db` after the suite (V2); `linter all` passes. |

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
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
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

## Acceptance Criteria

- [ ] AC1: After the full test suite with a fresh `TMPDIR`, no `data_*.db` file remains.
- [ ] AC2: The `ephemeral*()` configurations set no database, and `ephemeral_sqlite_database()`
  no longer exists.
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
| V1 | Reproduce | `TMPDIR=$(mktemp -d) cargo +stable test --tests --benches --examples --workspace --all-targets --all-features`, then count the files | Leaked files counted | TODO | `manual-verification-evidence.md` |
| V2 | Recheck | Same command after T4 | No `data_*.db`; no entry at all once #2907 is done | TODO | `manual-verification-evidence.md` |

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
- Depends on: the foundation issue, `docs/issues/drafts/2392-sqlite-test-database-guard/ISSUE.md`
- Found by: #2907 (T1 and T2), #2394 finding F2
