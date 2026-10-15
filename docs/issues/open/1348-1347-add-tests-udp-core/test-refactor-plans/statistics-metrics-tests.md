---
doc-type: file-test-plan
issue: 1348
package: torrust-tracker-udp-core
target-file: packages/udp-core/src/statistics/metrics.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-core/src/statistics/metrics.rs
    - packages/udp-core/src/statistics/mod.rs
    - packages/udp-core/src/statistics/event/handler.rs
    - packages/udp-server/src/statistics/metrics.rs
    - docs/issues/open/1348-1347-add-tests-udp-core/coverage-evidence.md
---

# UDP Core Statistics Metrics File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-core/src/statistics/metrics.rs`. It is processed before ledger order 10
(`statistics/event/handler.rs`), with maintainer approval, because the handler's per-family tests
are currently the only coverage of this file's projections.

## Current State

- **Unit-only coverage:** 56 / 65 lines (86.15%) at T1 and unchanged since.
- **Uncovered lines:** 35-43, `Metrics::set_gauge`, which has no caller in the repository.
- **Existing tests:** none in this file. The six projections are exercised only through the six
  per-family tests in `statistics/event/handler.rs`, one event each.
- **Module-owned decisions:** the six `udp{4,6}_{connections,announces,scrapes}_handled`
  projections. Each sums the requests-received counter over every series whose
  `server_binding_address_ip_family` and `request_kind` labels match, and returns 0 when no series
  matches. They feed the REST API stats (`udp4_connections_handled` and the rest).
- **T1 hypothesis:** test candidate.

## Current Tests Review

No tests in this file. R1 is skipped.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Each projection selects its own family and request kind | 3A unit | M1: one collection with a distinct count for each of the six (family, kind) series; the six projections return exactly those counts. A swapped label in any projection fails it. |
| Projections sum across listeners | 3A unit | M2: two series with the same family and kind but different server ports (two listeners) add up. This is the "sum" decision; the handler tests use one event, so they cannot see it. |
| No matching series | 3A unit | M3: an empty, described collection projects 0 for all six. |
| `f64` sum converted to `u64` | Owned elsewhere | The cast allowance is reviewed by #2244; not tested here. |
| `set_gauge` | Finding, not tested | No caller in the repository; recorded as a removal candidate. |
| Property-based testing | Not selected | Six enumerable projections. |

Test data: build a `Metrics` with the package's own `describe_metrics()` and increase counters with
small label sets (family, request kind, port). Expectations are written as literals.

**Findings (no production change in this issue; `lessons.md` candidates):**

1. `Metrics::set_gauge` has no caller anywhere in the repository.
2. The projection docs say "from IPv4 peers" / "from IPv6 peers", but they filter on the server
   socket's address family. On a dual-stack `[::]` listener, IPv4 clients are counted as `udp6`.
   `udp-server` and the REST API field docs use the same filter and the same wording, so this is a
   cross-package doc correction, not a `udp-core` bug.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests to refactor. | Recorded above. |
| R2 | DONE | Add M1-M3. | 73 package unit tests passed; pedantic Clippy, nightly formatting, and `git diff --check` clean. With `udp6_scrapes_handled` temporarily filtering on `announce`, M1 failed (as did the handler's udp6 scrape test, its only previous coverage); reverted by hand. |
| R3 | SKIPPED | No integration increment: pure projections over an in-memory collection. | Explicit rationale. |
| R4 | DONE | Re-run unit-only coverage; record before/after. | 56 / 65 to 110 / 119 lines; see Results. |

## Results

- **Unit-only coverage (test-inclusive file totals):** 56 / 65 (86.15%) to 110 / 119 (92.44%)
  lines; 144 / 152 regions; 11 / 12 functions. The nine uncovered lines are the dead `set_gauge`.
- **Tests added:** 3 (M1-M3); the projections now have direct tests independent of the handler.
- **Findings:** both recorded in [lessons.md](../lessons.md).
- **Integration selected:** No.
- **Completed-file review:** Approved by Jose Celano on 2026-10-13.

## Prose-First AAA Review

Recorded on 2026-10-15, late: the PR #2538 Task Reviewer found no record of it (AC7). For each
test, temporary Arrange, Act, and Assert prose was compared with the code and then removed.

| Test | Arrange → Act → Assert | Outcome |
| --- | --- | --- |
| M1 `it_should_count_in_each_projection_only_its_own_ip_family_and_request_kind` | Counts 1-6 per (family, kind) → six projections → `[1, 2, 3, 4, 5, 6]` | No change |
| M2 `it_should_add_up_in_a_projection_the_requests_received_by_every_listener` | IPv4 announces on two ports (2, 3) → `udp4_announces_handled` → 5 | No change |
| M3 `it_should_project_zero_everywhere_when_no_request_was_recorded` | Described metrics only → six projections → all 0 | No change |

## Approval

Approved by Jose Celano on 2026-10-13: M1-M3 and recording the two findings.

## Progress Log

- 2026-10-13 17:19 UTC - GitHub Copilot - Created the plan from `statistics/metrics.rs`, the
  handler tests that call it, the matching `udp-server` projections, and the REST API field docs.
  No Rust tests or production code changed. R2 awaits maintainer approval.
- 2026-10-13 17:38 UTC - GitHub Copilot - Plan approved. Implemented R2 and measured R4 (stable Rust
  1.99.0, nightly 1.101.0, `cargo-llvm-cov 0.6.16`). The plan awaits completed-file review.
- 2026-10-13 17:52 UTC - Jose Celano - Approved completed-file review. The file is ready for
  pre-commit and its signed per-file commit.
- 2026-10-15 10:30 UTC - GitHub Copilot - The PR #2538 review (F6) gave M1-M3 the `it_should_`
  prefix ("test(udp-core): name and structure the statistics tests by the unit-test
  conventions"). After the Task Reviewer found no prose-first record, recorded the review above.
  No further change.
