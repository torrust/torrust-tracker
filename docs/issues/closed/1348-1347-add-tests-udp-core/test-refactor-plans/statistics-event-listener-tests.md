---
doc-type: file-test-plan
issue: 1348
package: torrust-tracker-udp-core
target-file: packages/udp-core/src/statistics/event/listener.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-core/src/statistics/event/listener.rs
    - packages/udp-core/src/statistics/event/handler.rs
    - docs/issues/closed/2414-2410-si-22-2-drain-listener-queues-on-shutdown/ISSUE.md
    - docs/issues/closed/1348-1347-add-tests-udp-core/coverage-evidence.md
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
| R1 | DONE | Assert `Completion::Completed` in the closed-receiver test. | Renamed to `it_should_complete_when_the_receiver_is_closed`. With the closed branch returning `Completion::Cancelled`, it failed (the old counter assertion would have passed); reverted by hand. |
| R2 | DONE | Add L1. | 82 package unit tests passed (stable Rust 1.99.0); pedantic Clippy, nightly formatting (nightly 1.101.0), and `git diff --check` clean. With the policy gate admitting only `UdpConnect`, exactly `case_2_announce` and `case_3_scrape` failed; reverted by hand. Production code unchanged. |
| R3 | SKIPPED | No integration increment: the lifecycle that would need a running task belongs to #1488. | Explicit rationale. |
| R4 | DONE | Re-run unit-only coverage; record before/after. | 124 / 145 to 144 / 164 lines; see Results. |

## Results

- **Unit-only coverage (test-inclusive file totals, `cargo-llvm-cov 0.6.16`):** 124 / 145 (85.52%)
  to 144 / 164 (87.80%) lines; 193 / 211 regions; 21 / 24 functions. Line 109 is now covered. The
  remaining uncovered lines are the two entry points (17-37, owned by #1488 / #2414) and the
  warning field (100).
- **Tests:** 1 sharpened (R1), 1 added (L1, three `rstest` cases).
- **Integration selected:** No.
- **Completed-file review:** Approved by Jose Celano on 2026-10-14.

## Prose-First AAA Review

Recorded on 2026-10-15, late: the PR #2538 Task Reviewer found no record of it (AC7). For each
test this plan touched, temporary Arrange, Act, and Assert prose was compared with the code and
then removed.

| Test | Arrange → Act → Assert | Outcome |
| --- | --- | --- |
| R1 `it_should_complete_when_the_receiver_is_closed` | Receiver yields only `Closed` → `dispatch_events` → `Completed` | No change |
| L1 `it_should_count_an_event_of_any_kind_from_a_metrics_enabled_listener` | One event of the case's kind from an enabled listener → `dispatch_events` → one request counted | No change |

## Approval

Approved by Jose Celano on 2026-10-14: R1 and L1.

## Progress Log

- 2026-10-14 09:24 UTC - GitHub Copilot - Created the plan from `statistics/event/listener.rs`, the
  handler it calls, and #2414's scope for this file. No Rust tests or production code changed. R1
  awaits maintainer approval.
- 2026-10-14 10:03 UTC - Jose Celano - Approved R1 and L1.
- 2026-10-14 10:07 UTC - GitHub Copilot - Implemented R1 and R2 and measured R4 (stable Rust
  1.99.0, nightly 1.101.0, `cargo-llvm-cov 0.6.16`). The plan awaits completed-file review.
- 2026-10-14 10:13 UTC - Jose Celano - Approved completed-file review. The file is ready for
  pre-commit and its signed per-file commit.
- 2026-10-15 10:30 UTC - GitHub Copilot - Recorded the prose-first AAA review late, after the
  PR #2538 Task Reviewer found no record (AC7). No change.
