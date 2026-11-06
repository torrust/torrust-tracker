---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2758
github-issue: 2768
spec-path: docs/issues/open/2768-2758-tracker-client-json-only/ISSUE.md
branch: "2768-tracker-client-json-only"
related-pr: null
last-updated-utc: "2026-11-06 17:10"
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

# Issue #2768 - `tracker_client`: TTY refusal and JSON-only output

**Parent EPIC:** #2758 - Migrate the CLI tools to the global output contract. Order 9 of 11, wave 2.

## Goal

`tracker_client` refuses a TTY, writes only JSON on both channels, and has no plain-text output path.

## Background

Depends on the removal of the deprecated binaries (EPIC issue 6), which shares `console/tracker-client/**`. `console/tracker-client/src/bin/tracker_client.rs` has no TTY refusal (`tracker_client` has no `is_terminal` check; `ci-gate`, `wait-for-pr-review` and `github-review-threads` do). `unified/app.rs` initializes `tracing_subscriber::fmt()` whose default writer is stdout, in plain text. `check --format text` is not plain text: the `Text` format renders pretty-printed JSON (`unified/check.rs`), so it already satisfies the contract and only needs to keep doing so; the console layer (`checker/console.rs`, `checker/monitor/udp.rs`, `http/app.rs`, `udp/app.rs`, `unified/{check,http,udp}.rs`, `lib.rs`) still uses direct prints under `allow`s. The ADR table says `tracker_client` is `stdout-result-data` with monitor progress as NDJSON on stderr. Pretty-printed JSON stays allowed (it is JSON); the maintainer may still prefer to rename the misleading `Text` format value.

The global CLI output contract (`docs/adrs/20260519000000_define_global_cli_output_contract.md`), as amended by `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`, requires: result data only on stdout (one JSON object) for `stdout-result-data` commands, nothing on stdout for `no-stdout-result` commands; every stderr record is one JSON object per line (NDJSON) with a `kind`; no plain text on either channel; exit codes only `0` (did its job, the answer is in the result), `1` (could not do its job) and `2` (usage error or TTY refusal); and TTY refusal for `stdout-result-data` commands. A distinct outcome is a `kind` field: an answer to what the caller asked is a stdout result with exit `0`; "could not answer" is a stderr diagnostic with exit `1`. This issue is part of EPIC #2758, which migrates the tools found non-compliant by the 2026-11-06 audit. The reference implementations are `contrib/dev-tools/checks/ci-gate/src/main.rs` (`stdout-result-data`) and `contrib/dev-tools/checks/shipped-features-check/src/main.rs` (`no-stdout-result`).

## Scope

### In Scope

- TTY refusal at entry (exit `2`, `tty_refusal` record).
- `tracing` to stderr as JSON; stdout carries only the result object.
- Remove the direct print macros and the `print_*` allows in the console layer, writing results through one JSON emitter; check `checker/console.rs`, `checker/monitor/udp.rs`, `http/app.rs`, `udp/app.rs` and `unified/*` for any output that is not JSON.
- Update the tracker-client I/O contract doc and README; keep tests green.

### Out of Scope

- Removing the deprecated binaries (issue 6).
- Changing request or protocol behavior.

Files this issue may touch (a sibling issue in the same wave must not touch them):

- `console/tracker-client/**`
- `console/tracker-client/docs/contracts/tracker-cli-io-contract.md`

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

CLI tests for the TTY refusal and for stdout/stderr content of `tracker_client` subcommands (stdout one JSON object, stderr JSON). The TTY test needs a pseudo-terminal or an injected check; record which. If a higher boundary is used instead, record why it is clearer or the only practical option; link red and green output and the final recheck to `manual-verification-evidence.md`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | TODO | Wire TTY refusal | Test: exit `2` on a terminal (pty test or injected check). |
| T2 | TODO | Move tracing to stderr JSON | stdout only has the result. |
| T3 | TODO | Remove or convert plain-text output | No `print_*` allow left. |
| T4 | TODO | Update docs and tests | Green. |
| T5 | TODO | Verify against local UDP and HTTP trackers | Evidence recorded. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Wire TTY refusal | Commit after focused validation and required review. |
| T2 | Move tracing to stderr JSON | Commit after focused validation and required review. |
| T3 | Remove or convert plain-text output | Commit after focused validation and required review. |
| T4 | Update docs and tests | Commit after focused validation and required review. |
| T5 | Verify against local UDP and HTTP trackers | Commit after focused validation and required review. |

Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG. For test-producing work, use the `write-unit-test` skill and complete the design review it requires before each commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2758-tracker-client-json-only/ISSUE.md`
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
- 2026-11-06 17:25 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the spec; GitHub issue #2768 created.

## Acceptance Criteria

- [ ] AC1: Running `tracker_client` with stdout on a TTY exits `2` with a `tty_refusal` record.
- [ ] AC2: stdout contains exactly one JSON object on success and nothing on failure.
- [ ] AC3: Every stderr line is JSON.
- [ ] AC4: No direct print macro or `allow` for them remains in the crate.
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
| M1 | Announce and scrape | Run `tracker_client http announce ... \| jq .` and the UDP equivalent | One JSON object; stderr JSON | TODO | `manual-verification-evidence.md` section V1 |
| M2 | TTY refusal | Run in a terminal without piping | Exit `2`, JSON on stderr | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Check subcommand | `tracker_client check ... \| jq .` | JSON result, no plain text | TODO | `manual-verification-evidence.md` section V3 |

Manual verification is mandatory even when automated tests pass. Record actual commands, output and the toolchain that produced each result in `manual-verification-evidence.md` (from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`).

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO |  |
| AC2 | TODO |  |
| AC3 | TODO |  |
| AC4 | TODO |  |

## Risks and Trade-offs

- Callers use `--format text`. Mitigation: it already prints JSON, so keep it working; only a rename would be a breaking change.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise record in the progress log why none was needed.

## References

- Parent EPIC: #2758
- Related issues: #2715, #1798
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`
