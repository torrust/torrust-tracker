---
doc-type: test-refactor-plan-guidance
issue: 2637
package: torrust-tracker-axum-rest-api-server
status: in-progress
---

# Axum REST API Server File Test Plan Guidance

This folder holds one **file test plan** per Rust source file in
`packages/axum-rest-api-server/src/` that needs one (`<module>-tests.md`, with `/` replaced by `-`, for
example `v1-middlewares-auth-tests.md`). The workflow, approval gates, commit points, section layout,
slices, and processing order are defined once in
[ISSUE.md — Per-File Workflow](../ISSUE.md#per-file-workflow), the
[Pull-Request Slices](../ISSUE.md#pull-request-slices), and the
[Source File Ledger](../ISSUE.md#source-file-ledger); do not duplicate them here. The
[coverage evidence](../coverage-evidence.md) holds the numbers, the uncovered-line groups, and the T1
hypothesis for each file.

Create a plan right before starting its file, never in advance, so each plan learns from the
previous ones. A plan authorizes neither test changes nor work on another file until the maintainer
approves it. A group of declaration-only files may share one plan.

## Plans

| Plan sequence | Plan | State | Result |
| ---: | --- | --- | --- |
| 1 | [v1-middlewares-auth-tests.md](v1-middlewares-auth-tests.md) | DONE | Approved 2026-10-26; completed file approved 2026-10-26 |

Add a row when a plan is created; the remaining files and their order are in the ledger.

## Shared Guardrails

- Unit-only coverage is the primary metric. A package integration test requires a documented reason
  that the public-API boundary is clearer or necessary.
- Do not add tests merely for coverage, duplicate `tracker-core` business rules,
  `rest-api-application` use-case orchestration, `rest-api-runtime-adapter` adapters,
  `rest-api-protocol` DTO serialization, or `axum-server` helpers, or cross into lifecycle policy
  owned by #1488. Do not test `src/testing/` (test support; #2482, #2449).
- Judge existing tests against the `write-unit-test` skill (naming `it_should_*`, visible AAA,
  state-centred Arrange, one reason to fail, no production-derived expectations, no hidden Act or
  fixture coupling, no logic or loops in the test). List the concrete smells in the plan; "clean" is
  also a recorded conclusion.
- Test in-process: build only the router, middleware or handler under test and call it with
  `tower::ServiceExt::oneshot`; do not start a socket and do not reuse `testing::environment`.
  Assert whole values (status, `content-type` header, body) against literals written in the test,
  never against production response builders.
- A test whose outcome depends on a collaborator's semantics is a collaboration test; label it as
  such in the plan and keep the package-owned decision visible in its Arrange.
- Property-based tests are tried only where the file plan names a natural property, and kept only
  when they protect state that examples miss; otherwise the plan records why examples suffice.
- Never write a test that pins behavior a finding reports as wrong without a recorded maintainer
  decision. Follow the [findings procedure](../ISSUE.md#findings-during-implementation).
- Use the nightly Rust toolchain for formatting evidence: `cargo +nightly fmt --all -- --check`.
  Use stable Rust for tests and coverage. Record the toolchain with every validation result.
- Run `git diff --check` after each increment and `linter markdown` plus `linter cspell` after
  changing these records.
- Shared helpers require an approved cohesive responsibility; do not create generic fixtures for a
  one-file seam. A port fake used by many handler tests (S3 and S4) is a shared helper: its plan
  defines its contract and tests it against the mutant that breaks that contract.
- Every added or materially changed test gets a **Prose-First AAA Review** row in its plan before
  the file's completed-file review (#1348 retrospective): temporary Arrange, Act, and Assert prose,
  compared with the code, and the outcome.
- New dev-dependencies follow the `add-rust-dependency` skill (latest stable version, minimal
  features, rationale recorded in the plan that needs them) and pass `cargo machete --with-metadata`.

## Testing Lessons Carried From #1348 And #1349

- Re-check an "untestable" path against exported test doubles and derives before recording it as
  untestable. The application ports (`AuthKeyPort`, `WhitelistCommandPort`, `TorrentQueryPort`,
  `StatsQueryPort`) are traits, so a hand-written fake reaches every service outcome without a socket.
- An expectation-free double proves an ordering decision: a handler that records that it ran proves
  that a rejected request never reached it. Confirm the test fails under a temporary order swap.
- Assert whole values against literals, table-test the policy rather than one setting, and write
  mutants that change one property and compile under `unused = "deny"`; revert each exactly.
- Re-measure the package at the start of every slice and diff it against the previous baseline,
  because `develop` moves under the work.

## Reconciliation

Before final verification, reconcile every plan state with the ledger, `coverage-evidence.md`, and
EPIC #1347. Do not cite branch SHAs as evidence until the implementation is merged; use unique
Conventional Commit subjects instead.
