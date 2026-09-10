---
doc-type: issue
issue-type: task
status: draft
priority: p3
epic: 2190
github-issue: null
spec-path: docs/issues/drafts/2190-archive-closed-issue-specifications/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-10 09:45
semantic-links:
  skill-links:
    - create-issue
    - cleanup-completed-issues
  related-artifacts:
    - .github/skills/dev/planning/cleanup-completed-issues/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/open/README.md
    - docs/issues/closed/README.md
    - docs/issues/open/2190-maintenance-frictions-cleanup/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Archive the issue specifications of closed issues

**Parent EPIC:** #2190 - Repository maintenance frictions clean-up

## Goal

Leave `docs/issues/open/` containing a specification directory only for issues GitHub reports as open, so the directory listing is a usable picture of the active backlog.

## Background

Verified at revision `f6b73e29` on 2026-09-09 by checking every directory in `docs/issues/open/` against GitHub. Sixteen specification directories belong to issues GitHub reports as closed: 1586, 1588, 2121, 2122, 2130, 2132, 2134, 2136, 2138, 2140, 2150, 2151, 2155, 2156, 2160, and 2162.

The archival rule is stated in `docs/issues/closed/README.md` and the procedure in the `cleanup-completed-issues` skill; both are already correct. What is missing is a pass that applies them. Until it is applied, anyone reading the open backlog by listing the directory over-counts it by sixteen, and an agent choosing work from that listing can pick something already delivered.

Automating this flow is #1774, which is paused behind EPIC #2003's architecture decision. This subissue does the current pass by hand and constrains nothing in that script's design.

## Scope

### In Scope

- Re-verify every directory under `docs/issues/open/` against GitHub at implementation time, rather than trusting the list above.
- Move each directory whose issue is closed into `docs/issues/closed/`, following the `cleanup-completed-issues` skill.
- Repair the live references the moves break.

### Out of Scope

- Automating the archival flow. That is #1774.
- Stage-two deletion of old specifications from `docs/issues/closed/`.
- Migrating legacy single-file specifications to the folder-style layout. That is #2159.
- Editing the content of any moved specification beyond the reference repairs the move forces.

## Architectural Decisions

No architectural decision is expected. The lifecycle rule and the procedure both already exist; this applies them.

- Related ADRs: `None`
- ADRs to create: `None known`

## Design and Ownership Review

`Not applicable`. Directory moves and reference repairs only.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | TODO | Re-verify the state of every open specification directory | Each directory under `docs/issues/open/` is checked against GitHub and recorded as open or closed, with the check output kept as evidence. |
| T2 | TODO | Move the closed specifications | Every directory whose issue is closed is under `docs/issues/closed/`; no directory whose issue is open moved. |
| T3 | TODO | Repair references the moves break | `linter lychee` exits 0 after the moves. |
| T4 | TODO | Final verification and acceptance review | `linter all` exits 0 and every acceptance criterion is re-reviewed against observed behaviour. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T2 | The directory moves. | Commit on their own. This is a large diff of pure moves; mixing anything else into it would bury the other change. |
| T3 | The reference repairs. | Commit separately so the moves stay reviewable as moves. |
| T4 | Completion evidence. | Keep separate from the moves so the verification record is reviewable on its own. |

Record a justified no-change decision in the task's evidence without creating an empty commit. Use a Conventional Commit message with the narrow affected scope.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2190-archive-closed-issue-specifications/ISSUE.md`
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

- 2026-09-10 09:30 UTC - Specification author - Drafted from the verified evidence in EPIC #2190; inventory item F2 - https://github.com/torrust/torrust-tracker/issues/2190

## Acceptance Criteria

- [ ] AC1: `docs/issues/open/` contains a specification directory only for issues GitHub reports as open, verified issue by issue at merge time.
- [ ] AC2: Every moved directory maps to an issue GitHub reports as closed.
- [ ] AC3: `linter lychee` exits 0 after the moves.
- [ ] AC4: No moved specification's content changed beyond the reference repairs the move forced.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `linter lychee`, specifically after the moves, to catch references they break

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Every remaining directory is an open issue | For every directory left in `docs/issues/open/`, query its issue state on GitHub | Every remaining directory maps to an open issue | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Every moved directory is a closed issue | For every directory moved into `docs/issues/closed/`, query its issue state on GitHub | Every moved directory maps to a closed issue | TODO | `manual-verification-evidence.md` section V2 |
| M3 | The re-verification is repeated at merge time | Repeat the first two scenarios on the merge commit rather than on the branch head | The two lists still hold; any issue that closed while the pull request was open is either included or explicitly deferred | TODO | `manual-verification-evidence.md` section V3 |

Notes:

- Manual verification is mandatory even when automated tests pass. It is a real human-oriented use of the repository's tooling, not a simulated result and not merely running automated tests.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing these scenarios. Record actual prerequisites, actions, commands, program output, relevant logs, and outcomes there.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

A throwaway loop over the directory listing that queries each issue's state is acceptable here, because the scenario is a bulk state query rather than product behaviour and a maintained Rust test would have to reach GitHub to be meaningful. If one is written, keep it in this specification's directory, record what it verifies and who removes it, and prefer the repository's existing tooling over a new script.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | M1 and M3 |
| AC2 | TODO | M2 |
| AC3 | TODO | Automatic checks |
| AC4 | TODO | {PR link} |

## Risks and Trade-offs

- The list is a snapshot: issues close while the pull request is open, so it can be stale by the time it merges. Mitigation: M3 re-verifies at merge time rather than trusting the list drafted here.
- Moving sixteen directories can break inbound links that the link checker does not reach. Mitigation: `linter lychee` runs after the moves, and the reference repairs are a separate reviewable commit.
- A large move diff can hide an accidental content edit. Mitigation: the moves are committed alone, so the diff is reviewable as renames.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory if the moves break references the link checker did not catch, or the re-verification disagrees materially with the drafted list.
- If none of that occurs, add a concise progress-log entry explaining why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #2190 (parent EPIC), #1774 (archival automation), #2159 (folder-style spec adoption)
- Related PRs: #2193 (the EPIC specification that produced this draft)
- Related ADRs: `None`
