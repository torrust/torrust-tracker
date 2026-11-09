---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2758
github-issue: 2763
spec-path: docs/issues/open/2763-2758-dev-tool-checks-ndjson/ISSUE.md
branch: "2763-dev-tool-checks-ndjson"
related-pr: null
last-updated-utc: "2026-11-09 12:12"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md
    - docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #2763 - `agent-review-report-contract` and `frontmatter-schema`: NDJSON diagnostics; classify `workspace-coupling`

**Parent EPIC:** #2758 - Migrate the CLI tools to the global output contract. Order 4 of 11, wave 1.

## Goal

The two plain-text dev-tool checks follow the `no-stdout-result` contract, and `workspace-coupling` is checked and, if needed, fixed.

## Background

`contrib/dev-tools/checks/agent-review-report-contract/src/main.rs` writes a plain-text success line to stdout and plain-text failures to stderr. `contrib/dev-tools/checks/frontmatter-validator/src/bin/frontmatter-schema.rs` writes `frontmatter-schema: <error>` to stderr (usage already exits `2`). Neither is in the ADR table (EPIC issue 1 adds them as `no-stdout-result`). `contrib/dev-tools/analysis/workspace-coupling/src/main.rs` already emits NDJSON on stderr with exit codes for usage and runtime; verify where its Markdown report goes (file argument versus stdout) and whether stdout stays empty. Verify who calls each tool (workflows, hooks, `docs/packages.md`) and update those callers.

The global CLI output contract (`docs/adrs/20260519000000_define_global_cli_output_contract.md`), as amended by `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`, requires: result data only on stdout (one JSON object) for `stdout-result-data` commands, nothing on stdout for `no-stdout-result` commands; every stderr record is one JSON object per line (NDJSON) with a `kind`; no plain text on either channel; exit codes only `0` (did its job, the answer is in the result), `1` (could not do its job) and `2` (usage error or TTY refusal); and TTY refusal for `stdout-result-data` commands. A distinct outcome is a `kind` field: an answer to what the caller asked is a stdout result with exit `0`; "could not answer" is a stderr diagnostic with exit `1`. This issue is part of EPIC #2758, which migrates the tools found non-compliant by the 2026-11-06 audit. The reference implementations are `contrib/dev-tools/checks/ci-gate/src/main.rs` (`stdout-result-data`) and `contrib/dev-tools/checks/shipped-features-check/src/main.rs` (`no-stdout-result`).

## Scope

### In Scope

- `agent-review-report-contract`: nothing on stdout; one NDJSON record per failure on stderr (with `kind`); exit `0` when clean, `1` on a finding or failure.
- `frontmatter-schema`: NDJSON diagnostics on stderr; usage stays `2`.
- `workspace-coupling`: it is `no-stdout-result` (ADR row from #2760): verify stdout stays empty and the report goes only to the output file; fix only what the check finds and record the outcome.
- Update tests and any docs describing the output.

### Out of Scope

- Changing what the checks validate.
- Editing the ADR.

Files this issue may touch (a sibling issue in the same wave must not touch them):

- `contrib/dev-tools/checks/agent-review-report-contract/**`
- `contrib/dev-tools/checks/frontmatter-validator/src/bin/frontmatter-schema.rs` (and its tests)
- `contrib/dev-tools/analysis/workspace-coupling/**`

## Architectural Decisions

Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`. ADRs to create: None known.

During implementation, stop and create an ADR when a decision affects project architecture or design patterns, selects an approach among meaningful alternatives, or has consequences future contributors need to understand. Do not create ADRs for routine implementation details.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

This issue fixes a deviation from the CLI output contract, so it is bug work in substance. Follow the [fix-bug](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md) skill:

- analysis of the defect: the tool's current behavior against the contract (see Background);
- real-artifact reproduction: run the real binary, record the commands and output of the deviation in `manual-verification-evidence.md` before changing code;
- regression-test boundary: see Regression Test Strategy;
- red regression test, then the production fix;
- green test evidence and a like-for-like recheck of the same real-artifact commands.

## Regression Test Strategy

Unit tests of each tool's `run` function with an injected writer (the pattern `frontmatter-schema` already uses), asserting NDJSON on stderr and empty stdout. They fail today (plain text). If a higher boundary is used instead, record why it is clearer or the only practical option; link red and green output and the final recheck to `manual-verification-evidence.md`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | TODO | Find every caller of the three tools | List recorded in the progress log. |
| T2 | TODO | Convert `agent-review-report-contract` | Tests green; stdout empty. |
| T3 | TODO | Convert `frontmatter-schema` | Tests green; usage still `2`. |
| T4 | TODO | Check `workspace-coupling` against the contract | Fix or record compliance. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Find every caller of the three tools | Commit after focused validation and required review. |
| T2 | Convert `agent-review-report-contract` | Commit after focused validation and required review. |
| T3 | Convert `frontmatter-schema` | Commit after focused validation and required review. |
| T4 | Check `workspace-coupling` against the contract | Commit after focused validation and required review. |

Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG. For test-producing work, use the `write-unit-test` skill and complete the design review it requires before each commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2758-dev-tool-checks-ndjson/ISSUE.md`
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

- 2026-11-06 16:43 UTC - Claude Code (Claude Sonnet 5.5) - Drafted this spec as part of EPIC #2758 from the 2026-11-06 audit.
- 2026-11-06 16:51 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the spec; GitHub issue #2763 created.
- 2026-11-09 10:25 UTC - Claude Code (Claude Sonnet 5.5) - Corrected details after the review rounds on PR 2771 and PR 2774 and fixed the earlier log stamps from the commit and issue creation times.
- 2026-11-09 12:12 UTC - Claude Code (Claude Sonnet 5.5) - Addressed the review of PR 2771 (da2ce7 findings F1 to F27 and Copilot's later comments): specs corrected from the code, reproductions recorded where a spec has a bug-fix section.

## Acceptance Criteria

- [ ] AC1: Neither converted tool writes plain text on any channel.
- [ ] AC2: On success stdout is empty and the exit code is `0`; on a finding the exit code is `1` with NDJSON records.
- [ ] AC3: `workspace-coupling` matches its ADR row (`no-stdout-result`): stdout stays empty, the Markdown report goes to the file argument, status events are NDJSON on stderr.
- [ ] AC4: Callers are unaffected or updated.
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
| M1 | Failing check | Break a required text in a scratch copy and run `agent-review-report-contract 2>err; echo $?` | stdout empty, `err` is NDJSON, exit `1` | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Schema check | `frontmatter-schema check` on the tracked schema, then with a bad argument | Exit `0`; usage error exit `2` with a JSON record | TODO | `manual-verification-evidence.md` section V2 |

Manual verification is mandatory even when automated tests pass. Record actual commands, output and the toolchain that produced each result in `manual-verification-evidence.md` (from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`).

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO |  |
| AC2 | TODO |  |
| AC3 | TODO |  |
| AC4 | TODO |  |

## Risks and Trade-offs

- A hook or workflow greps the old text. Mitigation: T1 lists callers first.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise record in the progress log why none was needed.

## References

- Parent EPIC: #2758
- Related issues: #2715, #1798
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`
