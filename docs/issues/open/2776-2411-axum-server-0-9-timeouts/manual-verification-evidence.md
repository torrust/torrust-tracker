---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2776-2411-axum-server-0-9-timeouts/ISSUE.md
last-updated-utc: "2026-11-10 17:25"
---

# Manual Verification Evidence

## Purpose

Real evidence for issue #2776: the baseline reproduction attempted while drafting the
specification, the red and green runs, the with/without-wrapper comparison that decides the wrapper
removal, and the final like-for-like recheck. Toolchain for the sections added after
implementation: rustc 1.99.1, cargo 1.101.0-nightly (`cargo +nightly fmt`), 2026-11-10 UTC.

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

### Red and green runs (T3, T4)

Commands, toolchain and the raw output are in [red-run.txt](evidence/red-run.txt) and
[green-run.txt](evidence/green-run.txt).

- Red, idle-connection tests: at commit `test(axum-server): [#2776] add red idle-connection
  regression tests` (`first_request` declared in `Timeouts` but not applied) the silent plaintext
  and the silent TLS (after the handshake) tests fail at the 3 s test deadline; the other seven
  pass (stalled handshake, one-byte handshake, post-response idle for HTTP/1 and HTTP/2 over
  plaintext and TLS, and a well-behaved client). Those seven are already bounded with `axum-server`
  0.9, so they guard against regressions rather than reproduce a defect.
- Red, health check API: with only its constructor line reverted to `axum_server::from_tcp`, the
  test fails after its 8 s bound.
- Green: all four Axum packages pass on the final code (`green-run.txt`).

### V8 - Comparison with and without `TimeoutAcceptor` (T5, AC4 and AC5)

Both builds contain the `first_request_timeout` fix and differ only in the wrapper on the
plaintext HTTP tracker and REST API listeners. "With wrapper" is the build
`fix(axum-server): [#2776] close idle connections with first_request_timeout`; "without wrapper"
is the working tree that became `refactor(axum-server): [#2776] remove the TimeoutAcceptor
wrapper`. The with-wrapper run was repeated after review finding F1 (2026-11-10 UTC). Probes: `baseline-probe.py` plus
[probe-h2.py](evidence/probe-h2.py) (HTTP/2 prior knowledge or ALPN `h2`, one request, then
silence; the client never answers pings), run by [run-probes.sh](evidence/run-probes.sh) (the baseline tracker.toml with the ports below) on
loopback ports 27070, 27171, 21212 and 21313 (the default ports were used by other sessions).
Raw results: [with wrapper](evidence/results-with-wrapper.txt) and
[without wrapper](evidence/results-without-wrapper.txt).

| Scenario                                          | With wrapper | Without wrapper |
| ------------------------------------------------- | ------------ | --------------- |
| M1 silent, HTTP tracker plaintext                 | 5.0 s        | 5.0 s           |
| M1 silent, REST API plaintext                     | 5.0 s        | 5.0 s           |
| M1 silent, health check API                       | 5.0 s        | 5.0 s           |
| M2 silent, HTTP tracker TLS after handshake       | 5.0 s        | 5.0 s           |
| M3 TLS port, no handshake bytes                   | 10.0 s       | 10.0 s          |
| M4 HTTP/1 keep-alive after response, HTTP tracker | 5.0 s        | 5.0 s           |
| M4 HTTP/1 keep-alive after response, health check | 5.0 s        | 5.0 s           |
| M4 HTTP/2 idle after response, HTTP tracker plain | 10.0 s       | 10.0 s          |
| M4 HTTP/2 idle after response, HTTP tracker TLS   | 10.0 s       | 10.0 s          |
| M4 HTTP/2 idle after response, health check       | 10.0 s       | 10.0 s          |
| M4 HTTP/2 idle after response, REST API plain     | 10.0 s       | 10.0 s          |

Conclusion: the replacement is no worse than the wrapper in any measured case.

HTTP/2 and the wrapper (review F1): the wrapper did not shorten an idle HTTP/2 connection on the
plaintext listeners; the rerun with the wrapper still gives 10.0 s for the HTTP tracker and REST
API. Probable cause, read from the removed code and not separately tested: `TimeoutStream::poll_read`
consumes one `TimerSignal` per call. For `GET /` the service sends `Wait` and the empty response body
sends `Reset` at once; the first read handles `Wait`, so the timer stays disarmed, and Hyper does not
read the idle HTTP/2 socket again until the 5 s keep-alive ping goes unanswered and closes it after
5 s more. For HTTP/1 Hyper reads again right after the response, handles `Reset`, and the wrapper
closes at 5 s as measured.

The HTTP/2 idle bound is therefore about 10 s (5 s ping interval plus 5 s reply wait) with and
without the wrapper. Whether that satisfies AC4 ("no worse than today's plaintext 5 s") is for the
maintainer to rule: the wrapper's own HTTP/2 result is not better, but the 5 s figure in AC4 is
not met for HTTP/2. A client that answers HTTP/2 pings keeps the connection open; that is a
responsive peer, and bounding it is the connection-limit work of EPIC #2411 case A1.
No measurement of `develop` (the build before this change) was made for HTTP/2.

### V9 - Final like-for-like recheck (T9)

Build: `refactor(axum-server): [#2776] remove the TimeoutAcceptor wrapper`, the last production
change (2026-11-10 UTC, `cargo build --bin torrust-tracker`, dev profile). Same probes and ports as V8, raw output in
[results-final-build.txt](evidence/results-final-build.txt):

```text
M1-http-plain: closed by server after 5.0s
M1-rest-plain: closed by server after 5.0s
M1-health: closed by server after 5.0s
M2-tls-silent: closed by server after 5.0s
M3-tls-no-handshake: closed by server after 10.0s
M4-http-keep-alive: first response: HTTP/1.1 200 OK
M4-http-keep-alive: closed by server after 5.0s
M4-health-keep-alive: first response: HTTP/1.1 200 OK
M4-health-keep-alive: closed by server after 5.0s
M4-h2-http-plain: closed by server after 10.0s from request
M4-h2-http-tls: alpn=h2
M4-h2-http-tls: closed by server after 10.0s from request
M4-h2-health-plain: closed by server after 10.0s from request
M4-h2-rest-plain: closed by server after 10.0s from request
```

| Scenario                                  | Baseline (`develop`, 0.8.0)  | Final build                |
| ----------------------------------------- | ---------------------------- | -------------------------- |
| M1 silent health check API                | STILL OPEN after 25 s        | Closed after 5.0 s         |
| M2 silent TLS after handshake             | STILL OPEN after 25 s        | Closed after 5.0 s         |
| M4 keep-alive health check API (HTTP/1)   | STILL OPEN after 25 s        | Closed after 5.0 s         |
| M1 silent plaintext HTTP tracker, REST    | Closed after 5.0 s           | Closed after 5.0 s         |
| M3 TLS port, no handshake                 | Closed after 10.0 s          | Closed after 10.0 s        |
| M4 keep-alive HTTP tracker (HTTP/1)       | Closed after 5.0 s           | Closed after 5.0 s         |

Not probed on the REST API over TLS (same constructor and code path as the HTTP tracker TLS
listener, covered by the automatic TLS tests of the shared constructor).

### V5 to V7 - Traffic unaffected (M5)

On the final build with the same configuration:

```text
REST API   GET /api/v1/stats?token=...          200 (HTTP/1.1)
REST API   same request, h2c prior knowledge    200 (HTTP/2)
HTTP tracker plaintext  GET /health_check       200
HTTP tracker TLS        GET /health_check       200 (HTTP/1.1 and HTTP/2 over ALPN)
Health check API        GET /health_check       200
```

The health check API reports `Error` overall because it checks the TLS HTTP tracker with a
self-signed certificate it does not trust; the same happens without this change. The slow-handler
`408` path is covered by the existing unit test in `packages/axum-rest-api-server/src/routes.rs`
(passes in the package test run). M5a (the e2e runner) was not run: it needs Docker and runs in CI.

### TLS handshake timeout (T6)

The TLS listeners bound the handshake with `RustlsAcceptor::handshake_timeout`, now set from
`Timeouts::tls_handshake` (10 s, the `axum-server` default). A connection that sends no
handshake bytes, or one byte and stalls, is closed after that time (M3: 10.0 s; automatic tests
with an injected 300 ms value). `first_request_timeout` starts only after the handshake, so the
two bounds do not add up inside one phase.

## Results Summary

| Scenario ID | Status | Evidence section |
| ----------- | ------ | ---------------- |
| M1          | DONE (final build) | V8, V9 |
| M2          | DONE (final build) | V8, V9 |
| M3          | DONE (final build) | V8, V9 |
| M4          | DONE (final build, HTTP/1 and HTTP/2) | V8, V9 |
| M5a         | NOT RUN (needs Docker; CI) | V5 to V7 |
| M5b         | DONE | V5 to V7 |
| M5c         | DONE (existing unit test) | V5 to V7 |

## Open Issues and Follow-Up

- Repeat the final recheck (V9) after any production change made in review.
