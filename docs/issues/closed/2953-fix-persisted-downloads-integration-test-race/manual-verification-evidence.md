---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2953-fix-persisted-downloads-integration-test-race/ISSUE.md
last-updated-utc: "2026-11-19 18:23"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Preserving Verification Artifacts

All outputs this evidence relies on are recorded inline below.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-18, 17:37 to 17:40
- Artifact under test: `torrust-tracker-core` integration test binary built from `develop` at
  `96923f0d2` (Merge torrust/torrust-tracker#2945), unmodified except for the temporary probe in V2.
- Operating system / environment: Linux 7.0.0-43-generic, 32 CPUs; stable Rust toolchain
  `rustc 1.100.0 (0ef1bc386 2026-11-09)`, `cargo 1.100.0 (3acc79c0c 2026-11-05)`; ephemeral
  SQLite database created by the test environment.
- Prerequisites and setup performed: none beyond building the test binary.
- Original symptom (reported by the maintainer's merge session, merge of PR 2946):
  `it_should_persist_the_global_number_of_completed_peers_into_the_database` failed at
  `packages/tracker-core/tests/integration.rs:253` with `left: 0, right: 1` during the
  merge-script pre-push run of all tests.
- Hypothesis: `TestEnv::wait_for_global_downloads_persisted` waits for the database value only,
  and `increase_global_downloads` in `statistics/event/handler.rs` increments the in-memory
  counter after the database write, so the assertion can read the counter before the increment.

## Verification Processes

### V1 - Plain repeated runs on `develop`

- Goal: measure how often the race shows up without help.
- Initial state: clean worktree at `96923f0d2`.
- Status: `DONE`

#### Steps Performed

1. Built the integration test binary with
   `cargo test -p torrust-tracker-core --test integration --no-run` (stable Rust toolchain).
2. 17:37:45.603 - ran the binary 300 times filtered to the test:
   `$BIN it_should_persist_the_global_number_of_completed_peers_into_the_database -q`.
3. 17:39:07.139 - after step 2 ended, ran the unfiltered binary (all 12 tests in parallel) 100 times:
   `$BIN -q`. Ended 17:39:37.229.

#### Observed Result

```text
isolated: pass=300 fail=0
whole binary: pass=100 fail=0
```

#### Conclusion

The race does not show up in plain runs on this machine. It needs heavier load, such as the full
workspace pre-push run.

### V2 - Reproduction with a widened window

- Goal: observe the wrong outcome itself.
- Initial state: clean worktree at `96923f0d2`; V1 finished.
- Status: `DONE`
- Outcome: **Reproduced**.

#### Steps Performed

1. 17:39:42.426 - added a temporary sleep between the database write and the counter increment in
   `packages/tracker-core/src/statistics/event/handler.rs`. The applied diff, verbatim:

   ```diff
   @@ -120,2 +120,3 @@ async fn increase_global_downloads(
                tracing::debug!("Global number of downloads increased");
   +            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                let _unused = stats_repository
   ```

2. 17:39:42.433 - ran
   `cargo test -p torrust-tracker-core --test integration it_should_persist_the_global_number_of_completed_peers_into_the_database`
   (stable Rust toolchain). Ended 17:39:44.651.
3. 17:39:44.651 - reverted the probe with `git checkout -- packages/tracker-core/src/statistics/event/handler.rs`;
   `git diff --quiet HEAD -- packages/tracker-core/src/statistics/event/handler.rs` exited 0 after
   the restore (17:39:44.663).

#### Observed Result

```text
running 1 test
test it_should_persist_the_global_number_of_completed_peers_into_the_database ... FAILED

---- it_should_persist_the_global_number_of_completed_peers_into_the_database stdout ----

thread 'it_should_persist_the_global_number_of_completed_peers_into_the_database' (118236) panicked at packages/tracker-core/tests/integration.rs:253:5:
assertion `left == right` failed
  left: 0
 right: 1

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.16s
```

#### Conclusion

Same assertion, same line and same values as the original failure. The hypothesis holds: the
helper returns once the database holds `1`, while the counter is still `0`.

### V3 - Red on the implementation branch

- Goal: see the maintained regression test fail against the unchanged helper, and see the
  widened-window probe fail the integration test on the same branch.
- Initial state: branch `2953-fix-persisted-downloads-integration-test-race` cut from `develop` at
  `1f85267e8` (Merge torrust/torrust-tracker#2954), with `HEAD` still at `1f85267e8` and the new
  tests as an uncommitted change in `packages/tracker-core/tests/common/test_env.rs`, the helper
  unchanged. That change was committed right after these runs, unchanged, as
  `test(tracker-core): [#2953] add regression tests for the persisted downloads wait helper`. Same machine and toolchain as V1 and V2 (`rustc 1.100.0`).
- Status: `DONE`
- Outcome: **Red**, as planned.

#### Regression-test boundary

The defect is the helper's wait condition, so the tests sit at the helper: two tests in a
`tests` module of `packages/tracker-core/tests/common/test_env.rs`, next to the fixture they
guard. They build the environment with `TestEnv::new` (persistence on, no listener), so only the
test changes the database value and the in-memory counter, and no timing luck is involved. The
spec planned one test in two parts; it was split into two tests so each has one reason to fail
(see V5).

- `it_should_keep_waiting_for_the_persisted_global_downloads_when_only_the_database_has_reached_the_expected_value`
  guards the bug: the database holds `1`, the counter is `0`, and the wait must still be pending
  after 500 ms.
- `it_should_finish_waiting_for_the_persisted_global_downloads_when_the_database_and_the_in_memory_counter_have_reached_the_expected_value`
  guards the fix against waiting too long: with both at `1`, the wait must finish within 500 ms.
  It passes before and after the fix.

#### Steps Performed

1. 09:17:50 - ran `cargo test -p torrust-tracker-core --test integration -- common::test_env::tests`.
2. 09:18:00 - added the V2 probe (the same one-line 300 ms sleep after
   `tracing::debug!("Global number of downloads increased");` in
   `packages/tracker-core/src/statistics/event/handler.rs`, now at line 121) and ran
   `cargo test -p torrust-tracker-core --test integration -- it_should_persist_the_global_number_of_completed_peers_into_the_database`.
3. Reverted the probe with `git checkout -- packages/tracker-core/src/statistics/event/handler.rs`;
   `git status --short` then listed only `packages/tracker-core/tests/common/test_env.rs`.

#### Observed Result

Step 1, the helper test against the unchanged helper:

```text
running 2 tests
test common::test_env::tests::it_should_keep_waiting_for_the_persisted_global_downloads_when_only_the_database_has_reached_the_expected_value ... FAILED
test common::test_env::tests::it_should_finish_waiting_for_the_persisted_global_downloads_when_the_database_and_the_in_memory_counter_have_reached_the_expected_value ... ok

---- common::test_env::tests::it_should_keep_waiting_for_the_persisted_global_downloads_when_only_the_database_has_reached_the_expected_value stdout ----

thread '...' (720746) panicked at packages/tracker-core/tests/common/test_env.rs:387:9:
the wait for 1 persisted global download should still be pending after 500ms while the database holds 1 and the in-memory persisted downloads counter is still 0

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.01s
```

Step 2, the probe against the unchanged helper:

```text
thread 'it_should_persist_the_global_number_of_completed_peers_into_the_database' (721371) panicked at packages/tracker-core/tests/integration.rs:253:5:
assertion `left == right` failed
  left: 0
 right: 1

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.16s
```

#### Conclusion

The new test fails for the reason the bug gives: the helper returns at once (`0.01s`) because
the database already holds `1`, while the counter is `0`. On the branch, the probe still gives
the exact merge-run failure.

### V4 - Green and like-for-like recheck

- Goal: see the new tests pass with the fix, see the probe no longer break the integration test,
  and repeat the V1 plain runs.
- Initial state: the commit
  `fix(tracker-core): [#2953] wait for the in-memory persisted downloads counter in the test helper`
  on top of the T2 commit; clean worktree apart from this evidence file. Same machine and
  toolchain as V1 to V3 (`rustc 1.100.0`). No production code changed after these runs.
- Status: `DONE`
- Outcome: **Green**.

#### Steps Performed

1. 09:25:09.983 - ran `cargo test -p torrust-tracker-core --test integration -- common::test_env::tests`.
2. Added the same probe as V2 and V3 in `packages/tracker-core/src/statistics/event/handler.rs`.
   09:25:10.662 - ran
   `cargo test -p torrust-tracker-core --test integration -- it_should_persist_the_global_number_of_completed_peers_into_the_database`.
3. 09:25:13.383 - reverted the probe with `git checkout -- packages/tracker-core/src/statistics/event/handler.rs`;
   `git diff --quiet HEAD -- packages/tracker-core/src/statistics/event/handler.rs` exited 0.
4. Built the binary with `cargo test -p torrust-tracker-core --test integration --no-run` and, as
   in V1, ran it 300 times filtered to the test
   (`$BIN it_should_persist_the_global_number_of_completed_peers_into_the_database -q`,
   09:25:22.724 to 09:26:44.303), then 100 times unfiltered (`$BIN -q`, all 14 tests in
   parallel, 09:26:44.305 to 09:27:39.312), counting exit codes.

#### Observed Result

Step 1:

```text
test common::test_env::tests::it_should_finish_waiting_for_the_persisted_global_downloads_when_the_database_and_the_in_memory_counter_have_reached_the_expected_value ... ok
test common::test_env::tests::it_should_keep_waiting_for_the_persisted_global_downloads_when_only_the_database_has_reached_the_expected_value ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.51s
```

Step 2, with the probe:

```text
test it_should_persist_the_global_number_of_completed_peers_into_the_database ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.53s
```

Step 4:

```text
filtered pass=300 fail=0
binary pass=100 fail=0
```

#### Conclusion

With the fix, the helper waits out the 300 ms window and the integration test passes where it
failed in V2 and V3. The run takes `0.53s` against `0.16s` because the failing runs stop at
line 253, before the restarted environment: the passing run pays the probe's 300 ms, the
restarted `TestEnv::started`'s 100 ms listener-start sleep, and up to one 50 ms poll. The new tests
pass, and all 400 plain runs pass.

### V5 - Test-design review

- Goal: review the new tests and the changed fixture after focused green validation, before
  maintainer review (task T4).
- Initial state: the helper fix applied on top of the T2 commit; all 14 tests of the integration
  binary passed at 09:19:28 (`cargo test -p torrust-tracker-core --test integration`).
- Status: `DONE`; the maintainer approved the design on 2026-11-19 before V4.

#### Prose-first Arrange-Act-Assert comparison

Test that guards the bug (`it_should_keep_waiting_..._when_only_the_database_has_reached_the_expected_value`):

| Section | Temporary prose | Code that expresses it |
| ------- | --------------- | ---------------------- |
| Arrange | An environment with persistence where the database already holds 1 global download, the in-memory persisted downloads counter is still 0, and no listener runs that could change either. | `test_env_without_listeners()` and `persist_global_downloads(1)`. The counter's `0` is the initial state, so it has no code line; the first draft hid it, and a one-line comment now states it. |
| Act | Wait for 1 persisted global download, for a bounded time. | `timeout(WAIT_LIMIT, test_env.wait_for_global_downloads_persisted(1))` |
| Assert | The wait is still pending at the limit, because the counter has not reached 1. | `assert!(wait.is_err(), ...)`, with a message that names the limit and both values. |

Companion test (`it_should_finish_waiting_..._when_the_database_and_the_in_memory_counter_have_reached_the_expected_value`):

| Section | Temporary prose | Code that expresses it |
| ------- | --------------- | ---------------------- |
| Arrange | Same environment, with the database at 1 and the counter at 1. | `persist_global_downloads(1)` and `increment_persisted_downloads_counter()`, both visible. |
| Act | Wait for 1 persisted global download, for a bounded time. | Same line as above. |
| Assert | The wait finishes within the limit. | `assert!(wait.is_ok(), ...)` |

The one causal difference between the two tests, the counter increment, is visible in the second
test's Arrange and named by both test names. After the comparison the only prose kept is the
counter comment and the doc comments on the helpers and the constant.

#### Decisions and smells reviewed

- **Two tests instead of one in two parts** (the spec's plan): each now has one reason to fail.
  The first fails if the helper stops too early (the bug); the second fails if it waits too long.
- **Placement:** a `tests` module in `common/test_env.rs`, because the behavior under test is the
  fixture's and not tracker-core's. The module is compiled only into the `integration` test
  binary, which is the only binary that includes `common`.
- **Fixture helpers:** `persist_global_downloads` mirrors the existing `persist_torrent_downloads`
  (a value a previous run left in the database). `increment_persisted_downloads_counter` names
  the listener's second step. Each names one action at the level of the test narrative and hides
  only the container paths. `test_env_without_listeners` names the state (persistence on, nothing
  running) and returns the `EphemeralConfiguration` so the temporary database directory lives as
  long as the test.
- **Changed helper:** `wait_for_global_downloads_persisted` keeps its name. The counter it now
  also waits for is `tracker_core_persisted_torrents_downloads_total`, so "persisted" still
  describes both values. It keeps its single 5-second bound and 50 ms poll, and compares both
  values with `>=`. The new doc comment states the write order that makes waiting for the
  database alone insufficient.
- **Timing:** `WAIT_LIMIT` (500 ms) bounds both tests. The bug test always spends it (the wait
  must not finish), so the module adds about 0.5 s; the companion finishes on the first poll.
- **Smells checked:** no hidden production Act (the wait is visible), no expected value derived
  from production code (`1` is written by the test), no lifecycle change (no listener is started).

### V6 - Each clause of the wait condition has a failing test (review round 1)

- Goal: show that every clause of the helper's condition (database value and in-memory counter,
  each at least `expected`) is guarded by a test that fails when the clause is removed, as the
  `write-unit-test` section "Prove a Test Guards Each Clause of a Rule" requires (PR #2964
  review, finding F1).
- Initial state: the fix commit plus the uncommitted change that added the third test,
  `it_should_keep_waiting_for_the_persisted_global_downloads_when_only_the_in_memory_counter_has_reached_the_expected_value`,
  and gave the finishing test its own 4-second bound. That change was committed right after these
  runs, unchanged, as
  `test(tracker-core): [#2953] guard the database clause of the persisted downloads wait`.
- Status: `DONE`

#### Steps Performed

Each mutation edits the line `if persisted_in_database && persisted_in_memory {` in
`packages/tracker-core/tests/common/test_env.rs`, without staging it, and runs
`cargo test -p torrust-tracker-core --test integration -- common::test_env::tests`. The file was
restored from a copy after each mutation; `cmp` confirmed it matched the copy at 10:19:48.

1. 10:19:33 - no mutation.
2. 10:19:35 - first attempt: `if persisted_in_memory {` and `if persisted_in_database {`. Neither
   compiled (`error: unused variable`, turned into an error by the workspace's `-D warnings`), so
   no test ran; these results are not used.
3. 10:19:45 - M-a, database clause neutralized: `if (persisted_in_database || true) && persisted_in_memory {`.
4. 10:19:46 - M-b, in-memory clause neutralized: `if persisted_in_database && (persisted_in_memory || true) {`.

#### Observed Result

| Run | Database only (counter `0`) | Counter only (no database value) | Both at `1` |
| --- | --------------------------- | -------------------------------- | ----------- |
| No mutation | ok | ok | ok |
| M-a, database clause neutralized | ok | **FAILED** (`test_env.rs:427:9`) | ok |
| M-b, in-memory clause neutralized | **FAILED** (`test_env.rs:408:9`) | ok | ok |

M-a failure message: "the wait for 1 persisted global download should still be pending after
500ms while the in-memory persisted downloads counter holds 1 and the database holds no value".
M-b failure message: "the wait for 1 persisted global download should still be pending after
500ms while the database holds 1 and the in-memory persisted downloads counter is still 0". Each
mutated run ended `test result: FAILED. 2 passed; 1 failed`; the run without mutation ended
`test result: ok. 3 passed`.

#### Conclusion

Each clause of the condition has exactly one test that fails when that clause is removed. The
whole integration binary passed with the committed change (`15 passed; 0 failed`).

### V7 - Recheck after the review-round-2 helper change

- Goal: repeat the green checks after the last change to the helper (PR #2964 review round 2:
  fail at once on a database error, and look up the store and repository once before the loop).
- Initial state: the uncommitted helper change on top of
  `docs(issues): [#2953] record the clause mutations and correct the evidence`, committed right
  after these runs, unchanged, as
  `test(tracker-core): [#2953] fail the persisted downloads wait at once on a database error`.
  Same machine and toolchain as before. This is the last change to the helper.
- Status: `DONE`

#### Steps Performed

1. 10:28:37 - ran the three helper tests:
   `cargo test -p torrust-tracker-core --test integration -- common::test_env::tests`.
2. 10:28:39 and 10:28:40 - repeated the V6 mutations M-a and M-b; restored the file from a copy
   (`cmp` matched at 10:28:42).
3. 10:28:42 - added the V2 probe in `handler.rs` and ran
   `it_should_persist_the_global_number_of_completed_peers_into_the_database`; reverted it with
   `git checkout -- packages/tracker-core/src/statistics/event/handler.rs`, and
   `git diff --quiet HEAD -- packages/tracker-core/src/statistics/event/handler.rs` exited 0 at
   10:28:45. The check is limited to `handler.rs` because the helper change in `test_env.rs` was
   still uncommitted.
4. Repeated the V4 plain runs: 300 filtered runs (10:28:47.539 to 10:30:09.395) and 100
   whole-binary runs (10:30:09.397 to 10:31:03.470).

#### Observed Result

```text
step 1: test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.51s
step 2, M-a: only ..._when_only_the_in_memory_counter_has_reached_the_expected_value FAILED (test_env.rs:425:9)
step 2, M-b: only ..._when_only_the_database_has_reached_the_expected_value FAILED (test_env.rs:406:9)
step 3: test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.53s
step 4: filtered pass=300 fail=0
step 4: binary pass=100 fail=0
```

#### Conclusion

The V4 and V6 results hold for the final helper.
