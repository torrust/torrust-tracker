---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p2
epic: null
github-issue: 2505
spec-path: docs/issues/open/2505-udp-banned-ips-metric/ISSUE.md
branch: "2505-udp-banned-ips-metric-spec"
related-pr: null
last-updated-utc: "2026-10-13 16:59"
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

- `BanService::get_banned_ips_total` and `BanningStats::get_banned_ips_total` docs: "Returns the
  total number of banned IPs."
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
`get_banned_ips_total` dates from the package extraction (`1886593b4`, 2025-02-18); the trait
wrapper came with `637c17b18` (2026-06-23).

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
- Fix the documentation listed under "Contracts violated" to match decision A.
- Remove the unused `BanningStats` trait and its implementation.
- Remove the write-only `BanService::last_connection_id_errors_reset` field.
- Adjust the `udp-server` unit tests that pin the old value, keeping their intent:
  - the handler test `it_should_publish_the_distinct_client_ip_total_after_a_connection_cookie_error`;
  - the listener tests in `packages/udp-server/src/banning/event/listener.rs` that use
    `get_banned_ips_total()` as a proxy for "the event was processed" (switch them to
    `get_count(ip)` or another direct observation).
- Run the existing `udp_ban_service` Criterion benchmark before and after the change (see
  "Performance Alternatives").

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
`is_banned` read lock) and the error path, so it needs the before-and-after evidence below.

### Performance Evidence Plan

Follows [docs/benchmarking.md](../../../benchmarking.md) ("Choosing a Benchmark" and "Recording
Before-and-After Evidence") and the `run-benchmarks` skill, with issue #2458 as the worked example.

- **Microbenchmark (precise):** the existing Criterion groups `udp_ban_service/increase_counter`
  and `udp_ban_service/is_banned` in `packages/udp-core/benches/ban_service_benchmark.rs`. No
  benchmark covers the per-event step the handler runs (`increase_counter` followed by reading the
  banned total), so **B1** adds one, `udp_ban_service/record_error`, measured at the existing
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

- **P1 (baseline, T3)** runs after B1 and before any production change: three Criterion runs (each
  benchmark's median read from `target/criterion/<group>/<function>/new/estimates.json`) and five
  load-test runs. **P2 (after, T9)** repeats the same measurements on the finished code, on the same
  machine, with the same build profile, configuration, and load-test settings. The benchmark code
  may change between P1 and P2 only as far as changed signatures require.
- Check the machine before each measurement (`uptime`, other builds) and record it.
- Record the machine, toolchain, aquatic commit, commands, configuration, every run, and the
  observed noise in issue-local `performance-evidence.md`, with times taken from output files or
  `git log`.
- **Pass rule (one-sided):** the P2 load-test mean responses per second is not below the lowest P1
  run, and each P2 Criterion median is not above the highest of the three P1 medians for the same
  benchmark. State the observed P1 noise, so readers know which regressions the measurement can
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
| Property-based unit, `udp-core`               | Recommended | The invariant "banned total == number of IPs for which `is_banned` is true" for arbitrary error sequences, limits, and resets. Most valuable with P1/P3, where a second piece of state must stay consistent with the map.                                 |
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
  limit again to be counted. Passes on `develop` (the map is cleared); prove it guards the fix with
  mutate-then-restore (for P1: do not reset the counter).
- **R1p (property-based unit, `udp-core`, recommended):** for arbitrary sequences of
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
- **R6 (package-level integration, `udp-server`, `disabled`, recommended):** pins decision A. From
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
| B1  | TODO   | Add the per-event microbenchmark                   | `udp_ban_service/record_error` in `ban_service_benchmark.rs`, committed before any production change.          |
| T3  | TODO   | Performance baseline (P1)                          | Three Criterion runs and five `aquatic_udp_load_test` runs before any production change, in `performance-evidence.md`. |
| T4  | TODO   | R1, R5 (and R6) regression tests, red              | Recorded failing output against the current implementation.                                                    |
| T5  | TODO   | `udp-core` fix                                     | Chosen performance option; R1, R5, R6 green; R2 and R1p added and proven by mutate-then-restore; leftover trait and field removed. |
| T6  | TODO   | R4 regression test, red                            | Recorded failing output showing the stale gauge after one reset interval.                                       |
| T7  | TODO   | Gauge update on reset                              | Chosen option (G1 preferred); R4 green; R3 added and proven by mutate-then-restore.                             |
| T8  | TODO   | `udp-server` test and documentation updates        | Tests listed in scope updated; contracts listed under "Contracts violated" corrected for decision A.            |
| T9  | TODO   | Performance after the fix (P2)                     | Same measurements as T3; comparison against the one-sided pass rule in `performance-evidence.md`.              |
| T10 | TODO   | Verification, like-for-like recheck, and AC review | Automatic checks; V1-V4 rerun with the same script and configuration; acceptance criteria re-reviewed.          |

## Commit Points

| Task   | Coherent change set                                                  | Commit policy                                       |
| ------ | -------------------------------------------------------------------- | --------------------------------------------------- |
| T4-T5  | `udp-core` banned-count fix, leftover removal, R1, R1p, R2, R5, R6   | Commit after focused validation and design review.  |
| T6-T7  | Gauge-on-reset fix with R3 and R4                                    | Commit after focused validation and design review.  |
| T8     | `udp-server` test and documentation updates                          | Commit after focused validation.                    |
| B1     | `udp_ban_service/record_error` benchmark                             | One `test(udp-core)` commit before T3.              |
| T3     | P1 baseline in `performance-evidence.md`                             | One `docs(issues)` commit before any production change. |
| T9     | P2 results and comparison in `performance-evidence.md`               | One `docs(issues)` commit.                          |
| T10    | Recheck evidence and acceptance review                               | Commit after maintainer review.                     |

Record the red output of T4 and T6, and the mutate-then-restore output for R1p, R2, and R3, before
committing the fixes. For test-producing work, use the
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
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual recheck recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
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

## Acceptance Criteria

- [ ] AC1: The banned-IPs total counts only IPs whose error count exceeds the limit, in both `strict`
  and `disabled` mode (R1, R1p, R5, R6, M1-M3).
- [ ] AC2: After a ban reset, the banned total and the published gauge report the new value (R2, R3,
  R4, M4).
- [ ] AC3: The method, metric, `udp-server` helper, and REST field descriptions match the behavior,
  including the `disabled`-mode meaning.
- [ ] AC4: The `BanningStats` trait and `BanService::last_connection_id_errors_reset` are removed.
- [ ] AC5: `performance-evidence.md` records the P1 baseline and the P2 after measurements
  (Criterion, including `udp_ban_service/record_error`, and `aquatic_udp_load_test`), and P2 meets
  the one-sided pass rule in "Performance Evidence Plan", with the observed noise stated.
- [ ] AC6: The reproduction is repeated after the fix and recorded.
- [ ] AC7: The work done on each invalid-connection-ID event to maintain and read the banned total
  does not grow with the number of tracked IPs (no scan of the per-IP map on the event path).
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `cargo test -p torrust-tracker-udp-core -p torrust-tracker-udp-server`
- `cargo test -p torrust-tracker --lib bootstrap::jobs::udp_tracker_server` (R4)
- `cargo test --test integration` and the root banning tests under `tests/banning/`
- `cargo test --doc --workspace`
- `cargo bench -p torrust-tracker-udp-core --bench ban_service_benchmark` (B1, T3, T9; three runs
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
| M1  | One invalid ID, not banned                   | `udp-banned-ips-probe.sh invalid-announce 127.0.0.1`, `stats`, `connect`         | Before: 1 (observed). After: 0. Connect served in both.               | Before: DONE  | `manual-verification-evidence.md` section V1 |
| M2  | Several IPs with one invalid ID, none banned | One `invalid-announce` from `127.0.0.2` and `127.0.0.3`, `stats`, `connect`      | Before: 3 (observed). After: 0. Connects served in both.              | Before: DONE  | `manual-verification-evidence.md` section V2 |
| M3  | One IP really banned                         | 11 `invalid-announce` from `127.0.0.4`, `stats`, `connect` from `.4` and `.1`    | Before: 4 (observed). After: 1. `.4` dropped, `.1` served in both.    | Before: DONE  | `manual-verification-evidence.md` section V3 |
| M4  | Gauge after the periodic reset               | Wait for the first reset (3600 s), `stats`, `connect` from `127.0.0.4`           | Before: stale 4 (observed). After: 0. `.4` served again in both.      | Before: DONE  | `manual-verification-evidence.md` section V4 |

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
| AC1   | TODO   | R1, R1p, R5, R6, M1-M3 |
| AC2   | TODO   | R2, R3, R4, M4         |
| AC3   | TODO   | Diff               |
| AC4   | TODO   | Diff               |
| AC5   | TODO   | `performance-evidence.md` (T3, T9) |
| AC6   | TODO   | M1-M4 after fix    |
| AC7   | TODO   | Diff (event-path code), T3 and T9 evidence |

## Risks and Trade-offs

- **Dashboards built on the inflated value will drop.** Mention it in the PR and release notes.
  Operators who want "IPs that sent an invalid connection ID" lose that number; the existing
  counter `udp_tracker_server_connection_id_errors_total` still reports the error volume.
- **Per-error cost.** See "Performance Alternatives"; the benchmark guards against regressions.
- **Conflict with Issue #1348.** Its branch is adding tests to `banning.rs`'s test module. Keep
  production changes and new tests focused; #1348 rebases onto this work and deliberately leaves
  `get_banned_ips_total()` untested so it does not pin the wrong value.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this folder; otherwise add a progress-log
  entry explaining why the work had no material discovery.

## References

- Origin: Issue #1348, `services/banning.rs` file test plan.
- Leftover history: #1924 (SI-30), #1942 (SI-4), EPIC #1938, #1096.
- Package ADR: `packages/udp-core/docs/adrs/20260829204258_use_exact_ip_counters_for_udp_banning.md`.
- Similar hand-off precedent: Issue #2458.
