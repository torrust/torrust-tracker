---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2867-udp-health-check-panics-on-invalid-response/ISSUE.md
last-updated-utc: 2026-11-16 09:50
---

# Manual Verification Evidence

## Purpose

Record real verification of the defect and, later, of its fix. Every command and output below was
actually run; nothing is invented.

## Preserving Verification Artifacts

The temporary reproduction tests are stored verbatim as a diff in
[`evidence/reproduction-tests.diff`](evidence/reproduction-tests.diff). The test output of each
run is in [`evidence/junk-reply-run.log`](evidence/junk-reply-run.log),
[`evidence/closed-port-run.log`](evidence/closed-port-run.log) and
[`evidence/closed-port-value-check-run.log`](evidence/closed-port-value-check-run.log); the V3 test
change is in [`evidence/closed-port-value-check.diff`](evidence/closed-port-value-check.diff). The tests were reverted after the
runs; `git status --short` showed no change under `packages/` afterwards.

These three logs are not the full output: each starts at the `Finished` line, so the build lines
of those runs (`Compiling ..`) were not kept. The logs of the regression-test runs and of V4
(`evidence/t*-run.log`, `evidence/stopped-udp-run.log`) keep the whole output, build lines
included, with the local checkout path replaced by `<repo>`. Every `t*-run.log` followed a change
or a `touch` of a source file of the package it tests, so each shows that package being rebuilt
(`Compiling ..`). `stopped-udp-run.log` ran right after the T3 green run with no change in
between, so it reuses that build and has no `Compiling` line.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-15 12:30 to 12:45 (V1, V2), 21:28 to 21:29 (V3), and
  2026-11-16 09:20 to 09:45 (regression tests and V4)
- Artifact under test: `develop` at commit `abd7a10ff` (Merge torrust/torrust-tracker#2863), with
  the temporary tests from `evidence/reproduction-tests.diff` (V1, V2) or
  `evidence/closed-port-value-check.diff` (V3) applied to
  `packages/axum-health-check-api-server/tests/server/contract.rs`. The regression tests and V4 ran
  on the implementation branch, based on `develop` at commit `a25a27e23` (Merge
  torrust/torrust-tracker#2865), at the states named in their sections.
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
   `it_should_return_error_when_the_registered_udp_service_answers_the_check_with_junk`. V1 ran
   with only this test added: the closed-port test of V2 did not exist yet, so the log reports
   `11 filtered out` (12 tests). The stored `evidence/reproduction-tests.diff` adds both tests and is
   what V2 ran; its first test is the V1 test byte for byte. It sends `GET /health_check` and expects a `200`
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
   (see `evidence/reproduction-tests.diff`, which also contains the V1 test, so the log reports
   `12 filtered out`). The test was made by copying the V1 test, and its comment line still says
   the peer "answers the health check with junk bytes", which is wrong for this test. The diff is
   kept as it ran; V3 reruns this test with the comment corrected.
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

On this machine a closed port does not panic: the check reports an error. This test asserts only
`details.result.is_err()`; the `Timed Out` value is inferred from the 3.01 s run time (the 2 s
`select!` sleep). V3 asserts the value. The defect is triggered by a reply that arrives but does not
parse. A receive error (`UnableToReceivingData`) would reach the same `unwrap`, but it was not
observed here.

### V3 - Closed UDP port reports exactly `Timed Out` (rerun of V2 with a value assertion)

- Goal: observe the `Timed Out` value that V2 only inferred.
- Initial state: same as V2, with two changes to the closed-port test: it asserts
  `assert_eq!(details.result, Err("Timed Out".to_string()))` instead of `is_err()`, and its comment
  says nothing listens on the registered port.
- Status: `DONE`

#### Steps Performed

1. Applied `evidence/closed-port-value-check.diff` (the V2 diff with the two changes above) at
   2026-11-15 21:28 UTC, on `develop` at `abd7a10ff`, stable Rust toolchain `rustc 1.99.1`. The
   stored log starts at `Finished`, so no kept file shows that the integration test binary was
   rebuilt with the value assertion; the `Compiling` line was only seen in the terminal and was not
   kept. The `Timed Out` value does not rest on this run alone: V4 runs the maintained stopped-UDP
   contract test, which asserts the same value, with its full log kept.
2. Ran:

   ```bash
   cargo test -p torrust-tracker-axum-health-check-api-server --test integration is_a_closed_udp_port -- --nocapture
   ```

3. Reverted the test file (`git checkout packages/`); `git status --short` then listed only the
   new evidence files.

#### Observed Result

Full output: `evidence/closed-port-value-check-run.log`.

```text
test server::contract::udp::it_should_return_error_when_the_registered_udp_service_is_a_closed_udp_port ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 3.01s
```

#### Conclusion

A closed UDP port gives exactly `Err("Timed Out")`, with no panic. The maintained contract test
`it_should_return_error_when_udp_service_was_stopped_after_registration` asserts the same value for
a stopped service.

## Regression Tests: Red and Green

Toolchain for every run: stable Rust, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`. Each log keeps the
full output, build lines included.

| Test | Red run (against the bug) | Green run (after the fix) |
| ---- | ------------------------- | ------------------------- |
| T2 `check_should_return_an_error_naming_the_cause_when_the_peer_answers_with_an_invalid_response` (`tracker-client` unit test) | [`evidence/t2-red-run.log`](evidence/t2-red-run.log): panics at `client.rs:275` on `UnableToParseResponse`. Ran with the test added and no fix. | [`evidence/t2-green-run.log`](evidence/t2-green-run.log): `1 passed`, after `fix(tracker-client): [#2867] return an error from the UDP health check on an invalid response`. |
| T3 `it_should_return_error_when_the_registered_udp_service_answers_the_check_with_junk` (UDP contract test, AC2) | [`evidence/t3-red-run.log`](evidence/t3-red-run.log): mutate-then-restore. The `check` fix was reverted in the working tree (not staged) while the handler fix was present; the detail is `Err("the health check job failed: task .. panicked ..")`, which does not name the cause, so the assertion fails. The file was then restored. | [`evidence/t3-green-run.log`](evidence/t3-green-run.log): `1 passed`, with both fixes. |
| T5 `it_should_report_a_probe_job_that_panicked_as_a_failed_check_and_keep_the_other_results` (handler contract test, AC3) | [`evidence/t5-red-run.log`](evidence/t5-red-run.log): the probe panic cascades through `handlers.rs:71` and `handlers.rs:79`, and the client gets `hyper::Error(IncompleteMessage)`. Ran with the `check` fix and no handler fix. | [`evidence/t5-green-run.log`](evidence/t5-green-run.log): `1 passed`; the handler logs the failed job at `error` level. |

An earlier T3 red run, before any fix and with a weaker assertion (`is_err()` only), failed with
the same three panics and `IncompleteMessage` as V1. Its log was overwritten by the
mutate-then-restore run above after the assertion was tightened to require the cause, so it is not
kept; V1 records that outcome.

### V4 - Like-for-like recheck after the fix (M3)

- Goal: rerun the V1 scenario on the fixed build.
- Initial state: the V1 test is now the maintained T3 test, with the same junk-answering UDP socket
  and the production `Launcher::check`, and assertions that also require a `200` report, the
  message `health check failed`, and an error detail naming the unparsable response.
- Artifact under test: the implementation branch with both fixes
  (`fix(tracker-client): [#2867] return an error from the UDP health check on an invalid response`
  and `fix(axum-health-check-api-server): [#2867] report a failed check job instead of panicking`).
- Status: `DONE`

#### Steps Performed

1. Ran the V1 command (without `--nocapture`):

   ```bash
   cargo test -p torrust-tracker-axum-health-check-api-server --test integration answers_the_check_with_junk
   ```

2. Ran the maintained stopped-UDP contract test, which is ignored by default (#2393), to check
   that the closed-port case still reports exactly `Timed Out` (AC4):

   ```bash
   cargo test -p torrust-tracker-axum-health-check-api-server --test integration udp_service_was_stopped_after_registration -- --ignored
   ```

#### Observed Result

Full output: [`evidence/t3-green-run.log`](evidence/t3-green-run.log) and
[`evidence/stopped-udp-run.log`](evidence/stopped-udp-run.log).

```text
test server::contract::udp::it_should_return_error_when_the_registered_udp_service_answers_the_check_with_junk ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 1.01s

test server::contract::udp::it_should_return_error_when_udp_service_was_stopped_after_registration ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 3.03s
```

#### Conclusion

The symptom of V1 is gone: no panic, and the client gets a `200` report with status `Error` and a
UDP detail whose `Err(..)` names the unrecognized response. A closed port still reports exactly
`Err("Timed Out")`.

## Failures and Follow-up

- The first V1 attempt did not compile (`let _ = socket.send_to(..)` is denied by the
  `let_underscore_drop` lint in the workspace flags). It was changed to `drop(socket.send_to(..))`
  and rerun; the diff in `evidence/` is the version that ran.
