---
schema-version: 1
doc-type: issue
issue-type: bug
status: done
priority: p2
epic: 2392
github-issue: 2907
spec-path: docs/issues/closed/2907-2392-remove-leaked-test-temporary-files/ISSUE.md
branch: "2907-2392-remove-leaked-test-temporary-files"
related-pr: 2938
last-updated-utc: "2026-11-18 13:39"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - write-unit-test
  related-artifacts:
    - .claude/skills/fix-bug/SKILL.md
    - .claude/skills/create-issue/SKILL.md
    - docs/issues/open/2392-test-isolation/EPIC.md
    - docs/issues/closed/2394-2392-audit-shared-test-resources/inventory.md
    - docs/issues/open/2924-2392-sqlite-test-database-guard/ISSUE.md
    - docs/issues/open/2925-2392-migrate-tests-to-guarded-sqlite-database/ISSUE.md
    - packages/configuration/src/v2_0_0/mod.rs
    - packages/configuration/src/v3_0_0/mod.rs
    - packages/axum-server/src/tls.rs
---

<!-- skill-link: create-issue -->

# Issue #2907 - Remove the Temporary TOML and PEM Files Tests Leave Behind

Parent EPIC: #2392 - Test Isolation (`docs/issues/open/2392-test-isolation/EPIC.md`)

## Goal

The configuration-file tests and the TLS tests remove every temporary file they create, on
success, failure and drop paths (EPIC rule 1).

This issue was narrowed on 2026-11-17. It first covered three leaks; the SQLite database leak
turned out to be larger than one issue and moved to #2924 and #2925 (see Scope).

## Background

The shared-resource audit (#2394, finding F2 in its `inventory.md`) found three test helpers that
write files to `std::env::temp_dir()` with unique names and never remove them, or only when the
test passes. Two of them stay in this issue:

| Helper | File | Removed |
| --- | --- | --- |
| The configuration-file tests in `packages/configuration/src/{v2_0_0,v3_0_0}/mod.rs` | `test_config_<uuid>.toml` | Never |
| `make_temp_file` in `packages/axum-server/src/tls.rs` | `<prefix>-<nanoseconds>.pem` | Only when the test passes |

The third is out of scope here and moved to #2924 and #2925:
`torrust_tracker_test_helpers::configuration::ephemeral_sqlite_database()`, used by every
`ephemeral*()` configuration and the `tracker-core` SQLite tests. It leaves a
`data_<16 random characters>.db` file that is never removed.

The names are unique, so no two tests or runs collide; the problem is accumulation. One full
`cargo +stable test --workspace` run with a fresh `TMPDIR` leaves the files counted in this
folder's `manual-verification-evidence.md` (V1). On the maintainer's development machine on
2026-11-16, `/tmp` held 232,229 `data_*.db` files (about
10.6 GiB, all created since 2026-11-13) and 1,590 `test_config_*.toml` files. Several agents run
the full suite many times a day, so the directory grows until the disk or the inode table fills,
and then unrelated tests fail.

## Scope

### In Scope

- The configuration-file tests in `packages/configuration/src/{v2_0_0,v3_0_0}/mod.rs`: the
  `test_config_<uuid>.toml` file is removed when the test ends.
- `make_temp_file` in `packages/axum-server/src/tls.rs`: the PEM files are removed when the test
  fails too.

### Out of Scope

- The SQLite database leak (`data_*.db`). The maintainer split it out on 2026-11-17 (EPIC
  Decisions): `docs/issues/open/2924-2392-sqlite-test-database-guard/ISSUE.md` adds a guard-owned
  database, and `docs/issues/open/2925-2392-migrate-tests-to-guarded-sqlite-database/ISSUE.md` moves
  the tests to it and owns the check that a fresh `TMPDIR` is empty after the suite.
- Deleting the files already leaked on any machine (a one-off cleanup for the machine's owner).
- Examples that use fixed temporary paths; no test runs them (#2394 F7).
- Any other resource class.

## Architectural Decisions

- Related ADRs: none.
- ADRs to create: none expected.

## Design and Ownership Review

Both files are owned by the test that creates them, following patterns the workspace already
uses. The #2907 session proposed the designs below; the maintainer confirmed both in T2 on
2026-11-18.

- TOML: each test creates a `tempfile::tempdir()` and writes its configuration file inside it, as
  the tests in `src/bootstrap/config.rs` do.
- TLS: `make_temp_file` returns a `tempfile::NamedTempFile`, as the tests in
  `console/tracker-client/tests/tracker_checker/configuration.rs` do, so a failing test removes
  its files too.
- Both need `tempfile` as a dev-dependency of `torrust-tracker-configuration` and
  `torrust-tracker-axum-server`.
- Failure path: a test that panics still drops the guard.
- Deadline: not applicable, the change awaits no readiness operation.

### Test Development Loop

T3 and T4 change tests, so the work follows the progressive test-development loop of the `write-unit-test`
skill (`.claude/skills/write-unit-test/SKILL.md`):

- Work in the smallest behavior-focused increment that passes focused validation.
- After each passing increment, and before the next one starts, review its design with the
  mandatory prose-first Arrange-Act-Assert comparison: write temporary prose for each section,
  refactor until the code expresses it, and remove the redundant prose. Confirm that the test
  exposes the one causal initial-state difference, that its fixture owns only incidental mechanics
  (judged by named actions and abstraction level, not caller count), and that the production Act
  and the independently specified expected result stay visible.
- Record each review in `manual-verification-evidence.md` and commit the reviewed increment before
  starting the next one.

## Bug-Fix Process

The defect is in test-support code. Follow `.claude/skills/fix-bug/SKILL.md`:

1. Real-artifact reproduction: done while drafting with a fresh `TMPDIR` and recorded in this
   folder's `manual-verification-evidence.md` (V1); T1 repeats it on the `develop` the work starts
   from.
2. Regression boundary: a unit test of `make_temp_file` that asserts the file is gone after the
   guard is dropped. The TOML tests own their directory directly, so the recheck count covers
   them.
3. Red before the change, green after; final like-for-like recheck of step 1.

## Regression Test Strategy

One unit test for `make_temp_file`: create the file, drop the guard, assert the path no longer
exists. The step-1 count is the end-to-end recheck for both leaks.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Reproduce with a fresh `TMPDIR` | Before-counts recorded (V1): 2 `test_config_*.toml`, 0 PEM, 301 `data_*.db` at `develop` `61e3fedcf` (evidence file, "V1 repeated by T1"). |
| T2 | DONE | Confirm the two designs | The maintainer confirmed the TOML and TLS designs above on 2026-11-18. |
| T3 | DONE | Configuration-file tests | Files removed when the test ends (focused run: 1 file before, 0 after); test-design review recorded in the evidence file. |
| T4 | DONE | TLS test files | Red then green recorded for both maintained tests (the second red run after review finding F5 of PR 2938); test-design review recorded in the evidence file; the maintainer approved the tests of T3 and T4 on 2026-11-18. |
| T5 | DONE | Recheck | V2: 0 `test_config_*.toml`, 0 PEM, 4152 tests passed; `linter all` exit `0`. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1-T2 | Evidence and design | Documentation-only commit after maintainer review. |
| T3 | Configuration-file tests | Commit after focused validation and review. |
| T4 | TLS test files | Commit after focused validation and review. |
| T5 | Recheck evidence | Documentation-only commit. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2392-remove-leaked-test-temporary-files/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-16 17:30 UTC - Claude Code (Claude Opus 5.5), #2394 session - Drafted from audit finding
  F2 after the maintainer classified it as a new problem; counted the leaked files in `/tmp`.
- 2026-11-17 12:35 UTC - Claude Code (Claude Opus 5.5), EPIC #2392 coordinator - The
  maintainer approved the draft in the review of PR #2891. Created GitHub issue #2907 and
  moved the spec from `docs/issues/drafts/` to `docs/issues/open/`.
- 2026-11-17 18:03 UTC - Claude Code (Claude Fable 5.2), EPIC #2392 coordinator - Narrowed to the
  TOML and TLS leaks after the maintainer split the SQLite leak into two new issues (EPIC
  Decisions, 2026-11-17). The #2907 session's T1 and T2 findings showed about 100 tests need
  persistence and about 278 call sites use the `ephemeral*()` configurations. Renumbered the
  plan: the old T3 (SQLite) left, T4 to T6 became T3 to T5.
- 2026-11-17 18:39 UTC - Claude Code (Claude Fable 5.2), EPIC #2392 coordinator - The SQLite issues are
  #2924 and #2925. Updated this issue's title and body on GitHub to the narrowed scope.
- 2026-11-18 09:50 UTC - Claude Code (Claude Opus 5.5), #2907 session - Started implementation on
  branch `2907-2392-remove-leaked-test-temporary-files`, rebased on `develop` `90a7c7d3f`. T1 done:
  recorded the run of 2026-11-17 at `61e3fedcf` (2 TOML files, not 1: the `v2_0_0` and the `v3_0_0`
  test each leave one). Neither `packages/configuration` nor `packages/axum-server` changed between
  `61e3fedcf` and `90a7c7d3f`, so the result still holds.
- 2026-11-18 10:02 UTC - Claude Code (Claude Opus 5.5), #2907 session - T2 done: the maintainer
  confirmed both designs (TOML: a `tempfile::tempdir()` in each test; TLS: `make_temp_file` returns
  a `NamedTempFile`).
- 2026-11-18 10:06 UTC - Claude Code (Claude Opus 5.5), #2907 session - T3 done: both TOML tests
  write inside a `TempDir`. Found why the leaked-file count varies between runs: five `figment::Jail`
  tests in `packages/configuration/src/lib.rs` call `clear_env()`, which removes `TMPDIR` from the
  whole process while they run, so a parallel test can write to `/tmp` instead. Recorded in the
  evidence file and reported to the EPIC coordinator; out of this issue's scope.
- 2026-11-18 11:08 UTC - Claude Code (Claude Opus 5.5), #2907 session - T4 done: `make_temp_file`
  returns a `NamedTempFile`; the AC3 regression test was red against the old helper (it left
  `failing-test-<nanoseconds>.pem`) and is green now, with a new AC2 drop test. The maintainer
  reviewed and approved the tests of T3 and T4.
- 2026-11-18 11:11 UTC - Claude Code (Claude Opus 5.5), #2907 session - T5 done: V2 left 0
  `test_config_*.toml` and 0 PEM files (4152 tests passed), and `linter all` passed. Acceptance
  criteria reviewed against the evidence: AC1 to AC3 and the linter criterion pass. Completion
  review: no retrospective is needed. The design is the one the maintainer confirmed in T2, and the
  plan ran as written. The one discovery, the `figment::Jail::clear_env()` race that makes the
  leaked-file count a lower bound, is outside this issue: it is recorded in the evidence file and
  was reported to the EPIC coordinator, who owns the follow-up decision.
- 2026-11-18 11:19 UTC - Claude Code (Claude Opus 5.5), #2907 session - Rebased on a newer `develop`
  whose new commits change only Markdown files, then pushed the branch to the `josecelano-bot` fork.
  The pre-push hook passed (it ended with `cargo +nightly doc`, `cargo +stable package --workspace`
  and the full `cargo +stable test` run, 2026-11-18 11:16:22 to 11:18:25 UTC), so the automatic
  verification is complete. The independent Task Reviewer's report is in `agent-review-reports.md`.
- 2026-11-18 11:55 UTC - Claude Code (Claude Opus 5.5), #2907 session - Opened PR #2938. Processing
  its first reviews (audit: `docs/pr-reviews/pr-2938-review/PR-REVIEW.md`). Corrections to two
  entries above, found by review: the 10:06 entry says five `figment::Jail` tests call `clear_env()`;
  eighteen do (finding F4). The 11:19 entry says the new `develop` commits change only Markdown
  files; they also changed `.dockerignore` and `project-words.txt` and renamed six `.txt` evidence
  files of an archived spec, but no Rust or Cargo file (finding F6). Added a second red run, which
  covers both maintained TLS tests, to the evidence file (finding F5).
- 2026-11-18 13:39 UTC - Claude Code (Claude Opus 5.5), #2907 session - PR #2938 merged on 2026-11-18
  13:38 UTC as `develop` `bf178dd06`, and GitHub closed #2907. Archived this spec to
  `docs/issues/closed/` and marked order 7 DONE in the EPIC.

## Acceptance Criteria

- [x] AC1: After the full test suite with a fresh `TMPDIR`, no `test_config_*.toml` and no test
  PEM file remains.
- [x] AC2: `make_temp_file` has a unit test proving its file is removed when the guard is dropped.
- [x] AC3: A failing TLS test also removes its files.
- [x] `linter all` exits with code `0`

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| V1 | Reproduce | `TMPDIR=$(mktemp -d) cargo +stable test --tests --benches --examples --workspace --all-targets --all-features`, then count the files | Leaked files counted | DONE | `manual-verification-evidence.md` |
| V2 | Recheck | Same command after the change | No `test_config_*.toml` and no PEM file; `data_*.db` files remain until the SQLite issues are done | DONE | `manual-verification-evidence.md` |

Evidence policy: every recorded command result names the toolchain that produced it (for example
`cargo +stable test`), and the evidence file is created from
`docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | DONE | V2: 0 `test_config_*.toml` and 0 PEM files after the full suite |
| AC2 | DONE | `it_should_remove_the_temporary_file_when_its_guard_is_dropped` in `packages/axum-server/src/tls.rs` |
| AC3 | DONE | `it_should_remove_the_temporary_file_when_the_test_using_it_fails` in `packages/axum-server/src/tls.rs`, red against the old helper and green after (T4) |

## Risks and Trade-offs

- **Partial result.** After this issue the temporary directory still fills with `data_*.db`
  files, which are most of the leak (301 of 303 files in T1). The SQLite issues remove them.

## Implementation Completion Review

- Retrospective: `Not needed` (see the progress-log entry of 2026-11-18 11:11 UTC)
- If the work invalidates an assumption of this spec, changes its design materially, or finds
  something reusable, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this folder; otherwise add a progress-log
  entry that says why none was needed.
- An independent reviewer that receives this folder-style specification records its result in
  `agent-review-reports.md` (`docs/templates/AGENT-REVIEW-REPORTS.md`).

## References

- Parent EPIC: #2392
- Audit: #2394, `docs/issues/closed/2394-2392-audit-shared-test-resources/inventory.md` (F2)
