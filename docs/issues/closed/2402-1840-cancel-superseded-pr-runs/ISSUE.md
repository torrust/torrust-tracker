---
schema-version: 1
doc-type: issue
issue-type: task
status: done
priority: p2
epic: 1840
github-issue: 2402
spec-path: docs/issues/closed/2402-1840-cancel-superseded-pr-runs/ISSUE.md
branch: "2402-1840-cancel-superseded-pr-runs"
related-pr: 2419
last-updated-utc: "2026-10-04 19:09"
semantic-links:
  skill-links:
    - create-issue
    - update-github-workflow-actions
  related-artifacts:
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
    - docs/self-hosted-runner.md
    - "issue #2386"
    - "issue #1840"
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

| Setup                     | Jobs completed | Waited >15 min | 90th-percentile wait | Longest wait |
| ------------------------- | -------------- | -------------- | -------------------- | ------------ |
| 1 runner (today)          | 554            | 206            | 140 min              | 239 min      |
| 1 runner, cancelling      | 376            | 12             | 11 min               | 32 min       |
| 2 servers                 | 554            | 50             | 13 min               | 63 min       |
| 2 servers, cancelling     | 392            | 0              | 0 min                | 12 min       |

With cancelling, a new pull-request run cancels that pull request's queued and running runs. On one
runner it cancels 40 queued and 138 running runs, so it frees the runner at once instead of letting a
stale build finish. Cancelling gives a lower 90th-percentile and longest wait than a second server,
at no cost. The maintainer chose to do this first, and to add a second server only if waits over
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
| T1  | DONE   | Cancel superseded `Container` runs | `concurrency` block in `container.yaml`, with a one-line comment linking this issue.                           |
| T2  | DONE   | Cancel superseded `Testing` runs   | The same block in `testing.yaml`.                                                                               |
| T3  | DONE   | Update the operations guide       | `docs/self-hosted-runner.md` "Add Runner Capacity" states that superseded pull-request runs are cancelled.      |
| T4  | DONE   | Verify on real runs               | M1 and M2 are recorded in issue-local `manual-verification-evidence.md`.                                        |
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
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, pre-commit checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-30 15:26 UTC - josecelano, GitHub Copilot - Draft created from the #2386 queue replay and
  the maintainer's decision to cancel superseded runs before adding a second server - #2386
- 2026-10-01 14:52 UTC - josecelano, GitHub Copilot - Maintainer approved the spec, confirming that
  `testing.yaml` is included; created #2402, linked it as a sub-issue of #1840, and moved the spec to
  `docs/issues/open/` - #2402
- 2026-10-01 16:26 UTC - josecelano, GitHub Copilot - Addressed the Copilot review of PR #2403:
  corrected the replay figures (an event-driven replay of the cancellations), made M1 observe both
  workflows and a control pull request, made M2 overlap two `develop` pushes, and replaced issue
  paths in the frontmatter with issue numbers - `docs/pr-reviews/pr-2403-review/PR-REVIEW.md`
- 2026-10-02 17:17 UTC - josecelano, GitHub Copilot - Added the approved workflow-level
  concurrency policy to `container.yaml` and `testing.yaml`; `linter all` and the mandatory
  pre-commit gate passed - `ci(workflows): [#2402] cancel superseded PR runs`
- 2026-10-02 17:52 UTC - josecelano, GitHub Copilot - Opened implementation PR #2419 and started
  M1 with disposable draft control PR #2420; its independent `Container` run is active.
- 2026-10-02 17:54 UTC - josecelano, GitHub Copilot - M1 commit A started with `Container`
  queued and `Testing` in progress; pushed the superseding commit B next.
- 2026-10-02 18:21 UTC - josecelano, GitHub Copilot - Addressed Copilot review findings on #2419:
  the runner runbook now warns against rerunning stale cancelled runs, and the completed `linter all`
  acceptance criterion is marked done.
- 2026-10-03 11:24 UTC - josecelano, GitHub Copilot - M1 confirmed cancellation and successful
  replacements, but independent review found that its #2420 control run was not concurrent. M1
  remains in progress pending a rerun with an active control. M2 completed on the approved fork
  fallback: both target workflows succeeded for each of two overlapping `develop` pushes. Evidence
  and the fork-publication limitation are recorded in `manual-verification-evidence.md` and
  `implementation-retrospective.md`.
- 2026-10-03 12:16 UTC - josecelano, GitHub Copilot - Reran M1 with concurrent control PR #2426
  and verification PR #2427. Both superseded verification runs were cancelled; both replacement
  and control runs succeeded. Independent review can now validate AC3.
- 2026-10-03 19:03 UTC - josecelano, GitHub Copilot - Independent review confirmed the rerun
  proved running-run cancellation and AC3, but not queued-run cancellation. T4 and AC1 remain
  pending a dedicated queued-run scenario.
- 2026-10-04 08:09 UTC - josecelano, GitHub Copilot - Recorded the dedicated queued-run M1
  scenario as complete. The subsequent independent review found that A's target jobs had already
  started before B cancelled them, so this did not prove queued-run cancellation.
- 2026-10-04 08:33 UTC - Task Reviewer - Independent completion review failed: M1 still lacks
  proof that a queued target job is cancelled, and `agent-review-reports.md` is now recorded with
  the findings. T4, AC1, manual verification, and the post-implementation acceptance review
  remain pending.
- 2026-10-04 10:26 UTC - josecelano, GitHub Copilot - Reran M1 with control commit
  `99b47d67`, verification A `be79b16a`, and verification B `4cbe7a8d`. Immediately before B,
  A's self-hosted `Container` job was `queued` with no runner assigned. B cancelled A's
  `Container` and active `Testing` runs; both B replacements and the control `Container`
  succeeded. T4, AC1, manual verification, and the post-implementation acceptance review are
  complete pending a new independent review.
- 2026-10-04 10:29 UTC - Task Reviewer - Independent re-review passed all acceptance criteria.
  Corrected the evidence date range to include 2026-10-04 and appended the review report. The
  reviewer and report checkpoints are complete; GitHub issue closure and archival remain pending.
- 2026-10-04 10:30 UTC - josecelano, GitHub Copilot - Closed GitHub issue #2402 and disposable
  verification PRs #2426 and #2427, then archived this issue record in `docs/issues/closed/`.
- 2026-10-04 10:33 UTC - josecelano, GitHub Copilot - Completed the stale-reference audit,
  `linter all`, and mandatory pre-commit gate; the archived specification is ready to commit.
- 2026-10-04 10:34 UTC - josecelano, GitHub Copilot - Reconciled the completed manual-verification
  and acceptance-review checkboxes with their supporting M1/M2 evidence and passed independent
  review.
- 2026-10-04 19:09 UTC - josecelano, GitHub Copilot - Addressed both Copilot PR #2433 review
  findings by replacing bare `related-artifacts` filenames with repository-relative paths in the
  archived manual evidence and retrospective metadata.

## Acceptance Criteria

- [x] AC1: A newer push to a pull request cancels that pull request's older `Container` and
      `Testing` runs, whether queued or running.
- [x] AC2: Push runs, including consecutive pushes to `develop`, are never cancelled or replaced.
- [x] AC3: A newer run of one workflow does not cancel the other workflow's run for the same
      commit, and a pull request's runs do not cancel another pull request's runs.
- [x] AC4: `docs/self-hosted-runner.md` states that superseded pull-request runs are cancelled.
- [x] AC5: EPIC #1840 lists this issue.
- [x] `linter all` exits with code `0`
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all` (includes yamllint)
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                        | Human-oriented command/steps                                                                                                                  | Expected Result                                                                                   | Status | Evidence                                     |
| --- | ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Superseded pull-request run     | 1. Have a control pull request open with a non-documentation change (another open pull request, or a disposable draft one) whose `Container` run is queued or running. 2. Push commit A to the implementation pull request and wait until `gh run list --branch <branch> --json workflowName,headSha,status` shows both its `Container` and `Testing` runs `queued` or `in_progress`. 3. Push commit B. 4. Inspect each run with `gh run view <run-id> --json workflowName,headSha,status,conclusion`. | Commit A's `Container` and `Testing` runs end `cancelled`; commit B's `Container` and `Testing` runs both complete; the control pull request's run is not `cancelled` | DONE | `manual-verification-evidence.md` section V1 |
| M2  | Push runs are kept              | After the merge, push two commits to `develop` so that the second push's runs start while the first push's `Container` and `Testing` runs are still queued or running (two merges in quick succession). If no such overlap occurs, repeat on a fork's `develop` with Actions enabled, where both workflows run on `ubuntu-latest`. Inspect all four runs with `gh run view <run-id> --json workflowName,headSha,status,conclusion`. | All four runs complete, none `cancelled`; upstream, both `Container` runs publish (`Publish (Development)` succeeds) | DONE | `manual-verification-evidence.md` section V2 (publication is repository-gated and not executed by the fork fallback) |

### Disposable Verification Scripts

None.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | `manual-verification-evidence.md` V1 |
| AC2   | DONE                   | `manual-verification-evidence.md` V2 |
| AC3   | DONE                   | `manual-verification-evidence.md` V1 |
| AC4   | DONE                   | `docs/self-hosted-runner.md` |
| AC5   | DONE                   | EPIC #1840 row 18 |

## Risks and Trade-offs

- A cancelled run shows as `cancelled` in the pull request's checks, which can look like a failure.
  Mitigation: the guide note; the newest run is the one that reports on the head commit.
- Re-running an older run's jobs while a newer run is active puts the older run back in the group
  and cancels the newer one. Mitigation: the guide note says to re-run only the latest run.
- The replay (#2386 V3) models a fixed 801 s job; real jobs vary, and warm jobs are much faster.
  Mitigation: the recheck after this change measures the real queue times.
- AC2 relies on the `run-<run_id>` group. If it were wrong, a `develop` commit could go unpublished.
  Mitigation: M2 makes two `develop` pushes overlap, which a shared push group would fail.

## Implementation Completion Review

- Retrospective: `implementation-retrospective.md` records material fallback-verification findings,
  the review-driven concurrent-control rerun, and the remaining queued-run scenario.
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
