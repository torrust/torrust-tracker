---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p2
epic: null
github-issue: 2953
spec-path: docs/issues/open/2953-fix-persisted-downloads-integration-test-race/ISSUE.md
branch: "2953-fix-persisted-downloads-integration-test-race-spec"
related-pr: null
last-updated-utc: "2026-11-18 18:50"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .claude/skills/fix-bug/SKILL.md
    - .claude/skills/create-issue/SKILL.md
    - packages/tracker-core/tests/integration.rs
    - packages/tracker-core/tests/common/test_env.rs
    - packages/tracker-core/src/statistics/event/handler.rs
---

<!-- skill-link: create-issue -->

# Issue #2953 - Fix the race in the persisted global downloads integration test

## Goal

Make the tracker-core integration test
`it_should_persist_the_global_number_of_completed_peers_into_the_database` deterministic: it must
wait, with a bounded timeout, until the in-memory persisted downloads counter has been updated
before it asserts on it.

## Background

### What happens

1. The test announces a peer as `started` and then `completed`. The swarm coordination registry
   publishes a `PeerDownloadCompleted` event.
2. A background event listener handles the event in
   `packages/tracker-core/src/statistics/event/handler.rs` (`increase_global_downloads`). It
   awaits the database write of the global downloads count first, and only after that write
   returns it increments the in-memory counter
   `tracker_core_persisted_torrents_downloads_total`.
3. The test calls `TestEnv::wait_for_global_downloads_persisted(1)`
   (`packages/tracker-core/tests/common/test_env.rs`). This helper polls only the **database**
   value and returns as soon as it reads `1`.
4. The test then asserts (`packages/tracker-core/tests/integration.rs`, line 253 on `develop`)
   that `stats_repository.get_torrents_downloads_persisted_total()` is `1`.

Between steps 2 and 3 there is a short window: the database already holds `1`, but the listener
task has not yet run the counter increment. If the test reads the counter in that window, it reads
`0` and fails with `left: 0, right: 1`.

### Which expectation it violates

The helper's name and its use before the assertion suggest that "persisted" covers the whole
effect of the listener. It only covers the database half. The test is flaky, which breaks the
expectation that a test on `develop` passes on every run.

### Impact

- The production code is correct. The counter is meant to follow the database write: it counts
  downloads that were actually persisted. Nothing observable by tracker users is wrong.
- The flaky test blocks merges. It failed once during the maintainer's merge-script pre-push run of
  all tests on `develop`, while merging PR 2946, a docs-only PR. The merge had to be re-run.
- It is rare: 40 isolated runs (reported by the session that found it) and 400 further runs of the
  unmodified test binary passed. See `manual-verification-evidence.md`.

### Other callers

A search for `wait_for_global_downloads_persisted` and `get_torrents_downloads_persisted_total`
found no other test with this race:

- `wait_for_global_downloads_persisted` has one caller, the test above.
- The unit tests in `statistics/event/handler.rs` and `statistics/event/listener.rs` that read the
  counter run the handler (or the listener, with a scripted receiver) directly and await it, so
  their reads happen after the increment.
- `it_should_reset_in_session_completed_downloads_after_a_persistence_free_restart` runs without a
  database and expects the persisted counter to be `0`.
- The second half of the affected test reads the counter in a restarted `TestEnv`. That value is
  loaded from the database during `TestEnv::start` before the test reads it, so it has no race once
  the first half waits correctly.

## Scope

### In Scope

- Make `TestEnv::wait_for_global_downloads_persisted` wait, within its existing 5-second timeout,
  until both the database value and the in-memory
  `tracker_core_persisted_torrents_downloads_total` counter are at least the expected value
  (`>= expected`). The helper already compares the database value with `>=`; keeping the same
  comparison for the counter means a caller that waits for `1` after more downloads were
  persisted does not hang until the timeout.
- Update the helper's doc comment and the comment in the test to explain the write order.
- Issue-local reproduction and recheck evidence.

### Out of Scope

- Changing the order of the database write and the counter increment in production code. The
  counter must stay behind the database write, because it counts persisted downloads.
- Other flaky tests not related to the persisted downloads counter.
- Changes to the maintainer's merge script.

## Architectural Decisions

- Related ADRs: none known.
- ADRs to create: none known. The change only touches test support code.

## Design and Ownership Review

Not applicable for new collaborators. The change touches one reusable test-fixture method that
waits on asynchronous work. Its readiness wait keeps the existing absolute deadline: a single
`tokio::time::timeout` of 5 seconds around the whole polling loop, polling every 50 ms, which
covers both values. A timeout fails the test with an explicit message. No resources are created,
so there is no ownership or drop path to define.

Design-review checkpoint: after the first passing vertical slice (the new wait-helper test green
with the fixed helper, task T3), stop and record a design review of the changed fixture and the
new test before the remaining verification. Task T4 owns that review.

## Bug-Fix Process

Following `.claude/skills/fix-bug/SKILL.md`:

- **Analysis.** Hypothesis: the helper waits for the database only, while the handler increments
  the counter after the database write, so the test can read the counter before the increment.
- **Reproduction.** Reproduced. Plain repeated runs did not hit the window (400 runs, all passed).
  A temporary 300 ms `tokio::time::sleep` placed between the database write and the counter
  increment in `handler.rs` widens the window. With it, the unmodified test fails at
  `integration.rs:253` with `left: 0, right: 1`, which is the exact failure of the merge run. The
  temporary code is recorded verbatim in `manual-verification-evidence.md` and was reverted.
- **Regression-test boundary.** A new maintained test of the wait helper itself, with no running
  listener (see below). The existing integration test stays as the end-to-end check.
- **Red evidence.** The new helper test fails against the unchanged helper. The widened-window
  probe also fails the existing integration test against the unchanged helper.
- **Fix.** Change the test helper only.
- **Green and recheck.** The new helper test passes. Re-run the widened-window probe against the
  fixed helper (it must pass), revert the probe, then run the plain test and the whole integration
  binary repeatedly.

## Regression Test Strategy

The defect is the helper's wait condition, so the maintained regression test sits at that boundary:
a new test of `TestEnv::wait_for_global_downloads_persisted` in the tracker-core integration test
crate (`packages/tracker-core/tests/`). It needs no production delay hook and does not depend on
timing luck, because no listener runs:

- **Arrange:** build the environment with `TestEnv::new` and persistence enabled, without calling
  `start`, so no event listener runs. Write the global downloads value `1` straight into the
  database through the `torrent_metrics_store`. The in-memory persisted counter stays `0`.
- **Act and assert, part 1:** await the helper under a short outer timeout (for example 500 ms).
  It must still be pending when that timeout ends, because the counter is `0`.
- **Act and assert, part 2:** increment the in-memory persisted counter to `1` through the stats
  repository, then await the helper again. It must return well within its own 5-second timeout.

Against the unchanged helper, part 1 fails: the helper returns at once because the database
already holds `1`. That run is the recorded red evidence. The test name and file are chosen when
it is written, following the `write-unit-test` skill.

The existing integration test,
`it_should_persist_the_global_number_of_completed_peers_into_the_database`, stays as the
end-to-end check that combines the background listener, the ephemeral database and the counter.
It does not guard the defect on its own: it passed 400 plain runs with the faulty helper. The
temporary widened-window probe in `handler.rs` (mutate-then-restore) shows that the end-to-end
symptom, the exact merge-run failure, goes away with the fix. The probe is not kept, because a
maintained test cannot inject that delay without a test-only hook in production code.

The handler unit tests already prove the handler increments the counter after a successful
database write, so no new handler test is needed.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                     | Notes / Expected Output                                                                                                                       |
| --- | ------ | ---------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Reproduce the race                       | Widened-window probe fails at `integration.rs:253` with `left: 0, right: 1`. Recorded in `manual-verification-evidence.md` V1 and V2.         |
| T2  | TODO   | Add the wait-helper regression test (red) | New test as described in Regression Test Strategy. It fails against the unchanged helper. The same-branch widened-window probe also fails the integration test. Both recorded in V3. |
| T3  | TODO   | Fix the wait helper                      | `wait_for_global_downloads_persisted` waits for the database value and the in-memory counter in one bounded loop. Comments updated. The new helper test passes. |
| T4  | TODO   | Design review of the fixture and test, then maintainer stop | After focused green validation, record the review in V5: the prose-first Arrange-Act-Assert comparison of the new test, the helper's boundary and naming, and its abstraction level. Stop for maintainer review before T5. |
| T5  | TODO   | Green and like-for-like recheck          | New helper test passes; probe plus fix passes; probe reverted; plain test and whole integration binary pass on repeated runs. Recorded in V4. |
| T6  | TODO   | Automatic checks and completion review   | `linter all`, `cargo test -p torrust-tracker-core`, pre-push checks; acceptance criteria re-reviewed; completion review recorded.              |

## Commit Points

| Task   | Coherent change set                                                         | Commit policy                                                       |
| ------ | --------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| T1     | Issue specification and reproduction evidence                               | Spec-only PR; commit after maintainer approval of the spec.         |
| T2     | Wait-helper regression test, seen red against the unchanged helper          | Commit after the red run is recorded and the test design is reviewed. |
| T3, T4 | Fixed wait helper and updated comments, after the recorded design review    | Commit after the tests pass, the review is recorded and the maintainer has reviewed. |
| T5, T6 | Red, green and recheck evidence; spec progress and completion review update | One evidence commit, separate from the code change.                 |

Every commit uses a Conventional Commit message with the narrow affected scope and is signed with
GPG. Both test-producing tasks (the new wait-helper test in T2 and the changed reusable fixture in
T3) use the `write-unit-test` skill. After focused green validation, task T4 records an explicit
design review with the mandatory prose-first Arrange-Act-Assert comparison: write temporary prose
for each section of the new test, refactor until the code expresses it, remove redundant prose,
and keep only irreducible context. The review confirms the test exposes the one causal initial-state
difference (database at `1`, counter at `0`), the fixture owns only incidental mechanics, and the
Act and the independently specified expected result stay visible. It judges the helper's boundary
by its named action and abstraction level, not by caller count. Stop for maintainer review after
this review, before final verification, commit or pull request.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/fix-persisted-downloads-integration-test-race/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-18 17:41 UTC - Claude Code (Claude Opus 5.5) - Drafted the specification after
  reproducing the race with a widened-window probe; 400 plain runs did not hit it - see
  `manual-verification-evidence.md` V1 and V2.
- 2026-11-18 18:50 UTC - Claude Code (Claude Opus 5.5) - Maintainer approved the spec; created issue #2953 and
  moved the spec to `docs/issues/open/2953-fix-persisted-downloads-integration-test-race/`.

## Acceptance Criteria

- [ ] AC1: `TestEnv::wait_for_global_downloads_persisted(expected)` returns only when both the
      database global downloads value and the in-memory
      `tracker_core_persisted_torrents_downloads_total` counter are at least `expected`, and fails
      the test with an explicit message after 5 seconds otherwise.
- [ ] AC2: With the temporary 300 ms delay between the database write and the counter increment,
      `it_should_persist_the_global_number_of_completed_peers_into_the_database` fails before the
      fix and passes after it.
- [ ] AC3: Without the delay, the test and the whole tracker-core integration binary pass on
      repeated runs (300 isolated runs of the test, 100 runs of the binary).
- [ ] AC4: No other test reads the persisted downloads counter after waiting only for the
      database; the search result is recorded in this spec.
- [ ] AC5: No production code changes.
- [ ] AC6: A maintained test of the wait helper, with no running listener, writes `1` to the
      database while the in-memory counter is `0`, checks the helper is still pending after a
      short timeout, then increments the counter and checks the helper returns. It fails against
      the unchanged helper and passes after the fix.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-core` (stable Rust toolchain)
- `cargo +nightly fmt --all -- --check` (nightly Rust toolchain)
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                    | Human-oriented command/steps                                                                                     | Expected Result                                 | Status | Evidence                                     |
| --- | ------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- | ----------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Plain repeated runs on `develop`            | Run the built integration binary 300 times filtered to the test, then 100 times unfiltered                       | Shows how rare the race is without help         | DONE   | `manual-verification-evidence.md` section V1 |
| M2  | Reproduce with a widened window             | Add the 300 ms sleep in `handler.rs`, run the test with `cargo test`, revert the sleep                           | Fails at `integration.rs:253`, `left: 0, right: 1` | DONE   | `manual-verification-evidence.md` section V2 |
| M3  | Red on the implementation branch            | Run the new helper test before the fix; then the same probe as M2 on that branch, reverted after                 | Helper test fails; probe run fails as in M2     | TODO   | `manual-verification-evidence.md` section V3 |
| M4  | Green and like-for-like recheck             | Run the new helper test with the fix; the same probe with the fix, reverted after; repeat M1                     | Helper test passes; probe run passes; all plain runs pass | TODO   | `manual-verification-evidence.md` section V4 |
| M5  | Test-design review                          | Record the prose-first Arrange-Act-Assert comparison and the fixture review after green                          | Review recorded; maintainer reviewed before T5  | TODO   | `manual-verification-evidence.md` section V5 |

### Disposable Verification Scripts

None. The probe is a one-line temporary edit, recorded verbatim in the evidence and reverted.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                     |
| ----- | ---------------------- | -------------------------------------------- |
| AC1   | TODO                   | Implementation PR diff                       |
| AC2   | TODO                   | `manual-verification-evidence.md` V3 and V4  |
| AC3   | TODO                   | `manual-verification-evidence.md` V4         |
| AC4   | TODO                   | Background, "Other callers"                  |
| AC5   | TODO                   | Implementation PR diff                       |
| AC6   | TODO                   | `manual-verification-evidence.md` V3 and V4  |

## Risks and Trade-offs

- **The probe is not maintained.** It is the only check of the end-to-end symptom. Mitigation: the
  new wait-helper test guards the faulty wait condition deterministically, and the helper doc
  comment states the write order and why both values are awaited. Adding a delay hook to
  production code only for tests is not worth it.
- **Slower failure on a real bug.** If the counter is never incremented, the test now fails after
  the 5-second timeout instead of at the assertion. The timeout message names both values, so the
  cause stays clear.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository template at
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no
  material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- PR where the failure was observed: torrust/torrust-tracker#2946 (merge-script pre-push run)
- Similar bounded-retry pattern: the `restored` timeout loop of
  `it_should_persist_the_number_of_completed_peers_for_each_torrent_into_the_database`, above the
  affected test in `packages/tracker-core/tests/integration.rs`
