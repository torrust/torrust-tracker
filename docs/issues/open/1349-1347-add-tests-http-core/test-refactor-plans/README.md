---
doc-type: test-refactor-plan-guidance
issue: 1349
package: torrust-tracker-http-core
status: in-progress
---

# HTTP Core File Test Plan Guidance

This folder holds one **file test plan** per Rust source file in `packages/http-core/src/`
(`<module>-tests.md`, for example `services-announce-tests.md`). The workflow, approval gates,
commit points, section layout, slices, and processing order are defined once in
[ISSUE.md — Per-File Workflow](../ISSUE.md#per-file-workflow), the
[Pull-Request Slices](../ISSUE.md#pull-request-slices), and the
[Source File Ledger](../ISSUE.md#source-file-ledger); do not duplicate them here. The
[coverage evidence](../coverage-evidence.md) holds the numbers, the uncovered-line groups, and the
T1 hypothesis for each file.

Create a plan right before starting its file, never in advance, so each plan learns from the
previous ones. A plan authorizes neither test changes nor work on another file until the
maintainer approves it.

## Plans

| Plan sequence | Plan | State | Result |
| ---: | --- | --- | --- |
| 1 | [services-announce-tests.md](services-announce-tests.md) | DONE | Approved 2026-10-16; R1 approved 2026-10-20; completed-file review approved 2026-10-20 |
| 2 | [services-error-mapping-tests.md](services-error-mapping-tests.md) | DONE | Approved 2026-10-20; completed-file review approved 2026-10-20 |

Add a row when a plan is created; the remaining files and their order are in the ledger.

## Shared Guardrails

- Unit-only coverage is the primary metric. A package integration test requires a documented
  reason that the public-API boundary is clearer or necessary.
- Do not add tests merely for coverage, duplicate `tracker-core` business rules (announce handling,
  whitelist policy, key validity, scrape aggregation), `http-protocol` parsing and client-IP
  resolution, or `axum-http-server` transport behavior, or cross into lifecycle policy owned by
  #1488.
- Judge existing tests against the `write-unit-test` skill (naming, visible AAA, state-centred
  Arrange, one reason to fail, no production-derived expectations, no hidden Act or fixture
  coupling, no logic in the test). List the concrete smells in the plan; "clean" is also a recorded
  conclusion.
- A test whose outcome depends on a collaborator's semantics is a collaboration test; label it as
  such in the plan and keep the package-owned decision visible in its Arrange.
- Test bootstraps construct only the collaborators the test needs; they do not reuse production
  container factories.
- Property-based tests are tried only where the file plan names a natural property, and kept only
  when they protect state that examples miss; otherwise the plan records why examples suffice.
- Never write a test that pins behavior a handed-off finding reports as wrong. Follow the
  [findings procedure](../ISSUE.md#findings-during-implementation).
- Use the nightly Rust toolchain for formatting evidence: `cargo +nightly fmt --all -- --check`.
  Use stable Rust for tests and coverage. Record the toolchain with every validation result.
- Run `git diff --check` after each increment and `linter markdown` plus `linter cspell` after
  changing these records.
- Shared helpers require an approved cohesive responsibility; do not create generic fixtures for a
  one-file seam.
- Every added or materially changed test gets a **Prose-First AAA Review** row in its plan before
  the file's completed-file review (#1348 retrospective): temporary Arrange, Act, and Assert prose,
  compared with the code, and the outcome.

## Testing Lessons Carried From #1348

These two lessons came out of the `udp-core` work and apply to this package's services:

- **Re-check an "untestable" error path against `tracker-core`'s exported doubles.** `tracker-core`
  exports `#[automock]` store doubles (keys, whitelist, torrent metrics, schema) under
  `packages/tracker-core/src/databases/traits/`. A failing store reaches database-failure
  adaptation without a database. Do not record such a path as untestable until it has been checked
  against them.
- **An expectation-free double proves an ordering decision.** A double with no expectations panics
  on the first call, so a test that gives the service one and expects the earlier check's error
  proves that the later step never ran. `handle_announce` and `handle_scrape` own an order of
  checks (authentication, authorization, client-IP resolution, peer-IP selection); use this to
  protect it, and confirm the test fails under a temporary order swap.

## Reconciliation

Before final verification, reconcile every plan state with the ledger, `coverage-evidence.md`, and
EPIC #1347. Do not cite branch SHAs as evidence until the implementation is merged; use unique
Conventional Commit subjects instead.
