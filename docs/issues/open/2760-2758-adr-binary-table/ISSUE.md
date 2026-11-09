---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2758
github-issue: 2760
spec-path: docs/issues/open/2760-2758-adr-binary-table/ISSUE.md
branch: "2760-adr-binary-table"
related-pr: null
last-updated-utc: "2026-11-09 11:27"
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

# Issue #2760 - Align the ADR binary table with the audit

**Parent EPIC:** #2758 - Migrate the CLI tools to the global output contract. Order 1 of 11, wave 0.

## Goal

Make the binary table of the global CLI output contract ADR match the audit: add the missing in-scope programs, reclassify `http_health_check`, and list the daemon and the deprecated clients correctly.

## Background

The binary table in `docs/adrs/20260519000000_define_global_cli_output_contract.md` omits `agent-review-report-contract`, `frontmatter-schema` and `workspace-coupling`, calls `http_health_check` `stdout-result-data` although the container `HEALTHCHECK` needs a non-zero exit when unhealthy (maintainer decision 2026-11-06: reclassify as `no-stdout-result`), and does not say that the deprecated clients are to be removed. This issue changes only the table and the text that describes it; the tools themselves change in sibling issues. It runs first so the sibling issues cite a correct table, and no sibling edits this ADR (the close-out issue removes the 'currently non-compliant' notes).

The global CLI output contract (`docs/adrs/20260519000000_define_global_cli_output_contract.md`), as amended by `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`, requires: result data only on stdout (one JSON object) for `stdout-result-data` commands, nothing on stdout for `no-stdout-result` commands; every stderr record is one JSON object per line (NDJSON) with a `kind`; no plain text on either channel; exit codes only `0` (did its job, the answer is in the result), `1` (could not do its job) and `2` (usage error or TTY refusal); and TTY refusal for `stdout-result-data` commands. A distinct outcome is a `kind` field: an answer to what the caller asked is a stdout result with exit `0`; "could not answer" is a stderr diagnostic with exit `1`. This issue is part of EPIC #2758, which migrates the tools found non-compliant by the 2026-11-06 audit. The reference implementations are `contrib/dev-tools/checks/ci-gate/src/main.rs` (`stdout-result-data`) and `contrib/dev-tools/checks/shipped-features-check/src/main.rs` (`no-stdout-result`).

## Scope

### In Scope

- Add rows for `agent-review-report-contract` (`no-stdout-result`), `frontmatter-schema` (`no-stdout-result`) and `workspace-coupling` (class decided after reading where its report goes; record the decision).
- Reclassify `http_health_check` as `no-stdout-result` and say why (container `HEALTHCHECK`).
- Mark the table notes that describe current non-compliance, so each names its migration issue.
- Keep the ADR's `Date` and amendment pointer coherent; add a short dated note about this alignment.

### Out of Scope

- Any code change.
- Editing `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`.

Files this issue may touch (a sibling issue in the same wave must not touch them):

- `docs/adrs/20260519000000_define_global_cli_output_contract.md`

## Architectural Decisions

Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`. ADRs to create: None known (this amends the table of an existing ADR; if the maintainer wants a new ADR instead, decide in review).

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
| T1 | DONE | Read where `workspace-coupling` writes its report and pick its class | Decision recorded in the progress log. |
| T2 | DONE | Edit the table and notes in the ADR | Rows added, `http_health_check` reclassified, notes name their issues. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Read where `workspace-coupling` writes its report and pick its class | Commit after focused validation and required review. |
| T2 | Edit the table and notes in the ADR | Commit after focused validation and required review. |

Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG. For test-producing work, use the `write-unit-test` skill and complete the design review it requires before each commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2758-adr-binary-table/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-06 16:51 UTC - Claude Code (Claude Sonnet 5.5) - Drafted this spec as part of EPIC #2758 from the 2026-11-06 audit; the maintainer approved it and GitHub issue #2760 was created (time from the issue `created_at` on GitHub; PR #2771 commits the spec at 16:54 UTC).
- 2026-11-06 17:16 UTC - Claude Code (Claude Sonnet 5.5) - T1 decision: `workspace-coupling` is `no-stdout-result` (report goes to a file, only NDJSON status on stderr). T2: table edited; `linter all` passed.
- 2026-11-06 20:53 UTC - Claude Code (Claude Sonnet 5.5) - Implementation completion review: the result matches the spec; the only decision to make during implementation was the `workspace-coupling` class (`no-stdout-result`, recorded above) and no assumption was invalidated, so no retrospective is needed. Evidence file rewritten to the template after the Copilot review of PR #2774.
- 2026-11-09 10:27 UTC - Claude Code (Claude Sonnet 5.5) - Ran `linter all` after the second review round edits (table `N/A` row, exit codes, evidence rewrite): exit 0; the output is pasted in `manual-verification-evidence.md` section V2.
- 2026-11-09 11:27 UTC - Claude Code (Claude Sonnet 5.5) - Ran `linter all` after the third review round edits (the `github-review-threads` note, evidence wording, audit rows), before adding this entry: exit 0.

## Acceptance Criteria

- [x] AC1: Every program audited in EPIC #2758 appears in the table with a class, or `N/A` for the deprecated binaries that are to be removed.
- [x] AC2: `http_health_check` is `no-stdout-result` and the note explains the `HEALTHCHECK` reason.
- [x] AC3: No row claims a tool is compliant when the audit says it is not.
- [x] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
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
| M1 | Compare the table with the audit | Read the table and the EPIC audit table side by side | Every audited program is listed with a matching class | DONE | `manual-verification-evidence.md` section V1 |

Manual verification is mandatory even when automated tests pass. Record actual commands, output and the toolchain that produced each result in `manual-verification-evidence.md` (from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`).

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | `manual-verification-evidence.md` V1 |
| AC2 | DONE | `manual-verification-evidence.md` V1 |
| AC3 | DONE | `manual-verification-evidence.md` V1 |

## Risks and Trade-offs

- The table changes while sibling issues are open. Mitigation: only this issue and the close-out edit it.

## Implementation Completion Review

- Retrospective: Not needed. See the progress log entry of 2026-11-06 20:53 UTC.
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise record in the progress log why none was needed.

## References

- Parent EPIC: #2758
- Related issues: #2715, #1798
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`
