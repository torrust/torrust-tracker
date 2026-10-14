---
doc-type: file-test-plan
issue: 1348
package: torrust-tracker-udp-core
target-file: packages/udp-core/src/statistics/event/handler.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-core/src/statistics/event/handler.rs
    - packages/udp-core/src/statistics/event/listener.rs
    - packages/udp-core/src/statistics/metrics.rs
    - packages/udp-core/src/event.rs
    - docs/issues/open/1348-1347-add-tests-udp-core/coverage-evidence.md
---

# UDP Core Statistics Event Handler File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-core/src/statistics/event/handler.rs`.

## Current State

- **Unit-only coverage:** 199 / 200 lines (99.50%) at T1 and unchanged since.
- **Uncovered line:** 20, the `tracing::error!` when the repository fails to increase the counter.
- **Existing tests:** 7: six "should increase the udp4/udp6 connections/announces/scrapes counter"
  tests, one per event kind and IP family, and one test that checks the full label set plus
  `request_kind` for a scrape event with a public URL.
- **Module-owned decisions:** map each event to its `request_kind` value (`connect`, `announce`,
  `scrape`); add that label to the connection's labels; increase the requests-received counter
  once per event; log, not panic, when the counter cannot be increased.
- **Caller:** `statistics/event/listener.rs`.
- **T1 hypothesis:** probably no change. The test review below falsifies it.

## Current Tests Review

Conclusion: `refactor`.

- The six per-family tests decide nothing about the handler: the IP family comes from
  `ConnectionContext` (`event.rs`) and is selected by the `metrics.rs` projections, which now have
  direct tests. They differ only in event kind and server family, and each repeats about twenty
  lines of construction. They have no Arrange/Act/Assert sections.
- What they do protect, the event-to-`request_kind` mapping, can be expressed once: one table test
  over the three event kinds, asserting the counter for that `request_kind` is 1.
- `it_should_propagate_all_connection_labels_and_request_kind` is clean and stays. It is the only
  test showing that adding `request_kind` keeps every connection label, including `public_url`.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Event kind to `request_kind` | 3A unit | H1: one `rstest` parameterized test (one case per event kind) replacing the six per-family tests, as the `write-unit-test` skill recommends for input/output combinations. |
| Request kind added without dropping connection labels | 3A unit | Existing full-label test; kept. |
| IP family in labels and per-family counts | Owned elsewhere | `event.rs` (label projection) and `metrics.rs` (projections) test these directly. |
| Counter-increase failure is logged, not propagated | Not selected | Line 20 needs a repository whose counter cannot be increased. `Repository::new()` always describes the counter, and there is no injection point. |
| Property-based testing | Not selected | Three enumerable event kinds. |

**Stale doc, fixed in this file's commit (maintainer decision, 2026-10-13):** the `handle_event` doc
has a `# Panics` section, "panics if the IP version does not match the event type". The function has
no such check and cannot panic for that reason; the events no longer carry an IP version. R1 removes
the section.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Replace the six per-family tests with H1 (adding `rstest` as a `udp-core` dev-dependency) and remove the stale `# Panics` doc. | 70 package unit tests passed; pedantic Clippy, `cargo machete`, nightly formatting, and `git diff --check` clean. With `connect` and `scrape` temporarily swapped in the mapping, exactly `case_1_connect` and `case_3_scrape` failed by name (plus the full-label test and two listener tests); reverted by hand. |
| R2 | SKIPPED | No new behavior beyond H1. | Recorded above. |
| R3 | SKIPPED | No integration increment: the listener (`listener.rs`, its own ledger row) owns dispatch. | Explicit rationale. |
| R4 | DONE | Re-run unit-only coverage; record before/after. | 199 / 200 to 60 / 61 lines; see Results. |

## Results

- **Unit-only coverage (test-inclusive file totals):** 199 / 200 (99.50%) to 60 / 61 (98.36%)
  lines; 104 / 108 regions; 6 / 6 functions. The denominator shrank because six long tests became
  three `rstest` cases; the only uncovered line is still the error log.
- **Tests:** 6 per-family tests replaced by one `rstest` test with three named cases; the
  full-label test kept.
- **Production change:** the stale `# Panics` doc section removed (documentation only).
- **Dependency:** `rstest = "0"` added to `udp-core`'s dev-dependencies, as
  `swarm-coordination-registry` declares it; `Cargo.lock` gains one line and no new crate.
- **Integration selected:** No.
- **Completed-file review:** Approved by Jose Celano on 2026-10-14.

## Approval

Approved by Jose Celano on 2026-10-13: the `rstest` H1, adding `rstest`, removing the stale
`# Panics` doc in this commit, and keeping the full-label test. Guidance: use parameterized tests
only where they improve maintainability, readability, or expressiveness.

## Progress Log

- 2026-10-13 17:59 UTC - GitHub Copilot - Created the plan from `statistics/event/handler.rs`, its
  caller in `listener.rs`, and the now directly tested `metrics.rs` projections. No Rust tests or
  production code changed. R1 awaits maintainer approval.
- 2026-10-13 19:46 UTC - GitHub Copilot - Plan approved with `rstest` and the doc fix. Implemented
  R1 and measured R4 (stable Rust 1.99.0, nightly 1.101.0, `cargo-llvm-cov 0.6.16`). The plan
  awaits completed-file review.
- 2026-10-14 05:31 UTC - Jose Celano - Approved completed-file review. The file is ready for
  pre-commit and its signed per-file commit.
