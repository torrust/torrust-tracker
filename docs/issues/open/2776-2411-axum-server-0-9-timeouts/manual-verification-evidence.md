---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2776-2411-axum-server-0-9-timeouts/ISSUE.md
last-updated-utc: "2026-11-06 21:10"
---

# Manual Verification Evidence

## Purpose

Real evidence for issue #2776. This file holds the baseline reproduction attempted while drafting
the specification. The final recheck, and the comparisons that decide the wrapper removal, are
added after implementation.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-06, 20:56 to 21:00
- Artifact under test: `torrust-tracker` built from `develop` at the merge commit
  `Merge torrust/torrust-tracker#2755: chore(issues): archive closed issue #2715 spec to docs/issues/closed`
  (id `997d85416`, already on `develop`; the issue-spec commit on top of it changes only
  documentation). `axum-server` 0.8.0 from `Cargo.lock`.
- Operating system / environment: Linux 7.0.0-43-generic, rustc 1.99.1, Python 3.14.4, local
  loopback only.
- Setup: `cargo build --bin torrust-tracker`; a throwaway self-signed certificate made with
  `openssl req -x509 -newkey rsa:2048 -nodes -days 2 -subj "/CN=localhost"`; the tracker started
  with `TORRUST_TRACKER_CONFIG_TOML_PATH` pointing at
  [baseline-tracker.toml](evidence/baseline-tracker.toml) (HTTP tracker plaintext `:7070`, HTTP
  tracker TLS `:7171`, REST API plaintext `:1212`, health check API `:1313`).
- Probe: [baseline-probe.py](evidence/baseline-probe.py) connects, sends nothing (or one request
  for the keep-alive case), and waits up to 25 s for the server to close. The probes ran one at a
  time by [baseline-probe-results.txt](evidence/baseline-probe-results.txt). An earlier attempt
  measured the wrong thing (its pipeline always waited for its own `sleep 25`) and was discarded.

## Verification Processes

### V1 to V4 - Baseline reproduction (outcome: Reproduced)

Command: `python3 -I probe.py <label> <port> <plain|tls|persistent> [path]`, run in sequence from
20:58:24 to 21:00:04 UTC. Raw output:

```text
M1 plaintext HTTP tracker :7070, silent: closed by server after 5.0s
M1 plaintext REST API :1212, silent: closed by server after 5.0s
M1 health check API :1313, silent: STILL OPEN after 25.0s (client gave up)
M2 TLS HTTP tracker :7171, handshake done, silent: STILL OPEN after 25.0s (client gave up)
M3 TLS HTTP tracker :7171, TCP only, no handshake: closed by server after 10.0s
M4 plaintext HTTP tracker :7070, keep-alive after GET /health_check: first response: HTTP/1.1 200 OK
M4 plaintext HTTP tracker :7070, keep-alive after GET /health_check: closed by server after 5.0s
M4 health check API :1313, keep-alive after GET /health_check: first response: HTTP/1.1 200 OK
M4 health check API :1313, keep-alive after GET /health_check: STILL OPEN after 25.0s (client gave up)
```

| Scenario                                 | Result at baseline                                                  |
| ---------------------------------------- | ------------------------------------------------------------------- |
| M1 silent plaintext HTTP tracker, REST   | Closed after 5 s: the existing wrapper protects these listeners     |
| M1 silent health check API               | Still open after 25 s: **defect reproduced**                        |
| M2 silent TLS connection after handshake | Still open after 25 s: **defect reproduced**                        |
| M3 TLS port, no handshake bytes          | Closed after 10 s (the fixed `axum-server` handshake timeout)       |
| M4 keep-alive plaintext HTTP tracker     | Closed 5 s after the response (wrapper behaviour to preserve)       |
| M4 keep-alive health check API           | Still open after 25 s: **defect reproduced**                        |

Observations that change the specification:

- The defect is exactly the gap the EPIC case A2 describes: TLS listeners and the health check
  API. It is not present on the plaintext HTTP tracker and REST API, so #324's telnet symptom is
  the original report, not a defect on every current listener.
- The 5 s `header_read_timeout` and HTTP/2 keep-alive settings do not close a silent TLS
  connection after the handshake, which is the case `first_request_timeout` targets.
- The plaintext keep-alive close at 5 s after a response is the existing behaviour the
  replacement must reproduce before the wrapper can be removed (HTTP/1; HTTP/2 not yet measured).
- Not run at baseline: the REST API over TLS (same code path as the HTTP tracker TLS listener,
  not separately probed), HTTP/2 clients, and the slow-handler `408` path.

## Results Summary

| Scenario ID | Status | Evidence section |
| ----------- | ------ | ---------------- |
| M1          | DONE (baseline) | V1 to V4 |
| M2          | DONE (baseline) | V1 to V4 |
| M3          | DONE (baseline) | V1 to V4 |
| M4          | DONE (baseline, HTTP/1 only) | V1 to V4 |
| M5          | TODO   | After implementation |

## Open Issues and Follow-Up

- Final like-for-like recheck on the final build, named by its Conventional Commit subject, after
  the last production change including review fixes.
