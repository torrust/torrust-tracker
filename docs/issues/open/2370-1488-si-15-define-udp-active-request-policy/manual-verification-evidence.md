---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2370-1488-si-15-define-udp-active-request-policy/ISSUE.md
last-updated-utc: "2026-09-29 13:15"
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
- Status: `Trigger only`

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
live task lost its only handle. T3/T5's receive-loop collaboration test is the
smallest maintained observation boundary; it will prove the defect red and the
fix green.

### V2 - Red and Green Regression Evidence

- Goal: Prove the receive-loop collaboration test fails with ring-only ownership and passes after the T5 owner and drain wiring.
- Initial state: To be recorded when T3 adds the maintained test.
- Status: `TODO`

#### Steps Performed

1. Record the focused red test command and output before the production fix.
2. Record the focused green test command and output after the fix.
3. Mutate the fix without staging it, confirm the test fails, and restore it.

#### Observed Result

```text
Not yet executed.
```

#### Conclusion

Not yet assessed.

## Shutdown Verification

Record M1-M4 after implementation, including direct binary PID, command output,
relevant logs, bounded exit status, and listener rebind evidence.

## Failures and Follow-up

Record failed or blocked scenarios, diagnosis, remediation, and rerun status.
