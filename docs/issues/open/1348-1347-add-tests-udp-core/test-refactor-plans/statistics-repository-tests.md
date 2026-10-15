---
doc-type: file-test-plan
issue: 1348
package: torrust-tracker-udp-core
target-file: packages/udp-core/src/statistics/repository.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-core/src/statistics/repository.rs
    - packages/udp-core/src/statistics/mod.rs
    - packages/udp-core/src/statistics/metrics.rs
    - packages/udp-server/src/statistics/repository.rs
    - packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs
    - docs/issues/open/1348-1347-add-tests-udp-core/coverage-evidence.md
---

# UDP Core Statistics Repository File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-core/src/statistics/repository.rs` (ledger order 11).

## Current State

- **Unit-only coverage:** 21 / 26 lines (80.77%), 5 / 7 functions. Unchanged since T1.
- **Uncovered lines:** 31-33 (`Default::default`) and 70, 72
  (`UdpCoreStatsRepository::get_metrics_collection`).
- **Existing tests:** none in this file. `new()`, `get_stats()`, and `increase_counter()` run only
  through the event handler and listener tests, which assert on metric projections.
- **Module-owned decisions:**
  1. Both constructors start from `describe_metrics()`, so the requests-received counter is
     described before any request arrives.
  2. `increase_counter()` writes through the shared lock and returns the collection's result
     unchanged, including its error.
  3. `get_metrics_collection()` returns a point-in-time copy, not a live view.
- **Collaborator-owned:** when `MetricCollection::increase_counter` fails (a name collision with a
  gauge) belongs to `torrust-metrics`; the projections belong to `statistics/metrics.rs` (done).
- **T1 hypothesis:** test candidate.

## Current Tests Review

No tests in this file. R1 is skipped.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Constructors describe the metrics | 3A unit | S1: one `rstest` test with `new` and `default` cases; the collection contains the requests-received counter. Two named cases state that both entry points share the guarantee. |
| Counter increase is visible through the repository | 3A unit | S2: after one `increase_counter()`, `get_stats()` reads value 1 for that label set. |
| Counter errors are returned to the caller | 3A unit | S3: with a gauge registered under the same name (written through the public `stats` lock), `increase_counter()` returns `MetricNameCollisionAdding`. A mutation that swallows the result (`Ok(())`) fails it; nothing else covers that. |
| Snapshot is point-in-time | 3A unit | S4: take `get_metrics_collection()` after one increase, increase again; the snapshot still reads 1. |
| Lock released after the write | Not tested separately | The explicit `drop` only shortens the guard; S2 and S3 would hang if the write lock leaked into `get_stats()`. |
| `Clone` shares state through the `Arc` | Not tested | Derived; every caller shares the repository through an outer `Arc`. |
| Property-based testing | Not selected | No input-dependent decision; the repository forwards to the collection. |

Test data: label sets and counts written as literals; the metric name comes from the package's
`UDP_TRACKER_CORE_REQUESTS_RECEIVED_TOTAL`, as in the metrics tests. Timestamps are
`DurationSinceUnixEpoch::ZERO`; the repository does not interpret them.

**Findings (no production change in this issue; handed off as EPIC #1669 sub-issue drafts):**

1. `UdpCoreStatsRepository` has no consumer. It was added for REST API decoupling (SI-30), but the
   REST runtime adapter reads `get_stats().await.metric_collection` from the concrete
   `Repository`. The other two SI-30 traits, `BanningStats` and `UdpServerStatsRepository`, are in
   the same state; the REST API now decouples through `StatsQueryPort` in `rest-api-application`.
2. The `pub stats` field exposes the lock, so callers can bypass `increase_counter()`. The same
   public field exists in the `udp-server`, `http-core`, `tracker-core`, and
   `swarm-coordination-registry` statistics repositories; no caller outside those files uses it.
   S3 uses it only to build the collision state, and a test module inside the file keeps access if
   the field becomes private.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests to refactor. | Recorded above. |
| R2 | DONE | Add S1-S4. | 78 package unit tests passed (stable Rust 1.99.0); pedantic Clippy, nightly formatting (nightly 1.101.0), and `git diff --check` clean. Mutations, reverted by hand: `increase_counter()` returning `Ok(())` failed S3; `Default` building `Metrics::default()` without the description failed S1's `default` case. |
| R3 | SKIPPED | No integration increment: in-memory state behind a lock. | Explicit rationale. |
| R4 | DONE | Re-run unit-only coverage; record before/after. | 21 / 26 to 86 / 86 lines at the time, 70 / 70 after S4 was dropped; see Results. |

## Results

- **Unit-only coverage (test-inclusive file totals, `cargo-llvm-cov 0.6.16`):** 21 / 26 (80.77%)
  to 86 / 86 (100.00%) lines, 94 / 95 regions, 16 / 16 functions at completion; 70 / 70 lines,
  73 / 74 regions, 13 / 13 functions after the rebase onto #2517 and #2521.
- **Tests added:** 4 at completion (S1 as one `rstest` test with `new` and `default` cases,
  S2-S4); 3 remain after S4 was dropped with the trait method it tested.
- **Findings:** both handed off as EPIC #1669 sub-issue drafts and recorded in
  [lessons.md](../lessons.md).
- **Integration selected:** No.
- **Completed-file review:** Approved by Jose Celano on 2026-10-14.

## Prose-First AAA Review

Recorded on 2026-10-15, late: the PR #2538 Task Reviewer found no record of it (AC7). For each
test, temporary Arrange, Act, and Assert prose was compared with the code and then removed.

| Test | Arrange → Act → Assert | Outcome |
| --- | --- | --- |
| S1 `it_should_describe_the_requests_received_counter_when_created` | The case picks `new` or `default` → construct → the counter is described | No change |
| S2 `it_should_make_an_increased_counter_visible_through_the_repository_stats` | Empty repository → `increase_counter` → the stats read 1 | Changed: the `increase_counter` call is inline under `// Act`; the single-use helper that hid it is gone |
| S3 `it_should_return_the_metric_collection_error_when_a_counter_cannot_be_increased` | Name already a gauge → `increase_counter` → collision error | No change |

## Approval

Approved by Jose Celano on 2026-10-14: S1-S4. Both findings become separate EPIC #1669 sub-issues,
drafted with hand-off notes under `.tmp/` for another agent to take through spec-only PRs.

## Progress Log

- 2026-10-14 06:24 UTC - GitHub Copilot - Created the plan from `statistics/repository.rs`, its
  callers, the REST runtime adapter, the `udp-server` sibling repository and its tests, and the
  `torrust-metrics` 0.1.0 `increase_counter` error conditions. No Rust tests or production code
  changed. R2 awaits maintainer approval.
- 2026-10-14 06:42 UTC - Jose Celano - Approved S1-S4. The findings are widened (all three SI-30
  traits; the `pub stats` field in five packages) and handed off as two EPIC #1669 sub-issue drafts.
- 2026-10-14 07:28 UTC - GitHub Copilot - Implemented R2 and measured R4 (stable Rust 1.99.0,
  nightly 1.101.0, `cargo-llvm-cov 0.6.16`). The plan awaits completed-file review.
- 2026-10-14 08:07 UTC - Jose Celano - Approved completed-file review. The file is ready for
  pre-commit and its signed per-file commit.
- 2026-10-14 16:57 UTC - GitHub Copilot - Rebased onto #2521 (removed `UdpCoreStatsRepository`)
  and #2517 (made `stats` private, added a `new`-only construction test). As approved by Jose
  Celano: S4 was dropped with the trait method it tested, and #2517's construction test was
  dropped as superseded by S1's `new` case. S1-S3 remain; S3 still writes through `stats` from the
  in-file test module. Unit-only file totals: 70 / 70 lines, 73 / 74 regions, 13 / 13 functions.
- 2026-10-15 10:30 UTC - GitHub Copilot - The PR #2538 review (F6) gave S1 and S2 the
  `it_should_` prefix, AAA sections, and S1 an assertion message ("test(udp-core): name and
  structure the statistics tests by the unit-test conventions"). After the Task Reviewer found no
  prose-first record, recorded the review above: S2's Act is now inline. Unit-only file totals:
  66 / 66 lines, 68 / 69 regions, 11 / 11 functions.
