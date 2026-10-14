---
doc-type: file-test-plan
issue: 1348
package: torrust-tracker-udp-core
target-file: packages/udp-core/src/peer_builder.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-core/src/peer_builder.rs
    - packages/udp-core/src/services/announce.rs
    - packages/udp-protocol/src/announce.rs
    - docs/issues/open/1348-1347-add-tests-udp-core/coverage-evidence.md
---

# UDP Core Peer Builder File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/udp-core/src/peer_builder.rs`.

## Current State

- **Unit-only coverage:** 0 / 14 lines (0.00%) at T1; 11 / 14 lines (78.57%) and 22 / 25 regions
  after the announce service tests, which execute `from_request` but leave the `Completed`,
  `Stopped`, and `None` event arms uncovered. Those tests assert only the address and one full
  event peer for a `Started` request with zero counters, so the coverage is incidental.
- **Existing tests:** none in this file.
- **Module-owned decisions:** `from_request` maps a protocol announce request to a domain peer:
  peer ID; address from the caller-supplied IP plus the request port, never the request's
  `ip_address` field; `updated` from the current clock; uploaded, downloaded, and left counters into
  the matching domain fields; the four wire events into the domain events.
- **T1 hypothesis:** test candidate.

## Current Tests Review

No existing tests. R1 is skipped.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Peer ID | 3A unit | P1: the domain peer carries the request's peer ID. |
| Peer address | 3A unit | P2: the address is the supplied IP with the request port, although the request names another `ip_address`. |
| Transfer counters | 3A unit | P3: distinct uploaded, downloaded, and left values land in the matching fields, so a swap fails. |
| Wire event mapping | 3A unit | P4: one table test maps `Started`, `Stopped`, `Completed`, and `None` to the domain events. |
| Announce time | 3A unit | P5: with the stopped test clock set to a distinctive time, `updated` equals that time. |
| Unknown wire event values | Collaborator-owned | `udp-protocol` maps unknown values to `None`; no test here. |
| Property-based testing | Not selected | Counter mapping is a field copy; distinct example values already detect swaps, and arbitrary values add no state the examples miss. |

Test data: an in-file request with every field written in the Arrange so the mapped value is
visible next to its assertion. The announce service's request helper is not shared: these tests
vary the fields that helper fixes, and two short literals do not justify a crate-wide fixture.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests to refactor. | Recorded above. |
| R2 | DONE | Add P1-P5. | 5 focused `peer_builder` tests passed; nightly formatting, pedantic Clippy, and `git diff --check` clean; P3 failed under a temporary uploaded/downloaded swap that the announce service tests did not detect. |
| R3 | SKIPPED | No integration increment: a pure mapping function has no boundary to integrate. | Explicit rationale. |
| R4 | DONE | Re-run unit-only coverage; record before/after. | 11 / 14 to 79 / 79 lines; see Results. |

## Results

- **Unit-only coverage (test-inclusive file totals):** 0 / 14 (0.00%) at T1 and 11 / 14 (78.57%)
  after the announce tests, to 79 / 79 (100.00%) lines, 115 / 115 regions, and 8 / 8 functions.
- **Tests added:** 5 (P1-P5), with each mapped field set in the Arrange next to its assertion.
- **Integration selected:** No.
- **Completed-file review:** Approved by Jose Celano on 2026-10-05.

## Approval

Approved by Jose Celano on 2026-10-05: P1-P5, the local request data, and no property-based
testing.

## Progress Log

- 2026-10-05 12:01 UTC - GitHub Copilot - Rebased onto `develop` (25 commits; none touched
  `udp-core`, `udp-protocol`, or the domain peer). Created the plan from `peer_builder.rs` and the
  `udp-protocol` wire event conversion. No Rust tests or production code changed. R2 awaits
  maintainer approval.
- 2026-10-05 12:56 UTC - GitHub Copilot - Plan approved. Implemented R2 and measured R4 (stable
  Rust 1.99.0, nightly 1.101.0, `cargo-llvm-cov 0.6.16`). The plan awaits completed-file review.
- 2026-10-05 13:19 UTC - Jose Celano - Approved completed-file review. The file is ready for
  pre-commit and its signed per-file commit.
- 2026-10-14 05:40 UTC - GitHub Copilot - Follow-up (maintainer-approved, 2026-10-13): P4's loop
  became an `rstest` test with four named cases, so each wire-to-domain event pair is visible and
  fails on its own. 73 package unit tests passed; pedantic Clippy and nightly formatting clean.
