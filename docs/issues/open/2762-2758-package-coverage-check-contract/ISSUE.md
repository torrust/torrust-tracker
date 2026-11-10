---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2758
github-issue: 2762
spec-path: docs/issues/open/2762-2758-package-coverage-check-contract/ISSUE.md
branch: "2762-package-coverage-check-contract"
related-pr: null
last-updated-utc: "2026-11-10 12:00"
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

# Issue #2762 - `package-coverage-check`: NDJSON errors, usage exit 2, TTY refusal and a JSON `summary`

**Parent EPIC:** #2758 - Migrate the CLI tools to the global output contract. Order 3 of 11, wave 1.

## Goal

`package-coverage-check` follows the `stdout-result-data` contract for all three subcommands, and the coverage workflow consumes the new `summary` result.

## Background

`contrib/dev-tools/checks/package-coverage-check/src/main.rs` writes plain-text errors to stderr (`package-coverage-check: <error>`), exits `1` for every failure including a usage error, does not refuse a TTY, and `summary` prints Markdown. ADR `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md` (section 3) requires NDJSON errors, usage error exit `2`, TTY refusal, and `summary` returning one JSON object that carries the Markdown (for example in a `markdown` field). `.github/workflows/generate_coverage_pr.yaml` calls `matrix` (output into `$GITHUB_OUTPUT`), `compare` (redirected to a file) and `summary` (appended to `$GITHUB_STEP_SUMMARY`, around lines 49-54 and 150-160); the `summary` step must extract the field.

The global CLI output contract (`docs/adrs/20260519000000_define_global_cli_output_contract.md`), as amended by `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`, requires: result data only on stdout (one JSON object) for `stdout-result-data` commands, nothing on stdout for `no-stdout-result` commands; every stderr record is one JSON object per line (NDJSON) with a `kind`; no plain text on either channel; exit codes only `0` (did its job, the answer is in the result), `1` (could not do its job) and `2` (usage error or TTY refusal); and TTY refusal for `stdout-result-data` commands. A distinct outcome is a `kind` field: an answer to what the caller asked is a stdout result with exit `0`; "could not answer" is a stderr diagnostic with exit `1`. This issue is part of EPIC #2758, which migrates the tools found non-compliant by the 2026-11-06 audit. The reference implementations are `contrib/dev-tools/checks/ci-gate/src/main.rs` (`stdout-result-data`) and `contrib/dev-tools/checks/shipped-features-check/src/main.rs` (`no-stdout-result`).

## Scope

### In Scope

- NDJSON error records with a `kind` on stderr for every failure; empty stdout on failure.
- Usage errors exit `2`; runtime failures exit `1`.
- TTY refusal (exit `2`, `tty_refusal` record) for all subcommands.
- `summary` emits one JSON object with the Markdown in a `markdown` field; update `generate_coverage_pr.yaml` to extract it (for example `jq -r .markdown`).
- Tests for the new shapes.

### Out of Scope

- Changing the coverage computation or the `matrix` and `compare` result schemas beyond what the contract needs.
- Editing the ADR.

Files this issue may touch (a sibling issue in the same wave must not touch them):

- `contrib/dev-tools/checks/package-coverage-check/**`
- `.github/workflows/generate_coverage_pr.yaml`

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

Unit tests of the error and exit-code mapping in `package-coverage-check`, plus a CLI test for the TTY refusal and the `summary` JSON shape. They fail today (plain-text errors, exit `1` for usage). If a higher boundary is used instead, record why it is clearer or the only practical option; link red and green output and the final recheck to `manual-verification-evidence.md`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | DONE | Failing tests for NDJSON errors, exit codes and TTY refusal | Red tests. |
| T2 | DONE | Implement the error and exit-code changes | Green tests. |
| T3 | DONE | Change `summary` to a JSON result with `markdown` | Test asserts the field. |
| T4 | DONE | Update the workflow to extract the field | `actionlint`-clean; step summary still renders. |
| T5 | DONE | Exercise the three subcommands locally on two real revisions | Evidence recorded. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Failing tests for NDJSON errors, exit codes and TTY refusal | Commit after focused validation and required review. |
| T2 | Implement the error and exit-code changes | Commit after focused validation and required review. |
| T3 | Change `summary` to a JSON result with `markdown` | Commit after focused validation and required review. |
| T4 | Update the workflow to extract the field | Commit after focused validation and required review. |
| T5 | Exercise the three subcommands locally on two real revisions | Commit after focused validation and required review. |

Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG. For test-producing work, use the `write-unit-test` skill and complete the design review it requires before each commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2758-package-coverage-check-contract/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-06 16:43 UTC - Claude Code (Claude Sonnet 5.5) - Drafted this spec as part of EPIC #2758 from the 2026-11-06 audit.
- 2026-11-06 16:51 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the spec; GitHub issue #2762 created.
- 2026-11-09 10:25 UTC - Claude Code (Claude Sonnet 5.5) - Corrected details after the review rounds on PR 2771 and PR 2774 and fixed the earlier log stamps from the commit and issue creation times.
- 2026-11-09 11:26 UTC - Claude Code (Claude Sonnet 5.5) - Corrected details after the round 3 review of PR 2774.
- 2026-11-09 12:12 UTC - Claude Code (Claude Sonnet 5.5) - Addressed the review of PR 2771 (da2ce7 findings F1 to F27 and Copilot's later comments): specs corrected from the code, reproductions recorded where a spec has a bug-fix section.
- 2026-11-10 12:00 UTC - Claude Code (Claude Sonnet 5.5) - Implemented: NDJSON errors, usage and TTY exit `2`, JSON `summary` with `markdown`, workflow extracts it; evidence recorded.

## Acceptance Criteria

- [x] AC1: Every failure writes one JSON record with a `kind` to stderr and nothing to stdout.
- [x] AC2: A usage error exits `2`; a runtime failure exits `1`.
- [x] AC3: Running with stdout on a TTY exits `2` with a `tty_refusal` record.
- [x] AC4: `summary` prints one JSON object whose `markdown` field is the former report.
- [x] AC5: The workflow step summary shows the same Markdown as before.
- [ ] `linter all` exits with code `0`
- [x] Relevant tests pass
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [x] Documentation is updated when behavior or workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- Relevant tests for changed components (`cargo test -p <package>`)
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Usage error | `cargo build -p package-coverage-check`, then `target/debug/package-coverage-check bogus > out 2> err; echo "exit=$?"; wc -c out` (the built binary, not `cargo run`, which adds plain cargo messages to stderr) | JSON record in `err`, `out` empty, exit `2` | DONE | `manual-verification-evidence.md` section V1 |
| M2 | Summary round trip | Run `summary` on a sample discovery and results folder, redirect stdout to a file and run `jq -r .markdown` on it | Markdown identical to the previous output | DONE | `manual-verification-evidence.md` section V2 |
| M3 | Workflow syntax | `actionlint` (or the repo's workflow linter) on the workflow | No findings | DONE | `manual-verification-evidence.md` section V3 |

Manual verification is mandatory even when automated tests pass. Record actual commands, output and the toolchain that produced each result in `manual-verification-evidence.md` (from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`).

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | V1; `tests/cli.rs` |
| AC2 | DONE | V1; `tests/cli.rs`, `main.rs` unit test |
| AC3 | DONE | V1 (pseudo-terminal via `script`) |
| AC4 | DONE | V2; `tests/cli.rs` |
| AC5 | DONE | V2, V3 (same Markdown via `jq -r .markdown`; confirmed on the PR run) |

## Risks and Trade-offs

- The workflow change is exercised by this PR's own `pull_request` run (`generate_coverage_pr.yaml` checks out the head sha), but the `summary` step only shows its result in the job summary. Mitigation: also reproduce the three calls locally and keep the diff to the one extraction.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise record in the progress log why none was needed.

## References

- Parent EPIC: #2758
- Related issues: #2715, #1798
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`
