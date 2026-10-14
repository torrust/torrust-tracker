---
schema-version: 1
doc-type: issue
issue-type: bug
status: in-progress
priority: p2
epic: null
github-issue: 2505
spec-path: docs/issues/open/2505-udp-banned-ips-metric/ISSUE.md
branch: "2505-udp-banned-ips-metric"
related-pr: 2524
last-updated-utc: "2026-10-14 14:26"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - write-unit-test
    - run-benchmarks
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - docs/issues/open/2505-udp-banned-ips-metric/manual-verification-evidence.md
    - docs/issues/open/2505-udp-banned-ips-metric/udp-banned-ips-probe.sh
    - docs/issues/open/2505-udp-banned-ips-metric/evidence/tracker.log
    - docs/issues/open/2505-udp-banned-ips-metric/evidence/recheck-tracker.log
    - docs/issues/open/2505-udp-banned-ips-metric/evidence/final-recheck-tracker.log
    - docs/issues/open/2505-udp-banned-ips-metric/evidence/last-recheck-tracker.log
    - docs/issues/open/2505-udp-banned-ips-metric/agent-review-reports.md
    - docs/issues/open/2505-udp-banned-ips-metric/performance-evidence.md
    - docs/issues/open/2505-udp-banned-ips-metric/implementation-retrospective.md
    - docs/testing.md
    - tests/AGENTS.md
    - packages/udp-core/src/services/banning.rs
    - packages/udp-core/benches/ban_service_benchmark.rs
    - packages/udp-core/docs/adrs/20260829204258_use_exact_ip_counters_for_udp_banning.md
    - packages/udp-server/src/banning/event/handler.rs
    - packages/udp-server/src/banning/event/listener.rs
    - packages/udp-server/src/statistics/metrics.rs
    - packages/udp-server/src/statistics/mod.rs
    - packages/rest-api-protocol/src/v1/context/stats/resources/stats.rs
    - src/bootstrap/jobs/udp_tracker_server.rs
---

<!-- skill-link: create-issue -->

# Issue #2505 - The UDP Banned-IPs Metric Counts Clients That Are Not Banned

## Goal

Make the "banned IPs" metric report what its name and descriptions say, and keep it correct after
the periodic ban reset.

## Identifiers

Several short identifiers share a letter. In this specification and its issue-local documents:

| Identifier | Meaning | Defined in |
| ---------- | ------- | ---------- |
| Options P1-P4 | Performance alternatives for computing the banned total (option P1 was chosen) | "Performance Alternatives" |
| Baseline, after | The before-and-after performance measurements (earlier text and `performance-evidence.md` command labels call them P1 and P2) | "Performance Evidence Plan" |
| G1-G3 | Options for publishing the gauge on reset | "Gauge Update on Reset" |
| R1-R6, R1p | Regression tests | "Regression Test Strategy" |
| T1-T10, benchmark task B1 | Implementation tasks | "Implementation Plan" |
| Scenarios M1-M4 | Manual verification scenarios | "Manual Verification Scenarios" |
| V1-V4, R-V1-R-V4, F-V1-F-V4, L-V1-L-V4 | Reproduction, first recheck, second recheck (earlier called final), and last recheck on the final code | `manual-verification-evidence.md` |
| Mutations M0-M5, M1b, M3b | Code mutations for mutate-then-restore (their `== M...` labels are part of recorded command output) | `manual-verification-evidence.md` "Mutate-Then-Restore" sections |
| Task review B1, B2, N1-N11 | Findings of the independent pre-PR task review | `agent-review-reports.md` |
| F1-F14 | Pull-request review findings | `docs/pr-reviews/pr-2506-review/` and `docs/pr-reviews/pr-2524-review/` |

Progress Log entries are historical and keep the identifiers used when they were written.

## Background

### How UDP banning works

`packages/udp-core/src/services/banning.rs` defines `BanService`. It keeps an exact map from client
IP to the number of invalid connection IDs that IP sent (the package ADR
`packages/udp-core/docs/adrs/20260829204258_use_exact_ip_counters_for_udp_banning.md` replaced a
Bloom filter with this map). `is_banned(ip)` is true when the count is **strictly greater than**
`max_connection_id_errors_per_ip` (configuration `udp_tracker_server.max_connection_id_errors_per_ip`,
default 10). `udp-server`'s launcher drops requests from banned IPs when connection-ID validation is
`strict` (`packages/udp-server/src/server/launcher.rs`). A bootstrap job calls `reset_bans()` every
`ip_bans_reset_interval_in_secs` (minimum 3600, default 86400) and clears the whole map
(`src/bootstrap/jobs/udp_tracker_server.rs`).

### The bug, in plain terms

1. A client sends one request with an invalid connection ID.
2. `udp-server`'s banning event handler (`packages/udp-server/src/banning/event/handler.rs`) calls
   `ban_service.increase_counter(ip)`, then `ban_service.get_banned_ips_total()`, and stores the
   result in the gauge `udp_tracker_server_ips_banned_total`.
3. `get_banned_ips_total()` returns `accurate_error_counter.len()`: the number of IPs with **at least
   one** invalid connection ID, not the number whose count exceeds the limit. Both the inherent
   method and the `BanningStats` trait implementation do this.
4. So after step 1 the gauge reads 1, although nothing is banned: with the default limit, the client
   needs 11 invalid IDs before `is_banned` turns true.

The REST stats field `udp_banned_ips_total` is read from that same gauge
(`packages/udp-server/src/statistics/metrics.rs` `Metrics::udp_banned_ips_total`, mapped by
`packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs`), so the JSON stats, the Prometheus
stats, and the Prometheus metrics all show the wrong value.

**Contracts violated** (all say "banned"):

- `BanningStats::get_banned_ips_total` doc: "Returns the total number of banned IPs." The inherent
  `BanService::get_banned_ips_total` had no doc comment, so its contract was only the trait's, and
  the `BanService::reset_bans` doc ("Resets the counters and updates the reset timestamp.") becomes
  false once the write-only timestamp field is removed. (Corrected after PR #2506 review finding F7;
  an earlier version said both methods carried the sentence.)
- Metric description (`packages/udp-server/src/statistics/mod.rs`): "Total number of IPs banned from
  UDP requests" (Prometheus `udp_tracker_server_ips_banned_total`).
- `udp-server` `Metrics::udp_banned_ips_total` doc: "Total number of banned IPs."
- REST API stats field `udp_banned_ips_total`
  (`packages/rest-api-protocol/src/v1/context/stats/resources/stats.rs`): "Total number of IPs
  banned for UDP (UDP tracker) requests."

**Second defect in the same value:** the periodic reset clears the map but does not update the
gauge. The gauge is only written when the next connection-cookie error arrives, so after a reset it
keeps reporting the old total, on every surface listed above.

**Impact today:** operators reading the REST stats or Prometheus see an inflated "banned IPs" count
(every client that sent at least one bad connection ID since the last reset), and after a reset they
see a stale value until the next invalid connection ID arrives. Ban enforcement itself is correct;
only the reported number is wrong.

**Reproduced** on `develop` at `1c5939c76` against the running tracker (see
[manual-verification-evidence.md](manual-verification-evidence.md)): three IPs that sent one invalid
connection ID each, plus one IP that was really banned, made all three surfaces report 4 banned IPs
while exactly one IP was banned.

**Evidence that the mismatch was already noticed but not tracked:** `udp-server`'s handler test is
named `it_should_publish_the_distinct_client_ip_total_after_a_connection_cookie_error` and uses
`BanService::new(1)` with one error per IP, so no IP in it is banned.

### Leftover code found while analyzing the bug

- **`BanningStats` trait (no consumers).** SI-30 (#1924, commit `637c17b18`, 2026-06-23) added the
  trait so the REST API could read the banned count without depending on `BanService`. That commit
  added no consumer. SI-4 (#1942, commit `643487109`, 2026-06-26) then chose to aggregate stats in
  `TrackerStatsAdapter` from the `udp-server` gauge instead, so the trait was never consumed. #1924,
  #1942, and their EPIC #1938 are closed, so this is a leftover, not an unfinished refactor. Its
  implementation duplicates the buggy inherent method. The maintainer decided to remove it.
- **`BanService::last_connection_id_errors_reset` (write-only field).** Added in `10f9bdaac`
  (#1096, 2024-12-09). The constructor and `reset_bans()` write it; nothing has ever read it. The
  maintainer decided to remove it.

### Origin

Found while planning `packages/udp-core/src/services/banning.rs` tests in Issue #1348 (EPIC #1347).
`get_banned_ips_total` was added with its current body (`self.accurate_error_counter.len()`) by
`1ce2e3327` (`feat: [#1145] add banned ips total for UDP to stats`, 2025-01-07), in
`src/servers/udp/server/banning.rs`; the package extraction (`1886593b4`, 2025-02-18) only moved
the file. So the value has been wrong since the metric was introduced. The trait wrapper came with
`637c17b18` (2026-06-23). (Corrected after PR #2506 review finding F9; an earlier version dated the
method from the extraction.)

## Decision: Meaning of "Banned" When Validation Is `disabled`

In `disabled` mode the service still counts invalid connection IDs (for observability) but the
launcher enforces nothing. Options considered:

- **A.** Report IPs whose count exceeds the limit in both modes, and document that in `disabled`
  mode they are "would-be banned" (not enforced).
- **B.** Report 0 in `disabled` mode.
- **C.** Keep the current distinct-IP count under a new, honest metric name and add a separate,
  correct banned count. Breaking change to the metrics and REST contract.

**Decided (maintainer, 2026-10-13): A.** It fixes the meaning without renaming the metric or the REST
field. The documentation listed under "Contracts violated" must state the `disabled`-mode meaning.

## Scope

### In Scope

- Make `BanService::get_banned_ips_total()` return the number of IPs whose error count exceeds the
  limit, in both `strict` and `disabled` mode.
- Update the gauge when bans are reset, so it never reports a stale total.
- Fix the documentation listed under "Contracts violated" to match decision A: add a doc comment
  to the inherent `BanService::get_banned_ips_total` (the threshold and the `disabled`-mode
  meaning), update the `BanService::reset_bans` doc, and correct the metric, `Metrics`, and REST
  field descriptions.
- Remove the unused `BanningStats` trait and its implementation.
- Remove the write-only `BanService::last_connection_id_errors_reset` field.
- Adjust the `udp-server` unit tests that pin the old value, keeping their intent:
  - the handler test `it_should_publish_the_distinct_client_ip_total_after_a_connection_cookie_error`;
  - the listener tests in `packages/udp-server/src/banning/event/listener.rs` that use
    `get_banned_ips_total()` as a proxy for "the event was processed" (switch them to
    `get_count(ip)` or another direct observation).
- Measure performance before and after the change as the "Performance Evidence Plan" describes
  (a new per-event Criterion benchmark, the existing `udp_ban_service` benchmarks, and the
  end-to-end load test, recorded in `performance-evidence.md`).

### Out of Scope

- Changing the ban threshold semantics (`count > limit`), the reset interval, or enforcement.
- Renaming the metric or the REST field.
- Tests that already send errors until the IP is really banned and assert `+1`: the `udp-server`
  contract test `should_ban_the_client_ip_if_it_sends_more_than_10_requests_with_a_cookie_value_not_normal`
  (`packages/udp-server/tests/server/contract.rs`) and the root tests under `tests/banning/`. They are
  valid under both meanings and must keep passing unchanged.
- Issue #1348's test work in `banning.rs` beyond what this fix needs.
- The other SI-30 traits (`UdpCoreStatsRepository`, `UdpServerStatsRepository`). They may have the
  same leftover status; check them in a separate issue if wanted.

## Architectural Decisions

- Related ADRs: `packages/udp-core/docs/adrs/20260829204258_use_exact_ip_counters_for_udp_banning.md`.
- ADRs to create: none expected with decision A. If the implementer chooses an approach that changes
  where the gauge is published (see "Gauge Update on Reset"), assess whether it needs an ADR.

## Design and Ownership Review

- `udp-core` (`BanService`) owns the per-IP count, the threshold, and the banned total.
- `udp-server` owns publishing the gauge `udp_tracker_server_ips_banned_total`.
- The root crate's bootstrap job owns only the reset schedule.

### Performance Alternatives for Computing the Banned Total

The banning event handler reads the banned total after **every** invalid connection ID, while
holding the `BanService` write lock. The launcher takes the read lock for `is_banned` on every
incoming request. So whatever the total costs is paid on the error path and can delay the request
path. The map grows with every distinct source IP until the next reset (default 24 h), and UDP
source addresses can be spoofed, so the map can become large.

The maintainer asked for the preferred option and the alternatives to be documented here and
reconsidered when implementation starts. One constraint is fixed (AC7, maintainer decision on PR
review finding `review-finding:pr-2506-f1`): the work done on each invalid-connection-ID event to
maintain and read the banned total must not grow with the number of tracked IPs. This rules out P2;
P1, P3, and P4 remain open.

**P1 (preferred): maintain a banned-IP counter.** `increase_counter` increments a `banned_ips_total`
field when an IP's count goes from `limit` to `limit + 1` (the moment `is_banned` turns true).
`reset_bans()` sets it to 0.

- Pros: O(1) per error and per read; no extra per-IP memory; read is a field access.
- Cons: a second piece of state that must stay consistent with the map. Any future code that removes
  or decays a single IP's count must also update the counter. Needs tests at the crossing boundary
  (`limit`, `limit + 1`, `limit + 2` must count once).

**P2 (ruled out by AC7): filtered scan on read.** `get_banned_ips_total()` returns
`map.values().filter(|count| **count > limit).count()`.

- Pros: smallest change; no new state; obviously consistent with `is_banned`.
- Cons: O(number of tracked IPs) per invalid connection ID, under the write lock. A flood of
  invalid IDs from many (spoofed) source IPs makes total work grow quadratically and lengthens the
  time the launcher waits for the read lock.

**P3: keep a `HashSet` of banned IPs.** Insert the IP when it crosses the limit; clear on reset;
the total is the set length.

- Pros: O(1) per error and read; would also allow listing banned IPs later.
- Cons: extra memory per banned IP and duplicate state with the same consistency risk as P1, for a
  feature nobody has asked for.

**P4: publish the gauge periodically instead of per error.** A timer (or the stats read) computes
the total with a scan.

- Pros: removes the cost from the error path entirely.
- Cons: the gauge lags by up to one period; adds a job or a new pull path from the REST adapter;
  changes ownership more than the bug needs.

**Is performance testing needed to choose?** No: the complexity difference (O(1) versus O(n) per
error) is clear without measurement. The change still touches the UDP request path (the launcher's
`is_banned` read lock) and the error path, so it needs the before-and-after evidence below. If
option P4 is chosen, also add a benchmark case for its scan at high cardinality, so the scan cost is
measured, not assumed: the per-event benchmark and the load test do not exercise it. (Restored
after PR #2506 review finding F13; the implementation chose option P1, so it did not apply.)

### Performance Evidence Plan

Follows [docs/benchmarking.md](../../../benchmarking.md) ("Choosing a Benchmark" and "Recording
Before-and-After Evidence") and the `run-benchmarks` skill, with issue #2458 as the worked example.

- **Microbenchmark (precise):** the existing Criterion groups `udp_ban_service/increase_counter`
  and `udp_ban_service/is_banned` in `packages/udp-core/benches/ban_service_benchmark.rs`. No
  benchmark covers the per-event step the handler runs (`increase_counter` followed by reading the
  banned total), so **benchmark task B1** adds one, `udp_ban_service/record_error`, measured at the existing
  `CARDINALITIES` of tracked IPs, in its own commit before any production change. Its medians
  across cardinalities are also the AC7 evidence: they must not grow with the number of tracked IPs.
- **End-to-end load test (user-visible):** `aquatic_udp_load_test` against a release build started
  with `share/default/config/tracker.udp.benchmarking.toml`, following
  [E2E UDP load testing](../../../benchmarking.md#e2e-udp-load-testing) with the issue #2342
  settings (connect and announce weights 50/50, scrape weight 1, 30-second runs, last 20 seconds
  summarized), five runs per measurement, a fresh tracker per run. It exercises the launcher's
  `is_banned` check on every request; it sends valid connection IDs, so the error path itself is
  covered by the microbenchmark.

Method:

- **Baseline (T3)** runs after benchmark task B1 and before any production change: three Criterion runs (each
  benchmark's median read from `target/criterion/<group>/<function>/new/estimates.json`) and five
  load-test runs. **After (T9)** repeats the same measurements on the finished code, on the same
  machine, with the same build profile, configuration, and load-test settings. The benchmark code
  may change between the baseline and the after measurement only as far as changed signatures
  require.
- Check the machine before each measurement (`uptime`, other builds) and record it.
- Record the machine, toolchain, aquatic commit, commands, configuration, every run, and the
  observed noise in issue-local `performance-evidence.md`, with times taken from output files or
  `git log`.
- **Pass rule (one-sided):** the after load-test mean responses per second is not below the lowest
  baseline run, and each after Criterion median is not above the highest of the three baseline
  medians for the same benchmark. State the observed baseline noise, so readers know which regressions the measurement can
  detect. A failure is investigated, for example with [docs/profiling.md](../../../profiling.md),
  before the issue closes.

### Gauge Update on Reset

Today `reset_bans()` runs in the root crate's job, which has no metrics code. Options:

- **G1 (preferred):** add a function to `udp-server`'s banning module that resets the bans and
  publishes the new total (0) through the same path as the event handler. The root job calls it on
  each tick. Publishing stays in `udp-server`; the job stays a scheduler. R3 becomes a `udp-server`
  unit test.
- **G2:** the root job calls `reset_bans()` and then sets the gauge itself. Simple, but it duplicates
  `udp-server`'s publishing code in the root crate.
- **G3:** `reset_bans()` returns the new total and the caller publishes it. Still needs a caller that
  publishes, so it reduces to G1 or G2.

Record the chosen option and why in the progress log.

## Bug-Fix Process

Follow `.github/skills/dev/debugging/fix-bug/SKILL.md`:

1. **Analysis:** done above, re-verified on `develop` at `1c5939c76`.
2. **Reproduction:** done before spec review. The tracker was started from a dedicated configuration
   and probed with raw BEP 15 packets sent from chosen loopback source IPs
   ([udp-banned-ips-probe.sh](udp-banned-ips-probe.sh)). The REST JSON stats, the Prometheus stats,
   and the Prometheus metrics were read after each step. Classified outcome and full record:
   [manual-verification-evidence.md](manual-verification-evidence.md).
3. **Regression tests:** see below; prove each red before the fix.
4. **Fix**, then **green and recheck**: repeat the reproduction with the same script and
   configuration.

## Regression Test Strategy

### Layer Evaluation

The two defects live at different points of the path from the invalid request to the reported
number, so no single test layer protects both. The layers below follow
[docs/testing.md](../../../testing.md) ("test as close to the code as possible") and
[tests/AGENTS.md](../../../../tests/AGENTS.md) (root tests only for application orchestration).

```text
UDP request -> udp-server handler -> UdpError event -> banning listener -> BanService count (D1)
                                                                        -> gauge -> REST / Prometheus
root reset job (every >= 3600 s) -> BanService::reset_bans() -X-> gauge never updated (D2)
```

| Layer                                         | Fit       | What it protects here                                                                                                                                                                                                                                       |
| --------------------------------------------- | --------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Unit, `udp-core` (`BanService`)               | Required  | D1 at its causal seam: the threshold boundary of the banned total, and the total after reset. Fast and deterministic.                                                                                                                                      |
| Property-based unit, `udp-core`               | Required  | The invariant "banned total == number of IPs for which `is_banned` is true" for arbitrary error sequences, limits, and resets. Most valuable with options P1 and P3, where a second piece of state must stay consistent with the map.                                 |
| Unit, `udp-server` (banning module)           | Required  | The new reset-and-publish function (G1) writes the new total to the gauge.                                                                                                                                                                                 |
| Unit, root crate (reset job)                  | Required  | D2's actual cause is wiring: the job resets the bans but never publishes. A job test with a paused Tokio clock (precedent: `src/bootstrap/jobs/torrent_cleanup.rs`) advances one interval and reads the gauge. This is the test that is red on `develop`. |
| Package-level in-process integration, `udp-server` | Required  | The published gauge through the real server, event bus, banning listener, and stats repository (`packages/udp-server/tests/server/contract.rs` environment). It is the closest automated equivalent of the reported symptom; the handler unit test injects a hand-made event and does not cover this wiring. |
| Root application-level integration            | Not added | The REST field is a 1:1 mapping of the same gauge (`TrackerStatsAdapter`), already exercised by `tests/banning/*` for the banned case. A below-limit scenario would need a new binary for an isolated ban service, and the reset path cannot be driven there (minimum interval 3600 s, real clock). |
| Executable-boundary, container E2E, qBittorrent E2E | Not applicable | No process, image, or client-interoperability behavior changes.                                                                                                                                                                                    |
| Criterion benchmark and UDP load test         | Required  | Performance regression on the error path and the request path, see "Performance Evidence Plan". Not correctness tests.                                                                                                                                      |
| Manual verification                           | Required  | V1-V4 rerun on the fixed build with the same script and configuration, including the real reset after 3600 s, which no automated layer drives end to end.                                                                                                  |

### Planned Tests

- **R1 (unit, `udp-core`):** with limit `L`, an IP with `L` errors is not counted as banned; an IP
  with `L + 1` errors is counted once; an IP with `L + 2` errors is still counted once; several IPs
  below the limit report 0. **Red on `develop`** (the current total counts every IP).
- **R2 (unit, `udp-core`):** after `reset_bans()`, the banned total is 0 and an IP must exceed the
  limit again to be counted. Three cases: the reset leg and the re-crossing case (an IP that exceeds
  the limit again after a reset is counted once) pass on `develop` (the map is cleared, and one IP is
  one entry), so prove them with mutate-then-restore (for option P1: do not reset the counter). The
  below-limit case (an IP that was banned, reset, and then stays within the limit is not counted) is
  red on `develop`, where any IP with errors after the reset is counted. (Narrowed after PR #2506
  review finding F10, and corrected after PR #2524 review finding F7 by Cameron: the below-limit
  case was missing, so the earlier note called a passing case red.)
- **R1p (property-based unit, `udp-core`):** for arbitrary sequences of
  `increase_counter` over a small IP pool, interleaved with `reset_bans()`, and arbitrary limits,
  the banned total equals the number of IPs for which `is_banned` is true. Needs `quickcheck` as a
  `udp-core` dev-dependency (already used by `udp-protocol`); follow the `add-rust-dependency` skill
  and coordinate with Issue #1348, which may add the same dependency. Prove it guards the fix with
  mutate-then-restore.
- **R3 (unit, `udp-server`):** the reset-and-publish function sets the gauge to 0 when it was
  non-zero. Prove it with mutate-then-restore (remove the publish call).
- **R4 (unit, root crate, reset job):** with a paused Tokio clock, start the job with a ban service
  whose gauge was published as non-zero, advance one interval, and assert the gauge reads 0. **Red on
  `develop`** (the job never publishes).
- **R5 (package-level integration, `udp-server`, `strict`):** from one client, send `L` invalid
  connection IDs: the published `udp_banned_ips_total()` is 0. Send one more: it is 1. **Red on
  `develop`** for the first assertion (it reads 1).
- **R6 (package-level integration, `udp-server`, `disabled`):** pins decision A. From
  one client, `L` invalid connection IDs report 0; `L + 1` report 1, and the client's requests are
  still served. **Red on `develop`** for the first assertion.
- **Updated tests:** the `udp-server` handler and listener unit tests that pin the distinct-IP count
  (see Scope). Record each change in the progress log.
- **Unchanged guards:** the existing contract test
  `should_ban_the_client_ip_if_it_sends_more_than_10_requests_with_a_cookie_value_not_normal` and
  `tests/banning/*` send errors until the IP is really banned and must keep passing unchanged.

Each scenario keeps one observable contract and one reason to fail (tests/AGENTS.md "Scenario
Design"). If R5 and R6 need a different validation policy than the shared contract-test environment
provides, use a separate environment per policy rather than one combined scenario.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                               | Notes / Expected Output                                                                                         |
| --- | ------ | -------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Reproduce and record                               | [manual-verification-evidence.md](manual-verification-evidence.md) V1-V4, outcome **Reproduced**.               |
| T2  | DONE   | Maintainer decisions                               | Decision A; remove `BanningStats`; remove `last_connection_id_errors_reset`; document performance alternatives. |
| B1  | DONE   | Add the per-event microbenchmark                   | `udp_ban_service/record_error` in `ban_service_benchmark.rs`, committed before any production change.          |
| T3  | DONE   | Performance baseline                               | Three Criterion runs and five `aquatic_udp_load_test` runs before any production change, in `performance-evidence.md`. |
| T4  | DONE   | R1, R5, and R6 regression tests, red               | Recorded failing output against the current implementation.                                                    |
| T5  | DONE   | `udp-core` fix and dependent test updates          | Chosen performance option; R1, R5, R6 green; R2 and R1p added and proven by mutate-then-restore; leftover trait and field removed; the `udp-server` handler and listener unit tests that pin the distinct-IP count updated in the same change, so the whole workspace test suite passes. |
| T6  | DONE   | R4 regression test, red                            | Recorded failing output showing the stale gauge after one reset interval.                                       |
| T7  | DONE   | Gauge update on reset                              | Chosen option (G1 preferred); R4 green; R3 added and proven by mutate-then-restore.                             |
| T8  | DONE   | Documentation updates                              | Contracts listed under "Contracts violated" corrected for decision A.                                           |
| T9  | DONE   | Performance after the fix                          | Same measurements as T3; comparison against the one-sided pass rule in `performance-evidence.md`.              |
| T10 | DONE   | Verification, like-for-like recheck, and AC review | Automatic checks; V1-V4 rerun with the same script and configuration; acceptance criteria re-reviewed.          |

## Commit Points

| Task   | Coherent change set                                                  | Commit policy                                       |
| ------ | -------------------------------------------------------------------- | --------------------------------------------------- |
| T5 (leftovers) | Removal of the unused `BanningStats` trait and the write-only `last_connection_id_errors_reset` field, with the `reset_bans` doc update (no behavior change) | One `refactor(udp-core)` commit before the fix, green on its own. |
| T4-T5  | `udp-core` banned-count fix, R1, R1p, R2, R5, R6, and the `udp-server` handler and listener test updates that depend on the new count | One commit, green on the whole workspace test suite, after focused validation and design review. |
| T6-T7  | Gauge-on-reset fix with R3 and R4                                    | Commit after focused validation and design review.  |
| T8     | Documentation updates                                                | Commit after focused validation.                    |
| B1     | `udp_ban_service/record_error` benchmark                             | One `test(udp-core)` commit before T3.              |
| T3     | Baseline in `performance-evidence.md`                                | One `docs(issues)` commit before any production change. |
| T9     | After results and comparison in `performance-evidence.md`               | One `docs(issues)` commit.                          |
| T10    | Recheck evidence and acceptance review                               | Commit after maintainer review.                     |

Record the red output of T4 and T6, and the mutate-then-restore output for R1p, R2, and R3, before
committing the fixes. Every commit point must leave the workspace test suite green: a test that
asserts behavior a commit changes is updated in that same commit. For test-producing work, use the
`write-unit-test` skill and complete an explicit design review after each passing test increment,
before maintainer review and commit. Confirm that the test exposes the one causal initial-state
difference; its fixture owns only incidental mechanics; and the production Act plus independently
specified expected result remain visible. Use the mandatory prose-first Arrange-Act-Assert
comparison: write temporary prose for each section, refactor until the code expresses it, remove
redundant prose, and retain only irreducible context. Assess helper boundaries by meaningful named
actions and abstraction-level alignment, not caller count. Use Conventional Commits with the narrow
scope (`udp-core`, `udp-server`), signed with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/udp-banned-ips-metric/ISSUE.md`
- [x] Reproduction attempted and classified before review (**Reproduced**)
- [x] Maintainer decisions recorded (decision A, leftover removals, performance alternatives)
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual recheck recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [x] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-13 12:04 UTC - GitHub Copilot (Issue #1348 session) - Drafted from the
  `services/banning.rs` file test plan in Issue #1348, with a hand-off document.
- 2026-10-13 15:10 UTC - GitHub Copilot (spec session) - Moved the draft into
  `docs/issues/drafts/`; re-verified the code facts on `develop` at `1c5939c76`; found that the REST
  field reads the same gauge, that `BanningStats` is an unconsumed SI-30 leftover, and that
  `last_connection_id_errors_reset` has been write-only since #1096; recorded maintainer decisions
  (A, both removals, documented performance alternatives); reproduced the bug against the running
  tracker - [manual-verification-evidence.md](manual-verification-evidence.md).
- 2026-10-13 15:41 UTC - GitHub Copilot (spec session) - At the maintainer's request, evaluated
  every test layer in `docs/testing.md` and `tests/AGENTS.md` against both defects; the plan now
  combines `udp-core` unit tests (with an optional property test), a `udp-server` unit test, a
  root reset-job unit test with a paused clock, and `udp-server` package-level integration tests,
  and records why root application-level and E2E tests are not added.
- 2026-10-13 16:09 UTC - GitHub Copilot (spec session) - Maintainer approved the spec. Recorded
  V4: after the first periodic reset the gauge stayed at 4 although nothing was banned; saved the
  full tracker log in `evidence/tracker.log`.
- 2026-10-13 16:10 UTC - GitHub Copilot (spec session) - Created GitHub issue #2505, renamed the branch to
  `2505-udp-banned-ips-metric-spec` for a spec-only PR, and moved the spec to
  `docs/issues/open/2505-udp-banned-ips-metric/`.
- 2026-10-13 16:57 UTC - GitHub Copilot (spec session) - PR #2506 review finding F1: the
  maintainer chose an outcome-based constraint; added AC7 (per-event cost must not grow with the
  number of tracked IPs), which rules out P2 and keeps P1, P3, and P4 open.
- 2026-10-13 16:59 UTC - GitHub Copilot (spec session) - PR #2506 review finding F2: added the
  Performance Evidence Plan required by `docs/benchmarking.md` for request-path changes (Criterion
  plus `aquatic_udp_load_test`, a one-sided pass rule, and issue-local `performance-evidence.md`),
  with task B1 for a missing per-event microbenchmark, and updated T3, T9, AC5, and the checks.
- 2026-10-13 17:00 UTC - GitHub Copilot (spec session) - PR #2506 review finding F3: moved the
  `udp-server` handler and listener test updates into the T4-T5 commit, because they assert the old
  distinct-IP count and would fail on that commit; T8 is now documentation only.
- 2026-10-13 17:02 UTC - GitHub Copilot (spec session) - PR #2506 review finding F4: the
  maintainer made R1p and R6 required, matching T4, T5, and the commit points. Corrected the F3
  entry's stamp from 17:01 to 17:00, the time of the commit that carries it.
- 2026-10-13 17:05 UTC - GitHub Copilot (spec session) - PR #2506 review findings F5 and F6: the probe now
  validates its arguments (committed separately), and `manual-verification-evidence.md` records
  every reproduction command exactly as it was run, including its output filters, followed by the
  verbatim output.
- 2026-10-13 17:56 UTC - GitHub Copilot (implementation session) - Spec-only PR #2506 merged into
  `develop` as `b5f1801c6`. Started implementation on branch `2505-udp-banned-ips-metric`.
- 2026-10-13 17:57 UTC - GitHub Copilot (implementation session) - B1: added the Criterion group
  `udp_ban_service/record_error` (`increase_counter` followed by `get_banned_ips_total`, at 10,
  1,000, and 10,000 tracked IPs). Instrument check with `--quick` runs: flat at about 11-14 ns on
  `develop`; with `get_banned_ips_total` temporarily replaced by a filtered scan (P2, reverted
  without staging) it grew to about 17 ns, 592 ns, and 5.6 us, so the benchmark detects an AC7
  violation. The outputs are recorded in `performance-evidence.md` with the T3 baseline.
- 2026-10-13 18:21 UTC - GitHub Copilot (implementation session) - T3: recorded the P1 baseline in
  [performance-evidence.md](performance-evidence.md) on the code before any production change:
  load-test mean 150960.00 responses/s (lowest run 148018.51, spread 5.2%) and three Criterion
  runs of all 28 `ban_service_benchmark` cases (medians within about 3%; `record_error` flat at
  10.33-10.87 ns for IPv4 and 12.27-12.58 ns for IPv6 across 10 to 10,000 tracked IPs). Shared
  desktop, load average between 2.7 and 8.2 during the runs.
- 2026-10-13 19:58 UTC - GitHub Copilot (implementation session) - The maintainer chose P1 (a maintained
  banned-IP counter) at the start of implementation. T4: R1, R5, and R6 were red on the unfixed
  code (below-limit cases read 1 or 3 instead of 0, in `strict` and `disabled` mode; stable over 8
  runs). T5: `BanService` now keeps `banned_ips_total`, incremented when an IP's count crosses the
  limit and cleared by `reset_bans()`; `BanningStats` and `last_connection_id_errors_reset` are
  removed; R1p (quickcheck, added as a `udp-core` dev-dependency with `default-features = false`;
  1.1.0 is the latest stable and already in `Cargo.lock`) and R2 added; the `udp-server` handler
  test was renamed to `it_should_publish_the_banned_ip_total_after_a_connection_cookie_error` and
  now expects 2 banned IPs, and the listener tests observe `get_count` instead of the total. All
  green; mutate-then-restore proved every guard (M1-M4). Evidence:
  [manual-verification-evidence.md](manual-verification-evidence.md) "Regression Tests".
- 2026-10-13 20:03 UTC - GitHub Copilot (implementation session) - T6: R4 (root reset-job unit test under a
  paused Tokio clock) was red on the unfixed job: the published total stayed 1 after the first
  reset. T7: chose G1, because it keeps publishing in `udp-server` and the root job a scheduler;
  added `torrust_tracker_udp_server::banning::reset_bans`, which resets and publishes through the
  handler's existing gauge helper (now `pub(crate)`), and the job calls it with
  `CurrentClock::now()`. R3 added; R3 and R4 green; M5 (no publish on reset) fails both; R4 stable
  over 8 runs. The T5 commit also made `get_banned_ips_total` a `const fn` and consumed the property
  test's `Vec` (clippy), after the green capture in the evidence; the tests were rerun green.
- 2026-10-13 20:05 UTC - GitHub Copilot (implementation session) - T8: updated the remaining contracts for
  decision A: the `udp_tracker_server_ips_banned_total` metric description (Prometheus HELP text),
  `Metrics::udp_banned_ips_total`, and the REST field `udp_banned_ips_total` now say "IPs whose
  connection ID errors exceed the limit", banned in `strict` mode and would-be banned in
  `disabled` mode (`BanService::get_banned_ips_total` was documented in T5). No other docs or
  changelog describe the value.
- 2026-10-14 05:32 UTC - GitHub Copilot (implementation session) - T10 recheck V1-V3 done on the fixed build
  (results recorded with V4). T9: the first P2 load-test attempt aborted because the recheck
  tracker held the health-check port 1313; P2 was rerun after the recheck. P2 load test passes
  (mean 162437.63 against the lowest P1 run 148018.51), but 9 of 28 Criterion medians exceed the
  highest P1 median, all six `record_error` cases among them. A back-to-back comparison with
  `develop` on the same machine showed the fix adds about 0.4-0.6 ns (about 4%) per invalid
  connection ID, the inherent cost of maintaining the counter (flat across map sizes, so AC7
  holds); the `is_banned` failures are noise in unchanged code. A one-comparison variant B
  (`Some(count) == limit.checked_add(1)`) measured as fast or faster than the first form. The
  maintainer accepted the cost, chose variant B, and asked for P2 to be rerun on the final code.
  Variant B passed the mutate-then-restore check (M1b, M2, M3b, M4) and all suites.
- 2026-10-14 06:04 UTC - GitHub Copilot (implementation session) - T9: recorded P2a (first form), the
  same-machine investigation, and the final P2 on variant B in
  [performance-evidence.md](performance-evidence.md): load-test mean 158456.71 (passes against
  148018.51); 19 of 28 Criterion medians pass, and the 9 failures are the accepted per-event cost
  (`record_error`, `increase_counter/repeated/ipv6`) or noise in unchanged `is_banned` code.
  `record_error` stays flat across 10 to 10,000 tracked IPs (AC7). T10: recorded the like-for-like
  recheck R-V1 to R-V4 (reported 0, 0, 1, and 0 after the reset, against 1, 3, 4, and a stale 4
  before) in [manual-verification-evidence.md](manual-verification-evidence.md), with the full log in
  `evidence/recheck-tracker.log`.
- 2026-10-14 06:23 UTC - GitHub Copilot (implementation session) - T10: the pre-push checks passed
  (nightly fmt, check, and docs; the stable full test suite) and `linter all` passes in the
  pre-commit gate. Reviewed every acceptance criterion against the evidence: AC1-AC4, AC6, and AC7
  met; AC5 met with the maintainer-accepted Criterion deviation. Wrote
  [implementation-retrospective.md](implementation-retrospective.md) with four proposed process
  improvements (not applied in this issue).
- 2026-10-14 07:54 UTC - GitHub Copilot (implementation session) - The independent Task Reviewer failed the
  first pre-PR review on B1: the manual recheck ran on the build before the variant-B commit. At the
  maintainer's choice, the full recheck (F-V1 to F-V4, including a new one-hour reset wait) was
  repeated on the final code: 0, 0, 1, and 0 after the reset. A first restart of that recheck was
  discarded because V2 and V3 ran concurrently by mistake (recorded in the evidence). Non-blocking
  findings: N1, the maintainer confirmed that the accepted-cost decision covers the final P2 table
  and the decision date was clarified in `performance-evidence.md`; N2, the handler test now
  includes a client at the limit, so the old meaning fails it (committed as
  `test(udp-server): [#2505] make the handler test tell banned IPs from tracked IPs`, with the M0
  proof in the evidence); N3, N5, and N6 accepted as reported; N4, the evidence commits stay as
  they are (no history rewrite) and the PR body will say so. Report:
  [agent-review-reports.md](agent-review-reports.md).
- 2026-10-14 08:48 UTC - GitHub Copilot (implementation session) - Follow-up task review: B1, N1, and N2
  verified as resolved; new findings addressed: B2, recorded the prose-first Arrange-Act-Assert
  review of the three-client handler test and marked the earlier record superseded; N7, the manual
  scenarios table now cites the final recheck F-V1 to F-V4; N8, two lessons added to the
  retrospective; N9, `client_within_the_limit_ip` renamed `client_at_the_limit_ip`.
- 2026-10-14 08:59 UTC - GitHub Copilot (implementation session) - Process mistake, corrected before any push:
  the N9 rename commit failed with a GPG timeout, but the next commit was chained in the same
  command instead of stopping, so it was signed with the staged rename included under a `docs`
  subject. At the maintainer's choice the unpushed commit was undone with `git reset --soft` and
  split into `test(udp-server): [#2505] name the third handler-test client after its state` and this
  documentation commit.
- 2026-10-14 09:03 UTC - GitHub Copilot (implementation session) - Second follow-up task review: PASS (B2,
  N7, N8, N9 verified; optional N10 and N11). Rewrapped the long evidence line (N10), added the
  last three lessons to the completion-review summary and the chained-commit lesson to the
  retrospective (N11), and ticked the reviewer and committer checkpoints.
- 2026-10-14 09:46 UTC - GitHub Copilot (implementation session) - PR #2524 review (Copilot) finding F1: the
  reset path introduced a second gauge publisher, and both released the `BanService` lock before
  publishing, so concurrent writes could land out of order. Both paths now hold the lock until the
  gauge is written; two lock-ordering regression tests were red before and green after (evidence:
  "PR Review Follow-Up - Gauge Publication Ordering").
- 2026-10-14 09:58 UTC - GitHub Copilot (implementation session) - PR #2524 review (Cameron) finding F3: a
  per-IP count that wraps would be counted again. `increase_counter` now stops at `u32::MAX` with
  `checked_add` (the suggested `saturating_add` double-counts when the limit is `u32::MAX - 1`);
  two boundary tests seed the count through the private map and fail for both variants (evidence:
  "PR Review Follow-Up - Error Count Overflow").
- 2026-10-14 10:00 UTC - GitHub Copilot (implementation session) - Correction (PR #2524 review finding F1 by
  Cameron): the T3 entry above says the P1 Criterion medians were "within about 3%". Measured as
  (highest - lowest) / mean, like the load-test spread, the P1 spread is up to 5.1%, and 9 of 28
  benchmarks exceed 3%; 3.0% was the largest deviation from the mean. `performance-evidence.md` now
  states the measure and the 5.1% figure, and says that attributing the P2 excesses to the counter
  or to noise rests on the same-machine comparison, not on the P1/P2 tables. AC5's conclusion is
  unchanged.
- 2026-10-14 10:03 UTC - GitHub Copilot (implementation session) - Late PR #2506 review (Cameron, findings
  F7 and F9, handled on this branch with maintainer approval
  <https://github.com/torrust/torrust-tracker/pull/2506#issuecomment-6165699972>) and PR #2524
  finding F4 by Cameron: corrected the "Contracts violated" bullet (the inherent method had no doc
  comment), Scope, AC3, and its evidence row; dated `get_banned_ips_total` from `1ce2e3327` (#1145)
  and noted in "Risks and Trade-offs" that the value was wrong since the metric was introduced; and
  made the AC4 evidence say that no code reference remains, keeping the open EPIC's history row.
- 2026-10-14 10:04 UTC - GitHub Copilot (implementation session) - Late PR #2506 review findings F10, F13, and
  F14 (same approval): R2's note now says only its reset leg passes on `develop`; the P4 scan
  benchmark condition is restored in "Performance Alternatives" (not applicable, option P1 was
  chosen); Scope and "Risks and Trade-offs" now point at the Performance Evidence Plan.
- 2026-10-14 10:06 UTC - GitHub Copilot (implementation session) - PR #2524 finding F2 by Cameron and late PR
  #2506 finding F12: added an "Identifiers" glossary, renamed the measurement phases P1 and P2 to
  baseline and after in the current text of this specification, `performance-evidence.md`, and the
  retrospective, and qualified scenario, mutation, benchmark-task, and task-review citations.
  Progress Log entries and recorded command output keep their original identifiers.
- 2026-10-14 10:09 UTC - GitHub Copilot (implementation session) - Process mistake, corrected before any push:
  the `performance-evidence.md` part of the identifier change was swept into
  `docs(issues): [#2505] correct the contract, origin, and AC4 statements` by `git add -A` on the
  whole issue folder. At the maintainer's choice, that commit and the next one were rebuilt (stash,
  `git reset --soft`, recommit each from its original `ISSUE.md`), so the rename now belongs to the
  identifier commit.
- 2026-10-14 10:12 UTC - GitHub Copilot (implementation session) - Late PR #2506 review finding F8 (same
  approval): the reproduction evidence said invalid connection IDs are not logged; it now quotes
  the 15 `WARN` lines from `evidence/tracker.log` per process, in condensed form, with the script
  that condensed them.
- 2026-10-14 10:25 UTC - GitHub Copilot (implementation session) - Late PR #2506 review finding F11 (same
  approval; the maintainer chose to act on it and accepted rewriting the unmerged PR #2524 history):
  split the leftover removals out of `fix(udp-core): [#2505] count only IPs over the limit as
  banned` into `refactor(udp-core): [#2505] remove the unused BanningStats trait and the write-only
  reset timestamp`, placed before it. Interactive rebase with an `edit` stop; the refactor-only
  state passed `cargo check --workspace --all-targets`, the `udp-core` and `udp-server` tests,
  nightly fmt, clippy (pedantic), and the pre-commit hook; the recommitted fix has the original
  tree, and the final tree is byte-identical to the pre-split head. Every later commit has a new id,
  so the commit ids cited in the PR #2524 reviews are mapped to subjects in that pull request's
  audit. Commit Points and the AC4 evidence now name the refactor commit.
- 2026-10-14 12:32 UTC - GitHub Copilot (implementation session) - PR #2524 review round 2 by Cameron
  (review 5517039831). F10 and F11: both publishers now hold a dedicated `PublicationOrder` mutex
  (owned by the `udp-server` container) across the ban change and the gauge write, and release the
  ban service lock before the gauge write, so the request path's ban check no longer waits for the
  statistics store; the lock tests use a test-only `Repository::write_lock_for_test` seam instead of
  the `stats` field, which #2517 plans to make private. F7: added the missing R2 below-limit case
  (red on `develop`'s behavior) and corrected the R2 note. F9: the second recheck and the second
  after run predated four production commits, so the after measurement was redone on the final code
  (load test passes; 5 of 28 Criterion medians over the limit, the IPv6 `record_error` cost rising
  to about 0.8 ns with the overflow guard, accepted by the maintainer) and the whole recheck is
  being repeated on the final code (L-V1 to L-V4).
- 2026-10-14 12:55 UTC - GitHub Copilot (implementation session) - F9 completed: the recheck on the final code
  (L-V1 to L-V4, tracker started 11:54:15 UTC, reset at 12:54:15 UTC) reported 0, 0, 1, and 0 right
  after the reset, the same as both earlier rechecks; the earlier "final" recheck is relabeled the
  second recheck, and AC6 now cites the last one.
- 2026-10-14 14:26 UTC - GitHub Copilot (post-merge session) - PR #2524 merged into `develop` on
  2026-10-14 at 14:21:52 UTC as `8045bd252`, which closed this issue at 14:21:53 UTC. At the
  maintainer's decision, Cameron's two change requests were dismissed before the merge because his
  re-review was taking too long; all their findings were fixed, replied to, and resolved. Set
  `related-pr` to 2524.

## Acceptance Criteria

- [x] AC1: The banned-IPs total counts only IPs whose error count exceeds the limit, in both `strict`
  and `disabled` mode (R1, R1p, R5, R6, scenarios M1-M3).
- [x] AC2: After a ban reset, the banned total and the published gauge report the new value (R2, R3,
  R4, scenario M4).
- [x] AC3: The inherent `BanService::get_banned_ips_total` has a doc comment, and it, the
  `BanService::reset_bans` doc, the metric, the `udp-server` helper, and the REST field descriptions
  match the behavior, including the `disabled`-mode meaning.
- [x] AC4: The `BanningStats` trait and `BanService::last_connection_id_errors_reset` are removed.
- [x] AC5: `performance-evidence.md` records the baseline and the after measurements
  (Criterion, including `udp_ban_service/record_error`, and `aquatic_udp_load_test`), and the after
  measurement meets
  the one-sided pass rule in "Performance Evidence Plan", with the observed noise stated.
  **Met with a maintainer-accepted deviation:** on the final code the load test passes and 5 of 28
  Criterion medians exceed their baseline limit, explained in `performance-evidence.md` ("Second
  Investigation - Same-Machine Comparison", "Comparison") as the inherent cost of the counter and
  the overflow guard (up to about 0.8 ns, or 6%, per invalid connection ID on IPv6), which the
  maintainer accepted.
- [x] AC6: The reproduction is repeated after the fix and recorded.
- [x] AC7: The work done on each invalid-connection-ID event to maintain and read the banned total
  does not grow with the number of tracked IPs (no scan of the per-IP map on the event path).
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [x] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `cargo test -p torrust-tracker-udp-core -p torrust-tracker-udp-server`
- `cargo test -p torrust-tracker --lib bootstrap::jobs::udp_tracker_server` (R4)
- `cargo test --test integration` and the root banning tests under `tests/banning/`
- `cargo test --doc --workspace`
- `cargo bench -p torrust-tracker-udp-core --bench ban_service_benchmark` (benchmark task B1, T3, T9; three runs
  each for T3 and T9)
- `aquatic_udp_load_test` against `./target/release/torrust-tracker --config-toml-path
  ./share/default/config/tracker.udp.benchmarking.toml` (T3, T9; five runs each, see
  [E2E UDP load testing](../../../benchmarking.md#e2e-udp-load-testing))
- `linter all`
- `./contrib/dev-tools/git/hooks/pre-commit.sh` before every commit; pre-push checks before push

Record the toolchain for every validation command result.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`. "Before" values were observed on
`develop` at `1c5939c76`; "After" values are the expected result of the fix.

| ID  | Scenario                                     | Human-oriented command/steps                                                     | Expected Result                                                       | Status        | Evidence                                     |
| --- | -------------------------------------------- | -------------------------------------------------------------------------------- | --------------------------------------------------------------------- | ------------- | -------------------------------------------- |
| M1  | One invalid ID, not banned                   | `udp-banned-ips-probe.sh invalid-announce 127.0.0.1`, `stats`, `connect`         | Before: 1 (observed). After: 0. Connect served in both.               | DONE (after: 0) | `manual-verification-evidence.md` V1 and F-V1 |
| M2  | Several IPs with one invalid ID, none banned | One `invalid-announce` from `127.0.0.2` and `127.0.0.3`, `stats`, `connect`      | Before: 3 (observed). After: 0. Connects served in both.              | DONE (after: 0) | `manual-verification-evidence.md` V2 and F-V2 |
| M3  | One IP really banned                         | 11 `invalid-announce` from `127.0.0.4`, `stats`, `connect` from `.4` and `.1`    | Before: 4 (observed). After: 1. `.4` dropped, `.1` served in both.    | DONE (after: 1) | `manual-verification-evidence.md` V3 and F-V3 |
| M4  | Gauge after the periodic reset               | Wait for the first reset (3600 s), `stats`, `connect` from `127.0.0.4`           | Before: stale 4 (observed). After: 0. `.4` served again in both.      | DONE (after: 0) | `manual-verification-evidence.md` V4 and F-V4 |

### Disposable Verification Scripts

- Path: [udp-banned-ips-probe.sh](udp-banned-ips-probe.sh) (Bash).
- What it verifies: it sends a raw BEP 15 announce with connection ID 0 (an invalid cookie) or a
  connect request from a chosen loopback source IP, decodes the response, and reads the banned-IPs
  value from the REST JSON stats, the Prometheus stats, and the Prometheus metrics.
- Why temporary automation: the reproduction needs invalid connection IDs from several distinct
  source IPs against the real binary. The tracker client CLI cannot send an arbitrary connection ID
  or choose the source IP. The maintained Rust regression tests (R1-R3) protect the behavior; this
  script only supports the real-artifact reproduction and the like-for-like recheck.
- Owner: the implementer of this issue. Keep it with the spec until the recheck is recorded, and
  remove or retain it when the spec is archived.

### Acceptance Verification

| AC ID | Status | Evidence           |
| ----- | ------ | ------------------ |
| AC1   | DONE   | R1, R1p, R5, R6 green and proven (manual-verification-evidence.md "Regression Tests"); scenarios M1-M3 after the fix 0, 0, 1 |
| AC2   | DONE   | R2, R3, R4 green and proven; scenario M4 after the fix 0 right after the reset |
| AC3   | DONE   | `docs(udp-server): [#2505] describe the banned-IPs metric as IPs over the limit`; the new `BanService::get_banned_ips_total` doc comment and the updated `reset_bans` doc in `fix(udp-core): [#2505] count only IPs over the limit as banned` |
| AC4   | DONE   | `refactor(udp-core): [#2505] remove the unused BanningStats trait and the write-only reset timestamp`; no code reference remains. The open EPIC (`docs/issues/open/1669-overhaul-packages/EPIC.md`, SI-30 row) still names `BanningStats` as an SI-30 deliverable, which records history and stays as is. |
| AC5   | DONE (accepted deviation) | `performance-evidence.md` "Final After Run - Final Code" and "Comparison": load test passes; 5 Criterion medians over the limit, accepted by the maintainer |
| AC6   | DONE   | manual-verification-evidence.md "Recheck on the Final Code (T10)" (L-V1 to L-V4 on the final code) |
| AC7   | DONE   | `record_error` medians of the final after run flat across 10 to 10,000 tracked IPs; the benchmark task B1 check shows a scan would grow linearly |

## Risks and Trade-offs

- **Dashboards built on the inflated value will drop.** The value has been inflated since the metric
  was introduced (#1145), so every release that reports it is affected. Mention it in the PR and
  release notes.
  Operators who want "IPs that sent an invalid connection ID" lose that number; the existing
  counter `udp_tracker_server_connection_id_errors_total` still reports the error volume.
- **Per-error cost.** See "Performance Alternatives" for the options and the "Performance Evidence
  Plan" for the measurements that guard against regressions.
- **Conflict with Issue #1348.** Its branch is adding tests to `banning.rs`'s test module. Keep
  production changes and new tests focused; #1348 rebases onto this work and deliberately leaves
  `get_banned_ips_total()` untested so it does not pin the wrong value.

## Implementation Completion Review

- Retrospective: [implementation-retrospective.md](implementation-retrospective.md) (material
  discoveries: the pass rule for a deliberately added cost, a health check port collision between
  concurrent trackers, a non-compiling mutation, locale-dependent number formatting, a manual
  recheck that ran before the last production change, dependent manual steps run in parallel, and a
  commit chained after a failed GPG signature).
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this folder; otherwise add a progress-log
  entry explaining why the work had no material discovery.

## References

- Origin: Issue #1348, `services/banning.rs` file test plan.
- Leftover history: #1924 (SI-30), #1942 (SI-4), EPIC #1938, #1096.
- Package ADR: `packages/udp-core/docs/adrs/20260829204258_use_exact_ip_counters_for_udp_banning.md`.
- Similar hand-off precedent: Issue #2458.
