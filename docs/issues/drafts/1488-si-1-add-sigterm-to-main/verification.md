# Verification Evidence — SI-1: Add `SIGTERM` Handler to `main.rs`

This document contains two phases of verification:

- **Phase 1 (Pre-implementation)**: evidence that the current behaviour is broken.
- **Phase 2 (Post-implementation)**: evidence that the fix works correctly.

Both phases must be completed. Phase 1 was run on the `before` baseline. Phase 2
must be run after the implementation is merged.

> Copy-paste actual terminal output. Do not summarize or paraphrase. Raw output
> is the evidence.

---

## Environment

- **Date**: 2026-07-16
- **OS**: Linux josecelano-desktop 7.0.0-27-generic #27-Ubuntu SMP PREEMPT_DYNAMIC
  Thu Jun 18 19:13:49 UTC 2026 x86_64 GNU/Linux
- **Rust version**: rustc 1.99.0-nightly (da80ed070 2026-07-14)
- **Tracker git commit**: 49d8117f
- **Branch**: 1488-overhaul-tracker-shutdown-docs

---

## Phase 1 — Pre-Implementation (Baseline: Broken Behaviour)

> **Purpose**: prove that `main.rs` currently ignores `SIGTERM` even though
> server libraries react independently, leaving the tracker process running
> until it is force-killed with `SIGKILL`.
>
> **Status**: Completed on 2026-07-16.

### P1 — Preparation

Binary built and confirmed:

```bash
cargo build --release
ls -lh target/release/torrust-tracker
```

```text
-rwxrwxr-x 2 josecelano josecelano 127M Jul 16 17:54 target/release/torrust-tracker
```

### P1 — Test 1: `kill <pid>` bypasses `main.rs` (SIGTERM ignored by `main.rs`)

#### Procedure

```bash
RUST_LOG=info ./target/release/torrust-tracker > /tmp/tracker-si1-before-test1.log 2>&1 &
TRACKER_PID=$(pgrep -x torrust-tracker)
echo "Binary PID: $TRACKER_PID"
kill "$TRACKER_PID"
sleep 3
if kill -0 "$TRACKER_PID" 2>/dev/null; then
  echo "RESULT: Process IS STILL RUNNING — SIGTERM was IGNORED"
else
  echo "RESULT: Process has exited"
fi
```

#### Terminal Output

```text
Binary PID: 955797
RESULT: Process IS STILL RUNNING after SIGTERM — SIGTERM bypassed main.rs
```

#### Interpretation

The binary PID is 955797. After `kill 955797` (SIGTERM), the process was still
alive 3 seconds later. `main.rs` does not handle SIGTERM.

#### Key Finding: servers DID react, but `main.rs` did NOT

Each server's `global_shutdown_signal()` caught SIGTERM and began shutting down
its own connections — but `main.rs`'s `tokio::select!` never fired, so
`jobs.cancel()` and `jobs.wait_for_all()` were never called.

After the servers shut themselves down, the swarm coordination registry kept
emitting periodic metrics, proving the main process was still alive:

```text
2026-07-16T16:57:30.727509Z  WARN ...global_shutdown_signal: caught interrupt signal (terminate), halting...
2026-07-16T16:57:30.727530Z  WARN ...global_shutdown_signal: caught interrupt signal (terminate), halting...
2026-07-16T16:57:30.727535Z  WARN ...global_shutdown_signal: caught interrupt signal (terminate), halting...
2026-07-16T16:57:30.727553Z  WARN ...global_shutdown_signal: caught interrupt signal (terminate), halting...
2026-07-16T16:57:30.727596Z  INFO graceful_shutdown{address=0.0.0.0:7070}: !! Shutting down HTTP server ... in 90 seconds !!
2026-07-16T16:57:30.727608Z  INFO graceful_shutdown{address=0.0.0.0:7171}: !! Shutting down HTTP server ... in 90 seconds !!
2026-07-16T16:57:30.727613Z  INFO graceful_shutdown{address=0.0.0.0:1212}: All connections closed, shutting down server
2026-07-16T16:57:30.727621Z  INFO graceful_shutdown{address=0.0.0.0:7070}: All connections closed, shutting down server
2026-07-16T16:57:30.727615Z  INFO graceful_shutdown{address=0.0.0.0:7171}: All connections closed, shutting down server
2026-07-16T16:57:30.727648Z  WARN ...global_shutdown_signal: caught interrupt signal (terminate), halting...
2026-07-16T16:57:30.727664Z  WARN ...global_shutdown_signal: caught interrupt signal (terminate), halting...
2026-07-16T16:57:30.727679Z  WARN ...global_shutdown_signal: caught interrupt signal (terminate), halting...
2026-07-16T16:57:30.727731Z  INFO HEALTH CHECK API: Stopped server running on: http://127.0.0.1:1313
--- servers have stopped, but main.rs is still running: ---
2026-07-16T16:57:44.991005Z  INFO torrust_tracker_swarm_coordination_registry: active_peers_total=0 ...
2026-07-16T16:57:59.991645Z  INFO torrust_tracker_swarm_coordination_registry: active_peers_total=0 ...
```

#### What is absent from the log (critical evidence)

The log does **not** contain any of these lines that would appear if `main.rs`
had reacted:

- `Torrust tracker shutting down ...`
- `Waiting for job to finish`
- `Job completed gracefully`
- `Torrust tracker successfully shutdown.`

#### P1 Test 1 Result: CONFIRMED BUG

- [x] CONFIRMED: Process still running 3 seconds after SIGTERM
- [x] CONFIRMED: Servers reacted via `global_shutdown_signal()` but `main.rs` did not
- [x] CONFIRMED: `jobs.cancel()` and `jobs.wait_for_all()` were never called
- [x] CONFIRMED: No graceful shutdown message from `main.rs`

### P1 — Test 2: Force-killing with SIGKILL is required to stop the process

After SIGTERM bypassed `main.rs`, SIGKILL was required:

```bash
kill -9 955797
```

```text
Exit 137   ./target/release/torrust-tracker > /tmp/tracker-si1-before-test1.log 2>&1
```

Exit code 137 (= 128 + 9) confirms SIGKILL was used. SI-20 implements Q3's
approved process exit-result contract for the complete shutdown architecture.

### P1 — Test 3: Ports freed after SIGKILL

```bash
lsof -i :7070,6969,1212,1313 2>/dev/null | grep LISTEN || echo "All ports are now free"
```

```text
All ports are now free
```

Ports freed correctly when the OS killed the process.

---

## Phase 2 — Post-Implementation (After the Fix)

> **Status**: Not started — to be completed after SI-1 is implemented and merged.

Rebuild the binary from the branch with the SIGTERM fix applied, then run all
tests below. The environment block must be updated with the new commit hash.

### P2 Environment

- **Date**:
- **Tracker git commit** (`git rev-parse --short HEAD`):
- **Branch** (`git branch --show-current`):

### P2 — Test 1: `kill <pid>` triggers graceful shutdown (SIGTERM handled)

Same procedure as P1 Test 1 using the fixed binary.

```text
(paste terminal output — must show "Process has exited")
```

```text
(paste /tmp/tracker-si1-after-test1.log)
```

#### P2 Test 1 Pass/Fail

- [ ] PASS: Process exits without a second signal
- [ ] PASS: Log contains `shutting down (SIGTERM)`
- [ ] PASS: Log shows `jobs.cancel()` and managed-job waiting began
- [ ] PASS: Do not require every legacy component to complete in this
      incremental signal-boundary change

### P2 — Test 1a: Bounded direct-binary signal delivery

```text
(paste the bounded harness command and terminal output)
```

- [ ] PASS: The harness targets the release tracker binary PID, not `cargo run`.
- [ ] PASS: SIGTERM reaches `main()` and begins cancellation before the harness deadline.
- [ ] PASS: The recorded bound accommodates SI-1's current sequential legacy
      shutdown behavior; SI-20 owns the final process deadline.

### P2 — Test 2: `kill -TERM <pid>` — same outcome as P2 Test 1

```text
(paste log output)
```

- [ ] PASS: Outcome identical to P2 Test 1

### P2 — Test 3: Ctrl+C — log says SIGINT not SIGTERM

```text
(paste log output)
```

- [ ] PASS: Log contains `shutting down (SIGINT)`
- [ ] PASS: Log does NOT contain `shutting down (SIGTERM)`

### P2 — Test 4: SIGKILL — still force-terminates immediately (exit 137)

```bash
kill -9 <pid>
echo "Exit code: $?"
```

```text
(paste output)
```

- [ ] PASS: Exit code is 137
- [ ] PASS: No graceful shutdown log lines after the kill

### P2 — Test 5: `docker stop` forwards SIGTERM (exploratory)

```text
(paste `time docker stop` output and container log tail — or mark as SKIPPED)
```

- [ ] PASS: Container log shows `main.rs` received SIGTERM and began shutdown
- [ ] RECORD: Whether the configured Docker deadline was sufficient
- [ ] SKIPPED (reason: \_\_\_)

---

## Final Summary

| Phase | Test | Description                         | Result    |
| ----- | ---- | ----------------------------------- | --------- |
| P1    | T1   | SIGTERM ignored — process survives  | Confirmed |
| P1    | T2   | SIGKILL required — exit code 137    | Confirmed |
| P1    | T3   | Ports freed after SIGKILL           | Confirmed |
| P2    | T1   | SIGTERM reaches `main()`            | Pending   |
| P2    | T1a  | Bounded direct-binary delivery      | Pending   |
| P2    | T2   | `kill -TERM` — same as T1           | Pending   |
| P2    | T3   | Ctrl+C — log says SIGINT            | Pending   |
| P2    | T4   | SIGKILL — exit 137, no shutdown log | Pending   |
| P2    | T5   | `docker stop` forwards SIGTERM      | Pending   |

All P2 tests must PASS (or T5 is skipped with a reason) before this issue can
be closed. Complete lifecycle success is verified by the later component,
deadline, and exit-code work items.
