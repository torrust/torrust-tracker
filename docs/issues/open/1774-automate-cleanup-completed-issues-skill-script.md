---
doc-type: issue
issue-type: enhancement
status: planned
priority: p2
github-issue: 1774
spec-path: docs/issues/open/1774-automate-cleanup-completed-issues-skill-script.md
branch: "1774-automate-cleanup-completed-issues-skill-script"
related-pr: null
last-updated-utc: 2026-05-13 12:40
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/cleanup-completed-issues/SKILL.md
    - docs/issues/open/README.md
    - docs/issues/closed/README.md
---


# Issue #1774 - Automate cleanup of completed issue specs with a non-interactive script

## Goal

Automate the cleanup workflow for completed issue specs so moving closed issue specs from open to closed is fast, safe, and consistent for both humans and agents.

## Background

The workflow in .github/skills/dev/planning/cleanup-completed-issues/SKILL.md is clear but currently manual. Batch cleanup is repetitive and increases the chance of mistakes, especially when validating issue state and selecting the correct files.

The documented lifecycle already defines a safe two-stage process:

1. Stage 1 archive: move closed issue specs from docs/issues/open/ to docs/issues/closed/
2. Stage 2 delete: remove old specs from docs/issues/closed/ only when no longer referenced

This issue starts with Stage 1 automation and leaves Stage 2 deletion safeguards as a follow-up task in the same implementation scope.

## Scope

### In Scope

- Add script-based automation for Stage 1 archive.
- Keep script execution non-interactive and agent-friendly.
- Default to dry-run and require explicit apply mode for file changes.
- Verify GitHub issue state before moving files.
- Produce structured JSON results on stdout and diagnostics on stderr.
- Update cleanup skill documentation with script usage and examples.

### Out of Scope

- Automatically deleting files from docs/issues/closed/ without reference checks.
- Broad docs/issues taxonomy changes.
- Unrelated issue lifecycle process changes.

## Implementation Plan

Status values: TODO, IN_PROGRESS, BLOCKED, DONE.

| ID  | Status | Task                                 | Notes / Expected Output                                 |
| --- | ------ | ------------------------------------ | ------------------------------------------------------- |
| T1  | TODO   | Define script interface              | Flags, exit codes, output format, and error contract    |
| T2  | TODO   | Implement Stage 1 archive automation | Closed-state verification and deterministic file moves  |
| T3  | TODO   | Add safety and idempotency checks    | Re-runnable behavior with clear skip reasons            |
| T4  | TODO   | Update skill documentation           | SKILL.md includes script inventory, usage, and examples |
| T5  | TODO   | Validate quality gates               | linter all and targeted checks pass                     |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in docs/issues/drafts/
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into develop before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (linter all, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from docs/issues/open/ to docs/issues/closed/

### Progress Log

- 2026-05-13 12:20 UTC - Copilot - Created GitHub issue #1774 for cleanup automation.
- 2026-05-13 12:40 UTC - Copilot - Added open issue spec file for #1774 in docs/issues/open.

## Acceptance Criteria

- [ ] AC1: Stage 1 archive flow is automated with non-interactive CLI execution.
- [ ] AC2: Script defaults to dry-run and requires explicit apply mode for writes.
- [ ] AC3: Only closed GitHub issues are eligible for move; open/not-found issues are skipped with actionable diagnostics.
- [ ] AC4: Script output is machine-parsable JSON on stdout with per-issue outcomes.
- [ ] AC5: Cleanup skill documentation is updated with script usage and constraints.
- [ ] linter all exits with code 0
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- linter all
- Relevant tests for changed components
- Pre-push checks (when applicable)

### Manual Verification Scenarios

Status values: TODO, IN_PROGRESS, DONE, FAILED, BLOCKED.

| ID  | Scenario                           | Command/Steps                                                               | Expected Result                                              | Status | Evidence |
| --- | ---------------------------------- | --------------------------------------------------------------------------- | ------------------------------------------------------------ | ------ | -------- |
| M1  | Dry-run with closed and open issue | Run script with --issues containing one closed and one open issue           | Closed issue marked movable; open issue skipped with reason  | TODO   |          |
| M2  | Apply mode with closed issue       | Run script with --apply for one closed issue with file in docs/issues/open/ | File is moved to docs/issues/closed/ and result is reported  | TODO   |          |
| M3  | Idempotent rerun                   | Re-run the same command after successful move                               | Script reports already-moved or skipped without failing      | TODO   |          |
| M4  | Missing file behavior              | Run script for a closed issue without matching file in docs/issues/open/    | Script exits non-zero or reports explicit missing-file error | TODO   |          |

Notes:

- Manual verification is mandatory even when automated tests pass.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Acceptance Verification

| AC ID | Status (TODO/DONE) | Evidence |
| ----- | ------------------ | -------- |
| AC1   | TODO               |          |
| AC2   | TODO               |          |
| AC3   | TODO               |          |
| AC4   | TODO               |          |
| AC5   | TODO               |          |

## Risks and Trade-offs

- Script complexity could exceed the value for small batches.
  - Mitigation: keep MVP focused on Stage 1 archive and clear CLI boundaries.
- Incorrect file matching could move wrong files.
  - Mitigation: strict issue-number-based matching and explicit ambiguity errors.
- Over-automation could encourage unsafe deletion patterns.
  - Mitigation: keep Stage 2 deletion guarded and explicit, not implicit.

## References

- GitHub issue: https://github.com/torrust/torrust-tracker/issues/1774
- Cleanup skill: .github/skills/dev/planning/cleanup-completed-issues/SKILL.md
- Script guidance: https://agentskills.io/skill-creation/using-scripts
