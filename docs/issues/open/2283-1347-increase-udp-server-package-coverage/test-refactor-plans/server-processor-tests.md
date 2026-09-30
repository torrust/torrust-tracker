---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/server/processor.rs
status: done
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

- **Latest unit-only coverage:** 180 / 180 lines (100.00%), 25 / 25 functions, and 228 / 228
  regions from `cargo llvm-cov -p torrust-tracker-udp-server --all-features --lib --json` (stable
  Rust toolchain).
- **Module-owned decisions:**
  1. Discard a source-port-zero request and publish `UdpRequestDiscarded` (defense in depth; the
     launcher applies the same guard first).
  2. Classify each sent response as `UdpResponseKind::Ok { req_kind }` or
     `UdpResponseKind::Error { .. }` and publish `UdpResponseSent` naming the client.
- **Tests:** the port-zero discard event, successful connect response publication, and the parsed
  and unparsable error-response classifications.
- **T1 hypothesis:** no change. Falsified: decision 2 needed its successful-response contract.

## Independently Fixed Defect

The independently delivered #2345 fix, merged through PR #2354, preserves `opt_req_kind` for
`Response::Error`. Its two regression tests cover a parsed failing scrape request (`Some(Scrape)`)
and an unparsable payload (`None`). This coverage-review branch does not duplicate those tests.

The bug was out of scope for #2283, so this file was paused until the fix merged and this branch was
rebased onto `develop`.

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
| Partial assertion: the context (client, listener, instance) is not checked. | Resolved: assert the complete discard event, derived from the visible client and the processor's own binding and instance. |
| Comment noise: a `/// Scenario:` doc block restates the name; section banner comments; a four-line doc on the setup tuple. | Keep one line each where the code cannot say it (why the payload is parsable); drop the rest. |
| `setup_processor_with_event_receiver` returns an anonymous tuple and binds `0.0.0.0:0`. | Retained: the tuple stays focused on its two test-facing resources; expected context is derived in each test, and the server binding is incidental. |

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Port-zero discard event | 3A unit | R1 completed the full event assertion. |
| `Ok` classification and `UdpResponseSent` | 3A unit | R2 adds a valid connect request from a real loopback client and asserts the published context plus `Ok { Connect }`; elapsed time is intentionally not asserted. |
| `Error` classification | Independent bug regression | #2354 covers parsed and unparsable errors. |
| Non-error response without a request kind (`code-review: should never happen`) | Not reachable | `handle_packet` returns a kind for every successful response. |
| Serialization and send failures, trace logging | Not selected | Serialization belongs to `udp-protocol`; send failure needs socket fault injection; logging is diagnostic (#2149 P3 retained). |
| No event sender | Not selected | No observable output. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Complete the discard-event assertion; retain focused helper setup. | Asserts `UdpRequestDiscarded` with the full expected context. |
| R2 | DONE | Add `it_should_publish_a_response_sent_event_when_it_answers_a_connect_request`. | Asserts context and `Ok { Connect }` through a real loopback client. |
| R3 | DONE | Hand off the error-response request-kind bug. | Fixed independently by #2345 / PR #2354. |
| R4 | DONE | Re-run unit-only coverage and record resulting file coverage. | 180 / 180 lines, 25 / 25 functions, 228 / 228 regions. |

## Completion

The branch rebased cleanly onto `torrust/develop` after PR #2354 merged. The four focused processor
tests and the unit-only package library suite pass.

## Progress Log

- 2026-09-26 - GitHub Copilot - Created the plan from `processor.rs`, `handle_packet`, the
  `UdpResponseKind` contract, the response-sent handler, commit `27e2db4b`, the #2149 plan, and
  fresh unit-only coverage. Found the error-response request-kind defect. No Rust tests or
  production code changed.
- 2026-09-26 - Jose Celano - Approved R1-R4. The bug is handled as a separate issue by another
  agent; #2283 pauses until it merges, to avoid conflicts.
- 2026-09-26 - GitHub Copilot - Drafted the bug spec (R3). R1, R2, and R4 are blocked.
- 2026-09-28 - GitHub Copilot - Rebasing onto `torrust/develop` incorporated #2354's two
  error-response regressions. Completed R1 and R2, then measured 100.00% unit-only file coverage.
