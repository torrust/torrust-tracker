---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2758
github-issue: 2766
spec-path: docs/issues/open/2766-2758-daemon-json-logging/ISSUE.md
branch: "2766-daemon-json-logging"
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

# Issue #2766 - `torrust-tracker` daemon: JSON logs on stderr and JSON startup failure

**Parent EPIC:** #2758 - Migrate the CLI tools to the global output contract. Order 7 of 11, wave 1.

## Goal

The daemon writes its `tracing` output as JSON lines on stderr by default and reports a startup failure as a JSON record, as the contract requires of a `no-stdout-result` daemon.

## Background

`packages/configuration/src/v3_0_0/logging.rs` (and the older versions and `src/logging.rs`) initialize `tracing_subscriber::fmt()` with `with_test_writer()` and ANSI, which writes to stdout in the `full` style by default (`trace_style`, default `Full`). `src/main.rs` reports startup failure with `eprintln!("Tracker startup failed: {error}")`. The contract requires tracing on stderr as NDJSON, nothing on stdout. Maintainer decision (2026-11-06): migrate now, not deferred. This is operator-visible just after the 3.0.0 release: deployments that parse the plain text or read stdout break. The `trace_style` option and config schema versions are public, so decide the migration detail with the maintainer before writing code (default `json` versus keeping `trace_style` meaning, stream, ANSI, deprecation of the other styles, versioned-config handling) and record it, with an ADR if it is a material decision. The e2e runner parses the container logs in `src/console/ci/e2e/logs_parser.rs`; that parser must follow the new format. Search for other readers of log text (`docs/benchmarking.md`, container docs, tests with `with_test_writer`).

The global CLI output contract (`docs/adrs/20260519000000_define_global_cli_output_contract.md`), as amended by `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`, requires: result data only on stdout (one JSON object) for `stdout-result-data` commands, nothing on stdout for `no-stdout-result` commands; every stderr record is one JSON object per line (NDJSON) with a `kind`; no plain text on either channel; exit codes only `0` (did its job, the answer is in the result), `1` (could not do its job) and `2` (usage error or TTY refusal); and TTY refusal for `stdout-result-data` commands. A distinct outcome is a `kind` field: an answer to what the caller asked is a stdout result with exit `0`; "could not answer" is a stderr diagnostic with exit `1`. This issue is part of EPIC #2758, which migrates the tools found non-compliant by the 2026-11-06 audit. The reference implementations are `contrib/dev-tools/checks/ci-gate/src/main.rs` (`stdout-result-data`) and `contrib/dev-tools/checks/shipped-features-check/src/main.rs` (`no-stdout-result`).

## Scope

### In Scope

- Send `tracing` output to stderr; leave stdout empty.
- Make JSON the default style (or the agreed alternative) and keep the other `trace_style` values only if the decision keeps them.
- Startup failure as one JSON record on stderr, exit `1`; usage and config errors exit `2` if they are usage errors.
- Update `logs_parser.rs` and its tests; update docs, default configs and the changelog or release-note convention.
- Keep test-helper logging working (`packages/test-helpers/src/logging.rs`).

### Out of Scope

- The e2e runners' own logging (EPIC issue 10).
- Changing log levels or thresholds.

Files this issue may touch (a sibling issue in the same wave must not touch them):

- `packages/configuration/src/logging.rs`
- `packages/configuration/src/v3_0_0/logging.rs` (and v2_0_0 if still supported)
- `src/main.rs`
- `src/console/ci/e2e/logs_parser.rs`
- `share/default/config/**`
- the docs that show the log format

## Architectural Decisions

Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`. ADRs to create: probably one, for the default style, stream and config compatibility (decide with the maintainer in review).

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
| T1 | TODO | Agree the migration detail with the maintainer | Decision recorded in the spec; ADR if material. |
| T2 | TODO | Tests for stream and format | Red tests: stdout empty, stderr JSON. |
| T3 | TODO | Implement the logging change and the startup-failure record | Green tests. |
| T4 | TODO | Update `logs_parser.rs`, docs and default configs | e2e parser tests green. |
| T5 | TODO | Run the tracker locally and in a container | Evidence recorded. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Agree the migration detail with the maintainer | Commit after focused validation and required review. |
| T2 | Tests for stream and format | Commit after focused validation and required review. |
| T3 | Implement the logging change and the startup-failure record | Commit after focused validation and required review. |
| T4 | Update `logs_parser.rs`, docs and default configs | Commit after focused validation and required review. |
| T5 | Run the tracker locally and in a container | Commit after focused validation and required review. |

Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG. For test-producing work, use the `write-unit-test` skill and complete the design review it requires before each commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2758-daemon-json-logging/ISSUE.md`
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
- 2026-11-06 17:25 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the spec; GitHub issue #2766 created.

## Acceptance Criteria

- [ ] AC1: Running the tracker writes nothing to stdout.
- [ ] AC2: Every stderr line is a JSON object.
- [ ] AC3: A startup failure prints one JSON record and exits `1`.
- [ ] AC4: `logs_parser.rs` and its tests follow the new format.
- [ ] AC5: The operator-visible change is documented.
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
| M1 | Normal run | `torrust-tracker 2>err >out`, then stop it | `out` empty; `jq . err` parses every line | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Startup failure | Run with a bad config path or occupied port | One JSON record, exit `1` | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Container | Run the container image and read `docker logs` | JSON lines; e2e parser still finds the services | TODO | `manual-verification-evidence.md` section V3 |

Manual verification is mandatory even when automated tests pass. Record actual commands, output and the toolchain that produced each result in `manual-verification-evidence.md` (from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`).

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO |  |
| AC2 | TODO |  |
| AC3 | TODO |  |
| AC4 | TODO |  |
| AC5 | TODO |  |

## Risks and Trade-offs

- Operators parse the old text. Mitigation: release note and the agreed deprecation path.
- The e2e suite depends on log text. Mitigation: parser change is in scope and tested.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise record in the progress log why none was needed.

## References

- Parent EPIC: #2758
- Related issues: #2715, #1798
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`
