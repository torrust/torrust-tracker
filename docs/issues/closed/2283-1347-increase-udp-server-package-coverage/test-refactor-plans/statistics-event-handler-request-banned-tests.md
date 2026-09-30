---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/statistics/event/handler/request_banned.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/statistics/event/handler/request_banned.rs
    - packages/udp-server/src/statistics/event/handler/mod.rs
    - docs/issues/closed/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Request-Banned Statistics Handler File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/statistics/event/handler/request_banned.rs`.

## Current State

- **Current unit-only coverage:** 56 / 57 lines (98.25%) from the refreshed stable Rust toolchain
  report in [coverage-evidence.md](../coverage-evidence.md).
- **Module-owned decision:** convert a banned-request connection context into labels and route the
  increment to the UDP server requests-banned counter.
- **Existing tests:** two deterministic tests use the same IPv4 banned-request event and assert the
  same `udp_requests_banned_total` result.
- **T1 hypothesis:** probably no change. The observable metric route is covered; the residual
  repository write-error branch only logs a collaborator failure.

## Current Tests Review

| Test group | Smells | Response |
| --- | --- | --- |
| Banned-request metric tests | The tests duplicate the same Arrange, Act, and assertion. Both names lack the `it_should_*` convention. | R1 retain one renamed test and delete the duplicate. |

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Banned-request counter and IPv4 labels | 3A unit | Already covered by one direct handler test. |
| Repository counter-write failure | Collaborator-owned/logging-only | `Repository` and the metrics collection own failure semantics; this handler only logs the failure. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Retained one renamed metric-routing test and removed its duplicate without changing the contract. | Focused request-banned handler tests, nightly formatter, and diff checks pass. Maintainer review is requested. |
| R2 | SKIPPED | The residual path is repository counter-write failure logging. | No distinct handler-owned observable outcome is uncovered. |
| R3 | SKIPPED | No real UDP integration increment is selected. | The direct handler boundary covers observable metric routing. |
| R4 | DONE | Re-ran and recorded unit-only coverage. | 33 / 34 lines (97.06%), 4 / 4 functions, and 33 / 35 regions. |

## Results

Unit-only coverage changed from **56 / 57 lines (98.25%)** to **33 / 34 lines (97.06%)** because
R1 deleted duplicated in-file test code. No additional Rust test or production-code change is
appropriate.

## Completed-File Review

Approved by Jose Celano on 2026-09-29.

## Progress Log

- 2026-09-29 18:59 UTC - GitHub Copilot - Created the plan from the source, focused metric-routing
  tests, and current coverage inventory. Maintainer approval is requested before the R1 refactor.
- 2026-09-29 19:01 UTC - Jose Celano - Approved the row-33 plan.
- 2026-09-29 19:01 UTC - GitHub Copilot - Completed R1. The duplicate banned-request metric test
  was removed and the retained contract follows the `it_should_*` convention. Maintainer review is
  requested before R2.
- 2026-09-29 19:06 UTC - Jose Celano - Approved the R1 refactor.
- 2026-09-29 19:06 UTC - GitHub Copilot - Completed R2-R4. Fresh unit-only coverage is 33 / 34
  lines (97.06%) after deleting duplicated test code; the repository counter-write failure remains
  logging-only and does not select a new test. Completed-file review is requested.
- 2026-09-29 19:06 UTC - Jose Celano - Approved the completed-file result.
