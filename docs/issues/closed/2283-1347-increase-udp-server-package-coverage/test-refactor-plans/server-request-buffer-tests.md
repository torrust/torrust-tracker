---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/server/request_buffer.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/server/request_buffer.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/docs/adrs/20260907152707_keep_oldest_first_udp_request_eviction.md
    - docs/issues/closed/2149-1347-add-focused-udp-server-package-tests/test-refactor-plans/request-buffer-tests.md
    - docs/issues/drafts/1488-si-15-define-udp-active-request-policy/ISSUE.md
    - docs/issues/closed/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Request Buffer File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/server/request_buffer.rs`.

## Current State

- **Fresh unit-only coverage:** 162 / 174 lines (93.10%), 25 / 26 functions, and 227 / 254
  regions from `cargo llvm-cov -p torrust-tracker-udp-server --all-features --lib --json` (stable
  Rust toolchain).
- **Module-owned decisions:** admit a task while capacity remains; on capacity exhaustion, reclaim
  finished handles encountered before the first active one or abort the oldest active task; abort
  remaining active tasks when the buffer drops.
- **Existing tests:** #2149 added deterministic contracts for capacity-available admission,
  all-pending oldest-task eviction, and drop cleanup.
- **T1 hypothesis:** lifecycle-owned no change. Refined: the normal-operation contracts are already
  protected; the uncovered paths are not an additional stable test seam.

## Current Tests Review

The three existing tests are clean:

- **Arrange:** `PendingTask` exposes the controlled pending state, while
  `FullBufferWithPendingTasks` names the only coordinated causal state. Direct ring-buffer
  insertion is confined to setup so `force_push` remains the visible production Act.
- **Act:** each test directly calls `force_push` or drops `ActiveRequests`.
- **Assert:** each checks its owned fact: no eviction, the selected oldest-task eviction while
  retaining the other active tasks, or pending-task abort on drop. Cleanup waits are absolute
  diagnostic bounds, not scheduling delays.

Conclusion: clean. No test refactor proposed.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Capacity-available admission | 3A unit | Covered by the existing pending-task test. |
| Full buffer of active tasks | 3A unit | Covered by the oldest-task eviction scenario. |
| Drop aborts active tasks | 3A unit | Covered by the mixed completed/pending buffer scenario. |
| Finished handles before an active handle | Policy-sensitive | The oldest-first ADR intentionally bounds traversal and does not specify a full reclamation contract. A scheduler-controlled test would risk defining rejected behavior. |
| Incoming task finishes during admission | Defensive race guard | `launcher.rs` avoids calling `force_push` for an already-finished handle; completion during admission has no stable external contract. |
| Debug logging and finished-handle drop skip | Not selected | Diagnostic or benign cleanup implementation details. |
| Drain, cancellation, and task lifecycle | Lifecycle (#1488 SI-15) | Active-request policy ownership remains outside #2283. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Record the prose-first review and no-change conclusion. | This plan. |
| R2 | DONE | Do not add a finished-handle or admission-race test. | Ownership rationale above. |
| R3 | DONE | Re-run unit-only coverage. | 162 / 174 lines, 25 / 26 functions, 227 / 254 regions. |

## Completed-File Review

Approved by Jose Celano on 2026-09-28. No Rust change is selected.

## Progress Log

- 2026-09-28 - GitHub Copilot - Created the plan from the current implementation and tests, the
  completed #2149 plan, the oldest-first eviction ADR, and fresh unit-only coverage. No Rust tests
  or production code changed.
