---
doc-type: issue
issue-type: task
status: open
priority: p2
epic: null
github-issue: 2230
spec-path: docs/issues/open/2230-add-fix-bug-skill-and-bug-spec-guardrails/ISSUE.md
branch: 2230-add-fix-bug-skill-and-bug-spec-guardrails
related-pr: 2270
last-updated-utc: 2026-09-20 09:20
semantic-links:
  skill-links:
    - add-new-skill
    - create-issue
    - fix-bug
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - .github/agents/implementer.agent.md
    - docs/templates/ISSUE.md
    - "issue #2226"
---

# Issue #2230 - Add a `fix-bug` Skill and Bug-Spec Guardrails

## Goal

Establish one repository-owned workflow for investigating and fixing confirmed bugs, then require bug specifications to plan that workflow and teach the Implementer agent to apply it.

## Background

The confirmed stale activity-metrics cutoff bug exposed a missing explicit repository workflow. Its investigation used a useful sequence: analyse the defect, reproduce it against the real artifact, choose the smallest regression-test boundary, write a failing test, fix the code, and rerun both the automatic regression test and original manual reproduction.

The repo-global authoring workflow still lives under `.github/skills/add-new-skill/SKILL.md`, but the issue-specific workflow guidance below intentionally points only at the `dev/` paths that govern day-to-day tracking and validation. This keeps the taxonomy explicit without creating a second canonical source for the bug-fix process.

Current guidance is incomplete by design. Bug status is determined by the substance of the
reported work, not only by the `issue-type` metadata or GitHub labels; a misclassified issue that
describes broken behavior must still follow the bug-fix workflow.

- `create-issue` governs drafting and publishing all issue types, but has no bug-specific process requirement.
- `write-unit-test` governs test design and deterministic time, but does not prescribe when a bug needs unit, integration, end-to-end, or manual validation.
- Implementer requires TDD where practical, but does not require a bug reproduction or a like-for-like final recheck.
- `docs/templates/ISSUE.md` has no explicit bug-only sections for investigation and regression strategy.

A new canonical skill is preferable to duplicating operational procedure in an agent, generic template, and issue-creation skill. The related stale-cutoff bug specification is the worked example and remains out of scope for this documentation/process change.

## Scope

### In Scope

- Create `.github/skills/dev/debugging/fix-bug/SKILL.md` as the canonical bug-fix workflow.
- Add bug-conditional specification requirements to `create-issue` and `docs/templates/ISSUE.md` as thin references to `fix-bug`.
- Update the Implementer agent to load and apply `fix-bug` for any substantively identified bug,
  even when issue metadata or labels are missing or incorrect.
- Define evidence standards for source analysis, real-artifact reproduction, failing-test output, fixed-test output, and final recheck.
- Define a decision framework for selecting unit, integration, end-to-end, and manual regression verification.
- Add a worked-example reference to the stale activity-metrics cutoff issue.

### Out of Scope

- Changing the implementation or acceptance criteria of the stale activity-metrics cutoff bug.
- Requiring an issue-local reproduction artifact for every task, feature, or enhancement.
- Replacing `write-unit-test`, tracker-run, tracker-client, or testing guidance.
- Adding automated enforcement that rejects issue specs lacking bug-specific sections.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
- ADRs to create: `None known`. This is repository workflow guidance, not a product architecture decision. Create an ADR only if implementation reveals a consequential agent-governance decision not covered by the existing ADR.

## Design and Ownership Review

Not applicable. This work changes repository guidance and agent configuration; it introduces no asynchronous I/O, child process, readiness wait, or reusable runtime fixture.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | DONE | Define the canonical bug-fix workflow | Created `.github/skills/dev/debugging/fix-bug/SKILL.md` using the repository's `add-new-skill` guidance. For any work that is substantively a bug, regardless of metadata or labels, it requires the sequence: analyse, reproduce, select tests, red test, fix, and recheck. |
| T2 | DONE | Define test-selection and evidence rules | Review finding F13 is resolved: regression red proof follows the `write-unit-test` mutate-then-restore rule. |
| T3 | DONE | Link issue authoring guidance and template | Review findings F9, F12, F16, F18, and F19 are resolved by parseable frontmatter, canonical references, and validated links. |
| T4 | DONE | Link the Implementer agent workflow | Review finding F14 is resolved: Implementer preserves the ordered workflow and requires the final like-for-like recheck. |
| T5 | DONE | Validate the workflow end-to-end | Review 5256684050 findings F23-F27 are resolved; every review thread carries a finding-specific reply, all current threads are resolved, and final GraphQL refresh reports zero unresolved threads. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1-T2 | Canonical `fix-bug` skill and its decision/evidence policy | Commit after skill review and relevant validation. |
| T3 | Issue creation guidance and bug-only template sections | Commit separately if it improves reviewability. |
| T4 | Implementer skill reference | Commit separately if agent review identifies ambiguity. |
| T5 | Validation evidence and any worked-example link | Commit separately when it improves reviewability. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted and promoted to `docs/issues/open/2230-add-fix-bug-skill-and-bug-spec-guardrails/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2230 created and issue number added to this spec
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, skill-link validation, and relevant checks)
- [x] Manual review scenarios executed against the finished artifact and evidence recorded
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded for the final implementation
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-09-15 10:40 UTC - GitHub Copilot - Created parked follow-up draft after the confirmed stale activity-metrics cutoff bug established a reusable six-step investigation and regression-prevention process.
- 2026-09-15 11:10 UTC - GitHub Copilot - Promoted the draft after GitHub issue #2226 was created; it is ready for maintainer review.
- 2026-09-15 15:00 UTC - josecelano - Approved this workflow-specification draft for GitHub issue creation - Chat approval.
- 2026-09-15 15:05 UTC - GitHub Copilot - Created GitHub issue #2230 and promoted this specification to `docs/issues/open/` - https://github.com/torrust/torrust-tracker/issues/2230
- 2026-09-18 15:00 UTC - josecelano - Clarified that the workflow applies to any substantively identified bug regardless of metadata or labels, and that infeasible reproduction still requires traceable evidence - Chat feedback recorded in `docs(issues): refine bug-fix workflow requirements`.
- 2026-09-18 15:45 UTC - GitHub Copilot - Implemented the `fix-bug` skill, linked the issue template, `create-issue`, and Implementer agent, corrected the stale #2226 worked-example path, and recorded manual verification in `manual-verification-evidence.md`.
- 2026-09-18 15:45 UTC - GitHub Copilot - Completion review: no separate retrospective needed because implementation followed the approved workflow design without material design changes; the only discovery was a stale worked-example path and relative-link correction, both recorded in this issue and validation evidence.
- 2026-09-18 15:45 UTC - GitHub Copilot Task Reviewer - Independently verified all acceptance criteria, reran `linter all`, checked skill-link resolution, and recorded the review in `agent-review-reports.md`.
- 2026-09-18 17:05 UTC - GitHub Copilot - Addressed Copilot PR review feedback by correcting sibling skill links, using stable issue #2226 references, removing the generic template `fix-bug` skill-link, adding reciprocal long-lived skill markers, and fixing the sample validation artifact path.
- 2026-09-19 08:35 UTC - GitHub Copilot - Reopened T2-T5 and affected completion checkpoints after review 5255047473 found invalid semantic-link frontmatter, workflow-order gaps, and missing canonical PR-review audit/replies; normalized findings F1-F19 in `docs/pr-reviews/pr-2270-review/PR-REVIEW.md` before fixes.
- 2026-09-19 11:50 UTC - GitHub Copilot Task Reviewer - Revalidated AC1-AC6 and automatic checks at current HEAD after F9, F12-F16, F18, and F19 fixes. Readiness remains blocked until the post-F14 Implementer scenario is rerun and the completion review assesses the material follow-up corrections; GitHub thread replies and resolution were not assessed.
- 2026-09-19 11:58 UTC - GitHub Copilot - Reran manual scenario M2 against the corrected Implementer and bug-fix skills, recorded current-head evidence in `manual-verification-evidence.md` V3, and created `implementation-retrospective.md` for the material validation, workflow, and review-process discoveries.
- 2026-09-19 16:20 UTC - GitHub Copilot - Completed the canonical PR-review workflow for reviews 5250020892, 5255047473, and 5255823584: audit F1-F22 updated, every inline thread received a reply before resolution, consolidated response posted at https://github.com/torrust/torrust-tracker/pull/2270#issuecomment-5743396140, and final GraphQL refresh reported zero unresolved threads.
- 2026-09-19 17:20 UTC - GitHub Copilot - Reopened T5 after review 5256684050 raised F23-F24 about two stale current-state audit sentences and an overstatement of historical reply-before-resolution ordering.
- 2026-09-19 17:21 UTC - GitHub Copilot - Corrected the review-processing record for submitted reviews 5250020892, 5255047473, 5255075569, 5255823584, and 5255889360: body-only revalidation rounds were recorded with no new actionable findings, every inline thread carried a finding-specific reply, and zero unresolved threads remained.
- 2026-09-20 08:10 UTC - GitHub Copilot - Reopened T5 after review 5256684050 also raised F25-F27 about missing latest review-round audit rows, inaccurate progress-log timestamps, and an overwritten 16:20 progress-log entry.
- 2026-09-20 09:20 UTC - GitHub Copilot - Completed review 5256684050 follow-up: replied to F23-F27, resolved all remaining threads, recorded F1-F27 in `docs/pr-reviews/pr-2270-review/PR-REVIEW.md`, and final GraphQL refresh reported zero unresolved threads.

## Acceptance Criteria

- [x] AC1: `.github/skills/dev/debugging/fix-bug/SKILL.md` defines the required sequence: analyse, reproduce, select regression test type, write a failing test, fix, and recheck.
- [x] AC2: The skill requires real-artifact reproduction evidence with actual commands and output when reproduction is feasible, requires the reason and attempted evidence to be recorded when it is infeasible, and distinguishes it from maintained automated tests.
- [x] AC3: The skill requires selection of the smallest deterministic regression-test boundary, with a documented rationale for integration or end-to-end tests.
- [x] AC4: `create-issue` and `docs/templates/ISSUE.md` require bug-only `Bug-Fix Process` and `Regression Test Strategy` sections that link to the canonical skill instead of duplicating it.
- [x] AC5: The Implementer agent loads `fix-bug` for substantively identified bug specs, even when metadata or labels are incorrect, and preserves the existing test-design, complexity-review, independent-review, and signing workflow.
- [x] AC6: The stale activity-metrics draft is cited as a worked example without changing that issue's implementation scope.
- [x] `linter all` exits with code `0`.
- [x] Relevant skill-link validation passes.
- [x] Manual review scenarios are executed against the finished artifact and documented in issue-local `manual-verification-evidence.md`; when reproduction is infeasible, the file records the constraint, attempted commands, and the strongest available substitute evidence.

## Verification Plan

### Automatic Checks

- `linter all`
- Review every changed `skill-link:` value against the linked skill's frontmatter `name`; no `./scripts/validate-skill-links.sh` exists in this repository.
- Any repository validation command supplied by `add-new-skill`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Plan a bug regardless of metadata | Read the new skill and prepare a small sample whose substance is a bug while its metadata is not `issue-type: bug`. | The sample visibly records all six steps, selects a regression-test boundary, and links evidence. | DONE | `manual-verification-evidence.md` section V1 |
| M2 | Start bug implementation | Give Implementer the sample bug spec, including a reproduction constraint if applicable. | It loads `fix-bug`, records analysis/reproduction-or-infeasibility/test-selection evidence before code, and does not treat a passing test alone as final proof. | DONE | `manual-verification-evidence.md` sections V2 and V3 |

### Disposable Verification Scripts

None planned. Documentation and workflow behavior can be validated by direct review and repository checks.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | `.github/skills/dev/debugging/fix-bug/SKILL.md` Required Sequence lists analyse, reproduce, select regression-test boundary, write red test, fix, and green/recheck steps. |
| AC2 | DONE | `.github/skills/dev/debugging/fix-bug/SKILL.md` Evidence Requirements require real-artifact commands/output/logs, infeasibility constraints and attempted commands, and state automated tests are not a substitute for final artifact recheck. |
| AC3 | DONE | `.github/skills/dev/debugging/fix-bug/SKILL.md` Regression-Test Boundary Rules require the smallest deterministic maintained test and rationale for integration/end-to-end/manual boundaries; `sample-substantive-bug-spec.md` applies the unit-first rule. |
| AC4 | DONE | `.github/skills/dev/planning/create-issue/SKILL.md` and `docs/templates/ISSUE.md` require bug-only `Bug-Fix Process` and `Regression Test Strategy` sections linking to `fix-bug` without duplicating the full workflow. |
| AC5 | DONE | `.github/agents/implementer.agent.md` loads `fix-bug` based on substantive behavior, preserves ordered reproduction and boundary selection before the red test, requires final like-for-like recheck evidence, and retains test-design, complexity-review, independent-review, and signed-commit steps. |
| AC6 | DONE | `.github/skills/dev/debugging/fix-bug/SKILL.md` cites issue #2226 as a review-only worked example using the stable issue reference and says not to change that issue's implementation scope; current diff leaves the #2226 issue unchanged. |
| LINT | DONE | `linter all` rerun by Task Reviewer at current HEAD on 2026-09-19 and exited `0`, including local Markdown links and fragments. |
| SKILL-LINKS | DONE | PyYAML parsed all reviewed frontmatter; every declared skill-link resolved to an actual skill frontmatter name; `linter all` passed local Markdown links and fragments. |
| MANUAL | DONE | `manual-verification-evidence.md` V3 records the fresh review-only Implementer interaction against the corrected F13/F14 artifacts, including the semantic trigger, ordered pre-red steps, mutate-then-restore proof, and final like-for-like evidence requirement. |

## Risks and Trade-offs

- **Procedure duplication:** copying detailed bug-fix instructions into the skill, template, and agent will cause drift. Mitigation: `fix-bug` is canonical; other artifacts only state triggers and required spec sections, then link to it.
- **Overly rigid TDD rule:** some bugs cannot be safely reproduced locally or tested directly. Mitigation: require a documented constraint and the strongest practical boundary, rather than pretending every bug has a unit test.
- **Agent-only knowledge:** a provider-specific agent must not become the source of truth. Mitigation: the Git-tracked skill and template contain the workflow; agent instructions are an optional invocation adapter.

## Implementation Completion Review

- Retrospective: `implementation-retrospective.md`
- The retrospective records reusable lessons from frontmatter parsing, semantic-link quoting,
  regression red proof, workflow ordering, append-only review evidence, and canonical PR-review
  processing.

## References

- GitHub issue: #2230
- Worked example: issue #2226
- Worked-example issue: #2226
- Related evidence: issue #2226 evidence artifacts
- Repository skill authority: `AGENTS.md`
