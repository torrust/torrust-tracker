---
semantic-links:
  related-artifacts:
    - docs/issues/open/2274-1488-si-10-add-token-aware-axum-drain-helper/ISSUE.md
    - packages/axum-server/examples/token_aware_drain.rs
    - packages/axum-server/src/signals.rs
---

# Implementation Retrospective - Issue #2274

## Outcome

The additive helper provides token-driven Axum draining with caller-owned task
lifecycle and typed `Drained` or `TimedOut` observation. The retained local
server example manually verifies both outcomes, while focused deterministic
tests verify the deadline behavior without OS signals.

## What Went Well

1. The helper's typed timeout outcome kept escalation policy in future server
   component migrations rather than adding an unowned or forceful shutdown path.
2. A real loopback server verifier exposed a transport-level condition that a
   handle-only test could not show.

## What Changed During Implementation

The first real-server drain scenario released its request handler but left the
HTTP response unread. Axum correctly retained that connection and the helper
returned `TimedOut`. The retained verifier now sends `Connection: close` and
reads the response before awaiting `Drained`.

## Root Cause

The verification plan treated handler completion as equivalent to TCP
connection drain. A graceful server drain is complete only after the request
and connection lifecycle complete; the client side must participate in that
observable condition.

## Improvements for Future Work

1. Real-server shutdown scenarios should state both the application handler
   transition and the client connection transition required for drain.
2. Keep deadline-sensitive collaboration tests below the status-polling
   interval, as the paused-time helper test does, so polling regressions remain
   observable.

## Avoiding Overcorrection

This does not justify changing production drain behavior, adding a forced
timeout to the helper, or requiring every unit test to read network responses.
The requirement is limited to real connection-drain verification.

## Evidence

- [Issue #2274 specification](ISSUE.md)
- [Manual verification evidence](manual-verification-evidence.md)
- [Focused verification evidence](verification.md)
- [Retained verifier example](../../../../packages/axum-server/examples/token_aware_drain.rs)
