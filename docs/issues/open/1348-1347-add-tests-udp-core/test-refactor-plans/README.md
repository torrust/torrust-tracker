---
doc-type: test-refactor-plan-guidance
issue: 1348
package: torrust-tracker-udp-core
status: in-progress
---

# UDP Core File Test Plan Guidance

This folder holds one **file test plan** per Rust source file in `packages/udp-core/src/`
(`<module>-tests.md`, for example `services-announce-tests.md`). The workflow, approval gates,
commit points, section layout, and processing order are defined once in
[ISSUE.md — Per-File Workflow](../ISSUE.md#per-file-workflow) and the
[Source File Ledger](../ISSUE.md#source-file-ledger); do not duplicate them here. The
[coverage inventory](../coverage-evidence.md) holds the numbers and the T1 hypothesis for each file.

Create a plan right before starting its file, never in advance, so each plan learns from the
previous ones. A plan authorizes neither test changes nor work on another file until the
maintainer approves it.

## Plans

| Order | Plan | State | Result |
| ---: | --- | --- | --- |
| 1 | [services-announce-tests.md](services-announce-tests.md) | IN_PROGRESS | Plan approved; R2 paused for the maintainer's bug-fix PR |

Add a row when a plan is created; the remaining files and their order are in the ledger.

## Shared Guardrails

- Unit-only coverage is the primary metric. A package integration test requires a documented
  reason that the public-API boundary is clearer or necessary.
- Do not add tests merely for coverage, duplicate `tracker-core` business rules, `udp-protocol`
  encoding, or `udp-server` transport behavior, or cross into lifecycle policy owned by #1488.
- Judge existing tests against the `write-unit-test` skill (naming, visible AAA, state-centred
  Arrange, one reason to fail, no production-derived expectations, no hidden Act or fixture
  coupling). List the concrete smells in the plan; "clean" is also a recorded conclusion.
- A test whose outcome depends on a collaborator's semantics is a collaboration test; label it as
  such in the plan and keep the package-owned decision visible in its Arrange.
- Test bootstraps construct only the collaborators the test needs; they do not reuse production
  container factories.
- Property-based tests are tried only where the file plan names a natural property, and kept only
  when they protect state that examples miss; otherwise the plan records why examples suffice.
- Use the nightly Rust toolchain for formatting evidence: `cargo +nightly fmt --all -- --check`.
- Record the toolchain/runtime with every validation result. Run `git diff --check` after each
  increment and `linter markdown` plus `linter cspell` after changing these records.
- Shared helpers require an approved cohesive responsibility; do not create generic fixtures for a
  one-file seam.

## Reconciliation

Before final verification, reconcile every plan state with the ledger, `coverage-evidence.md`, and
EPIC #1347. Do not cite branch SHAs as evidence until the implementation is merged; use unique
Conventional Commit subjects instead.
