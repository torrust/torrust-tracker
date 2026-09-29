---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/statistics/event/handler/mod.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/statistics/event/handler/mod.rs
    - docs/issues/closed/2149-1347-add-focused-udp-server-package-tests/test-refactor-plans/statistics-event-dispatch-tests.md
    - docs/issues/open/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Statistics Event Dispatcher File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/statistics/event/handler/mod.rs`.

## Current State

- **Fresh unit-only coverage:** 21 / 21 lines (100.00%), 2 / 2 functions, and 50 / 52 regions
  from the stable Rust toolchain report `cargo llvm-cov -p torrust-tracker-udp-server --all-features
  --lib --json`. No package source has changed since the row-27 report was generated.
- **Module-owned decisions:** exhaustive delegation of each UDP `Event` variant, unchanged payload,
  repository, and timestamp to its specialized statistics handler.
- **Existing tests:** none colocated. Specialized handler tests already call this parent dispatcher
  where their metric-owned result provides an observable contract.
- **T1 hypothesis:** probably no change. The #2149 review documented that no strict direct routing
  seam exists without asserting collaborator side effects.

## Current Tests Review

| Test | Smells | Response |
| --- | --- | --- |
| No colocated tests | An exhaustive `match` prevents omitted variants at compile time; direct assertions would depend on specialized handler semantics. | R1 skipped. |

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Event-variant delegation | Collaboration | Specialized handler tests retain observable metric contracts; do not add a dispatch matrix. |
| Omitted event variant | Compiler-enforced | Rust exhaustiveness rejects an omitted arm. |
| `UdpError` delegation payload | Collaboration | A metric assertion would fail for error-handler or repository behavior, not only dispatcher routing. |
| Metric names, labels, aggregation, and error classification | Collaborator-owned | Specialized handlers, `metrics.rs`, and the metrics crate own these behaviors. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing test needs refactoring. | Current-tests review above. |
| R2 | SKIPPED | No package-owned unit contract has an independent observable seam. | #2149 dispatcher review and coverage analysis above. |
| R3 | SKIPPED | No real UDP integration increment is selected. | Integration cannot make a routing-only collaborator assertion more specific. |
| R4 | DONE | Recorded the current no-change result. | 21 / 21 lines (100.00%), 2 / 2 functions, and 50 / 52 regions. |

## Results

No Rust test or production-code change is appropriate.

## Completed-File Review

Approved by Jose Celano on 2026-09-29.

## Progress Log

- 2026-09-29 - GitHub Copilot - Created the plan from the current dispatcher, specialized-handler
  ownership, current coverage, and the completed #2149 dispatcher review. No Rust test or production
  code changed. Maintainer approval is requested.
- 2026-09-29 17:20 UTC - Jose Celano - Approved the no-change plan. Completed-file review is
  requested.
- 2026-09-29 17:23 UTC - Jose Celano - Approved the completed-file result.
