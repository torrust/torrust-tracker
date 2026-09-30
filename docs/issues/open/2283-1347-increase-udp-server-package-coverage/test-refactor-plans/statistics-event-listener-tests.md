---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/statistics/event/listener.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/statistics/event/listener.rs
    - packages/udp-server/src/statistics/event/handler/mod.rs
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/issues/open/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Statistics Event Listener File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/statistics/event/listener.rs`.

## Current State

- **Fresh unit-only coverage:** 164 / 172 lines (95.35%) from the stable Rust toolchain report
  `cargo llvm-cov -p torrust-tracker-udp-server --all-features --lib --json`. No package source has
  changed since the row-25 report was generated.
- **Module-owned decisions:** receive-loop dispatch, metrics-policy filtering, closed/lagged
  receiver handling, and cancellation priority.
- **Existing tests:** four deterministic tests invoke `dispatch_events` with a scripted receiver to
  cover enabled/disabled policy, closed receiver completion, lag recovery, and cancellation before a
  ready event.
- **T1 hypothesis:** lifecycle owned by #1488. The direct dispatch tests cover the module's stable
  decision seam; spawned task lifecycle remains deferred.

## Current Tests Review

| Test group | Smells | Response |
| --- | --- | --- |
| Scripted receiver dispatch tests | State-centred arrangements, visible `dispatch_events` act, and typed metric/receive assertions. | Retain; R1 initially clean. |
| Cancellation-priority test | Deterministically cancels before the act and confirms the queued event is not processed. | Retain; direct loop-selection contract, not spawned lifecycle. |

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Metrics-policy filtering, closure, lag recovery, and cancellation priority | 3A unit | Already covered by direct deterministic `dispatch_events` tests. |
| Spawned task completion, joining, drop cleanup, and shutdown propagation | Lifecycle (#1488) | `run_event_listener` task ownership and coordinated cancellation remain #1488-owned. |
| Event-to-metric update behavior | Collaborator-owned | `handler/mod.rs` and specialized handlers own event routing and metric changes. |
| Receiver implementation semantics | Collaborator-owned | The events crate owns receiver mechanics; this module tests the receive-loop response. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Reviewed the current direct dispatch tests; no concrete readability defect justified a refactor. | `cargo test -p torrust-tracker-udp-server statistics::event::listener::tests` passed 4 tests. |
| R2 | SKIPPED | The residual paths are spawned/wrapper lifecycle code and exhaustive event-context accessor arms. | Task ownership is #1488-owned; accessor-table tests would repeat event representation without a new listener decision. |
| R3 | SKIPPED | No real UDP integration increment is selected. | Direct receive-loop policy is deterministic at the unit boundary; spawned lifecycle remains #1488-owned. |
| R4 | DONE | Re-ran and recorded unit-only coverage. | 164 / 172 lines (95.35%), 23 / 23 functions, and 210 / 219 regions. |

## Results

Unit-only coverage remained **164 / 172 lines (95.35%)** before and after review. The current
direct dispatch tests are clean, and no Rust test or production-code change is appropriate.

## Completed-File Review

Approved by Jose Celano on 2026-09-29.

## Progress Log

- 2026-09-29 - GitHub Copilot - Created the plan from the source, current scripted-receiver tests,
  coverage inventory, and #1488 lifecycle ownership. Maintainer approval is requested before the
  current-test review or any change.
- 2026-09-29 17:08 UTC - GitHub Copilot - Completed R1-R4. Focused listener tests passed 4 tests
  on the stable Rust toolchain. Fresh unit-only coverage is 164 / 172 lines (95.35%); residual
  spawned lifecycle and event-context representation paths do not select a new test. Completed-file
  review is requested.
- 2026-09-29 17:13 UTC - Jose Celano - Approved the completed-file result.
