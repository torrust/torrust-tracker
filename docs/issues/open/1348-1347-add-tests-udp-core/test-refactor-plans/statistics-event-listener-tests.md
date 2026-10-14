---
doc-type: file-test-plan
issue: 1348
package: torrust-tracker-udp-core
target-file: packages/udp-core/src/statistics/event/listener.rs
status: approved
semantic-links:
  related-artifacts:
    - packages/udp-core/src/statistics/event/listener.rs
    - packages/udp-core/src/statistics/event/handler.rs
    - docs/issues/open/2414-2410-si-22-2-drain-listener-queues-on-shutdown/ISSUE.md
    - docs/issues/open/1348-1347-add-tests-udp-core/coverage-evidence.md
---

# UDP Core Statistics Event Listener File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-core/src/statistics/event/listener.rs` (ledger order 14).

## Current State

- **Unit-only coverage:** 124 / 145 lines (85.52%), 184 / 204 regions, 18 / 21 functions.
  Unchanged since T1.
- **Uncovered lines:**
  - 17-29 and 32-37: `run_event_listener` (spawns) and `run_event_listener_unspawned` (logs, then
    delegates). Task lifecycle.
  - 100: the `configuration_instance_id` field of the "ignoring" warning; evaluated only when a
    tracing subscriber enables the event.
  - 109: the `UdpAnnounce` and `UdpScrape` arms of `event_connection_id()`. Every test sends
    `UdpConnect`.
- **Existing tests:** four, all on `dispatch_events()` with a scripted receiver.
- **Module-owned decisions:**
  1. Stop with `Completion::Completed` when the receiver closes.
  2. Keep running after `RecvError::Lagged`.
  3. Count an event only if the metrics policy enables the listener that produced it; ignore
     disabled and unknown listeners (#2039).
  4. Check cancellation before a ready event (`biased`).
- **Owned elsewhere:**
  - Decision 4 and the spawn/unspawned entry points: the listener lifecycle, owned by #1488.
    [#2414](https://github.com/torrust/torrust-tracker/issues/2414) (drain queues on shutdown)
    names this file and will replace the `it_should_prioritize_*` characterization test.
  - Which counter each event kind increases: `statistics/event/handler.rs` (done).
- **T1 hypothesis:** lifecycle (#1488).

## Current Tests Review

| Test | Review |
| --- | --- |
| `it_should_stop_when_the_receiver_is_closed` | **Smell: the assertion does not state the decision.** It asserts the counter is 0, which an empty run also gives; the stop is proven only because the test would hang otherwise. `dispatch_events()` returns `Completion`, so the test can assert `Completion::Completed` directly. |
| `it_should_continue_after_lag_and_process_an_enabled_event` | Clean: the lag comes before an enabled event, and the count of 1 proves the loop continued. |
| `it_should_prioritize_cancellation_over_a_ready_event` | Owned by #2414; left unchanged. |
| `it_should_ignore_disabled_and_unknown_events_before_processing_an_enabled_event` | Clean: the trailing enabled event shows the earlier two were ignored, not lost to an early stop. |

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Stop on a closed receiver | 3A unit | R1: assert `Completion::Completed` instead of the counter. |
| Continue after lag | Covered | No change. |
| Policy gate for every event kind | 3A unit | R2 L1: one `rstest` test with `connect`, `announce`, and `scrape` cases. An event of that kind from an enabled listener is counted (requests-received total of 1), so the gate reads the listener ID from every variant. Covers line 109. |
| Ignore disabled and unknown listeners | Covered | No change; L1 does not repeat it per kind. |
| Cancellation priority, spawn and unspawned entry points | Owned by #1488 / #2414 | Not tested here. |
| Warning field (line 100) | Not tested | Needs a tracing subscriber; the log content is not a module decision. |
| Property-based testing | Not selected | Three enumerable event kinds. |

L1 asserts the requests-received counter summed across all series rather than a per-kind
projection, so the expected value is the same literal (1) for every case; per-kind routing stays
with the handler tests. The `announce` case needs an `InfoHash` and a `PeerAnnouncement`; it uses
literals and `PeerBuilder`, as the other `udp-core` tests do.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | PENDING | Assert `Completion::Completed` in the closed-receiver test. | - |
| R2 | PENDING | Add L1. | - |
| R3 | SKIPPED | No integration increment: the lifecycle that would need a running task belongs to #1488. | Explicit rationale. |
| R4 | PENDING | Re-run unit-only coverage; record before/after. | - |

## Results

Pending.

## Approval

Approved by Jose Celano on 2026-10-14: R1 and L1.

## Progress Log

- 2026-10-14 09:24 UTC - GitHub Copilot - Created the plan from `statistics/event/listener.rs`, the
  handler it calls, and #2414's scope for this file. No Rust tests or production code changed. R1
  awaits maintainer approval.
- 2026-10-14 10:03 UTC - Jose Celano - Approved R1 and L1.
