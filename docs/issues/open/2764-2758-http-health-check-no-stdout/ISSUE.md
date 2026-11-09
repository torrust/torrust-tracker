---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2758
github-issue: 2764
spec-path: docs/issues/open/2764-2758-http-health-check-no-stdout/ISSUE.md
branch: "2764-http-health-check-no-stdout"
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

# Issue #2764 - `http_health_check`: NDJSON diagnostics and usage error exit 2

**Parent EPIC:** #2758 - Migrate the CLI tools to the global output contract. Order 5 of 11, wave 1.

## Goal

`http_health_check` becomes a compliant `no-stdout-result` program: empty stdout, NDJSON diagnostics on stderr, exit `0` healthy, `1` unhealthy or unreachable, `2` usage error.

## Background

`src/bin/http_health_check.rs` prints `Health check ...` and `STATUS: <code>` to stdout, plain-text errors, and exits `1` for a usage error; it carries `#![allow(clippy::print_stdout, ...)]`. It is the container `HEALTHCHECK` in the `Containerfile` (`CMD /usr/bin/http_health_check http://localhost:${HEALTH_CHECK_API_PORT}/health_check`), so unhealthy must stay a non-zero exit. Maintainer decision (2026-11-06): classify it `no-stdout-result` (the table change is in EPIC issue 1). The binary avoids third-party crates on purpose (see its module doc); keep dependencies minimal, and write the JSON without adding a crate if `reqwest` already brings what is needed.

The global CLI output contract (`docs/adrs/20260519000000_define_global_cli_output_contract.md`), as amended by `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`, requires: result data only on stdout (one JSON object) for `stdout-result-data` commands, nothing on stdout for `no-stdout-result` commands; every stderr record is one JSON object per line (NDJSON) with a `kind`; no plain text on either channel; exit codes only `0` (did its job, the answer is in the result), `1` (could not do its job) and `2` (usage error or TTY refusal); and TTY refusal for `stdout-result-data` commands. A distinct outcome is a `kind` field: an answer to what the caller asked is a stdout result with exit `0`; "could not answer" is a stderr diagnostic with exit `1`. This issue is part of EPIC #2758, which migrates the tools found non-compliant by the 2026-11-06 audit. The reference implementations are `contrib/dev-tools/checks/ci-gate/src/main.rs` (`stdout-result-data`) and `contrib/dev-tools/checks/shipped-features-check/src/main.rs` (`no-stdout-result`).

## Scope

### In Scope

- Emit nothing on stdout.
- On stderr, one JSON record per event with a `kind` (for example `usage_error`, `unhealthy` with the HTTP status, `unreachable` with the error, `healthy`).
- Exit `0` on a 2xx response, `1` on non-2xx or a request failure, `2` on usage error.
- Remove the `print_*` allow, using the same stderr JSON writer pattern as the other tools.
- Tests for the exit codes and records; confirm the `Containerfile` `HEALTHCHECK` still works.

### Out of Scope

- Changing the health endpoint or its timeout.
- Editing the ADR.

Files this issue may touch (a sibling issue in the same wave must not touch them):

- src/bin/http_health_check.rs (and its tests)
- Containerfile (verify only; edit only if the command line must change)

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
| T1 | TODO | Tests for exit codes and records against a local test server | Red tests. |
| T2 | TODO | Rewrite `main` to the contract | Green tests; no `print_*` allow. |
| T3 | TODO | Verify the container `HEALTHCHECK` | `docker inspect` shows `healthy`, then `unhealthy` after stopping the endpoint. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Tests for exit codes and records against a local test server | Commit after focused validation and required review. |
| T2 | Rewrite `main` to the contract | Commit after focused validation and required review. |
| T3 | Verify the container `HEALTHCHECK` | Commit after focused validation and required review. |

Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG. For test-producing work, use the `write-unit-test` skill and complete the design review it requires before each commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2758-http-health-check-no-stdout/ISSUE.md`
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
- 2026-11-06 17:25 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the spec; GitHub issue #2764 created.

## Acceptance Criteria

- [ ] AC1: stdout is empty in every case.
- [ ] AC2: Every stderr line is one JSON object with a `kind`.
- [ ] AC3: Exit codes are `0`, `1`, `2` as specified.
- [ ] AC4: The container still reports `unhealthy` when the endpoint fails.
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
| M1 | Healthy and unhealthy | Run against a running tracker's `/health_check`, then a closed port | Exit `0` then `1`; stderr is NDJSON | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Container health | Build the image, run it, stop the health API, wait for the interval | `docker inspect` reports `unhealthy` | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Usage error | `http_health_check` with no argument | Exit `2`, JSON record | TODO | `manual-verification-evidence.md` section V3 |

Manual verification is mandatory even when automated tests pass. Record actual commands, output and the toolchain that produced each result in `manual-verification-evidence.md` (from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`).

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO |  |
| AC2 | TODO |  |
| AC3 | TODO |  |
| AC4 | TODO |  |

## Risks and Trade-offs

- Logging to stderr on every healthy probe is noisy in container logs. Mitigation: decide in the issue whether `healthy` emits a record; the contract does not require one.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise record in the progress log why none was needed.

## References

- Parent EPIC: #2758
- Related issues: #2715, #1798
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`
