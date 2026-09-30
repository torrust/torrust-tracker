---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/statistics/repository.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/statistics/repository.rs
    - packages/udp-server/src/statistics/metrics.rs
    - docs/issues/open/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Statistics Repository File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/statistics/repository.rs`.

## Current State

- **Fresh unit-only coverage:** 547 / 549 lines (99.64%), 61 / 62 functions, and 771 / 773
  regions from the stable Rust toolchain report `cargo llvm-cov -p torrust-tracker-udp-server
  --all-features --lib --json`. No package source has changed since that report was generated for
  row 23.
- **Module-owned decisions:** synchronization around statistics access, initialization with the
  UDP metric collection, and forwarding metric updates while releasing the write lock.
- **Existing tests:** deterministic repository tests cover collection initialization, read/write
  access, label isolation, averages, and concurrent updates. R1 reduced the suite from 17 to 15
  tests by removing two derived-trait implementation checks; R2 then added two behavioral wrapper
  contracts, leaving 17 tests.
- **T1 hypothesis:** probably no change. The existing suite covers the repository boundary, but a
  narrow readability refactor is appropriate before preserving the no-change conclusion.

## Current Tests Review

| Test group | Smells | Response |
| --- | --- | --- |
| `Default` and `Clone` tests | Raw pointer comparison observes wrapper stack addresses, not the derived trait's relevant behavior or shared state. | R1 refactor: remove or replace only when a direct repository contract can be stated. |
| Metric initialization and read/write tests | Observable collection and projection behavior are visible, though some comments narrate mechanics. | Retain behavior; simplify only comments that obscure AAA prose. |
| Three moving-average tests | Repeated metric setup and duplicated formula narration dilute the single causal difference: request kind and values. | R1 refactor into clear state-centred cases without hiding the production Act or expected average. |
| Concurrent-update test | Named helpers retain asynchronous mechanics while assertions name per-server results and collection integrity. | Retain pending focused review. |

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Repository lock, metric writes, and reads | 3A unit | Existing tests directly exercise the public asynchronous repository API. |
| Metric declaration composition | Collaboration | `statistics/mod.rs` declares metrics; repository initialization remains the observable consumer boundary. |
| Moving-average arithmetic and metric storage errors | Collaborator-owned | `statistics/metrics.rs` and `torrust-metrics` own the calculation/storage semantics. |
| Concurrent serialization of repository calls | 3A unit | Existing concurrent-update test covers the repository's lock-backed shared-state boundary. |
| Multi-listener aggregation | Integration/root-owned | Root composition owns aggregation across repositories. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Removed two non-behavioral wrapper-address checks and consolidated duplicated moving-average cases into a label-preservation contract. | Focused repository tests passed 15 tests; nightly formatter and diff checks passed. Maintainer review is requested. |
| R2 | DONE | Added direct trait and default-construction contracts for initialized metrics collections. | Focused repository tests passed 17 tests; each test asserts a registered UDP request counter through the external trait boundary. |
| R3 | SKIPPED | No real UDP integration increment is selected. | Repository synchronization and collection snapshots are directly testable; multi-listener aggregation is root-owned. |
| R4 | DONE | Re-ran and recorded unit-only coverage. | 523 / 523 lines (100.00%), 60 / 60 functions, and 749 / 749 regions. |

## Results

Unit-only coverage increased from **547 / 549 lines (99.64%)** to **523 / 523 lines (100.00%)**
after the R1 test refactor and two R2 behavioral wrapper tests.

## Completed-File Review

Approved by Jose Celano on 2026-09-29.

## Progress Log

- 2026-09-29 - GitHub Copilot - Created the plan from the source, existing tests, and current
  unit-only coverage. The plan requests approval before the R1 test refactor.
- 2026-09-29 15:18 UTC - GitHub Copilot - Completed R1. The refactor removes stack-address
  checks that did not observe a repository contract and consolidates the three one-sample moving
  average cases into one test that visibly preserves independent request-kind labels. Focused tests,
  nightly Rust formatting, and diff checks pass. Maintainer review is requested before R2.
- 2026-09-29 - Jose Celano - Approved the R1 refactor.
- 2026-09-29 15:31 UTC - GitHub Copilot - Completed R2-R4. Focused repository tests passed 17
  tests on the stable Rust toolchain; nightly Rust formatting and diff checks passed. Fresh
  unit-only coverage is 523 / 523 lines (100.00%). Completed-file review is requested.
- 2026-09-29 15:34 UTC - Jose Celano - Approved the completed-file result.
