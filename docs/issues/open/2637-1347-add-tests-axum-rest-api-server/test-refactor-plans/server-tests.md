---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/server.rs
status: done
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/server.rs
    - packages/axum-rest-api-server/src/test_support.rs
    - packages/axum-rest-api-server/tests/fixtures/https-health-check-cert.pem
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/lessons.md
---

# Server Lifecycle File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/axum-rest-api-server/src/server.rs`, ledger
row 18 (rank 6), the only file of S5. Status: done; the maintainer approved the plan and the completed file on 2026-11-06.

## Current State

- **Unit-only coverage:** 403 / 458 lines (87.99%), 666 / 767 regions, 35 / 53 functions (stable Rust 1.99.1,
  `cargo-llvm-cov 0.6.16`, clean workspace, `--lib`, the S5 baseline in `coverage-evidence.md`). The integration tests add
  no line of this file. The 458 lines include the in-file test module.
- **Existing tests (6):** the legacy start and stop of `ApiServer`; the token-aware drain on cancellation; the drain timeout
  with a held connection; the release of the listener when the token-aware registration fails; `health_check_url` for both
  protocols; the TLS REST API reported healthy through `check_fn_with_client` (from #2730, using the fixtures
  `tests/fixtures/https-health-check-{cert,key}.pem`, `install_rustls_crypto_provider` and `RustlsConfig::from_pem`).
- **Module-owned decisions:**
  - `start_with_cancellation` / `start_token_aware_runtime`: bind the configured address (a bind failure is `Error::Bind`,
    surfaced by `start_with_cancellation`), choose the protocol from the launcher's TLS option, build the service binding
    from the address the OS assigned, serve (plain with the timeout acceptor, TLS without it), start the drain controller
    with the launcher's drain timeout, register the service with `check_fn`, and on a registration failure cancel, abort
    and join both tasks so the listener is released.
  - `check_fn` / `check_fn_with_client` / `health_check_url`: probe `<binding url>api/health_check` and report the status
    text, or the error text when the request fails.
  - `Launcher`'s `Display`: the wording that `Stopped` and the `ApiServer::start` span print.
  - Owned elsewhere: the HTTP and TLS serving, the drain outcome and timeout policy (`torrust-tracker-axum-server`), the
    registration rules (`torrust-server-lib`), the router (`routes.rs`, tested in S2), the container
    (`rest-api-runtime-adapter`), and the legacy `Halted` path (#1488, which deprecates and removes it).
- **T1 hypothesis:** test candidate for the token-aware start errors, `check_fn`, and the `Launcher` text; the rest is
  owned elsewhere or unreachable.

## Current Tests Review

Conclusion: `refactor` (R1), small and test-only. Concrete smells against `write-unit-test`:

1. **A private `api_container` duplicates `crate::test_support::api_container`** (S2) and leaves its SQLite file in the
   temporary directory, the problem fixed for the shared helper in Cameron's F3. Replace it with the shared helper and keep
   the returned `TestApiContainer` alive for as long as the server runs.
2. **Awaited operations without an absolute deadline** (spec rule for async and TLS code, and Cameron's F4): container
   setup, `start_with_cancellation`, `Registar::services` and the legacy `start`/`stop` are awaited bare in every test.
   Wrap them in the shared `within_deadline`, and replace the local `TEST_COMPLETION_TIMEOUT` by the shared
   `TEST_DEADLINE` (same 5 s).
3. **A dead branch in the legacy test:** `if let Some(tls_config) = &http_api_config.tls_config { make_rust_tls ... }`
   (lines 583-584) never runs, because `ephemeral_public()` has no TLS. Remove the branch and pass `None`; the legacy test
   itself is otherwise untouched (#1488 owns that path).
4. **The TLS health check test does not join the drain controller:** it ends with `drop(timeout(running.task))`. Cancel,
   then join the task and the controller, each within the deadline.
5. The registration-failure test binds a free port, drops it and binds again later: a small race with parallel tests that
   is unavoidable, because the duplicate registration must name the exact address the launcher binds. Keep, record here.

## Cleanup Ownership And Deadlines

For every new or changed test: **normal path**: the test cancels its token and joins the runtime task and the drain
controller within `TEST_DEADLINE` (the pattern of the existing drain test). **Failure path** (an assertion panics): the
`#[tokio::test]` runtime is dropped, which aborts the spawned server and controller tasks and closes the listener; the
`TestApiContainer` removes its database directory afterwards. **Cancellation:** owned by the `CancellationToken` of the
test. **Drop path:** no thread, process or file outlives the test. Port 0 is used wherever the address does not matter,
and the tests that need a taken or known port hold the occupying `TcpListener` until the end. Every awaited operation
(`api_container`, `start_with_cancellation`, `Registar::services`, the health check job, a request) is inside
`within_deadline` or `tokio::time::timeout(TEST_DEADLINE, ...)`.

## Coverage And Decision Analysis

The 55 uncovered lines, by group:

| Lines | Behavior group | Classification | Decision |
| --- | --- | --- | --- |
| 277, 242 | `start_token_aware_runtime` bind failure, and its `?` in `start_with_cancellation` | 3A unit | N1: bind to an address that is already bound; expect `Error::Bind` whose source kind is `AddrInUse`. Fails before any task is spawned, so nothing needs cleanup but the occupying listener. |
| 379-381 | `check_fn` (the production wrapper with the default client) | 3A unit | N2: start the plain REST API, take its registered binding and run `check_fn`; expect `Ok("200 OK")`. Also assert the job's `info` text (`checking api health check at: <url>`) if `info` is public. |
| 396 | `check_fn_with_client` error arm | 3A unit | N3: a binding to a free port that nothing listens on (bound, read and dropped); the job returns `Err(_)`. Assert it is an `Err` and that the message names the checked URL; the exact `reqwest` wording is not pinned. |
| 428-434 | `Launcher`'s `Display` (TLS and plain) | 3A unit | N4: two cases, literal text `(with socket): 127.0.0.1:1212, using TLS` and `... without TLS` (a TLS option from the fixtures). The `Stopped` prefix is derive output and not tested. |
| plain protocol selection (covered, not asserted) | `Protocol::HTTP` chosen without TLS and registered | 3A unit | N5: after a plain start the registered binding has protocol `HTTP` and the address `running.local_addr` (the TLS counterpart is implied by the existing #2730 test, whose health check only succeeds over `https`). Shares the start helper with N2. |
| 244-245 | Public-URL log arm | Logging-only | Not tested: a `tracing` line without an observer; asserting log output at unit level would test the subscriber. |
| 279-280, 289-290, 294, 310 | `set_nonblocking`, `local_addr`, `ServiceBinding::new` and the server constructors failing | Platform-owned / unreachable | Need an OS or library fault; the address is always valid after a successful bind. No test; owners are the OS and `torrust-tracker-axum-server`. |
| 296-302, 312-319 | Runtime error arms of the serving futures (a server that stops with an error after start) | Collaborator-owned / unreachable | The server only fails when `axum_server` fails; no seam without changing production code. Owner `torrust-tracker-axum-server`. |
| 198, 358-363, 455-457, 469-472, 478-492, 494-502, 514 | Legacy `Halted` path: the startup notification error, `stop` error arms, `Launcher::start` listener errors, its TLS start and the dropped-notification arm | Lifecycle (#1488) | Not tested beyond what exists: #1488 deprecates and removes the path. |
| 583-584 | Dead TLS branch of the legacy test | Test hygiene | R1 item 3: remove it (it leaves the denominator). |
| 742-743 | Failure arms (`panic!`) of the `match` in the registration-failure test | Structural | They run only when the test already fails; leave them. |
| `Error` and `RuntimeError` `Display` texts | `thiserror` messages | Not tested | Same precedent as `QueryParamError` in S4 (maintainer's decision): nothing in the package branches on the wording, and a test would restate the attribute. A candidate for the maintainer's call. |
| `check_fn` returning `Ok` for a failing HTTP status | Finding | #2733 (fixed on `develop` after the slice-close rebase) | No test of mine pins the old behavior; the fix and its own two tests are on `develop`. |

Expected effect (an estimate; the measured result is in Results): about 12 uncovered lines were expected to be covered (277, 242, 379-381, 396, 428-434) and 2 removed, with every remaining line owned as above.

## Property-Test Decision

None. Every case is a boundary example (a taken address, a dead address, two `Display` forms); no dependency is added.

## Dependencies

None new: `tempfile`, `rustls`, `reqwest` and the fixtures are already available.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Refactor the existing tests as in Current Tests Review (shared `api_container`, deadlines, dead branch, join in the TLS test). | The six existing tests still pass; focused run, `cargo +nightly fmt --all -- --check`, `git diff --check`; prose-first AAA review. |
| R2 | DONE | Add N1-N5 as in-file tests (a shared start helper for N2 and N5). | Same validation plus the named mutations below. |
| R3 | DONE | Integration increment: `none` expected (the integration tests already serve the plain REST API over the test environment). | Recorded in Results. |
| R4 | DONE | Re-run the three coverage scopes; record before and after. | Results and `coverage-evidence.md`. |

## Named Mutations

Each is a temporary hand-made change a test must catch; apply one at a time, revert exactly, and confirm an assertion
failure, not a compile error (`unused = "deny"`).

| ID | Site | Temporary change | Expected killer |
| --- | --- | --- | --- |
| S1 | `start_token_aware_runtime` | Map the bind failure to `Error::Listener`. | N1 |
| S2 | `start_token_aware_runtime` | Choose `Protocol::HTTPS` for the plain launcher. | N5 |
| S3 | `start_token_aware_runtime` | Choose `Protocol::HTTP` for the TLS launcher. | the #2730 TLS test (its check needs `https`) |
| S4 | `check_fn_with_client` | Turn `Err(err) => Err(err.to_string())` into `Ok(String::new())`. | N3 |
| S5 | `check_fn` | Use a client that cannot connect (a `Client` with a proxy at a dead address). | N2 |
| S6 | `Launcher::fmt` | Swap the two texts. | N4 |
| S7 | `check_fn_with_client` | Change the info text. | the info assertion of N2, if `info` is public |
| S8 | `start_with_cancellation` | Remove `cancellation_token.cancel()` from the registration-failure arm. | the token assertion added to the registration-failure test after the review (it survived the first run) |
| S9 | the new deadline helpers | Place a pending future before the awaited start. | the deadline message, in one test of each kind |

## Prose-First AAA Review

Temporary prose, written before comparing with the code.

| Test | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_fail_to_start_with_a_bind_error_when_the_address_is_already_in_use` (N1) | A listener already holds an address; a launcher is told to bind that same address; the container exists. | Start the token-aware server. | The result is a bind error whose cause is "address in use"; the occupying listener is dropped at the end. | Matches the code: one `match` extracts the error kind, one `assert_eq!` on the literal kind. |
| `it_should_register_the_plain_rest_api_with_an_http_binding_on_the_address_it_serves` (N5) | A plain server is started with a registar. | Read the registered services. | One service, with protocol HTTP and the address the server reports. | Matches; one tuple assertion, then the server is stopped and joined. |
| `it_should_report_a_plain_rest_api_as_healthy_with_the_default_health_check` (N2) | A plain server is running and registered; the expected info text is a literal built from the served address. | Run the production `check_fn` on the registered binding. | The job's info text and result are the literal text and `200 OK`. | Matches; the info text is built from `local_addr`, not by the production helper. |
| `it_should_report_an_error_with_the_checked_url_when_nothing_listens_on_the_checked_address` (N3) | An address that was free and is dropped, so nothing listens. | Run `check_fn`. | The job fails and its message names the checked URL. | Matches; the `reqwest` wording is not pinned. |
| `it_should_display_the_launcher_with_its_socket_and_whether_it_uses_tls` (N4) | A TLS launcher and a plain one for the same socket. | Format both. | The two literal texts. | Matches; needs the process-level crypto provider because it loads the fixture certificate. |

Refactored (R1): `start_plain` and `stop` hide the incidental start and join mechanics for the tests that start the plain
server, the shared `api_container` and `within_deadline` replace the private container and the bare awaits, the dead TLS
branch is gone, and the TLS test now joins both tasks.

## Results

- **Unit-only coverage** (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace, latest figures after the slice-close rebase and the review fix): `server.rs` 403 / 458 lines at the S5 baseline; 579 / 624 lines (92.79%), 987 / 1,067 regions and 58 / 73 functions after (counts include the test module and the tests `develop` added for #2733). The 45 uncovered lines are 198, 244-245, 279-280, 289-290, 294, 296, 301-302, 310, 312, 318-319, 358-359, 362-363, 456-458, 470, 472-473, 479-484, 488-489, 491-493, 495, 497, 503, 515, 747-748 and 869-870; every one is in a group of the analysis above (logging-only, platform, runtime error arms, legacy #1488) or is a `panic!` arm of a test that runs only when the test already fails (747-748 of the registration-failure test and 869-870 of the bind-error test). Package unit-only lines 2,324 / 2,403 (96.71%), aggregate 2,348 / 2,403 (97.71%), integration-only 682 / 903 (75.53%).
- **Tests:** 5 added by this slice. The package had 159 unit tests at the S5 baseline and has 167 after the rebase (`develop` added tests of its own for #2733); 58 integration tests, unchanged.
- **Mutations** (each applied alone, stable toolchain, then reverted; the file was identical to the tested one afterwards):
  S1 killed by the bind-error test; S2 by the plain registration test, the plain health check test and the existing
  listener-release test; S3 by the existing TLS health check test; S4 by the dead-address test; S5 by the plain health check
  test; S6 by the `Display` test; S7 by the plain health check test (its info assertion); S9 by three tests that start the
  plain server, through the deadline. **S8 first survived, and the claim that it was an equivalent mutant was wrong** (found
  by the independent review): removing only `cancellation_token.cancel()` from the registration-failure arm leaves the
  listener released, because the aborts do that, but it stops cancelling the caller's token, which the production caller
  passes as a clone. The registration-failure test now keeps a clone of its token and asserts `is_cancelled()`; S8 is
  killed by it. The mutant S8b (the cancel, both aborts and both joins removed) is killed by the listener-release assertions
  of the same test. Corrections while running: S7's first
  anchor matched twice (the test text), so it was anchored on the `let info = format!(` line.
- **Dependency:** none new. **Clippy allows:** none added.
- **Validation:** `cargo +nightly fmt --all -- --check`, `cargo clippy -p torrust-tracker-axum-rest-api-server
  --all-targets` (stable), `cargo machete --with-metadata`, `git diff --check` and `cargo test -p
  torrust-tracker-axum-rest-api-server` (164 unit and 58 integration tests) pass.
- **Review outcome:** the maintainer approved the plan on 2026-11-06; the completed file was approved on 2026-11-06. The independent review (REVIEW PASSED) led to the token assertion above, to this refresh of the figures, and to a note: the `check_fn` tests (N2, N3) use `reqwest::Client::new()`, which honors a proxy set in the environment (`HTTP_PROXY`), whereas the TLS test builds its client with `no_proxy()`; on a machine with a proxy configured for localhost they could fail. Accepted for now: the production `check_fn` has the same property and the tests must exercise it.

## Progress Log

- 2026-11-05 22:03 UTC - Claude Code - Plan drafted from the S5 baseline; waiting for the maintainer's approval.
- 2026-11-06 08:07 UTC - Claude Code - Plan approved by the maintainer; R1-R4 done; the completed-file review is pending.
- 2026-11-06 08:10 UTC - Claude Code - Completed file approved by the maintainer; row 18 `DONE`.
- 2026-11-06 08:13 UTC - Claude Code - Correction after the slice-close rebase: #2733 is fixed on `develop` (`check_fn` reports `Err` for a non-success status and its own two tests cover it), so the plan's finding row for #2733 is obsolete and no test of mine touches that arm. Re-measured: `server.rs` 576 / 621 lines, 981 / 1,061 regions, 58 / 73 functions; the same 45 uncovered lines (198, 244-245, 279-280, 289-290, 294, 296, 301-302, 310, 312, 318-319, 358-359, 362-363, 456-458, 470, 472-473, 479-484, 488-489, 491-493, 495, 497, 503, 515, 745-746, 863-864). Package unit-only lines 2,321 / 2,400 (96.71%), 167 unit tests.
- 2026-11-06 08:23 UTC - Claude Code - Independent review (REVIEW PASSED, finding 1): S8 is not an equivalent mutant, because `cancel()` cancels the caller's token. Added an `is_cancelled()` assertion to the registration-failure test; S8 and S8b are killed. Refreshed the Results body to the latest figures (see the entry above for the rebase); re-measured: `server.rs` 579 / 624 lines, 987 / 1,067 regions, 58 / 73 functions; package unit-only lines 2,324 / 2,403 (96.71%).
