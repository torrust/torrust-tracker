---
doc-type: issue
issue-type: enhancement
status: planned
priority: p1
github-issue: 1768
spec-path: docs/issues/open/1768-refactor-update-dependencies-skill-automation.md
branch: "1768-refactor-update-dependencies-skill-automation"
related-pr: null
last-updated-utc: 2026-05-13 09:28
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/maintenance/update-dependencies/SKILL.md
    - .github/skills/dev/maintenance/add-rust-dependency/SKILL.md
---


# Issue #1768 - Refactor update-dependencies skill automation

## Goal

Automate the update-dependencies workflow so branch creation, update execution, classification, validation, and commit metadata generation are script-assisted and less error-prone for both humans and agents.

## Background

The current update workflow in [.github/skills/dev/maintenance/update-dependencies/SKILL.md](../../../.github/skills/dev/maintenance/update-dependencies/SKILL.md) is clear but mostly manual.

Current pain points:

- Branch-first flow is documented but not enforced.
- No-op updates (no `Cargo.lock` changes) are detected manually.
- Update logs and commit body generation are manual.
- Repeated command runs can drift from the prescribed sequence.

This issue focuses only on dependency-skill automation. Pre-commit performance/verbosity is tracked separately in [docs/issues/open/1769-refactor-pre-commit-checks-performance-and-verbosity.md](1769-refactor-pre-commit-checks-performance-and-verbosity.md).

Automation policy constraint:

- We do not want to lock core dependency-maintenance workflow execution to GitHub-only services (for example Dependabot).
- The update process must remain portable to different infrastructures and reusable with different AI providers.
- GitHub ecosystem tooling is acceptable as optional integration, but not as a mandatory dependency for the workflow.

## Scope

### In Scope

- Add script-backed automation to the dependency update workflow, aligned with Agent Skills script support (https://agentskills.io/skill-creation/using-scripts).
- Define script placement policy:
  - skill-local scripts when usage is skill-private
  - `contrib/dev-tools/` for scripts reusable outside that skill
- Update skill documentation to make scripts first-class while preserving a manual fallback.

### Out of Scope

- Refactoring pre-commit/pre-push hooks.
- CI check-tier redesign.
- Non-dependency workflow changes.

## Deep Analysis Summary

Current workflow in [.github/skills/dev/maintenance/update-dependencies/SKILL.md](../../../.github/skills/dev/maintenance/update-dependencies/SKILL.md):

- Branch creation is documented but not enforced.
- `cargo update` output capture to `/tmp/cargo-update.txt` is documented, but downstream consumption is manual.
- Trivial/no-op updates rely on user judgment.
- Breaking-change triage is manual and repeated across runs.

Risk:

- Agents/developers can deviate from required sequence.
- Inconsistent branch naming and commit metadata across dependency update PRs.
- Higher operational friction than necessary for a routine maintenance workflow.

## Proposed Changes

### Task 1: Add script entrypoints for dependency updates

- [ ] Add a script directory under the skill path (example: `.github/skills/dev/maintenance/update-dependencies/scripts/`).
- [ ] Apply placement decision per script:
  - keep under the skill when only used by that skill
  - place in `contrib/dev-tools/` when useful as standalone dev tooling
- [ ] Implement script entrypoints for:
  - branch preparation (`prepare-branch.sh`)
  - update execution (`run-update.sh`)
  - verification (`verify-update.sh`)
  - commit message/body generation (`build-commit-message.sh`)
- [ ] Ensure scripts are idempotent and safe to rerun.

### Task 2: Enforce branch-first workflow

- [ ] Script fails early when current branch is `develop` and dependency update changes are already present.
- [ ] Script creates timestamp branch for trivial updates (`YYYYMMDD-update-dependencies`) unless issue branch is explicitly provided.
- [ ] Script prints deterministic next actions.

### Task 3: Automate update classification and no-op exit

- [ ] Use `cargo update --dry-run` plus lockfile diff checks to classify:
  - no changes
  - lockfile-only trivial update
  - update requiring code changes
- [ ] On no-op, exit success with clear message.
- [ ] Persist update logs to a deterministic path and print it.

### Task 4: Automate verification sequence

- [ ] Script wrapper executes required checks:
  - `cargo machete`
  - `./contrib/dev-tools/git/hooks/pre-commit.sh`
- [ ] Support output modes:
  - concise (default): step summary + log paths
  - verbose (opt-in): streaming mode

### Task 5: Update skill documentation and examples

- [ ] Refactor skill to script-first usage.
- [ ] Keep manual fallback path for constrained environments.
- [ ] Document recovery actions for common failure modes.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                              | Notes / Expected Output                               |
| --- | ------ | --------------------------------- | ----------------------------------------------------- |
| T1  | TODO   | Design script interfaces          | Stable script inputs/outputs and invocation examples. |
| T2  | TODO   | Implement scripts                 | Script set created with idempotent behavior.          |
| T3  | TODO   | Integrate scripts into skill docs | Script-first flow with manual fallback.               |
| T4  | TODO   | Validate quality gates            | `linter all` and relevant tests pass.                 |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-05-13 07:19 UTC - Copilot - Drafted initial combined proposal.
- 2026-05-13 07:24 UTC - Copilot - Added script placement policy (skill-local vs reusable `contrib/dev-tools`).
- 2026-05-13 07:33 UTC - Copilot - Split combined proposal into two drafts; this spec now focuses only on dependency skill automation.
- 2026-05-13 09:26 UTC - Copilot - Opened GitHub issue #1768 and moved this spec to `docs/issues/open/`.

## Acceptance Criteria

- [ ] AC1: Dependency update workflow supports script-based execution with branch-first enforcement and no-op detection.
- [ ] AC2: Skill docs for dependency updates are updated to script-first with manual fallback.
- [ ] AC3: Script-location policy is documented and applied consistently (skill-local vs `contrib/dev-tools`).
- [ ] AC4: Required verification sequence is script-assisted and reproducible.
- [ ] AC5: `linter all` exits with code `0` after changes.
- [ ] AC6: Relevant tests pass for modified scripts/skill behavior.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                 |
| ----- | ---------------------- | -------------------------------------------------------- |
| AC1   | TODO                   | Script run outputs for branch enforcement and no-op case |
| AC2   | TODO                   | Updated skill docs                                       |
| AC3   | TODO                   | Script inventory and final placement map                 |
| AC4   | TODO                   | Verification script output/logs                          |
| AC5   | TODO                   | `linter all` output                                      |
| AC6   | TODO                   | Test outputs                                             |

## Risks and Trade-offs

- Automation scripts add maintenance surface.
  - Mitigation: keep scripts small, composable, and with clear interfaces.
- Over-enforcement can reduce flexibility in exceptional cases.
  - Mitigation: allow explicit override flags with clear warnings.

## References

- Agent Skills script usage: https://agentskills.io/skill-creation/using-scripts
- Dependency update skill: [.github/skills/dev/maintenance/update-dependencies/SKILL.md](../../../.github/skills/dev/maintenance/update-dependencies/SKILL.md)
- Related dependency skill: [.github/skills/dev/maintenance/add-rust-dependency/SKILL.md](../../../.github/skills/dev/maintenance/add-rust-dependency/SKILL.md)
- GitHub issue: https://github.com/torrust/torrust-tracker/issues/1768
- Related split issue spec: [docs/issues/open/1769-refactor-pre-commit-checks-performance-and-verbosity.md](1769-refactor-pre-commit-checks-performance-and-verbosity.md)
