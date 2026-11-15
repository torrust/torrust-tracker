---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2867-udp-health-check-panics-on-invalid-response/ISSUE.md
last-updated-utc: 2026-11-15 12:45
---

# Manual Verification Evidence

## Purpose

Record real verification of the defect and, later, of its fix. Every command and output below was
actually run; nothing is invented.

## Preserving Verification Artifacts

The temporary reproduction tests are stored verbatim as a diff in
[`evidence/reproduction-tests.diff`](evidence/reproduction-tests.diff). The full test output of
each run is in [`evidence/junk-reply-run.log`](evidence/junk-reply-run.log) and
[`evidence/closed-port-run.log`](evidence/closed-port-run.log). The tests were reverted after the
runs; `git status --short` showed no change under `packages/` afterwards.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-15 12:30 to 12:45
- Artifact under test: `develop` at commit `abd7a10ff` (Merge torrust/torrust-tracker#2863), with
  the temporary tests from `evidence/reproduction-tests.diff` applied to
  `packages/axum-health-check-api-server/tests/server/contract.rs`
- Operating system / environment: Linux 7.0.0-43-generic
- Toolchain: stable Rust toolchain, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo 1.99.1`
- Prerequisites and setup performed: none beyond a normal workspace build

## Verification Processes

### V1 - Health check of a UDP service that answers with junk bytes (initial reproduction)

- Goal: observe what a client of `GET /health_check` receives when a registered UDP tracker
  answers the connect request with a datagram that is not a valid tracker response.
- Initial state: a plain `std::net::UdpSocket` on `127.0.0.1:0`, with a thread that answers every
  datagram with the 4 bytes `junk`. It is registered in a `Registar` as a `UdpTracker` service
  whose check function is the production `torrust_tracker_udp_server::server::launcher::Launcher::check`.
  The health-check API server is started with that registar.
- Status: `DONE`
- Outcome: **Reproduced** (the wrong outcome itself was observed: the HTTP client gets no
  response at all).

#### Steps Performed

1. Applied the temporary test
   `it_should_return_error_when_the_registered_udp_service_answers_the_check_with_junk`
   (see `evidence/reproduction-tests.diff`). It sends `GET /health_check` and expects a `200`
   report whose status is `Error` and whose UDP detail has an `Err` result.
2. Ran:

   ```bash
   cargo test -p torrust-tracker-axum-health-check-api-server --test integration answers_the_check_with_junk -- --nocapture
   ```

#### Observed Result

Test names are shortened to `...answers_the_check_with_junk`; the full output is in
`evidence/junk-reply-run.log`.

```text
thread '...answers_the_check_with_junk' panicked at packages/tracker-client/src/udp/client.rs:275:40:
called `Result::unwrap()` on an `Err` value: UnableToParseResponse { err: Custom { kind: InvalidData, error: "invalid data" }, response: [106, 117, 110, 107] }

thread '...answers_the_check_with_junk' panicked at packages/axum-health-check-api-server/src/handlers.rs:71:26:
it should be able to join into the checking function: JoinError::Panic(Id(5), "called `Result::unwrap()` on an `Err` value: UnableToParseResponse { ... }", ...)

thread '...answers_the_check_with_junk' panicked at packages/axum-health-check-api-server/src/handlers.rs:79:20:
it should be able to connect to the job: JoinError::Panic(Id(6), "it should be able to join into the checking function: ...", ...)

thread '...answers_the_check_with_junk' panicked at packages/axum-health-check-api-server/tests/server/client.rs:18:72:
called `Result::unwrap()` on an `Err` value: reqwest::Error { kind: Request, url: "http://127.0.0.1:46559/health_check", source: hyper_util::client::legacy::Error(SendRequest, hyper::Error(IncompleteMessage)) }
test server::contract::udp::it_should_return_error_when_the_registered_udp_service_answers_the_check_with_junk ... FAILED
```

#### Conclusion

The expected result (a `200` report with status `Error` and an error detail for the UDP service)
was not met. Three panics chain: the UDP check task, then the per-service report task, then the
request handler. The handler panic drops the HTTP connection, so the client sees
`IncompleteMessage` and gets no report for any service.

### V2 - Health check of a closed UDP port (scope check, not a reproduction)

- Goal: check whether a closed port (which can make `recv` fail on a connected UDP socket) reaches
  the same `unwrap`.
- Initial state: same as V1, but the socket is dropped right after reading its address, so nothing
  listens on the registered port.
- Status: `DONE`

#### Steps Performed

1. Applied the temporary test
   `it_should_return_error_when_the_registered_udp_service_is_a_closed_udp_port`
   (see `evidence/reproduction-tests.diff`).
2. Ran:

   ```bash
   cargo test -p torrust-tracker-axum-health-check-api-server --test integration is_a_closed_udp_port -- --nocapture
   ```

#### Observed Result

```text
test server::contract::udp::it_should_return_error_when_the_registered_udp_service_is_a_closed_udp_port ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 3.01s
```

#### Conclusion

On this machine a closed port does not panic: the check reports an error (the 2 s `select!` sleep
wins and the result is `Timed Out`). The defect is triggered by a reply that arrives but does not
parse. A receive error (`UnableToReceivingData`) would reach the same `unwrap`, but it was not
observed here.

## Failures and Follow-up

- The first V1 attempt did not compile (`let _ = socket.send_to(..)` is denied by the
  `let_underscore_drop` lint in the workspace flags). It was changed to `drop(socket.send_to(..))`
  and rerun; the diff in `evidence/` is the version that ran.
