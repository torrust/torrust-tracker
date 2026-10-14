---
doc-type: file-test-plan
issue: 1348
package: torrust-tracker-udp-core
target-file: packages/udp-core/src/statistics/mod.rs
status: approved
semantic-links:
  related-artifacts:
    - packages/udp-core/src/statistics/mod.rs
    - packages/udp-core/src/statistics/repository.rs
    - packages/udp-core/src/statistics/event/handler.rs
    - docs/issues/open/1348-1347-add-tests-udp-core/coverage-evidence.md
---

# UDP Core Statistics Module File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-core/src/statistics/mod.rs` (ledger order 15).

## Current State

- **Unit-only coverage:** 8 / 8 lines (100.00%), 10 / 10 regions, 1 / 1 function. Unchanged since
  T1.
- **Existing tests:** none in this file. `describe_metrics()` runs through every repository and
  metrics test; the repository test S1 checks that the counter is described.
- **Module-owned decisions:** the exported metric declaration: the counter name
  `udp_tracker_core_requests_received_total`, its unit (`Count`), and its description.
- **External contract:** the name is what Prometheus scrapes from the REST `/api/v1/metrics`
  endpoint, so dashboards and alerts depend on it. The label names are already pinned by literals
  in the handler tests (`request_kind`, `server_binding_*`).
- **Gap:** every test reaches the name through the `UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL`
  constant, so renaming the string passes all tests.
- **T1 hypothesis:** probably no change.

## Current Tests Review

No tests in this file. R1 is skipped.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Exported metric name and type | 3A unit | D1: `describe_metrics()` declares a counter named by the literal `"udp_tracker_core_requests_received_total"`. A renamed string, or the metric declared as a gauge, fails it. |
| Unit and description text | Not tested | Presentation metadata; pinning the description text would make wording fixes break a test without protecting a consumer. |
| Property-based testing | Not selected | One fixed declaration. |

Line coverage is already 100%; D1 adds a contract check, not coverage.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests to refactor. | Recorded above. |
| R2 | PENDING | Add D1. | - |
| R3 | SKIPPED | No integration increment: a REST metrics contract test belongs to the REST API packages. | Explicit rationale. |
| R4 | PENDING | Re-run unit-only coverage; record before/after. | - |

## Results

Pending.

## Approval

Approved by Jose Celano on 2026-10-14: D1.

## Progress Log

- 2026-10-14 10:52 UTC - GitHub Copilot - Created the plan from `statistics/mod.rs`, the tests that
  reach it through the name constant, and the literal label names in the handler tests. No Rust
  tests or production code changed. R2 awaits maintainer approval.
- 2026-10-14 10:55 UTC - Jose Celano - Approved D1.
