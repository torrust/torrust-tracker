---
doc-type: issue
issue-type: task
status: draft
priority: p3
epic: 2190
github-issue: null
spec-path: docs/issues/drafts/2190-resolve-stalled-dependency-update-pull-requests/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-10 09:45
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/open/2190-maintenance-frictions-cleanup/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Resolve the two dependency update pull requests stalled since August

**Parent EPIC:** #2190 - Repository maintenance frictions clean-up

## Goal

Bring the two long-stalled dependency update pull requests to a decision, either merged after a rebase or closed on the merits of the bump, so neither sits indefinitely as an unread failure.

## Background

This issue changes no file in this repository. It is a decision about two open pull requests, recorded as an issue so it has an owner and a visible state.

Verified on 2026-09-09. Pull request #2055 raises `base64` from 0.22.1 to 0.23.1 and was opened on 2026-08-07; #2106 raises `syn` from 2.0.119 to 3.0.4 and was opened on 2026-08-27. Comparing each head against `develop` puts `develop` 263 and 137 commits ahead respectively. Both report an unstable merge state, and on each the only failing check is `Docker E2E`.

That failing check is the symptom #2179 diagnoses and fixes; it is not a property of either bump. Until #2179 merges, a rebase would only reproduce the same failure, so the correct action is to wait rather than to churn the branches.

Both are major-version bumps, so merging is not automatic even once the check is green: each deserves a decision on its own merits.

## Scope

### In Scope

- Wait for #2179 to merge.
- Rebase each pull request onto `develop` and re-run its checks.
- Decide each on its merits: merge it, or close it with the reason recorded.

### Out of Scope

- Fixing the `Docker E2E` job. That is #2179.
- Any change to either branch's dependency choice beyond accepting or rejecting the bump as proposed.
- A wider review of the repository's dependency update policy.
- The allowlist change that blocks a different dependency update, which is a separate subissue under this EPIC.

## Architectural Decisions

No architectural decision is expected from the handling itself. If either major-version bump forces a code change with design consequences, that change belongs to its own issue, not to this one.

- Related ADRs: `None`
- ADRs to create: `None known`

## Design and Ownership Review

`Not applicable`. No implementation work is planned in this repository.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | BLOCKED | Wait for the Docker E2E repair to merge | #2179 is merged and the job passes on `develop`. |
| T2 | TODO | Rebase and re-run both pull requests | Each pull request is current with `develop` and its checks report a real result. |
| T3 | TODO | Decide each bump on its merits | Each pull request is merged, or closed with the reason recorded in this specification's progress log. |
| T4 | TODO | Final acceptance review | Every acceptance criterion is re-reviewed against observed state. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 to T4 | No repository change in this issue's own branch. | Record the outcome in the progress log as a justified no-change decision; do not create an empty commit. Any merge happens in the dependency pull requests themselves. |

This issue produces decisions and evidence rather than a commit of its own.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2190-resolve-stalled-dependency-update-pull-requests/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created, linked as a subissue of the parent EPIC, and issue number added to this spec
- [ ] Specification moved from `docs/issues/drafts/` to `docs/issues/open/` under its assigned number
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-10 09:30 UTC - Specification author - Drafted from the verified evidence in EPIC #2190; inventory item A2 - https://github.com/torrust/torrust-tracker/issues/2190

## Acceptance Criteria

- [ ] AC1: Neither pull request is still open in a stalled state: each is merged or closed.
- [ ] AC2: A closure records the reason the bump was not wanted on its own merits, not the transient check failure.
- [ ] AC3: A merge happened only after the checks reported a real result on a rebased head.
- [ ] AC4: No file changed as part of this issue itself.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- The checks on each dependency pull request, after rebase
- `linter all` on `develop` after any merge

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | The blocking failure is gone | After #2179 merges, re-run the checks on each pull request and read the result | `Docker E2E` reports a real result rather than the failure #2179 diagnoses | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Each pull request reaches a decision | Read the state of both pull requests at the end of the work | Each is merged or closed, with the reason visible | TODO | `manual-verification-evidence.md` section V2 |

Notes:

- Manual verification is mandatory even when automated tests pass. It is a real human-oriented use of the repository's tooling, not a simulated result and not merely running automated tests.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing these scenarios. Record actual prerequisites, actions, commands, program output, relevant logs, and outcomes there.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None is planned. Both scenarios are reads of pull request state.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | M2 |
| AC2 | TODO | M2 |
| AC3 | TODO | M1 |
| AC4 | TODO | {issue link} |

## Risks and Trade-offs

- Waiting on another issue can turn into waiting indefinitely. Mitigation: T1 names the dependency explicitly and this issue stays blocked and visible rather than silently open.
- Major-version bumps that sat for months may no longer be the current version by the time they can run. Mitigation: the decision step accepts closing a stale bump in favour of a fresh one, with the reason recorded.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory if either bump turns out to require code changes with design consequences, or the rebase reveals a failure unrelated to the one diagnosed in #2179.
- If none of that occurs, add a concise progress-log entry explaining why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #2190 (parent EPIC), #2179 (the failing check both pull requests are blocked behind)
- Related PRs: #2055, #2106, #2193 (the EPIC specification that produced this draft)
- Related ADRs: `None`
