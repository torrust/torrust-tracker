---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/statistics/event/handler/error.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/statistics/event/handler/error.rs
    - packages/udp-server/src/statistics/event/handler/mod.rs
    - docs/issues/open/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Statistics Error Handler File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/statistics/event/handler/error.rs`.

## Current State

- **Fresh unit-only coverage:** 131 / 151 lines (86.75%) from the stable Rust toolchain report
  `cargo llvm-cov -p torrust-tracker-udp-server --all-features --lib --json`. No package source has
  changed since the row-27 report was generated.
- **Module-owned decisions:** route general errors to the labelled error counter and connection-cookie
  announce errors to the client-software counter.
- **Existing tests:** three deterministic tests cover IPv4 general errors, request-kind labelling, and
  QBitTorrent connection-cookie metrics.
- **T1 hypothesis:** probably no change. Existing tests cover observable metric routing; residual
  peer-client representation and logging-only failure paths do not select a new test.

## Current Tests Review

| Test group | Smells | Response |
| --- | --- | --- |
| General error metric test | Name lacks the `it_should_*` convention; Arrange, Act, and Assert are otherwise visible. | R1 rename only. |
| Labelled general and connection-cookie metric tests | Causal labels and independently specified expected counts remain visible. | Retain; R1 otherwise clean. |

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| General error and request-kind metric routing | 3A unit | Already covered by direct handler tests. |
| Announce connection-cookie client metric routing | 3A unit | Already covered by the QBitTorrent contract. |
| Peer-client name/version representation | Collaborator-owned | `torrust-peer-id` owns peer-client classification and representation. |
| Repository write failures | Collaborator-owned/logging-only | `Repository` and the metrics collection own failure semantics; this handler only logs them. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Renamed the nonconforming general-error test without changing its contract. | Focused error-handler tests passed 3 tests; nightly formatter and diff checks passed. Maintainer review is requested. |
| R2 | SKIPPED | Residual paths are repository-error logging and peer-client representation branches. | No distinct handler-owned metric-routing outcome is uncovered. |
| R3 | SKIPPED | No real UDP integration increment is selected. | The direct handler boundary covers observable metric routing. |
| R4 | DONE | Re-ran and recorded unit-only coverage. | 131 / 151 lines (86.75%), 16 / 16 functions, and 191 / 259 regions. |

## Results

Unit-only coverage remained **131 / 151 lines (86.75%)** before and after the R1 rename. No
additional Rust test or production-code change is appropriate.

## Completed-File Review

Approved by Jose Celano on 2026-09-29.

## Progress Log

- 2026-09-29 - GitHub Copilot - Created the plan from the source, focused metric-routing tests,
  and current coverage inventory. Maintainer approval is requested before the R1 test refactor.
- 2026-09-29 17:57 UTC - GitHub Copilot - Completed R1. The general-error metric test now follows
  the `it_should_*` convention; focused tests, nightly Rust formatting, and diff checks pass.
  Maintainer review is requested before R2.
- 2026-09-29 - Jose Celano - Approved the R1 refactor.
- 2026-09-29 17:58 UTC - GitHub Copilot - Completed R2-R4. Fresh unit-only coverage is 131 / 151
  lines (86.75%); residual repository-error logging and peer-client representation paths do not
  select a new test. Completed-file review is requested.
- 2026-09-29 17:59 UTC - Jose Celano - Approved the completed-file result.
