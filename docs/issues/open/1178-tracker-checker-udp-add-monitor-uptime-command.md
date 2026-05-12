# Issue #1178 — Tracker Checker (UDP): Add Command to Monitor Uptime

## Overview

Add a new `monitor` subcommand (or standalone binary) to the Tracker Checker that periodically
sends UDP `announce` requests to a tracker and prints live statistics. The goal is to reproduce
locally what <https://newtrackon.com/> does, so maintainers can investigate intermittent uptime
drops without relying on a third-party service.

- GitHub issue: <https://github.com/torrust/torrust-tracker/issues/1178>
- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>
- Related: <https://github.com/torrust/torrust-demo/issues/26>

## Background

[newtrackon.com](https://newtrackon.com/) reported 93% uptime for the Torrust demo UDP tracker.
The host `netstat -su` output shows no packet loss at the network level, and the measured
announce processing time inside the tracker is well under 10 ms. Yet newtrackon reports ~222 ms
response time and occasional timeouts.

To reproduce and diagnose the problem, a local monitoring loop is needed that does the same as
newtrackon: sends an announce request at a fixed interval and accumulates response-time
statistics.

The relevant newtrackon checking interval is every 5 minutes; the tool should default to the
same interval, but the interval should be configurable.

## Goals

- [ ] Add a UDP uptime-monitor command to the tracker-client toolbox
- [ ] The command accepts a UDP tracker URL and optional configuration (interval, timeout, info-hash)
- [ ] On every probe the command prints one JSON object per line to stderr (NDJSON)
- [ ] At the end of execution, the command prints final statistics to stdout in JSON format
- [ ] Final statistics include:
  - Total probe count
  - Timeout count (and percentage)
  - Minimum response time
  - Maximum response time
  - Average response time
  - Last response time
- [ ] The command accepts a duration argument and exits automatically after that duration
- [ ] `Ctrl+C` is supported to stop monitoring early and still print final JSON results
- [ ] `linter all` exits with code `0`
- [ ] `cargo machete` reports no unused dependencies
- [ ] Existing tests pass

## Proposed CLI

```text
cargo run --bin tracker_checker -- monitor udp \
    --url     udp://127.0.0.1:6969 \
    --interval 300 \
  --timeout  10 \
  --duration 86400
```

Or as part of a possible future unified `tracker-client` CLI:

```text
cargo run --bin torrust-tracker-client -- \
    checker monitor udp \
    --url     udp://127.0.0.1:6969 \
    --interval 300 \
    --timeout  10
```

### Options

| Option       | Default | Description                                  |
| ------------ | ------- | -------------------------------------------- |
| `--url`      | —       | UDP tracker URL (required)                   |
| `--interval` | `300`   | Seconds between probes                       |
| `--timeout`  | `10`    | Seconds to wait for a response before timeout |
| `--duration` | `86400` | Total monitor runtime in seconds             |

### Sample Output

```text
stderr:
{"event":"probe","sequence":1,"url":"udp://127.0.0.1:6969","status":"ok","elapsed_ms":122}
{"event":"probe","sequence":2,"url":"udp://127.0.0.1:6969","status":"ok","elapsed_ms":98}
{"event":"probe","sequence":3,"url":"udp://127.0.0.1:6969","status":"timeout","elapsed_ms":null}

stdout:
{"udp_trackers":[{"url":"udp://127.0.0.1:6969","status":{"code":"ok","message":"monitor completed","stats":{"total":3,"timeouts":1,"timeout_percent":33.3,"min_ms":98,"max_ms":122,"average_ms":110,"last_ms":null}}}]}
```

## Implementation Plan

### Task 1: Add `monitor udp` subcommand to `tracker_checker`

In `console/tracker-client/src/console/clients/checker/app.rs`, add a new CLI subcommand
`monitor` (or extend the existing args structure) that accepts:

- `--url` (required): UDP tracker URL
- `--interval` (optional, default 300): probe interval in seconds
- `--timeout` (optional, default 10): per-probe timeout in seconds
- `--duration` (optional, default 86400): total monitor runtime in seconds

### Task 2: Implement probe loop

Create a new module, e.g.
`console/tracker-client/src/console/clients/checker/monitor/udp.rs`, containing:

- A `run_monitor` async function that loops forever (until Ctrl+C signal)
- Each iteration sends a UDP `announce` request using the existing `UdpTrackerClient`
- Records `start` / `end` timestamps and computes elapsed milliseconds
- Treats no response within `--timeout` as a timeout event

### Task 3: Track statistics

Maintain an in-memory stats struct across iterations:

```rust
struct Stats {
    total: u64,
    timeouts: u64,
    min_ms: Option<u64>,
    max_ms: Option<u64>,
    sum_ms: u64,
    last_ms: Option<u64>,
}
```

Implement `average_ms` as `sum_ms / (total - timeouts)` (guard against divide-by-zero).

### Task 4: Print status and stats after each probe

After each probe, print to stderr:

1. A one-line JSON probe event (NDJSON) including sequence number, status, and elapsed time
2. Optionally, a compact running summary (still on stderr)

At the end of monitoring (timeout reached or Ctrl+C), print final aggregate stats to stdout as JSON.
The JSON shape should align with the existing checker output structure.

### Task 5: Add duration-based stop condition and Ctrl+C support

Stop automatically when `--duration` elapses.

Register a `tokio::signal::ctrl_c` handler (or `signal_hook`) that breaks the loop cleanly and
still prints final JSON stats before exiting.

When monitoring completes (including timeout-heavy runs), return exit code `0` if the tool itself
ran successfully.

### Task 6: Wire the new subcommand into the binary entry point

Update `console/tracker-client/src/bin/tracker_checker.rs` to dispatch to the new monitor loop
when the `monitor` subcommand is selected.

## Key Files

| File                                                                                  | Role                              |
| ------------------------------------------------------------------------------------- | --------------------------------- |
| `console/tracker-client/src/console/clients/checker/app.rs`                          | CLI argument parsing, entry point |
| `console/tracker-client/src/console/clients/checker/`                                | Checker module root               |
| `packages/tracker-client/src/udp/`                                                   | Existing UDP tracker client       |
| `console/tracker-client/src/bin/tracker_checker.rs`                                  | Binary entry point                |

## Acceptance Criteria

- [ ] AC1: `monitor udp --url udp://127.0.0.1:6969` starts a probe loop and prints a status
           JSON line after each probe to stderr (NDJSON)
- [ ] AC2: When monitoring ends, final aggregate statistics are printed to stdout as valid JSON
- [ ] AC3: When a probe does not receive a response within the timeout, it is recorded as
           `TIMEOUT` and excluded from response-time averages
- [ ] AC4: `--duration` controls total runtime and the command exits normally when elapsed
- [ ] AC5: `Ctrl+C` stops monitoring early and still emits final JSON stats
- [ ] AC6: The `--interval` option controls the delay between probes
- [ ] AC7: `--duration` defaults to `86400` seconds when omitted
- [ ] AC8: If all probes timeout but execution is otherwise successful, exit code is `0`
- [ ] AC9: `linter all` exits with code `0`
- [ ] AC10: `cargo machete` reports no unused dependencies
- [ ] AC11: Existing tests pass

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |
| AC7   | TODO                   |          |
| AC8   | TODO                   |          |
| AC9   | TODO                   |          |
| AC10  | TODO                   |          |
| AC11  | TODO                   |          |

## Risks and Trade-offs

- **Scope**: A continuously running loop binary is heavier than a one-shot check. The feature is
  explicitly for developer/admin use, so this is acceptable.
- **Signal handling**: Cross-platform `Ctrl+C` handling in async Tokio requires `tokio::signal`.
  Windows support is nice-to-have but not a hard requirement for the initial implementation.
- **UDP announcement contents**: The monitor sends a real announce request. The info-hash and
  peer fields will be test values (re-using the existing `QueryBuilder::with_default_values`
  defaults unless overridden). This is acceptable for monitoring purposes.

## Metadata

| Field              | Value                                                                  |
| ------------------ | ---------------------------------------------------------------------- |
| Type               | Feature                                                                |
| Status             | Planned                                                                |
| Priority           | P2                                                                     |
| GitHub Issue       | [#1178](https://github.com/torrust/torrust-tracker/issues/1178)        |
| Spec Path          | `docs/issues/open/1178-tracker-checker-udp-add-monitor-uptime-command.md` |
| Branch             | `1178-tracker-checker-udp-add-monitor-uptime-command`                 |
| Related PR         | To be assigned                                                         |
| Last Updated (UTC) | 2026-05-12 08:00                                                       |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/open/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] Implementation completed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-05-11 20:00 UTC - Agent - Spec created from GitHub issue #1178 content
- 2026-05-12 00:00 UTC - Agent - Incorporated maintainer decisions: monitor in tracker_checker, seconds unit, UDP-only scope, duration-controlled run, stderr live output plus final JSON on stdout
- 2026-05-12 08:00 UTC - Agent - Incorporated answered follow-ups: default duration `86400`, align final JSON with checker shape, keep exit code `0` for timeout-heavy but successful runs

## Open Questions

No open questions at this time.

## References

- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>
- newtrackon uptime discussion: <https://github.com/torrust/torrust-demo/issues/26>
- Existing UDP checker: `console/tracker-client/src/console/clients/udp/checker.rs`
- UDP tracker client: `packages/tracker-client/src/udp/`
- Tracker CLI I/O contract: `console/tracker-client/docs/contracts/tracker-cli-io-contract.md`
- Tracker CLI ADR: `console/tracker-client/docs/adrs/20260512080000_define_tracker_cli_io_contract_and_error_handling.md`
