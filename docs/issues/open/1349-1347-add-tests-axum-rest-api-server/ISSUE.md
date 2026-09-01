---
doc-type: issue
issue-type: task
status: open
priority: p2
epic: 1347
github-issue: 1349
spec-path: docs/issues/open/1349-1347-add-tests-axum-rest-api-server/ISSUE.md
branch: "1349-add-tests-axum-rest-api-server"
related-pr: null
last-updated-utc: 2026-09-01 18:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #1349 - Add Tests to the Axum REST API Server Package

Parent EPIC: #1347 - Overhaul: Packages Testing

## Goal

Improve maintainable test coverage for `torrust-tracker-axum-rest-api-server`, building a fast, package-local safety net for its internal REST transport contracts, authentication, configuration-gated routing, and lifecycle behavior.

## Background

`axum-rest-api-server` exposes the tracker management API below `/api/v1`, applies token authentication, and composes context-specific routes. Historical high-level tests cover much of this behavior, but contributors changing this independently publishable package need a stronger, faster safety net close to the code. This issue establishes a package coverage baseline, aims to increase it, and adds valuable unit, integration, or end-to-end coverage at the implementation boundary.

## Scope

### In Scope

- Record the starting package coverage baseline and the coverage increase achieved while testing critical behavior.
- Prefer fast unit tests close to the implementation whenever they provide the appropriate regression boundary.
- Review and improve tests for HTTP/HTTPS startup, registration failure cleanup, listener errors, and graceful shutdown.
- Test public routing and middleware contracts: unauthenticated health checks, protected API routes, token sources and precedence, and transport headers.
- Test private and listed configuration-gated route composition and unavailable-route behavior.
- Test response serialization, extraction, and error mapping where those transport contracts are not already adequately covered.

### Out of Scope

- Unrelated production refactoring; a small refactoring is allowed only when needed to create a clear test seam.
- Arbitrary coverage-percentage targets that displace testing of critical behavior.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260623200526_adopt_contract-first_architecture_for_rest_api.md`
- ADRs to create: None known. Create one if implementation requires a lasting REST transport-architecture decision.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                  | Notes / Expected Output                                                                                                                                                                  |
| --- | ------ | ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Establish the baseline                | Run package coverage and identify critical untested paths.                                                                                                                               |
| T2  | TODO   | Plan coverage improvement             | Identify critical behavior and record the coverage increase achieved without pursuing an arbitrary percentage.                                                                           |
| T3  | TODO   | Add routing and authentication tests  | Cover public health checks, protection, token semantics, and middleware gaps; prefer fast tests close to the code when appropriate.                                                      |
| T4  | TODO   | Add configuration and lifecycle tests | Cover configuration-gated routes and meaningful server lifecycle gaps, including behavior otherwise covered only at higher levels when package-level coverage provides regression value. |
| T5  | TODO   | Review transport boundaries           | Cover behavior at the level it is implemented; simplify setup only when justified and retain valuable package integration coverage.                                                      |
| T6  | TODO   | Verify and record evidence            | Complete automated checks, manual scenarios, and the post-implementation AC review.                                                                                                      |

## Progress Tracking

### Workflow Checkpoints

- [x] Repository-local folder-style spec created for existing GitHub issue #1349
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

- 2026-09-01 18:00 UTC - GitHub Copilot - Created a repository-local folder-style specification from GitHub issue #1349 and EPIC #1347. - https://github.com/torrust/torrust-tracker/issues/1349
- 2026-09-01 18:00 UTC - User/maintainer - Clarified that the work should increase the recorded coverage baseline by testing critical behavior, prioritize fast unit tests close to package code, and retain or add valuable package-level integration and end-to-end tests. - https://github.com/torrust/torrust-tracker/issues/1349

## Acceptance Criteria

- [ ] A coverage baseline and the coverage increase achieved are recorded, with critical behavior prioritized over an arbitrary percentage.
- [ ] Tests cover identified critical REST-server transport gaps, including behavior previously covered only at a higher level when package-level coverage provides regression value.
- [ ] Authentication, public/protected routing, configuration-gated routes, and lifecycle behavior are tested or explicitly justified as already covered.
- [ ] Tests reuse appropriate fixtures and remain readable and maintainable.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant tests pass.
- [ ] Manual verification scenarios are executed and documented.
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [ ] Documentation is updated when behavior or workflow changes.

## Verification Plan

### Automatic Checks

- `cargo llvm-cov -p torrust-tracker-axum-rest-api-server --all-features --summary-only`
- `cargo test -p torrust-tracker-axum-rest-api-server`
- `cargo test -p torrust-tracker-axum-rest-api-server --test integration`
- `linter all`
- Pre-push checks when applicable

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                     | Command/Steps                                                                                                       | Expected Result                                                                       | Status | Evidence                              |
| --- | ---------------------------- | ------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- | ------ | ------------------------------------- |
| M1  | Public and protected routes  | Start the test environment, request `/api/health_check`, then a protected API route with and without a valid token. | Health check succeeds without authentication; protected routes require a valid token. | TODO   | To be recorded during implementation. |
| M2  | Token precedence             | Request a protected route with conflicting bearer-header and query-string tokens.                                   | The header token takes precedence and the response reflects its validity.             | TODO   | To be recorded during implementation. |
| M3  | Configuration-gated contexts | Start private/listed and disabled-mode configurations, then call auth-key and whitelist routes.                     | Routes are available only in their enabled configuration modes.                       | TODO   | To be recorded during implementation. |
| M4  | Server lifecycle             | Start and stop the test environment using an ephemeral listener.                                                    | The API server accepts a health check and stops cleanly.                              | TODO   | To be recorded during implementation. |

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

- The existing REST server fixture composes multiple services and can make contract tests expensive; prefer direct router tests when they exercise the correct transport boundary, while keeping higher-level tests that provide distinct value.
- Tests that reach into private authentication helpers may constrain refactoring; favor HTTP-level authentication contracts unless a narrow unit test has clear value.
- Configuration matrices can multiply test time; cover meaningful route-composition distinctions without duplicating equivalent endpoint tests.

## References

- GitHub issue: https://github.com/torrust/torrust-tracker/issues/1349
- Parent EPIC: #1347
- Package: `packages/axum-rest-api-server/`
- Test environment: `packages/axum-rest-api-server/src/testing/environment.rs`
- REST contract-first ADR: `docs/adrs/20260623200526_adopt_contract-first_architecture_for_rest_api.md`
- Historical test-environment work: `docs/issues/closed/1903-1669-si-23-relocate-axum-rest-api-server-test-environment.md`
