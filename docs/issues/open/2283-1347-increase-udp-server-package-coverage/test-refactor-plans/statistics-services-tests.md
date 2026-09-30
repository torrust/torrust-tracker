---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/statistics/services.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/statistics/services.rs
    - packages/udp-server/src/statistics/repository.rs
    - docs/issues/open/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Statistics Services File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/statistics/services.rs`.

## Current State

- **Fresh unit-only coverage:** 32 / 32 lines (100.00%), 4 / 4 functions, and 27 / 27 regions
  from the stable Rust toolchain report `cargo llvm-cov -p torrust-tracker-udp-server --all-features
  --lib --json`. No package source has changed since the row-24 report was generated.
- **Module-owned decision:** combine aggregate torrent metadata and a cloned UDP metric collection
  into `TrackerMetrics`.
- **Existing tests:** one deterministic asynchronous test constructs empty repositories and asserts
  the complete returned `TrackerMetrics` value.
- **T1 hypothesis:** probably no change. The service aggregation is directly and fully covered.

## Current Tests Review

| Test | Smells | Response |
| --- | --- | --- |
| `the_statistics_service_should_return_the_tracker_metrics` | The name is not behavior-driven and unnecessary `Arc` clones obscure that `get_metrics` consumes the test-owned handles. | R1 refactor: use an `it_should_*` name and pass the handles directly; retain the whole-value assertion. |

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Aggregation of torrent metadata and UDP metric collection | 3A unit | Already covered by the direct whole-value service result assertion. |
| Torrent aggregate calculation | Collaborator-owned | `InMemoryTorrentRepository` owns aggregate swarm metadata. |
| Metrics collection registration and mutation | Collaborator-owned | `statistics/mod.rs`, `repository.rs`, and `metrics.rs` own registration and updates. |
| Cross-listener or application exposure | Integration/root-owned | Root composition owns cross-service aggregation and exposure. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Renamed the test for its aggregation contract and removed unnecessary `Arc` clones. | Focused services test passed; nightly formatter and diff checks passed. Maintainer review is requested. |
| R2 | SKIPPED | No additional unit test is proposed. | The module's only behavior is already fully covered. |
| R3 | SKIPPED | No real UDP integration increment is selected. | The service API directly observes its aggregation result. |
| R4 | DONE | Re-ran and recorded unit-only coverage after the approved refactor. | 32 / 32 lines (100.00%), 4 / 4 functions, and 25 / 25 regions. |

## Results

Unit-only coverage remained **32 / 32 lines (100.00%)** before and after the R1 test refactor.
No additional Rust test or production-code change is appropriate.

## Completed-File Review

Approved by Jose Celano on 2026-09-29.

## Progress Log

- 2026-09-29 - GitHub Copilot - Created the plan from the source, current test, and unit-only
  coverage report. The plan requests approval before the R1 test refactor.
- 2026-09-29 15:51 UTC - GitHub Copilot - Completed R1. The test now names its aggregate torrent
  and protocol metrics contract and passes the test-owned repositories directly to `get_metrics`.
  Focused tests, nightly Rust formatting, and diff checks pass. Maintainer review is requested
  before R4.
- 2026-09-29 - Jose Celano - Approved the R1 refactor.
- 2026-09-29 15:59 UTC - GitHub Copilot - Completed R2-R4. Fresh unit-only coverage is 32 / 32
  lines (100.00%), 4 / 4 functions, and 25 / 25 regions. Completed-file review is requested.
- 2026-09-29 16:02 UTC - Jose Celano - Approved the completed-file result.
