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
| 8 | [handlers-scrape-tests.md](handlers-scrape-tests.md) | DONE | 3 test cleanups and 1 test added; unit-only 99.38% → 99.72% lines; post-#2320 fixture refactor and 3 more tests |
| 9 | [handlers-error-tests.md](handlers-error-tests.md) | DONE | 3 tests refactored, 2 classifier tests added; unit-only 87.08% → 100.00% lines |
| 10 | [server-mod-tests.md](server-mod-tests.md) | DONE | 1 Tokio-owned test deleted; unit-only 99.44% → 99.28% lines (smaller denominator) |
| 11 | [server-bound-socket-tests.md](server-bound-socket-tests.md) | DONE | 2 unit tests added; unit-only 84.62% → 89.53% lines |
| 12 | [server-spawner-tests.md](server-spawner-tests.md) | DONE | No code or test change; unit-only 100.00% lines |
| 13 | [server-launcher-tests.md](server-launcher-tests.md) | DONE | 2 admit-path unit tests added; unit-only 95.86% → 96.98% lines; post-commit readability refactor and 1 test split |
| 14 | [server-receiver-tests.md](server-receiver-tests.md) | DONE | No code or test change; unit-only 98.21% lines |
| 15 | [server-processor-tests.md](server-processor-tests.md) | DONE | Discard assertion completed; connect response event covered; #2354 covers error classification; unit-only 100.00% lines |
| 16 | [server-request-buffer-tests.md](server-request-buffer-tests.md) | DONE | No Rust change; current admission, eviction, and drop contracts remain sufficient; unit-only 93.10% lines |
| 17 | [server-states-tests.md](server-states-tests.md) | DONE | No Rust change; startup-notification mappings are complete and lifecycle behavior remains #1488-owned; unit-only 93.51% lines |
| 18 | [banning-mod-tests.md](banning-mod-tests.md) | DONE | No Rust change; namespace wiring has no executable entries |
| 19 | [banning-event-mod-tests.md](banning-event-mod-tests.md) | DONE | No Rust change; module declarations have no executable entries |
| 20 | [banning-event-handler-tests.md](banning-event-handler-tests.md) | DONE | No Rust change; current focused contracts cover the handler's orchestration |
| 21 | [banning-event-listener-tests.md](banning-event-listener-tests.md) | DONE | No Rust change; direct dispatch contracts retained and spawned lifecycle deferred to #1488 |
| 22 | [statistics-mod-tests.md](statistics-mod-tests.md) | DONE | No Rust change; metric composition remains fully covered through repository initialization |
| 23 | [statistics-metrics-tests.md](statistics-metrics-tests.md) | DONE | No Rust change; metric projections and moving-average behavior remain extensively covered |
| 24 | [statistics-repository-tests.md](statistics-repository-tests.md) | DONE | Refactored repository tests and added initialized collection snapshot contracts |
| 25 | [statistics-services-tests.md](statistics-services-tests.md) | DONE | Refactored the fully covered service aggregation test |
| 26 | [statistics-event-mod-tests.md](statistics-event-mod-tests.md) | DONE | No Rust change; declaration wiring has no independent observable contract |
| 27 | [statistics-event-listener-tests.md](statistics-event-listener-tests.md) | DONE | No Rust change; direct dispatch contracts retained and spawned lifecycle deferred to #1488 |
| 28 | [statistics-event-handler-mod-tests.md](statistics-event-handler-mod-tests.md) | DONE | No Rust change; routing-only dispatcher has no independent observable seam |
| 29 | [statistics-event-handler-error-tests.md](statistics-event-handler-error-tests.md) | DONE | Renamed the general-error test; focused metric routing remains sufficient |
| 30 | [statistics-event-handler-request-received-tests.md](statistics-event-handler-request-received-tests.md) | DONE | Renamed the received-request test; focused metric routing remains sufficient |
| 31 | [statistics-event-handler-request-accepted-tests.md](statistics-event-handler-request-accepted-tests.md) | DONE | Renamed six accepted-request tests; focused metric routing remains sufficient |
| 32 | [statistics-event-handler-request-discarded-tests.md](statistics-event-handler-request-discarded-tests.md) | DONE | Retained the clean discarded-request metric routing test |
| 33 | [statistics-event-handler-request-banned-tests.md](statistics-event-handler-request-banned-tests.md) | DONE | Removed one duplicate test and renamed the retained banned-request metric contract |
| 34 | [statistics-event-handler-request-aborted-tests.md](statistics-event-handler-request-aborted-tests.md) | DONE | Removed one duplicate test and renamed the retained aborted-request metric contract |
| 35 | [statistics-event-handler-response-sent-tests.md](statistics-event-handler-response-sent-tests.md) | DONE | Renamed two response-counter tests; focused response metrics remain sufficient |
| 36 | [testing-mod-tests.md](testing-mod-tests.md) | DONE | No Rust change; module declaration has no independent runtime contract |
| 37 | [testing-environment-tests.md](testing-environment-tests.md) | DONE | 2 direct policy mapping tests added; lifecycle orchestration remains #1488-owned |

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
