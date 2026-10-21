---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2518-http-first-announce-benchmark/ISSUE.md
last-updated-utc: "2026-10-21 15:00"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-21 09:54.
- Artifact under test: the commit `feat(http-core): [#2518] benchmark first HTTP announces into a
  swarm of fixed size`, on top of `refactor(http-core): [#2518] let a benchmark choose its torrent
  repository`, both cut from `develop` `d21282aa4` (`torrust/develop`).
- Operating system / environment: Linux x86_64, a shared desktop.
- Toolchain: stable Rust 1.99.0 (`b940084d7`); `test` profile for `cargo test`.
- Prerequisites and setup performed: none. Each mutation was applied with `sed` to the committed
  files, the run was recorded, and the files were restored with `git checkout -- .`; after each
  restore `git diff --quiet HEAD -- .` exited 0.

## Verification Processes

### V1 - M1: Each iteration is a first announce (`cargo test --benches` half)

- Goal: show that the maintained checks pass on the unchanged benchmark and that each fails under a
  deliberate mutation (AC2, AC3, AC6).
- Initial state: clean working tree at the commit under test.
- Status: `DONE`. Both halves ran: `cargo test` (this section) and the unchanged `cargo bench`
  (V1b below).

#### Steps Performed

Each step ran alone, one after the other. The command for every run was
`cargo test -p torrust-tracker-http-core --bench http_tracker_core_benchmark`. The output below is
filtered to the benchmark lines (`Testing http_tracker/announce_once/...` lines are omitted); the
start and end times are from `date -u +%T.%3N` around each run.

| Run | Mutation (the only change from the committed files)                                                                  | Guard it exercises                                                |
| --- | -------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| M0  | none                                                                                                                 | all pass                                                          |
| M-a | `guard.track(context.first_announce()).await;` became `drop(guard.track(context.first_announce()));`                 | `RoutineGuard` (AC3)                                              |
| M-b | `context.depart().await;` in the timed loop replaced by a comment                                                    | swarm size after the run (AC2)                                    |
| M-c | an extra `context.first_announce().await;` before `context.depart().await;` in the timed loop                        | swarm metrics, exact counts (AC6)                                 |
| M-d | pre-fill loop `for index in 0..swarm_size` became `for index in 1..swarm_size`                                       | response size of the first announce (AC6)                         |
| M-e | `SWARM_EVENT_CHANNEL_CAPACITY` set to 2, so the lag rule applies; no other change                                    | lag rule passes an unchanged run                                  |
| M-f | M-c together with M-e                                                                                                | lag rule still detects an update                                  |

#### Observed Result

```text
### M0 unchanged
start 09:54:13.525
Testing http_tracker/first_announce/0
Success
Testing http_tracker/first_announce/74
Success
Testing http_tracker/first_announce/1000
Success
end   09:54:16.548

### M-a
start 09:54:16.578
thread 'main' panicked at packages/http-core/benches/helpers/routine_guard.rs:52:9:
benchmark `http_tracker/first_announce/0` was invoked 1 time(s) but its measured routine never ran: an async routine must be awaited, for example with `b.to_async(&runtime).iter(...)`
end   09:54:18.761

### M-b
start 09:54:18.903
thread 'main' panicked at packages/http-core/benches/helpers/first_announce.rs:192:9:
assertion `left == right` failed: benchmark `http_tracker/first_announce/0`: the swarm should have exactly 0 peers after the run, but it has 1; every first announce should be followed by the departure of its peer
  left: 1
 right: 0
end   09:54:19.323

### M-c
start 09:54:19.336
thread 'main' panicked at packages/http-core/benches/helpers/first_announce.rs:215:13:
assertion `left == right` failed: benchmark `http_tracker/first_announce/0` (swarm of 0 peers, 3 first announces, 2 departures): every announce should be an insert or a removal, never an update
  left: PeerEventCounts { added: 2, updated: 1, removed: 2 }
 right: PeerEventCounts { added: 3, updated: 0, removed: 2 }
end   09:54:20.605

### M-d
start 09:54:20.618
thread 'main' panicked at packages/http-core/benches/helpers/first_announce.rs:142:9:
assertion `left == right` failed: the first announce into a swarm of 74 peers should return the other peers, up to the announce limit
  left: 73
 right: 74
end   09:54:21.892

### M-e
start 09:54:21.904
Testing http_tracker/first_announce/0
Success
Testing http_tracker/first_announce/74
Success
Testing http_tracker/first_announce/1000
Success
end   09:54:23.202

### M-f
start 09:54:23.208
thread 'main' panicked at packages/http-core/benches/helpers/first_announce.rs:220:13:
assertion `left == right` failed: benchmark `http_tracker/first_announce/0` (swarm of 0 peers, 3 first announces, 2 departures): no announce should update an existing peer; counted PeerEventCounts { added: 2, updated: 1, removed: 2 }
  left: 1
 right: 0
end   09:54:24.446
```

#### Conclusion

Every maintained check passed on the unchanged benchmark and failed, naming the benchmark and the
counted values, under its mutation. M-a also shows that the `RoutineGuard` still fails when the
timed announce is not awaited, with `iter_custom`. M-e and M-f show that the lag rule accepts a
correct run and still detects an update. The unchanged `cargo bench` half is V1b.

### V1b - M1: unchanged benchmark under `cargo bench`

- Goal: show that the unchanged benchmark passes all assertions in a real run, where millions of
  iterations send more swarm events than the event bus holds, so the D5 lag rule applies.
- Status: `DONE`.
- Date and time (UTC): 2026-10-21 13:06:33-13:06:41. The maintainer asked for the run ("DO it now")
  while the computer was not idle; load average at the start: 6.62.

```text
$ cargo bench -p torrust-tracker-http-core --bench http_tracker_core_benchmark   # exit 0
http_tracker/announce_once/announce_once          time:   [346.71 ns 359.80 ns 375.52 ns]
http_tracker/first_announce/0                     time:   [441.85 ns 447.32 ns 454.43 ns]
http_tracker/first_announce/74                    time:   [1.1428 µs 1.1500 µs 1.1571 µs]
http_tracker/first_announce/1000                  time:   [1.1651 µs 1.1752 µs 1.1853 µs]
```

No assertion failed. At about 1 µs per announce, one second of measurement is about a million
iterations per size, so the context sent far more than 65,536 swarm events and the lag rule's
weaker checks (no update counted, no count above its expected value) ran. This run does not show
whether the listener dropped any event.

### V2 - M2: Results are stable and ordered as expected

- Goal: three `cargo bench` runs with the median per swarm size (AC1, AC4).
- Status: `DONE`.
- Toolchain: stable Rust 1.99.0 (`b940084d7`), `bench` profile, on the same shared Linux desktop as
  above; the maintainer was not idling it. Each run was
  `cargo bench -p torrust-tracker-http-core --bench http_tracker_core_benchmark -- first_announce`,
  and the median was read from `target/criterion/http_tracker_first_announce/<N>/new/estimates.json`
  right after the run, before the next one. Times are UTC; `uptime` was recorded at each start.
  Run 1 includes the compile time of the bench profile.

| Run | Start (UTC) | Load average at start (1/5/15 min) | Median, 0 peers | Median, 74 peers | Median, 1000 peers |
| --- | ----------- | ---------------------------------- | --------------- | ---------------- | ------------------ |
| 1   | 13:04:48    | 4.16 / 5.18 / 4.80                 | 432.2 ns        | 1170.2 ns        | 1165.9 ns          |
| 2   | 13:06:12    | 7.71 / 6.16 / 5.19                 | 469.7 ns        | 1248.0 ns        | 1261.8 ns          |
| 3   | 13:06:19    | 7.49 / 6.14 / 5.19                 | 432.8 ns        | 1212.5 ns        | 1161.5 ns          |

Run-to-run variation of the median (maximum over minimum): 0 peers 8.7%, 74 peers 6.6%, 1000 peers
8.6%. Run 3's 0-peer confidence interval was wide (462-507 ns), with a mean of 475 ns against a
median of 433 ns.

#### Conclusion

AC4 holds: the 74- and 1000-peer results are about 2.7 times the 0-peer result in every run. The
74- and 1000-peer results are within the run-to-run variation of each other, so swarm size alone,
beyond a full 74-peer list, does not change the cost; building the peer list does (the second
reason for D2's sizes). The machine was loaded (load average 4-8 on 32 logical CPUs), so the
absolute numbers are not a baseline; the ordering and the 74-versus-1000 equality repeated in all
three runs.

## Design Review (T1, T2)

Prose-first Arrange-Act-Assert review of the benchmark, after the increment passed.

- Arrange: a swarm of exactly `N` other peers, with the registry events and the statistics listener
  wired as in production. It lives in `FirstAnnounceBenchmarkContext::new`, which also makes the
  one unmeasured first announce and checks its response size.
- Act: one awaited `started` announce of a peer that is new to the swarm. It stays visible in
  `bench_first_announce`, wrapped by the `RoutineGuard` and between the two clock reads.
- Assert: `assert_swarm_unchanged_by_run`, after the run and outside the measured time: the swarm
  size and the listener's counts, with the lag rule of D5.
- The causal condition (a new peer announced into a swarm of exactly `N` peers) is readable in the
  benchmark function. The context owns only incidental mechanics: building the services, the
  listener, the pre-fill, and the bounded wait for the listener.
- One departure announce per iteration is outside the measured time but inside the loop, and its
  events may still be processed during the next timed announce (Risks in the spec).

## Failures and Follow-up

The first compile of the benchmark failed `-D dead-code` on an unused context field, and the first
clippy run failed `clippy::useless_conversion`; both were fixed before the commit.
