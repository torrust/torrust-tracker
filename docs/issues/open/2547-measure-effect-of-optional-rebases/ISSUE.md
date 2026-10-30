---
schema-version: 1
doc-type: issue
issue-type: task
status: in-review
priority: p3
epic: null
github-issue: 2547
spec-path: docs/issues/open/2547-measure-effect-of-optional-rebases/ISSUE.md
branch: "2547-measure-effect-of-optional-rebases"
related-pr: 2690
last-updated-utc: "2026-10-30 13:49"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/closed/2507-stop-requiring-pull-request-rebases/ISSUE.md
    - docs/issues/closed/2507-stop-requiring-pull-request-rebases/baseline-evidence.md
    - docs/issues/closed/2507-stop-requiring-pull-request-rebases/manual-verification-evidence.md
    - docs/adrs/20261013161419_stop_requiring_pull_request_rebases.md
    - "issue #2507"
    - "issue #2525"
---

<!-- skill-link: create-issue -->

# Issue #2547 - Measure the effect of no longer requiring pull-request rebases

## Goal

Find out whether making pull-request rebases optional (issue #2507) reduced force-pushes, pull-request
CI runner time, and rebase-fallout commits. Compare a two-week window under the new policy with the
baseline that #2507 recorded for 1 to 13 October 2026.

## Background

Issue #2507 stopped requiring a rebase onto `develop` before merge. The ADR was accepted in spec
PR #2508, merged 2026-10-14 06:42 UTC. The maintainer merge tool was changed in PR #2526, merged
2026-10-14 12:18 UTC, to refresh a pull request's stale recorded base instead of asking for a rebase.
The goal was to cut CI reruns, review noise, and serialized merging.

The #2507 specification planned this comparison as its task T7 and acceptance criterion AC9. It needs
at least two weeks of merges under the new policy. The maintainer decided on 2026-10-15 to move it into
this issue so #2507 could close once its implementation and manual verification were complete.

Issue #2507 recorded the baseline, with reproduction commands, in
[`baseline-evidence.md`](../../closed/2507-stop-requiring-pull-request-rebases/baseline-evidence.md).
That file lives in `docs/issues/closed/2507-stop-requiring-pull-request-rebases/` since #2507 was
archived. The baseline has these sections:

- section 4: pull-request volume and queue;
- section 5: force-pushes per merged PR;
- section 6: CI cost of pull-request runs;
- section 7: rebase-fallout commits.

| Measure (baseline)                    | June 2026 | 1 to 13 October 2026 |
| ------------------------------------- | --------: | -------------------: |
| Merged PRs                            |        67 |                   67 |
| Median open PRs waiting               |         0 |                    8 |
| Force-pushes per merged PR            |      0.39 |                 1.58 |
| Merged PRs force-pushed at least once |       34% |                  81% |
| Estimated PR CI runner-hours          |         — |                197.8 |

The early merges are already mixed. Of the 18 merges from #2526 to #2543 (2026-10-14 12:18 to
2026-10-15 16:33 UTC), nine were merged while behind `develop` without a rebase, and six of those
relied on the merge tool's refresh. Recommended rebases are still common, though: for example,
pull requests #2530, #2538, and #2430 rebased clean branches. See the #2507
`manual-verification-evidence.md`, section "Merges Under the New Policy".

## Scope

### In Scope

- Measure the window from 2026-10-15 00:00 UTC to 2026-10-28 23:59 UTC (14 days) with the #2507
  baseline commands, and normalize per merged PR and per day so the 13-day baseline stays comparable.
  Per-day figures divide by calendar days, counted inclusively: 14 for the window (2026-10-15 to
  2026-10-28) and 13 for the baseline (2026-10-01 to 2026-10-13). Per-merged-PR figures divide by the
  number of pull requests merged in the same period. This specification fixes how each measure selects
  its data, because the baseline sections do not all say how they selected theirs:
  - force-pushes per merged PR and the share of merged PRs force-pushed at least once. The population
    is the pull requests merged in the window (`merged_at`), with every force-push of each of them,
    including events before 2026-10-15: baseline section 5 selects pull requests by merge date and
    counts each one's whole force-push timeline. The classification below uses the same population;
  - estimated PR CI runner-hours per workflow, sampled as in the baseline: six randomly sampled runs
    per workflow, with the random seed recorded in the evidence so the sample can be reproduced. The
    runs are the `pull_request` workflow runs created within the window, not the runs of the pull
    requests merged in it, so runs of pull requests still open count too. Baseline section 6 reports
    899 runs for 1 to 13 October and names no date field; creation date is this specification's reading
    of it;
  - rebase-fallout commits, compared with a baseline recounted for 1 to 13 October only: section 7 of
    the baseline lists 23 commits "since September 2026", of which 13 have author dates from 1 to 13
    October, so do not divide the whole list by 13 days or by the 67 October merges. A commit is in
    the window when its author date is, as in that 13-of-23 recount. The command in section 7 filters
    with `--since`, which is the committer date, and prints author dates;
  - pull-request volume and the daily open-PR queue, for context, as in baseline section 4.
- Classify every force-push of those pull requests with the method of #2507 manual-verification section V4,
  which compares where the push put the head's base, not only where the new head sits. The `develop`
  tip of its time is the last first-parent `develop` commit whose pull request has a `merged_at`
  before the push. A rewrite of a head that was already on the tip must not be counted as a rebase:
  - rebase: the push moved the base, that is, `git merge-base <tip> <beforeCommit>` differs from
    `git merge-base <tip> <afterCommit>`. It is required when the pushed-away head conflicted with the
    tip (`git merge-tree --write-tree`), and recommended when it merged cleanly;
  - not a rebase: the push left the base unchanged. Since PR #2531 merged (`merged_at` 2026-10-14 16:09 UTC),
    `AGENTS.md` allows other rewrites of an unmerged pull request's history, such as splitting or
    reordering commits, which the baseline could not contain: baseline section 5 counts every
    force-push as a rebase because history rewrites were otherwise forbidden.
  Report the total force-pushes per merged PR, which is the baseline's measure, and the rebase-only
  figure, which excludes the second class.
- Count merges made while behind `develop` whose head was pushed before `develop` reached `M^1`, the
  `develop` tip the merge was built on, and not pushed again before the merge (the recorded-base
  refresh path). The cutoff is `M^1`, not the Git merge base, which a head always postdates. As in
  #2507 manual-verification section V1, the count is inferred, because the merge tool's output is not
  captured.
- Record the comparison and a short conclusion in issue-local `post-change-evidence.md`.

### Out of Scope

- Changing the rebase policy, the ADR, the skills, or the merge tool. If the results argue for a
  change, propose it in a new issue.
- Building scripts or tooling for the measurement. Ad-hoc `gh` and `git` commands are enough. Record
  them verbatim.
- Measuring the `CI gate` work of #2525. Record every merge in the window that changed
  `.github/workflows/**` (found with `git log --first-parent` over that path), with its merge time and
  whether it belongs to #2525, so no workflow change is mistaken for an effect of the rebase policy.

## Architectural Decisions

- Related ADRs:
  [`20261013161419_stop_requiring_pull_request_rebases.md`](../../../adrs/20261013161419_stop_requiring_pull_request_rebases.md).
- ADRs to create: None known. A result that argues for changing the policy is a new issue, not an ADR
  edit inside this one.

## Design and Ownership Review

Not applicable. This task only measures existing GitHub and Git data.

## Bug-Fix Process

Not applicable. This is a measurement task.

## Regression Test Strategy

Not applicable. No code changes.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                         | Notes / Expected Output                                                                                                                                                                                                                                     |
| --- | ------ | ---------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Wait for the window to close | Do not start before 2026-10-29. List the workflow-changing merges between 2026-10-15 and 2026-10-28 (see Out of Scope), and record them.                                                                                                                    |
| T2  | DONE   | Collect the measurements     | Run the baseline commands of sections 4 to 7 for the window, plus the V1 and V4 classifications. Recount the rebase-fallout baseline for 1 to 13 October only. Write the raw per-PR results as a table in `post-change-evidence.md`, not as generated JSON. |
| T3  | DONE   | Compare and conclude         | Baseline vs window table normalized per merged PR and per day, a short conclusion, and any caveats (sampling, workflow changes, PR volume). If the result argues for a policy change, draft a new issue and link it here.                                   |

## Commit Points

| Task  | Coherent change set                       | Commit policy                                         |
| ----- | ----------------------------------------- | ----------------------------------------------------- |
| T2+T3 | `post-change-evidence.md` and spec update | Commit after `linter all` passes and maintainer read. |

T1 changes no files. Use a Conventional Commit with the `docs(issues)` scope, and sign every commit
with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/measure-effect-of-optional-rebases/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-15 17:00 UTC - AI assistant - Drafted from #2507 task T7 and acceptance criterion AC9, after
  the maintainer decided to move the two-week measurement out of #2507 so it could close.
- 2026-10-16 06:12 UTC - AI assistant - Maintainer approved the draft; created issue #2547 and moved
  the spec to `docs/issues/open/`. It ships with the #2507 close-out pull request; no separate
  spec-only PR.
- 2026-10-16 06:40 UTC - AI assistant - Applied the Task Reviewer's findings of the #2507 close-out to
  this specification: "for example" before the list of recommended rebases, the corrected creation
  time of this issue, and the progress-log wording.
- 2026-10-16 08:25 UTC - AI assistant - Processed the review of PR #2550 on this specification: the
  rebase-fallout baseline is recounted for 1 to 13 October only, the refresh-path cutoff is `M^1`,
  force-pushes that are not rebases form a third class (listed among the confounders), and PR #2531's
  merge time is the one GitHub reports. The GitHub issue body was updated to match - audit
  `docs/pr-reviews/pr-2550-review/PR-REVIEW.md`
- 2026-10-16 08:55 UTC - AI assistant - Classify a force-push by whether it moved the base (the
  merge bases of the tip with the pushed-away and the new head), with the tip taken by `merged_at`,
  after the second review round of PR #2550 showed that "the new head is based on the tip" cannot
  tell a rebase from a rewrite. The GitHub issue body was updated to match.
- 2026-10-16 10:24 UTC - AI assistant - Labelled the merge time of PR #2531 as GitHub's `merged_at`
  (the evidence dates merges by Git committer time, so the two differ by three minutes), after the
  second review round of PR #2550.
- 2026-10-16 11:16 UTC - AI assistant - The population of every force-push measure is now the pull
  requests merged in the window, with all of their force-pushes, so the total and the classification
  count the same pushes (#2537 merged in the window, but its required rebase was on 2026-10-14).
  Raised by the third review round of PR #2550. The GitHub issue body was updated to match.
- 2026-10-16 11:51 UTC - AI assistant - Limited that population to the force-push measures, after the
  fourth review round of PR #2550: CI runs are selected by creation date as in baseline section 6,
  and fallout commits by author date as in section 7, each as its baseline section does. The GitHub
  issue body was updated to match.
- 2026-10-16 12:14 UTC - AI assistant - Stated the CI and fallout selection rules as this
  specification's own (runs created in the window; commits whose author date is in the window), after
  the fifth review round of PR #2550 showed that baseline sections 6 and 7 do not state them. The
  GitHub issue body was updated to match.
- 2026-10-16 15:40 UTC - AI assistant - #2507 was closed and its specification archived, so the links
  to its baseline and evidence now point to `docs/issues/closed/2507-stop-requiring-pull-request-rebases/`.
- 2026-10-29 17:30 UTC - AI assistant - Maintainer feedback before implementation: per-day figures use
  inclusive calendar days (14 for the window, 13 for the baseline); the CI sample stays at six runs per
  workflow with a recorded seed; the confounder list covers every merge that changed
  `.github/workflows/**` in the window, not only #2525 (the window already holds the ci-gate merges
  #2574 and #2580 and several other workflow edits).
- 2026-10-30 11:06 UTC - AI assistant - Collected the window data (T1 to T3) and wrote
  `post-change-evidence.md`. Findings: force-pushes per merged PR fell from 1.58 to 0.09, but
  estimated PR CI runner-hours rose from 15.2 to 40.5 per day (3.0 to 4.7 per merged PR). The baseline
  merge count recounts to 68, not 67. The workflow-changing merges are #2572, #2601, #2665 and #2676;
  #2574, #2580 and #2675 change no workflow file.
- 2026-10-30 11:49 UTC - AI assistant - Ran M1 and M2 (at 11:09 to 11:10 UTC) as API cross-checks (no browser in the session),
  all counts and sums matched; recorded in `manual-verification-evidence.md`.
- 2026-10-30 12:45 UTC - AI assistant - At the maintainer's request added section 7 to
  `post-change-evidence.md`, which measures merge bunching: consecutive merges within 15 minutes rose
  from 4% to 41% and the median gap fell from 68 to 23 minutes. The time from the last CI run to merge
  grew, but it does not measure rebase waiting (the baseline merges followed the rebase's CI by design).
- 2026-10-30 12:57 UTC - AI assistant - Set `related-pr` to 2690, the pull request opened from the bot's
  fork at 12:09 UTC.
- 2026-10-30 13:45 UTC - AI assistant - Completed the close-out state after the first review of PR #2690:
  acceptance verification, completion review (`implementation-retrospective.md`), status `in-review`, the
  M1/M2 deviation in the scenario table, and the chronological order of this log. No independent reviewer
  agent was given this folder, so no `agent-review-reports.md` is recorded.

## Acceptance Criteria

- [x] AC1: `post-change-evidence.md` compares the window with the #2507 baseline for force-pushes per
      merged PR, share of merged PRs force-pushed, estimated PR CI runner-hours, and rebase-fallout
      commits, normalized per merged PR and per day. The rebase-fallout baseline is recounted for
      1 to 13 October with the same subject reading, not taken from the whole section 7 list.
- [x] AC2: Every force-push of the pull requests merged in the window, including events before it, is
      classified as required (conflict), recommended (clean), or not a rebase, and the count of merges
      through the recorded-base refresh is reported.
- [x] AC3: Every number can be reproduced from commands recorded verbatim in the evidence.
- [x] AC4: The conclusion states whether the policy reduced each measure, names the caveats, and
      links a new issue if a policy change is proposed.
- [x] `linter all` exits with code `0`
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                     | Human-oriented command/steps                                                                                                   | Expected Result                                       | Status | Evidence                                     |
| --- | ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Spot-check force-push counts | Pick three merged PRs from the window and count "force-pushed" entries in each PR's GitHub conversation timeline in a browser (executed through the REST timeline API, no browser was available). | The counts match the per-PR table.                    | DONE   | `manual-verification-evidence.md` section V1 |
| M2  | Spot-check a CI sample       | Open one sampled workflow run per workflow in the GitHub Actions UI and add up its job durations (executed with `gh run view --json jobs`, not the UI).                              | The sum matches the job-minutes recorded for the run. | DONE   | `manual-verification-evidence.md` section V2 |

Notes:

- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
- M1 and M2 were executed as API cross-checks on 2026-10-30; the deviation from the browser and UI
  checks is recorded in `manual-verification-evidence.md`.

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | `post-change-evidence.md` sections 1, 2, 4, 5 and 9 |
| AC2   | DONE                   | `post-change-evidence.md` sections 2.1 and 3        |
| AC3   | DONE                   | `post-change-evidence.md` Appendix A and the commands of each section |
| AC4   | DONE                   | `post-change-evidence.md` sections 10 and 11        |

## Risks and Trade-offs

- **Small samples.** Two weeks of merges and six sampled runs per workflow give estimates, not exact
  figures. Mitigation: report sample sizes and use the same sampling as the baseline.
- **Confounders.** PR volume, the mix of documentation and code PRs, and #2525 workflow changes all
  affect CI time. Mitigation: normalize per merged PR, report the documentation and code split, and
  record any workflow changes in the window.
- **Policy change on rewrites.** PR #2531 (`merged_at` 2026-10-14 16:09 UTC) allows force-pushes that are
  not rebases, so the window's force-push count can exceed the baseline's rebase count for a reason
  unrelated to the rebase policy. Mitigation: classify each force-push (not a rebase) and report both
  the total and the rebase-only figures.
- **Expired data.** GitHub keeps workflow-run logs for a limited time. Mitigation: start soon after
  2026-10-29.

## Implementation Completion Review

- Retrospective: [`implementation-retrospective.md`](implementation-retrospective.md) records the baseline
  population, median, merge-list and reproducibility discoveries of the first review round.

## References

- Related issues: #2507 (origin, baseline), #2525 (possible confounder)
- Related documents:
  [`baseline-evidence.md`](../../closed/2507-stop-requiring-pull-request-rebases/baseline-evidence.md),
  [`manual-verification-evidence.md`](../../closed/2507-stop-requiring-pull-request-rebases/manual-verification-evidence.md)
