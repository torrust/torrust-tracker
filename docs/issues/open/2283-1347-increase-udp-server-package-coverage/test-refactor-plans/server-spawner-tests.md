---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/server/spawner.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/server/spawner.rs
    - packages/udp-server/src/server/states.rs
    - packages/udp-server/src/server/launcher.rs
    - docs/issues/closed/2149-1347-add-focused-udp-server-package-tests/test-refactor-plans/spawner-tests.md
    - docs/issues/open/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Spawner File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/server/spawner.rs`.

## Current State

- **Fresh unit-only coverage:** 17 / 17 lines, 2 / 2 functions, and 17 / 17 regions (100.00%)
  from `cargo llvm-cov -p torrust-tracker-udp-server --all-features --lib --json` (stable Rust
  toolchain).
- **Module-owned decisions:** none beyond capturing the bind address and spawning one Tokio task
  that delegates to `Launcher::run_with_graceful_shutdown`, mapping success back to a `Spawner`.
- **Existing tests:** none in this file. Coverage comes from the `server/mod.rs` start/stop
  contracts and `testing/environment.rs`.
- **T1 hypothesis:** no change. Confirmed.

## Current Tests Review

No colocated tests to review.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Task spawn and bind-address capture | Collaboration | Covered through the public `Server` start/stop contracts. A direct test would assert that `tokio::spawn` ran or that the address round-trips, which is structure, not behavior (#2149 P1 retained). |
| Launcher failure, halt, abort, cleanup | Lifecycle (#1488) | Owned by `Launcher` and #1488; forcing them through this wrapper needs a production-only launcher seam (#2149 P2 retained). |
| UDP socket transport | Integration | Not selected: covered by existing package contracts. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Recorded the no-change conclusion. | This plan. |
| R2 | SKIPPED | No new unit test: no distinct decision is unprotected. | Explicit rationale above. |
| R3 | SKIPPED | No integration increment. | Explicit rationale above. |
| R4 | DONE | Unit-only coverage recorded. No code change, so no re-measurement is needed. | 17 / 17 lines (100.00%). |

## Completed-File Review

Approved by Jose Celano on 2026-09-24.

## Progress Log

- 2026-09-24 - GitHub Copilot - Created the plan from `spawner.rs`, its callers, the #2149
  no-change decision, and fresh unit-only coverage. No Rust tests or production code changed.
- 2026-09-24 - Jose Celano - Approved the no-change conclusion as the completed-file result.
