---
doc-type: performance-evidence
issue-spec: docs/issues/open/2370-1488-si-15-define-udp-active-request-policy/ISSUE.md
last-updated-utc: "2026-09-30 07:27"
---

# UDP Performance Evidence

## Method

- Tracker source: `develop` at `02c026c0`, before SI-15 code changes.
- Tracker build: `cargo build --release` on `rustc 1.101.0-nightly (c1070d693 2026-09-28)`.
- Tracker configuration: [b0-tracker.toml](b0-tracker.toml), which disables
  persistence so every fresh tracker process starts with an empty swarm state.
- Load-test tool: Aquatic `master` at `a2ddc4b3`, built release binary
  `aquatic_udp_load_test`.
- Load-test configuration: [b0-load-test.toml](b0-load-test.toml).
- Runs: five 30-second iterations on the same machine, with no intentional
  concurrent workload.
- Environment: Linux 7.0.0-34-generic x86_64 GNU/Linux.

## B0 - Develop Baseline

An initial shared-SQLite attempt is invalid and excluded: its first run measured
`161409.14` responses/s, but later runs declined to zero or near zero because
each process reopened the same accumulated torrent and peer state. B0 uses the
nonpersistent configuration above instead.

| Run | Average responses per second | Notes |
| --- | ---------------------------- | ----- |
| 1 | 165129.04 | Fresh nonpersistent tracker process |
| 2 | 157337.00 | Fresh nonpersistent tracker process |
| 3 | 152965.92 | Fresh nonpersistent tracker process |
| 4 | 158734.29 | Fresh nonpersistent tracker process |
| 5 | 162900.68 | Fresh nonpersistent tracker process |

### Result

Mean: 159413.39 responses/s. Median: 158734.29 responses/s. Range:
152965.92-165129.04 responses/s.

## B1 - Processor `Result` Only

Source: the T4 change ("feat(udp-server): return a Result from UDP request
processors") on top of T3; the receive loop still discards the result. Same
machine, toolchain, tracker configuration, load-test tool, and settings as B0.

| Run | Average responses per second | Notes |
| --- | ---------------------------- | ----- |
| 1 | 165488.27 | Fresh nonpersistent tracker process |
| 2 | 165152.68 | Fresh nonpersistent tracker process |
| 3 | 164598.75 | Fresh nonpersistent tracker process |
| 4 | 163440.01 | Fresh nonpersistent tracker process |
| 5 | 164492.15 | Fresh nonpersistent tracker process |

### B1 Result

Mean: 164634.37 responses/s. Median: 164598.75 responses/s. Range:
163440.01-165488.27 responses/s.

AC11 for B1 passes: the B1 mean is above the lowest B0 run (152965.92). Every
B1 run is also above the B0 mean, so the `Result` return type shows no
measurable cost; the difference is within this shared machine's noise.

## B2 - `JoinSet` Drain Wiring

Source: the T5 change on top of T4. The receive loop now spawns every
processor into a `JoinSet`, reaps finished processors with `try_join_next`
before each spawn, and drains the set on shutdown. The overload ring is
unchanged. Same machine, toolchain, tracker configuration, load-test tool, and
settings as B0. Each run used a fresh tracker process stopped with `SIGTERM`;
all five load tests and tracker processes exited with status 0.

| Run | Average responses per second | Notes |
| --- | ---------------------------- | ----- |
| 1 | 169580.45 | Fresh nonpersistent tracker process |
| 2 | 167329.09 | Fresh nonpersistent tracker process |
| 3 | 167005.40 | Fresh nonpersistent tracker process |
| 4 | 167587.93 | Fresh nonpersistent tracker process |
| 5 | 167078.13 | Fresh nonpersistent tracker process |

### B2 Result

Mean: 167716.20 responses/s. Median: 167329.09 responses/s. Range:
167005.40-169580.45 responses/s.

AC11 for B2 passes. The B2 mean is above the lowest B0 run (152965.92), and
every B2 run is above both the B0 and B1 means. The extra `JoinSet`
bookkeeping on the request path shows no measurable cost. The gain over B0 and
B1 is within this shared machine's noise and is not claimed as an improvement.
