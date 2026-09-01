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
last-updated-utc: 2026-09-01 18:00 UTC
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
| T1  | TODO   | Establish the baseline          | Run package coverage and identify critical untested paths.                                                                                                            |
| T2  | TODO   | Plan coverage improvement       | Identify critical behavior and record the coverage increase achieved without pursuing an arbitrary percentage.                                                        |
| T3  | TODO   | Add lifecycle and routing tests | Cover meaningful listener, route, and middleware gaps; prefer fast tests close to the code when appropriate.                                                          |
| T4  | TODO   | Add protocol-boundary tests     | Cover announce/scrape response selection and error mapping, including behavior already exercised only at higher levels when package-level tests add regression value. |
| T5  | TODO   | Review test design              | Simplify duplicated setup when justified; retain readable AAA-style tests and valuable package integration coverage.                                                  |
| T6  | TODO   | Verify and record evidence      | Complete automated checks, manual scenarios, and the post-implementation AC review.                                                                                   |

## Progress Tracking

### Workflow Checkpoints

- [x] Repository-local folder-style spec created for existing GitHub issue #1348
- [ ] Spec reviewed and approved by user/maintainer
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-09-01 18:00 UTC - GitHub Copilot - Created a repository-local folder-style specification from GitHub issue #1348 and EPIC #1347. - https://github.com/torrust/torrust-tracker/issues/1348
- 2026-09-01 18:00 UTC - User/maintainer - Clarified that the work should increase the recorded coverage baseline by testing critical behavior, prioritize fast unit tests close to package code, and retain or add valuable package-level integration and end-to-end tests. - https://github.com/torrust/torrust-tracker/issues/1348

## Acceptance Criteria

- [ ] A coverage baseline and the coverage increase achieved are recorded, with critical behavior prioritized over an arbitrary percentage.
- [ ] Tests cover all identified critical `axum-http-server` transport gaps, including behavior previously covered only at a higher level when package-level coverage provides regression value.
- [ ] Lifecycle, routing/middleware, and announce/scrape protocol-boundary tests are added or explicitly justified as already covered.
- [ ] Tests reuse appropriate fixtures and remain readable and maintainable.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant tests pass.
- [ ] Manual verification scenarios are executed and documented.
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [ ] Documentation is updated when behavior or workflow changes.

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
| M1  | HTTP tracker announce contract       | Start the test environment and issue valid compact and non-compact announce requests. | Both requests produce valid bencoded tracker responses with the selected peer representation.                   | TODO   | To be recorded during implementation. |
| M2  | Scrape and failure response contract | Issue valid scrape and invalid announce requests against the test environment.        | Scrape data is returned when available; invalid announce input is represented as a BitTorrent failure response. | TODO   | To be recorded during implementation. |
| M3  | Server lifecycle                     | Start and stop the test environment using an ephemeral listener.                      | The server registers, accepts a health check, and stops cleanly.                                                | TODO   | To be recorded during implementation. |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                          |
| ----- | ---------------------- | ------------------------------------------------- |
| AC1   | TODO                   | Coverage command output recorded in progress log. |
| AC2   | TODO                   | Added test paths and test output.                 |
| AC3   | TODO                   | Test output and review notes.                     |
| AC4   | TODO                   | Code review of test fixtures and assertions.      |
| AC5   | TODO                   | `linter all` output.                              |
| AC6   | TODO                   | Package test output.                              |
| AC7   | TODO                   | Manual-verification table and evidence.           |
| AC8   | TODO                   | Post-implementation review entry.                 |
| AC9   | TODO                   | Relevant documentation diff.                      |

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
