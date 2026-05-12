# Refactor Plan — Issue #1178 Monitor UDP: Post-Implementation Improvements

## Goal

Address quality gaps identified after the initial implementation of the `monitor udp` subcommand
(issue #1178). Items are ordered from **highest impact / lowest effort** to **lowest impact /
highest effort** so they can be tackled incrementally.

Related issue spec: `docs/issues/open/1178-tracker-checker-udp-add-monitor-uptime-command.md`

## Items

### 1. [ ] Fix stale `timeout_percent` sample value in spec [HIGH impact / TRIVIAL effort]

**Problem**: The "Sample Output" section in the issue spec shows `"timeout_percent":33.3` (a
float). The implementation produces `33` (integer `u64`). Any reader using the spec as a
reference for the output contract will be misled.

**Files**: `docs/issues/open/1178-tracker-checker-udp-add-monitor-uptime-command.md`

**Change**: Replace `33.3` → `33` in the sample output block.

---

### 2. [ ] Add `--info-hash` to the Options table in the spec [HIGH impact / TRIVIAL effort]

**Problem**: The implementation exposes `--info-hash` with a sensible default, but the spec's
CLI Options table omits it. A user reading the spec will not know the option exists.

**Files**: `docs/issues/open/1178-tracker-checker-udp-add-monitor-uptime-command.md`

**Change**: Add a row for `--info-hash` (default `9c38422213e30bff212b30c360d26f9a02136422`,
description "Info-hash used in announce requests").

---

### 3. [ ] Tick completed Goals and Workflow Checkpoints in the spec [HIGH impact / TRIVIAL effort]

**Problem**: Implementation is complete, manually verified, and committed, but both the `Goals`
checklist and the `Workflow Checkpoints` list still show unchecked `[ ]` items. They look like
open work to any reader.

**Files**: `docs/issues/open/1178-tracker-checker-udp-add-monitor-uptime-command.md`

**Change**: Mark all completed goals and checkpoints as `[x]`.

---

### 4. [ ] Add a unit test asserting all-null latency fields when every probe times out [HIGH impact / LOW effort]

**Problem**: The "down tracker" scenario (every probe times out → `min_ms`, `max_ms`,
`average_ms`, `last_ms` all `null`) is the most important correctness property of the stats
struct, but it has no dedicated test. It is only validated by a manual run against a live tracker.

**Files**: `console/tracker-client/src/console/clients/checker/monitor/udp.rs`

**Change**: Add a unit test in the existing `#[cfg(test)]` block that:

1. Creates a `Stats` with only `record_timeout()` calls.
2. Asserts `min_ms`, `max_ms`, `average_ms()`, and `last_ms` are all `None`.
3. Asserts `timeout_percent()` returns `100`.

---

### 5. [ ] Document that the integration test exercises only the timeout path [HIGH impact / LOW effort]

**Problem**: `spawn_udp_sink()` silently discards UDP packets without ever sending a valid
`ConnectResponse`. Every probe in the integration test therefore times out. The test validates
JSON shape and exit code but not a successful probe event. This is non-obvious and could mask
regressions in the success path.

**Files**: `console/tracker-client/tests/tracker_checker.rs`

**Change**: Add a doc comment on the `monitor_udp` test module explaining that the UDP sink
intentionally produces timeouts, and note that a success-path integration test requires a proper
mock tracker responding to the UDP protocol (tracked as a follow-up).

---

### 6. [ ] Correct Task 6 file reference in the Implementation Plan [MEDIUM impact / TRIVIAL effort]

**Problem**: Implementation Plan Task 6 says "Update
`console/tracker-client/src/bin/tracker_checker.rs`", but the actual dispatch was added to
`console/tracker-client/src/console/clients/checker/app.rs`. A future contributor tracing a
regression will look in the wrong file.

**Files**: `docs/issues/open/1178-tracker-checker-udp-add-monitor-uptime-command.md`

**Change**: Correct the file path in Task 6 to reference `app.rs`.

---

### 7. [ ] Document `last_ms: null` on timeout in AC3 [MEDIUM impact / LOW effort]

**Problem**: AC3 states that timed-out probes are "excluded from response-time averages" but
does not mention that `last_ms` also becomes `null` when a probe times out. This is a separate,
non-obvious contract detail buried only in the manual verification notes.

**Files**: `docs/issues/open/1178-tracker-checker-udp-add-monitor-uptime-command.md`

**Change**: Update the AC3 description to explicitly state that `last_ms` is set to `null` when
the most recent probe times out.

---

### 8. [ ] Document the double duration-check intent in `run_monitor` [MEDIUM impact / LOW effort]

**Problem**: `run_monitor` contains two `if started_at.elapsed() >= config.duration { break; }`
guards — one before the probe and one before the sleep. This is intentional (avoids sleeping
after the last probe) but reads like an accidental duplication and will confuse reviewers.

**Files**: `console/tracker-client/src/console/clients/checker/monitor/udp.rs`

**Change**: Add inline comments on each guard explaining its distinct purpose:

- First guard: "exit before starting a new probe if the budget is exhausted"
- Second guard: "exit before sleeping if duration elapsed during the probe itself"

---

### 9. [ ] Document `u64::MAX` fallback for `elapsed_ms` [MEDIUM impact / LOW effort]

**Problem**:

```rust
let elapsed_ms = u64::try_from(probe_started.elapsed().as_millis()).unwrap_or(u64::MAX);
```

`u64::MAX` as a fallback would make a conversion-overflow probe appear as ~584 million years of
latency. Since `as_millis()` returns `u128`, overflow could only occur if a single probe ran for
over 584 million years (impossible in practice), but the fallback is still an incorrect sentinel
in principle — no reader will understand it without a comment.

**Files**: `console/tracker-client/src/console/clients/checker/monitor/udp.rs`

**Change**: Add a comment explaining why overflow is unreachable in practice and that `u64::MAX`
is a placeholder that cannot realistically occur.

---

### 10. [ ] Document that `timeout_percent` denominator includes error probes [MEDIUM impact / LOW effort]

**Problem**: `timeout_percent = timeouts × 100 / total`, where
`total = successes + timeouts + errors`. A probe that errors (not timeout) reduces the percentage
without being a success. The name `timeout_percent` implies "fraction of probes that timed out"
but errors silently dilute the denominator. This behaviour is not documented anywhere in the
spec or code.

**Files**:

- `console/tracker-client/src/console/clients/checker/monitor/udp.rs`
- `docs/issues/open/1178-tracker-checker-udp-add-monitor-uptime-command.md`

**Change**:

- Add a doc comment on `timeout_percent()` explaining the denominator includes errors.
- Add a note in the spec's Risks and Trade-offs section.

---

### 11. [ ] Document that `elapsed_ms` includes DNS resolution time [MEDIUM impact / MEDIUM effort]

**Problem**: The `probe_started` timer is captured before `resolve_socket_addr()`. For trackers
with non-trivial DNS lookup times, the reported latency includes DNS resolution, not just
network round-trip time. This deviates from what most users expect "announce response time" to
mean.

**Files**:

- `console/tracker-client/src/console/clients/checker/monitor/udp.rs`
- `docs/issues/open/1178-tracker-checker-udp-add-monitor-uptime-command.md`

**Options** (choose one):

- **Document only**: Add a comment in code and a note in the spec explaining what is measured.
- **Fix timing**: Move `probe_started` to after `resolve_socket_addr()` — DNS time is then
  excluded from latency. Note that this changes the reported metric.

---

### 12. [ ] Extract `run_probe_loop` from `run_monitor` [LOW impact / MEDIUM effort]

**Problem**: `run_monitor` is ~90 lines handling multiple concerns: the probe loop, signal
handling, sleep, outcome dispatch, stats recording, event emission, and final JSON output. This
makes each piece harder to read and impossible to test independently.

**Files**: `console/tracker-client/src/console/clients/checker/monitor/udp.rs`

**Change**: Extract a private `async fn run_probe_loop(config: &MonitorUdpConfig) -> (Stats, bool /* interrupted */)` that:

1. Runs the loop.
2. Returns final stats and the interrupted flag.

`run_monitor` then calls `run_probe_loop`, formats, and prints. This makes the loop logic unit-
testable without spawning a subprocess.

---

### 13. [ ] Implement `From<&Stats> for MonitorStats` [LOW impact / LOW effort]

**Problem**: The conversion from `Stats` to `MonitorStats` is an inline struct literal embedded
inside the already-long `run_monitor` function. A `From` implementation would express the
intent clearly and clean up `run_monitor`.

**Files**: `console/tracker-client/src/console/clients/checker/monitor/udp.rs`

**Change**: Add `impl From<&Stats> for MonitorStats` and replace the inline literal with
`MonitorStats::from(&stats)`.

---

### 14. [ ] Add a success-path integration test using a mock UDP tracker [LOW impact / HIGH effort]

**Problem**: The only integration test uses a UDP sink that never responds, so the success path
(probe receives a valid `AnnounceResponse`, `elapsed_ms` is Some, latency stats are populated)
is never exercised at the integration level.

**Files**: `console/tracker-client/tests/tracker_checker.rs`

**Change**: Implement a minimal mock UDP tracker in the test helper that:

1. Binds a UDP socket.
2. Responds to a `ConnectRequest` with a valid `ConnectResponse`.
3. Responds to an `AnnounceRequest` with a valid `AnnounceResponse`.

Then add a test asserting that `elapsed_ms` is non-null, `status` is `"ok"`, and `stats.total`,
`stats.successes`, `min_ms`, `max_ms`, `average_ms`, and `last_ms` are all populated.

This is the highest-confidence validation of the happy path and closes the gap left by item 5.

---

## Order of Execution

| Order | Status | Item                                                   | Impact | Effort  |
| ----- | ------ | ------------------------------------------------------ | ------ | ------- |
| 1     | [ ]    | Fix stale `timeout_percent` sample value               | High   | Trivial |
| 2     | [ ]    | Add `--info-hash` to Options table                     | High   | Trivial |
| 3     | [ ]    | Tick completed Goals and Checkpoints                   | High   | Trivial |
| 4     | [ ]    | Unit test: all-null latency on all-timeouts            | High   | Low     |
| 5     | [ ]    | Document integration test exercises timeout path only  | High   | Low     |
| 6     | [ ]    | Correct Task 6 file reference                          | Medium | Trivial |
| 7     | [ ]    | Document `last_ms: null` on timeout in AC3             | Medium | Low     |
| 8     | [ ]    | Document double duration-check intent                  | Medium | Low     |
| 9     | [ ]    | Document `u64::MAX` fallback                           | Medium | Low     |
| 10    | [ ]    | Document `timeout_percent` denominator includes errors | Medium | Low     |
| 11    | [ ]    | Document / fix `elapsed_ms` includes DNS time          | Medium | Medium  |
| 12    | [ ]    | Extract `run_probe_loop` from `run_monitor`            | Low    | Medium  |
| 13    | [ ]    | `From<&Stats> for MonitorStats`                        | Low    | Low     |
| 14    | [ ]    | Success-path integration test with mock UDP tracker    | Low    | High    |
