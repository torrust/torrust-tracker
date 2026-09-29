---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2370-1488-si-15-define-udp-active-request-policy/ISSUE.md
last-updated-utc: "2026-09-29 21:12"
---

# Manual Verification Evidence

## Purpose

Record the orphaned-processor bug reproduction and final like-for-like recheck,
then the executable-boundary scenarios from the issue specification.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-29 18:03.
- Artifact under test: `develop` at `02c026c0`, before SI-15 code changes.
- Operating system / environment: Linux 7.0.0-34-generic x86_64 GNU/Linux;
  `rustc 1.101.0-nightly (c1070d693 2026-09-28)`.
- Prerequisites and setup performed: Built the release tracker with `cargo build
  --release`. Started it with the nonpersistent B0 configuration, changing only
  `trace_filter` from `error` to `warn` for this trigger attempt. Used Aquatic
  `a2ddc4b3` and [b0-load-test.toml](b0-load-test.toml).

## Bug Evidence

### V1 - Orphaned-Processor Reproduction

- Goal: Observe the current full-buffer path losing a live processor handle.
- Initial state: Current ring-only processor ownership before the T5 fix.
- Status: `Reproduced` (real tracker: `Trigger only`; nearest seam: observed in V2)

#### Steps Performed

1. Started `./target/release/torrust-tracker --config-toml-path .tmp/si15-trigger-tracker.toml`, where the temporary configuration is the committed B0 tracker configuration with warning-level logging.
2. Ran `/home/josecelano/Documents/git/greatest-ape/aquatic/target/release/aquatic_udp_load_test -c b0-load-test.toml` for 30 seconds.
3. Sent `SIGTERM` to the direct tracker PID and counted its captured warning log.

#### Observed Result

```text
Average responses per second: 149546.55
aborting request: (no finished tasks) count: 94
```

#### Conclusion

Trigger only. The real tracker reached `ActiveRequests::force_push` overload
eviction, but its public logs and UDP responses do not expose whether a later
live task lost its only handle.

The wrong outcome was then observed at the nearest seam in V2: a real receive
loop on a loopback socket, with real processors held open by an injected event
sender, returned after cancellation while 48 processors it spawned were still
running. Per the fix-bug workflow, a capture at the nearest observation seam
counts as a reproduction; that test is maintained rather than temporary.

#### T5 Like-for-Like Recheck

Repeated the same trigger scenario at 2026-09-29 21:10 UTC with the T5 fix,
changing only `trace_filter` to `info` (temporary copy
`.tmp/si15-t5-tracker.toml`) so the drain logs are visible. The throughput
figure is not a benchmark: info logging with hundreds of eviction warnings
slows the tracker.

```text
Average responses per second: 126064.07
aborting request: (no finished tasks) count: 408
UDP TRACKER: Draining UDP request processors local_addr="udp://0.0.0.0:3000" active=6 deadline=5s
UDP TRACKER: UDP request processors drained local_addr="udp://0.0.0.0:3000" completed=6 failed=0 aborted=0 evicted=0 elapsed=2.745µs
Torrust tracker successfully shutdown.
```

The overload eviction path was exercised 408 times. On `SIGTERM`, the receive
loop still owned every live processor: six were active, and all six were joined
as `completed`. This run did not capture the process exit status; M1-M3 record
it under the bounded direct-PID protocol.

### V2 - Red and Green Regression Evidence

- Goal: Prove the receive-loop collaboration test fails with ring-only ownership and passes after the T5 owner and drain wiring.
- Initial state: A real receive loop on a loopback socket. One released connect request completes first, so it is the oldest ring handle; 49 held connect requests fill the 50-handle ring; one more held request makes `force_push` traverse the full ring. Held processors block at their `UdpRequestAccepted` publication through an injected `Sender` and are counted until their futures are dropped.
- Status: `PASS` (red before T5; green after T5; mutation proof recorded)

#### Steps Performed

1. Red, before the production fix, on `rustc 1.101.0-nightly (c1070d693 2026-09-28)`: `cargo test -p torrust-tracker-udp-server --lib receive_loop_shutdown`.
2. Repeated the red run ten times to confirm it is deterministic.
3. Green, after T5 wired the `JoinSet` owner and drain into the receive loop: removed `#[ignore]`, drove the scenario with a 100 ms drain deadline (so the drain must abort the held processors), and added assertions that cancellation returns `Ok(())` and that the socket is bindable again. Ran the same command ten times.
4. Mutation proof: replaced `processors.spawn(...)` with the old detached `tokio::task::spawn(...).abort_handle()` in the working tree (never staged), reran the command, then restored the line by hand. The test file was uncommitted, so `git checkout --` was not used.

#### Observed Result

```text
Red (before T5):
test server::launcher::tests::receive_loop_shutdown::it_should_not_return_while_processors_orphaned_by_the_request_ring_are_still_running ... FAILED
assertion `left == right` failed: the receive loop returned Ok(Ok(())) while 48 request processors it spawned were still running
  left: 48
 right: 0

Ten repeated runs: 10 of 10 failed with 48 running processors.

Green (after T5), ten repeated runs:
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 188 filtered out; finished in 0.13s

Mutation (detached spawn restored):
assertion `left == right` failed: the receive loop returned Ok(Ok(())) while 48
request processors it spawned were still running
test result: FAILED. 0 passed; 2 failed
```

#### Conclusion

Red confirmed and deterministic. The count matches the analysis: the released
handle is reclaimed, the last live handle traversed is re-inserted, and the
other 48 live handles are dropped. After T5 the same scenario is green ten out
of ten times, and reverting only the ownership wiring brings back the exact red
failure, so the test guards the bug. The companion scenario
`it_should_let_an_accepted_request_finish_and_answer_before_returning_on_cancellation`
releases one held processor after cancellation with a long drain deadline and
checks that it answers before the loop returns. It also failed under the
mutation, but by timing out while waiting for the response, so it is not counted
as the deterministic guard.

## Shutdown Verification

Record M1-M4 after implementation, including direct binary PID, command output,
relevant logs, bounded exit status, and listener rebind evidence.

## Failures and Follow-up

Record failed or blocked scenarios, diagnosis, remediation, and rerun status.
