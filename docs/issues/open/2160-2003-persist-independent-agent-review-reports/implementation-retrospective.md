---
semantic-links:
  related-artifacts:
    - issue #2160
    - .github/agents/task-reviewer.agent.md
    - .github/agents/pr-reviewer.agent.md
    - contrib/dev-tools/checks/tests/test-agent-review-report-contract.sh
---

# Implementation Retrospective — Issue #2160

## Purpose

Record the material commit-ownership correction found while exercising the new independent-review
report workflow on a real pull request.

## Outcome

The workflow now explicitly keeps commit authority with the caller and Committer for every
independent reviewer. The structural contract test verifies the rule for Task Reviewer and PR
Reviewer as well as the existing Complexity Auditor boundary.

## What Went Well

1. Sequential Complexity Auditor, Task Reviewer, and PR Reviewer entries exposed a gap that a
   template-only review would not have found.
2. The failed review reports were retained as append-only evidence and directly guided the
   correction.

## What Changed During Implementation

The initial implementation prohibited Complexity Auditor from invoking Committer but did not state
the equivalent boundary for Task Reviewer and PR Reviewer. Both profiles now prohibit invoking
Committer or self-committing a report, and require the caller to request a Committer-handled commit
when a report changes a branch or pull-request worktree. The contract test now verifies those two
profiles.

## Root Cause

The first contract test focused on the auditor's narrow no-modification role and treated the other
reviewers' existing `agent` capability as sufficient context. This left the ownership transition
implicit instead of expressing and testing it for each report-producing reviewer.

## Improvements for Future Work

1. Exercise profile workflows with their real prerequisites, including an actual PR for PR Reviewer.
2. Test every shared policy for every participating profile, not just a representative profile.

## Avoiding Overcorrection

Do not remove `agent` capability from Task Reviewer or PR Reviewer. Their workflows legitimately
use it for tasks other than committing; the explicit workflow rule is the narrow commit boundary.

## Evidence

- Issue specification: `docs/issues/open/2160-2003-persist-independent-agent-review-reports/ISSUE.md`
- Review record: `docs/issues/open/2160-2003-persist-independent-agent-review-reports/agent-review-reports.md`
- Contract test: `contrib/dev-tools/checks/tests/test-agent-review-report-contract.sh
