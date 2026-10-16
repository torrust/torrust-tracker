---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/open/2507-stop-requiring-pull-request-rebases/ISSUE.md
    - docs/issues/open/2507-stop-requiring-pull-request-rebases/manual-verification-evidence.md
    - docs/adrs/20261013161419_stop_requiring_pull_request_rebases.md
---

# Implementation Retrospective — Stop Requiring Pull-Request Rebases

## Purpose

Record evidence-based process improvements discovered while implementing issue #2507. This is a
blameless review of the implementation approach; it does not replace acceptance-criteria
verification.

## Outcome

Rebasing onto `develop` is recommended, not required (ADR `20261013161419`). The maintainer merge
tool refreshes a stale recorded base with a no-op base edit instead of asking for a rebase
(PR #2526). The force-push ban is active again in its own ruleset, the unsatisfiable and abandoned
rulesets are deleted, and `docs/github-rulesets.md` documents the active configuration. In the 18
merges from PR #2526 to #2543, nine pull requests merged while behind `develop` without a rebase at merge
time, and six of them went through the refresh. No ruleset was disabled. See
`manual-verification-evidence.md`, section "Merges Under the New Policy".

## What Went Well

1. The fork sandbox decision gates (T1, T5) tested GitHub's behavior before the design depended on
   it, and they overturned two assumptions of the specification (below).
2. Deleted rulesets were saved as JSON under `evidence/` before deletion, so each one can be
   recreated exactly.
3. The manual scenarios ran on real maintainer merges instead of throwaway pull requests. One day
   of normal merging covered M1 to M6 and M8, including two conflict rebases (M4).

## What Changed During Implementation

- **GitHub's merge condition.** The specification and the ADR assumed that GitHub accepts the
  maintainer's merge push when `refs/pull/<n>/merge` is fresh. The sandbox showed that the condition
  is the pull request's recorded base (`base.sha`) matching the target tip. The merge tool's guard
  was checking the wrong signal. The ADR was corrected, and the tool now refreshes the recorded base
  itself (T1, T2).
- **Required checks.** T5 planned to make the existing required-checks ruleset satisfiable. The
  sandbox showed that GitHub enforces required checks on the direct merge push and that a commit
  status cannot override a failing Actions check run. A satisfiable design needs one
  always-reporting gate with an override, which is substantial CI work, so it became issue #2525
  (T8). Until then no check is required, and AC6 was accepted on that basis.
- **Time-gated measurement.** T7 and AC9 needed two weeks of merges under the new policy, which
  would have kept #2507 open after everything else was done. They moved to follow-up issue
  #2547.
- **Unrecorded tool output.** M1 expects the merge tool to print `Refreshed the recorded base`, but
  nobody captured the output during the real merges. M1 was inferred from GitHub state:
  each head was pushed before `develop` reached the recorded base, yet the merge push was accepted.

## Root Cause

- The baseline inferred GitHub's merge condition from the existing guard (PR #2229) instead of
  testing it. The specification did contain a decision gate (T1), which caught the error before the
  design depended on it.
- The issue template has no rule for a post-change measurement that needs a waiting period. Putting
  it inside the implementation issue made closure depend on the calendar.
- The manual scenarios said they would run "during real maintainer merges" but not who records the
  tool output, or when.

## Improvements for Future Work

1. When a specification depends on an external platform's behavior (GitHub rulesets, refs, checks),
   keep a sandbox decision gate before the design, as T1 and T5 did here.
2. Plan a post-change measurement that needs a waiting period as a separate follow-up issue from the
   start, so the implementation issue can close when its own work is verified. This is a candidate
   note for the `create-issue` skill, pending maintainer approval.
3. When a manual scenario is verified during real merges, record the tool output in the evidence at
   merge time, or state in the scenario that GitHub state is the evidence.

## Avoiding Overcorrection

No script or hook is needed to capture merge-tool output: GitHub state supported the M1 inference
well enough. Do not require a sandbox for every workflow change; it paid off here because the design
depended on undocumented GitHub behavior.

## Evidence

- [`ISSUE.md`](ISSUE.md): implementation plan, progress log, and closure plan
- [`manual-verification-evidence.md`](manual-verification-evidence.md): sections T1, T5, and
  "Merges Under the New Policy"
- [`baseline-evidence.md`](baseline-evidence.md)
- PR #2508 (specification and ADR) and PR #2526 (implementation)
