---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2758
github-issue: 2769
spec-path: docs/issues/open/2769-2758-e2e-runners-json/ISSUE.md
branch: "2769-e2e-runners-json"
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

# Issue #2769 - `e2e_tests_runner` and `qbittorrent_e2e_runner`: JSON diagnostics

**Parent EPIC:** #2758 - Migrate the CLI tools to the global output contract. Order 10 of 11, wave 2.

## Goal

The two e2e runners are compliant `no-stdout-result` programs: empty stdout, NDJSON diagnostics on stderr, exit `0`/`1`/`2`.

## Background

Depends on the deprecated-client removal (EPIC issue 6, #2765), which edits `src/console/ci/e2e/tracker_checker.rs`, and on the daemon logging change (EPIC issue 7), which edits `src/console/ci/e2e/docker.rs` and `src/console/ci/e2e/logs_parser.rs`. `src/console/ci/e2e/runner.rs` initializes `tracing_subscriber::fmt()` (stdout, plain text) and both runners return `anyhow::Result` from `main`, so a failure prints `Error: <debug text>` and exits `1`. The runners are in `packages/e2e-tools/src/bin/` and `src/console/ci/{e2e,qbittorrent_e2e}/`.

The global CLI output contract (`docs/adrs/20260519000000_define_global_cli_output_contract.md`), as amended by `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`, requires: result data only on stdout (one JSON object) for `stdout-result-data` commands, nothing on stdout for `no-stdout-result` commands; every stderr record is one JSON object per line (NDJSON) with a `kind`; no plain text on either channel; exit codes only `0` (did its job, the answer is in the result), `1` (could not do its job) and `2` (usage error or TTY refusal); and TTY refusal for `stdout-result-data` commands. A distinct outcome is a `kind` field: an answer to what the caller asked is a stdout result with exit `0`; "could not answer" is a stderr diagnostic with exit `1`. This issue is part of EPIC #2758, which migrates the tools found non-compliant by the 2026-11-06 audit. The reference implementations are `contrib/dev-tools/checks/ci-gate/src/main.rs` (`stdout-result-data`) and `contrib/dev-tools/checks/shipped-features-check/src/main.rs` (`no-stdout-result`).

## Scope

### In Scope

- `tracing` to stderr as JSON; nothing on stdout.
- Failures as one JSON record with a `kind`; usage errors exit `2`.
- `src/console/ci/e2e/runner.rs` logs the tracker configuration verbatim (`tracing::info!("tracker config:\n{tracker_config}")`), and `src/console/ci/e2e/tracker_checker.rs:13` logs the checker configuration. Only the tracker configuration can carry a secret: the checker's argument is the JSON of `RunningServices` (`runner.rs:117-118`), three lists of addresses that `logs_parser.rs` matches as scheme, IP and port, so it is masked only as a precaution (same helper, no separate test needed). ADR section 7 forbids secrets in output: mask secret values of the tracker configuration before logging and add a test. No tracing macro under `src/console/ci/qbittorrent_e2e/` logs a configuration, so nothing to mask there.
- Child processes inherit the runner's streams: `src/console/ci/e2e/tracker_checker.rs` runs `cargo run ...` with `.status()`, and `src/console/ci/e2e/docker.rs` (lines 41, 110, 128) and `src/console/ci/compose.rs` (line 104) run `docker` the same way, so the children's stdout and the plain text of `cargo` and `docker` reach the runner's channels, and `AC1`/`AC2` cannot hold. Capture each child's stdout and stderr and report them as JSON records (or discard them), for every runner path, normal and cleanup. Once `tracker_client` refuses a TTY (issue 2768), a checker child that inherits the terminal exits `2` and the runner panics (`runner.rs` calls `.expect` on it, exit status `101`), so capturing the checker's stdout is required; the interactive run itself is verified in M4 of issue 2768.
- Update tests and docs that show the runner output.

### Out of Scope

- Changing what the runners test.
- `profiling` (developer-only).

Files this issue may touch (a sibling issue in the same wave must not touch them):

- `src/console/ci/**`
- `packages/e2e-tools/src/bin/{e2e_tests_runner,qbittorrent_e2e_runner}.rs`

## Architectural Decisions

Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`. ADRs to create: None known.

During implementation, stop and create an ADR when a decision affects project architecture or design patterns, selects an approach among meaningful alternatives, or has consequences future contributors need to understand. Do not create ADRs for routine implementation details.

## Design and Ownership Review

This issue changes child-process I/O in the e2e runners (`cargo`, `docker`, `docker compose` and the checker), in the normal and the cleanup paths, so this section applies.

- Responsibilities: one small helper owns running a child command with captured stdout and stderr and reporting them as JSON records on the runner's stderr; the call sites (`docker.rs`, `compose.rs`, `tracker_checker.rs`) only ask it to run a command and use its result. The helper owns the child, its pipes and their reaping; the runner owns the tracker container and compose stack lifetimes (the existing RAII guards).
- Lifetime invariants: every spawned child is waited for or killed and reaped before the helper returns, on success, on failure and on a panic in the caller; the pipes are drained so a full pipe cannot block the child; cleanup commands (`docker compose down --volumes`, container removal) use the same helper on the drop path and must not panic.
- Deadlines: every child wait has an absolute deadline (a hung `docker` or checker must not hang the runner); every readiness wait the runners already do keeps its own absolute deadline.
- Design-review checkpoint: after the first passing vertical slice (one captured child in the normal path), review the helper's ownership and the drop-path behavior before converting the remaining call sites.

## Bug-Fix Process

This issue fixes a deviation from the CLI output contract, so it is bug work in substance. Follow the [fix-bug](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md) skill:

- analysis of the defect: the tool's current behavior against the contract (see Background);
- real-artifact reproduction: run the real binary, record the commands and output of the deviation in `manual-verification-evidence.md` before changing code;
- regression-test boundary: see Regression Test Strategy;
- red regression test, then the production fix;
- green test evidence and a like-for-like recheck of the same real-artifact commands.

## Regression Test Strategy

Unit tests of the runners' error reporting and of the config masking, and a run of each runner asserting empty stdout and NDJSON on stderr. If a higher boundary is used instead, record why it is clearer or the only practical option; link red and green output and the final recheck to `manual-verification-evidence.md`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | TODO | Tracing and error output to JSON on stderr | Tests green. |
| T2 | TODO | Usage errors exit `2` | Test. |
| T3 | TODO | Capture the output of the child processes (`cargo`, `docker`, `docker compose`, the checker) and report it as JSON | AC5; the helper from the Design and Ownership Review. |
| T4 | TODO | Run both runners locally | Evidence recorded. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Tracing and error output to JSON on stderr | Commit after focused validation and required review. |
| T2 | Usage errors exit `2` | Commit after focused validation and required review. |
| T3 | Capture the output of the child processes and report it as JSON | Commit after focused validation and required review. |
| T4 | Run both runners locally | Commit after focused validation and required review. |

Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG. For test-producing work, use the `write-unit-test` skill and complete the design review it requires before each commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2758-e2e-runners-json/ISSUE.md`
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
- 2026-11-06 16:52 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the spec; GitHub issue #2769 created.
- 2026-11-09 10:25 UTC - Claude Code (Claude Sonnet 5.5) - Corrected details after the review rounds on PR 2771 and PR 2774 and fixed the earlier log stamps from the commit and issue creation times.
- 2026-11-09 12:12 UTC - Claude Code (Claude Sonnet 5.5) - Addressed the review of PR 2771 (da2ce7 findings F1 to F27 and Copilot's later comments): specs corrected from the code, reproductions recorded where a spec has a bug-fix section.
- 2026-11-09 12:39 UTC - Claude Code (Claude Sonnet 5.5) - Addressed Copilot's latest review of PR 2771: filled the Design and Ownership Review for the child-process work and corrected the pseudo-terminal check in the evidence.
- 2026-11-09 13:21 UTC - Claude Code (Claude Sonnet 5.5) - Addressed da2ce7's review 5826238636 of PR 2771 on this spec.
- 2026-11-09 16:03 UTC - Claude Code (Claude Sonnet 5.5) - Addressed da2ce7's review 5826857760 of PR 2771 on this spec.

## Acceptance Criteria

- [ ] AC1: Both runners write nothing to stdout.
- [ ] AC2: Every stderr line is JSON.
- [ ] AC3: A failure exits `1` with a `kind`; a usage error exits `2`.
- [ ] AC4: The tracker and checker configurations are logged with secret values masked.
- [ ] AC5: No child process (`cargo`, `docker`, `docker compose`, the checker) writes to the runner's stdout or stderr; its output is captured and reported as JSON records, in the normal and cleanup paths.
- [ ] AC6: The checker child's stdout is captured, so a run from an interactive terminal does not meet the TTY refusal that issue 2768 adds; the interactive case itself is verified in M4 of issue 2768 and is not a pass condition of this issue alone.
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
| M1 | e2e run | `cargo build -p torrust-tracker-e2e-tools --bins`, then `target/debug/e2e_tests_runner ... >out 2>err` (not `cargo run`, which adds plain cargo messages on stderr; `torrust-tracker-e2e-tools` is not a default workspace member) | `out` empty, `err` NDJSON, exit as expected | TODO | `manual-verification-evidence.md` section V1 |
| M2 | qBittorrent run | Same with `target/debug/qbittorrent_e2e_runner` | Same | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Captured child output | Run `target/debug/e2e_tests_runner ... >out 2>err` and check that `out` is empty and every line of `err` parses with `jq -c .`, including the lines that came from `cargo`, `docker` and the checker | No plain `cargo` or `docker` text on either channel | TODO | `manual-verification-evidence.md` section V3 |

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

- CI workflows grep runner output. Mitigation: search workflows first.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise record in the progress log why none was needed.

## References

- Parent EPIC: #2758
- Related issues: #2715, #1798
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`
