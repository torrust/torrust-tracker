---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/statistics/event/handler/request_received.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/statistics/event/handler/request_received.rs
    - packages/udp-server/src/statistics/event/handler/mod.rs
    - docs/issues/open/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Request-Received Statistics Handler File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/statistics/event/handler/request_received.rs`.

## Current State

- **Current unit-only coverage:** 33 / 34 lines (97.06%) from the refreshed stable Rust toolchain
  report in [coverage-evidence.md](../coverage-evidence.md).
- **Module-owned decision:** convert a received-request connection context into labels and route the
  increment to the UDP server requests-received counter.
- **Existing test:** one deterministic IPv4 received-request event test observes the incremented
  `udp4_requests_received_total` value.
- **T1 hypothesis:** probably no change. The observable metric route is covered; the residual
  repository write-error branch only logs a collaborator failure.

## Current Tests Review

| Test group | Smells | Response |
| --- | --- | --- |
| IPv4 received-request metric test | Name lacks the `it_should_*` convention; Arrange, Act, and Assert are otherwise visible. | R1 rename only. |

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Received-request counter and IPv4 labels | 3A unit | Already covered by the direct handler test. |
| Repository counter-write failure | Collaborator-owned/logging-only | `Repository` and the metrics collection own failure semantics; this handler only logs the failure. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Renamed the nonconforming existing test without changing its contract. | Focused request-received handler tests, nightly formatter, and diff checks pass. Maintainer review is requested. |
| R2 | SKIPPED | The residual path is repository counter-write failure logging. | No distinct handler-owned observable outcome is uncovered. |
| R3 | SKIPPED | No real UDP integration increment is selected. | The direct handler boundary covers observable metric routing. |
| R4 | DONE | Re-ran and recorded unit-only coverage. | 33 / 34 lines (97.06%), 4 / 4 functions, and 33 / 35 regions. |

## Results

Unit-only coverage remained **33 / 34 lines (97.06%)** before and after the R1 rename. No
additional Rust test or production-code change is appropriate.

## Completed-File Review

Approved by Jose Celano on 2026-09-29.

## Progress Log

- 2026-09-29 18:09 UTC - GitHub Copilot - Created the plan from the source, focused metric-routing
  test, and current coverage inventory. Maintainer approval is requested before the R1 test
  refactor.
- 2026-09-29 18:13 UTC - Jose Celano - Approved the row-30 plan.
- 2026-09-29 18:13 UTC - GitHub Copilot - Completed R1. The received-request metric test now
  follows the `it_should_*` convention. Maintainer review is requested before R2.
- 2026-09-29 18:17 UTC - Jose Celano - Approved the R1 refactor.
- 2026-09-29 18:17 UTC - GitHub Copilot - Completed R2-R4. Fresh unit-only coverage is 33 / 34
  lines (97.06%); the repository counter-write failure remains logging-only and does not select a
  new test. Completed-file review is requested.
- 2026-09-29 18:19 UTC - Jose Celano - Approved the completed-file result.
