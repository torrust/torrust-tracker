---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/statistics/metrics.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/statistics/metrics.rs
    - packages/udp-server/src/statistics/repository.rs
    - docs/issues/closed/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/closed/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Statistics Metrics File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/statistics/metrics.rs`.

## Current State

- **Fresh unit-only coverage:** 827 / 834 lines (99.16%), 86 / 87 functions, and 1,228 / 1,238
  regions from the stable Rust toolchain report `cargo llvm-cov -p torrust-tracker-udp-server
  --all-features --lib --json`. No package source has changed since this report was generated for
  row 18.
- **Module-owned decisions:** label-specific UDP metric projections and the incremental average of
  request-processing time.
- **Existing tests:** 60 colocated, deterministic unit tests cover empty values, counter/gauge
  updates, IPv4/IPv6 and request-kind filtering, mixed labels, and moving-average updates.
- **T1 hypothesis:** probably no change. The small remaining coverage gap does not yet identify a
  distinct observable module contract.

## Current Tests Review

| Test group | Smells | Response |
| --- | --- | --- |
| Empty and populated accessor tests | Clear state, direct production Act, and typed assertions. Repeated counter setup is visible and identifies the tested metric. | Retain; R1 initially clean. |
| IPv4/IPv6 and request-kind separation tests | Causal labels and expected counts remain visible. | Retain; R1 initially clean. |
| Moving-average tests | Deterministic clock values, visible inputs, and result assertions cover the module's calculation. | Retain; R1 initially clean. |
| Trait and write-delegation tests | `Debug`, `PartialEq`, and `MetricCollection` write success originate in derived or collaborator behavior. | Do not expand or refactor without a concrete readability defect. |

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Label-filtered UDP totals and IPv4/IPv6 separation | 3A unit | Already covered by direct accessor tests and combined-label tests. |
| Incremental processing-time average | 3A unit | Already covered by deterministic current tests; inspect the residual branches before selecting any change. |
| Counter/gauge storage, aggregation, and write errors | Collaborator-owned | `torrust-metrics::MetricCollection` owns storage and aggregation semantics; do not duplicate its failure cases. |
| Multi-listener aggregation | Integration/root-owned | Root integration owns aggregation across listener instances. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Reviewed the current tests; no concrete readability defect justified a refactor. | `cargo test -p torrust-tracker-udp-server statistics::metrics::tests` passed 60 tests. |
| R2 | SKIPPED | The seven uncovered lines are the `Err` logging arms for internal `MetricCollection` writes; the ten uncovered regions are their associated fallback/cast paths. | The metric collection owns those error semantics; no distinct module-owned behavior is uncovered. |
| R3 | SKIPPED | No real UDP integration increment is selected. | Label projection and average calculation have direct deterministic unit tests; multi-listener aggregation is root-owned. |
| R4 | DONE | Re-ran and recorded unit-only coverage. | 827 / 834 lines (99.16%), 86 / 87 functions, and 1,228 / 1,238 regions. |

## Results

Unit-only coverage remained **827 / 834 lines (99.16%)** before and after review. The current tests
are clean, and no Rust test or production-code change was appropriate.

## Completed-File Review

Approved by Jose Celano on 2026-09-29.

## Progress Log

- 2026-09-29 - GitHub Copilot - Created the plan from the current source, 60 colocated tests,
  the current unit-only coverage report, and the #2149 coverage boundary. Maintainer approval is
  requested before reviewing or changing tests.
- 2026-09-29 14:47 UTC - GitHub Copilot - Reviewed the 60 current tests and recorded R1 as clean.
  Focused metrics tests passed on the stable Rust toolchain. The fresh unit-only report confirmed
  827 / 834 lines (99.16%); the residual paths are `MetricCollection` write-error logging and
  fallback/cast paths, so R2 and R3 are skipped. Completed-file review is requested.
- 2026-09-29 14:59 UTC - Jose Celano - Approved the completed-file result.
