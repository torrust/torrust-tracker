---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 1840
github-issue: 2402
spec-path: docs/issues/open/2402-1840-cancel-superseded-pr-runs/ISSUE.md
branch: "2402-1840-cancel-superseded-pr-runs"
related-pr: null
last-updated-utc: "2026-10-01 14:52"
semantic-links:
  skill-links:
    - create-issue
    - update-github-workflow-actions
  related-artifacts:
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
    - docs/self-hosted-runner.md
    - docs/issues/open/2386-1840-self-hosted-runner-minimum-capacity/ISSUE.md
    - docs/issues/open/2386-1840-self-hosted-runner-minimum-capacity/manual-verification-evidence.md
    - docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #2402 - Cancel Superseded Pull-Request CI Runs

Parent EPIC: #1840 - Improve PR Workflow Performance

## Goal

When a new commit is pushed to a pull request, cancel that pull request's older `Container` and
`Testing` runs that are still queued or running, so the single self-hosted runner does not build
commits that will never be merged. Runs for pushes to `develop`, `main`, and `releases/**` are
never cancelled.

## Background

Every push to a pull request targeting `develop` starts a `Test (Docker)` job on the one
self-hosted runner, `torrust-runner-01`. The maintainer normally works with 6 AI agents and about 4
open pull requests, and review rework runs each pull request's checks 3 to 5 times. When a fix is
pushed while the previous run is still queued or running, the older run tests a commit nobody will
merge, but it still runs in full, and every other pull request's job waits behind it. No workflow
in `.github/workflows/` has a `concurrency` group.

Issue #2386 measured the job at 801 s on the current server and replayed the 554 real `Container`
runs from 2026-08-31 to 2026-09-30 through the runner (V3 of its
`manual-verification-evidence.md`):

| Setup                     | Jobs run | Waited >15 min | 90th-percentile wait | Longest wait |
| ------------------------- | -------- | -------------- | -------------------- | ------------ |
| 1 runner (today)          | 554      | 206            | 140 min              | 239 min      |
| 1 runner, cancelling      | 395      | 28             | 12 min               | 79 min       |
| 2 servers                 | 554      | 50             | 13 min               | 63 min       |
| 2 servers, cancelling     | 395      | 0              | 0 min                | 11 min       |

Cancelling superseded runs gives about the same 90th-percentile wait as a second server, at no
cost. The maintainer chose to do this first, and to add a second server only if waits over
15 minutes remain common afterwards (#2386 Decision).

## Scope

### In Scope

- A workflow-level `concurrency` group in `.github/workflows/container.yaml` and
  `.github/workflows/testing.yaml` that cancels a pull request's in-progress or pending run when a
  newer run for the same pull request starts.
- Keeping every push run (`develop`, `main`, `releases/**`, and other branches) in its own group,
  so it is neither cancelled nor replaced.
- A short note in `docs/self-hosted-runner.md` ("Add Runner Capacity") that superseded pull-request
  runs are cancelled.
- The EPIC #1840 row for this issue.

### Out of Scope

- A second server or a second runner instance (#2386 Decision; test isolation is EPIC #2392).
- Other workflows (for example `docs-lint.yaml`): they run briefly on GitHub-hosted runners and do
  not use the self-hosted runner.
- Cancelling superseded pushes to feature branches in this repository.
- Changing job routing, triggers, or the security gating of fork pull requests.

## Architectural Decisions

- Related ADRs:
  `docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md`
  (one runner instance; add instances only when measured queue time requires it).
- ADRs to create: none expected. The choice between cancelling and adding a server is recorded in
  the #2386 Decision section.

## Design and Ownership Review

Proposed group, identical in both workflows:

```yaml
concurrency:
  group: ${{ github.workflow }}-${{ github.event_name == 'pull_request' && format('pr-{0}', github.event.pull_request.number) || format('run-{0}', github.run_id) }}
  cancel-in-progress: ${{ github.event_name == 'pull_request' }}
```

- Pull-request runs share one group per workflow and pull request, so a newer run cancels the older
  one.
- Push runs get a group of their own (`run-<run_id>`). A shared group would not work even with
  `cancel-in-progress: false`: by default (`queue: single`) GitHub keeps at most one pending run
  per group and cancels the older pending one, so two quick merges could leave a `develop` commit
  untested and unpublished. The GitHub documentation uses the same `github.run_id` fallback
  ([workflow syntax, `concurrency`](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#concurrency)).
- `github.workflow` keeps `Container` and `Testing` in separate groups, so one does not cancel the
  other.

Other aspects: Not applicable (no child processes, asynchronous I/O, or test fixtures).

## Bug-Fix Process

Not applicable. This is a CI efficiency change, not a defect.

## Regression Test Strategy

Not applicable. Workflow behavior is verified manually on real runs (M1, M2).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                              | Notes / Expected Output                                                                                         |
| --- | ------ | --------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Cancel superseded `Container` runs | `concurrency` block in `container.yaml`, with a one-line comment linking this issue.                           |
| T2  | TODO   | Cancel superseded `Testing` runs   | The same block in `testing.yaml`.                                                                               |
| T3  | TODO   | Update the operations guide       | `docs/self-hosted-runner.md` "Add Runner Capacity" states that superseded pull-request runs are cancelled.      |
| T4  | TODO   | Verify on real runs               | M1 and M2 in issue-local `manual-verification-evidence.md`.                                                     |
| T5  | DONE   | Update EPIC #1840                 | Row 18 for this issue.                                                                                          |

## Commit Points

| Task   | Coherent change set                                 | Commit policy                                        |
| ------ | --------------------------------------------------- | ---------------------------------------------------- |
| T1, T2 | `concurrency` blocks in both workflows              | Commit after `linter all` and maintainer review.     |
| T3     | Guide note                                          | Commit after focused validation.                     |
| T4     | `manual-verification-evidence.md`                   | Commit after the runs complete.                      |
| T5     | EPIC #1840 row and progress log                     | Commit after focused validation.                     |

Use Conventional Commits (`ci(workflows)`, `docs(self-hosted-runner)`, `docs(issues)`) and sign
every commit with GPG. Both workflow files carry a `skill-link: update-github-workflow-actions`
marker; review that skill when changing them.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1840-cancel-superseded-pr-runs/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, pre-commit checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-30 15:26 UTC - josecelano, GitHub Copilot - Draft created from the #2386 queue replay and
  the maintainer's decision to cancel superseded runs before adding a second server - #2386
- 2026-10-01 14:52 UTC - josecelano, GitHub Copilot - Maintainer approved the spec, confirming that
  `testing.yaml` is included; created #2402, linked it as a sub-issue of #1840, and moved the spec to
  `docs/issues/open/` - #2402

## Acceptance Criteria

- [ ] AC1: A newer push to a pull request cancels that pull request's older `Container` and
      `Testing` runs, whether queued or running.
- [ ] AC2: Push runs, including consecutive pushes to `develop`, are never cancelled or replaced.
- [ ] AC3: `Container` and `Testing` runs do not cancel each other, and runs of different pull
      requests do not cancel each other.
- [ ] AC4: `docs/self-hosted-runner.md` states that superseded pull-request runs are cancelled.
- [x] AC5: EPIC #1840 lists this issue.
- [ ] `linter all` exits with code `0`
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all` (includes yamllint)
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                        | Human-oriented command/steps                                                                                                                  | Expected Result                                                                                   | Status | Evidence                                     |
| --- | ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Superseded pull-request run     | On the implementation pull request, push a second non-documentation commit while the first `Container` run is queued or running; `gh run list --branch <branch>` | The older `Container` and `Testing` runs end `cancelled`; the newer runs complete; other pull requests' runs are unaffected | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | Push runs are kept              | After the merge, list `Container` runs for `develop` pushes: `gh run list --workflow container.yaml --branch develop --event push`            | The merge commit's run completes and publishes; no `develop` push run is `cancelled`              | TODO   | `manual-verification-evidence.md` section V2 |

### Disposable Verification Scripts

None.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | DONE                   | EPIC #1840 row 18 |

## Risks and Trade-offs

- A cancelled run shows as `cancelled` in the pull request's checks, which can look like a failure.
  Mitigation: the guide note; the newest run is the one that reports on the head commit.
- Re-running an older run's jobs while a newer run is active puts the older run back in the group
  and cancels the newer one. Mitigation: the guide note says to re-run only the latest run.
- A run cancelled part-way has already used runner time, so the real saving is smaller per
  cancelled run than the replay assumes; but queued runs are superseded more often than the replay
  counts, which works the other way.
- AC2 relies on the `run-<run_id>` group. If it were wrong, a `develop` commit could go unpublished.
  Mitigation: M2, and the design keeps `cancel-in-progress` `false` for pushes as a second guard.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise add a progress-log
  entry explaining why not.
- An independent reviewer records its result in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #1840
- Related issues: #2386 (capacity decision and queue replay), #2323 (self-hosted runner), #2392
  (test isolation)
- Related ADRs: `docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md`
