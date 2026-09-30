---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/statistics/event/handler/request_discarded.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/statistics/event/handler/request_discarded.rs
    - packages/udp-server/src/statistics/event/handler/mod.rs
    - docs/issues/open/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Request-Discarded Statistics Handler File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/statistics/event/handler/request_discarded.rs`.

## Current State

- **Current unit-only coverage:** 33 / 34 lines (97.06%) from the refreshed stable Rust toolchain
  report in [coverage-evidence.md](../coverage-evidence.md).
- **Module-owned decision:** convert a discarded-request connection context into labels and route
  the increment to the UDP server requests-discarded counter.
- **Existing test:** one deterministic IPv4 discarded-request event test observes the incremented
  `udp_requests_discarded_total` value.
- **T1 hypothesis:** probably no change. The observable metric route is covered; the residual
  repository write-error branch only logs a collaborator failure.

## Current Tests Review

| Test group | Smells | Response |
| --- | --- | --- |
| Discarded-request metric test | The `it_should_*` name, causal port-zero context, visible Act, and counter assertion are clean. | Retain without change. |

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Discarded-request counter and IPv4 labels | 3A unit | Already covered by the direct handler test. |
| Repository counter-write failure | Collaborator-owned/logging-only | `Repository` and the metrics collection own failure semantics; this handler only logs the failure. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Reviewed the existing test against the shared guidance and retained it without change. | Focused request-discarded handler tests, nightly formatter, and diff checks pass. Maintainer review is requested. |
| R2 | SKIPPED | The residual path is repository counter-write failure logging. | No distinct handler-owned observable outcome is uncovered. |
| R3 | SKIPPED | No real UDP integration increment is selected. | The direct handler boundary covers observable metric routing. |
| R4 | DONE | Re-ran and recorded unit-only coverage. | 33 / 34 lines (97.06%), 4 / 4 functions, and 33 / 35 regions. |

## Results

Unit-only coverage remained **33 / 34 lines (97.06%)** before and after the R1 review. No
additional Rust test or production-code change is appropriate.

## Completed-File Review

Approved by Jose Celano on 2026-09-29.

## Progress Log

- 2026-09-29 18:48 UTC - GitHub Copilot - Created the plan from the source, focused metric-routing
  test, and current coverage inventory. Maintainer approval is requested before the R1 review.
- 2026-09-29 18:54 UTC - Jose Celano - Approved the row-32 plan.
- 2026-09-29 18:54 UTC - GitHub Copilot - Completed R1. The existing discarded-request metric
  test is clean and remains unchanged. Maintainer review is requested before R2.
- 2026-09-29 18:55 UTC - Jose Celano - Approved the R1 review.
- 2026-09-29 18:55 UTC - GitHub Copilot - Completed R2-R4. Fresh unit-only coverage is 33 / 34
  lines (97.06%); the repository counter-write failure remains logging-only and does not select a
  new test. Completed-file review is requested.
- 2026-09-29 18:57 UTC - Jose Celano - Approved the completed-file result.
