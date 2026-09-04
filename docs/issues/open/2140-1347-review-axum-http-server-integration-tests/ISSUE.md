---
doc-type: issue
issue-type: task
status: open
priority: p2
epic: 1347
github-issue: 2140
spec-path: docs/issues/open/2140-1347-review-axum-http-server-integration-tests/ISSUE.md
branch: "2140-review-axum-http-server-integration-tests"
related-pr: 2137
last-updated-utc: 2026-09-04
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - docs/testing/refactoring-patterns/README.md
    - packages/axum-http-server/tests/server/v1/contract/configured_as_private_and_whitelisted.rs
    - packages/axum-http-server/tests/server/v1/contract/configured_as_private.rs
    - packages/axum-http-server/tests/server/v1/contract/configured_as_whitelisted.rs
    - packages/http-core/src/services/announce.rs
    - packages/http-core/src/services/scrape.rs
---

<!-- skill-link: create-issue -->

# Issue #2140 - Review and Improve Axum HTTP Server Integration Tests

Parent EPIC: #1347 - Overhaul: Packages Testing

## Goal

Systematically review and improve maintainable package-integration coverage for
`packages/axum-http-server`. Use executable coverage evidence, domain behavior analysis, and a
file-by-file test-design review to identify and close high-value HTTP tracker contract gaps.

## Background

Issue #2136 strengthened package-local unit coverage and retained existing integration coverage at
its appropriate boundary, but it did not perform a complete file-by-file review of the integration
suite. A preliminary assessment found that
`tests/server/v1/contract/configured_as_private_and_whitelisted.rs` contains only placeholder
modules. That is a high-value candidate, but it is not sufficient evidence to limit the successor's
scope before the whole suite, coverage report, and domain contracts have been examined.

This successor continues package testing after #2136 without reopening it. It first establishes an
evidence-based, prioritized integration-test backlog, then implements only the approved
high-value cases from that backlog.

## Scope

### In Scope

- Read every integration-test source under `packages/axum-http-server/tests/`, grouping existing
  contracts by configuration mode, route, protocol behavior, listener behavior, and failure class.
- Measure current package-source coverage with the reproducible `cargo llvm-cov` command and record
  aggregate, per-file, and uncovered-function/region evidence. Treat the test-inclusive aggregate
  as navigation evidence, not proof that observable integration contracts are complete.
- Compare integration tests with the package's transport, router, extractor, lifecycle, and
  `http-core` domain behavior to find meaningful edge cases that coverage alone cannot expose.
- Perform a dedicated, file-by-file test-design review before adding tests. Identify duplication and
  opportunities to improve readability, maintainability, and expressiveness; use the test-pattern
  catalog to select inline values, builders, or scenario fixtures at the right granularity.
- Create and obtain maintainer approval for an integration-test refactor/coverage plan whose items
  are ordered from high-impact/low-effort to lower-impact work.
- Implement only approved behavior-focused test increments, including the combined
  private-and-whitelisted mode if the complete analysis confirms it remains a priority.
- Update issue-local coverage evidence after approved behavior-adding tests and record explicit
  coverage-boundary or deferral decisions for candidates not selected.

### Out of Scope

- Reopening #2136 or changing its completed unit-test/refactor plans.
- Generic cross-file integration-test factories or replacing package-local fixtures with production
  bootstrap factories.
- Automatically adding a test for every uncovered line, region, branch, framework rejection, or
  configuration permutation merely to increase a percentage.
- Root application-composition testing or containerized E2E work unless analysis demonstrates that
  the behavior cannot be covered at the package integration boundary.
- Framework-only error variants, middleware ordering, trace-log text, compression negotiation,
  real timeout waiting, or synthetic server-task failures.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md`
- ADRs to create: None expected. Create one only if test work introduces a durable package or
  cross-package architecture decision.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`, `DEFERRED`.

| ID  | Status | Task                                     | Expected output                                                                                                                                                                |
| --- | ------ | ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| T1  | TODO   | Inventory all integration contracts      | Read every package integration-test file and map its observable behavior, configuration mode, boundary, and existing scenario coverage.                                        |
| T2  | TODO   | Measure and analyze coverage             | Record reproducible package-source aggregate and per-file coverage, then compare uncovered areas with package/domain behavior to identify gaps coverage alone does not reveal. |
| T3  | TODO   | Review test design before expansion      | Create a file-by-file integration-test refactor plan covering readability, maintainability, expressiveness, fixture selection, and justified duplication reduction.            |
| T4  | TODO   | Approve prioritized improvement plan     | Review the evidence-backed plan with the maintainer; implement one approved refactoring or behavior increment at a time.                                                       |
| T5  | TODO   | Improve selected integration contracts   | Add only approved high-value edge cases. The combined private-and-whitelisted announce/scrape matrix is an initial candidate, not a preselected outcome.                       |
| T6  | TODO   | Final verification and acceptance review | Run full package tests, linters, required hooks, manual real-server scenarios, and post-implementation acceptance review.                                                      |

## Test Development Loop

Apply this loop to every test-producing task:

1. First complete T1–T4; do not add a new test before the integration-suite analysis and plan are
   approved.
2. Add the smallest approved behavior-focused integration-test increment.
3. Review the changed test before beginning the next increment. Make the causal initial state
   (authentication plus whitelist state) visible; use an inline fixture, readable builder, or named
   scenario fixture according to the [test-pattern catalog](../../../testing/refactoring-patterns/README.md).
4. Run focused tests and correct failures.
5. After the final test-producing task, stop for maintainer review before final verification,
   committing, or opening a pull request.
6. Address feedback, then complete verification and acceptance review.

## Progress Tracking

### Workflow Checkpoints

- [x] Preliminary integration-suite gap identified during #2136 follow-up analysis.
- [x] Draft folder-style specification created.
- [x] Maintainer reviewed and approved draft specification.
- [x] GitHub subissue #2140 created under #1347.
- [x] Draft moved to `docs/issues/open/` with assigned issue number.
- [ ] Implementation completed.
- [ ] Automatic and manual verification completed.
- [ ] Acceptance criteria reviewed after implementation.

### Progress Log

- 2026-09-04 - GitHub Copilot - Performed a preliminary review of `packages/axum-http-server/tests/` after #2136. The combined private-and-whitelisted configuration has placeholder modules but no contract tests; private-only and whitelisted-only suites are existing references.
- 2026-09-04 - User/maintainer - Expanded the draft scope: before selecting new tests, analyze every package integration test, current coverage, and relevant domain behavior; conduct a dedicated test-design review for readability, maintainability, and expressiveness; then propose the prioritized implementation plan.
- 2026-09-04 - User/maintainer - Approved this specification. GitHub subissue #2140 was created under #1347; this specification is now the open, tracked work item.

## Acceptance Criteria

- [ ] Every `packages/axum-http-server/tests/` integration-test source is analyzed and its current
      contracts, configuration modes, and boundary are recorded.
- [ ] Coverage evidence and domain-behavior analysis identify and prioritize meaningful integration
      gaps; coverage percentages alone do not determine the selection.
- [ ] A dedicated integration-test design review records readability, maintainability, and
      expressiveness opportunities before any new tests are added.
- [ ] Maintainer-approved, high-value integration contracts are added at the real package HTTP
      listener boundary and assert stable observable HTTP/bencoded behavior.
- [ ] New tests make their causal initial states readable and do not hide the Act or assertions in
      generic infrastructure.
- [ ] Out-of-scope timing, framework-internal, generic-bootstrap, root-composition, or E2E work is
      not added without separate evidence and approval.
- [ ] Relevant tests and `linter all` pass.
- [ ] Manual real-server verification is executed and recorded.
- [ ] Package coverage evidence is refreshed or an explicit integration-only measurement limitation
      is recorded.

## Verification Plan

### Automatic Checks

- `cargo test -p torrust-tracker-axum-http-server --test integration`
- `cargo test -p torrust-tracker-axum-http-server`
- `linter all`
- `cargo +nightly fmt --all -- --check`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh`

### Manual Verification Scenarios

| ID  | Scenario                                    | Command/steps                                                     | Expected result                                                                               | Status | Evidence |
| --- | ------------------------------------------- | ----------------------------------------------------------------- | --------------------------------------------------------------------------------------------- | ------ | -------- |
| M1  | Selected HTTP tracker integration contracts | Run focused real-server tests selected by the approved plan.      | Each selected configuration and edge case has its documented observable response/side effect. | TODO   | —        |
| M2  | Full package integration suite              | Run the full package integration target after the last increment. | Existing and newly selected HTTP listener contracts pass together.                            | TODO   | —        |

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | TODO   | —        |
| AC2   | TODO   | —        |
| AC3   | TODO   | —        |
| AC4   | TODO   | —        |
| AC5   | TODO   | —        |
| AC6   | TODO   | —        |
| AC7   | TODO   | —        |
| AC8   | TODO   | —        |

## Risks and Trade-offs

- Broad analysis can become speculative. Maintain a prioritized evidence table and select only
  observable contracts that have a clear package-integration boundary.
- A combined private-and-whitelisted matrix can become repetitive. If selected, keep only distinct
  authentication/whitelist outcomes; do not reproduce private-only or whitelisted-only assertions
  without a combined-mode interaction.
- The existing real-listener environment is slower than unit tests but is the correct package
  boundary for validating HTTP route extraction, bencoding, authentication, and authorization
  together.
- Package source coverage aggregates test-inclusive code and may not isolate integration-test value.
  Treat coverage as navigation evidence, not a substitute for the selected behavioral contracts.

## References

- Parent EPIC: https://github.com/torrust/torrust-tracker/issues/1347
- Completed predecessor: https://github.com/torrust/torrust-tracker/issues/2136
- Target tests: `packages/axum-http-server/tests/server/v1/contract/configured_as_private_and_whitelisted.rs`
- Private reference tests: `packages/axum-http-server/tests/server/v1/contract/configured_as_private.rs`
- Whitelisted reference tests: `packages/axum-http-server/tests/server/v1/contract/configured_as_whitelisted.rs`
