---
doc-type: file-test-plan
issue: 1348
package: torrust-tracker-udp-core
target-file: packages/udp-core/src/statistics/services.rs
status: approved
semantic-links:
  related-artifacts:
    - packages/udp-core/src/statistics/services.rs
    - packages/udp-core/src/statistics/repository.rs
    - packages/udp-core/src/statistics/metrics.rs
    - packages/udp-server/src/statistics/services.rs
    - docs/issues/open/1348-1347-add-tests-udp-core/coverage-evidence.md
---

# UDP Core Statistics Services File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-core/src/statistics/services.rs` (ledger order 13).

## Current State

- **Unit-only coverage:** 32 / 32 lines (100.00%), 27 / 27 regions, 4 / 4 functions. Unchanged
  since T1.
- **Existing tests:** one, `the_statistics_service_should_return_the_tracker_metrics`.
- **Module-owned decision:** `get_metrics()` combines two sources into one `TrackerMetrics`
  snapshot: the swarm aggregate from `InMemoryTorrentRepository` and a copy of the protocol metric
  collection from the statistics `Repository`.
- **Collaborator-owned:** how the aggregate is computed (`tracker-core`); how counters are stored
  (`statistics/repository.rs`, done).
- **Consumers:** none outside this file. The REST runtime adapter reads the statistics repository
  directly; `udp-server`'s `get_metrics()` / `TrackerMetrics` are in the same state.
- **T1 hypothesis:** probably no change.

## Current Tests Review

`the_statistics_service_should_return_the_tracker_metrics` (V0):

- **Smell: empty Arrange cannot prove the service reads its inputs.** Both repositories are empty,
  so the expected value equals what a service that ignores them would return. Two mutations
  survive: returning `AggregateActiveSwarmMetadata::default()` instead of the aggregate, and
  returning a fresh `describe_metrics()` instead of copying the repository's collection.
- **Smell: production-derived expectation.** The expected protocol metrics come from
  `describe_metrics()`, the same function the repository uses to start.
- Minor: AAA sections are implicit; `.clone()` of both `Arc`s is unnecessary.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Swarm aggregate comes from the torrent repository | 3A unit | R1a: replace V0 with V1: one seeder announced on one torrent; `torrents_metrics` reports 1 torrent and 1 complete peer (literals). Fails the first mutation above. |
| Protocol metrics come from the statistics repository | 3A unit | R1b: V2: one requests-received counter increased (an IPv4 announce); `protocol_metrics.udp4_announces_handled()` is 1. Fails the second mutation above. |
| Aggregate computation, counter storage | Owned elsewhere | Not retested here. |
| Property-based testing | Not selected | No input-dependent decision. |

V0 is replaced by V1 and V2 rather than extended, so each source has one test with one reason to
fail. Line coverage is already 100%; the gain is mutation strength, not coverage.

**Stale module docs (fix in this file, per the maintainer's rule for stale docs found in the file
being worked on):**

1. The `rust,no_run` example shows `Metrics` with a `pub udp4_connections_handled: u64` field.
   `Metrics` has only `metric_collection`; `udp4_connections_handled()` is a method projecting the
   requests-received counter. Proposed: replace the fake struct with a `text` line naming the
   method, or remove the example.
2. "An statistics" → "A statistics".
3. "UDP connections from IPv4 peers" has the server-socket-family wording issue already recorded
   for the metrics docs; reword to "on IPv4 server sockets" here, since the line is being edited
   anyway.

**Finding (recorded in [lessons.md](../lessons.md); no production change in this issue):**
`get_metrics()` and `TrackerMetrics` have no consumer outside their own tests, in both `udp-core`
and `udp-server`. The maintainer forwards it to the agent handling the unused SI-30 traits draft.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | PENDING | Replace V0 with V1 and V2; fix the three stale doc points. | - |
| R2 | SKIPPED | No further tests: the module has one decision, covered by V1 and V2. | Recorded above. |
| R3 | SKIPPED | No integration increment: in-memory collaborators only. | Explicit rationale. |
| R4 | PENDING | Re-run unit-only coverage; record before/after. | - |

## Results

Pending.

## Approval

Approved by Jose Celano on 2026-10-14: V1-V2 and the three doc fixes. The finding goes to
`lessons.md`.

## Progress Log

- 2026-10-14 08:16 UTC - GitHub Copilot - Created the plan from `statistics/services.rs`, its
  collaborators, its absent consumers, and `udp-server`'s matching service. No Rust tests or
  production code changed. R1 awaits maintainer approval.
- 2026-10-14 08:57 UTC - Jose Celano - Approved V1-V2 and the doc fixes; the finding is recorded in
  `lessons.md` and forwarded by the maintainer.
