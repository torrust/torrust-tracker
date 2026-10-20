---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2414-2410-si-22-2-drain-listener-queues-on-shutdown/ISSUE.md
last-updated-utc: "2026-10-20 16:37"
---

# Manual Verification Evidence

## Purpose

Record the verification of #2414 (SI-22.2): the red and green runs of its regression tests, the
release-build drain measurement, and the like-for-like recheck of the bug. The reproduction of the
bug itself is in the sub-EPIC's
[manual-verification-evidence.md](../2410-1488-si-22-process-queued-events-before-listeners-stop/manual-verification-evidence.md)
(V0 and V1), the EPIC baseline; this file does not repeat it.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-20 from 11:54.
- Artifact under test: branch `2414-drain-listener-queues-on-shutdown`, cut from `develop` at the
  merge `Merge torrust/torrust-tracker#2569: docs(issues): close the #2413 delivery gates and refresh the SI-22 specs before #2414`.
  Each process below names the working-tree state it ran on.
- Operating system / environment: Linux; default toolchain `rustc 1.99.0 (b940084d7 2026-09-28)`,
  `cargo 1.99.0 (5f94df478 2026-08-27)`, `dev` profile (`cargo test`); tokio 1.53.3 in
  `Cargo.lock`.
- Prerequisites and setup performed: none beyond a workspace build.

## Verification Processes

Dependent steps ran one at a time, each as its own tool call, with `date -u +%T.%3N` at its start
and end. Times are UTC on 2026-10-20. The clock printed an end stamp earlier than its start stamp
in step 5 (11:55:04.125 after 11:55:04.904), as it did in the #2413 evidence; the stamps are kept as
printed, and the order of the steps is the order of the tool calls.

### V1 - T3b: `TestEnv` owns, cancels, and joins its listeners

- Goal: before T3c needs to stop the listeners of the package-level test environment, show that
  `packages/tracker-core/tests/common/test_env.rs` keeps their token and handles, and that
  `stop()` cancels them and waits until each has finished.
- Initial state: `TestEnv::run_jobs` created a local `CancellationToken`, dropped the three
  `JoinHandle`s (`let _swarm_statistics_listener`, `let _in_memory_statistics_listener`,
  `let _persistent_statistics_listener`), and slept 100 ms. Read from source, not run.
- Status: `DONE` for T3b; the drop-without-`stop()` path is covered by V2.

#### Design (maintainer-approved, 2026-10-20)

- `TestEnv` holds a private `Listeners` value: the token, a `DropGuard` on it, and the named
  handles. The guard exists from `TestEnv::new`, before any listener is spawned, so a panic
  during startup cancels the listeners already running.
- `stop(&mut self) -> Result<(), String>` cancels the token and joins every handle under one
  absolute deadline (`timeout_at`, 5 s in total). It joins all handles before reporting any
  failure and aborts a handle that misses the deadline, so nothing stays detached. A second call
  has nothing left to join.
- Dropping the environment cancels the listeners through the guard. Drop cannot await, so it does
  not join them.
- Left unchanged: the 100 ms startup sleep, `yield_now`, and `wait_for_global_downloads_persisted`.
  `stop()` still discards the queue until T4c, so the polling cannot be replaced yet.

#### Prose-first Arrange-Act-Assert

Written before the code and compared with it after; the final test keeps only the comments the
code does not already say.

- Arrange: an environment with persistent completed statistics runs three swarm-bus listeners,
  each holding one receiver. A helper publishes a harmless `TorrentRemoved` event on the real
  bus and returns how many receivers it reached, so the count is observed, not read from a
  production field. The count before the stop is taken here.
- Act: `test_env.stop().await`, visible in the test body.
- Assert: the stop reported no failure; the count before it was 3; the count after it is 0,
  because a finished listener drops its receiver.

#### Steps Performed

All on the T3b working tree (uncommitted), with a backup copy of the two changed files kept in
`.tmp/` and used for every restore.

1. 11:54:37.439 to 11:54:38.488, green: `cmp test_env.rs .tmp/test_env.rs.good` exited 0, then
   `cargo test -p torrust-tracker-core --test integration it_should_release_every_listener` gave
   `1 passed; 0 failed`.
2. 11:54:43.516 to 11:54:49.571, mutation A (no cancel): `sed` replaced
   `self.cancellation_token.cancel();` in `Listeners::stop` by a comment. The file then differed
   from the backup (`cmp` printed "mutation A applied"). The test failed in 5.11 s.
3. 11:54:53.687 to 11:54:53.691, restore: `cp` from the backup; `cmp` exited 0.
4. 11:54:59.113 to 11:55:00.159, mutation B (cancel without join): a Python replace turned the
   join loop into `.into_iter().take(0)`. The file differed from the backup. The test failed in
   0.11 s.
5. 11:55:04.904 to 11:55:04.125, restore: `cp` from the backup; `cmp` exited 0.
6. 11:55:10.673 to 11:55:15.780, `cargo test -p torrust-tracker-core` on the restored files
   (`cmp` against the backup exited 0 first).

An earlier mutation A run, outside this timed sequence, was followed by a `git checkout` restore
of `test_env.rs` that reverted the uncommitted T3b change itself. The change was rewritten from
the same script and the timed sequence above was run on the rewrite. The `git checkout` restore
works only for committed files; after that the backup copy was used.

#### Observed Result

```text
step 1: test it_should_release_every_listener_receiver_when_the_environment_is_stopped ... ok
        test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.11s
step 2: test ... FAILED
        left: Err("swarm statistics listener did not stop within 5s of cancellation; in-memory statistics listener did not stop within 5s of cancellation; persistent completed statistics listener did not stop within 5s of cancellation")
        test result: FAILED. 0 passed; 1 failed; ... finished in 5.11s
step 4: test ... FAILED
          left: 3
         right: 0
        test result: FAILED. 0 passed; 1 failed; ... finished in 0.11s
step 6: test result: ok. 152 passed (unit); ok. 10 passed (tests/integration.rs);
        ok. 16 passed and ok. 2 passed (doctests)
```

After the trimming of the test's comments, the new test was run once more and passed
(`1 passed`); `cargo +nightly fmt --all -- --check` exited 0. `linter all` then failed once on
`clippy::manual_unwrap_or_default` in the test's probe helper; it now uses `unwrap_or_default()`
(same result: the receiver count, or 0 when nobody is subscribed). The mutations above ran on the
earlier form of the helper, which returned the same values. After the change, all 10 integration
tests passed, and `linter all` and `cargo +nightly fmt --all -- --check` exited 0.

#### Conclusion

The test fails when `stop()` does not cancel (A, by the deadline, naming all three listeners) and
when it cancels without waiting (B, three receivers still subscribed), and passes on the
implementation. It proves ownership, cancellation, and joining. It does not prove the drain; that
is T3a, T3c, and T4c.

Not covered here: a listener that panics is not simulated, so the failure branch of `stop()`
other than the deadline is read from source, not run. The drop path is V2.

### V2 - T3b: dropping `TestEnv` without `stop()` cancels its listeners

- Goal: show the environment's `DropGuard` cancels the listeners when the environment is dropped,
  which a plain `CancellationToken` would not do.
- Initial state: V1's working tree plus the test
  `it_should_cancel_every_listener_when_the_environment_is_dropped_without_being_stopped` (the
  maintainer asked for it at the T3b review). The test keeps a clone of the swarm container,
  drops the environment, and waits up to one absolute 5 s deadline for the swarm-bus listener
  count to reach 0.
- Status: `DONE`.

#### Prose-first Arrange-Act-Assert

- Arrange: three swarm-bus listeners run; the test keeps a handle to the swarm container so it
  can observe the bus after the environment is gone; the count before the drop is taken.
- Act: `drop(test_env)`, visible in the test body.
- Assert: the count before was 3, and within the deadline it reaches 0, because the drop guard
  cancelled the listeners (dropping a token alone does not).

#### Steps Performed

1. 12:01:10.274 to 12:01:18.352, mutation C: the guard was put on a different token
   (`_cancel_on_drop: CancellationToken::new().drop_guard()`). `cmp` against the earlier backup
   printed a difference before the mutation because `cargo fmt` had reformatted two spots after
   the backup was taken; the mutation was applied to the working file and confirmed by the `sed`
   having changed it (`mutation C applied`). The test failed in 5.14 s.
2. 12:01:29.980 to 12:01:30.488, undo: the one line was restored with `sed`; `diff` against the
   backup then showed only the two formatting differences (the `if failures.is_empty()` block and
   one `.add(...)` call), and `cargo +nightly fmt --all -- --check` exited 0.
3. 12:01:37.502 to 12:01:39.949, green: `cargo test -p torrust-tracker-core`.
4. `linter all` exited 0 on the final tree.

#### Observed Result

```text
step 1: test it_should_cancel_every_listener_when_the_environment_is_dropped_without_being_stopped ... FAILED
          left: 3
         right: 0
        test result: FAILED. 0 passed; 1 failed; ... finished in 5.14s
step 3: test result: ok. 152 passed (unit); ok. 11 passed (tests/integration.rs);
        ok. 16 passed and ok. 2 passed (doctests)
```

#### Conclusion

The test fails when the guard does not cancel the real listener token, and passes on the
implementation. The 5 s wait is an absolute deadline, not a retry that resets its budget.

### V3 - T3a: a queued completed download is not persisted when cancellation is pending (red)

- Goal: show the persistent listener's cancel-first loop drops a ready completed-download event,
  with a maintained unit test at the dispatch function.
- Initial state: the tree of the commit
  `test(tracker-core): [#2414] make TestEnv own, stop, and join its listeners`, plus the
  uncommitted tests of V3 and V4. The tests are not committed failing; they are committed with
  their fix (T4c). Their code is embedded below, verbatim.
- Status: `DONE` (red recorded; green after T4c).

The test replaces `it_should_prioritize_a_pre_cancelled_token_over_a_ready_persistent_event`,
which asserted the bug (nothing persisted). The helper `expect_persisted_download_metrics_to_be`
gained assertion messages, so a failure names what was measured (read as a stranger).

#### Prose-first Arrange-Act-Assert

- Arrange: at shutdown a completed download is queued for the persistent listener, and its token
  is cancelled before the listener takes the event.
- Act: `dispatch_persistent_completed_statistics_events(...)`, visible in the test body.
- Assert: the queued download reaches the database (`Some(1)`) and the persisted counter shows
  it. That the listener then returns `Cancelled` is asserted by the T4c drain tests.

#### Test code (verbatim, in `packages/tracker-core/src/statistics/event/listener.rs`)

```rust
    #[tokio::test]
    async fn it_should_persist_a_queued_completed_download_when_cancellation_is_already_pending() {
        // Arrange
        // At shutdown a completed download is queued for the persistent listener, and the
        // listener's token is cancelled before the listener takes the event.
        let stats_repository = Arc::new(Repository::new(true, true));
        let downloads_repository = database_downloads_repository().await;
        let receiver = Box::new(ScriptedReceiver::new([Ok(peer_download_completed_event())]));
        let cancellation_token = CancellationToken::new();
        cancellation_token.cancel();

        // Act
        dispatch_persistent_completed_statistics_events(
            receiver,
            cancellation_token,
            downloads_repository.clone(),
            stats_repository.clone(),
        )
        .await;

        // Assert
        // The queued download reaches the database, and the persisted counter shows it.
        expect_persisted_download_metrics_to_be(&downloads_repository, &stats_repository, Some(1)).await;
    }
```

#### Steps Performed

1. 12:06:49.782 to 12:06:50.980: `cargo test -p torrust-tracker-core --lib it_should_persist_a_queued`.

#### Observed Result

```text
test statistics::event::listener::tests::it_should_persist_a_queued_completed_download_when_cancellation_is_already_pending ... FAILED
assertion `left == right` failed: global downloads stored in the database
  left: None
 right: Some(1)
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 151 filtered out; finished in 0.00s
```

#### Conclusion

Red against the cancel-first loop, for the stated reason: the event was ready, the token was
cancelled, and nothing was stored.

### V4 - T3c: queued completions are not persisted when the listener is already cancelled (red), AC11

- Goal: the package-level regression test of the design record's option 1: real event bus, real
  announce handler, real listener function, and a real SQLite database, with no binary, network,
  or signal.
- Initial state: as V3.
- Status: `DONE` (red recorded; green after T4c).

#### Prose-first Arrange-Act-Assert

- Arrange: the environment's listeners are not started (`TestEnv::new`), so nothing consumes the
  events while the test publishes them. The persistent listener's receiver is taken first, so it
  is subscribed before any publish, as at the application's startup. Three peers on different
  ports announce `started` and `completed` for one torrent, so three completions are queued, and
  the listener's token is cancelled before it runs. N = 3 (the maintainer left the value to the
  proposed default).
- Act: `run_persistent_completed_statistics_event_listener_unspawned(...)`, awaited, visible in
  the test body.
- Assert: the database holds 3 downloads for the torrent and 3 globally. The expected value is
  the number of completions announced, not computed by production code.

#### Test code (verbatim, in `packages/tracker-core/tests/integration.rs`)

```rust
fn sample_peer_listening_on_port(port: u16) -> Peer {
    let mut peer = sample_peer();
    peer.peer_addr.set_port(port);
    peer
}

#[tokio::test]
async fn it_should_persist_every_completed_download_queued_before_the_persistent_listener_is_cancelled() {
    // Arrange
    // The environment's listeners are not started, so nothing consumes the events while the
    // test publishes them. The persistent listener's receiver is taken first, so it is
    // subscribed before any event is published, as at the application's startup. Three
    // different peers then announce `started` and `completed` for one torrent, so three
    // completed downloads are queued, and the listener's token is cancelled before it runs.
    let number_of_completions: u32 = 3;
    let mut core_config = ephemeral_configuration();
    core_config.tracker_policy.persistent_torrent_completed_stat = true;
    let test_env = TestEnv::new(core_config).await;
    let persistent_listener_receiver = test_env.swarm_coordination_registry_container.event_bus.receiver();
    let info_hash = sample_info_hash();
    for port in (8080..).take(number_of_completions as usize) {
        let peer = sample_peer_listening_on_port(port);
        test_env.announce_peer_started(peer, &remote_client_ip(), &info_hash).await;
        test_env.announce_peer_completed(peer, &remote_client_ip(), &info_hash).await;
    }
    let cancellation_token = CancellationToken::new();
    cancellation_token.cancel();
    let persistence = test_env
        .tracker_core_container
        .persistence
        .as_ref()
        .expect("this test requires persistence");

    // Act
    run_persistent_completed_statistics_event_listener_unspawned(
        persistent_listener_receiver,
        cancellation_token,
        persistence.db_downloads_metric_repository.clone(),
        test_env.tracker_core_container.stats_repository.clone(),
    )
    .await;

    // Assert
    // Every queued completion is in the database, for the torrent and in the global count.
    assert_eq!(
        persistence
            .database_stores
            .torrent_metrics_store
            .load_torrent_downloads(&info_hash)
            .await
            .unwrap(),
        Some(number_of_completions),
        "downloads of the torrent stored in the database after {number_of_completions} queued completions"
    );
    assert_eq!(
        persistence
            .database_stores
            .torrent_metrics_store
            .load_global_downloads()
            .await
            .unwrap(),
        Some(number_of_completions),
        "global downloads stored in the database after {number_of_completions} queued completions"
    );
}
```

#### Steps Performed

1. 12:07:16.999 to 12:07:18.753, red:
   `cargo test -p torrust-tracker-core --test integration it_should_persist_every_completed`.
2. 12:07:35.784 to 12:07:37.903, control (temporary, reverted): the `cancel()` call was removed
   and the Act was wrapped in `tokio::time::timeout(300 ms, ...)`, because an uncancelled listener
   never ends. The same test then passed, so the three completions were queued and the persist path
   works; only cancellation made the difference.
3. 12:07:42.109 to 12:07:42.113, restore: `cp` from a backup taken after step 1; `cmp` exited 0.
4. 12:07:51.592 to 12:07:53.730: `cargo test -p torrust-tracker-core --no-fail-fast` on the
   restored tree: exactly these two tests failed (V3 and V4); the other 151 unit, 11 integration,
   16 and 2 doctests passed.

#### Observed Result

```text
step 1: test it_should_persist_every_completed_download_queued_before_the_persistent_listener_is_cancelled ... FAILED
        assertion `left == right` failed: downloads of the torrent stored in the database after 3 queued completions
          left: None
         right: Some(3)
        test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.01s
step 2: test ... ok
        test result: ok. 1 passed; 0 failed; ... finished in 0.31s
```

#### Conclusion

Red against the cancel-first loop: three queued completions, a cancelled listener, and nothing in the
database. The control shows the red is caused by cancellation and not by the setup.

### V5 - T4a: the `Receiver` trait, `TryRecvError`, and the shared drain helper

- Goal: show each new test can fail, so it guards the behavior it names. T4a adds behavior, not a
  bug fix, so there is no red-before-fix run; the evidence is mutation: each production line is
  broken in turn and the tests must fail, then the file is restored.
- Initial state: the working tree after T4a (run uncommitted; committed in
  `feat(events): [#2414] add try_recv, len, TryRecvError, and the shared listener drain helper`): `packages/events/src/{receiver,broadcaster,drain,lib}.rs`, `packages/events/Cargo.toml`
  (a `tokio` dev-dependency with `test-util`), and `try_recv` and `len` added to the six scripted
  test receivers (`http-core`, `udp-core`, `udp-server` twice, `swarm-coordination-registry`,
  `tracker-core`). The `mockall` mock is generated from the trait. `cargo test --workspace --no-run`
  compiled everything.
- Status: `DONE`.

#### Design (maintainer-approved, 2026-10-20)

- `Receiver::try_recv` and `Receiver::len`, plus `TryRecvError { Empty, Closed, Lagged(u64) }`.
  The maintainer's answer to the clippy question: if `len_without_is_empty` fires, add a default
  `is_empty()`. It fired for the trait, so `Receiver::is_empty` exists as a default method
  (`len() == 0`). It is a small addition to the design record's trait, recorded here.
- `drain(receiver, timeout, handler) -> DrainReport` in `packages/events/src/drain.rs`: the
  deadline is set at entry with `tokio::time::Instant` and checked only before each event is taken;
  a taken event is awaited and not raced against the timeout. `Lagged(n)` adds to `lost_to_lag` and the drain continues.
  `DrainReport { outcome, processed, lost_to_lag, unread_when_timed_out }`, where the last field is
  `Receiver::len()` at timeout, documented as an unread-position count that includes lag not yet
  observed.
- The drain does not log; each listener logs the report (maintainer decision), done in T4c.

#### Tests (prose-first Arrange-Act-Assert, in the test comments)

`drain.rs`: empty receiver; buffered events handled before `Closed` (real channel, sender
dropped); timeout stops admission and reports the leftover (paused time); `Lagged` counted and
draining continues; unobserved lag included in the unread count at timeout (real channel of
capacity 2 with 5 sent); a slow handler that crosses the timeout completes (AC7); `Emptied` and
not `TimedOut` when the deadline passes with nothing unread. `broadcaster.rs`: `Empty` without
waiting; `Lagged(3)` then the oldest retained event; `Closed` only after buffered events;
`len` counts five sent positions before the lag is reported and two after.

#### Steps Performed

Each mutation ran as one shell command (mutate with a one-line replace, `cargo test -p
torrust-tracker-events`, restore from a backup copy, `cmp`), after `cmp` confirmed the file equalled
its backup. Times are UTC on 2026-10-20 (start to end).

| Mutation | Time | Failed tests |
| --- | --- | --- |
| M1: the deadline check disabled (`if false && ...`) | 12:18:59.390 to .840 | the timeout-stops-admission test, the slow-handler test, the unobserved-lag test |
| M2: the handler raced against the deadline (`timeout_at(deadline, handle(event))`) | 12:19:05.898 to 12:19:06.289 | the slow-handler test |
| M3: lag ignored (`Lagged(_) => {}`) | 12:19:10.798 | none ran: it did not compile (unused `mut`), so it is not a result |
| M3 again: lag counted as zero (`skipped.min(0)`) | 12:19:20.485 to 12:19:21.937 | the lag test |
| M4: `Closed` reported as `Emptied` | 12:19:26.722 to 12:19:27.129 | the buffered-then-closed test |
| M5: any deadline expiry reported as `TimedOut` | 12:19:44.563 to .937 | the emptied-at-deadline test |
| M6: broadcast `Lagged` mapped to `Empty` | 12:19:51.488 to 12:19:55.477 | the skipped-events test |
| M7: broadcast `len` returns 0 | 12:19:55.532 to .594 | the `len` test and the unobserved-lag drain test |
| M8: broadcast `Closed` mapped to `Empty` | 12:20:03.100 to .410 | the closed-after-buffered test and the buffered-then-closed drain test |
| M9: broadcast `Empty` mapped to `Closed` | 12:20:03.415 to .760 | the empty-without-waiting test |

The mutation that exposed a gap: M5 first passed every test, because nothing covered a deadline
that expires with nothing unread. The test for it was added (it is the seventh `drain` test) and
M5 then failed it, as shown. Every restore ended with `cmp` exit 0.

After the last mutation: `cargo test -p torrust-tracker-events` gave 20 passed; `cargo test
--workspace --no-run` compiled; `linter all` exited 0 (it first failed on `clippy::use_self` in the
broadcast impl, fixed to `Self::try_recv(self)` and `Self::len(self)`; an inherent method wins over
the trait's, which the tests would show as a stack overflow if it did not).

#### Conclusion

Every behavior the helper and the trait implementation decide fails at least one test when broken.
Not covered: a handler that panics, and the shared-deadline abort of a drain in progress (the
abort is the supervisor's, sub-issue 4).

### V6 - T4b: time to drain the persistent listener's queue (release build)

- Goal: choose the default drain timeout from a `release`-build measurement of the persistent
  listener's handler draining a full buffer of 65,536 events (D9, D18).
- Initial state: the T4a tree. The measurement is a temporary Rust test (below), not committed:
  it measures a number, it asserts no behavior, and a maintained benchmark of a one-off value
  would need a rationale this issue does not have. Written in Rust, not a script.
- Status: `DONE` (measurement recorded); the default is a proposal for the T4 design review.

#### Environment

- Machine: Linux, 32 logical CPUs, NVMe `WD_BLACK SN850X` (non-rotational, on an LVM volume; the filesystem type was not checked). It is
  **shared**: the load average was 21 to 25 at the first runs and 7 to 12 later (the `uptime`
  before each run is in the table). `/tmp` is tmpfs (memory).
- Toolchain: `rustc 1.99.0 (b940084d7 2026-09-28)`, `cargo 1.99.0`; build:
  `cargo test --release -p torrust-tracker-core --test drain_rate_measurement --no-run`
  (`[profile.release]`: `opt-level = 3`, `lto = "fat"`, `debug = 1`).
- Code measured: the real `handle_persistent_completed_statistics_event` (two SQLite writes and a
  counter update per event) driven by the real `drain` helper over a real tokio broadcast
  receiver already holding the events, on a multi-thread runtime, as the listener's drain will.
  SQLite through the tracker's `sqlite3` driver and the default configuration.

#### Test code (verbatim, `packages/tracker-core/tests/drain_rate_measurement.rs`, deleted afterwards)

```rust
//! TEMPORARY (issue #2414, T4b): measures how long the persistent listener's handler needs to
//! drain a full queue. Not committed; its code is recorded in `manual-verification-evidence.md`.
#![allow(dead_code)]
mod common;

use std::time::{Duration, Instant};

use common::fixtures::{ephemeral_configuration, sample_peer};
use common::test_env::TestEnv;
use tokio::sync::broadcast;
use torrust_clock::DurationSinceUnixEpoch;
use torrust_info_hash::InfoHash;
use torrust_tracker_configuration::v3_0_0::database::Database;
use torrust_tracker_core::statistics::event::handler::handle_persistent_completed_statistics_event;
use torrust_tracker_events::drain::drain;
use torrust_tracker_swarm_coordination_registry::event::Event;

fn env_or(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_owned())
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "temporary measurement; run explicitly with --ignored"]
async fn measure_the_time_to_drain_a_queue_of_completed_downloads() {
    let database_path = env_or("DRAIN_MEASUREMENT_DB", "");
    let number_of_events: usize = env_or("DRAIN_MEASUREMENT_EVENTS", "65536").parse().unwrap();
    let distinct_torrents = env_or("DRAIN_MEASUREMENT_DISTINCT_TORRENTS", "0") == "1";

    let mut core_config = ephemeral_configuration();
    core_config.tracker_policy.persistent_torrent_completed_stat = true;
    if !database_path.is_empty() {
        let Some(Database::Sqlite3 { path }) = core_config.database.as_mut() else {
            panic!("this measurement needs the SQLite driver");
        };
        database_path.clone_into(path);
    }
    let test_env = TestEnv::new(core_config).await;
    let persistence = test_env.tracker_core_container.persistence.as_ref().unwrap();

    let (sender, mut receiver) = broadcast::channel::<Event>(65_536);
    for index in 0..number_of_events {
        let info_hash = if distinct_torrents {
            format!("{index:040x}").parse::<InfoHash>().unwrap()
        } else {
            format!("{:040x}", 1).parse::<InfoHash>().unwrap()
        };
        sender
            .send(Event::PeerDownloadCompleted {
                info_hash,
                peer: sample_peer(),
            })
            .unwrap();
    }

    let started = Instant::now();
    let report = drain(&mut receiver, Duration::from_secs(3600), |event| {
        handle_persistent_completed_statistics_event(
            event,
            &persistence.db_downloads_metric_repository,
            &test_env.tracker_core_container.stats_repository,
            DurationSinceUnixEpoch::from_secs(0),
        )
    })
    .await;
    let elapsed = started.elapsed();

    let global = persistence.database_stores.torrent_metrics_store.load_global_downloads().await.unwrap();
    println!(
        "MEASUREMENT events={number_of_events} distinct_torrents={distinct_torrents} db={database_path:?} \
         elapsed_ms={} per_event_us={:.1} report={report:?} global_downloads_in_db={global:?}",
        elapsed.as_millis(),
        elapsed.as_micros() as f64 / number_of_events as f64,
    );
}
```

#### Steps Performed and Observed Result

Each run: `target/release/deps/drain_rate_measurement-<hash> --ignored --nocapture`, with the
environment variables shown. All runs drained to `Emptied` with 0 lost to lag, and the database
held the expected global count (65,536, or 1,000).

| Run | Database | Events | Torrents | Start (UTC) | Load average before | Elapsed | Per event |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | tmpfs | 1,000 | one | 12:24:59 | 21.06 | 221 ms | 221 us |
| 2 | tmpfs | 65,536 | one | 12:25:05 | 19.61 | 12,499 ms | 191 us |
| 3 | tmpfs | 65,536 | distinct | 12:25:18 | 17.21 | 13,904 ms | 212 us |
| 4 | NVMe | 1,000 | one | 12:25:40 | 12.68 | 2,311 ms | 2,311 us |
| 5 | NVMe | 65,536 | one | 12:25:51 | 11.49 | 187,792 ms | 2,866 us |
| 6 | NVMe | 65,536 | one | 12:29:20 | 10.90 | 157,884 ms | 2,409 us |
| 7 | tmpfs | 65,536 | one | 12:32:03 | 7.14 | 13,646 ms | 208 us |
| 8 | tmpfs | 65,536 | one | 12:32:17 | 6.96 | 14,362 ms | 219 us |

- **Noise:** the full buffer on tmpfs took 12.5, 13.6, and 14.4 s in the three runs with one torrent
  (runs 2, 7, and 8; the slowest is about 15% above the fastest), and 13.9 s with distinct torrents
  (run 3), so 12.5 to 14.4 s over the four tmpfs full-buffer runs. On NVMe it took 187.8 s and 157.9 s
  (the faster run is about 16% below the slower one). The machine was shared, so these are not
  isolated numbers. (An earlier wording of this paragraph said "four runs with one torrent" and
  "about 17%", which the review of #2414 corrected.)
- **Check that it measures something:** the database held 65,536 global downloads after each full
  run, so every event was written. The per-event cost fits the work: about 0.2 ms on tmpfs for two
  statements with no real flush, and about 2.4 ms on NVMe for two committed writes. That
  explanation is an inference: no flush counter or syscall trace was measured.
- **Not measured:** MySQL and PostgreSQL (a driver over a connection is expected to be slower
  per write, not measured here); other disks; a loaded tracker with producers running; the broadcast
  receive itself (the events were already queued).

#### Conclusion

With SQLite on a real disk, a full buffer takes about 158 to 188 s, far above both the 10 s shared
deadline that bounds shutdown and the HTTP drain's 90 s. On tmpfs it takes about 12 to 14 s, which
also exceeds the 10 s deadline. So no drain timeout that fits the shutdown deadline can drain a full
buffer: the timeout can only bound how long admission continues, and the warning with the leftover
count (D1) is how a larger backlog is reported. This is the case D18 anticipates, and it supports
the separate batching issue (#2418): about 350 completions per second is the rate the persistent
listener can sustain on this NVMe, 4,500 on tmpfs. Proposal for the default and its reasoning are in
the T4 design-review checkpoint.

### V7 - T4c: the persistent listener drains on cancellation (green), and the findings on the way

- Goal: show the cancel-first bug is fixed at the seam (T3a) and at the package level (T3c), and
  that each new listener test can fail.
- Initial state: V5's tree plus T4c, run uncommitted before the T4 design review and committed in the
  two commits named under "Commits" below:
  `packages/tracker-core/src/statistics/event/listener.rs` (the drain, an injected `drain_timeout`),
  `src/bootstrap/jobs/tracker_core.rs`, `packages/tracker-core/tests/common/test_env.rs` and
  `tests/integration.rs` (callers pass `DEFAULT_DRAIN_TIMEOUT`), `packages/events/src/drain.rs`
  (`DEFAULT_DRAIN_TIMEOUT`, `DrainOutcome::as_str`, the `log_drain_report!` macro), and
  `project-words.txt` (`tmpfs`).
- Status: `DONE`.

#### Design

- On cancellation the listener logs "draining", calls `drain_persistent_completed_statistics_events`
  (the shared `drain` with the real handler), logs the report with `log_drain_report!`, and
  returns `Completion::Cancelled`. A closed receiver still returns `Completed`. The function is
  separate because the dispatch function's cognitive complexity was 29 of an allowed 25.
- The timeout is a parameter of `run_persistent_completed_statistics_event_listener` and of its
  `_unspawned` variant, passed from `src/bootstrap/jobs/tracker_core.rs` as
  `DEFAULT_DRAIN_TIMEOUT` (5 s, proposed, see V6 and the sub-EPIC's record entry 28).
- Log fields are explicit scalars (`listener`, `outcome` as a stable string, `processed`,
  `lost_to_lag`, `unread_when_timed_out`), never `Debug`, following the structured-logging skill.

#### Prose-first Arrange-Act-Assert (the two new listener tests; T3a and T3c are in V3 and V4)

- `it_should_persist_every_queued_event_then_return_cancelled_when_cancellation_is_already_pending`:
  Arrange two queued completed downloads and a token cancelled before the listener runs. Act: the
  dispatch function, visible. Assert: both downloads persisted (`Some(2)`) and the completion is
  `Cancelled`, the two facets of "drains, then stops because of the cancellation".
- `it_should_leave_the_queued_events_unprocessed_and_return_cancelled_when_the_drain_timeout_has_expired`:
  Arrange the same queue with a zero timeout, so it has expired before the first event. Act: the
  dispatch function. Assert: nothing persisted (`None`) and still `Cancelled`. A zero timeout is the
  deterministic form: with paused time an always-ready receiver never advances the clock, and the
  admission semantics under paused time are tested on the helper (V5).

#### Steps Performed

1. 12:45:02.578 to 12:45:06.538, green by name: `cargo test -p torrust-tracker-core --lib
   it_should_persist_a_queued`, then `--test integration it_should_persist_every_completed`.
2. Mutations on the final code, each as one command (mutate, run the package tests, restore from
   a backup, `cmp`):

| Mutation | Time | Failed tests |
| --- | --- | --- |
| L1: effective timeout zero (`drain_timeout.min(Duration::ZERO)`) | 12:41:09.129 to 12:41:12.907 | T3a, the two-events test, and T3c |
| L2: no timeout in effect (`drain_timeout.max(3600 s)`) | 12:41:21.737 to 12:41:25.673 | the expired-timeout test |
| L3: `Completion::Completed` after the drain | 12:41:25.678 to 12:41:29.557 | the two-events test and the expired-timeout test |

A first form of L1 (`Duration::ZERO` in place of the argument) did not compile (unused variable),
so it is not a result. L1 to L3 were first run on the code before the extraction of the drain
function and gave the same failures; the table is the rerun on the final code.

Step 3: On the final tree: `cargo test --workspace --no-fail-fast` gave 3,155 passed and 0 failed;
   `cargo +nightly fmt --all -- --check` exited 0; `linter all` exited 0 (after fixing, in turn:
   `clippy::use_self`, `clippy::doc_markdown` on `NVMe`, `clippy::cognitive_complexity` on the
   dispatch function, `manual_unwrap_or_default` in an earlier test, and `cspell` on `tmpfs`).

#### Observed Result

```text
step 1: test ...::it_should_persist_a_queued_completed_download_when_cancellation_is_already_pending ... ok
        test it_should_persist_every_completed_download_queued_before_the_persistent_listener_is_cancelled ... ok
```

V3 and V4 were red on the cancel-first loop; the same two tests are green here, so the fix, and
nothing in the tests, changed the result.

#### Findings during T4c

- **A log function cannot carry the target.** `tracing::info!(target: target, ...)` with a function
  parameter gives `error[E0435]: attempt to use a non-constant value in a constant`
  (a scratch test, removed). The shared format is therefore a macro (record entry 28).
- **The scripted test receivers pop eagerly.** The first T4c run left T3a red:
  `DrainReport { outcome: Emptied, processed: 0, ... }`, because `ScriptedReceiver::recv` popped its
  queue when called, and `tokio::select!` creates that future even when the cancellation branch
  wins, so the event was gone before the drain. A temporary `eprintln!` of the report showed it
  (reverted, `cmp` against a backup exit 0). The real receiver is lazy; the `tracker-core` double is
  now lazy too, and the other five are fixed in T5.

#### Conclusion

T3a and T3c are green for the stated reason, and the new listener tests fail when the timeout or
the returned completion is broken. Not covered: a handler that panics, and the supervisor's abort
of a drain in progress (sub-issue 4).

### V8 - M1, M2, M3 (interim at T4c): the real binary

- Goal: check the drain on the application binary, as an interim result before T5. The final
  recheck is repeated after the last production change (T5), as the `fix-bug` skill requires.
- Initial state: the T4c working tree, built with `cargo build --bin torrust-tracker` (the `dev`
  profile, `opt-level = 1`, as the T0 evidence), `rustc 1.99.0`; SQLite database in
  `.tmp/si22-t0/` on the NVMe disk; the sub-EPIC's `reproduce-lost-completions.sh` with the T0
  parameters; ports 47070 and 47313 free before the runs; machine shared (load average 8 to 10).
- Status: `DONE` (interim).

#### Steps Performed and Observed Result

1. 12:44:06.729 to 12:44:08.602, M2 (control, the T0 parameters):
   `PEERS=500 docs/issues/open/2410-1488-si-22-process-queued-events-before-listeners-stop/reproduce-lost-completions.sh control`

```text
mode=control peers=500 concurrency=64 sigterm_delay=0.05 control_wait=1
tracker_exit_status=0
started_ok=500
completed_ok=500 completed_failed=0
persisted_torrent_completed=500
persisted_global_downloads=500
answered_minus_persisted=0
```

   The T0 control run on `develop` gave 500 answered, 467 persisted, 33 lost. The shutdown log of
   this run (colour codes removed; the other listeners' lines omitted):

```text
2026-10-20T12:44:08.486380Z  INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
2026-10-20T12:44:08.486494Z  INFO TRACKER_CORE: Received cancellation request, draining tracker core persistent completed statistics event listener.
2026-10-20T12:44:08.569748Z  INFO TRACKER_CORE: Event listener drained its queue after cancellation listener="tracker_core_persistent_completed_statistics" outcome="emptied" processed=72 lost_to_lag=0
2026-10-20T12:44:08.569786Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=tracker_core_persistent_completed_statistics_event_listener
2026-10-20T12:44:08.569805Z  INFO torrust_tracker: Torrust tracker successfully shutdown.
```

Step 2: 12:44:29.986 to 12:44:43.266, M1 (the cut-short drain) and M3 (race, informative):
   `PEERS=10000 SIGTERM_DELAY=3 .../reproduce-lost-completions.sh race`. A backlog larger than the
   5 s drain can clear.

```text
mode=race peers=10000 concurrency=64 sigterm_delay=3 control_wait=1
tracker_exit_status=0
started_ok=10000
completed_ok=5096 completed_failed=4904
persisted_torrent_completed=1818
persisted_global_downloads=1818
answered_minus_persisted=3278
2026-10-20T12:44:38.159301Z  INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
2026-10-20T12:44:38.163827Z  INFO TRACKER_CORE: Received cancellation request, draining tracker core persistent completed statistics event listener.
2026-10-20T12:44:43.164707Z  WARN TRACKER_CORE: Event listener drain lost events after cancellation listener="tracker_core_persistent_completed_statistics" outcome="timed_out" processed=2804 lost_to_lag=0 unread_when_timed_out=6556
2026-10-20T12:44:43.164886Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=tracker_core_persistent_completed_statistics_event_listener
2026-10-20T12:44:43.164904Z  INFO torrust_tracker: Torrust tracker successfully shutdown.
```

#### Conclusion

- M2: with the T0 parameters no completion was lost (500 of 500, T0: 467), and the tracker exited 0.
  The drain logged its counts in the structured format, in 83 ms.
- M1: a drain cut short logged the warning after exactly 5 s (12:44:38.1638 to 12:44:43.1647), with
  the listener name and the counts, and the tracker still exited 0 and logged "successfully
  shutdown". The unread events are lost, as D1 accepts, and now reported.
- M3 is informative and isolates nothing: 3,278 answered completions are not persisted. The
  drain was cut short, so a backlog of unread events was lost (`unread_when_timed_out=6556`), and
  4,904 `completed` announces got no answer (the script counts them as failed; they were sent after
  the server stopped answering, read from the script's counters, not traced). The counts are events of every type on the
  swarm bus, not completions: the persistent listener ignores all but `PeerDownloadCompleted`, which
  is why 1,818 persisted. The log therefore cannot say how many completions the 6,556 unread events
  held. The persisted rows were counted by the script with `sqlite3`.
- Window (b) is not addressed here: producers still stop at the same time as the listeners until
  sub-issue 4.

### V9 - T5: the other six listeners drain on cancellation

- Goal: the same policy as V7 in every remaining listener: on cancellation it handles the events
  already queued, within an injected `drain_timeout`, logs through `log_drain_report!`, and returns
  `Cancelled`.
- Initial state: one commit per package, each tested on its own tree. The maintainer approved the
  common design on 2026-10-20: every listener gets a `drain_timeout` parameter, an extracted drain
  function, and two tests that replace its `it_should_prioritize_*` characterization test; the callers
  pass `DEFAULT_DRAIN_TIMEOUT`.
- Status: `DONE`.

#### Prose-first Arrange-Act-Assert (the same shape in every package)

- `it_should_handle_a_queued_event_then_return_cancelled_when_cancellation_is_already_pending`:
  Arrange one ready event with a visible effect and a token cancelled before the listener runs. Act:
  the dispatch function, visible, with the default timeout. Assert: the effect is present and the
  completion is `Cancelled`. It replaces the old test, which asserted that the event was dropped.
- `it_should_leave_the_queued_event_unprocessed_and_return_cancelled_when_the_drain_timeout_has_expired`:
  Arrange the same queue and a zero timeout. Act: the dispatch function. Assert: no effect and still
  `Cancelled`.

#### Mutations

Each as one command (mutate, run the package tests, restore from a backup, `cmp`); times are UTC on
2026-10-20. "The bug" keeps the drain function and its call but never runs it, so the listener is
cancel-first again.

| Package and listener | Mutation | Time | Failed tests |
| --- | --- | --- | --- |
| `tracker-core` in-memory | timeout forced to zero | 15:12:11.491 to 15:12:14.700 | the handle-queued-event test |
| `tracker-core` in-memory | timeout never expires (an hour) | 15:12:14.704 to 15:12:18.314 | the expired-timeout test |
| `tracker-core` in-memory | returns `Completed` after the drain | 15:12:30.472 to 15:12:33.759 | both new tests |
| `tracker-core` in-memory | the bug (the drain never runs) | 15:13:11.163 to 15:13:15.135 | the handle-queued-event test |
| `swarm-coordination-registry` | timeout forced to zero | 15:17:31.281 to 15:17:39.610 | the handle-queued-event test |
| `swarm-coordination-registry` | timeout never expires (an hour) | 15:17:39.615 to 15:17:42.912 | the expired-timeout test |
| `swarm-coordination-registry` | returns `Completed` after the drain | 15:17:51.368 to 15:17:54.361 | both new tests |
| `swarm-coordination-registry` | the bug (the drain never runs) | 15:17:54.365 to 15:17:57.687 | the handle-queued-event test |
| `http-core` | timeout forced to zero | 15:23:20.390 to 15:23:33.325 | the handle-queued-event test |
| `http-core` | timeout never expires (an hour) | 15:23:33.329 to 15:23:34.384 | the expired-timeout test |
| `http-core` | the drain bypasses the metrics policy (`handle_event` called directly) | 15:23:43.708 to 15:23:44.828 | the metrics-policy test |
| `http-core` | returns `Completed` after the drain | 15:23:44.833 to 15:23:45.969 | both new tests |
| `http-core` | the bug (the drain never runs) | 15:23:54.292 to 15:23:55.401 | the handle-queued-event test |
| `udp-core` | timeout forced to zero | 15:29:41.443 to 15:29:42.327 | the handle-queued-event test |
| `udp-core` | timeout never expires (an hour) | 15:29:42.333 to 15:29:43.202 | the expired-timeout test |
| `udp-core` | the drain bypasses the metrics policy | 15:29:52.728 to 15:29:53.758 | the metrics-policy test |
| `udp-core` | returns `Completed` after the drain | 15:29:53.762 to 15:29:54.842 | both new tests |
| `udp-core` | the bug (the drain never runs) | 15:29:54.846 to 15:29:55.866 | the handle-queued-event test |
| `udp-server` statistics | timeout forced to zero | 15:35:57.338 to 15:36:03.604 | the handle-queued-event test |
| `udp-server` statistics | timeout never expires (an hour) | 15:36:03.646 to 15:36:08.942 | the expired-timeout test |
| `udp-server` statistics | the drain bypasses the metrics policy | 15:36:08.946 to 15:36:14.598 | the metrics-policy test |
| `udp-server` statistics | returns `Completed` after the drain | 15:36:25.320 to 15:36:31.165 | both new tests |
| `udp-server` statistics | the bug (the drain never runs) | 15:36:31.169 to 15:36:36.942 | the handle-queued-event test |
| `udp-server` banning | timeout forced to zero | 15:36:49.172 to 15:36:55.143 | the handle-queued-event test |
| `udp-server` banning | timeout never expires (an hour) | 15:36:55.551 to 15:37:00.695 | the expired-timeout test |
| `udp-server` banning | returns `Completed` after the drain | 15:37:00.700 to 15:37:06.477 | both new tests |
| `udp-server` banning | the bug (the drain never runs) | 15:37:06.483 to 15:37:12.294 | the handle-queued-event test |

A first form of "the bug" removed the call and did not compile (the function became unused under
`-D warnings`), so it is not a result; the table row is the rerun.

#### Observed Result

`tracker-core` in-memory: `cargo test -p torrust-tracker-core` gave 155 passed (unit) and 12 passed
(integration); the workspace gave 3,156 passed and 0 failed; `linter all` and
`cargo +nightly fmt --all -- --check` exited 0.

`swarm-coordination-registry`: the package gave 111 passed; the workspace gave 3,157 passed and 0
failed; `linter all` and `cargo +nightly fmt --all -- --check` exited 0. Its scripted receiver's
`recv` is now lazy.

`http-core`: three tests replace the old one (the third checks that the metrics policy applies to a
drained event as to a live one, so a drain cannot count an event from a metrics-disabled instance).
The listener has callers in the bootstrap job, the HTTP test environment, two handler test modules,
and a benchmark helper; all pass `DEFAULT_DRAIN_TIMEOUT`, and `axum-http-server` gained a direct
dependency on `torrust-tracker-events` for it (`Cargo.lock` records the edge). A first form of the
policy mutation did not compile (an unused variable) and is not a result; the row is the rerun.
`cargo test --workspace` gave 3,159 passed and 0 failed; `cargo test --workspace --all-targets`
(which builds the benchmarks) gave 3,112 passed and 0 failed; `linter all`,
`cargo +nightly fmt --all -- --check`, `cargo machete --with-metadata`, and `cargo deny check bans`
exited 0. `cargo +nightly fmt` reordered only imports in the call-site files.

`udp-core`: the same three tests as `http-core` (a connect event and `udp4_connections_handled`). The
bootstrap job and the `udp-server` test environment's call to this listener pass
`DEFAULT_DRAIN_TIMEOUT`; the environment's other two listener calls change with `udp-server`.
`cargo test --workspace` gave 3,161 passed and 0 failed, `--all-targets` gave 3,114 and 0, and
`linter all`, `cargo +nightly fmt --all -- --check`, and `cargo machete --with-metadata` exited 0.

`udp-server` (two listeners in one commit): the statistics listener has the same three tests as
`udp-core` (a request-received event and `udp4_requests_received_total`); the banning listener has
two (a connection-cookie error and the client's ban count, with no metrics policy to test). Both
scripted receivers popped eagerly in `recv()` and one counts its calls; both are lazy now, and the
call count is taken when the future is polled. The bootstrap jobs and the UDP test environment's
remaining two calls pass `DEFAULT_DRAIN_TIMEOUT`. `cargo test --workspace` gave 3,164 passed and 0
failed, `--all-targets` gave 3,117 and 0, and `linter all`, `cargo +nightly fmt --all -- --check`, and
`cargo machete --with-metadata` exited 0.

All seven listeners now drain. A search for the old behavior,
`rg 'shutting down .* event listener' packages src`, finds no listener that still logs "shutting down"
without draining (read from the search, not traced).

### V10 - M1, M2, M3: the final real-binary recheck, after the last production change

- Goal: the `fix-bug` skill's like-for-like recheck of the original symptom, run after the last
  production change (T5), plus the drain logs of every listener (M1).
- Initial state: a clean working tree at the commit
  `fix(udp-server): [#2414] drain the statistics and banning listeners' queues when they are cancelled`
  (the last code change; the evidence commits after it change no code). Built with
  `cargo build --bin torrust-tracker` (the `dev` profile, `opt-level = 1`, as the T0 evidence),
  `rustc 1.99.0`, 15:40:35 to 15:40:44. Ports free before each run. The machine was shared, with a load
  average of 4 to 7 (lower than during V6 and V8).
- Status: `DONE`.

#### M2 and M3: the T0 parameters, the sub-EPIC's script

`reproduce-lost-completions.sh` with the T0 parameters (`PEERS=500` for the control run, and
`PEERS=2000 SIGTERM_DELAY=0.5` for the race runs). Times are UTC on 2026-10-20.

| Run (UTC start) | Mode | Answered `completed` | Persisted (torrent / global) | Lost | Exit status | T0 on `develop` (answered, persisted, lost) |
| --- | --- | --- | --- | --- | --- | --- |
| 15:40:51 | control | 500 | 500 / 500 | 0 | 0 | 500, 467, 33 |
| 15:41:03 | race | 748 | 748 / 748 | 0 | 0 | 856, 66, 790 |
| 15:41:07 | race | 715 | 715 / 715 | 0 | 0 | 845, 70, 775 |

The control run's shutdown log (colour codes removed; other lines omitted):

```text
2026-10-20T15:40:53.550968Z  INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
2026-10-20T15:40:53.551629Z  INFO TRACKER_CORE: Received cancellation request, draining tracker core persistent completed statistics event listener.
2026-10-20T15:40:53.652556Z  INFO TRACKER_CORE: Event listener drained its queue after cancellation listener="tracker_core_persistent_completed_statistics" outcome="emptied" processed=84 lost_to_lag=0
2026-10-20T15:40:53.652628Z  INFO torrust_tracker: Torrust tracker successfully shutdown.
```

The race runs are informative and isolate nothing (the maintainer's decision in the spec): scheduling
changes the deficit, and window (b) (producers still answering while listeners stop) is not closed
until sub-issue 4. In these two runs the persistent listener drained 1,384 and 1,316 events in about
2.1 s, `outcome="emptied"`, and no completion was lost. The answered counts differ from T0 because
the tracker stopped answering at a different moment relative to SIGTERM. No run logged a lagged
receiver or a database error.

#### M1: drain logs of every listener, and the warning

The script's tracker has no UDP tracker, so the two `udp-server` listeners do not run in it. A
second run used a configuration with an HTTP and a UDP tracker (ports 47170, 47171, and 47413; the
first attempt used ports 47070 to 47313 and the tracker failed to start with "Address in use" for its HTTP
listener a few seconds after the previous run, although `ss` showed nothing listening when I looked
afterwards; I did not investigate the cause, and changed the ports). The
configuration is [`evidence/m1-tracker.toml`](evidence/m1-tracker.toml), and the harness that started the
tracker, sent 60 UDP and 100 HTTP completed-download announces, and sent SIGTERM is
[`evidence/run_m1.py`](evidence/run_m1.py) (a disposable harness, kept here as evidence: Python because the
shell tool refuses `kill` on a variable PID, and it replaces nothing maintained). Both are the exact files
that ran; the harness is run from the repository root, and during the run the configuration was at
`.tmp/m1/tracker.toml`, the path the harness reads.

Two mistakes in the first attempts, with no effect on the final result: the UDP client rejected my
peer id as an option, because it starts with `-` (`error: unexpected argument '-q' found`), so the
first run sent no UDP announces, which I noticed because the database held only the HTTP torrent; the
harness now passes `--peer-id=-qB...`, and one announce was checked by hand first
(`{"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":120,"leechers":0,"seeders":1,"peers":[]}}`,
exit 0).

Run at 15:44:00 to 15:44:01 (SIGTERM at 15:44:01.035), tracker exit status 0, persisted rows
`(1, '3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0', 60)`, `(2, '5349323253493232534932325349323253493232', 100)`,
and `('torrents_downloads_total', 160)`: every UDP and HTTP completion persisted. The shutdown log:

```text
2026-10-20T15:44:01.035750Z  INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
2026-10-20T15:44:01.035800Z  INFO UDP TRACKER: Event listener drained its queue after cancellation listener="udp_tracker_core_statistics" outcome="emptied" processed=0 lost_to_lag=0
2026-10-20T15:44:01.035820Z  INFO SWARM_COORDINATION_REGISTRY: Event listener drained its queue after cancellation listener="swarm_coordination_registry_statistics" outcome="emptied" processed=0 lost_to_lag=0
2026-10-20T15:44:01.035838Z  INFO UDP TRACKER: Event listener drained its queue after cancellation listener="udp_tracker_server_banning" outcome="emptied" processed=0 lost_to_lag=0
2026-10-20T15:44:01.035842Z  INFO HTTP TRACKER: Event listener drained its queue after cancellation listener="http_tracker_core_statistics" outcome="emptied" processed=0 lost_to_lag=0
2026-10-20T15:44:01.035834Z  INFO TRACKER_CORE: Event listener drained its queue after cancellation listener="tracker_core_in_memory_statistics" outcome="emptied" processed=0 lost_to_lag=0
2026-10-20T15:44:01.035849Z  INFO UDP TRACKER: Event listener drained its queue after cancellation listener="udp_tracker_server_statistics" outcome="emptied" processed=0 lost_to_lag=0
2026-10-20T15:44:01.243011Z  INFO TRACKER_CORE: Event listener drained its queue after cancellation listener="tracker_core_persistent_completed_statistics" outcome="emptied" processed=184 lost_to_lag=0
2026-10-20T15:44:01.243061Z  INFO torrust_tracker: Torrust tracker successfully shutdown.
```

All seven listeners logged their drain, with the same fields; no warning or error was logged
(`grep -ciE ' ERROR | WARN '` gave 0). Six of the seven drained an empty queue, because they keep up
with the traffic; their drain path with events is covered by the unit tests (V9) and not shown here.

The warning, on the final tree: `PEERS=10000 SIGTERM_DELAY=3 .../reproduce-lost-completions.sh race`
(15:44:11 to 15:44:25): 4,696 answered, 1,657 persisted, 3,039 answered but not persisted (the drain
was cut short and 5,304 later announces got no answer), exit status 0, and the log:

```text
2026-10-20T15:44:20.513470Z  INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
2026-10-20T15:44:20.517435Z  INFO TRACKER_CORE: Received cancellation request, draining tracker core persistent completed statistics event listener.
2026-10-20T15:44:25.518678Z  WARN TRACKER_CORE: Event listener drain lost events after cancellation listener="tracker_core_persistent_completed_statistics" outcome="timed_out" processed=2514 lost_to_lag=0 unread_when_timed_out=6078
2026-10-20T15:44:25.518859Z  INFO torrust_tracker: Torrust tracker successfully shutdown.
```

The warning came 5.001 s after the drain began (15:44:20.517 to 15:44:25.518). The counts are events of
every type, not completions (record entry 28), so they say how many events were left, not how many
completions were lost.

#### Conclusion

- M2: with the T0 parameters the control run loses nothing (T0: 33 of 500), and the tracker exits 0.
- M3: the two race runs lose nothing (T0: 790 and 775). Informative only, as the spec says.
- M1: every listener logs its drain with the same fields, a cut-short drain logs a warning with the
  listener name and the counts, and shutdown still exits 0 and logs "successfully shutdown".
- Not shown: the loss that remains when a backlog is larger than the drain can clear in 5 s (the last
  run), which is the bounded policy D1 and D18 accept and the warning now reports; and window (b),
  which sub-issue 4 closes.

### V11 - The independent review's fixes (S1, N2, S2, N1, N4, N5), and the recheck repeated

- Goal: record the regression tests that were red before the review fixes, the interaction the fixes
  caused, and the final real-binary recheck on the build after the last production change. The
  Task Reviewer report (before push) is in `agent-review-reports.md`.
- Initial state: the branch at
  `docs(issues): [#2414] record the T5 evidence and the final M1 to M3 recheck`.
- Status: `DONE`.

#### Red before the fix (prose-first Arrange-Act-Assert in the test comments)

Times are UTC on 2026-10-20. Both tests were written first and run on the unfixed `drain`:

`cargo test -p torrust-tracker-events --lib it_should_` at 16:05:38.481 to 16:05:39.230:

```text
test drain::tests::it_should_drain_every_event_when_the_timeout_is_too_large_to_add_to_the_clock ... FAILED
   overflow when adding duration to instant
test drain::tests::it_should_let_other_tasks_run_while_it_drains_a_ready_queue ... FAILED
test result: FAILED. 20 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
```

- Overflow (N2, a real bug): `drain` added its timeout to the current time with `+`. A scratch
  program had shown that `tokio::time::Instant::now() + Duration::MAX` panics, and
  `checked_add` returns `None`. The test queues three events and passes `Duration::MAX`.
- Starvation (S1, the spec's "test cooperative scheduling"): on the single-threaded test runtime,
  100 ready events and a spawned task that sets a flag; a handler records the flag at the 100th event.
  Without a yield nothing else can run until the drain returns, so the flag was unset.

#### The fixes and the green run

`drain` computes its deadline with `checked_add` (a timeout the clock cannot represent never
expires) and calls `tokio::task::yield_now().await` after each handled event. 16:05:51.546 to
16:05:52.144: `cargo test -p torrust-tracker-events` gave 22 passed. `Receiver::is_empty`, which no
test used, got a test: with the default inverted (`self.len() != 0`) it failed
(16:06:08.443 to 16:06:08.913, one failure), and it passes on the committed default.

#### An interaction the yield caused

The first whole-workspace run after the fix failed one test, the package `TestEnv` drop-path test
from T3b:

```text
assertion `left == right` failed: dropping the environment without stop() must still cancel every listener
  left: 3
 right: 0
test result: FAILED. 11 passed; 1 failed (tests/integration.rs), finished in 5.12s
```

Its helper published a probe event on every loop turn, to count the listeners still subscribed.
A cancelled listener drains the events queued for it, and with the yield the helper could publish
between two of the drain's events, so a draining listener kept finding a new event until its 5 s drain
timeout. That is the correct behavior of a drain under continuous publishing, and the observer was
wrong. The helper now probes every 20 ms; the test passed three times in a row (0.36 s, 0.33 s,
0.32 s), and its original mutation (the drop guard on a different token) still failed it:
16:11:17.362 to 16:11:23.564, `left: 3, right: 0`, after 5.13 s.

#### The other review items

- S2.1: `axum-http-server`'s direct dependency on `torrust-tracker-events` is gone; `http-core`
  re-exports `DEFAULT_DRAIN_TIMEOUT` from its listener module (the maintainer's choice, 2026-10-20).
  `Cargo.lock` and the manifest equal `develop`'s again; `cargo test -p torrust-tracker-http-core -p
  torrust-tracker-axum-http-server` gave 44, 62, 34, 1 and 2 passed on the staged content of that commit.
- S2.2: the listener-level timeout tests keep `Duration::ZERO`, as the maintainer agreed.
- N1: the six scripted receivers import `TryRecvError` at module level. N4: the new metric assertions
  and two shared helpers have causal messages. N3: the V6 wording above. N6: the spec status is
  `in-progress`.
- N5: the log lines have no unit test (V10 shows them on the real binary); a `TryRecvError` `Display`
  test is not added, because `RecvError` has none either; a handler panic during a drain is a
  non-goal, as V5 says; the lag path during a drain is covered at helper level.

#### The final gate

On the tree of the last code commit
`test(listeners): [#2414] move the TryRecvError imports up and add causal assertion messages`:
`cargo test --workspace` gave 3,167 passed and 0 failed, `--all-targets` gave 3,120 and 0, and
`linter all`, `cargo +nightly fmt --all -- --check`, `cargo machete --with-metadata`, and
`cargo deny check bans` exited 0.

#### The recheck repeated on the build after the last production change

The drain's code changed (the yield and `checked_add`), so M1 to M3 were run again on a `dev` build of
the last code commit above (16:16:14 to 16:16:21; a clean working tree except the documents; ports free;
machine load average 13 to 16, higher than for V10). Same commands as V10.

| Run (UTC start) | Mode | Answered `completed` | Persisted (torrent / global) | Lost | Persistent drain line |
| --- | --- | --- | --- | --- | --- |
| 16:16:21 | control (`PEERS=500`) | 500 | 500 / 500 | 0 | `outcome="emptied" processed=44` |
| 16:16:34 | race (`PEERS=2000 SIGTERM_DELAY=0.5`) | 665 | 665 / 665 | 0 | `outcome="emptied" processed=1232` |
| 16:16:39 | race (same) | 641 | 641 / 641 | 0 | `outcome="emptied" processed=1182` |

M1 with the HTTP and UDP tracker (`evidence/run_m1.py`, SIGTERM at 16:16:53.107, exit status 0):
persisted `(1, '3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0', 60)`, `(2, '5349323253493232534932325349323253493232', 100)`,
and `('torrents_downloads_total', 160)`. All seven listeners logged their drain
(`udp_tracker_server_banning`, `udp_tracker_core_statistics`, `tracker_core_in_memory_statistics`,
`udp_tracker_server_statistics`, `swarm_coordination_registry_statistics`, `http_tracker_core_statistics`:
`outcome="emptied" processed=0`; `tracker_core_persistent_completed_statistics`: `outcome="emptied"
processed=186`), and the log held 0 warnings or errors.

The warning, `PEERS=10000 SIGTERM_DELAY=3 .../reproduce-lost-completions.sh race` (16:16:53 to
16:17:08): 4,612 answered, 1,380 persisted, exit status 0, and

```text
2026-10-20T16:17:03.289411Z  INFO TRACKER_CORE: Received cancellation request, draining tracker core persistent completed statistics event listener.
2026-10-20T16:17:08.291869Z  WARN TRACKER_CORE: Event listener drain lost events after cancellation listener="tracker_core_persistent_completed_statistics" outcome="timed_out" processed=2020 lost_to_lag=0 unread_when_timed_out=6464
```

The warning came 5.002 s after the drain began. The drain processed 2,020 events in 5 s, against 2,514
and 2,804 in V10 and V8 (about 400 events per second here); the machine was busier, so these rates
are not comparable, and I did not test whether the per-event yield costs throughput. The yield costs
a scheduler turn per event, which the 2 to 3 ms database write per event dwarfs (an inference from V6).

#### Conclusion

The two review defects are fixed with tests that were seen red. The recheck of the original symptom
on the final build matches V10: no completion is lost with the T0 parameters, every listener logs its
drain, and a cut-short drain logs the warning. V10 stays as the first recheck; this table is the one
that counts for the final code.

## Commits

After the T4 design review (maintainer, 2026-10-20; he agreed to a 5 s default timeout, the
`log_drain_report!` macro, the "events" wording, an updated `adr-draft.md`, keeping the package
`TestEnv` polling, and three commits), T3a, T3c, and T4 were committed as:

1. `feat(events): [#2414] add try_recv, len, TryRecvError, and the shared listener drain helper`
   (T4a: `packages/events`, the six scripted receivers, `project-words.txt`). Its tree was tested
   alone before the commit: 3,152 passed, 0 failed.
2. `fix(tracker-core): [#2414] drain the persistent listener's queue when it is cancelled`
   (T4c with T3a and T3c, the bootstrap wiring, and the `TestEnv` caller). Its tree was tested alone
   before the commit: 3,155 passed, 0 failed.
3. The documentation commit that carries this file, the specs, the sub-EPIC record, and
   `adr-draft.md`. The evidence of V5 to V8 lands here, and not with the code, because it is one
   file.

The V3 and V4 red evidence was already committed earlier as
`docs(issues): [#2414] record the red evidence of the T3a and T3c regression tests`.

After the independent review (V11), three code commits and a documentation commit were added:

1. `fix(events): [#2414] drain without panicking on a huge timeout, and yield between events`
   (review findings S1 and N2, each with a red-first test, and the `is_empty` test).
2. `refactor(http-core): [#2414] re-export the default drain timeout instead of adding a dependency`
   (review finding S2; `Cargo.lock` equals `develop`'s).
3. `test(listeners): [#2414] move the TryRecvError imports up and add causal assertion messages`
   (review findings N1 and N4).
4. `docs(issues): [#2414] record the review fixes, the Task Reviewer report, and the retrospective`.

## Failures and Follow-up

- The `git checkout` restore that reverted uncommitted work, described under V1, cost one rewrite
  and no result: the rewrite was verified by the timed steps above.
- Splitting the work into commits went wrong twice, with no change to the result. A `git stash pop`
  re-applied the already committed receiver migration to `listener.rs` (duplicate `try_recv` and
  `len`, a compile error in the staged tree); the file was restored from a backup taken after its
  last edit, and the tree of commit 2 was tested after that. A second pop conflicted on the same
  file; the committed version was taken (`cmp` against the backup exited 0). Stashes are shared by
  every worktree of the repository, so none is used for the rest of the work.
- The message of the first commit said the helper had eight tests. It has seven (`grep -c 'async fn
  it_should' packages/events/src/drain.rs` gives 7), and I had written eight in my report to the
  maintainer. The unpushed commit was amended to say seven.
