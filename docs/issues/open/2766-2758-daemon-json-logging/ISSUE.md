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
last-updated-utc: "2026-11-09 16:03"
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

# Issue #2766 - `torrust-tracker` daemon: JSON logs on stderr and JSON startup failure

**Parent EPIC:** #2758 - Migrate the CLI tools to the global output contract. Order 7 of 11, wave 1.

## Goal

The daemon writes its `tracing` output as JSON lines on stderr by default and reports a startup failure as a JSON record, as the contract requires of a `no-stdout-result` daemon.

## Background

`packages/configuration/src/v3_0_0/logging.rs` (and the older versions and `packages/configuration/src/logging.rs`) initialize `tracing_subscriber::fmt()` with `with_test_writer()` and ANSI, which writes to stdout in the `full` style by default (`trace_style`, default `Full`). `src/main.rs` reports startup failure with `eprintln!("Tracker startup failed: {error}")`. The contract requires tracing on stderr as NDJSON, nothing on stdout. Maintainer decision (2026-11-06): migrate now, not deferred. This is operator-visible just after the 3.0.0 release: deployments that parse the plain text or read stdout break. The `trace_style` option and config schema versions are public, so decide the migration detail with the maintainer before writing code (default `json` versus keeping `trace_style` meaning, stream, ANSI, deprecation of the other styles, versioned-config handling) and record it, with an ADR if it is a material decision. The e2e runner parses the container logs in `src/console/ci/e2e/logs_parser.rs`; that parser must follow the new format. Search for other readers of log text (`docs/benchmarking.md`, container docs, tests with `with_test_writer`).

The global CLI output contract (`docs/adrs/20260519000000_define_global_cli_output_contract.md`), as amended by `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`, requires: result data only on stdout (one JSON object) for `stdout-result-data` commands, nothing on stdout for `no-stdout-result` commands; every stderr record is one JSON object per line (NDJSON) with a `kind`; no plain text on either channel; exit codes only `0` (did its job, the answer is in the result), `1` (could not do its job) and `2` (usage error or TTY refusal); and TTY refusal for `stdout-result-data` commands. A distinct outcome is a `kind` field: an answer to what the caller asked is a stdout result with exit `0`; "could not answer" is a stderr diagnostic with exit `1`. This issue is part of EPIC #2758, which migrates the tools found non-compliant by the 2026-11-06 audit. The reference implementations are `contrib/dev-tools/checks/ci-gate/src/main.rs` (`stdout-result-data`) and `contrib/dev-tools/checks/shipped-features-check/src/main.rs` (`no-stdout-result`).

## Scope

### In Scope

- Send `tracing` output to stderr; leave stdout empty.
- Make JSON the default style (or the agreed alternative) and keep the other `trace_style` values only if the decision keeps them.
- Startup failure as one JSON record on stderr, exit `1`; usage and config errors exit `2` if they are usage errors.
- Update `logs_parser.rs` and its tests; update docs, default configs and the changelog or release-note convention.
- `Docker::logs` in `src/console/ci/e2e/docker.rs` returns only the container's stdout; the daemon's logs move to stderr, so it must capture stderr too (or both streams) and its tests must follow.
- Exit codes for a startup failure: configuration, semantic-validation and persistence-requirement errors are usage errors, exit `2`; a runtime failure such as a bind error or a composition failure is exit `1`. Today every startup failure returns `ExitCode::FAILURE` (`1`), so `src/main.rs` needs the mapping.
- `Cli::parse()` in `src/main.rs` lets clap print plain-text argument errors on stderr and help on stdout. Use fallible parsing and emit JSON diagnostics and help on stderr instead (usage errors exit `2`), with automated and manual cases for `--unknown`, `-c ""` and `--help`.
- The root tests that pin today's behavior: `tests/configuration/cli_configuration/invalid_sources.rs` (lines 60, 76, 92, 109, 127) assert exit `1` through `STARTUP_FAILURE_EXIT_CODE` (`tests/common/native_tracker/failed_start.rs:21`) and become exit `2` for configuration failures; `tests/common/native_tracker/health.rs` (line 68) takes everything after `Started on: http://` as the address, which a JSON line breaks, so the harness's readiness detection must follow the new format; `tests/lifecycle/shutdown_policy.rs:110` asserts `timeout_seconds=25` on the formatted text.
- Startup failure reports. A runtime failure after logging has started produces two failure reports today (`src/main.rs:45` logs `tracing::error!(%error, "Tracker startup failed")` and `src/main.rs:46` writes `report_startup_failure`), after the startup records already written ("Logging initialized" first). A configuration failure happens before logging starts (`bootstrap::app::setup` loads and validates the configuration before `initialize_global_services`, `src/bootstrap/app.rs:56-64`), so only the `:46` line is written, with no "Logging initialized" before it (the evidence file shows this). In both cases keep exactly one record that carries the failure `kind` (remove the duplicate report) and state in the spec what the stream contains.
- Keep test-helper logging working (`packages/test-helpers/src/logging.rs`).

### Out of Scope

- The e2e runners' own logging (EPIC issue 10).
- Changing log levels or thresholds.

Files this issue may touch (a sibling issue in the same wave must not touch them):

- `packages/configuration/src/logging.rs`
- `packages/configuration/src/v3_0_0/logging.rs` (and v2_0_0 if still supported)
- `src/main.rs`
- `tests/configuration/cli_configuration/invalid_sources.rs`, `tests/common/native_tracker/failed_start.rs`, `tests/common/native_tracker/health.rs`, `tests/lifecycle/shutdown_policy.rs`
- `src/console/ci/e2e/logs_parser.rs`
- `src/console/ci/e2e/docker.rs`
- the default configs under `share/default/config/` that set `[logging]` (not `share/default/config/tracker_checker.json`, which issue 2765 owns)
- the docs that show the log format

## Architectural Decisions

Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`. ADRs to create: probably one, for the default style, stream and config compatibility (decide with the maintainer in review).

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

Unit tests of the logging initialization (writer and format) and of the startup-failure record and exit-code mapping, plus the updated `logs_parser.rs` tests and the root tests listed in Scope, updated to the new output. The new unit tests fail today (stdout, plain text, exit `1` for configuration errors); the existing root tests pass today and pin the old output, so they are changed rather than added. If a higher boundary is used instead, record why it is clearer or the only practical option; link red and green output and the final recheck to `manual-verification-evidence.md`.

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

- 2026-11-06 16:43 UTC - Claude Code (Claude Sonnet 5.5) - Drafted this spec as part of EPIC #2758 from the 2026-11-06 audit.
- 2026-11-06 16:52 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the spec; GitHub issue #2766 created.
- 2026-11-09 10:25 UTC - Claude Code (Claude Sonnet 5.5) - Corrected details after the review rounds on PR 2771 and PR 2774 and fixed the earlier log stamps from the commit and issue creation times.
- 2026-11-09 12:12 UTC - Claude Code (Claude Sonnet 5.5) - Addressed the review of PR 2771 (da2ce7 findings F1 to F27 and Copilot's later comments): specs corrected from the code, reproductions recorded where a spec has a bug-fix section.
- 2026-11-09 13:21 UTC - Claude Code (Claude Sonnet 5.5) - Addressed da2ce7's review 5826238636 of PR 2771 on this spec.
- 2026-11-09 16:03 UTC - Claude Code (Claude Sonnet 5.5) - Addressed da2ce7's review 5826857760 of PR 2771 on this spec.

## Acceptance Criteria

- [ ] AC1: Running the tracker writes nothing to stdout.
- [ ] AC2: Every stderr line is a JSON object.
- [ ] AC3: A configuration error exits `2` and a runtime startup failure (for example a bind error) exits `1`; in both, exactly one record on stderr carries the failure `kind`.
- [ ] AC4: `logs_parser.rs` and its tests follow the new format.
- [ ] AC5: The operator-visible change is documented.
- [ ] AC6: Argument errors (`--unknown`, `-c ""`) and `--help` follow the contract (JSON on stderr, stdout empty, usage errors exit `2`), proven by M2c.
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
| M2 | Configuration error | Run with a bad config path | Exactly one failure record, exit `2` | TODO | `manual-verification-evidence.md` section V2 |
| M2b | Runtime startup failure | Run with a port already in use | Exactly one failure record, exit `1` | TODO | `manual-verification-evidence.md` section V2b |
| M2c | Argument errors and help | `torrust-tracker --unknown`, `torrust-tracker -c ""` and `torrust-tracker --help`, each with stdout and stderr redirected to files | Usage errors: JSON diagnostic on stderr, exit `2`, stdout empty; `--help`: JSON help on stderr, exit `0`, stdout empty (the ADR puts help text on stderr and gives the daemon no stdout), no plain text | TODO | `manual-verification-evidence.md` section V2c |
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
| AC6 | TODO |  |

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
