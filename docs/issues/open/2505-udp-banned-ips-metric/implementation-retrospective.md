---
semantic-links:
  skill-links:
    - write-markdown-docs
    - run-benchmarks
    - fix-bug
  related-artifacts:
    - docs/issues/open/2505-udp-banned-ips-metric/ISSUE.md
    - docs/issues/open/2505-udp-banned-ips-metric/performance-evidence.md
    - docs/issues/open/2505-udp-banned-ips-metric/manual-verification-evidence.md
    - docs/benchmarking.md
---

# Implementation Retrospective — UDP Banned-IPs Metric (Issue #2505)

## Purpose

Record evidence-based process improvements discovered while implementing issue #2505. This is a
blameless review of the implementation approach; it does not replace acceptance-criteria
verification.

## Outcome

The banned-IPs total now counts only IPs whose connection ID errors exceed the limit (a counter
maintained in `BanService`, updated when an IP's count reaches `limit + 1` and cleared on reset),
and the periodic reset publishes the new total through `udp-server`'s `banning::reset_bans`. The
unused `BanningStats` trait and the write-only `last_connection_id_errors_reset` field are gone,
and the metric, REST field, and method documentation describe the `disabled`-mode meaning.
Regression tests R1, R1p, R2, R3, R4, R5, and R6 were proven red or by mutate-then-restore, the
like-for-like recheck no longer shows either defect, and the end-to-end load test shows no
regression.

## What Went Well

1. Reproducing against the real tracker before the spec review exposed facts the draft did not
   have: the REST field reads the same gauge, so both surfaces were stale after a reset.
2. Adding the `record_error` benchmark first (B1) and checking it against a deliberately linear
   variant showed that it would detect the AC7 violation the review had worried about.
3. The independent model in the property test (R1p) caught all six counter mutations (M1-M4 and
   their variant-B forms), more than any single example test.
4. When the Criterion pass rule failed, a back-to-back comparison with `develop`, A, and B on the
   same machine separated the real cost (about 0.4-0.6 ns per invalid connection ID) from machine
   drift and noise before asking the maintainer to decide.

## What Changed During Implementation

- **The one-sided Criterion pass rule failed by construction.** The fix deliberately adds a
  comparison and a conditional increment to the error path that `record_error` measures, so "each
  after median is not above the highest baseline median" could not pass. The spec did not say what
  cost was acceptable. The maintainer accepted the cost after the investigation and chose the
  one-comparison form (`perf(udp-core): [#2505] detect the ban crossing with one comparison`).
  Evidence: `performance-evidence.md`, "P2 Investigation" and "Comparison".
- **Concurrent trackers collided on the health check port.** The first P2 load test ran while the
  recheck tracker was waiting for its one-hour reset, and both used the default health check
  address `127.0.0.1:1313`, so the benchmark tracker exited at startup and the run measured 0
  responses per second. P2 was rerun after the recheck.
- **A mutation that did not compile looked like "no test failed".** The first M1 mutation left a
  variable unused, which the workspace's `-D warnings` rejects; the filtered output was empty. It was
  noticed only because the result was implausible, and the mutation was redone.
- **Locale-dependent number formatting.** `awk` under the desktop's Spanish locale printed `9,000`
  for 9.0 and truncated decimals, which briefly looked like quantized Criterion medians. Running
  with `LC_ALL=C` fixed it; the raw `estimates.json` values were correct.
- **Manual-only timing cost.** The 3600-second minimum reset interval made both the reproduction
  and the recheck of V4 take an hour each; the work was arranged around it (V1-V3 first, other
  tasks during the wait).

## Root Cause

- The performance plan copied the regression-detection pass rule from issues that expected no
  extra work on the measured path. For a fix that intentionally adds work, the plan needs an
  expected cost or an explicit tolerance, decided before measuring.
- The benchmarking configuration and the reproduction configuration both left the health check API
  on its default address, and the plan did not say that measurements need exclusive use of the
  machine's tracker ports.
- The mutate-then-restore procedure does not require checking that the mutated code compiled
  before reading the test results.

## Improvements for Future Work

1. In `docs/benchmarking.md` ("Recording Before-and-After Evidence"), add that when a change
   deliberately adds work to a measured path, the specification states the expected cost or an
   explicit tolerance for that benchmark, decided with the maintainer before P2.
2. In `docs/benchmarking.md` and the `run-benchmarks` skill, add a pre-measurement check that no
   other tracker is running and that the ports in the benchmarking configuration, including the
   health check API, are free.
3. In the `write-unit-test` skill's mutate-then-restore step, require confirming that each mutation
   compiled (or reading the full output) before treating "no failing test" as a result.
4. In evidence-processing commands, use `LC_ALL=C` for `awk`, `printf`, and `sort` on numeric data.

These are proposals for the maintainer; none was applied in this issue.

## Avoiding Overcorrection

- Do not replace the one-sided pass rule; it worked as designed and flagged a real cost. Only the
  planning of an expected cost is missing.
- Do not add a rule that every benchmark run must avoid all concurrent activity: the machine is a
  shared desktop, and recording the load average was enough to interpret the results.
- Do not lower the minimum reset interval for testing; the paused-clock R4 test covers the reset
  path automatically, and the hour-long manual check is needed only for the real-artifact recheck.

## Evidence

- Specification: [ISSUE.md](ISSUE.md)
- Performance: [performance-evidence.md](performance-evidence.md)
- Reproduction, regression tests, mutations, and recheck:
  [manual-verification-evidence.md](manual-verification-evidence.md)
- Commits: `fix(udp-core): [#2505] count only IPs over the limit as banned`,
  `fix(udp-server): [#2505] publish the banned-IPs total when bans are reset`,
  `perf(udp-core): [#2505] detect the ban crossing with one comparison`
