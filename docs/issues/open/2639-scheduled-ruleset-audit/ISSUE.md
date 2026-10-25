---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p3
epic: null
github-issue: 2639
spec-path: docs/issues/open/2639-scheduled-ruleset-audit/ISSUE.md
branch: "2639-scheduled-ruleset-audit"
related-pr: null
last-updated-utc: "2026-10-24 20:30"
semantic-links:
  skill-links:
    - create-issue
    - implement-workflow
  related-artifacts:
    - docs/issues/closed/2525-ci-gate-required-check/ISSUE.md
    - docs/github-rulesets.md
    - docs/adrs/20261013161419_stop_requiring_pull_request_rebases.md
    - .github/workflows/security-scan.yaml
---

<!-- skill-link: create-issue -->

# Issue #2639 - Audit the repository rulesets on a schedule

## Goal

Detect drift between the live GitHub rulesets of `torrust/torrust-tracker` and the rulesets documented in `docs/github-rulesets.md`, on a schedule, so that a ruleset that is disabled, deleted, or changed without a record is noticed soon.

## Background

Issue #2507 found that the only required-checks ruleset had been effectively disabled since 2026-05-05 and nobody noticed: renamed jobs and path-filtered workflows made it unsatisfiable, and the maintainer disabled it around every merge. The decision of 2026-10-22 in #2525 keeps CI checks advisory and relies on the force-push ban, the signed-commit rule, and the other rulesets, which are now separate rulesets, so a disabled rule can no longer remove another. `docs/github-rulesets.md` lists the active rulesets and says to compare it with the live state, but nothing does that comparison unless a person remembers to.

The live rulesets are readable with `gh api repos/torrust/torrust-tracker/rulesets`. Reading them needs a token with repository administration read access, which the default `GITHUB_TOKEN` of a workflow does not have, so how to run the audit is a decision.

## Scope

### In Scope

- Decide where the audit runs (T1): a scheduled workflow with a dedicated read-only token, or a command the maintainer runs and a skill step. Record the token, its permissions, and who owns it.
- A Rust tool, under `contrib/dev-tools/`, that compares the live rulesets (id, name, target, enforcement, include and exclude conditions, rules, bypass actors) with an expected-state file kept in the repository, and prints a JSON report that follows the global CLI output contract.
- The expected-state file, and a statement in `docs/github-rulesets.md` of how to update it together with a ruleset change.
- If it runs on a schedule, a thin workflow that runs the tool and fails, or opens an issue, on drift.
- Unit tests for the comparison, and verification against the live rulesets.

### Out of Scope

- Changing any ruleset, or making any status check required.
- Auditing branch protection that is not a ruleset, repository settings other than rulesets, or the allowed-actions list.

## Architectural Decisions

- Related ADRs: [`20261016154414_keep_ci_decision_logic_in_rust_behind_thin_provider_agnostic_workflows.md`](../../../adrs/20261016154414_keep_ci_decision_logic_in_rust_behind_thin_provider_agnostic_workflows.md) (decision logic in Rust, thin workflows) and [`20260519000000_define_global_cli_output_contract.md`](../../../adrs/20260519000000_define_global_cli_output_contract.md).
- ADRs to create: None known.

## Design and Ownership Review

Not applicable. The tool reads one API and compares two documents; it starts no child process beyond `gh` and holds no resource.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                         | Notes / Expected Output                                                                                                           |
| --- | ------ | ---------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Decide where the audit runs  | Maintainer-approved choice and token model; record the permissions and owner. Follow `implement-workflow` if a workflow is added. |
| T2  | TODO   | Rust comparison tool         | Compares live and expected rulesets, with unit tests; the output follows the global CLI output contract.                          |
| T3  | TODO   | Expected-state file and docs | The file mirrors the active rulesets of `docs/github-rulesets.md`; the document explains how to change both together.             |
| T4  | TODO   | Schedule or skill step       | The thin workflow, or the documented command; drift is reported as a failure or an issue.                                         |
| T5  | TODO   | Verification                 | Run against the live rulesets; confirm that a simulated drift (a disabled ruleset in the expected file) is reported.              |

## Commit Points

| Task | Coherent change set                   | Commit policy                                        |
| ---- | ------------------------------------- | ---------------------------------------------------- |
| T1   | Decision record (spec update)         | Commit after maintainer approval.                    |
| T2   | Comparison tool with tests            | Commit after focused validation and required review. |
| T3   | Expected-state file and documentation | Commit after `linter all`.                           |
| T4   | Workflow or skill step                | Commit after a sandbox or real run.                  |
| T5   | Verification evidence                 | Commit after `linter all`.                           |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/scheduled-ruleset-audit/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
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

- 2026-10-24 20:30 UTC - GitHub Copilot - Drafted as follow-up T9(a) of #2525; priority p3 because the maintainer decided it is not a priority now.

## Acceptance Criteria

- [ ] AC1: The tool reports every difference between the live rulesets and the expected-state file, and reports nothing when they match.
- [ ] AC2: The audit runs on a schedule or by a documented command, and drift produces a visible failure or issue.
- [ ] AC3: `docs/github-rulesets.md` says how to update the expected-state file with a ruleset change.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- Unit tests of the new tool
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                    | Human-oriented command/steps                                         | Expected Result                          | Status | Evidence                                     |
| --- | --------------------------- | -------------------------------------------------------------------- | ---------------------------------------- | ------ | -------------------------------------------- |
| M1  | Live state matches          | Run the tool against the live rulesets                               | No drift reported                        | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | Simulated drift is reported | Run it against an expected file with one ruleset changed or disabled | The difference is reported by ruleset id | TODO   | `manual-verification-evidence.md` section V2 |

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence           |
| ----- | ---------------------- | ------------------ |
| AC1   | TODO                   | {test/log/PR link} |
| AC2   | TODO                   | {test/log/PR link} |
| AC3   | TODO                   | {test/log/PR link} |

## Risks and Trade-offs

- A token with repository administration read access is a sensitive secret to put in a workflow; the alternative is a maintainer-run command, which can be forgotten. T1 decides.
- The expected-state file can itself drift from the documentation; keep one source and check the other against it.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise add a progress-log entry explaining why none was needed.

## References

- Related issues: #2525, #2507
- Related docs: [`docs/github-rulesets.md`](../../../github-rulesets.md)
