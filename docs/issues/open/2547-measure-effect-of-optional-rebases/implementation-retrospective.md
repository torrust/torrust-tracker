---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/open/2547-measure-effect-of-optional-rebases/ISSUE.md
    - docs/issues/open/2547-measure-effect-of-optional-rebases/post-change-evidence.md
---

# Implementation Retrospective — Measure the Effect of Optional Pull-Request Rebases

## Purpose

Record evidence-based process improvements discovered while implementing issue #2547. This is a
blameless review of the measurement approach; it does not replace the acceptance-criteria verification.

## Outcome

The 14-day window was measured against the #2507 baseline and recorded in `post-change-evidence.md`:
force-pushes per merged PR fell by about 94%, merges became much more bunched, and the per-PR CI cost
rose together with a change in the mix of documentation and code merges. The first review round of the PR
found populations, a median, and a classification that the first version of the evidence stated
wrongly or too loosely; they were corrected in the PR.

## What Went Well

1. Recording the selection rule of each measure in the specification before collecting data made the
   later recount and the review checks possible.
2. Classifying force-pushes by the merge bases (not by the new head alone) separated the one
   non-rebase rewrite from the nine rebases.

## What Changed During Implementation

1. **The baseline population differed from the recount.** The baseline was collected before the end of
   13 October, so it holds 67 merges; the full window holds 68. The first version of the evidence mixed
   the two and claimed its median reproduced the baseline's, which was true only for 67 merges: the code
   took the upper middle value of an even count, which hid the difference until the review.
2. **A time-limited `git log --since` walk missed two merges** of 1 October, because the walk stops at
   an older commit. They were found by `--grep="#<n>:"` and added to the merge-bunching and mix figures.
3. **The documentation and code split was promised by the specification and still omitted.** The
   per-PR CI cost rose, which the expectation of this issue did not predict, and the omitted split is the
   confounder that most plausibly explains it.
4. **Aggregations were not recorded.** The retrieval commands were, but the commands that turn them into
   tables were not, so AC3 (every number reproducible) did not hold until Appendix A was added.

## Root Cause

The specification fixed what to measure and how to select data, but not the statistics (median of an even
count, which denominator) or the rule that the aggregation step is part of "commands recorded verbatim".
The workload mix was listed as a confounder without a task that reports it.

## Improvements for Future Work

1. A measurement specification should name the statistic and the denominator of each figure, and make the
   per-class split a task, not only a risk.
2. Record the aggregation commands next to the retrieval commands in the first version of the evidence,
   and keep per-run data that a figure rests on in the issue folder.
3. Check a recount against the published baseline population before comparing, and state which of the two
   each headline uses.

## Avoiding Overcorrection

No new repository rule is justified by one measurement task. The improvements above belong in the
specification of a future measurement, not in a global policy.

## Evidence

- `docs/issues/open/2547-measure-effect-of-optional-rebases/ISSUE.md`
- `docs/issues/open/2547-measure-effect-of-optional-rebases/post-change-evidence.md`
- Pull request #2690 and its first review
