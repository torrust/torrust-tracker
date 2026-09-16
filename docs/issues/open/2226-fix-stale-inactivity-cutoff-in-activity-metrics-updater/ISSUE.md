---
doc-type: issue
issue-type: bug
status: open
priority: p2
epic: null
github-issue: 2226
spec-path: docs/issues/open/2226-fix-stale-inactivity-cutoff-in-activity-metrics-updater/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-15 11:05
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - src/bootstrap/jobs/activity_metrics_updater.rs
    - packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs
    - packages/swarm-coordination-registry/src/swarm/coordinator.rs
    - packages/swarm-coordination-registry/src/swarm/registry.rs
    - packages/tracker-core/src/torrent/manager.rs
    - docs/issues/drafts/1488-si-5-migrate-activity-metrics-updater/ISSUE.md
    - docs/issues/open/2226-fix-stale-inactivity-cutoff-in-activity-metrics-updater/evidence.md
---

<!-- skill-link: create-issue -->

# Issue #2226 - Fix Stale Inactivity Cutoff in Activity Metrics Updater

## Goal

Make the activity metrics updater derive its peer-inactivity cutoff at each update so the inactive-peer and inactive-torrent gauges reflect the current time rather than the tracker startup time.

## Background

Source inspection confirms a defect in the current updater wiring:

- `src/bootstrap/jobs/activity_metrics_updater.rs` calls `peer_inactivity_cutoff_timestamp` once while constructing the periodic job.
- That precomputed cutoff is passed to `statistics::activity_metrics_updater::start_job` and reused for every 15-second tick.
- `SwarmCoordinator::count_inactive_peers` classifies a peer as inactive when `peer.updated <= cutoff`.

Consequently, a peer that announces after the tracker starts always has an `updated` time later than the frozen cutoff and can never be counted as inactive by the updater, even after `max_peer_timeout` elapses. The inactive gauges can therefore report too little inactivity for the lifetime of the process.

`TorrentsManager::current_cutoff` independently derives the equivalent cutoff immediately before cleanup. This confirms the intended time-relative policy, but it also exposes duplicated cutoff logic.

This bug is related to, but is not a child of, EPIC #1488. Its SI-5 draft changes the same updater's shutdown lifecycle. Coordinate implementation order with SI-5 to avoid conflicting edits to the job signature and loop.

## Scope

### In Scope

- Derive the inactivity cutoff from the current clock and `max_peer_timeout` for every activity-metrics update.
- Add deterministic `Stopped`-clock unit coverage proving that a peer which announced after job startup becomes inactive after `max_peer_timeout` without another announce.
- Assess whether both the updater and `TorrentsManager` can use one appropriately owned cutoff helper without introducing an undesirable package dependency; implement the shared helper only if that assessment supports it.
- Manually verify the inactive-peer gauge for a peer that becomes inactive after tracker startup.

### Out of Scope

- Making the 15-second updater interval configurable.
- Changing metric names, labels, the statistics repository API, or metric semantics other than correcting the stale cutoff.
- Changing cleanup-job behavior or the updater shutdown lifecycle; coordinate with SI-5 rather than duplicating its lifecycle migration.
- Making this bug a subissue of EPIC #1488.

## Architectural Decisions

- Related ADRs: none.
- ADRs to create: `None known`. If unifying cutoff computation requires a new dependency or changes extractable-package boundaries, stop and record that decision in an ADR at the appropriate scope.

## Design and Ownership Review

Not applicable. The change affects a pure time-relative calculation in an existing periodic job. It must retain the job's existing weak-reference ownership behavior; this issue introduces no child process, readiness wait, or resource-lifetime policy.

## Bug-Fix Process

This bug follows the repository's fixed sequence for defects. Each step must be complete, with evidence, before the next starts.

| Step | Activity | Status | Evidence |
| ---- | -------- | ------ | -------- |
| B1 | Analyse the defect at source level | DONE | `Background` section; source wiring captured in `evidence.md` step 7 |
| B2 | Reproduce the defect against the real artifact | DONE | `evidence.md` V1: post-startup peer still counted active ~267 s after its timeout expired |
| B3 | Analyse which test type best protects against regression | DONE | `Regression Test Strategy` section below |
| B4 | Write the failing regression test(s) first | TODO | T1 |
| B5 | Fix the defect | TODO | T2 |
| B6 | Recheck: rerun the regression test and the original reproduction | TODO | T4; `manual-verification-evidence.md` M1 repeats the `evidence.md` scenario against the fixed build |

## Regression Test Strategy

### Where the defect lives

The defect is not inside `count_inactive_peers` (correctly tested in `coordinator.rs` and `registry.rs`); it is in **who computes the cutoff and when**. Today the caller (`src/bootstrap/jobs/activity_metrics_updater.rs`) computes it once and the library job (`packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs`) treats it as a constant. No existing test observes this seam.

### Test types considered

| Candidate | Verdict | Reason |
| --------- | ------- | ------ |
| Unit test in `swarm-coordination-registry` on the per-update function, with the `Stopped` clock | **Primary** | Deterministic, milliseconds, exercises the exact seam (cutoff derived at update time). Requires the update function to receive the timeout policy rather than a precomputed cutoff, which is also the fix. |
| Unit test of the spawned loop with `tokio::time::pause` + `advance` | Optional secondary | Proves the loop calls the per-update path repeatedly with fresh cutoffs, but the hardcoded 15 s interval and spawned task make it less readable. Add only if the loop cannot be trivially inspected. |
| Bootstrap-level test in `src/bootstrap/jobs/activity_metrics_updater.rs` | Rejected as primary | Requires an `AppContainer`; wall-clock coupled; duplicates the library test at a heavier boundary. |
| Integration or E2E test waiting real `max_peer_timeout` + 15 s | Rejected | Slow, timing-fragile, and no better at exposing the causal state. Real-time verification stays a manual scenario (`M1`). |

### Primary regression test specification

Location: `packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs` (`#[cfg(test)] mod tests`), using `CurrentClock = Stopped` per the `write-unit-test` skill.

- **Arrange**: pin the stopped clock; create a `Registry` and stats `Repository`; construct the updater's per-update policy from `max_peer_timeout` (e.g. 20 s) at this instant (this models job creation). Then announce one peer **after** that instant and advance the clock by `max_peer_timeout + 1` s. The single causal difference is: *the peer announced after the policy was created, and more than `max_peer_timeout` has since elapsed*.
- **Act**: run one activity-metrics update through the production function.
- **Assert**: `swarm_coordination_registry_peers_inactive_total` gauge equals `1` (and `torrents_inactive_total` equals `1`).

A companion test asserts the complementary state (clock advanced by less than `max_peer_timeout`, gauge stays `0`) so the pair fixes both boundaries of the behavior.

Obtaining a genuine red against the current API: the existing `update_activity_metrics(..., inactivity_cutoff)` can be called with a cutoff computed at job-creation time (before the peer is added and the clock advanced), exactly as the bootstrap does today. Asserting `inactive_peers_total == 1` then fails, reproducing the defect at unit level. The green step changes the function to take the timeout policy and derive the cutoff itself; the test's Arrange, Act, and Assert intent does not change, only the argument passed. Record the red run output in task evidence. The test-design review (prose-first AAA) must confirm the post-startup announce and the elapsed time remain visible in the test body.

### Why the manual reproduction is still required

The unit test proves the library computes the cutoff per update. Only the real tracker run (`M1`) proves the bootstrap wiring passes the policy, not a frozen value, into the job. `M1` must reuse the exact scenario in `evidence.md` so the before/after comparison is like-for-like.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | TODO | Write the failing regression tests (B4) | Add the two `Stopped`-clock tests from `Regression Test Strategy` against the current API; the "inactive after timeout" test must fail (red) with the frozen job-creation cutoff. Record the red output and the prose-first AAA review in task evidence. |
| T2 | TODO | Fix: compute the cutoff for each update (B5) | Derive the cutoff immediately before `get_activity_metadata` from the current clock and `max_peer_timeout`; update the bootstrap call site to pass the policy; remove `peer_inactivity_cutoff_timestamp`'s startup-time evaluation. Tests from T1 pass. |
| T3 | TODO | Review cutoff-helper ownership | Decide whether a shared helper belongs at an existing dependency boundary; either use it in both consumers or document why two thin call sites are preferable. |
| T4 | TODO | Recheck and record evidence (B6) | Run focused tests and `linter all`; repeat the `evidence.md` scenario against the fixed build and record it as `M1` in `manual-verification-evidence.md`; update acceptance verification. |
| T5 | TODO | Link the process follow-up issue | A separate follow-up issue for the `fix-bug` skill and bug-spec guardrails is being prepared (see `Process Follow-up`); once it exists, link its number here. No code change in this issue. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1-T2 | Regression tests plus the per-update cutoff fix that turns them green | One reviewed commit; never commit the red build. The recorded red output in task evidence is the proof that the test preceded the fix. Commit after focused validation, test-design review, and required review. |
| T3 | Deliberate helper-ownership refactor or documented no-change decision | Commit separately if it crosses package boundaries; do not create an empty commit for a no-change decision. |
| T4 | Completion evidence and specification progress | Commit separately when it improves reviewability. |
| T5 | Follow-up issue link only | No code change in this issue. |

For the test-producing task, use the `write-unit-test` skill. Complete a prose-first Arrange-Act-Assert review after the passing increment: make the post-startup announce the one visible causal state difference; keep incidental setup in the fixture; and keep the production Act and independently specified expected result visible. Record the review in task evidence before maintainer review and commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted and promoted to `docs/issues/open/2226-fix-stale-inactivity-cutoff-in-activity-metrics-updater/ISSUE.md`
- [x] Source-level and local-runtime defect confirmation recorded in `evidence.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2226 created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation, if maintainers require it
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

- 2026-09-15 10:00 UTC - GitHub Copilot - Copied and refreshed the parked draft as a folder-style bug specification; source inspection confirmed that a startup-time cutoff is reused on every updater tick, while peer inactivity compares against that cutoff.
- 2026-09-15 10:11 UTC - GitHub Copilot - Reproduced the defect against an isolated local tracker: one peer announced after startup remained active in every metrics update for about 287 seconds after the announce (about 267 seconds past its expiry) despite `max_peer_timeout = 20`; see `evidence.md`.
- 2026-09-15 10:30 UTC - GitHub Copilot - Added the explicit bug-fix process table, regression-test strategy (unit test at the per-update seam with the `Stopped` clock), and a process follow-up assessment for repository guidance.
- 2026-09-15 10:45 UTC - GitHub Copilot - Review pass: clarified how a genuine red test is obtained against the current API, merged the T1/T2 commit point, and aligned the follow-up with the agreed `.github/skills/dev/debugging/fix-bug` location.
- 2026-09-15 11:00 UTC - josecelano - Approved this independent bug specification for GitHub issue creation - Chat approval.
- 2026-09-15 11:05 UTC - GitHub Copilot - Created GitHub issue #2226 and promoted this specification to `docs/issues/open/` - https://github.com/torrust/torrust-tracker/issues/2226

## Acceptance Criteria

- [ ] AC1: Every activity-metrics update derives its inactivity cutoff from the current clock and configured `max_peer_timeout`.
- [ ] AC2: A peer that announces after tracker startup is counted as inactive after more than `max_peer_timeout` without a subsequent announce.
- [ ] AC3: Cutoff computation has deliberate ownership: both consumers use one suitable shared helper, or the implementation documents why existing package boundaries require separate thin call sites.
- [ ] AC4: Deterministic `Stopped`-clock unit tests in `swarm-coordination-registry` cover both boundaries: a post-startup peer is counted inactive after `max_peer_timeout` elapses, and remains active before it. The inactive-case test's red run against the pre-fix code is recorded in task evidence.
- [ ] AC5: The original reproduction scenario from `evidence.md` is repeated against the fixed build and the inactive-peer gauge changes to `1`.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant tests pass.
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`.
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [ ] Documentation is updated when behavior or workflow changes.

## Verification Plan

### Automatic Checks

- `cargo test -p torrust-tracker-swarm-coordination-registry --lib statistics::activity_metrics_updater`
- `cargo test -p torrust-tracker-swarm-coordination-registry --lib swarm::coordinator`
- `cargo test -p torrust-tracker-core --lib torrent::manager`
- `linter all`
- Pre-push checks when applicable

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Post-startup peer becomes inactive (recheck, B6) | Repeat the exact `evidence.md` V1 scenario against the fixed build: isolated tracker with `max_peer_timeout = 20`, cleanup disabled, usage statistics enabled; announce one peer with `tracker_client`; wait more than 35 seconds without another announce. | `swarm_coordination_registry_peers_inactive_total` changes from `0` to `1` in the metrics endpoint and the updater log reports `inactive_peers_total=1`. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Pre-fix behavior | Already executed against the pre-fix revision. | The inactive-peer gauge remained `0` for ~267 s after the timeout expired. | DONE | `evidence.md` V1 |

Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing M1. Record actual prerequisites, commands, output, relevant tracker logs, and outcomes so it can be compared line-for-line with `evidence.md`.

### Disposable Verification Scripts

None planned. The behavior must be covered by maintained Rust tests and a human-oriented tracker run, not a temporary script.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | |
| AC2 | TODO | |
| AC3 | TODO | |
| AC4 | TODO | |
| AC5 | TODO | |

## Risks and Trade-offs

- **Concurrent SI-5 edits:** the lifecycle migration and this fix touch the updater API and loop. Mitigation: rebase the draft's paths and signature assumptions after SI-5, or implement the changes together only with explicit maintainer approval.
- **Wrong helper placement:** sharing policy logic across `tracker-core` and `swarm-coordination-registry` could invert or expand package dependencies. Mitigation: assess the dependency direction before extracting a helper; retain two explicit call sites when that is the cleaner boundary.
- **Runtime timing:** the fixed 15-second update interval means the manual scenario needs enough wait time after the timeout. Mitigation: wait for both the timeout and a subsequent update tick.

## Process Follow-up

The bug-fix sequence used here (analyse, reproduce, choose the test type, write the failing test, fix, recheck) is not written down anywhere in the repository today: the `create-issue` skill and `docs/templates/ISSUE.md` are type-agnostic, and the Implementer agent describes TDD generically. Assessment of the three options:

| Option | Assessment |
| ------ | ---------- |
| Extend the `create-issue` skill | Partial fit. It can require bug specs to contain a `Bug-Fix Process` table and a `Regression Test Strategy` section, but the skill is about writing specs, not executing fixes. Keep it to the spec-shape requirement only. |
| New `fix-bug` skill at `.github/skills/dev/debugging/fix-bug/` | Best fit for the procedure itself: the six steps, what counts as a reproduction (real artifact, recorded commands and output, no throwaway scripts as evidence), how to choose the test type (unit at the causal seam first; integration or E2E only when the seam is not reachable), and the recheck rule (rerun the failing test and the original reproduction). It would link to `write-unit-test`, `run-tracker-locally`, `use-tracker-client`, and `create-issue`, and the Implementer agent would invoke it for `issue-type: bug`. |
| Change `docs/templates/ISSUE.md` | Add the two sections as bug-conditional blocks ("Required when `issue-type: bug`; delete otherwise"). Keeps a single template while making the process visible in every bug spec. |

Decision: do all three in one small follow-up issue, with the new skill as the canonical source and the `create-issue`/template/Implementer changes as thin links to it (documentation single-source-of-truth policy). A parked draft for that issue already exists and will be opened after this issue is created; this bug's spec is cited there as the worked example. Do not expand this issue's scope to include that work.

## Implementation Completion Review

After implementation, compare the observed behavior and validation evidence with this specification. Record reusable lessons, material design changes, and deviations from the plan in `implementation-retrospective.md` if warranted; otherwise record why none was needed in the progress log. Independent reviewers who receive this folder-style specification record their result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issue: #1488 (shutdown overhaul; SI-5 touches the same updater lifecycle)
- Related draft: `docs/issues/drafts/1488-si-5-migrate-activity-metrics-updater/ISSUE.md`
- GitHub issue: #2226
- Reproduction evidence: `evidence.md`
- Related code: `src/bootstrap/jobs/activity_metrics_updater.rs`, `packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs`, and `packages/tracker-core/src/torrent/manager.rs`
