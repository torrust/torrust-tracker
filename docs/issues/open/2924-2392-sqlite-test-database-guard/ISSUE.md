---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p2
epic: 2392
github-issue: 2924
spec-path: docs/issues/open/2924-2392-sqlite-test-database-guard/ISSUE.md
branch: "2924-2392-sqlite-test-database-guard"
related-pr: null
last-updated-utc: "2026-11-23 18:38"
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
    - docs/issues/open/2925-2392-migrate-tests-to-guarded-sqlite-database/ISSUE.md
    - docs/issues/closed/2394-2392-audit-shared-test-resources/inventory.md
    - packages/test-helpers/src/configuration.rs
    - packages/axum-rest-api-server/src/test_support.rs
---

<!-- skill-link: create-issue -->

# Issue #2924 - Add a Guard-Owned SQLite Test Database

Parent EPIC: #2392 - Test Isolation (`docs/issues/open/2392-test-isolation/EPIC.md`)

## Goal

A test that needs persistence gets its SQLite database from a factory that also returns a guard. The
guard owns the database's directory and removes it when the guard is dropped, so the directory goes
whether the test passes or fails (EPIC rule 1). This issue adds the guard and the factory next to
the existing functions and converts the helpers outside `tracker-core` that build the database path
themselves. It is the foundation for the migration issue, #2925
(`docs/issues/open/2925-2392-migrate-tests-to-guarded-sqlite-database/ISSUE.md`).

## Background

`torrust_tracker_test_helpers::configuration::ephemeral_sqlite_database()` only builds a path,
`<temp dir>/data_<16 random characters>.db`. The tracker creates the file when it first opens the
database, and nothing removes it. #2907 found the leak (#2394 finding F2); its T1 on `develop`
`61e3fedcf` counted 301 `data_*.db` files after one full test run in a fresh `TMPDIR`.

The T2 of #2907 showed the fix is larger than one issue, and the maintainer split it (EPIC
Decisions, 2026-11-17):

- An experiment set `config.core.database = None` in `ephemeral()`. The full suite then left 45
  files instead of 301, and 100 tests failed. So about 256 files come from the `ephemeral*()`
  configurations, and about 100 tests need persistence.
- The other 45 files come from helpers that call `ephemeral_sqlite_database()` directly:
  `packages/tracker-core/src/test_helpers.rs`, `packages/tracker-core/tests/common/fixtures.rs`,
  `packages/tracker-core/src/databases/driver/sqlite/mod.rs`,
  `packages/tracker-core/src/databases/driver/sqlite/schema_migrator.rs`,
  `packages/tracker-core/src/authentication/key/repository/persisted.rs` and
  `packages/http-core/src/container.rs`.
- Some tests start a second environment on the same database to simulate a restart
  (`packages/tracker-core/tests/integration.rs`). A guard owned by one environment would remove
  the database under the second one, so the guard must be independent of the environment.

How the workspace already keeps a `tempfile` guard:

| Who keeps the guard | Example |
| --- | --- |
| A struct, next to the resource it serves | `TestApiContainer` in `packages/axum-rest-api-server/src/test_support.rs` |
| The test, with a `TempDir` local | `src/bootstrap/config.rs`, `src/bootstrap/jobs/http_tracker.rs` |
| The test, with a `NamedTempFile` local | `console/tracker-client/tests/tracker_checker/configuration.rs` |
| Returned next to a configuration | none on `develop`; PR #2923 adds it for `tracker-core` |

PR #2923 (#2902, open on 2026-11-17) already fixes this leak inside `packages/tracker-core`: its
tests go from 72 `data_*.db` files per run to 0. It leaves the shared test-helpers function and
the other packages unchanged. Its shape, approved by the maintainer at #2902's design review, is
a struct `EphemeralConfiguration { pub config: Core, _database_directory: TempDir }` that the
test owns; the database is `data.db` inside the directory, so a guard dropped too early makes
SQLite fail to open the file instead of leaking. This issue reuses that shape for the shared
helpers and starts after PR #2923 merges.

## Scope

### In Scope

- A guard type in `torrust-tracker-test-helpers` that owns a `tempfile::TempDir` holding the
  SQLite database file and exposes the database path.
- A factory, next to the existing `ephemeral*()` functions, that returns a configuration with
  SQLite persistence together with its guard. The test binds the guard for its whole body.
- Convert the helpers outside `tracker-core` that call `ephemeral_sqlite_database()` directly
  (`packages/http-core/src/container.rs`, and any other that T2 finds) to the guard. The
  `tracker-core` ones are converted by PR #2923.
- Unit tests of the guard: the directory is gone after the guard is dropped, and a connection
  opened after the drop fails.

### Out of Scope

- Changing `ephemeral()` or the other existing `ephemeral*()` functions; their callers move in
  #2925.
- Removing `ephemeral_sqlite_database()`; #2925 removes it after its last caller.
- The TOML and TLS leaks (#2907).

## Architectural Decisions

- Related ADRs: none.
- ADRs to create: none expected.
- Decided by the maintainer (EPIC Decisions, 2026-11-17): persistence in tests is opt-in through
  a factory that returns a guard the test keeps; the guard is a `TempDir`, independent of the
  test environment.
- Starting point: the wrapper struct of PR #2923, which keeps the configuration and its
  `TempDir` together. T1 decides, with the maintainer's approval, the names of the shared type
  and factory and whether `tracker-core` then uses the shared type instead of its own. The
  maintainer's suggestion for the no-database side is a name like
  `ephemeral_without_persistence`; naming is open.

## Design and Ownership Review

- Ownership: the test owns the guard. The guard is dropped after the tracker and its database
  handles, so declare it before them or keep it in the fixture struct that owns them.
- Failure path: a test that panics still drops the guard.
- Fail closed: a guard dropped early removes the directory, so a new connection fails instead of
  leaking a file. SQLite on Linux keeps working on an unlinked file through an open handle, which
  can hide an early drop; the unit tests check a connection opened after the drop.
- Restart tests: two environments may share one guard, because the guard does not belong to
  either.
- Deadline: not applicable, the change awaits no readiness operation.
- Design-review checkpoint: after the guard, the factory and the first converted helper pass,
  stop for a design review before converting the other helpers.

### Test Development Loop

Follow the progressive test-development loop of the `write-unit-test` skill
(`.claude/skills/write-unit-test/SKILL.md`): work in the smallest behavior-focused increment,
review each passing increment with the prose-first Arrange-Act-Assert comparison, record each
review in `manual-verification-evidence.md` and commit the reviewed increment before the next.

### T1 Decision (approved by the maintainer, 2026-11-23)

- Guard type: `SqliteDatabaseGuard` in `torrust-tracker-test-helpers`
  (`packages/test-helpers/src/configuration.rs`). It owns a `tempfile::TempDir` and exposes
  `path()` (`<dir>/data.db`). "Guard" is the Rust RAII idiom (like `MutexGuard`): the value releases
  the resource when dropped. It is not named `SqliteTempDir` because it creates the directory and
  is also responsible for removing it. The type's doc comment records this reasoning.
- Factory: `ephemeral_with_sqlite_database() -> (Configuration, SqliteDatabaseGuard)`. The tuple
  makes the test bind the guard visibly (`let (config, _guard) = ...`, never a bare `_`).
- Existing `ephemeral*()` functions keep their signatures (AC4).
- `tracker-core` keeps its own wrappers. No issue owns their adoption of the shared type yet: the
  spec of #2925 skips `tracker-core` and does not list it, so the maintainer decides later whether
  to add it to #2925 or open an issue.
- The `http-core` caller is the test `it_should_panic_when_the_configured_database_cannot_be_initialized`
  (`packages/http-core/src/container.rs`). It converts to a guard-owned directory with a missing
  sub-directory, so the test still gets a path whose parent does not exist.
- T2 finding: that `http-core` test creates no file today, so the conversion cannot lower the
  leak count; it removes the last outside caller of `ephemeral_sqlite_database()` (AC3).

## Bug-Fix Process

The defect is in test-support code. Follow `.claude/skills/fix-bug/SKILL.md`:

1. Real-artifact reproduction: count the `data_*.db` files the `http-core` test binaries leave
   in a fresh `TMPDIR` (V1).
2. Regression boundary: unit tests of the guard.
3. Red before the change, green after; like-for-like recheck of step 1 (V2).

## Regression Test Strategy

Unit tests in `torrust-tracker-test-helpers`: create a database through the factory, open it,
drop the handles and the guard, assert the directory no longer exists. The V1 and V2 counts are
the end-to-end check for the converted helpers.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Decide the names and the return shape | Recorded in the T1 Decision section above; approved by the maintainer. |
| T2 | DONE | Reproduce for the direct users | Before-count recorded (V1): 6 files; the `http-core` direct caller creates none. |
| T3 | TODO | Guard type and factory | Unit tests red then green; design review recorded. |
| T4 | TODO | Convert the direct users | The direct users outside `tracker-core` use the guard; test-design review recorded. |
| T5 | TODO | Recheck | Direct users leave no `data_*.db` (V2); `linter all` passes. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1-T2 | Design and evidence | Documentation-only commit after maintainer review. |
| T3 | Guard, factory and their tests | Commit after focused validation and review. |
| T4 | Converted helpers | Commit after focused validation and review. |
| T5 | Recheck evidence | Documentation-only commit. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2392-sqlite-test-database-guard/ISSUE.md`
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
- 2026-11-17 18:10 UTC - Claude Code (Claude Fable 5.2), EPIC #2392 coordinator - PR #2923 (#2902)
  fixes the leak inside `tracker-core`. Removed that package's helpers from the scope, made its
  approved wrapper struct the starting point, and made this issue start after it merges.
- 2026-11-17 18:39 UTC - Claude Code (Claude Fable 5.2), EPIC #2392 coordinator - The maintainer approved
  the draft. Created GitHub issue #2924 and moved the spec from `docs/issues/drafts/` to
  `docs/issues/open/`. The names and the return shape stay open until T1.

- 2026-11-23 16:50 UTC - Claude Code (Claude Sonnet 5.5), #2924 implementation session - T1 approved by
  the maintainer (names, tuple return shape, `tracker-core` unchanged). T2: V1 counted 6 leaked files
  for `http-core`, none from the direct caller. Evidence in `manual-verification-evidence.md`.

## Acceptance Criteria

- [ ] AC1: `torrust-tracker-test-helpers` has a guard type and a factory that returns a
  configuration with SQLite persistence together with its guard.
- [ ] AC2: Unit tests prove the database directory is removed when the guard is dropped.
- [ ] AC3: No code outside `torrust-tracker-test-helpers` calls `ephemeral_sqlite_database()`.
  Inside it, only the existing `ephemeral*()` functions call it, until #2925 removes it.
- [ ] AC4: The existing `ephemeral*()` functions keep their signatures, so no other package
  changes.
- [ ] `linter all` exits with code `0`

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| V1 | Reproduce | `export TMPDIR=$(mktemp -d)`, then `cargo +stable test -p torrust-tracker-http-core --all-targets --all-features`, then count the `data_*.db` files in `$TMPDIR` | Leaked files counted | DONE (6 files) | `manual-verification-evidence.md` |
| V2 | Recheck | The same three steps after the change | Only `data_*.db` files created through the `ephemeral*()` configurations remain, counted and listed by test | TODO | `manual-verification-evidence.md` |

Evidence policy: every recorded command result names the toolchain that produced it, and the
evidence file is created from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | Code review |
| AC2 | TODO | Unit tests |
| AC3 | TODO | `grep` for the function's callers |
| AC4 | TODO | `git diff --stat` shows no other package |

## Risks and Trade-offs

- **Two ways for a while.** Until #2925 ends, the old path helper and the guard
  coexist. That is the price of small pull requests: changing `ephemeral()` in place would break
  every package in one pull request.
- **Early drop.** A guard dropped before the tracker stops deletes an open database. The unit
  tests check the order, and the design review checks each converted helper.

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
- Found by: #2907 (T1 and T2), #2394 finding F2
- Related: PR #2923 (#2902), the same fix inside `tracker-core`
