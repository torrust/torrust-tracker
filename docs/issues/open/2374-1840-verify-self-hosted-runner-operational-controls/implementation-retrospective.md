---
semantic-links:
  related-artifacts:
    - ISSUE.md
    - ../../closed/2323-1840-hetzner-self-hosted-ci-runner/manual-verification-evidence.md
    - ../../../self-hosted-runner.md
---

# Implementation Retrospective: #2374 Operational Controls

## Outcome

The follow-up verified runner-offline recovery, Dependabot and external-contributor routing, and
the secrets boundary. Evidence is recorded in #2323's V5 and V7, and the routing guide now states
the approval-based fork behavior accurately.

## What Went Well

1. Durable GitHub run and job identifiers made the M5 queue and recovery result recoverable after
   transient UI state was no longer available.
2. The independent review exposed unsupported completion claims before the task was closed.

## What Changed During Implementation

The planned maintainer merge update on Dependabot PR #2369 created a pull-request workflow but no
branch-push workflow. Its `Docker E2E` job was skipped because the PR targeted `develop`. A signed
temporary Rust doc-comment commit triggered the required branch-push route, and a signed immediate
revert restored the source content. The observation therefore proves GitHub-hosted routing, not
`Docker E2E` test success.

The review also corrected the operations guide: an approved external fork targeting `develop` can
select the self-hosted runner. Approval is a control point, not a rule that forces GitHub-hosted
routing.

## Root Cause

The verification plan assumed a Dependabot PR merge update would exercise the branch-push path and
did not separate pull-request job conditions from branch-push job conditions. It also generalized
fork routing from the unapproved state without checking the approved-workflow behavior.

## Improvements for Future Work

1. For every routing scenario, list the required GitHub event, workflow trigger, and job-level
   condition before beginning the observation.
2. Record whether routing evidence establishes runner assignment, job completion, or both; do not
   infer one from the other.

## Avoiding Overcorrection

No permanent workflow trigger or production-code change is justified. The temporary trigger was a
controlled observation and was immediately reverted. Capacity and server-sizing work remain a
separate follow-up.

## Evidence

- [#2374 issue specification](ISSUE.md)
- [#2323 manual verification evidence](../../closed/2323-1840-hetzner-self-hosted-ci-runner/manual-verification-evidence.md)
- [Self-hosted runner operations guide](../../../self-hosted-runner.md)
- Signed trigger and restoration commits: `11180ff9`, `cd4cdfba`
- GitHub Actions run `36631486290`, job `109621539097`
