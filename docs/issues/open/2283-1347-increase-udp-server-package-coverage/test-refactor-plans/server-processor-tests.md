---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/server/processor.rs
status: proposed
semantic-links:
  related-artifacts:
    - packages/udp-server/src/server/processor.rs
    - packages/udp-server/src/handlers/mod.rs
    - packages/udp-server/src/event.rs
    - packages/udp-server/src/statistics/event/handler/response_sent.rs
    - docs/issues/closed/2149-1347-add-focused-udp-server-package-tests/test-refactor-plans/processor-tests.md
    - docs/issues/open/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Processor File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/server/processor.rs`.

## Current State

- **Fresh unit-only coverage:** 72 / 85 lines (84.71%), 10 / 14 functions, and 95 / 103 regions
  from `cargo llvm-cov -p torrust-tracker-udp-server --all-features --lib --json` (stable Rust
  toolchain).
- **Uncovered:** all of `send_response` and `send_packet` (lines 104-179): no unit test processes a
  request that is answered. Line 284 is test-only.
- **Module-owned decisions:**
  1. Discard a source-port-zero request and publish `UdpRequestDiscarded` (defense in depth; the
     launcher applies the same guard first).
  2. Classify each sent response as `UdpResponseKind::Ok { req_kind }` or
     `UdpResponseKind::Error { .. }` and publish `UdpResponseSent` naming the client.
- **Existing tests:** one, from #2149: a port-zero request publishes a discard event.
- **T1 hypothesis:** no change. Falsified: decision 2 has no unit test, and review found a defect in
  it (below).

## Suspected Defect (Out of Scope)

For every `Response::Error`, `send_response` publishes `UdpResponseKind::Error { opt_req_kind:
None }` and discards the `opt_req_kind` it received. `handle_packet` returns `Some(req_kind)` when a
parsed request fails in its handler (for example an announce with an invalid cookie), and
`UdpResponseKind::Error` is documented as containing "the request kind if the request was parsed
successfully". Commit `27e2db4b` ("include req kind in UDP error response if it's known") passed
the kind down but hard-coded `None` here.

Impact today: none on metrics. The only consumer, `statistics/event/handler/response_sent.rs`,
ignores `opt_req_kind`. The published fact is still wrong against its documented contract.

Per the maintainer's direction, bugs are out of scope for #2283. An error-response classification
test is deferred: written now it would either fail or codify `None`.

## Current Tests Review

Prose-first review of `it_should_publish_a_discard_event_when_a_client_uses_port_zero`:

- **Arrange:** "a processor receives a parsable request from a client on port zero." Visible:
  `client_with_port_0` and `connect_request_from(..)`; setup is one named call.
- **Act:** `processor.process_request(..)`. Visible.
- **Assert:** "it publishes a discard fact naming that client." The code asserts only the variant
  (`matches!(.., Event::UdpRequestDiscarded { .. })`), so a wrong client in the context passes.

Smells:

| Smell | Proposed response |
| --- | --- |
| Partial assertion: the context (client, listener, instance) is not checked. | Assert the complete event, built from the visible client and the processor's own binding and instance. |
| Comment noise: a `/// Scenario:` doc block restates the name; section banner comments; a four-line doc on the setup tuple. | Keep one line each where the code cannot say it (why the payload is parsable); drop the rest. |
| `setup_processor_with_event_receiver` returns an anonymous tuple and binds `0.0.0.0:0`. | Return a small `ProcessorTestContext` (processor, events, and the expected-context values) and bind loopback, matching the other server tests. |

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Port-zero discard event | 3A unit | R1 complete the assertion. |
| `Ok` classification and `UdpResponseSent` | 3A unit | R2 add: a valid connect request from a real loopback client is answered, and the processor publishes `UdpResponseSent { kind: Ok { req_kind: Connect } }` naming that client. The processing time is measured, so the assertion matches the kind and context, not the duration. A loopback client socket gives a nonzero source port and a valid send target; the test does not read the reply. |
| `Error` classification | Deferred (defect) | Blocked by the suspected defect above. |
| Non-error response without a request kind (`code-review: should never happen`) | Not reachable | `handle_packet` returns a kind for every successful response. |
| Serialization and send failures, trace logging | Not selected | Serialization belongs to `udp-protocol`; send failure needs socket fault injection; logging is diagnostic (#2149 P3 retained). |
| No event sender | Not selected | No observable output. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | BLOCKED | Introduce `ProcessorTestContext`; complete the discard-event assertion; trim comments. | Waits for the bug fix; rebase first. |
| R2 | BLOCKED | Add `it_should_publish_a_response_sent_event_when_it_answers_a_connect_request`. | Waits for the bug fix; rebase first. |
| R3 | DONE | Drafted the bug spec in `.tmp/udp-error-response-request-kind-bug-spec.md` for a separate issue. | Draft exists; not committed. |
| R4 | BLOCKED | Re-run unit-only coverage and record before/after file results. | After R1 and R2. |

## Paused

Work on #2283 is paused until the error-response request-kind fix merges. Then rebase this branch
on `develop`, re-read `processor.rs` and its new regression tests, revise R1 and R2 against them
(the fix's tests may already cover the `Error` classification), and continue.

## Progress Log

- 2026-09-26 - GitHub Copilot - Created the plan from `processor.rs`, `handle_packet`, the
  `UdpResponseKind` contract, the response-sent handler, commit `27e2db4b`, the #2149 plan, and
  fresh unit-only coverage. Found the error-response request-kind defect. No Rust tests or
  production code changed.
- 2026-09-26 - Jose Celano - Approved R1-R4. The bug is handled as a separate issue by another
  agent; #2283 pauses until it merges, to avoid conflicts.
- 2026-09-26 - GitHub Copilot - Drafted the bug spec (R3). R1, R2, and R4 are blocked.
