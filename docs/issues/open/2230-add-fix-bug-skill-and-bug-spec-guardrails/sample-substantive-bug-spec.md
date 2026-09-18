---
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: null
github-issue: null
spec-path: docs/issues/open/2230-add-fix-bug-skill-and-bug-spec-guardrails/sample-substantive-bug-spec.md
branch: "sample-substantive-bug"
related-pr: null
last-updated-utc: 2026-09-18 15:40
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
---

# Issue #[To be assigned] - Sample Substantive Bug With Incorrect Metadata

## Goal

Fix a sample stale counter that continues showing an old value after the source value changes.

## Background

This issue-local validation artifact intentionally uses `issue-type: task` to verify that bug
handling is semantic rather than metadata-only. The described behavior is broken because a visible
value remains stale after the state it reports has changed.

## Scope

### In Scope

- Diagnose why the sample counter reports stale values.
- Reproduce the stale value against the running artifact.
- Add a maintained regression test at the smallest deterministic boundary.
- Fix the counter and recheck the original artifact-level symptom.

### Out of Scope

- Changing unrelated counter behavior.
- Adding broad end-to-end coverage when a unit seam is sufficient.

## Architectural Decisions

- Related ADRs: `None known`
- ADRs to create: `None known`

## Design and Ownership Review

Not applicable. The sample does not introduce asynchronous I/O, child processes, readiness waits,
or reusable runtime fixtures.

## Bug-Fix Process

Use `.github/skills/dev/debugging/fix-bug/SKILL.md` because the described stale value is a bug even
though the metadata says `issue-type: task`.

Planned sequence:

1. Analyze the stale-counter code path and record the local hypothesis.
2. Reproduce the stale counter against the real artifact and record output in
   `manual-verification-evidence.md`.
3. Select the smallest deterministic regression-test boundary.
4. Run the red regression test and record failing output.
5. Fix the stale counter at the causal seam.
6. Rerun the regression test and the original artifact-level reproduction.

## Regression Test Strategy

Use a unit test at the counter's causal decision seam if the stale value can be observed without
process or protocol setup. Escalate to integration or end-to-end only if the stale behavior is not
observable through a deterministic unit API. Record the selected boundary, rationale, red output,
green output, and final recheck in issue-local `manual-verification-evidence.md`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | TODO | Reproduce stale counter | Artifact-level output showing the stale value. |
| T2 | TODO | Add regression test | Red test fails against the stale behavior. |
| T3 | TODO | Fix and recheck | Green test and matching artifact-level recheck. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1-T3 | Sample bug fix | Commit after focused validation, manual evidence, and review. |

## Progress Tracking

### Workflow Checkpoints

- [ ] Issue-local validation artifact drafted in `sample-substantive-bug-spec.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-18 15:40 UTC - GitHub Copilot - Created sample draft for issue #2230 manual validation.

## Acceptance Criteria

- [ ] AC1: The stale counter bug is reproduced against the real artifact or infeasibility is recorded.
- [ ] AC2: The smallest deterministic maintained regression-test boundary is selected and justified.
- [ ] AC3: Red, green, and final recheck evidence is recorded.
- [ ] `linter all` exits with code `0`.
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`.

## Verification Plan

### Automatic Checks

- `linter all`
- Relevant tests for changed components

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Reproduce stale counter | Run the artifact command that exposes the stale value. | Output shows the stale value before the fix. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Recheck stale counter | Rerun the original command after the fix. | Output shows the corrected value. | TODO | `manual-verification-evidence.md` section V2 |

## Risks and Trade-offs

- **Metadata drift:** the issue type can be wrong. Mitigation: follow `fix-bug` when the substance
  is a bug.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- Issue #2230 validation artifact.
