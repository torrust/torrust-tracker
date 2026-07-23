---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: 1460
spec-path: docs/issues/drafts/1460-add-hadolint-to-container-workflow.md
branch: "1460-add-hadolint-to-container-workflow"
related-pr: null
last-updated-utc: 2026-07-23 09:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/workflows/container.yaml
    - Containerfile
    - docs/security/analysis/non-affecting/
---

<!-- skill-link: create-issue -->

# Issue #1460 - Docker Security Overhaul: Add a linter step to the `container.yaml` workflow

> **EPIC position**: Subissue of [Docker Security Overhaul #1457](https://github.com/torrust/torrust-tracker/issues/1457).

## Goal

Add a [hadolint](https://github.com/hadolint/hadolint) (Dockerfile linter) step to the `container.yaml` GitHub Actions workflow to ensure the `Containerfile` meets Docker best practices. The workflow should fail when hadolint detects violations that are not explicitly allowed (via ignore directives). Fix the existing hadolint warnings in `Containerfile` where appropriate, and explicitly document/suppress false positives or non-applicable warnings.

## Background

The `Containerfile` currently has several hadolint warnings (see output in issue #1460). These fall into two categories:

1. **Fixable warnings** — genuine improvements to Dockerfile quality and security (e.g., pinning package versions, adding `--no-install-recommends`, consolidating `RUN` commands).
2. **Non-applicable or false-positive warnings** — rules that do not apply to this project's build strategy (e.g., `DL4006` pipefail in Alpine-based images where `/bin/sh` is symlinked to `/bin/ash`, or `SC2046` in shell lines that are intentionally unquoted).

Adding hadolint as a CI step will catch regressions and enforce consistent Dockerfile quality going forward.

## Scope

### In Scope

- Add a hadolint step to `.github/workflows/container.yaml` that runs `hadolint` on the `Containerfile`
- The hadolint step runs before the build step (early feedback)
- Fix or suppress all existing hadolint warnings
- Document the ignore policy for any suppressed rules with rationale
- The workflow step fails when hadolint finds violations not explicitly allowed
- Provide a mechanism to safely ignore false positives (inline `# hadolint ignore=` comments)
- Add hadolint to the pre-commit hook (`contrib/dev-tools/git/hooks/pre-commit.sh`), ideally running only if `Containerfile` has changed (git diff check), though newer hadolint versions can detect new problems even for unchanged files — the workflow catches those

### Out of Scope

- Fixing CVEs in container base images (covered by #1898)
- Adding linters for other container-related files (docker-compose, etc.)
- Modifying the publish workflow steps
- Adding new container build features or stages

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                       | Notes / Expected Output                                             |
| --- | ------ | -------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| T1  | TODO   | Run hadolint on current `Containerfile` and catalog all warnings           | Capture full output, categorize each warning as fixable or suppress |
| T2  | TODO   | Fix fixable hadolint warnings in `Containerfile`                           | Apply fixes and verify no regressions                               |
| T3  | TODO   | Suppress non-applicable warnings with inline `# hadolint ignore=` comments | Each suppression must have a rationale comment                      |
| T4  | TODO   | Add hadolint step to `container.yaml` workflow                             | New step before build; fail on violations not explicitly allowed    |
| T5  | TODO   | Add hadolint to pre-commit hook                                            | Run only if Containerfile changed; workflow catches broader changes |
| T6  | TODO   | Run `linter all` and tests to verify no breakage                           |                                                                     |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-07-23 09:00 UTC - Agent - Initial draft spec created
- 2026-07-23 09:05 UTC - Agent - Added pre-commit hook scope per user feedback

## Acceptance Criteria

- [ ] AC1: Hadolint runs as a CI step in `container.yaml` and fails the workflow on disallowed violations
- [ ] AC2: All existing hadolint warnings are either fixed or explicitly suppressed with documented rationale
- [ ] AC3: The `container.yaml` workflow passes for the current `Containerfile`
- [ ] AC4: False-positive warnings have a documented mechanism for safe ignoring (inline `# hadolint ignore=` comments)
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- `cargo test --doc --workspace`
- `cargo test --tests --benches --examples --workspace --all-targets --all-features`
- Pre-push checks (when applicable)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                               | Command/Steps                                            | Expected Result                         | Status | Evidence |
| --- | -------------------------------------- | -------------------------------------------------------- | --------------------------------------- | ------ | -------- |
| M1  | Run hadolint locally                   | `docker run --rm -i hadolint/hadolint < ./Containerfile` | Clean output (no unexpected warnings)   | TODO   |          |
| M2  | Verify workflow passes with violations | Push branch and check container.yaml workflow run        | Workflow passes or fails as expected    | TODO   |          |
| M3  | Verify ignored warnings are documented | Check each `# hadolint ignore=` comment has rationale    | All suppressions have rationale comment | TODO   |          |
