---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2953-fix-persisted-downloads-integration-test-race/ISSUE.md
last-updated-utc: "2026-11-19 07:31"
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

- Status: `TODO`

### V4 - Green and like-for-like recheck

- Status: `TODO`

### V5 - Test-design review

- Status: `TODO`
