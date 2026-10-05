---
doc-type: implementation-retrospective
issue-spec: docs/issues/closed/2402-1840-cancel-superseded-pr-runs/ISSUE.md
last-updated-utc: 2026-10-04 19:07
semantic-links:
  related-artifacts:
      - docs/issues/closed/2402-1840-cancel-superseded-pr-runs/ISSUE.md
      - docs/issues/closed/2402-1840-cancel-superseded-pr-runs/manual-verification-evidence.md
---

# Implementation Retrospective - Issue #2402

## Outcome

The merged workflow-level concurrency policy cancelled both obsolete pull-request target workflows
and retained both target workflows for overlapping `develop` pushes. Manual evidence is recorded in
`manual-verification-evidence.md`.

## What Went Well

1. The separate control pull request demonstrated cancellation isolation.
2. The `github.run_id` push fallback produced distinct workflow groups: both overlapping push runs
   completed successfully.

## What Changed During Implementation

The planned upstream-push overlap did not occur in the available merge sequence, so M2 used the
approved fork fallback. Empty commits do not trigger the target workflows because both workflows
ignore documentation-only paths and GitHub treats an empty commit as having no changed paths.
The valid fallback used two configuration-fixture comment changes instead.

The M2 expected-result wording also included upstream `Publish (Development)` success. That job is
explicitly gated to `torrust/torrust-tracker`, so the approved fork fallback can validate the
concurrency behavior but cannot validate publication.

## Root Cause

The fallback did not distinguish behavior shared by the fork from repository-gated publication, and
it did not require a path-changing successor commit.

The first M1 record also cited a control run that had already completed. Independent review exposed
the timing gap before close-out, and the scenario was rerun with an actually concurrent control PR.
That rerun proved running-run cancellation but not queued-run cancellation. The later attempted
queued-run scenario observed queued workflow runs, but independent review found that both target
jobs had already started. The final scenario used the meaningful GitHub queued-job predicate for
the self-hosted `Container` job: `status: queued` with no assigned runner. It then demonstrated
the expected queued-container cancellation.

## Improvements for Future Work

1. State which expected results are validated by a fork fallback and which require an upstream run.
2. For workflow tests subject to path filters, require two minimal, path-changing commits rather than
   allowing an empty successor commit.
3. Capture each control run's status and timestamp immediately before the action under test.
4. For self-hosted GitHub Actions jobs, treat `status: queued` plus no assigned runner as the
   executable queue predicate; `started_at` can be populated before the job is assigned a runner.

## Avoiding Overcorrection

No new test workflow or permanent fixture is justified. The existing manual verification approach is
adequate once its fallback constraints are explicit.

## Evidence

- `manual-verification-evidence.md`
- [#2402](https://github.com/torrust/torrust-tracker/issues/2402)
- [#2419](https://github.com/torrust/torrust-tracker/pull/2419)
