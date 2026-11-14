---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/closed/2855-container-regression-timeout-sigint/ISSUE.md
    - docs/issues/closed/2855-container-regression-timeout-sigint/manual-verification-evidence.md
    - contrib/dev-tools/containers/tests/test-mounted-no-persistence-configuration.sh
    - .github/skills/dev/planning/create-issue/SKILL.md
---

# Implementation Retrospective — Issue #2855

## Purpose

Record evidence-based process improvements discovered while fixing #2855 (Container regression
script fails when `timeout` is uutils). This is a blameless review of the implementation approach.

## Outcome

`run_tracker` now runs `timeout --foreground --signal=INT --kill-after=3s 10s docker run ...`, so
the tracker receives SIGINT under both GNU and uutils `timeout`, and the script prints the
unexpected exit status when it fails. The merged script passes locally under both
implementations (`manual-verification-evidence.md` V3 rerun, on `develop` at `24230dd01`), and
PR #2857's `Test (Docker) (release)` job passed on its final head, which includes the review-fix
commit, before it merged into `develop`.

## What Went Well

1. Isolating the cause with a matrix of signal paths (`docker kill`, `kill -INT`, GNU and uutils
   `timeout`, with and without `--foreground`; evidence V2) cleared the tracker's shutdown code
   quickly and pointed at a single invocation.
2. The runner's apt history (evidence V4) explained why a docs-only diff turned the workflow red:
   an unattended `rust-coreutils` upgrade between the last green and first red runs.

## What Changed During Implementation

- The implementation PR (#2857) was opened before this spec-only PR (#2856) merged, against the
  spec-first sequence in `create-issue`. The fix itself changed after the spec review began: PR
  #2857's review-fix commit, `fix(container): [#2855] address PR #2857 review findings F2-F4,F7`,
  replaced the exit-status `test` with a `case` that prints the unexpected status. V3 had been run
  before that commit, so it was rerun on the merged script (V3 rerun). The spec PR also needed many
  review rounds to reconcile its checkpoints, evidence and wording with an implementation that had
  already moved on (including merging first).
- Leftover containers on the runner were found while gathering evidence (V5): failed runs leave
  `docker run --rm` containers running, and the daily prune only removes stopped ones. Cleaning
  that backlog was kept out of scope because it changes shared-runner state.
- Evidence first relied on a working-tree copy of the script and unqualified line numbers, which
  went stale once the fix merged; it now names the pre-fix script by its `develop` commit
  `ec1f58ec9`.

## Root Cause

- The regression came from a host-tool behavior difference (uutils `timeout` without
  `--foreground`), and the self-hosted runner upgrades packages unattended, so CI behavior can
  change with no repository change.
- Running spec and implementation PRs in parallel meant every implementation milestone (job
  passing, merge, issue closing) forced another round of spec edits.

## Improvements for Future Work

1. Finish and merge the spec-only PR before opening the implementation PR, as `create-issue`
   already recommends; the maintainer has confirmed this as the expected sequence.
2. Repeat the final recheck after any review-driven change to the fix, as the `fix-bug` skill
   asks, and name the commit it ran on.
3. In evidence, name pre-fix code by a durable `develop` commit id (`git show <rev>:<path>`), not
   by a working-tree copy or a line number in a file the fix will change.
4. In scripts that wrap `docker run` with `timeout`, pass `--foreground` so the signal reaches the
   `docker` client regardless of the `timeout` implementation.

## Avoiding Overcorrection

No new rule for pinning runner packages or disabling unattended upgrades is justified by one
incident; the script fix makes it independent of the `timeout` implementation. Cleaning up the
leftover containers is a one-off maintenance action, not a new process.

## Evidence

- `docs/issues/closed/2855-container-regression-timeout-sigint/ISSUE.md`
- `docs/issues/closed/2855-container-regression-timeout-sigint/manual-verification-evidence.md`
  V1-V5
- PR #2857: `fix(container): [#2855] stop the tracker with timeout --foreground`
- `docs/pr-reviews/pr-2856-review/PR-REVIEW.md`
