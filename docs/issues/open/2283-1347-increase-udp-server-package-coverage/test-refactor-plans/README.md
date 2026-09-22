---
doc-type: test-refactor-plan-guidance
issue: 2283
package: torrust-tracker-udp-server
status: in-progress
---

# UDP Server File Test Plan Guidance

This folder holds one **file test plan** per Rust source file in `packages/udp-server/src/`
(`<module>-tests.md`, for example `error-tests.md` or `handlers-announce-tests.md`). The workflow,
approval gates, commit points, section layout, and processing order are defined once in
[ISSUE.md — Per-File Workflow](../ISSUE.md#per-file-workflow) and the
[Source File Ledger](../ISSUE.md#source-file-ledger); do not duplicate them here. The
[coverage inventory](../coverage-evidence.md) holds the numbers and the T1 hypothesis for each file.

Create a plan right before starting its file, never in advance, so each plan learns from the
previous ones. A plan authorizes neither test changes nor work on another file until the
maintainer approves it.

## Plans

| Order | Plan | State | Result |
| ---: | --- | --- | --- |
| 1 | [error-tests.md](error-tests.md) | DONE | 2 tests refactored, 2 unit tests added; unit-only 83.87% → 100.00% lines |
| 2 | [lib-tests.md](lib-tests.md) | DONE | No code or test change; unit-only 96.55% lines |
| 3 | [container-tests.md](container-tests.md) | DONE | No code or test change; unit-only 100.00% lines |
| 4 | [event-tests.md](event-tests.md) | DONE | 1 test refactored, 4 unit tests added; unit-only 95.42% → 100.00% lines |
| 5 | [handlers-mod-tests.md](handlers-mod-tests.md) | DONE | 2 unit tests added; unit-only 85.98% → 96.40% lines |
| 6 | [handlers-connect-tests.md](handlers-connect-tests.md) | DONE | 3 tests refactored, 1 duplicate removed; unit-only 100.00% lines |
| 7 | [handlers-announce-tests.md](handlers-announce-tests.md) | DONE | 2 terminology corrections and 2 tests added; unit-only 97.63% → 99.78% lines |

Add a row when a plan is created; the remaining files and their order are in the ledger.

## Shared Guardrails

- Unit-only coverage is the primary metric. Integration tests require a documented reason that a
  real-loopback boundary is clearer or necessary.
- Do not add tests merely for coverage, reopen #2149's completed contracts, duplicate `udp-core`
  service behavior, or cross into lifecycle policy owned by #1488.
- Judge existing tests against the `write-unit-test` skill (naming, visible AAA, state-centred
  Arrange, one reason to fail, no production-derived expectations, no hidden Act or fixture
  coupling). List the concrete smells in the plan; "clean" is also a recorded conclusion.
- Use the nightly Rust toolchain for formatting evidence: `cargo +nightly fmt --all -- --check`.
- Record the toolchain/runtime with every validation result. Run `git diff --check` after each
  increment and `linter markdown` plus `linter cspell` after changing these records.
- Shared helpers require an approved cohesive responsibility; do not create generic fixtures for a
  one-file seam.

## Reconciliation

Before final verification, reconcile every plan state with the ledger, `coverage-evidence.md`, and
EPIC #1347. Do not cite branch SHAs as evidence until the implementation is merged; use unique
Conventional Commit subjects instead.
