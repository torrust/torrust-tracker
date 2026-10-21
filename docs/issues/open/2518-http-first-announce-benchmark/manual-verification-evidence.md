---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2518-http-first-announce-benchmark/ISSUE.md
last-updated-utc: "2026-10-21 10:00"
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
- Status: `IN_PROGRESS`. The `cargo test` half is `DONE`. The unchanged `cargo bench` run is
  pending: the maintainer asked to be told before any `cargo bench`, to run it when the computer is
  idle.

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
correct run and still detects an update. The unchanged `cargo bench` half of M1 is not yet done.

### V2 - M2: Results are stable and ordered as expected

- Goal: three `cargo bench` runs with the median per swarm size (AC1, AC4).
- Status: `TODO`. Waiting for the maintainer to run it, or to say the computer is idle.

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
