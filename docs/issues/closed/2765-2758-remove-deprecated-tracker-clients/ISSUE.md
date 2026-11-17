---
schema-version: 1
doc-type: issue
issue-type: task
status: done
priority: p2
epic: 2758
github-issue: 2765
spec-path: docs/issues/closed/2765-2758-remove-deprecated-tracker-clients/ISSUE.md
branch: "2765-remove-deprecated-client-bins"
related-pr: 2901
last-updated-utc: "2026-11-17 17:58"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .claude/skills/create-issue/SKILL.md
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md
    - docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #2765 - Remove the deprecated tracker client binaries

**Parent EPIC:** #2758 - Migrate the CLI tools to the global output contract. Order 6 of 11, wave 1.

## Goal

Remove the `http_tracker_client`, `udp_tracker_client` and `tracker_checker` binaries, which the ADR says to remove rather than migrate; keep the library code the unified `tracker_client` uses.

## Background

The ADR (section 10) says deprecated binaries `http_tracker_client`, `udp_tracker_client` and `tracker_checker` should be removed rather than migrated. They still exist under `console/tracker-client/src/bin/` and print a plain-text deprecation warning; the README says to use `tracker_client`. The unified `tracker_client check` reuses the checker library module, which stays. Find every reference: `console/tracker-client/Cargo.toml`, README, `console/tracker-client/tests/tracker_checker.rs` and `tests/tracker_checker/`, `docs/packages.md`, `share/default/config/tracker_checker.json`, the Containerfile and workflows. This is a breaking change for anyone still using the old binaries: record it in the release notes or changelog convention of the repo.

The global CLI output contract (`docs/adrs/20260519000000_define_global_cli_output_contract.md`), as amended by `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`, requires: result data only on stdout (one JSON object) for `stdout-result-data` commands, nothing on stdout for `no-stdout-result` commands; every stderr record is one JSON object per line (NDJSON) with a `kind`; no plain text on either channel; exit codes only `0` (did its job, the answer is in the result), `1` (could not do its job) and `2` (usage error or TTY refusal); and TTY refusal for `stdout-result-data` commands. A distinct outcome is a `kind` field: an answer to what the caller asked is a stdout result with exit `0`; "could not answer" is a stderr diagnostic with exit `1`. This issue is part of EPIC #2758, which migrates the tools found non-compliant by the 2026-11-06 audit. The reference implementations are `contrib/dev-tools/checks/ci-gate/src/main.rs` (`stdout-result-data`) and `contrib/dev-tools/checks/shipped-features-check/src/main.rs` (`no-stdout-result`).

## Scope

### In Scope

- Delete the three binary entry points (`console/tracker-client/Cargo.toml` has no `[[bin]]` section and names none of them; they are discovered from `src/bin/`).
- Delete or migrate their tests so coverage of the shared code is kept through `tracker_client`.
- Update README, `docs/packages.md` and any doc or config that tells the reader to run them.
- Add the breaking-change note.
- Migrate `src/console/ci/e2e/tracker_checker.rs`, which launches `cargo run -p torrust-tracker-client --bin tracker_checker` for the e2e runner, to the unified `tracker_client check` (keeping `TORRUST_CHECKER_CONFIG`), so the e2e runner still works. That helper uses `Command::status()`, so the child inherits the runner's stdout; once issue 2768 adds TTY refusal to `tracker_client`, the checker child of an e2e run from a terminal would exit `2`, `tracker_checker::run` would turn that into an error and `runner.rs` would panic with exit status `101`. Capture the child's stdout (and forward or log it as JSON) when migrating the caller; check the e2e runner and any workflow that calls it. The interactive-terminal case needs the TTY refusal, which arrives with issue 2768, and is verified there (M4).

### Out of Scope

- Changing `tracker_client` behavior (EPIC issue 9).
- Editing the ADR.

Files this issue may touch (a sibling issue in the same wave must not touch them):

- `console/tracker-client/src/bin/{http_tracker_client,udp_tracker_client,tracker_checker}.rs`
- `console/tracker-client/README.md`
- `console/tracker-client/tests/tracker_checker*`
- `docs/packages.md`
- `share/default/config/tracker_checker.json` (if only used by the old binary)
- `src/console/ci/e2e/tracker_checker.rs`

## Architectural Decisions

Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`. ADRs to create: None known.

During implementation, stop and create an ADR when a decision affects project architecture or design patterns, selects an approach among meaningful alternatives, or has consequences future contributors need to understand. Do not create ADRs for routine implementation details.

## Design and Ownership Review

The checker caller migration in `src/console/ci/e2e/tracker_checker.rs` spawns a child process and captures its output, so this section applies to that helper.

- Responsibilities: `tracker_checker::run` owns launching `tracker_client check` (with `TORRUST_CHECKER_CONFIG` in the child's environment), the pipes for the child's stdout and stderr, and turning the exit status and captured output into a result for the runner; the runner owns what it does with that result.
- Lifetime invariants: the child is always waited for (or killed and reaped) before `run` returns, on success, on a non-zero exit, on a read error and on a panic in the caller; both pipes are read to the end (or on separate threads) so a full pipe cannot block the child.
- Deadline: the checker has its own network timeout, and `run` adds an absolute deadline to the wait on the child so a hung checker cannot hang the runner; a missing deadline is a defect.
- Design-review checkpoint: after the first passing vertical slice (the helper captures a successful `check` run), review the ownership of the child and the pipes before handling the failure paths.
- Design-review checkpoint record (taken from the code after the review of PR 2901, which asked for it; it was not done as a separate step at the first slice): the child is owned by a `ChildGuard` that kills (if still running) and reaps it on drop, so every path reaps it; each pipe is owned by one reader thread that sends chunks over a channel, and the collector gives up after a grace period, so neither a full pipe nor a pipe kept open by a descendant blocks the child or the deadline; the deadline is an absolute bound on the wait, and the build runs as its own step under its own deadline so a cold build does not count against the check. Assumption: no descendant of the child outlives it (on Unix `cargo run` replaces itself with the binary). Not covered: terminating a process tree.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | DONE | List every reference to the three binaries | `grep` result in the progress log. |
| T2 | DONE | Remove the binaries | Workspace builds. |
| T3 | DONE | Keep the test coverage through `tracker_client` | No lost scenarios. |
| T4 | DONE | Update docs and add the breaking-change note | `grep` finds no stale instruction. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | List every reference to the three binaries | Commit after focused validation and required review. |
| T2 | Remove the binaries | Commit after focused validation and required review. |
| T3 | Keep the test coverage through `tracker_client` | Commit after focused validation and required review. |
| T4 | Update docs and add the breaking-change note | Commit after focused validation and required review. |

Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG. For test-producing work, use the `write-unit-test` skill and complete the design review it requires before each commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2758-remove-deprecated-tracker-clients/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-06 16:43 UTC - Claude Code (Claude Sonnet 5.5) - Drafted this spec as part of EPIC #2758 from the 2026-11-06 audit.
- 2026-11-06 16:52 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the spec; GitHub issue #2765 created.
- 2026-11-09 10:25 UTC - Claude Code (Claude Sonnet 5.5) - Corrected details after the review rounds on PR 2771 and PR 2774 and fixed the earlier log stamps from the commit and issue creation times.
- 2026-11-09 11:26 UTC - Claude Code (Claude Sonnet 5.5) - Corrected details after the round 3 review of PR 2774.
- 2026-11-09 12:12 UTC - Claude Code (Claude Sonnet 5.5) - Addressed the review of PR 2771 (da2ce7 findings F1 to F27 and Copilot's later comments): specs corrected from the code, reproductions recorded where a spec has a bug-fix section.
- 2026-11-09 12:39 UTC - Claude Code (Claude Sonnet 5.5) - Addressed Copilot's latest review of PR 2771: filled the Design and Ownership Review for the child-process work and corrected the pseudo-terminal check in the evidence.
- 2026-11-09 13:21 UTC - Claude Code (Claude Sonnet 5.5) - Addressed da2ce7's review 5826238636 of PR 2771 on this spec.
- 2026-11-17 09:08 UTC - Claude Code (Claude Sonnet 5.5) - Implemented T1 to T4 on branch `2765-remove-deprecated-client-bins`. T1 references found: the three `src/bin` files, the doc comments in `clients/{http,udp,checker}/app.rs`, `console/tracker-client/README.md`, `docs/packages.md`, the contract doc, the sccache experiment Containerfile and `src/console/ci/e2e/tracker_checker.rs`; no workflow, `Containerfile` or `Cargo.toml` named them. T2: the three files deleted (the library modules stay: `tracker_client` reuses their checks). T3: `tests/tracker_checker*` already run `tracker_client check`, so nothing moved. T4: README breaking-change note (the repo has no changelog). The e2e helper now runs `tracker_client check` with captured, drained pipes and a 15 minute deadline. Evidence: V1 and V2.
- 2026-11-17 10:47 UTC - Claude Code (Claude Sonnet 5.5) - Addressed the review of PR 2901 (da2ce7 F1 to F9, Copilot's three comments): the helper builds first under its own deadline, bounds the drain, returns the captured output and has Arrange-Act-Assert tests for the capture, the kill and a descendant that holds the pipes; the migrated examples use `--format text` and `--ip`; the open spec of #2594 names `tracker_client check`; V2 was re-run on the new helper with its harness recorded, V3 records the AC2 search; AC4 stays open until the container job passes; the frontmatter names the real branch and PR; the completion review is recorded below.
- 2026-11-17 17:36 UTC - Claude Code (Claude Sonnet 5.5) - PR 2901 was merged into `develop` on 2026-11-17 (merged at 17:33:46 UTC) and issue #2765 closed (17:33:47 UTC); spec archived to `docs/issues/closed/` and the EPIC row set to DONE in the close-out PR. Review: eight fix rounds (da2ce7 F1 to F9, F13, F14, F16 to F24 and F28; Copilot F10 to F12, F15 and F25 to F27; audit in `docs/pr-reviews/pr-2901-review/PR-REVIEW.md`); F28 (no test for the choice between the full log and the excerpt) was left on the maintainer's decision. "Reviewer validated acceptance criteria" stays unchecked: the reviewers approved the PR but did not validate the criteria checkboxes.

## Acceptance Criteria

- [x] AC1: `cargo build --workspace --all-targets` passes without the three binaries.
- [x] AC2: No document or script tells the reader to run a removed binary.
- [x] AC3: `tracker_client check` still passes the tests that covered the checker.
- [x] AC4: `src/console/ci/e2e/tracker_checker.rs` launches `tracker_client check` with `TORRUST_CHECKER_CONFIG`, proven by a runner-level run (M2), because the binary name is a runtime string that the workspace build cannot check. Proven by the container test job of PR 2901, which runs `e2e_tests_runner`: it passed on the head that carries the commit `docs(pr-reviews): [#2765] audit review round 7 of PR 2901` (job https://github.com/torrust/torrust-tracker/actions/runs/44115063923/job/134153598863), and its log shows the helper running `cargo build -p torrust-tracker-client --bin tracker_client` and then `TORRUST_CHECKER_CONFIG=[config] cargo run -p torrust-tracker-client --bin tracker_client -- check`, which ran `target/debug/tracker_client check`. The job of the head before it (the commit `docs(pr-reviews): [#2765] audit review round 6 of PR 2901`) failed in the image test stage, before the e2e step, on a racy unit test that the round-7 commit `test(e2e): [#2765] drop the racy byte-delta assertion of the receiver-gone test` fixed. Two later commits changed only the reader and its logging: `fix(e2e): [#2765] retry interrupted reads and bound the output logged on success` and the audit commit `docs(pr-reviews): [#2765] audit review round 8 of PR 2901` (the head merged); the Docker job of that final head was still in progress when the PR was merged. `BUILD_ARGS`, `CHECKER_ARGS` and the `TORRUST_CHECKER_CONFIG` variable did not change after the passing head.
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
| M1 | Unified checker | Run `tracker_client check` against a local tracker as the README documents | Works as before | DONE | `manual-verification-evidence.md` section V1 |
| M2 | E2E checker launch | Run the e2e runner (or call the helper) after migrating `tracker_checker.rs` and check that it launches `tracker_client check` with `TORRUST_CHECKER_CONFIG` (for example from the logged command line); the interactive-terminal case is verified in issue 2768 once the TTY refusal exists, and this row is only a smoke test | The checker step runs against the unified binary | DONE | `manual-verification-evidence.md` section V2 |

Manual verification is mandatory even when automated tests pass. Record actual commands, output and the toolchain that produced each result in `manual-verification-evidence.md` (from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`).

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | `cargo build --workspace --all-targets` finished without the three binaries (progress log). |
| AC2 | DONE | Evidence V3: the search for an instruction to run a removed binary prints nothing (exit 1); the open spec of #2594 that named `tracker_checker` was updated. |
| AC3 | DONE | `cargo test -p torrust-tracker-client`: tests in `tests/tracker_checker/` already launch `tracker_client check` and all pass (no scenario lost; the removed binaries had no tests of their own). |
| AC4 | DONE | Helper builds and launches `cargo run ... --bin tracker_client -- check` with `TORRUST_CHECKER_CONFIG`: unit tests in the helper, smoke M2 (evidence V2), and the runner-level run in the container test job of PR 2901 (see the AC4 line above for the job and what its log shows). |

## Risks and Trade-offs

- External users still call the old names. Mitigation: they have printed a deprecation warning; add the breaking-change note.

## Implementation Completion Review

- Retrospective: `implementation-retrospective.md` in this directory.
- Prose-first Arrange-Act-Assert comparison (`write-unit-test`), recorded for every unit test of `src/console/ci/e2e/tracker_checker.rs` at the head of the last review round (each test carries the Arrange, Act and Assert sections in code, and the comparison found the code to state the prose):

| Test | Arrange (prose) | Act (prose) | Assert (prose) | Result of the comparison |
| ---- | --------------- | ----------- | -------------- | ------------------------ |
| `it_should_launch_the_unified_tracker_client_check_command` | The expected command line of the checker. | Join the constant argument list. | It equals `run -p torrust-tracker-client --bin tracker_client -- check`. | Code matches; the expected string is independent of production code. |
| `it_should_build_the_same_binary_before_running_it` | The expected build command line. | Join the build arguments. | It equals `build -p torrust-tracker-client --bin tracker_client`. | Matches. |
| `it_should_pass_the_configuration_in_the_checker_config_env_var` | A configuration and a script that prints the variable. | Run the child with the configuration. | The captured stdout is the configuration. | Matches; replaced a weaker `is_ok` check. |
| `it_should_capture_stdout_and_stderr_separately` | A script that writes one word to each stream. | Run it. | Each captured stream holds its own word. | Matches; this is the test that fails if the pipes are inherited (the captured text would be empty). |
| `it_should_report_a_non_zero_exit_status_as_an_error` | A child that exits with `1`. | Run the command. | The error kind is `Other` (a failed command, not a timeout or a missing program). | Matches; replaced a bare `is_err`. |
| `it_should_capture_everything_when_the_child_writes_more_than_a_pipe_buffer_to_both_streams` | A child that writes 300000 bytes to each stream, more than a pipe buffer. | Run it. | Both streams are captured whole. | Matches; replaced an `is_ok` check, and also fails if the capture is lost. |
| `it_should_kill_the_child_and_return_promptly_when_the_deadline_expires` | A child that sleeps 60 s and a 300 ms deadline. | Run it. | A `TimedOut` error in under 20 s: without the kill the wait takes the 60 s. | Matches; the elapsed bound pins the kill. |
| `it_should_not_wait_for_a_descendant_that_keeps_the_pipes_open` | A hung child whose descendant holds both pipes for 8 s, a 300 ms deadline and a 500 ms grace. | Run it. | `TimedOut` in under 5 s. | Matches. |
| `it_should_keep_the_output_read_before_a_descendant_held_the_pipe_open` | A child that prints a line and exits while a descendant keeps stdout open. | Run it with a 500 ms grace. | The line is captured and the call returns in under 5 s. | Matches. |
| `it_should_return_the_output_written_before_the_deadline_expired` | A child that writes a line to each stream and then hangs, with a 500 ms deadline. | Run it. | The error is `TimedOut` and both captured streams hold their line. | Matches. |
| `it_should_keep_the_chunks_already_queued_when_the_drain_window_ended` | A chunk read and queued, and a drain window that ended a second ago. | Collect. | The chunk is returned (what was read before the end is kept). | Matches. |
| `it_should_cut_the_output_at_the_byte_cap_even_when_more_is_queued` | More than the byte cap queued and an ended window. | Collect. | The result has exactly the cap's length. | Matches. |
| `it_should_queue_at_most_the_byte_cap_from_a_pipe_that_writes_more` | A pipe that holds more than the byte cap. | Read it with the reader thread. | The channel closes (the reader read everything) and the queued bytes equal the cap. | Matches. |
| `it_should_stop_reading_a_pipe_past_the_cap_when_the_receiver_is_gone` | A pipe that never ends, read by the reader thread until it is well past the byte cap. | Drop the receiver. | The reader ends: the count of bytes it reads stops growing. | Matches; with the probe removed the count keeps growing and the test fails. |
| `it_should_retry_a_read_interrupted_by_a_signal_instead_of_ending_the_stream` | A pipe whose first read is interrupted and then returns data. | Collect through the reader thread. | The data is collected (the interrupted read is retried). | Matches; without the retry the collected text is empty. |
| `it_should_cut_the_log_excerpt_on_a_character_boundary` | A string with a three-byte character that straddles the limit. | Take the excerpt. | The excerpt stops before the character. | Matches. |
| `it_should_keep_the_whole_content_when_it_is_within_the_log_limit` | A string shorter than the limit. | Take the excerpt. | It is returned whole. | Matches. |
| `it_should_keep_the_output_of_the_second_pipe_when_the_first_one_uses_the_whole_window` | A child that writes to both pipes and exits while a descendant holds them open, with a 500 ms window. | Run it. | Both streams hold their text. | Matches. |
| `it_should_share_one_drain_window_between_both_pipes` | A child that exits while a descendant holds both pipes for 8 s, with a 3 s window. | Run it. | The call returns in under 5 s (one window is about 3.1 s, two would be about 6.1 s). | Matches. |
| `it_should_fail_when_the_program_cannot_be_launched` | A program that does not exist. | Run it. | The error kind is `NotFound`. | Matches; replaced a bare `is_err`. |

## References

- Parent EPIC: #2758
- Related issues: #2715, #1798
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`
