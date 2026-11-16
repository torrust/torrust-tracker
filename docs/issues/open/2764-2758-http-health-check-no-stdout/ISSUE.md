---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p2
epic: 2758
github-issue: 2764
spec-path: docs/issues/open/2764-2758-http-health-check-no-stdout/ISSUE.md
branch: "2764-http-health-check-no-stdout"
related-pr: 2889
last-updated-utc: "2026-11-16 19:09"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .claude/skills/create-issue/SKILL.md
    - .claude/skills/fix-bug/SKILL.md
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

- `src/bin/http_health_check.rs` (and its tests)
- `Containerfile` (verify only; edit only if the command line must change)

## Architectural Decisions

Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`. ADRs to create: None known.

During implementation, stop and create an ADR when a decision affects project architecture or design patterns, selects an approach among meaningful alternatives, or has consequences future contributors need to understand. Do not create ADRs for routine implementation details.

## Design and Ownership Review

The test plan spawns the binary as a child process against a local test server, so this section applies to that fixture.

- Responsibilities: `http_health_check` makes one probe and maps it to an exit code and JSON records; the test fixture owns the test server (an ephemeral port it binds and serves) and the child process it spawns; the test body owns the assertions.
- Lifetime invariants: the fixture binds the server before the child starts and stops it, and waits for or kills the child, on the normal path and on assertion failure or panic (a guard that cleans up on drop). The unreachable case holds a socket bound to port 0 that never listens for the whole probe, so no concurrent test can take the port and answer, and the connection is refused (the form of `packages/axum-rest-api-server/src/server.rs`; changed from bind-and-close after review finding `review-finding:pr-2889-f1`).
- Deadline: the binary's own request timeout (5 seconds) bounds each probe; the fixture adds an absolute deadline to the child wait, and every readiness wait for the server has an absolute deadline.
- Design-review checkpoint: after the first passing vertical slice (healthy and unhealthy cases), review the fixture ownership before adding the remaining cases.

## Bug-Fix Process

This issue fixes a deviation from the CLI output contract, so it is bug work in substance. Follow the [fix-bug](../../../../.claude/skills/fix-bug/SKILL.md) skill:

- analysis of the defect: the tool's current behavior against the contract (see Background);
- real-artifact reproduction: run the real binary, record the commands and output of the deviation in `manual-verification-evidence.md` before changing code;
- regression-test boundary: see Regression Test Strategy;
- red regression test, then the production fix;
- green test evidence and a like-for-like recheck of the same real-artifact commands.

## Regression Test Strategy

A test that runs the binary against a local test server and a port that is bound but not listening and asserts stdout empty, JSON records on stderr and the exit codes. It fails today (plain text on stdout, exit `1` for usage). If a higher boundary is used instead, record why it is clearer or the only practical option; link red and green output and the final recheck to `manual-verification-evidence.md`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | DONE | Tests for exit codes and records against a local test server | Red tests: evidence R1. |
| T2 | DONE | Rewrite `main` to the contract | Green tests: evidence G1; no `print_*` allow. |
| T3 | DONE | Verify the container `HEALTHCHECK` | `docker inspect` shows `healthy` for a container whose probe reaches the health API, and `unhealthy` for a container whose probe port has nothing listening (the health API cannot be stopped on its own; see evidence V2). |

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
- [x] Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-06 16:43 UTC - Claude Code (Claude Sonnet 5.5) - Drafted this spec as part of EPIC #2758 from the 2026-11-06 audit.
- 2026-11-06 16:51 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the spec; GitHub issue #2764 created.
- 2026-11-09 10:25 UTC - Claude Code (Claude Sonnet 5.5) - Corrected details after the review rounds on PR 2771 and PR 2774 and fixed the earlier log stamps from the commit and issue creation times.
- 2026-11-09 12:12 UTC - Claude Code (Claude Sonnet 5.5) - Addressed the review of PR 2771 (da2ce7 findings F1 to F27 and Copilot's later comments): specs corrected from the code, reproductions recorded where a spec has a bug-fix section.
- 2026-11-16 16:59 UTC - Claude Code (Claude Sonnet 5.5) - Implemented T1 to T3 on a branch rebased onto `torrust/develop` after the image-build job cap (PR 2854) merged. The tests ran red against the old binary, the binary now follows the contract, and the evidence (R1, R2, G1, V1 to V3) is recorded. The first image build failed because the image test stages could not find the binary through `CARGO_BIN_EXE_http_health_check`; the test now reads `NEXTEST_BIN_EXE_http_health_check` first, and the second build passed with 1883 tests. No `Containerfile` change was needed. `healthy` emits one stderr record (see Risks).
- 2026-11-16 17:49 UTC - Claude Code (Claude Opus 5.5) - Correction to the entry stamped 2026-11-16 16:59 UTC (review finding `review-finding:pr-2889-f7`): that entry and `last-updated-utc` were written before the commit that carries them, which was made at 16:58:26Z; the stamp is later than its commit. The entry is kept as written.
- 2026-11-16 17:49 UTC - Claude Code (Claude Opus 5.5) - Opened PR 2889 (`related-pr`). Addressed review round 1 (da2ce7 review 5924341133, Copilot review 5924435062; audit in `docs/pr-reviews/pr-2889-review/PR-REVIEW.md`): the unreachable test holds a bound, non-listening port; the binary reads `args_os`, reports a non-UTF-8 or malformed argument as a usage error, keeps the URL out of every record (contract redaction policy) and keeps the outcome exit code when the record cannot be written; the tests show stderr in exit-code assertions and clear the proxy variables; M2 and T3 describe the run that happened; R1 and R2 are completed. Rechecked G1, V1 to V3 on the changed binary (evidence section "Round 1 of PR 2889"). `tests/AGENTS.md` mentions the tests of root `src/bin` tools.
- 2026-11-16 18:15 UTC - Claude Code (Claude Opus 5.5) - Addressed review round 2 of PR 2889 (da2ce7 review 5925054031, APPROVED, audit findings F13 to F15): a URL scheme other than `http` or `https` is a usage error (new test, red then green); R2 gains mutation rows for exit 0 on a 2xx answer and exit 1 on a failed request, and AC3 names the five clauses R2 covers; the round-1 image run is kept with the stage-result argument written out.
- 2026-11-16 18:29 UTC - Claude Code (Claude Opus 5.5) - Addressed review round 3 of PR 2889 (da2ce7 review 5925258553, APPROVED, audit findings F16 and F17): reran the V1 and V3 real-tracker commands, plus an `ftp://` case, on the build of the last production change (evidence section "Round 3 of PR 2889"); the R2 timing sentence names its rows by clause.
- 2026-11-16 18:54 UTC - Claude Code (Claude Opus 5.5) - Addressed review round 4 of PR 2889 (Copilot review 5925595121, audit findings F18 to F22): the evidence marks G1, V1 to V3 as runs on the first implementation and says where the results for the final binary are; the test server fails the test on an `accept` error other than `WouldBlock` or `Interrupted` (failure path checked, evidence section "Round 4 of PR 2889").
- 2026-11-16 19:09 UTC - Claude Code (Claude Opus 5.5) - Addressed review round 5 of PR 2889 (da2ce7 review 5925836500, audit findings F23 and F24): the evidence records that the round-4 accept-error check covered the panic arm only (the exit-code assertion fired) and adds a run where the server panics after answering, which covers the `Drop` re-raise; the evidence's ordering note no longer says the sections are in run order.

## Acceptance Criteria

- [x] AC1: stdout is empty in every case.
- [x] AC2: Every stderr line is one JSON object with a `kind`.
- [x] AC3: Exit codes are `0`, `1`, `2` as specified.
- [x] AC4: The container still reports `unhealthy` when the endpoint fails.
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [x] Documentation is updated when behavior or workflow changes (no document describes the tool's output; the ADR's classification table is out of scope for this issue)

## Verification Plan

### Automatic Checks

- `linter all`
- Relevant tests for changed components (`cargo test -p <package>`)
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Healthy and unhealthy | Run against a running tracker's `/health_check`, then a closed port | Exit `0` then `1`; stderr is NDJSON | DONE | `manual-verification-evidence.md` section V1 |
| M2 | Container health | Build the image; run it and wait for `healthy`; run a second container with `HEALTH_CHECK_API_PORT=1314` (nothing listens there) and wait past the retries. Stopping the health API alone is not possible: it runs in the tracker process | `docker inspect` reports `healthy` for the first container and `unhealthy` for the second | DONE | `manual-verification-evidence.md` section V2 |
| M3 | Usage error | `http_health_check` with no argument | Exit `2`, JSON record | DONE | `manual-verification-evidence.md` section V3 |

Manual verification is mandatory even when automated tests pass. Record actual commands, output and the toolchain that produced each result in `manual-verification-evidence.md` (from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`).

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | Evidence V1, V3 and the tests: stdout is 0 bytes in every case |
| AC2 | DONE | Evidence V1, V3, V2; the test helper parses every stderr line as one JSON object with a `kind` |
| AC3 | DONE | Evidence V1, V3 (`0`, `1`, `1`, `2`); the tests; mutation checks R2: exit `0` on a 2xx answer, exit `1` on a non-2xx answer, exit `1` on a failed request, exit `2` on a usage error, and JSON on stderr |
| AC4 | DONE | Evidence V2: `unhealthy` after the probe exited `1` (`unreachable`) |

## Risks and Trade-offs

- Logging to stderr on every healthy probe is noisy in container logs. Decision: `healthy` emits one record. The `HEALTHCHECK` output is kept by Docker in `.State.Health.Log` (the last 5 probes), not in the container's logs, so the cost is small and the record shows the status (evidence V2).

## Implementation Completion Review

- Retrospective: not needed. One unplanned finding, recorded in the evidence (G1) and the progress log: the image test stages could not find the binary through `CARGO_BIN_EXE_*`, fixed in the test.
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise record in the progress log why none was needed.

## References

- Parent EPIC: #2758
- Related issues: #2715, #1798
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`, `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`
