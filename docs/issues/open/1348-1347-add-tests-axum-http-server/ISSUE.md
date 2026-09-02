---
doc-type: issue
issue-type: task
status: open
priority: p2
epic: 1347
github-issue: 1348
spec-path: docs/issues/open/1348-1347-add-tests-axum-http-server/ISSUE.md
branch: "1348-add-tests-axum-http-server"
related-pr: null
last-updated-utc: 2026-09-01 18:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #1348 - Add Tests to the Axum HTTP Server Package

Parent EPIC: #1347 - Overhaul: Packages Testing

## Goal

Improve maintainable test coverage for `torrust-tracker-axum-http-server`, concentrating on a fast, package-local safety net for its HTTP tracker transport contracts, listener lifecycle, and protocol-response boundary.

## Background

`axum-http-server` implements the BitTorrent HTTP tracker transport, including announce, scrape, health-check routes, server lifecycle, and HTTP middleware. Historical high-level tests cover much of this behavior, but contributors changing this independently publishable package need a stronger, faster safety net close to the code. This issue records a package-specific coverage baseline, aims to increase it, and closes material gaps through valuable unit, integration, or end-to-end tests.

## Scope

### In Scope

- Record the starting package coverage baseline and the coverage increase achieved while testing critical behavior.
- Prefer fast unit tests close to the implementation whenever they provide the appropriate regression boundary.
- Review and improve tests for HTTP/HTTPS listener binding, startup registration cleanup, and graceful shutdown.
- Test route availability and transport behavior for announce, scrape, health checks, request IDs, timeouts, and client address handling where gaps exist.
- Test compact versus non-compact announce responses and service-error-to-BitTorrent-failure response mapping where gaps exist.
- Reuse the existing test environment and shared test helpers where they fit the test boundary.

### Out of Scope

- Unrelated production refactoring; a small refactoring is allowed only when needed to create a clear test seam.
- Arbitrary coverage-percentage targets that displace testing of critical behavior.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md`
- ADRs to create: None known. Create one if implementation requires a lasting transport-architecture decision.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                            | Notes / Expected Output                                                                                                                                               |
| --- | ------ | ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Establish the baseline          | Baseline recorded: 1,229 lines, 1,153 covered (93.82%); 1,763 regions, 1,616 covered (91.66%); 153 functions, 137 covered (89.54%).                                  |
| T2  | DONE   | Plan coverage improvement       | The response adapters were the smallest direct package seam: previous higher-level tests exercised them, but no focused tests decoded the handlers' returned bencode. |
| T3  | DONE   | Add lifecycle and routing tests | Added fast router tests for propagated and generated request IDs, plus a lifecycle test that proves failed registration releases the HTTP listener. |
| T4  | DONE   | Add protocol-boundary tests     | Added fast handler tests that decode accepted, omitted, and non-compact announce responses and scrape responses from domain data. |
| T5  | DONE   | Review test design              | Tests use the existing `PeerBuilder`, protocol deserializers, AAA comments, direct handler-response seams, and an in-process router; no production refactor was needed. |
| T6  | DONE   | Verify and record evidence      | Automated and manually invoked real-server verification evidence is recorded below; final independent review and acceptance review passed. |

## Progress Tracking

### Workflow Checkpoints

- [x] Repository-local folder-style spec created for existing GitHub issue #1348
- [x] Spec reviewed and approved by user/maintainer
- [x] Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed
- [x] Manual verification scenarios executed and recorded
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-09-01 18:00 UTC - GitHub Copilot - Created a repository-local folder-style specification from GitHub issue #1348 and EPIC #1347. - https://github.com/torrust/torrust-tracker/issues/1348
- 2026-09-01 18:00 UTC - User/maintainer - Clarified that the work should increase the recorded coverage baseline by testing critical behavior, prioritize fast unit tests close to package code, and retain or add valuable package-level integration and end-to-end tests. - https://github.com/torrust/torrust-tracker/issues/1348
- 2026-09-01 - GitHub Copilot - Measured package-source coverage with `cargo llvm-cov -p torrust-tracker-axum-http-server --all-features --json`, summing the `summary` objects for files beneath `packages/axum-http-server/src/`: baseline: 1,229 lines / 1,153 covered (93.82%), 1,763 regions / 1,616 covered (91.66%), and 153 functions / 137 covered (89.54%). Final: 1,441 lines / 1,368 covered (94.93%), 2,072 regions / 1,930 covered (93.15%), and 172 functions / 156 covered (90.70%). These package-source totals include test code and are not a production-only coverage measurement; the new tests directly assert previously high-level-only response-adapter, request-ID middleware, and registration-cleanup behavior.
- 2026-09-01 - GitHub Copilot - Automatic verification passed: `cargo test -p torrust-tracker-axum-http-server` (30 unit tests and 55 integration tests), `cargo test -p torrust-tracker-axum-http-server --test integration` (55 tests), `linter all`, and `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=json`. The complexity audit found all added tests and helpers to have cyclomatic complexity 1, nesting depth 0, and fewer than 50 lines. `cargo clippy --package torrust-tracker-axum-http-server -- -W clippy::cognitive_complexity -D warnings` was blocked only by two pre-existing high-complexity diagnostics in `torrust-tracker-swarm-coordination-registry`; the normal Clippy validation included in `linter all` passed.
- 2026-09-01 - Task Reviewer - Independently reviewed the focused response-adapter tests. The compact-response test name was corrected to state that it verifies the omitted-parameter default. Review identified follow-up work: use reproducible package-source coverage totals, complete manual verification, and keep EPIC/subissue progress state aligned. The focused tests themselves passed review.
- 2026-09-01 - GitHub Copilot - Added fast in-process router tests for client-supplied and generated request IDs, a listener-release test for registration failure, and an explicit accepted-compact response test. Re-ran focused real-server announce, scrape, health-check, and start/stop scenarios successfully.
- 2026-09-01 - Task Reviewer - Focused tests passed review. Follow-up review identified documentation-evidence alignment work, which was corrected before final review.
- 2026-09-01 - Task Reviewer - Final independent review passed after verifying focused test scope, reproducible coverage evidence, manually invoked real-server scenarios, and documentation consistency.

## Acceptance Criteria

- [x] A coverage baseline and the coverage increase achieved are recorded, with critical behavior prioritized over an arbitrary percentage.
- [x] Tests cover all identified critical `axum-http-server` transport gaps, including behavior previously covered only at a higher level when package-level coverage provides regression value.
- [x] Lifecycle, routing/middleware, and announce/scrape protocol-boundary tests are added or explicitly justified as already covered.
- [x] Tests reuse appropriate fixtures and remain readable and maintainable.
- [x] `linter all` exits with code `0`.
- [x] Relevant tests pass.
- [x] Manual verification scenarios are executed and documented.
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [x] Documentation is updated when behavior or workflow changes.

## Verification Plan

### Automatic Checks

- `cargo llvm-cov -p torrust-tracker-axum-http-server --all-features --summary-only`
- `cargo test -p torrust-tracker-axum-http-server`
- `cargo test -p torrust-tracker-axum-http-server --test integration`
- `linter all`
- Pre-push checks when applicable

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                             | Command/Steps                                                                         | Expected Result                                                                                                 | Status | Evidence                              |
| --- | ------------------------------------ | ------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- | ------ | ------------------------------------- |
| M1  | HTTP tracker announce contract       | Invoke the specified real-server integration tests for compact and non-compact announce requests. | Both requests produce valid bencoded tracker responses with the selected peer representation.                   | DONE   | Invoked `should_return_the_compact_response` and `should_return_the_list_of_previously_announced_peers`; both passed. |
| M2  | Scrape and failure response contract | Invoke the specified real-server integration tests for valid scrape and invalid announce requests. | Scrape data is returned when available; invalid announce input is represented as a BitTorrent failure response. | DONE   | Invoked `should_return_the_file_with_the_incomplete_peer_when_there_is_one_peer_with_bytes_pending_to_download` and `should_fail_when_the_url_query_component_is_empty`; both passed. |
| M3  | Server lifecycle                     | Invoke the specified real-server integration tests for health checks and start/stop. | The server registers, accepts a health check, and stops cleanly.                                                | DONE   | Invoked `health_check_endpoint_should_return_ok_if_the_http_tracker_is_running` and `it_should_start_and_stop`; both passed. |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                          |
| ----- | ---------------------- | ------------------------------------------------- |
| AC1   | DONE                   | Coverage command output recorded in progress log. |
| AC2   | DONE                   | Added test paths and test output.                 |
| AC3   | DONE                   | Test output and review notes.                     |
| AC4   | DONE                   | Code review of test fixtures and assertions.      |
| AC5   | DONE                   | `linter all` output.                              |
| AC6   | DONE                   | Package test output.                              |
| AC7   | DONE                   | Manual-verification table and evidence.           |
| AC8   | DONE                   | Post-implementation review entry.                 |
| AC9   | DONE                   | Relevant documentation diff.                      |

All planned work is complete and has independent review evidence recorded above. No public behavior, workflow, or governance change required documentation beyond this issue specification and the EPIC progress update.

## Risks and Trade-offs

- End-to-end-style fixture tests can be slow and costly to compose; prefer direct router or focused unit tests where they cover the intended boundary, while keeping higher-level tests that provide distinct value.
- Testing implementation details creates brittle tests; assert public HTTP and bencoded protocol contracts instead.
- TLS health checks require appropriately configured trust in tests; use the injectable client seam rather than weakening production TLS behavior.

## References

- GitHub issue: https://github.com/torrust/torrust-tracker/issues/1348
- Parent EPIC: #1347
- Package: `packages/axum-http-server/`
- Test environment: `packages/axum-http-server/src/testing/environment.rs`
- Protocol/domain boundary ADR: `docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md`
- Historical test-environment work: `docs/issues/closed/1904-1669-si-24-relocate-http-server-test-environment.md`
