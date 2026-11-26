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
last-updated-utc: "2026-11-26 17:09"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .claude/skills/create-issue/SKILL.md
    - .claude/skills/fix-bug/SKILL.md
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md
    - docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - docs/issues/open/2766-2758-daemon-json-logging/decisions.md
---

<!-- skill-link: create-issue -->

# Issue #2766 - `torrust-tracker` daemon: JSON logs on stderr and JSON startup failure

**Parent EPIC:** #2758 - Migrate the CLI tools to the global output contract. Order 7 of 13, wave 1.

## Dependencies

This issue started after issue #3004 (EPIC #2758 schedule; closed 2026-11-24) and after sub-issue SI-1 of EPIC #2387 (issue #3031, a bug fix that does not depend on the log format; merged 2026-11-25 by PR 3043). Both are done, so nothing blocks this issue. The other sub-issues of EPIC #2387 (SI-2 to SI-23) run after this issue (maintainer decision, 2026-11-24, which replaces the decision of 2026-11-21 that this issue waits for SI-1 to SI-20; decision D1 in `docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md`). The log convention of EPIC #2387 does not depend on the format, so this issue changes the format and the stream of today's lines, and EPIC #2387 then fixes their content in the JSON format. Consequences for this issue:

- The log lines and the readers named in Background and Scope are as they are on `develop` at the refresh of 2026-11-26. `logs_parser.rs` and `tests/common/native_tracker/health.rs` still take the service URL from the `Started on: …` text, and `tests/lifecycle/shutdown_policy.rs` still asserts `timeout_seconds`. This issue moves these readers to JSON and keeps reading the URL from the `message` key of the JSON record. Sub-issue SI-11 of EPIC #2387 later moves them to the `service_binding` field.
- EPIC #2387 has not yet added its check of the lifecycle log convention (its SI-20). Its SI-2 planned to create `docs/logging.md`; this issue creates that page (decision D7), so SI-2 extends it. The coordinator of EPIC #2387 is told.
- Sub-issue SI-22 of EPIC #2387 (`run_id` on every log record) waits for this issue, because it adds a field in the formatter this issue writes. The JSON formatter has one place where a per-record field is added.

## Goal

By default the daemon writes its `tracing` output as JSON lines on stderr and nothing on stdout, and reports every reason it could not start (argument error, configuration error, runtime failure, panic) as a JSON record, as the contract requires of a `no-stdout-result` daemon. An operator can still choose a text style for the log records.

## Background

The global CLI output contract (`docs/adrs/20260519000000_define_global_cli_output_contract.md`), as amended by `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`, requires: result data only on stdout (one JSON object) for `stdout-result-data` commands, nothing on stdout for `no-stdout-result` commands; every stderr record is one JSON object per line (NDJSON) with a `kind`; no plain text on either channel; exit codes only `0` (did its job, the answer is in the result), `1` (could not do its job) and `2` (usage error or TTY refusal). Help text, usage errors and panic records also go to stderr as JSON. This issue is part of EPIC #2758, which migrates the tools found non-compliant by the 2026-11-06 audit. The reference implementations are `contrib/dev-tools/checks/ci-gate/src/main.rs` (`stdout-result-data`), `contrib/dev-tools/checks/shipped-features-check/src/main.rs` (`no-stdout-result`) and, for a tool with `tracing` output and a panic hook, `console/tracker-client/src/console/clients/unified/{app.rs,output.rs}`.

What the daemon does on `develop` (checked 2026-11-26):

- **Setup.** `torrust_tracker_configuration::v3_0_0::logging::setup` (`packages/configuration/src/v3_0_0/logging.rs:65`) installs `tracing_subscriber::fmt()` with `with_ansi(true)` and `with_test_writer()` (lines 88 to 98), which writes to stdout, in the style of `logging.trace_style` (default `full`; `pretty`, `compact` and `json` also exist). `bootstrap::app::initialize_global_services` calls it (`src/bootstrap/app.rs:79`) after the configuration is loaded, validated and checked for persistence requirements (`src/bootstrap/app.rs:56-64`).
- **Copies.** `packages/configuration/src/v2_0_0/logging.rs` has the same setup for the old schema; the daemon never calls it. `packages/configuration/src/logging.rs` is not compiled: `packages/configuration/src/lib.rs` has no `mod logging`.
- **Other callers of the setup.** In unit tests: `packages/udp-server/src/server/mod.rs:61` and `launcher.rs:534`, `packages/axum-http-server/src/server.rs:486`, `packages/axum-rest-api-server/src/server.rs:334`. Behind the `testing` feature: `packages/axum-rest-api-server/src/testing/environment.rs:279`. In-process tests of the application call `initialize_global_services` (seven times in `src/app.rs`, and in `src/bootstrap/jobs/{tracker_apis,udp_tracker,http_tracker}.rs`).
- **Startup failure.** `src/main.rs` reports a failure twice: `tracing::error!(%error, "Tracker startup failed")` (line 32) and then `report_startup_failure` (line 33), which is `eprintln!("Tracker startup failed: {error}")` (line 44). It then returns `ExitCode::FAILURE` (`1`) for every failure. A configuration failure happens before any subscriber exists, so only the `eprintln!` line appears (the evidence file shows this). The "Loading … configuration" records that `packages/configuration/src/lib.rs:197-224` emits are lost for the same reason.
- **Arguments.** `Cli::parse()` (`src/main.rs:27`) lets clap print plain-text argument errors on stderr and help on stdout.
- **Panics.** No panic hook: Rust prints plain text on stderr. No profile sets `panic = "abort"`.
- **Other binaries with the same entry point.** `packages/hung-component-tracker/src/main.rs` copies the daemon's `main` (its own `report_startup_failure` at line 61); the lifecycle tests run it and read its output. `src/console/profiling.rs:202` calls `app::start()` and reports a failure the same way; the contract ADR excludes the `profiling` binary.
- **Readers of the log text.**
  - `src/console/ci/e2e/logs_parser.rs` parses the text lines of the container log; `Docker::logs` (`src/console/ci/e2e/docker.rs:142-152`) returns only the container's stdout.
  - `tests/common/native_tracker/health.rs` takes the address after `Started on: http://` (`STARTED_ON_PREFIX`, line 9; `parse_started_address`, line 65) and, since #3031, after `Started on: udp://` (`UDP_STARTED_ON_PREFIX`, line 10; `parse_started_udp_address`, line 72), and checks the log target by searching the line. The harness reads stdout and stderr together (`tests/common/native_tracker/output.rs`), so only the format matters to it.
  - `tests/lifecycle/shutdown_policy.rs` asserts `timeout_seconds=25`, `=7`, `=10` and `=7` on the formatted text (lines 110, 146, 176 and 219). The other lifecycle tests search for message text, which is also present inside a JSON record.
  - `packages/hung-component-tracker/tests/common/mod.rs` reads the child's stdout and stderr for a message.
- **Test helpers.** `packages/test-helpers/src/logging.rs` has its own subscribers: `setup` and `setup_capturing_every_level` (global capture buffer) and, since #3031, `capture_logs_in_scope` (line 245; a scoped plain-text capture). They capture in-process events and never read the daemon's output, so their format does not have to change. `setup` imports `TraceStyle` from the configuration crate and only ever uses `Full`.
- **Configs and docs.** Every default config under `share/default/config/` sets `trace_style = "full"`. The decision record lists the documents that show `trace_style` or sample log lines (D7).
- **Precedent.** `tracker_client` (#2768) writes its tracing with tracing-subscriber's built-in JSON (text under `fields.message`) and a direct `{"kind":"panic",…}` record. The e2e runners still use the plain `fmt()` (issue #2769).
- **Two mistaken events.** `packages/udp-core/src/services/announce.rs:178` and `scrape.rs:140` pass `target = crate::UDP_TRACKER_LOG_TARGET` as a field instead of `target:`. Issue #3060 (open) corrects them.

Maintainer decision (2026-11-06): migrate now, not deferred. This is operator-visible just after the 3.0.0 release: deployments that parse the plain text or read stdout break.

## Decisions

The maintainer decided the migration detail on 2026-11-26. `decisions.md` in this directory has every alternative with pros, cons, examples and the table of combinations. In short:

| ID | Decision |
| --- | --- |
| D1 | `trace_style` stays, with `json` as the new default. `full`, `pretty` and `compact` remain an operator's opt-in for the log records. The default follows the contract; the operator is not forced to. Records written before the configuration is known, and the `startup_failure` and `panic` records, are JSON in every style |
| D2 | The logging is installed first thing in `main`, in JSON at level `info`. When the configuration is loaded, the level and the style of the log records are replaced. One writer for every stderr record |
| D3 | The setup code moves from the configuration crate to the tracker application (`src/bootstrap/logging.rs`). The configuration crate keeps only data (`trace_filter`, `TraceStyle`) and loses its `tracing-subscriber` dependency. No package gains a dependency. A shared crate comes later, with the follow-up that moves `tracker_client` to the same shape |
| D4 | JSON records are flat, with `kind` on every record: key order `timestamp`, `level`, `kind`, `target`, `message`, then the event fields. `kind` is `log`, `startup_failure`, `usage_error`, `help` or `panic`. Reserved keys cannot be duplicated: an event field with a name already in the record is written with the prefix `field_` |
| D5 | Exit `2` for argument and configuration errors, exit `1` for runtime startup failures; the mapping is a tested method on the error types (table in `decisions.md`) |
| D6 | A panic hook writes one `panic` record through the same formatter; a panic that unwinds out of `main` exits `1` |
| D7 | This issue creates `docs/logging.md` (where the logs go and how to get them, styles, record shape, exit codes, "Changes in 4.0.0") and reviews the other documents that show how to run the tracker or what it prints |
| D8 | No ANSI colour codes in any style. A configuration key for colours is postponed until a need is shown |
| D9 | Every style writes to stderr; stdout is always empty. An operator who wants stdout redirects (`2>&1`) |

D1, D2, D4, D6 and D8 become one ADR (task T2).

## Scope

### In Scope

- **Logging setup (D2, D3).** A public install function in `torrust_tracker_lib` that `main` calls before argument parsing; a level and a log-record style that are set after the configuration is loaded; `initialize_global_services` sets them and does nothing when no subscriber is installed. Remove `setup` and `tracing_init` from `v3_0_0/logging.rs` and `v2_0_0/logging.rs`, delete `packages/configuration/src/logging.rs`, and remove `tracing-subscriber` from `packages/configuration/Cargo.toml`. The tests of `udp-server`, `axum-http-server` and `axum-rest-api-server` that called the removed function use `torrust_tracker_test_helpers::logging::setup()`; the `testing` environment of `axum-rest-api-server` stops installing logging.
- **Formatter (D4, D8, D9).** A JSON formatter in its own module, depending only on `tracing`, `tracing-subscriber` and `serde_json`, with the reserved-key rule and one place to add a per-record field. All output on stderr, no ANSI codes in any style.
- **Default (D1).** `trace_style` defaults to `json`. The default configs under `share/default/config/` and the config that `tests/common/native_tracker/command.rs` writes do not set a text style.
- **Startup failure (D5).** One `startup_failure` record instead of the two reports of today, with the `exit_code` it returns. `exit_code()` on `app::Error` and `bootstrap::app::Error`, one unit test per variant.
- **Arguments.** Fallible parsing in `src/main.rs`: an argument error is a `usage_error` record and exit `2`; `--help` is a `help` record on stderr and exit `0`.
- **Panics (D6).** A panic hook that writes a `panic` record; a panic that unwinds out of `main` exits `1`.
- **The other entry points.** `packages/hung-component-tracker/src/main.rs` and `src/console/profiling.rs` use the same install function and failure report, and stop using `eprintln!`.
- **Readers.** `src/console/ci/e2e/logs_parser.rs` and its tests parse JSON records (`target`, `level`, `message`); `Docker::logs` in `src/console/ci/e2e/docker.rs` returns the container's stderr (or both streams) and its tests follow. `tests/common/native_tracker/health.rs` reads `target` and `message` from the JSON record for both the HTTP and the UDP reader. `tests/lifecycle/shutdown_policy.rs` reads the `timeout_seconds` key (four assertions). `packages/hung-component-tracker/tests/common/mod.rs` and the other lifecycle tests are checked and changed only if they fail.
- **Root tests that pin today's behavior.** `tests/configuration/cli_configuration/invalid_sources.rs` (lines 60, 76, 92, 109, 127) asserts exit `1` through `STARTUP_FAILURE_EXIT_CODE` (`tests/common/native_tracker/failed_start.rs:21`) and becomes exit `2` for configuration failures; its usage-error cases (lines 28, 44) keep exit `2` and assert the JSON record.
- **Test helpers.** `packages/test-helpers/src/logging.rs` keeps working and keeps its plain-text captures; `setup` stops importing `TraceStyle`.
- **Docs (D7).** `docs/logging.md`, the review of the documents listed in `decisions.md` (D7), the rustdoc of the configuration and of `logs_parser.rs`, `docs/index.md`, and the ADR.
- **Follow-ups filed as issues by this issue (T10).** Move `tracker_client` to the D4 record shape and create the shared crate.

### Out of Scope

- The e2e runners' own logging (EPIC issue 10, #2769). It can use this issue's formatter from the tracker application.
- `src/console/ci/e2e/tracker_checker.rs` (#2769) and the result shape of `tracker_client` (#3006).
- Changing log levels, thresholds, targets or message text (EPIC #2387).
- A configuration key for colours (D8) or for the stream (D9).
- The shared crate for the output contract (D3).

Files this issue may touch (a sibling issue in the same wave must not touch them):

- `src/main.rs`, `src/app.rs`, `src/bootstrap/app.rs`, a new `src/bootstrap/logging.rs` (and its formatter module), `src/console/profiling.rs`
- `packages/configuration/src/logging.rs` (deleted), `packages/configuration/src/v3_0_0/{logging.rs,mod.rs}`, `packages/configuration/src/v2_0_0/logging.rs`, `packages/configuration/Cargo.toml`, `packages/configuration/docs/migrate-v2-to-v3.md`
- `packages/hung-component-tracker/src/main.rs` and `tests/common/mod.rs`
- `packages/test-helpers/src/logging.rs`
- the test modules of `packages/udp-server/src/server/{mod.rs,launcher.rs}`, `packages/axum-http-server/src/server.rs`, `packages/axum-rest-api-server/src/{server.rs,testing/environment.rs,lib.rs}`
- `tests/configuration/cli_configuration/invalid_sources.rs`, `tests/common/native_tracker/{failed_start.rs,health.rs,command.rs}`, `tests/lifecycle/shutdown_policy.rs`
- `src/console/ci/e2e/logs_parser.rs`, `src/console/ci/e2e/docker.rs`
- the default configs under `share/default/config/` that set `[logging]` (not `share/default/config/tracker_checker.json`, which issue 2765 owns)
- `docs/logging.md` (new), `docs/index.md`, a new ADR and `docs/adrs/index.md`, `README.md`, `docs/containers.md`, `docs/benchmarking.md`, `docs/profiling.md`, `src/lib.rs` (crate docs), and the skills `setup-dev-environment`, `run-tracker-locally`, `structured-runtime-logging`, `manual-http-download-completion-e2e`, `manual-udp-download-completion-e2e`, `manual-torrent-cleanup-e2e`

## Architectural Decisions

Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`. ADR to create (task T2, `create-adr` skill): how the daemon meets the output contract: JSON by default with the text styles as an operator's exception (D1), logging installed before the configuration (D2), the record shape and `kind` values (D4), the panic record (D6), no colour codes (D8). It also updates the daemon's row in the binary table of the contract ADR.

During implementation, stop and create an ADR when a decision affects project architecture or design patterns, selects an approach among meaningful alternatives, or has consequences future contributors need to understand. Do not create ADRs for routine implementation details.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

This issue fixes a deviation from the CLI output contract, so it is bug work in substance. Follow the [fix-bug](../../../../.claude/skills/fix-bug/SKILL.md) skill:

- analysis of the defect: the tool's current behavior against the contract (see Background);
- real-artifact reproduction: run the real binary, record the commands and output of the deviation in `manual-verification-evidence.md` before changing code;
- regression-test boundary: see Regression Test Strategy;
- red regression test, then the production fix;
- green test evidence and a like-for-like recheck of the same real-artifact commands.

## Regression Test Strategy

Unit tests of the formatter (record shape, key order, `kind`, reserved keys, no ANSI), of the logging setup (writer is stderr; level and style change after the configuration), of the `exit_code()` mapping (one per variant), of the argument and help records, and of the panic record, plus the updated `logs_parser.rs` tests and the root tests listed in Scope, updated to the new output. The new unit tests fail today (stdout, plain text, exit `1` for configuration errors); the existing root tests pass today and pin the old output, so they are changed rather than added. If a higher boundary is used instead, record why it is clearer or the only practical option; link red and green output and the final recheck to `manual-verification-evidence.md`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

The issue is delivered as several pull requests of this one issue: the tasks are steps of one process and each needs the one before (see "Delivery" in `decisions.md`). Every pull request leaves `develop` working: the readers change in the same pull request as the output they read.

| ID  | Status | Task | PR | Notes / Expected Output |
| --- | ------ | ---- | -- | ----------------------- |
| T0 | DONE | Refresh this spec against `develop` | spec | Dependencies, Background, Scope and file references corrected (2026-11-26). |
| T1 | DONE | Agree the migration detail with the maintainer | spec | `decisions.md` (D1 to D9), summary in section Decisions. |
| T2 | TODO | Write the ADR | 1 | ADR created with `create-adr`; row of the daemon in the contract ADR updated. |
| T3 | TODO | Record the real-artifact reproduction of a normal run and a bind failure | 1 | `manual-verification-evidence.md` R0 completed (it has only the configuration and argument cases). |
| T4 | TODO | Move the logging setup to the application and install it first (D2, D3) | 1 | Tests first. Same text output and stream as today; "Loading … configuration" records now visible; configuration crate without `tracing-subscriber`; the other entry points and the package tests follow. |
| T5 | TODO | JSON formatter, stderr, no colour codes, `json` default (D1, D4, D8, D9), with the readers | 2 | Tests first. `logs_parser.rs`, `Docker::logs`, `health.rs`, `shutdown_policy.rs` and the default configs change in the same pull request. |
| T6 | TODO | Startup-failure record and exit codes (D5) | 3 | Tests first. One record, `exit_code()` mapping, `invalid_sources.rs` to exit `2`. |
| T7 | TODO | Argument errors and help as JSON; panic hook (D6) | 3 | Tests first. |
| T8 | TODO | `docs/logging.md` and the review of the other documents (D7) | 4 | The page, the corrected examples, the "Changes in 4.0.0" section. |
| T9 | TODO | Run the tracker locally and in a container | 4 | M1 to M5 recorded in `manual-verification-evidence.md`. |
| T10 | TODO | File the follow-ups | 4 | Issue for `tracker_client` and the shared crate; final report to the coordinator of EPIC #2758. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T0, T1 | Refresh this specification and record the decisions | Documentation commits in the spec pull request. |
| T2 | The ADR | One documentation commit. |
| T3 | Reproduction evidence | One documentation commit, before the code of T4. |
| T4 | Logging setup moved and installed first | Red tests, then the change; commit after focused validation and required review. |
| T5 | Formatter, stream, default and readers | Red tests, then the change; commit after focused validation and required review. |
| T6 | Startup-failure record and exit codes | Red tests, then the change; commit after focused validation and required review. |
| T7 | Argument errors, help and panic hook | Red tests, then the change; commit after focused validation and required review. |
| T8 | Documentation | Commit after focused validation and required review. |
| T9 | Manual verification evidence | Commit after focused validation and required review. |
| T10 | Follow-up issues and spec update | One documentation commit. |

Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG. For test-producing work, use the `write-unit-test` skill and complete the design review it requires before each commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2758-daemon-json-logging/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec refreshed against `develop` and migration detail decided with the maintainer (2026-11-26)
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
- 2026-11-23 16:13 UTC - Claude Code (Fable 5.2) - Recorded the maintainer's decision of 2026-11-21 that this issue starts after EPIC #2387, with the consequences for its scope (section Dependencies, task T0).
- 2026-11-23 16:38 UTC - Claude Code (Claude Opus 5.5) - Updated the EPIC total to 13 after #3004 and #3006 were added (PR 3007 review).
- 2026-11-24 13:23 UTC - Claude Code (Claude Opus 5.5) - Recorded the maintainer's decision of 2026-11-24: this issue waits only for SI-1 of EPIC #2387, and moves today's log readers to JSON (section Dependencies, task T0).
- 2026-11-26 17:09 UTC - Claude Code (Claude Opus 5.5, then Fable 5.2) - T0 and T1: refreshed the spec against `develop` (#3004 closed, #2387 SI-1 merged, references corrected, new readers and callers added) and recorded the maintainer's decisions D1 to D9 in `decisions.md`; the plan is now four pull requests.

## Acceptance Criteria

- [ ] AC1: Running the tracker writes nothing to stdout, in every `trace_style`.
- [ ] AC2: With the default configuration (`json`), every stderr line is a JSON object with the keys `timestamp`, `level`, `kind`, `target` and `message`, and no line has ANSI codes.
- [ ] AC3: A configuration error exits `2` and a runtime startup failure (for example a bind error) exits `1`; in both, exactly one record on stderr has the kind `startup_failure`, in every `trace_style`.
- [ ] AC4: `logs_parser.rs`, `Docker::logs`, the native test harness and their tests follow the new format and stream.
- [ ] AC5: The operator-visible change is documented: `docs/logging.md` says where the logs go and how to get them, shows every style, the record shape, the exit codes and the "Changes in 4.0.0" section, and no reviewed document shows an example that no longer works.
- [ ] AC6: Argument errors (`--unknown`, `-c ""`) and `--help` follow the contract (JSON on stderr, stdout empty, usage errors exit `2`), proven by M2c.
- [ ] AC7: A text style (`full`, `pretty`, `compact`) still works as an opt-in: the log records are text on stderr without ANSI codes, and the records of D1 that are always JSON are JSON.
- [ ] AC8: A panic writes one `panic` record on stderr and no plain text, and a panic that unwinds out of `main` exits `1`.
- [ ] AC9: `torrust-tracker-configuration` has no function that installs a subscriber and no `tracing-subscriber` dependency, and no package gained a dependency.
- [ ] AC10: The ADR exists and the follow-up issues are filed.
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
| M4 | Text styles | Run with `trace_style` set to `full`, `pretty` and `compact`; repeat M2 and M2b with `full` | `out` empty; log records are text without ANSI codes; the first records and the `startup_failure` record are JSON | TODO | `manual-verification-evidence.md` section V4 |
| M5 | Panic | Run a build with a forced panic (test-only trigger) or the unit test of the hook against the real binary | One `panic` record on stderr, no plain text | TODO | `manual-verification-evidence.md` section V5 |

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
| AC7 | TODO |  |
| AC8 | TODO |  |
| AC9 | TODO |  |
| AC10 | TODO |  |

## Risks and Trade-offs

- Operators parse the old text or read stdout. Mitigation: `docs/logging.md` with the "Changes in 4.0.0" section; `trace_style = "full"` and `2>&1` give back the old lines (without colour codes).
- The e2e suite depends on log text. Mitigation: the readers change in the same pull request as the output (T5).
- A stream with a text style mixes JSON and text records (D1). Accepted by the maintainer; documented in `docs/logging.md`.
- `tracker_client` keeps another JSON shape until its follow-up. Mitigation: the follow-up issue is filed by T10 and the ADR names the shape.
- A third-party project may call the removed `logging::setup` of the configuration crate. Mitigation: the "Changes in 4.0.0" section and the crate's migration document say what to do.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise record in the progress log why none was needed.

## References

- Parent EPIC: #2758
- Related issues: #2715, #1798, #2387 (EPIC whose SI-1 goes before this issue and whose SI-2 to SI-23 come after it), #3031 (SI-1 of EPIC #2387), #3004, #2768 and #2769
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`
