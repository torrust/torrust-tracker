---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2758
github-issue: 2770
spec-path: docs/issues/open/2770-2758-close-out-adr-and-epic/ISSUE.md
branch: "2770-close-out-adr-and-epic"
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

# Issue #2770 - Close the CLI contract migration: update the ADR notes and the EPIC

**Parent EPIC:** #2758 - Migrate the CLI tools to the global output contract. Order 11 of 11, wave last.

## Goal

After the migration issues are merged, remove the 'currently non-compliant' notes from the ADR binary table and close the EPIC bookkeeping.

## Background

Sibling issues do not edit the ADR table (only issue 1 and this one do), so the table still says some tools are non-compliant when they are done. This is the close-out PR described by the epic-coordination process: move the EPIC spec to `docs/issues/closed/`, update the Subissues table and acceptance verification. Also remove the scripts-audit follow-ups that became issues, and record leftover non-compliance with its issue.

The global CLI output contract (`docs/adrs/20260519000000_define_global_cli_output_contract.md`), as amended by `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`, requires: result data only on stdout (one JSON object) for `stdout-result-data` commands, nothing on stdout for `no-stdout-result` commands; every stderr record is one JSON object per line (NDJSON) with a `kind`; no plain text on either channel; exit codes only `0` (did its job, the answer is in the result), `1` (could not do its job) and `2` (usage error or TTY refusal); and TTY refusal for `stdout-result-data` commands. A distinct outcome is a `kind` field: an answer to what the caller asked is a stdout result with exit `0`; "could not answer" is a stderr diagnostic with exit `1`. This issue is part of EPIC #2758, which migrates the tools found non-compliant by the 2026-11-06 audit. The reference implementations are `contrib/dev-tools/checks/ci-gate/src/main.rs` (`stdout-result-data`) and `contrib/dev-tools/checks/shipped-features-check/src/main.rs` (`no-stdout-result`).

## Scope

### In Scope

- Update the ADR table notes to match the final state.
- Update the EPIC spec: statuses, acceptance verification, progress log; move it to `docs/issues/closed/`.
- Repair every live reference to the EPIC's old path: the child specs (open or closed) cite `docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md` in their `semantic-links.related-artifacts`, and the frontmatter validator reports a moved file as missing. Update them in the same PR as the move, and run the frontmatter validator to confirm.
- Verify with a fresh audit that no in-scope tool is non-compliant, or that the leftover is recorded with an issue.

### Out of Scope

- Any code change.

Files this issue may touch (a sibling issue in the same wave must not touch them):

- `docs/adrs/20260519000000_define_global_cli_output_contract.md`
- `docs/issues/open/2758-*/EPIC.md` (moved to `docs/issues/closed/`)
- `docs/issues/{open,closed}/27*-2758-*/ISSUE.md` (only the references to the EPIC path)

## Architectural Decisions

Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`. ADRs to create: None known.

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
| T1 | TODO | Re-run the audit on `develop` | Result recorded. |
| T2 | TODO | Update the ADR table | No stale note. |
| T3 | TODO | Update and move the EPIC spec | EPIC closeable. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Re-run the audit on `develop` | Commit after focused validation and required review. |
| T2 | Update the ADR table | Commit after focused validation and required review. |
| T3 | Update and move the EPIC spec | Commit after focused validation and required review. |

Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG. For test-producing work, use the `write-unit-test` skill and complete the design review it requires before each commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2758-close-out-adr-and-epic/ISSUE.md`
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
- 2026-11-06 17:25 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the spec; GitHub issue #2770 created.

## Acceptance Criteria

- [ ] AC1: The ADR table matches the code.
- [ ] AC2: The EPIC's acceptance criteria are checked with evidence.
- [ ] AC3: The EPIC spec is under `docs/issues/closed/`.
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
| M1 | Audit re-run | Repeat the audit commands recorded in the EPIC | No new finding | TODO | `manual-verification-evidence.md` section V1 |

Manual verification is mandatory even when automated tests pass. Record actual commands, output and the toolchain that produced each result in `manual-verification-evidence.md` (from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`).

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO |  |
| AC2 | TODO |  |
| AC3 | TODO |  |

## Risks and Trade-offs

- A sibling slips. Mitigation: the fresh audit is the gate.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise record in the progress log why none was needed.

## References

- Parent EPIC: #2758
- Related issues: #2715, #1798
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`
