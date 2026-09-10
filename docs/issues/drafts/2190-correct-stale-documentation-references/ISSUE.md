---
doc-type: issue
issue-type: task
status: draft
priority: p3
epic: 2190
github-issue: null
spec-path: docs/issues/drafts/2190-correct-stale-documentation-references/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-10 09:10
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - AGENTS.md
    - .github/skills/dev/testing/manual-http-download-completion-e2e/SKILL.md
    - .github/skills/dev/testing/manual-udp-download-completion-e2e/SKILL.md
    - docs/profiling.md
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - project-words.txt
    - docs/issues/open/2150-add-lychee-link-checker/ISSUE.md
    - docs/issues/open/2190-maintenance-frictions-cleanup/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Correct stale documentation references left by completed moves and decisions

**Parent EPIC:** #2190 - Repository maintenance frictions clean-up

## Goal

Leave no live document naming a binary at a path it no longer occupies, and no dictionary entry for a file the project decided never to create.

## Background

Two stale references share one shape: each names a referent that does not exist. Both were verified at revision `f6b73e29` on 2026-09-09.

`src/bin/` now contains only `http_health_check.rs`. The `e2e_tests_runner`, `profiling`, and `qbittorrent_e2e_runner` binaries moved to `packages/e2e-tools/src/bin/` in commit `c47173f53`, and five live references were never updated: `AGENTS.md:39`, `.github/skills/dev/testing/manual-http-download-completion-e2e/SKILL.md:21` and `:307`, `.github/skills/dev/testing/manual-udp-download-completion-e2e/SKILL.md:207`, and `docs/profiling.md:10`. Separately, `docs/adrs/20260519000000_define_global_cli_output_contract.md:8` lists `src/bin/` as a related artifact and now under-describes the binary landscape.

`docs/issues/open/2150-add-lychee-link-checker/ISSUE.md:93` records the decision not to add a `.lycheeignore` file, and `project-words.txt:306` still carries the corresponding dictionary entry. The file does not exist and, by that decision, never will.

The cost of each is small and identical: a reader follows a path that is not there, or a dictionary entry that documents nothing.

## Scope

### In Scope

- Correct the five live references to name `packages/e2e-tools/src/bin/`.
- Extend the ADR's related-artifacts list so it describes the current binary landscape, without rewriting the decision.
- Confirm no remaining prose needs the stale dictionary word, then drop the entry.

### Out of Scope

- Changing anything under `docs/issues/closed/`. Those are immutable historical records.
- Rewriting the decision recorded in the ADR.
- A general audit of the project dictionary, or of documentation paths beyond the ones listed above.
- Revisiting the decision in #2150 not to create the ignore file.

## Architectural Decisions

No architectural decision is expected. The ADR touched here gains one entry in its related-artifacts list; its decision text is unchanged.

- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
- ADRs to create: `None known`

## Design and Ownership Review

`Not applicable`. Documentation and dictionary content only.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | TODO | Correct the five stale binary-path references | Each of the five references names `packages/e2e-tools/src/bin/`, and each named path exists in the tree. |
| T2 | TODO | Extend the ADR related-artifacts list | The list names both `src/bin/` and `packages/e2e-tools/src/bin/`; the decision text is unchanged. |
| T3 | TODO | Retire the stale dictionary entry | `linter cspell` exits 0 with the entry removed, or the entry is kept with a one-line justification in the progress log. |
| T4 | TODO | Final verification and acceptance review | `linter all` exits 0 and every acceptance criterion is re-reviewed against observed behaviour. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1, T2 | The five references and the ADR artifact list. | Commit together. They are one documentation correction and independently revertible as a unit. |
| T3 | The dictionary entry. | Fold into the commit above if it is a one-word edit; commit separately if the word survives and the decision needs its own message. |
| T4 | Completion evidence. | Keep separate when it improves reviewability. |

Record a justified no-change decision in the task's evidence without creating an empty commit. Use a Conventional Commit message with the narrow affected scope.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2190-correct-stale-documentation-references/ISSUE.md`
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

- 2026-09-10 08:43 UTC - Specification author - Drafted from the verified evidence in EPIC #2190; inventory items F4 and F8 - https://github.com/torrust/torrust-tracker/issues/2190

## Acceptance Criteria

- [ ] AC1: No live document, skill, or workflow places `e2e_tests_runner`, `profiling`, or `qbittorrent_e2e_runner` under `src/bin/`.
- [ ] AC2: Every path named by a corrected reference exists in the tree.
- [ ] AC3: Records under `docs/issues/closed/` are unchanged.
- [ ] AC4: The ADR's decision text is unchanged and its related-artifacts list describes the current binary landscape.
- [ ] AC5: The stale dictionary entry is gone, or the progress log records why it stays.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `linter cspell`
- `linter lychee`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Corrected paths point at real files | For each reference changed, read the file at the path it now names | Every path exists | TODO | `manual-verification-evidence.md` section V1 |
| M2 | The profiling document still describes a runnable flow | Follow `docs/profiling.md` from its first command | The described flow runs against the binary at its new location | TODO | `manual-verification-evidence.md` section V2 |
| M3 | No prose still needs the retired dictionary word | Search the tracked tree for the word before removing the entry, then run the spell check | The word appears in no live document, and the spell check exits 0 | TODO | `manual-verification-evidence.md` section V3 |

Notes:

- Manual verification is mandatory even when automated tests pass. It is a real human-oriented use of the repository's tooling, not a simulated result and not merely running automated tests.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing these scenarios. Record actual prerequisites, actions, commands, program output, relevant logs, and outcomes there.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None is planned. Every scenario is a direct read or a spell-check run.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | M1 |
| AC2 | TODO | M1 |
| AC3 | TODO | {PR link} |
| AC4 | TODO | {PR link} |
| AC5 | TODO | M3 |

## Risks and Trade-offs

- A search-and-replace could reach historical records. Mitigation: the five references are listed individually, and an acceptance criterion asserts that `docs/issues/closed/` is untouched.
- The dictionary word may still be needed by a document not found by a naive search. Mitigation: M3 searches before removing, and the specification allows keeping the entry with a recorded reason.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory if the path correction uncovers further live references beyond the five listed, or the profiling flow does not run at the corrected path.
- If none of that occurs, add a concise progress-log entry explaining why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #2190 (parent EPIC), #2150 (the decision behind the retired dictionary entry), #2179 (names this drift as out of its own scope)
- Related PRs: #2193 (the EPIC specification that produced this draft)
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
