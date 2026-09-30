---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/server/states.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/server/states.rs
    - packages/udp-server/src/server/mod.rs
    - packages/udp-server/src/server/bound_socket.rs
    - docs/issues/closed/2149-1347-add-focused-udp-server-package-tests/test-refactor-plans/server-states-tests.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/issues/open/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server States File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/server/states.rs`.

## Current State

- **Fresh unit-only coverage:** 72 / 77 lines (93.51%), 16 / 20 functions, and 91 / 102 regions
  from `cargo llvm-cov -p torrust-tracker-udp-server --all-features --lib --json` (stable Rust
  toolchain).
- **Module-owned decisions:** map a closed startup-notification channel according to the launcher's
  completed, failed, or aborted task outcome; compose startup and stop lifecycle transitions.
- **Existing tests:** one focused test for each deterministic `await_startup_notification` mapping:
  launcher I/O failure, successful launcher completion, and aborted launcher task.
- **T1 hypothesis:** lifecycle-owned no change. Confirmed: the remaining paths are not another
  deterministic state-helper contract.

## Current Tests Review

The three `await_startup_notification` tests are clean:

- **Arrange:** each keeps the closed startup sender and the task outcome visible. The successful
  launcher helper hides only repeated incidental `Spawner` construction.
- **Act:** each directly awaits `await_startup_notification`.
- **Assert:** each checks exactly one `UdpError` mapping; the I/O-failure test additionally proves
  the `BrokenPipe` kind survives the mapping.

Conclusion: clean. No test refactor proposed.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Closed-notification error mapping | 3A unit | Covered by the three existing tests. |
| Bind-error conversion | Public socket boundary | `BoundSocket` owns binding; `server/mod.rs` owns registration failure and listener-release coverage. |
| Registration failure cleanup | Collaboration | Covered by the public `server/mod.rs` transition test; do not duplicate it here. |
| `Running::stop`, halt signalling, and launcher joining | Lifecycle (#1488) | Shutdown policy is owned by #1488; a new test would define legacy lifecycle behavior. |
| Derived aliases, constructors, and display | Not selected | Representation-only behavior. |
| Defensive test fallback | Not selected | A test-only impossible arm is not a production contract. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Record the prose-first review and no-change conclusion. | This plan. |
| R2 | DONE | Do not add bind, registration, stop, or representation tests. | Ownership rationale above. |
| R3 | DONE | Re-run unit-only coverage. | 72 / 77 lines, 16 / 20 functions, 91 / 102 regions. |

## Completed-File Review

Approved by Jose Celano on 2026-09-28. No Rust change is selected.

## Progress Log

- 2026-09-28 - GitHub Copilot - Created the plan from the current implementation and tests, the
  completed #2149 plan, #1488 lifecycle ownership, and fresh unit-only coverage. No Rust tests or
  production code changed.
