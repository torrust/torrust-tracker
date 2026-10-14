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
| 1 | [services-announce-tests.md](services-announce-tests.md) | DONE | 8 unit tests added (R2h reopened); unit-only 0.00% → 98.48% lines |
| 2 | [peer-builder-tests.md](peer-builder-tests.md) | DONE | 5 unit tests added; unit-only 78.57% → 100.00% lines |
| 3 | [services-scrape-tests.md](services-scrape-tests.md) | DONE | 6 unit tests added; unit-only 0.00% → 96.70% lines |
| 4 | [connection-cookie-tests.md](connection-cookie-tests.md) | DONE | 8 tests refactored, 6 added (1 property); unit-only 96.30% → 100.00% lines |
| 5 | [crypto-keys-tests.md](crypto-keys-tests.md) | SUPERSEDED | File deleted by #2458; no Rust change made under this plan |
| 6 | [crypto-cookie-cipher-tests.md](crypto-cookie-cipher-tests.md) | DONE | 1 test refactored, 1 compile-time guarantee added; unit-only 100.00% lines |
| 7 | [services-connect-tests.md](services-connect-tests.md) | DONE | 5 tests replaced by 3 contract tests; 4 unused shared helpers removed; only getters uncovered |
| 8 | [services-banning-tests.md](services-banning-tests.md) | DONE | 4 tests refactored, 2 added (1 property); banned-IPs total handed off as a bug |
| 9 | [event-tests.md](event-tests.md) | DONE | 7 tests refactored around one fixture, 2 added; only the dead `server_socket_addr()` uncovered |
| 10 | [statistics-metrics-tests.md](statistics-metrics-tests.md) | DONE | 3 direct projection tests added; only the dead `set_gauge` uncovered |
| 11 | [statistics-event-handler-tests.md](statistics-event-handler-tests.md) | DONE | 6 per-family tests replaced by one `rstest` test; stale `# Panics` doc removed |
| 12 | [statistics-repository-tests.md](statistics-repository-tests.md) | DONE | 4 unit tests added (1 `rstest` with 2 cases); unit-only 80.77% → 100.00% lines; findings handed off to #1669 |
| 13 | [statistics-services-tests.md](statistics-services-tests.md) | DONE | 1 empty-state test replaced by 2 source tests; stale module docs fixed |
| 14 | [statistics-event-listener-tests.md](statistics-event-listener-tests.md) | DONE | 1 assertion sharpened, 1 `rstest` test added; lifecycle left to #1488 / #2414 |
| 15 | [statistics-mod-tests.md](statistics-mod-tests.md) | APPROVED | 1 contract test planned for the exported metric name |

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
