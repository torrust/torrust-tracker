---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2414-2410-si-22-2-drain-listener-queues-on-shutdown/ISSUE.md
last-updated-utc: "2026-10-20 12:02"
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

## Failures and Follow-up

- The `git checkout` restore that reverted uncommitted work, described under V1, cost one rewrite
  and no result: the rewrite was verified by the timed steps above.
