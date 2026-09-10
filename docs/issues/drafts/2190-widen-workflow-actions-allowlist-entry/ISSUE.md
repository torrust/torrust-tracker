---
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2190
github-issue: null
spec-path: docs/issues/drafts/2190-widen-workflow-actions-allowlist-entry/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-10 09:45
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/maintenance/update-github-workflow-actions/SKILL.md
    - .github/workflows/testing.yaml
    - .github/workflows/coverage.yaml
    - .github/workflows/generate_coverage_pr.yaml
    - docs/issues/open/2190-maintenance-frictions-cleanup/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Widen the allowed-actions allowlist entry that pins an exact patch version

**Parent EPIC:** #2190 - Repository maintenance frictions clean-up

## Goal

Let dependency updates of the install action run at all, by replacing the exact patch version in the repository's allowed-actions allowlist with the version pattern the repository's own skill prescribes.

## Background

This issue changes no file in this repository. It is an administrative change to the repository's GitHub Actions settings, recorded as an issue so it has an owner, an acceptance criterion, and a visible state.

Verified on 2026-09-09. The repository's selected-actions permissions list `taiki-e/install-action@v2.87.2` among its allowed patterns. Three workflows use that action, at `.github/workflows/testing.yaml:80`, `coverage.yaml:51`, and `generate_coverage_pr.yaml:46`, so no automated bump of it can run: the workflow is refused before it starts.

The failure mode is worse than a red check. On pull request #2180, which bumped the action from 2.87.2 to 2.87.6 at head `a90c3bb1`, the runs for `Testing` on push, `Testing` on pull request, and `Generate Coverage Report (PR)` all show `startup_failure`, while the ordinary pull request check view lists eighteen rows that are all passing or skipped. The blocked bump therefore looks green to a reviewer.

The repository's own skill already prescribes the fix, at `.github/skills/dev/maintenance/update-github-workflow-actions/SKILL.md:49`.

## Scope

### In Scope

- A repository administrator changes the allowlist entry from the exact patch version to `taiki-e/install-action@v2.*`, as the maintenance skill directs.
- Confirm the value before and after through the readable permissions endpoint.
- Confirm that a previously blocked bump of that action now produces real workflow runs.

### Out of Scope

- Any change to a file in this repository. The allowlist lives in the repository settings.
- Widening or reviewing other entries in the allowlist.
- Changing the three workflows that use the action.
- Merging or closing the blocked dependency pull requests, which is a separate subissue under this EPIC.

## Architectural Decisions

No architectural decision is expected. The maintenance skill already records the convention this change restores.

- Related ADRs: `None`
- ADRs to create: `None known`

## Design and Ownership Review

`Not applicable`. A settings change with no runtime, ownership, or lifetime dimension.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | TODO | Read and record the current allowlist value | The current patterns are recorded as evidence, showing the exact patch pin. |
| T2 | TODO | Repository administrator widens the entry | The allowlist carries `taiki-e/install-action@v2.*` in place of the exact patch pin. |
| T3 | TODO | Confirm a blocked bump now runs | A re-run of an affected pull request produces workflow runs that are not `startup_failure`. |
| T4 | TODO | Final acceptance review | Every acceptance criterion is re-reviewed against observed behaviour and recorded in the progress log. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 to T4 | No repository change. | Record the outcome in `manual-verification-evidence.md` and as a justified no-change decision in the progress log; do not create an empty commit. |

This issue produces evidence rather than a commit. Should it turn out that a file in this repository does need to change, record why and use a Conventional Commit message with the narrow affected scope.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2190-widen-workflow-actions-allowlist-entry/ISSUE.md`
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

- 2026-09-10 09:30 UTC - Specification author - Drafted from the verified evidence in EPIC #2190; inventory item A1 - https://github.com/torrust/torrust-tracker/issues/2190

## Acceptance Criteria

- [ ] AC1: The repository's allowed-actions allowlist carries a version pattern rather than an exact patch pin for the install action.
- [ ] AC2: The before and after values are both recorded from the permissions endpoint.
- [ ] AC3: An affected dependency pull request produces workflow runs that reach their jobs rather than failing at startup.
- [ ] AC4: No file in this repository changed.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`, to confirm the tree is unchanged and still green

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | The allowlist value changes as intended | Read the repository's selected-actions permissions before the change and again after it | The exact patch pin is replaced by the version pattern | TODO | `manual-verification-evidence.md` section V1 |
| M2 | A blocked bump now runs | Re-run the checks on an affected dependency pull request and list the workflow runs for its head | No run reports `startup_failure`; the jobs start and report a real result | TODO | `manual-verification-evidence.md` section V2 |
| M3 | The ordinary check view agrees | Read the pull request check list for the same head | The check list no longer shows an all-green picture over failed startups | TODO | `manual-verification-evidence.md` section V3 |

Notes:

- Manual verification is mandatory even when automated tests pass. It is a real human-oriented use of the repository's tooling, not a simulated result and not merely running automated tests.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing these scenarios. Record actual prerequisites, actions, commands, program output, relevant logs, and outcomes there.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

- The permissions endpoint is readable with ordinary access, so the value can be confirmed before and after without administrator rights. Making the change itself requires a repository administrator.

### Disposable Verification Scripts

None is planned. Every scenario is a direct read of repository settings or of a workflow run list.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | M1 |
| AC2 | TODO | M1 |
| AC3 | TODO | M2 and M3 |
| AC4 | TODO | {PR link} |

## Risks and Trade-offs

- A wider pattern admits future patch releases of that action without review. Mitigation: that is the convention the repository's own maintenance skill prescribes, and the alternative is a pin that silently blocks every update to the same action.
- The change is invisible in the repository history. Mitigation: this issue is the record, and the before and after values are captured as evidence.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory if the widened pattern admits a release that breaks a workflow, or the permissions endpoint disagrees with what the settings interface shows.
- If none of that occurs, add a concise progress-log entry explaining why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #2190 (parent EPIC)
- Related PRs: #2180 (the bump that showed the failure mode), #2193 (the EPIC specification that produced this draft)
- Related ADRs: `None`
