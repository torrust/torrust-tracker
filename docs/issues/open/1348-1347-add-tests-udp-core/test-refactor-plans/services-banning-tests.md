---
doc-type: file-test-plan
issue: 1348
package: torrust-tracker-udp-core
target-file: packages/udp-core/src/services/banning.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-core/src/services/banning.rs
    - packages/udp-core/docs/adrs/20260829204258_use_exact_ip_counters_for_udp_banning.md
    - packages/udp-server/src/banning/event/handler.rs
    - src/bootstrap/jobs/udp_tracker_server.rs
    - docs/issues/open/1348-1347-add-tests-udp-core/coverage-evidence.md
---

# UDP Core Banning Service File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-core/src/services/banning.rs`.

## Current State

- **Unit-only coverage:** 60 / 66 lines (90.91%) at T1 and unchanged since.
- **Uncovered lines:** 46-48 (`BanService::get_banned_ips_total`) and 68-70 (the same value
  through the `BanningStats` trait).
- **Existing tests:** 5: counter increment, ban above the limit, no ban at the limit, no ban
  without errors, reset.
- **Module-owned decisions:** count invalid connection IDs per exact source IP (the package ADR
  replaced a Bloom filter to avoid collision-driven bans); ban when the count is strictly greater
  than the limit; reset clears all counters; report a total through `BanningStats`.
- **Consumers:** `udp-server`'s banning event handler (increments and publishes the total as the
  `udp_banned_ips_total` metric) and the bootstrap job (`src/bootstrap/jobs/udp_tracker_server.rs`)
  that calls `reset_bans()` periodically.
- **T1 hypothesis:** property candidate (the ban threshold).

## Current Tests Review

Conclusion: `refactor`.

- The first three tests have no Arrange/Act/Assert sections; the last two do.
- The fixture `ban_service(counter_limit)` carries a stale doc comment, "Sample service with one day
  ban duration". The service has no duration. Fix the comment and name the parameter after the
  constructor (`max_connection_id_errors_per_ip`).
- `it_should_allow_resetting_all_the_counters` asserts both that the count is gone and that the IP
  is no longer banned. The second follows from the first, so it is a redundant reason to fail;
  keep the count assertion.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Errors from one IP never count against another | 3A unit | B1: two IPs; errors from one past the limit leave the other unbanned with no count. This is the package ADR's decision and no test pins it today. |
| Strict threshold (`count > limit`) for any limit | Property (trial) | P1: for arbitrary limits and error counts (bounded to keep the test fast), `is_banned` is `count > limit`. The examples use only limit 1, so they cannot catch a limit-dependent defect such as a narrowing cast. Kept only if a temporary truncating-cast mutation passes the examples and fails P1; otherwise dropped and the reason recorded. |
| Reset clears every IP | 3A unit | Existing test (one IP); kept after R1. |
| Banned-IPs total | Finding, not tested here | See below. |

**Findings (no production change in this issue; `lessons.md` candidates):**

1. **`get_banned_ips_total()` does not count banned IPs.** It returns the number of IPs with at
   least one invalid connection ID, whether or not they exceed the limit. `udp-server` publishes it
   as `udp_banned_ips_total`, so operators see "banned IPs" that include clients that were never
   banned. `udp-server`'s own test already calls it "the distinct client IP total". This looks like
   a bug (a misleading metric), so it needs its own issue. A test here would pin the misleading
   value, so lines 46-48 and 68-70 stay uncovered until that issue decides the meaning.
2. **`last_connection_id_errors_reset` is never read.** `reset_bans()` and the constructor write it;
   nothing in the repository reads it.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Add AAA sections to the first three tests, fix the fixture comment and parameter name, drop the redundant reset assertion. | Focused tests passed; nightly formatting and `git diff --check` clean. |
| R2 | DONE | Add B1; trial P1 with the truncating-cast mutation and keep or drop it. | B1 added. P1 kept: with `is_banned` temporarily comparing through `as u8`, all 6 example tests passed and only P1 failed (limit 0, 30,976 errors); reverted by hand. 68 package unit tests passed; pedantic Clippy clean; P1 runs in about 0.04 s. |
| R3 | SKIPPED | No integration increment: the ban decision is in-memory; `udp-server` owns enforcement. | Explicit rationale. |
| R4 | DONE | Re-run unit-only coverage; record before/after. | 60 / 66 to 78 / 84 lines; see Results. |

## Results

- **Unit-only coverage (test-inclusive file totals):** 60 / 66 (90.91%) to 78 / 84 (92.86%) lines;
  168 / 176 regions; 14 / 16 functions. The six uncovered lines are exactly the banned-IPs total
  (46-48, 68-70), left for the bug issue.
- **Tests refactored:** 4 (AAA sections, fixture comment and parameter, redundant assertion).
- **Tests added:** 2: B1 (per-IP isolation) and P1 (`quickcheck` threshold property).
- **Property testing kept:** yes, with mutation evidence above.
- **Findings:** handed off as a bug and fixed by
  [#2505](../../../closed/2505-udp-banned-ips-metric/ISSUE.md) (merged in PR #2524); also recorded in
  [lessons.md](../lessons.md).
- **Integration selected:** No.
- **Completed-file review:** Approved by Jose Celano on 2026-10-13.

## Approval

Approved by Jose Celano on 2026-10-13: R1, B1, the P1 trial with its keep-or-drop rule, and
leaving the banned-IPs total untested. He asked for a bug draft and hand-off so another agent can
fix the banned-IPs total in parallel, as with #2458.

## Progress Log

- 2026-10-10 11:31 UTC - GitHub Copilot - Created the plan from `services/banning.rs`, the package
  banning ADR, the `udp-server` banning handler and its tests, and the bootstrap reset job.
  `develop` was 15 commits ahead with no `udp-core` change. No Rust tests or production code
  changed. R1 and R2 await maintainer approval.
- 2026-10-13 14:26 UTC - GitHub Copilot - Plan approved. Wrote the bug draft and hand-off in
  `.tmp/udp-banned-ips-metric/`. Implemented R1 and R2, kept P1 after the mutation trial, and
  measured R4 (stable Rust 1.99.0, nightly 1.101.0, `cargo-llvm-cov 0.6.16`). The plan awaits
  completed-file review.
- 2026-10-13 15:03 UTC - Jose Celano - Approved completed-file review. For the handed-off bug he
  chose Option A: in `disabled` validation mode, keep counting IPs over the limit, so operators know
  how many clients would be banned if enforcement were on. The stale gauge after a reset is part of
  the same bug.
- 2026-10-14 14:43 UTC - GitHub Copilot - The handed-off bug became
  [#2505](../../../closed/2505-udp-banned-ips-metric/ISSUE.md), merged in PR #2524. Rebased #1348 onto it: the
  only conflict was the test module imports (`quickcheck` types from #2505, the `quickcheck` macro
  from this plan); both kept. All 17 banning tests pass.
