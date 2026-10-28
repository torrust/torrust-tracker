---
doc-type: manual-verification-evidence
issue-spec: docs/issues/drafts/1488-si-21-mark-health-unhealthy-during-shutdown/ISSUE.md
last-updated-utc: "2026-10-28 17:57"
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
drain open (SIGTERM exit at about 6 s, not 15 s), so the long-drain case is not
yet measured; it is needed to evidence design (a). Cause per source (read from
source, not run): `start_health_check_api` in `src/app.rs` adds no stop-before
edge.

## Failures and Follow-up

- Early SIGINT attempts failed with "Address already in use", most likely
  lingering TIME_WAIT sockets; waiting about 40 s between runs fixed it.
- Follow-up in the implementation: repeat with a real drain holder, SIGTERM and
  SIGINT, exit status and listener release, after the chosen design.
