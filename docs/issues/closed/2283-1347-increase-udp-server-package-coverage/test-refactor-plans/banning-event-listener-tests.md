---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/banning/event/listener.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/banning/event/listener.rs
    - packages/udp-server/src/banning/event/handler.rs
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/issues/closed/2342-1488-si-14-migrate-udp-receive-reset-token-lifecycle/ISSUE.md
    - docs/issues/closed/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Banning Event Listener File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/banning/event/listener.rs`.

## Current State

- **Fresh unit-only coverage:** 147 / 150 lines (98.00%), 21 / 21 functions, and 182 / 185 regions
  from the stable Rust toolchain report `cargo llvm-cov -p torrust-tracker-udp-server --all-features
  --lib --json`. No package source has changed since this report was generated for row 18.
- **Module-owned decisions:** `dispatch_events` prioritizes cancellation, stops on receiver closure,
  continues after lag, and forwards received events to the banning handler.
- **Existing tests:** four direct deterministic scripted-receiver tests cover closure, lag continuation,
  pre-cancelled-token priority, and non-cookie-event forwarding.
- **T1 hypothesis:** lifecycle (#1488). The current tests and coverage support a no-change
  conclusion, approved by the maintainer.

## Current Tests Review

| Test | Smells | Response |
| --- | --- | --- |
| `it_should_stop_when_the_receiver_is_closed` | Clean: closed receiver is visible, direct dispatch Act, one receive-count assertion. | R1 no change. |
| `it_should_process_connection_cookie_errors_after_lagging` | Clean: scripted lag, event, and closure state is visible; one assertion proves dispatch continues after lag. | R1 no change. |
| `it_should_prioritize_cancellation_over_a_ready_event` | Clean: a pre-cancelled token and ready event make the `biased` choice visible; one assertion proves the event was not dispatched. | R1 no change. |
| `it_should_ignore_non_connection_cookie_errors` | Clean: direct non-cookie event, direct dispatch Act, one observable banning result. | R1 no change. |

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Closure, lag continuation, cancellation priority, and event forwarding | Unit / lifecycle | Existing deterministic `dispatch_events` contracts retain the listener's current behavior. |
| `run_event_listener_unspawned` forwarding boundary (line 42) | Representation-only / lifecycle (#1488) | No direct test: it only logs then delegates; a forwarding assertion would not add a maintained contract. |
| Spawned listener completion log (line 29) | Lifecycle (#1488) | No task/join test: task ownership, cancellation, joining, and drop cleanup are governed by #1488 and its UDP lifecycle work. |
| Handler classification and `BanService` policy | Collaborator-owned | Owned by `banning/event/handler.rs` and `udp-core` respectively. |

## Lifecycle Boundary

No lifecycle test change is selected. Existing tests use immediate scripted receives and require no
deadline. A new test of the spawned boundary would need to define normal, cancellation, join, and
drop ownership with an absolute deadline; that work remains with #1488.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Confirmed current tests require no refactor. | Focused listener tests pass; current-tests review above. |
| R2 | SKIPPED | No package-owned unit contract remains unprotected. | Coverage analysis above. |
| R3 | SKIPPED | No real UDP boundary contract is selected. | Listener lifecycle remains #1488-owned. |
| R4 | DONE | Recorded the completed no-change result. No code change, so no re-measurement is needed. | Current unit-only result is 147 / 150 lines (98.00%). |

## Results

Unit-only coverage remained **147 / 150 lines (98.00%)** before and after review. No Rust test or
production-code change was appropriate.

## Completed-File Review

Approved by Jose Celano on 2026-09-29.

## Progress Log

- 2026-09-29 - GitHub Copilot - Created the plan from the listener, its four deterministic tests,
  the current unit-only coverage report, and #1488 SI-14 lifecycle evidence. #2149 has no
  listener-specific plan; its handler plan explicitly retained listener reception and lifecycle at
  this module's boundary. No Rust tests or production code changed.
- 2026-09-29 - Jose Celano - Approved the plan's no-change conclusion.
- 2026-09-29 - GitHub Copilot - Completed R1 after
  `cargo test -p torrust-tracker-udp-server banning::event::listener::tests` passed four tests on
  the stable Rust toolchain. R2 and R3 are skipped; R4 reuses the current unit-only report because
  no package source changed. Completed-file review is requested.
- 2026-09-29 - Jose Celano - Approved the completed-file result.
