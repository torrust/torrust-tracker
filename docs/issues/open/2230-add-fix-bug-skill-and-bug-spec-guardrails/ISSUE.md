---
doc-type: issue
issue-type: task
status: open
priority: p2
epic: null
github-issue: 2230
spec-path: docs/issues/open/2230-add-fix-bug-skill-and-bug-spec-guardrails/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-15 15:05
semantic-links:
  skill-links:
    - add-new-skill
    - create-issue
    - write-unit-test
  related-artifacts:
    - .github/skills/add-new-skill/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - .github/agents/implementer.agent.md
    - docs/templates/ISSUE.md
    - docs/issues/drafts/fix-stale-inactivity-cutoff-in-activity-metrics-updater/ISSUE.md
---

# Issue #2230 - Add a `fix-bug` Skill and Bug-Spec Guardrails

## Goal

Establish one repository-owned workflow for investigating and fixing confirmed bugs, then require bug specifications to plan that workflow and teach the Implementer agent to apply it.

## Background

The confirmed stale activity-metrics cutoff bug exposed a missing explicit repository workflow. Its investigation used a useful sequence: analyse the defect, reproduce it against the real artifact, choose the smallest regression-test boundary, write a failing test, fix the code, and rerun both the automatic regression test and original manual reproduction.

Current guidance is incomplete by design:

- `create-issue` governs drafting and publishing all issue types, but has no bug-specific process requirement.
- `write-unit-test` governs test design and deterministic time, but does not prescribe when a bug needs unit, integration, end-to-end, or manual validation.
- Implementer requires TDD where practical, but does not require a confirmed-bug reproduction or a like-for-like final recheck.
- `docs/templates/ISSUE.md` has no explicit bug-only sections for investigation and regression strategy.

A new canonical skill is preferable to duplicating operational procedure in an agent, generic template, and issue-creation skill. The related stale-cutoff bug specification is the worked example and remains out of scope for this documentation/process change.

## Scope

### In Scope

- Create `.github/skills/dev/debugging/fix-bug/SKILL.md` as the canonical bug-fix workflow.
- Add bug-conditional specification requirements to `create-issue` and `docs/templates/ISSUE.md` as thin references to `fix-bug`.
- Update the Implementer agent to load and apply `fix-bug` for `issue-type: bug` specifications.
- Define evidence standards for source analysis, real-artifact reproduction, failing-test output, fixed-test output, and final recheck.
- Define a decision framework for selecting unit, integration, end-to-end, and manual regression verification.
- Add a worked-example reference to the stale activity-metrics cutoff draft.

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
| T1 | TODO | Define the canonical bug-fix workflow | Create `fix-bug` using the repository's `add-new-skill` guidance. Require the sequence: analyse, reproduce, select tests, red test, fix, and recheck. |
| T2 | TODO | Define test-selection and evidence rules | Prefer a deterministic unit test at the causal seam; require a written reason when a higher-level boundary is selected. Distinguish maintained tests from real manual reproduction evidence. |
| T3 | TODO | Link issue authoring guidance and template | Make `create-issue` require `Bug-Fix Process` and `Regression Test Strategy` sections for `issue-type: bug`; add clearly marked bug-only template blocks that link to the canonical skill. |
| T4 | TODO | Link the Implementer agent workflow | Require Implementer to read and apply `fix-bug` for a confirmed bug, preserve its existing TDD and independent-review rules, and record the red/green/recheck evidence in the issue folder. |
| T5 | TODO | Validate the workflow end-to-end | Use the stale-cutoff draft as a review-only worked example; validate Markdown, spelling, skill links, and a new minimal bug draft against the required sections. |

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
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, skill-link validation, and relevant checks)
- [ ] Manual review scenarios executed and evidence recorded
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-09-15 10:40 UTC - GitHub Copilot - Created parked follow-up draft after the confirmed stale activity-metrics cutoff bug established a reusable six-step investigation and regression-prevention process.
- 2026-09-15 11:10 UTC - GitHub Copilot - Promoted the draft after GitHub issue #2226 was created; it is ready for maintainer review.
- 2026-09-15 15:00 UTC - josecelano - Approved this workflow-specification draft for GitHub issue creation - Chat approval.
- 2026-09-15 15:05 UTC - GitHub Copilot - Created GitHub issue #2230 and promoted this specification to `docs/issues/open/` - https://github.com/torrust/torrust-tracker/issues/2230

## Acceptance Criteria

- [ ] AC1: `.github/skills/dev/debugging/fix-bug/SKILL.md` defines the required sequence: analyse, reproduce, select regression test type, write a failing test, fix, and recheck.
- [ ] AC2: The skill requires real-artifact reproduction evidence with actual commands and output when reproduction is feasible, and distinguishes it from maintained automated tests.
- [ ] AC3: The skill requires selection of the smallest deterministic regression-test boundary, with a documented rationale for integration or end-to-end tests.
- [ ] AC4: `create-issue` and `docs/templates/ISSUE.md` require bug-only `Bug-Fix Process` and `Regression Test Strategy` sections that link to the canonical skill instead of duplicating it.
- [ ] AC5: The Implementer agent loads `fix-bug` for confirmed bug specs and preserves the existing test-design, complexity-review, independent-review, and signing workflow.
- [ ] AC6: The stale activity-metrics draft is cited as a worked example without changing that issue's implementation scope.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant skill-link validation passes.
- [ ] Manual review scenarios are executed and documented in issue-local `manual-verification-evidence.md`.

## Verification Plan

### Automatic Checks

- `bash ./scripts/validate-skill-links.sh`
- `linter all`
- Any repository validation command supplied by `add-new-skill`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Plan a confirmed bug | Read the new skill and prepare a small sample `issue-type: bug` spec using the template. | The sample visibly records all six steps, selects a regression-test boundary, and links evidence. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Start bug implementation | Give Implementer the sample confirmed-bug spec. | It loads `fix-bug`, records analysis/reproduction/test-selection evidence before code, and does not treat a passing test alone as final proof. | TODO | `manual-verification-evidence.md` section V2 |

### Disposable Verification Scripts

None planned. Documentation and workflow behavior can be validated by direct review and repository checks.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | |
| AC2 | TODO | |
| AC3 | TODO | |
| AC4 | TODO | |
| AC5 | TODO | |
| AC6 | TODO | |

## Risks and Trade-offs

- **Procedure duplication:** copying detailed bug-fix instructions into the skill, template, and agent will cause drift. Mitigation: `fix-bug` is canonical; other artifacts only state triggers and required spec sections, then link to it.
- **Overly rigid TDD rule:** some bugs cannot be safely reproduced locally or tested directly. Mitigation: require a documented constraint and the strongest practical boundary, rather than pretending every bug has a unit test.
- **Agent-only knowledge:** a provider-specific agent must not become the source of truth. Mitigation: the Git-tracked skill and template contain the workflow; agent instructions are an optional invocation adapter.

## Implementation Completion Review

After implementation, compare the delivered guidance with this specification. Record reusable lessons, material changes, and deviations in `implementation-retrospective.md` when warranted; otherwise state why none was needed in the progress log.

## References

- GitHub issue: #2230
- Worked example: `docs/issues/drafts/fix-stale-inactivity-cutoff-in-activity-metrics-updater/ISSUE.md`
- Worked-example issue: #2226
- Related evidence: `docs/issues/open/2226-fix-stale-inactivity-cutoff-in-activity-metrics-updater/evidence.md`
- Repository skill authority: `AGENTS.md`
