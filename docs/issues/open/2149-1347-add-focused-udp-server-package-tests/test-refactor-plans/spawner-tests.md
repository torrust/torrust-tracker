---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/server/spawner.rs
status: completed
semantic-links:
  related-artifacts:
    - packages/udp-server/src/server/spawner.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/src/server/states.rs
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/ISSUE.md
---

# UDP Spawner Test Assessment Plan

Follow the shared [purpose, quality goals, plan structure, and required two-phase
sequence](README.md). This assessment applies only to `packages/udp-server/src/server/spawner.rs`.

## Phase 1 - Clean Current Tests

### Current state

`Spawner::spawn_launcher` captures its configured bind address, starts a Tokio task, and delegates
all server behavior to `Launcher::run_with_graceful_shutdown`. Its only result adaptation maps a
successful launcher completion back to the captured `Spawner`. The file has no colocated tests;
clean baseline evidence already reports 17/17 lines, 17/17 regions, and 2/2 functions covered.

### Decision

Do not add direct tests. A success-path test would recreate launcher inputs and assert a
representation-level `Spawner` result already protected by `server/states.rs`. Failure,
cancellation, and task completion behavior belongs to `Launcher` and #1488 lifecycle work. A mock
launcher or injected task factory would be production-only indirection with no distinct observable
contract.

## Phase 2 - Assess Missing Behavior Tests

### Strengths to preserve

1. `Spawner` owns only task creation and bind-address capture.
2. `Launcher` owns server startup, receive-loop work, task management, and graceful shutdown.
3. `server/states.rs` owns startup notification and task-result state mapping.
4. #1488 owns cancellation, joining, and lifecycle policy.

### Problems and opportunities

#### P1 - No distinct deterministic spawner behavior remains unprotected

**Decision.** No change. The wrapper's full existing coverage comes from the real server-state path.
Adding a test merely to assert `tokio::spawn` was invoked or the successful result returns the
same address would test implementation structure or duplicate server-state behavior.

#### P2 - Lifecycle branches must not be forced through a thin wrapper

**Decision.** Do not test launcher failure, halted signals, task aborts, cancellation, or resource
cleanup here. Such tests would need a controllable launcher/task seam and would preempt #1488's
pending ownership and lifecycle design.

## Proposed Refactorings

### R1 - Record fully-covered thin-wrapper and lifecycle deferral

- **Status:** DONE
- **Priority:** Low impact / low effort
- **Change:** Document the no-test decision and ownership boundary.
- **Decision:** The source is fully covered, no distinct observable contract is missing, and test
  injection would add unjustified production abstraction. Retain the existing indirect coverage.
- **Done when:** The plan records why no test is selected.

## Progress Tracking

### Plan Checklist

- [x] Spawner, launcher, server-state, and lifecycle ownership boundaries reviewed.
- [x] No-change decision recorded.
- [x] Plan completed and ready for final verification.

### Progress Log

- 2026-09-11 - GitHub Copilot - Completed this no-test assessment after confirming that the thin
  wrapper is fully covered through server-state paths and has no distinct deterministic observable
  contract. Launcher lifecycle behavior remains owned by #1488.

### Validation Evidence

| Increment | Status | Evidence |
| --- | --- | --- |
| R1 | DONE | Source review and existing clean baseline show 17/17 lines, 17/17 regions, and 2/2 functions covered. No code change is selected. |

## Non-Goals

- Do not change spawner, launcher, server-state, task, signal, or shutdown production behavior.
- Do not add mock launchers, task factories, Tokio spawn interaction tests, or lifecycle tests.

## Completion Criteria

- The fully covered thin-wrapper decision is documented.
- Lifecycle behavior remains at the launcher/server-state and #1488 boundaries.
