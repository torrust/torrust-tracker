---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2758
github-issue: 2767
spec-path: docs/issues/open/2767-2758-scripts-audit/ISSUE.md
branch: "2767-scripts-audit"
related-pr: null
last-updated-utc: "2026-11-06 17:10"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md
    - docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #2767 - Audit and classify the scripts called by CI and AI agents

**Parent EPIC:** #2758 - Migrate the CLI tools to the global output contract. Order 8 of 11, wave 1.

## Goal

Produce an audit document that lists the shell and Python scripts under `contrib/dev-tools/` and `.github/skills/` that CI or agents call, classifies each under the contract, and records a decision per script (migrate, rewrite in Rust, exempt with a reason, or follow-up issue).

## Background

ADR `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md` covers every first-party program that machines call, wherever it lives; maintainer decision (2026-11-06): audit the scripts in this EPIC, do not rewrite them here. Known examples: the `resolve-review-threads/scripts/*.sh` scripts (JSON on stdout, plain-text progress on stderr), `contrib/dev-tools/git/hooks/*.sh`, `merge-pull-request.sh`, `github-merge.py`, `process-pr-review/scripts/validate-audit-record.py`, `contrib/dev-tools/checks/*.sh`, and the container and runner scripts. Determine, per script, who calls it and whether a machine consumes its output or exit code.

The global CLI output contract (`docs/adrs/20260519000000_define_global_cli_output_contract.md`), as amended by `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`, requires: result data only on stdout (one JSON object) for `stdout-result-data` commands, nothing on stdout for `no-stdout-result` commands; every stderr record is one JSON object per line (NDJSON) with a `kind`; no plain text on either channel; exit codes only `0` (did its job, the answer is in the result), `1` (could not do its job) and `2` (usage error or TTY refusal); and TTY refusal for `stdout-result-data` commands. A distinct outcome is a `kind` field: an answer to what the caller asked is a stdout result with exit `0`; "could not answer" is a stderr diagnostic with exit `1`. This issue is part of EPIC #2758, which migrates the tools found non-compliant by the 2026-11-06 audit. The reference implementations are `contrib/dev-tools/checks/ci-gate/src/main.rs` (`stdout-result-data`) and `contrib/dev-tools/checks/shipped-features-check/src/main.rs` (`no-stdout-result`).

## Scope

### In Scope

- Inventory the scripts and their callers.
- Classify each (`stdout-result-data`, `no-stdout-result`, or developer-only/out of scope) and record compliance.
- Decide per script and list the follow-up issues to open (do not open them in this issue unless the maintainer asks).

### Out of Scope

- Changing any script.
- Editing the ADR.

Files this issue may touch (a sibling issue in the same wave must not touch them):

- `a new audit document inside this issue's folder`

## Architectural Decisions

Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`. ADRs to create: None known.

During implementation, stop and create an ADR when a decision affects project architecture or design patterns, selects an approach among meaningful alternatives, or has consequences future contributors need to understand. Do not create ADRs for routine implementation details.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | TODO | Inventory scripts and callers | Table in the audit document. |
| T2 | TODO | Classify and check compliance | Per-script findings. |
| T3 | TODO | Record decisions and propose follow-ups | Maintainer review of the list. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Inventory scripts and callers | Commit after focused validation and required review. |
| T2 | Classify and check compliance | Commit after focused validation and required review. |
| T3 | Record decisions and propose follow-ups | Commit after focused validation and required review. |

Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG. For test-producing work, use the `write-unit-test` skill and complete the design review it requires before each commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2758-scripts-audit/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-06 17:10 UTC - Claude Code (Claude Sonnet 5.5) - Drafted this spec as part of EPIC #2758 from the 2026-11-06 audit.
- 2026-11-06 17:25 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the spec; GitHub issue #2767 created.

## Acceptance Criteria

- [ ] AC1: Every script that CI or an agent calls appears in the audit.
- [ ] AC2: Each has a class, a compliance finding and a decision.
- [ ] AC3: No script is changed by this issue.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior or workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- Relevant tests for changed components (`cargo test -p <package>`)
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Spot check | Run three flagged scripts and compare their channels and exit codes with the audit | Audit matches behavior | TODO | `manual-verification-evidence.md` section V1 |

Manual verification is mandatory even when automated tests pass. Record actual commands, output and the toolchain that produced each result in `manual-verification-evidence.md` (from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`).

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO |  |
| AC2 | TODO |  |
| AC3 | TODO |  |

## Risks and Trade-offs

- The inventory misses indirect callers. Mitigation: search workflows, hooks, skills and docs, and list the search commands in the audit.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise record in the progress log why none was needed.

## References

- Parent EPIC: #2758
- Related issues: #2715, #1798
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`
