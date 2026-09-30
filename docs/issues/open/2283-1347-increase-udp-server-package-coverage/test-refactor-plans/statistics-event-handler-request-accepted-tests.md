---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/statistics/event/handler/request_accepted.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/statistics/event/handler/request_accepted.rs
    - packages/udp-server/src/statistics/event/handler/mod.rs
    - docs/issues/open/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Request-Accepted Statistics Handler File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/statistics/event/handler/request_accepted.rs`.

## Current State

- **Current unit-only coverage:** 162 / 163 lines (99.39%) from the refreshed stable Rust toolchain
  report in [coverage-evidence.md](../coverage-evidence.md).
- **Module-owned decision:** add the request kind to connection-context labels and route the accepted
  request to the UDP server requests-accepted counter.
- **Existing tests:** six deterministic contracts cover IPv4 and IPv6 accepted requests for connect,
  announce, and scrape kinds.
- **T1 hypothesis:** probably no change. Observable protocol-family and request-kind metric routing
  is covered; the residual repository counter-write failure only logs a collaborator failure.

## Current Tests Review

| Test group | Smells | Response |
| --- | --- | --- |
| IPv4 and IPv6 accepted-request metric tests | All six names lack the `it_should_*` convention; each explicit protocol-family and request-kind Arrange makes its metric route visible. | R1 rename only. |

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| IPv4 and IPv6 counter routing for connect, announce, and scrape | 3A unit | Already covered by six direct handler tests. |
| Repository counter-write failure | Collaborator-owned/logging-only | `Repository` and the metrics collection own failure semantics; this handler only logs the failure. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Renamed the six nonconforming existing tests without changing their contracts. | Focused request-accepted handler tests, nightly formatter, and diff checks pass. Maintainer review is requested. |
| R2 | SKIPPED | The residual path is repository counter-write failure logging. | No distinct handler-owned observable outcome is uncovered. |
| R3 | SKIPPED | No real UDP integration increment is selected. | The direct handler boundary covers observable metric routing. |
| R4 | DONE | Re-ran and recorded unit-only coverage. | 162 / 163 lines (99.39%), 14 / 14 functions, and 165 / 167 regions. |

## Results

Unit-only coverage remained **162 / 163 lines (99.39%)** before and after the R1 renames. No
additional Rust test or production-code change is appropriate.

## Completed-File Review

Approved by Jose Celano on 2026-09-29.

## Progress Log

- 2026-09-29 18:21 UTC - GitHub Copilot - Created the plan from the source, focused metric-routing
  tests, and current coverage inventory. Maintainer approval is requested before the R1 test
  refactor.
- 2026-09-29 18:38 UTC - Jose Celano - Approved the row-31 plan.
- 2026-09-29 18:38 UTC - GitHub Copilot - Completed R1. The accepted-request metric tests now
  follow the `it_should_*` convention. Maintainer review is requested before R2.
- 2026-09-29 18:45 UTC - Jose Celano - Approved the R1 refactor.
- 2026-09-29 18:45 UTC - GitHub Copilot - Completed R2-R4. Fresh unit-only coverage is 162 / 163
  lines (99.39%); the repository counter-write failure remains logging-only and does not select a
  new test. Completed-file review is requested.
- 2026-09-29 18:46 UTC - Jose Celano - Approved the completed-file result.
