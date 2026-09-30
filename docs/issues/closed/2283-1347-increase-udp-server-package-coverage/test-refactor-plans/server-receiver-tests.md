---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/server/receiver.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/server/receiver.rs
    - packages/udp-server/src/server/bound_socket.rs
    - docs/issues/closed/2149-1347-add-focused-udp-server-package-tests/test-refactor-plans/receiver-tests.md
    - docs/testing/refactoring-patterns/prose-first-arrange-act-assert-verification.md
    - docs/issues/closed/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Receiver File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/server/receiver.rs`.

## Current State

- **Fresh unit-only coverage:** 55 / 56 lines (98.21%), 7 / 7 functions, and 77 / 79 regions
  from `cargo llvm-cov -p torrust-tracker-udp-server --all-features --lib --json` (stable Rust
  toolchain).
- **Uncovered:** line 49, `Err(err) => Some(Err(err))`: a receive error on a live socket.
- **Module-owned decision:** adapt one received datagram into a `RawRequest` carrying the exact
  payload bytes and the sender address.
- **Existing tests:** one, added by #2149: a queued loopback datagram is yielded as the matching
  raw request.
- **T1 hypothesis:** no change. Confirmed.

## Current Tests Review

Prose-first review of `it_should_yield_a_raw_request_with_the_received_datagram_and_sender_address`:

- **Arrange:** "a receiver has one datagram with a known payload queued from a loopback client."
  The code says exactly this: `ReceiverWithQueuedLoopbackDatagram::new(vec![1, 2, 3])`. The
  fixture owns socket binding, client binding, and the pre-Act send; the causal payload is visible.
- **Act:** "the receiver yields its next request." The code is `scenario.receiver.next()`, wrapped
  in an absolute diagnostic deadline and three `expect`s (deadline, stream end, receive result).
- **Assert:** "the request carries that payload and the client's address." One whole-value
  `RawRequest` comparison against the fixture's expected request, whose payload is the Arrange
  input and whose address is the fixture's client (a fixture-derived incidental detail, derived
  from the same client used for the send).

This test is the repository's own example of the pattern in
[prose-first AAA verification](../../../../testing/refactoring-patterns/prose-first-arrange-act-assert-verification.md)
("a receiver test can name the coordinated state `ReceiverWithQueuedLoopbackDatagram`").

Considered and not selected: moving the deadline-and-`expect` chain into a `next_raw_request`
helper. It would shorten the Act by four lines but hide the production `next()` call, which the
skill keeps visible; unlike the launcher event wait, this chain is the Act, not assertion plumbing.
It would also diverge from the documented pattern example.

Conclusion: clean. No change.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Datagram to `RawRequest` adaptation | 3A unit | Covered by the existing test. |
| Receive error (line 49) | Not reachable | A live UDP socket has no portable receive-error injection; a mock socket would add hot-path indirection (#2149 P2 retained). |
| `Poll::Pending` | Not selected | Tokio readiness mechanics; the branch is covered but not asserted (#2149 P2 retained). |
| `bound_socket_address` | Not selected | Forwards to the directly tested `BoundSocket::address` (#2149 P3 retained). |
| Stream termination | Lifecycle (#1488) | Receive-loop termination belongs to #1488. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Recorded the no-change conclusion, including the prose-first review. | This plan. |
| R2 | SKIPPED | No new unit test: the only uncovered line needs socket fault injection. | Explicit rationale above. |
| R3 | SKIPPED | No integration increment. | Explicit rationale above. |
| R4 | DONE | Unit-only coverage recorded. No code change, so no re-measurement is needed. | 55 / 56 lines (98.21%). |

## Completed-File Review

Approved by Jose Celano on 2026-09-26.

## Progress Log

- 2026-09-25 - GitHub Copilot - Created the plan from `receiver.rs`, its test, the #2149 plan,
  the prose-first pattern document, and fresh unit-only coverage. No Rust tests or production code
  changed.
- 2026-09-26 - Jose Celano - Approved the no-change conclusion as the completed-file result.
