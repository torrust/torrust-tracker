---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2697-1488-si-21-mark-health-unhealthy-during-shutdown/ISSUE.md
last-updated-utc: "2026-11-04 12:20"
---

# SI-21 Manual Verification Evidence

## Purpose

Record what the health-check API returns to NEW connections during shutdown,
before any readiness change exists (baseline). This answers the design question
in the spec: is a 503 observable at all in the current ordering?

## Preserving Verification Artifacts

The probe script and configuration are kept outside the repository. The
configuration is described below; the probe script opened one keep-alive
connection to the health API and one to the HTTP tracker, sent the signal to the
exact PID, then polled `/health_check` on a new connection and on the open
connection every 100 ms.

## Environment and Prerequisites

- Date: 2026-10-28 (UTC).
- Artifact under test: `develop` at the merge of #2686 (no SI-21 code), debug
  build from `cargo build`.
- OS: Linux.
- Config: `[application.shutdown]` defaults (P 25 s, HTTP drain 15 s, UDP 5 s,
  listener 5 s); sqlite3 database in `/tmp`; UDP 6868/6969, HTTP 7070/7171, REST
  1212, health-check API `127.0.0.1:1313`.

## Verification Processes

### V1: Baseline probes during SIGINT and SIGTERM

- Goal: see what new and open health-check connections receive after the signal.
- Initial state: tracker ready, pre-probe `HTTP 200`.
- Status: Trigger only (baseline). No readiness flip exists yet, so a 503 cannot
  occur; this records the behaviour a flip placed before `jobs.cancel()` would
  meet. Single run per signal.

Steps Performed: start the binary, wait for `/health_check` to answer, open the
two connections, `kill -SIGINT|-SIGTERM <exact PID>`, poll.

Observed Result (offsets from script start):

| Signal | Offset      | New connection              | Open connection        |
| ------ | ----------- | --------------------------- | ---------------------- |
| SIGINT | 0.211 s     | HTTP 200 (before signal)    | -                      |
| SIGINT | 0.215 s     | Connection reset by peer    | empty read (closed)    |
| SIGINT | 0.315 s on  | Connection refused (3.4 s+) | Broken pipe            |
| SIGTERM| first probe | timed out (0.5 s)           | empty read (closed)    |
| SIGTERM| after that  | Connection refused          | Broken pipe            |

No probe ever saw HTTP 503. SIGTERM exit code 0 at about 6 s.

SIGTERM log timeline (2026-10-28 UTC):

- 17:48:15.572 "Torrust tracker shutting down (SIGTERM) ..." followed within
  about 70 µs by "Requested the job to stop: it waits for no other job" for every
  component, including the health-check API.
- 17:48:17.5755 health-check API "Stopped server".
- 17:48:20.5787 HTTP job completed, then the listener-drain jobs.
- 17:48:20.5788 "successfully shutdown".

Conclusion: in the current ordering the health-check server is cancelled with
everything else, so a readiness flip before `jobs.cancel()` yields no observable
503 on new connections. Caveat: the HTTP-drain holder did not really keep the
drain open (SIGTERM exit at about 6 s, not 15 s); the held-drain case is V2.
Cause per source (read from
source, not run): `start_health_check_api` in `src/app.rs` adds no stop-before
edge.

### V2: Probes during SIGINT and SIGTERM with a held REST request

Classification: Reproduced for "no 503 on new connections" with a real drain
of about 5 s. Not a 15 s drain: see the caveats.

Same tracker and configuration as V1 (defaults: P 25 s, HTTP/REST drain 15 s,
UDP drain 5 s, listener drain 5 s). Holder: a partial
`POST /api/v1/keys?token=MyAccessToken` to the REST API (port 1212) with
`Content-Length: 100` and body `{"key":`, so the request stays in flight. The
probes ran every 100 ms on a new connection and on a keep-alive connection
opened before the signal. Signals went to the exact PID.

SIGTERM (relative seconds from start):

- 0.211 pre-probe HTTP 200; 0.216 signal sent.
- New connections at 0.216, 0.316 and 0.417: "Connection refused".
- Open connection at 0.216: empty read, then "Broken pipe".
- Holder still open at every poll (the component was still draining).
- 5.234 holder received `HTTP/1.1 408 Request Timeout` (`connection: close`)
  and the process exited with code 0.
- Log: `graceful_shutdown_on_cancellation{address=0.0.0.0:1212 drain_timeout=15s}`,
  then "Job completed ... job=http_api", then "successfully shutdown".

SIGINT:

- 0.211 pre-probe HTTP 200; 0.215 signal sent.
- New connection at 0.215: "Connection reset by peer".
- Open connection at 0.216: empty read; holder still open.
- 5.233 holder received a 408; process exited with code 0 at 5.234.
- The per-probe lines between 0.3 s and 5 s were filtered out by my script and
  are assumed, not verified, to match SIGTERM.

Caveats: one run per signal; the hold ended through hyper's roughly 5 s
request-body timeout, not the 15 s drain, so a longer hold is not measured;
the SIGINT intermediate probes are missing.

Conclusion: the process stayed alive about 5 s while a request was in flight,
and the health-check server refused or reset every new connection and every
probe on the open connection, and never answered 503.

## After the Change (implementation)

Artifact under test: the implementation branch `2697-mark-health-unhealthy-during-shutdown` (debug
build, `cargo build --locked`), 2026-11-04 (UTC). Probe script: throwaway, kept outside the
repository (it signals the exact PID, holds a partial `POST /api/v1/keys` on the REST API with
`Content-Length: 100`, then polls `/health_check` every about 250 ms on a new connection and on a
keep-alive connection opened before the signal). Configuration: loopback listeners (UDP 6969, HTTP
7070, REST 1212, health 1313), sqlite3, and `[application.shutdown]` as stated per run.

### V3: Deterministic readiness tests

Status: DONE. `cargo test --locked` for the touched packages, all passing:

- `torrust-tracker-primitives` `readiness::tests` (5): ready by default, not ready after the mark,
  one-way when marked twice, shared between clones, eight concurrent readers all observe it.
- Health check API `tests/server/contract.rs` `readiness` (3): not ready answers 503 with the
  `shutting down` report and **runs no registered probe** (a counting probe), ready answers 200 and
  runs the probe, and 503 stays on every later request.
- `JobManager` stop-last tests (5): the stop-last component is cancelled only after every other
  component completed (they declare no edge themselves), a panicked component still releases it, a hung
  component leaves it `Aborted` at the deadline with exit code 1, a dropped reservation blocks no one,
  and reserving it after another component panics.
- `src/app.rs`: the existing stop-order test now asserts that `health_check_api` waits for every other
  component (the legacy `udp_ban_cleanup` job has no component id and is not covered), and
  `it_should_mark_the_application_not_ready_before_any_component_observes_its_cancellation` runs
  `begin_shutdown` 50 times with an observer on another thread.

Mutation checks: bypassing the readiness check in the handler fails two of the three contract tests;
swapping `begin_shutdown` to cancel first fails the ordering test in repeated runs (a single attempt
can pass, hence the 50 attempts).

### V4: Drain with SIGTERM and SIGINT on the binary

Status: DONE (one run per signal). Defaults for `[application.shutdown]` (P 25 s, H 15 s). The REST
holder ends after about 5 s through hyper's request-body timeout (as in V2), so the drain is about
5 s, not 15 s.

SIGTERM (offsets from process start): pre-probe `HTTP/1.1 200 OK` on a new and on the open
connection at 0.107/0.112 s; SIGTERM sent at 0.317 s. From 0.317 s to 5.269 s every probe, on new and
on the open connection (20 probes each), got `HTTP/1.1 503 Service Unavailable` with
`{"status":"Error","message":"shutting down","details":[]}`. At 5.269 s the holder received `408`. From
5.519 s new connections were refused, and the process exited with status 0 at 6.530 s (6.2 s after
the signal).

SIGINT: the same: pre-probe 200, SIGINT at 0.316 s, 503 on new and open connections for the whole
drain (20 probes), exit status 0, 6.2 s after the signal.

Log order (SIGINT run): "Torrust tracker shutting down (SIGINT)" 11:52:08.003; every other component
"Requested the job to stop" in the same millisecond, and `health_check_api` listed as waiting for
every other job; `http_api` completed 11:52:13.0085; "Requested the job to stop: every job it waited
for has completed job=health_check_api last_completed_job=http_api"; health check "Stopped server"
11:52:14.0097; "successfully shutdown" 11:52:14.0098. The first 503 precedes every drain-complete log
and the exit.

Observation: the health server's own drain took 1.0 s after `http_api` completed, with the probe's
open keep-alive connection present. It is covered by the process deadline but is not "near-instant"
in that case; a probe that opens a new connection each time (as Kubernetes does) leaves nothing to
wait for. Not measured without a keep-alive connection.

### V5: Healthy tracker unchanged

Status: DONE. The pre-probe of V4 and V6 answers `HTTP/1.1 200 OK` with
`{"status":"Ok","message":"","details":[...]}` listing the UDP, HTTP and REST services (the body is
unchanged; the integration test `it_should_probe_the_services_and_answer_200_while_the_application_is_ready`
and the existing contract tests also pass).

### V6: Failure path (shortened deadline)

Status: DONE (SIGTERM, one run). `[application.shutdown]`: `deadline_in_secs = 4`,
`http_drain_in_secs = 2`, `udp_request_drain_in_secs = 1`, `event_listener_drain_in_secs = 1`. The
partial REST request outlived the 2 s drain. 503 on new and open connections from the signal (0.316 s)
until the REST drain ended; `http_api` was reported as failed ("tracker API graceful drain timed
out", 11:53:38.730), then `health_check_api` was released and stopped 1.0 s later. The process exited
with status 1 at 3.4 s, 3.1 s after the signal and inside P = 4 s, and logged "shutdown was
incomplete ... incomplete_components=http_api". The health server did not delay the exit past P.
A hung component that never completes would be aborted at P together with the health server (unit
test above); the validated budgets make that hard to provoke on the binary, and it was not run.

### V7: Startup failure

Status: DONE. With the REST port already bound by another process, the binary printed "Tracker
startup failed: ... Address already in use", exited with status 1 after 0.05 s, and nothing listened on
the health check port afterwards (connection refused): a failed start never advertises readiness.

### V8: Operator documentation

Status: DONE. `docs/features/shutdown-process/README.md` ("Readiness Contract for Operators", with a
Kubernetes readiness-probe snippet), the health check package README, and the ADR
`docs/adrs/20261104120915_stop_the_health_check_api_last_to_report_not_ready_during_shutdown.md`.

## Failures and Follow-up

- Early SIGINT attempts failed with "Address already in use", most likely
  lingering TIME_WAIT sockets; waiting about 40 s between runs fixed it.
- Follow-up in the implementation (done): repeated with a real drain holder, SIGTERM and SIGINT, exit
  status and the failure path after the chosen design (V4, V6).
- Listener release was not separately measured: the refused connections after the drain show the
  health port closed; the other ports were not probed again.
- The health check API logs each 503 at error level (a server-error response); noisy during a drain.
