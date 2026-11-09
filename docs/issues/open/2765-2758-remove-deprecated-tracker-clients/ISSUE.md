---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2758
github-issue: 2765
spec-path: docs/issues/open/2765-2758-remove-deprecated-tracker-clients/ISSUE.md
branch: "2765-remove-deprecated-tracker-clients"
related-pr: null
last-updated-utc: "2026-11-09 13:21"
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

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | TODO | List every reference to the three binaries | `grep` result in the progress log. |
| T2 | TODO | Remove the binaries | Workspace builds. |
| T3 | TODO | Keep the test coverage through `tracker_client` | No lost scenarios. |
| T4 | TODO | Update docs and add the breaking-change note | `grep` finds no stale instruction. |

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
- 2026-11-06 16:52 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the spec; GitHub issue #2765 created.
- 2026-11-09 10:25 UTC - Claude Code (Claude Sonnet 5.5) - Corrected details after the review rounds on PR 2771 and PR 2774 and fixed the earlier log stamps from the commit and issue creation times.
- 2026-11-09 11:26 UTC - Claude Code (Claude Sonnet 5.5) - Corrected details after the round 3 review of PR 2774.
- 2026-11-09 12:12 UTC - Claude Code (Claude Sonnet 5.5) - Addressed the review of PR 2771 (da2ce7 findings F1 to F27 and Copilot's later comments): specs corrected from the code, reproductions recorded where a spec has a bug-fix section.
- 2026-11-09 12:39 UTC - Claude Code (Claude Sonnet 5.5) - Addressed Copilot's latest review of PR 2771: filled the Design and Ownership Review for the child-process work and corrected the pseudo-terminal check in the evidence.
- 2026-11-09 13:21 UTC - Claude Code (Claude Sonnet 5.5) - Addressed da2ce7's review 5826238636 of PR 2771 on this spec.

## Acceptance Criteria

- [ ] AC1: `cargo build --workspace --all-targets` passes without the three binaries.
- [ ] AC2: No document or script tells the reader to run a removed binary.
- [ ] AC3: `tracker_client check` still passes the tests that covered the checker.
- [ ] AC4: `src/console/ci/e2e/tracker_checker.rs` launches `tracker_client check` with `TORRUST_CHECKER_CONFIG`, proven by a runner-level run (M2), because the binary name is a runtime string that the workspace build cannot check.
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
| M1 | Unified checker | Run `tracker_client check` against a local tracker as the README documents | Works as before | TODO | `manual-verification-evidence.md` section V1 |
| M2 | E2E checker launch | Run the e2e runner (or call the helper) after migrating `tracker_checker.rs` and check that it launches `tracker_client check` with `TORRUST_CHECKER_CONFIG` (for example from the logged command line); the interactive-terminal case is verified in issue 2768 once the TTY refusal exists, and this row is only a smoke test | The checker step runs against the unified binary | TODO | `manual-verification-evidence.md` section V2 |

Manual verification is mandatory even when automated tests pass. Record actual commands, output and the toolchain that produced each result in `manual-verification-evidence.md` (from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`).

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO |  |
| AC2 | TODO |  |
| AC3 | TODO |  |
| AC4 | TODO |  |

## Risks and Trade-offs

- External users still call the old names. Mitigation: they have printed a deprecation warning; add the breaking-change note.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise record in the progress log why none was needed.

## References

- Parent EPIC: #2758
- Related issues: #2715, #1798
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`
