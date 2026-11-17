---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2847-1488-si-28-verify-hung-component-abort/ISSUE.md
last-updated-utc: 2026-11-17 21:02
---

# Manual Verification Evidence

## Purpose

Record real runs of the `hung-component-tracker` test binary (M1 to M3) and of the shipped
`torrust-tracker` binary before and after the shutdown-sequence refactor (M4), as listed in the
spec's Manual Verification Scenarios.

## Preserving Verification Artifacts

The configurations and the run script are embedded below. The full logs, with ANSI color escapes
removed and the local checkout path replaced by `<repo>`, are in [`evidence/`](evidence/):

- [`evidence/m1.log`](evidence/m1.log) - M1, `SIGTERM`
- [`evidence/m2.log`](evidence/m2.log) - M2, `SIGINT`
- [`evidence/m4-before.log`](evidence/m4-before.log) and [`evidence/m4-after.log`](evidence/m4-after.log) - M4

The runs wrote to `.tmp/m/` in the checkout; nothing there is the only copy of the evidence.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-17 16:44 to 16:46.
- Artifact under test: branch `2847-verify-hung-component-abort` with the patches
  "refactor(app): [#2847] move the shutdown sequence from main.rs into the library" and
  "test(hung-component-tracker): [#2847] verify the hung-component abort on a real tracker binary"
  on top of `torrust/develop` at `a544a58fb` (the merge of PR #2909). The "before" binary of M4 is
  `torrust/develop` at `a544a58fb`, built from a temporary detached worktree.
- Operating system / environment: Linux 7.0.0-43-generic (x86_64), debug builds
  (`cargo build`, dev profile). Load average about 4.4 during the runs.
- Prerequisites and setup performed: built both binaries of the branch and the pre-refactor
  `torrust-tracker`, then copied all three to `.tmp/m/` (see "Stale build in the shared target
  directory" below for how each copy was confirmed).

### Run script

Each run used this script. It starts the binary, waits for the readiness marker, signals the exact
PID, and reports the exit code and the time from the signal to the exit:

```bash
#!/usr/bin/env bash
# Usage: run-and-signal.sh <binary> <config.toml> <TERM|INT> <log-file>
# Starts the binary, waits for the readiness marker, signals its exact PID, and reports the exit.
set -u
binary=$1 config=$2 signal=$3 log=$4
env -u TORRUST_TRACKER_CONFIG_TOML -u TORRUST_TRACKER_CONFIG_TOML_PATH "$binary" --config-toml-path "$config" > "$log" 2>&1 &
pid=$!
echo "started pid=$pid at $(date -u +%T.%3N)"
for _ in $(seq 300); do grep -q "Tracker shutdown signal handlers installed." "$log" && break; sleep 0.1; done
echo "readiness marker seen at $(date -u +%T.%3N)"
t0=$(date +%s.%N); echo "sending SIG$signal to pid=$pid at $(date -u +%T.%3N)"
kill "-$signal" "$pid"
wait "$pid"; code=$?
t1=$(date +%s.%N)
echo "exited with code=$code at $(date -u +%T.%3N), $(echo "$t1 - $t0" | bc) s after the signal"
```

### Configuration of M1 and M2 (`hung.toml`)

The same budgets as the integration test: `P = 4 s`, drains of 1 s (`max(1, 1) + 1 = 2 < 4`).

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"

[core]
listed = false
private = false

[core.database]
driver = "sqlite3"
path = "<repo>/.tmp/m/storage-hung/sqlite3.db"

[health_check_api]
bind_address = "127.0.0.1:0"

[application.shutdown]
deadline_in_secs = 4
http_drain_in_secs = 1
udp_request_drain_in_secs = 1
event_listener_drain_in_secs = 1
```

### Configuration of M4 (`real-before.toml`, `real-after.toml`)

The default shutdown budgets (`P = 25 s`), one HTTP tracker and the health check API on ephemeral
ports. The two files differ only in the storage directory (`storage-before`, `storage-after`).

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"

[core]
listed = false
private = false

[core.database]
driver = "sqlite3"
path = "<repo>/.tmp/m/storage-after/sqlite3.db"

[[http_trackers]]
bind_address = "127.0.0.1:0"
tracker_usage_statistics = false

[health_check_api]
bind_address = "127.0.0.1:0"
```

## Verification Processes

Every run below was started only after the previous one had exited, one command per scenario.

### V1 - Hung component, SIGTERM (M1)

- Goal: the supervisor aborts the hung component at `P` and the process exits with code 1.
- Initial state: `hung-component-tracker` running with `hung.toml`, readiness marker logged.
- Status: `DONE`

#### Steps Performed

1. `.tmp/m/run-and-signal.sh .tmp/m/hung-component-tracker .tmp/m/hung.toml TERM .tmp/m/m1.log`

#### Observed Result

```text
started pid=3409447 at 16:44:57.322
readiness marker seen at 16:44:57.430
sending SIGTERM to pid=3409447 at 16:44:57.434
exited with code=1 at 16:45:01.448, 4.013367193 s after the signal
```

```text
2026-11-17T16:44:57.359555Z  INFO torrust_tracker: Tracker shutdown signal handlers installed.
2026-11-17T16:44:57.434731Z  INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
2026-11-17T16:45:01.435991Z  WARN torrust_tracker_lib::bootstrap::jobs::manager: Job was never asked to stop because the jobs it waits for had not completed job=health_check_api waiting_for=hung_component
2026-11-17T16:45:01.436014Z  WARN torrust_tracker_lib::bootstrap::jobs::manager: Job shutdown deadline expired; aborting remaining jobs timeout_seconds=4
2026-11-17T16:45:01.436061Z  WARN torrust_tracker_lib::bootstrap::jobs::manager: Job was aborted after the shutdown deadline job=health_check_api
2026-11-17T16:45:01.436072Z  WARN torrust_tracker_lib::bootstrap::jobs::manager: Job was aborted after the shutdown deadline job=hung_component
2026-11-17T16:45:01.436082Z ERROR torrust_tracker: Torrust tracker shutdown was incomplete: a component failed, panicked, or was aborted at the shutdown deadline. incomplete_components=health_check_api, hung_component
```

#### Conclusion

Met. Exit code 1, 4.013 s after the signal (`P` = 4 s); every other component completed after
cooperative cancellation within milliseconds of the signal (full log), and `hung_component` was
aborted at the deadline and named in `incomplete_components`.

### V2 - Hung component, SIGINT (M2)

- Goal: as V1, with `SIGINT`.
- Initial state: a fresh `hung-component-tracker` with `hung.toml`, readiness marker logged.
- Status: `DONE`

#### Steps Performed

1. `.tmp/m/run-and-signal.sh .tmp/m/hung-component-tracker .tmp/m/hung.toml INT .tmp/m/m2.log`

#### Observed Result

```text
started pid=3410219 at 16:45:01.454
readiness marker seen at 16:45:01.563
sending SIGINT to pid=3410219 at 16:45:01.567
exited with code=1 at 16:45:05.582, 4.013819743 s after the signal
```

```text
2026-11-17T16:45:01.480431Z  INFO torrust_tracker: Tracker shutdown signal handlers installed.
2026-11-17T16:45:01.568195Z  INFO torrust_tracker: Torrust tracker shutting down (SIGINT) ...
2026-11-17T16:45:05.569379Z  WARN torrust_tracker_lib::bootstrap::jobs::manager: Job was never asked to stop because the jobs it waits for had not completed job=health_check_api waiting_for=hung_component
2026-11-17T16:45:05.569409Z  WARN torrust_tracker_lib::bootstrap::jobs::manager: Job shutdown deadline expired; aborting remaining jobs timeout_seconds=4
2026-11-17T16:45:05.569453Z  WARN torrust_tracker_lib::bootstrap::jobs::manager: Job was aborted after the shutdown deadline job=health_check_api
2026-11-17T16:45:05.569465Z  WARN torrust_tracker_lib::bootstrap::jobs::manager: Job was aborted after the shutdown deadline job=hung_component
2026-11-17T16:45:05.569475Z ERROR torrust_tracker: Torrust tracker shutdown was incomplete: a component failed, panicked, or was aborted at the shutdown deadline. incomplete_components=health_check_api, hung_component
```

#### Conclusion

Met. Same result as V1 for `SIGINT`: exit code 1, 4.014 s after the signal.

### V3 - The health check API does not delay the exit (M3)

- Goal: `health_check_api`, which stops last, is aborted at `P` with the hung component and does not
  push the exit past `P` plus the margin.
- Initial state: the logs of V1 and V2.
- Status: `DONE`

#### Steps Performed

1. Read the V1 and V2 logs above.

#### Observed Result

In both runs the supervisor logs at `P` that `health_check_api` "was never asked to stop" because it
waits for `hung_component`, then aborts both in the same millisecond, and the error line follows
about 10 µs later. The process exit is observed 4.013 s and 4.014 s after the signal.

#### Conclusion

Met. The health check API is aborted at the deadline with the hung component; it does not wait for
its own 1 s drain, and the exit stays about 13 ms past `P`, well inside the 3 s margin of the test.

### V4 - Real binary unchanged (M4)

- Goal: the shipped `torrust-tracker` shuts down gracefully with exit code 0, with the same log lines
  and targets before and after the refactor.
- Initial state: the pre-refactor binary (`a544a58fb`) and the branch binary, each with its own
  configuration and empty storage.
- Status: `DONE`

#### Steps Performed

1. `.tmp/m/run-and-signal.sh .tmp/m/torrust-tracker-before .tmp/m/real-before.toml TERM .tmp/m/m4-before.log`
2. After step 1 exited: `.tmp/m/run-and-signal.sh .tmp/m/torrust-tracker-after .tmp/m/real-after.toml TERM .tmp/m/m4-after.log`
3. After step 2 exited: normalize both logs (strip ANSI escapes and the leading timestamp, replace
   `127.0.0.1:<port>` by `127.0.0.1:PORT` and the storage directory name, sort the lines) and
   compare them with `diff`.

```bash
norm() { sed 's/\x1b\[[0-9;]*m//g' "$1" | sed -E 's/^[0-9T:.-]+Z +//; s/127\.0\.0\.1:[0-9]+/127.0.0.1:PORT/g; s/storage-(before|after)/storage-X/g' | sort; }
norm .tmp/m/m4-before.log > .tmp/m/m4-before.norm; norm .tmp/m/m4-after.log > .tmp/m/m4-after.norm
diff .tmp/m/m4-before.norm .tmp/m/m4-after.norm && echo "normalized logs identical"
```

The lines are sorted because the components stop concurrently, so their order may differ between
two runs of the same binary.

#### Observed Result

```text
== M4 before (rerun)
started pid=3412222 at 16:45:33.588
readiness marker seen at 16:45:33.697
sending SIGTERM to pid=3412222 at 16:45:33.702
exited with code=0 at 16:45:33.713, .011660537 s after the signal
== M4 after (rerun)
started pid=3412275 at 16:45:33.720
readiness marker seen at 16:45:33.828
sending SIGTERM to pid=3412275 at 16:45:33.833
exited with code=0 at 16:45:33.845, .012249852 s after the signal
normalized logs identical
```

The lines the refactor moved, with their target, before and after:

```text
== before
2026-11-17T16:45:33.636381Z  INFO torrust_tracker: Tracker shutdown signal handlers installed.
2026-11-17T16:45:33.702406Z  INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
2026-11-17T16:45:33.702671Z  INFO torrust_tracker: Torrust tracker successfully shutdown.
== after
2026-11-17T16:45:33.768333Z  INFO torrust_tracker: Tracker shutdown signal handlers installed.
2026-11-17T16:45:33.833716Z  INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
2026-11-17T16:45:33.833964Z  INFO torrust_tracker: Torrust tracker successfully shutdown.
```

#### Conclusion

Met. Both binaries exit with code 0, and the 145 normalized log lines are identical, targets included.
The moved events still log under `torrust_tracker`. The incomplete-shutdown error line keeps the same
target too (V1 and V2).

## Failures and Follow-up

- **Stale build in the shared target directory.** The pre-refactor binary was built from a
  temporary detached worktree at `a544a58fb` with `CARGO_TARGET_DIR` pointing at the checkout's own
  `target/`. Afterwards `target/debug/torrust-tracker` was the pre-refactor binary (`cmp` against
  the `.tmp/m/torrust-tracker-before` copy reported them equal), and the next `cargo build` of the
  branch failed in `src/main.rs` with "cannot find function `run_until_shutdown_signal` in module
  `app`": the library build in `target/` was still the pre-refactor one. Touching `src/lib.rs`,
  `src/app.rs` and `src/main.rs` forced a rebuild, after which `cmp` reported the branch binary
  different from the pre-refactor copy. The copies used in V1 to V4 were not affected:
  `.tmp/m/torrust-tracker-after` and `.tmp/m/hung-component-tracker` were copied at 16:44:10, after
  a `cargo build` at the branch head and before the worktree build, and
  `.tmp/m/torrust-tracker-before` was copied at 16:44:21, right after the worktree build. Since the
  M4 logs are meant to be identical, they cannot show which binary ran, so the copies were checked
  afterwards with `nm -C <binary> | grep -c 'torrust_tracker_lib::app::run_until_shutdown_signal'`:
  4 matches in `torrust-tracker-after` and in `hung-component-tracker`, 0 in
  `torrust-tracker-before`. The sizes differ too (190,411,400 bytes after, 190,406,376 bytes before).
  V1 and V2 also show by behavior that the hung-component binary was not the mutated one of the
  test-design review's mutation check, which exits 0.
- **First M4 run discarded (16:45:13 to 16:45:14).** It also ended with exit code 0 for both
  binaries and the moved lines identical, but the stamps of the run script disagreed with the
  tracker's own log: the "after" tracker logged its shutdown at 16:45:14.044, while the script
  printed "sending SIGTERM" at 16:45:14.438. The script takes that stamp before it runs `kill`, and
  the tracker can log its shutdown only after the signal arrives, so a late `date` cannot produce
  this order. Its "readiness marker seen" stamp (16:45:14.393) was also about 0.4 s after the
  tracker's marker line (16:45:13.977), more than the 0.1 s poll interval allows. The cause is
  unknown: it was not investigated before the rerun, and nothing recorded from that run identifies
  it. Its sorted-log diff had one line: `health_check_api ... last_completed_job=http_core_event_listener`
  before and `last_completed_job=tracker_core_in_memory_event_listener` after, which names whichever
  concurrent job finished last and is not a change of the refactor. M4 was rerun at once; the rerun
  above has consistent stamps and identical normalized logs and stands on its own committed logs.
- **Case 4 limit.** These runs and the integration test cover a component that hangs at an await
  point (cases 1 to 3 of the spec). A component that blocks its worker thread, or one waiting on
  `spawn_blocking` work, never reaches an await point: `JoinSet::abort_all` cannot stop it, and the
  runtime drop waits for a blocking thread, so the process would not exit at `P`. Neither this test
  nor these runs show that case; the external stop timeout (`deadline_in_secs` + 5 s, then
  `SIGKILL`) is what bounds it.
