---
doc-type: issue
issue-type: task
status: draft
priority: p3
epic: 2190
github-issue: null
spec-path: docs/issues/drafts/2190-remove-unquotable-character-type-generic/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-10 09:30
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - contrib/dev-tools/analysis/workspace-coupling/src/main.rs
    - docs/issues/open/2190-maintenance-frictions-cleanup/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Remove the unquotable character-type generic from the workspace-coupling tool

**Parent EPIC:** #2190 - Repository maintenance frictions clean-up

## Goal

Let every line of the workspace-coupling tool survive being quoted in a review discussion, by removing the one explicit generic argument that chat and review transports rewrite.

## Background

Verified at revision `f6b73e29` on 2026-09-09. `contrib/dev-tools/analysis/workspace-coupling/src/main.rs:191` writes an explicit generic argument naming Rust's character type: the five-character token spelled angle bracket, c-h-a-r, angle bracket. It is the only occurrence in the tracked tree.

Chat and review transport layers substitute that token with a role or account name. A reviewer who quotes the line therefore receives text that would not compile, and correctly distrusts the quoted artifact. The defect is not in what the code does; it is that this one line cannot be discussed in the medium where the repository's code is discussed.

The proposed remedy takes the character by value and moves the optionality to the call sites, which already hold values from the adjacent-character lookups at line 187 and can express the same intent without the token.

## Scope

### In Scope

- Change `is_rust_identifier_char` to take the character by value.
- Move the optionality to the two call sites at line 187, keeping the existing meaning that an absent adjacent character reads as not an identifier character.
- Leave the tool's output unchanged.

### Out of Scope

- Any other change to the workspace-coupling tool's behaviour, output format, or structure.
- A repository-wide rule or automated check banning the token. If one is wanted, it is a separate proposal against EPIC #2003.
- Refactoring the adjacent-character lookups beyond what the signature change requires.

## Architectural Decisions

No architectural decision is expected. Where a token-free signature is genuinely impossible in some future case, a local type alias or inference through a collecting call serves the same purpose; that fallback is a coding convention, not an architecture decision.

- Related ADRs: `None`
- ADRs to create: `None known`

## Design and Ownership Review

`Not applicable`. A single synchronous helper function with no ownership or lifetime question beyond taking a copy type by value.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | TODO | Take the character by value and adjust the two call sites | The function signature carries no explicit generic argument; the call sites express the absent-character case explicitly. |
| T2 | TODO | Compare the tool's output before and after | The reports produced on `develop` and on the branch are identical apart from any embedded timestamp. |
| T3 | TODO | Final verification and acceptance review | `linter all` exits 0, Clippy is clean, and every acceptance criterion is re-reviewed against observed behaviour. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | The signature change and its two call sites. | Commit on its own after the output comparison in T2 is clean. |
| T2, T3 | Comparison and completion evidence. | No repository change. Record the comparison in `manual-verification-evidence.md`. |

Record a justified no-change decision in the task's evidence without creating an empty commit. Use a Conventional Commit message with the narrow affected scope.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2190-remove-unquotable-character-type-generic/ISSUE.md`
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

- 2026-09-10 09:30 UTC - Specification author - Drafted from the verified evidence in EPIC #2190; cluster C6, friction F6 - https://github.com/torrust/torrust-tracker/issues/2190

## Acceptance Criteria

- [ ] AC1: The tracked tree contains no occurrence of the explicit character-type generic described above.
- [ ] AC2: `contrib/dev-tools/analysis/workspace-coupling` produces the same report as before the change, apart from any embedded timestamp.
- [ ] AC3: Clippy reports nothing new for the changed file.
- [ ] AC4: No file outside `contrib/dev-tools/analysis/workspace-coupling/` changes.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- Clippy for the workspace-coupling package
- The tool's own run, compared against a run on `develop`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | The tool is unchanged | Run the tool on `develop` and on the branch, and compare the two reports | The reports are identical apart from any embedded timestamp | TODO | `manual-verification-evidence.md` section V1 |
| M2 | The changed line quotes cleanly | Paste the changed line into a review comment draft and read what arrives | The pasted text is the same text as the file holds | TODO | `manual-verification-evidence.md` section V2 |

Notes:

- Manual verification is mandatory even when automated tests pass. It is a real human-oriented use of the repository's tooling, not a simulated result and not merely running automated tests.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing these scenarios. Record actual prerequisites, actions, commands, program output, relevant logs, and outcomes there.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None is planned. The comparison is two runs of the tool and a diff of their output.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | M2 |
| AC2 | TODO | M1 |
| AC3 | TODO | Automatic checks |
| AC4 | TODO | {PR link} |

## Risks and Trade-offs

- Changing a function signature in a clean-up pull request touches behaviour-adjacent code. Mitigation: M1 compares the tool's output before and after; if the comparison is anything but identical, the change is reverted and the item deferred to its own investigation.
- Removing one occurrence does not stop a new one appearing. Mitigation: that is a rule question, explicitly out of scope here, and the tree currently holds exactly one occurrence to remove.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory if the output comparison is not identical, or a token-free signature turns out to require more than the two call-site changes.
- If none of that occurs, add a concise progress-log entry explaining why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #2190 (parent EPIC)
- Related PRs: #2193 (the EPIC specification that produced this draft)
- Related ADRs: `None`
