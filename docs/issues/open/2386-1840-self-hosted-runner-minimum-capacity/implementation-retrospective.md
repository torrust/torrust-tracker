---
semantic-links:
  related-artifacts:
    - ISSUE.md
    - manual-verification-evidence.md
    - ../../../self-hosted-runner.md
    - docs/pr-reviews/pr-2383-review/PR-REVIEW.md
    - docs/pr-reviews/pr-2403-review/PR-REVIEW.md
    - "issue #2402"
---

# Implementation Retrospective: #2386 Minimum Self-Hosted Runner Capacity

## Outcome

The realistic pull-request build was measured on the runner host in one maintainer-approved window:
639 s at 8 vCPU / 16 GB (projected job 801 s) and 1065 s constrained to 4 vCPU / 8 GB (projected
job 1227 s), against a 15-minute target. The maintainer kept the current server and one runner, and
chose to cancel superseded pull-request runs (#2402) before adding a second server. The operations
guide records the server facts, the cost trade-off, the one-runner-per-server limit, and the
recheck commands (PR #2389); the measurements, the 30-day queue replay, and the decision are in
`manual-verification-evidence.md` and the Decision section (PR #2403).

## What Went Well

1. Replaying 30 days of real `Container` run arrivals, calibrated against an observed busy day,
   turned the capacity question from an impression into numbers, and showed that cancelling
   superseded runs is a larger lever than a second server.
2. Measuring both sizes from one warm BuildKit cache in a single window gave comparable results;
   the runner was out of service for 84 minutes and no job was delayed.
3. Copilot's review of PR #2403 caught a wrong queue model before merge. The corrected model
   strengthened the decision instead of reversing it.

## What Changed During Implementation

- **Workload assumption.** A first decision to keep one runner and defer the issue rested on two
  quiet days. The maintainer's normal workload (about 4 open pull requests, checks rerun 3 to 5
  times) is much heavier; the replay then showed a one-runner 90th-percentile wait of 99 to 140
  minutes, and the "concurrency is not needed" conclusion was withdrawn.
- **Cancellation model.** The first cancelling replay dropped a run only when a newer run on its
  branch arrived within one job time, so it missed runs superseded while queued and treated
  cancelled running jobs as free. The event-driven replay corrected the one-runner figures at
  801 s from 12 to 11 minutes (90th percentile) and from 79 to 32 minutes (maximum)
  (`review-finding:pr-2403-f1`).
- **Memory evidence.** V2 first claimed the constrained build ran at its memory limit "throughout";
  the samples covered only its last 5 minutes and touched the limit in 14 of 262
  (`review-finding:pr-2403-f2`).
- **Window mechanics.** Git run as `root` refuses the Actions checkout owned by `runner`; Buildx
  needs `-f Containerfile`; writing `0` to the cgroup peak counters did not reset them; and
  `pgrep -f` matched its own command line, so a watch loop and a memory sampler never ended.
- **Daily prune.** It was first suspected of not bounding the build cache; it trims to 120 GiB once
  a day as configured, and the guide's wording was corrected.
- **Lifecycle.** The first pull request (#2383) used a branch named after the EPIC and was replaced
  by PR #2389. That PR's body said "The link will change to `Closes #2386` ..."; the merge tool
  copies the body into the merge commit, and GitHub closed #2386 on merge, so it was reopened.

## Root Cause

- The specification sized the runner from job time alone; demand was not modelled until the
  maintainer described the normal workload, so a quiet sample stood in for it.
- The queue replay was validated against observed data only for the no-cancellation case. The
  cancellation policy was written as a pre-filter in a one-liner and never checked against a case
  with a known answer.
- Monitoring for the window was improvised while the runner was down, not prepared and tried
  beforehand.
- The pull-request skill warned against closing keywords only in the issue-link line, not in prose
  elsewhere in the body.

## Improvements for Future Work

1. For capacity or queueing decisions, model demand from at least 30 days of real arrivals and the
   maintainer's normal workflow before concluding; quiet days are not a baseline.
2. When a simulation informs a decision, implement each policy as a state model, not a filter over
   inputs, and check every mode against a case with a known result before using its numbers.
3. For a maintenance window, prepare and dry-run the monitoring commands before stopping the
   service: match processes with a `pgrep -f` pattern that cannot match its own command line (wrap
   one character in brackets), sample from the start of each run, and watch from the desktop with
   short remote checks.
4. Never write a closing keyword followed by an issue reference anywhere in a pull-request body,
   even in a sentence about a later change. Applied in the `open-pull-request` skill in the
   close-out pull request.

## Avoiding Overcorrection

No benchmark tool, reusable measurement script, or ADR is justified: the measurement is a one-off
and the commands are recorded in the evidence. Simulations are not required for every decision,
only for decisions that rest on one. A second server is not justified until the recheck after #2402.

## Evidence

- [#2386 issue specification](ISSUE.md), Decision section
- [Manual verification evidence](manual-verification-evidence.md), V1 to V4
- [Self-hosted runner operations guide](../../../self-hosted-runner.md)
- PR #2389 (spec, guide, first queue evidence) and PR #2403 (measurements and decision)
- Review audits: `docs/pr-reviews/pr-2383-review/PR-REVIEW.md`,
  `docs/pr-reviews/pr-2403-review/PR-REVIEW.md`
- Follow-up: #2402 (cancel superseded pull-request runs)
