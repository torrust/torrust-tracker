---
schema-version: 1
doc-type: issue
issue-type: task
status: done
priority: p2
epic: 2758
github-issue: 2761
spec-path: docs/issues/closed/2761-2758-reply-status-result/ISSUE.md
branch: "2761-reply-status-result"
related-pr: 2775
last-updated-utc: "2026-11-09 10:23"
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

# Issue #2761 - `github-review-threads reply-status`: a missing reply is a result, not a failure

**Parent EPIC:** #2758 - Migrate the CLI tools to the global output contract. Order 2 of 11, wave 1.

## Goal

`github-review-threads reply-status` reports threads without a reply as a JSON result on stdout with a `kind` and exit `0`, and the skills that rely on the old exit `1` are updated in the same change.

## Background

Today `reply-status` exits `1` and writes a `missing_reply` diagnostic to stderr when an unresolved thread has no reply (`contrib/dev-tools/github/github-review-threads/src/main.rs`, around the `missing_reply` record, and `tests/cli.rs`). The ADR `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md` says a thread without a reply is an answer to what the caller asked, so it is a result with exit `0`. Failure to fetch or parse stays exit `1`, a usage error exit `2`. `resolve-review-threads/SKILL.md` (Batch Pattern and checklist) says the command 'exits with code 1 and a `missing_reply` JSON record'; `fetch-review-threads/SKILL.md` also describes `reply-status`. The batch scripts under `resolve-review-threads/scripts/` do not call `reply-status`; verify that. Callers must now branch on the result `kind` (for example with `jq -e`), not on the exit code.

The global CLI output contract (`docs/adrs/20260519000000_define_global_cli_output_contract.md`), as amended by `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`, requires: result data only on stdout (one JSON object) for `stdout-result-data` commands, nothing on stdout for `no-stdout-result` commands; every stderr record is one JSON object per line (NDJSON) with a `kind`; no plain text on either channel; exit codes only `0` (did its job, the answer is in the result), `1` (could not do its job) and `2` (usage error or TTY refusal); and TTY refusal for `stdout-result-data` commands. A distinct outcome is a `kind` field: an answer to what the caller asked is a stdout result with exit `0`; "could not answer" is a stderr diagnostic with exit `1`. This issue is part of EPIC #2758, which migrates the tools found non-compliant by the 2026-11-06 audit. The reference implementations are `contrib/dev-tools/checks/ci-gate/src/main.rs` (`stdout-result-data`) and `contrib/dev-tools/checks/shipped-features-check/src/main.rs` (`no-stdout-result`).

## Scope

### In Scope

- Define the result: one JSON object on stdout with a `kind` (for example `all_replied` and `missing_reply`) and the per-thread rows (`has_reply`).
- Exit `0` for both kinds; `1` only if the tool could not answer; `2` for usage and TTY refusal (unchanged).
- Update unit and CLI tests, including the fixture-based tests in `tests/cli.rs`.
- Update `resolve-review-threads/SKILL.md`, `fetch-review-threads/SKILL.md` and any other skill or doc that cites the exit `1`, with the `jq -e` branching recipe.

### Out of Scope

- Changing `fetch`, `list` or `show` output.
- Editing the ADR (the close-out issue does).
- Changing the batch resolver scripts unless they are found to depend on the exit code.

Files this issue may touch (a sibling issue in the same wave must not touch them):

- `contrib/dev-tools/github/github-review-threads/**`
- `.github/skills/dev/pr-reviews/resolve-review-threads/SKILL.md`
- `.github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md`

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
| T1 | DONE | Write failing tests for the new result shape and exit `0` | Red tests for both kinds. |
| T2 | DONE | Change `reply-status` to emit the result and exit `0` | Green tests; diagnostics only for could-not-answer. |
| T3 | DONE | Update the skills and any other consumer | `grep -rn 'missing_reply\|reply-status'` shows no stale exit-1 claim. |
| T4 | DONE | Verify against a real PR thread file | Evidence recorded. |

## Test Design Review (Prose-First AAA)

Completed on 2026-11-06 at about 21:03 UTC, in review round 1 of PR #2775 (it was not recorded before the first commit, as the `write-unit-test` skill requires; this closes that gap). For each new or changed test in `contrib/dev-tools/github/github-review-threads/tests/cli.rs` the prose Arrange, Act and Assert were written and compared with the code:

| Test | Prose (Arrange / Act / Assert) | Outcome of the comparison |
| ---- | ------------------------------ | ------------------------- |
| `it_should_report_the_threads_without_a_reply_as_a_result_with_exit_zero` | The fixture has two unresolved threads and `author` replied to one; ask `reply-status` for `author`; exit 0, empty stderr, `kind` `missing_reply`, rows and counts equal to the retired script's capture. | Code states each paragraph; expected rows come from the capture, not from production code. No change. |
| `it_should_report_all_replied_as_a_result_with_exit_zero_when_every_thread_has_a_reply` | `reviewer` commented on both unresolved threads; ask for `reviewer`; exit 0, `kind` `all_replied`, zero without reply, empty stderr. | Matches. No change. |
| `it_should_fail_with_exit_one_and_a_diagnostic_when_the_threads_file_cannot_be_read` | A path that does not exist; ask for the status; exit 1, empty stdout, `reply_status_error`. | Matches. No change. |
| `it_should_fail_with_exit_one_and_a_diagnostic_when_the_threads_file_is_not_valid_json` | A readable file with malformed JSON; same action; same result. | Added after Copilot finding F1. |
| `it_should_report_a_missing_login_argument_as_a_usage_error` | A valid threads file and no `--login`; run; exit 2, empty stdout, a usage error that names `--login` as the only missing argument. | Arrange was too broad (it also omitted `--threads-file`); narrowed after Copilot finding F2. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Write failing tests for the new result shape and exit `0` | Commit after focused validation and required review. |
| T2 | Change `reply-status` to emit the result and exit `0` | Commit after focused validation and required review. |
| T3 | Update the skills and any other consumer | Commit after focused validation and required review. |
| T4 | Verify against a real PR thread file | Commit after focused validation and required review. |

Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG. For test-producing work, use the `write-unit-test` skill and complete the design review it requires before each commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2758-reply-status-result/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation (did not happen: the spec reached `develop` together with the implementation in `646d1cab2`, which was stacked on spec PR #2771)
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-06 17:10 UTC - Claude Code (Claude Sonnet 5.5) - Drafted this spec as part of EPIC #2758 from the 2026-11-06 audit.
- 2026-11-06 17:25 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the spec; GitHub issue #2761 created.
- 2026-11-06 17:18 UTC - Claude Code (Claude Sonnet 5.5) - Implemented: kinds `all_replied` / `missing_reply`, result `{kind, threads, summary}` (no `message`), exit 0 for both; batch scripts do not call `reply-status`; both skills updated; manual evidence recorded.
- 2026-11-09 10:19 UTC - PR #2775 merged (merge commit `646d1cab2`); issue #2761 closed on GitHub.
- 2026-11-09 10:23 UTC - Claude Code (Claude Sonnet 5.5) - Archived this spec to `docs/issues/closed/`: `status: done`, `spec-path` and `related-pr` updated, checkpoints ticked, EPIC table updated. The "Reviewer validated acceptance criteria" checkpoint stays unticked: no separate reviewer sign-off was recorded beyond the Copilot review audit and the merge.

## Acceptance Criteria

- [x] AC1: `reply-status` on a threads file with a thread without a reply prints one JSON result with a `kind` and exits `0`.
- [x] AC2: On a fully replied file it prints the matching `kind` and exits `0`.
- [x] AC3: A fetch or parse failure exits `1` with empty stdout and a JSON diagnostic; a usage error exits `2`.
- [x] AC4: The skills describe the new branching and no document claims exit `1` for a missing reply.
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
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
| M1 | Thread without a reply | `cargo run -p github-review-threads -- reply-status --threads-file <fixture> --login author \| jq -e '.kind=="missing_reply"'` | jq succeeds and `$?` of the tool was 0 | DONE | `manual-verification-evidence.md` section V1 |
| M2 | Usage error | Run `reply-status` with no arguments, stdout piped | Exit `2`, JSON on stderr, stdout empty | DONE | `manual-verification-evidence.md` section V2 |

Manual verification is mandatory even when automated tests pass. Record actual commands, output and the toolchain that produced each result in `manual-verification-evidence.md` (from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`).

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | manual-verification-evidence.md V1; tests/cli.rs |
| AC2 | DONE | manual-verification-evidence.md V1; tests/cli.rs |
| AC3 | DONE | manual-verification-evidence.md V2; tests/cli.rs |
| AC4 | DONE | `grep -rn "missing_reply\|reply-status" .github` shows both skills describing `kind` branching |

## Risks and Trade-offs

- Callers that tested `$?` break silently. Mitigation: the only in-repo callers are the two skills, which this issue updates; `grep` for others is a task.

## Implementation Completion Review

- Retrospective: Not needed; the change was small and matched the spec. Two process gaps: the spec reached `develop` only together with the implementation (not through its own spec-only PR), and the prose-first test design review was recorded only in review round 1 of PR #2775 (see the "Test Design Review" section).
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise record in the progress log why none was needed.

## References

- Parent EPIC: #2758
- Related issues: #2715, #1798
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`
