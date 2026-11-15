---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md
last-updated-utc: "2026-11-15 12:40"
---

<!-- markdownlint-disable MD003 -->

# Manual Verification Evidence

## Purpose

Record the initial real-artifact reproduction and the like-for-like recheck required by the bug-fix
workflow. This draft contains observed results only; it does not claim a fix.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-30 13:15 to 13:29
- Artifact under test: `develop` at `4d64d3c34` (merge of PR #2390). Its only differences from
  `9d840f55d` (merge of PR #2388) are documentation files.
- Operating system: Linux workstation with 32 logical processors.
- Test runner: `cargo-nextest 0.9.88 (b44b7adf7 2025-01-15)`.
- Command configuration: default nextest concurrency, `--no-fail-fast` for repeated runs. Logs are
  retained locally under `.tmp/flaky/` and are not versioned.

## Verification Processes

### V1 - Original parallel-suite reproduction

- Goal: observe the intermittent failure against a real test artifact before any code change.
- Current hypothesis: tests either release a port selected by a port-`0` bind before the tested
  service binds it, or use a fixed port inside the system ephemeral range. Parallel tests can then
  claim the same address.
- Status: `REPRODUCED`

#### Steps Performed

1. Built the `axum-http-server` test binaries:

   ```sh
   cargo nextest run -p torrust-tracker-axum-http-server --no-run
   ```

2. Ran the two duplicate-registration tests 200 times:

   ```sh
   cargo nextest run -p torrust-tracker-axum-http-server --no-fail-fast \
     -E 'test(/duplicate_binding_error_when_registration_fails|registration_fails$/)'
   ```

3. Ran the full `axum-http-server` package suite 100 times:

   ```sh
   cargo nextest run -p torrust-tracker-axum-http-server --no-fail-fast
   ```

4. Built the default-member tests and ran the full suite once, then 50 more times:

   ```sh
   cargo nextest run --tests --all-features --no-run
   cargo nextest run --tests --all-features --no-fail-fast
   ```

#### Observed Result

```text
Two duplicate-registration tests only: 200 passed, 0 failed
axum-http-server package suite: 100 passed, 0 failed
full-suite run 1: 1147 passed, 1 failed
  torrust-tracker::cli-configuration
  base_source_precedence::it_should_select_the_cli_configuration_when_both_child_environment_base_sources_are_set
  tracker startup failed because the configured health-check API listener address was already in use
full-suite runs 2-51: 49 passed, 1 failed
  torrust-tracker-axum-http-server
  server::tests::it_should_release_the_listener_and_preserve_the_duplicate_binding_error_when_registration_fails
  HTTP listener should be released after registration failure: Address already in use (os error 98)
```

The CLI test configures the health-check API to `127.0.0.1:43152`; the Linux ephemeral range on
this host includes that port. The duplicate-registration fixture chooses a port by binding
`127.0.0.1:0`, reading its address, then dropping the listener before the tested `HttpServer`
binds it. Both sequences allow a concurrent test to claim the address.

#### Conclusion

Reproduced. The two narrow scopes did not fail, while the full parallel suite independently
reproduced both port-allocation failure modes. The exact failure observed during #2298's Docker run
is the second failure above, at the same `axum-http-server` assertion.

### V2 - Duplicate-registration repair

- Status: `NOT_APPLICABLE`
- The scope changed on 2026-11-14: the duplicate-registration test is ignored, not repaired, so
  there is no repair to verify. Its fate is in the spec's per-test table.

### V3 - CLI configuration repair

- Status: `NOT_APPLICABLE`
- On `develop` the CLI configuration tests already use `reserve_free_ports` (#2581), which draws
  below the ephemeral range. This PR does not change them.

### V4 - Full recheck

- Status: `DONE`
- Goal: show that the full parallel suite no longer fails on a port race, with enough runs for the
  claim. At the V1 rate of 1 failure in 50 runs on the `axum-http-server` assertion, 150 green runs
  would happen with no change at a probability of about 0.05.
- Environment: same workstation as V1 (32 logical processors), `cargo-nextest 0.9.88 (b44b7adf7
  2025-01-15)`, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, default nextest concurrency. Logs are kept
  locally under `.tmp/recheck-2393*/` and are not versioned.
- Command, run 150 times in a row after one `--no-run` build:

  ```sh
  cargo nextest run --tests --all-features --no-fail-fast
  ```

- Load: pre-commit hooks of other commits on this branch ran during part of each recheck, so the
  machine was sometimes busier than during V1.

#### Recheck 1 - the head ending in `refactor(tests): [#2393] name the UDP fixture constructor after its released address`, 2026-11-14 09:38 to 10:09 UTC

```text
148 runs: 1808 tests run: 1808 passed, 18 skipped
  2 runs: 1808 tests run: 1807 passed, 1 failed, 18 skipped
failed test, both times:
  torrust-tracker-axum-health-check-api-server::integration
  server::contract::api::it_should_return_error_when_api_service_was_stopped_after_registration
  it should start the health check service: Registration { source: DuplicateBinding(ServiceBinding
  { protocol: HTTP, bind_address: 127.0.0.1:39449 }) }   (the second run: 127.0.0.1:39709)
```

The test stops the REST API, which leaves its registration in the registry, then starts the
health-check API on port `0`. The kernel handed the health-check API the port just released, so its
registration collided with the stale one. This is a third racing shape, not in the original
inventory; the test exists unchanged on `develop`. The fix starts the health-check API before
stopping the REST API, and does the same in the HTTP variant (the UDP variant cannot collide).

#### Recheck 2 - the head ending in `fix(tests): [#2393] start the health-check API before stopping the probed service`, 2026-11-14 10:11 to 10:41 UTC

```text
150 runs: 1808 tests run: 1808 passed, 18 skipped
failed runs: 0
```

#### Check 3 - after the third review, 2026-11-14 11:23 to 11:27 UTC

On the head ending in `docs(pr-reviews): [#2393] join the findings table and update the T6 state in
the PR 2859 audit`. Its test changes since recheck 2 only move tests between running and ignored:
three health-check contract tests are ignored, and the launcher test runs again without its
socket-release assertion, which moved to an ignored companion. No test binds a port in a new way, so
this is a smaller check of the same command, not a new sized recheck:

```text
20 runs: 1806 tests run: 1806 passed, 21 skipped
failed runs: 0
```

#### Conclusion

All 150 runs of recheck 2 passed, with the 18 tests ignored at that head skipped. At the V1 rate, 150
green runs with the race still present would have a probability of about 0.05. The recheck covers
this workstation only; CI runs one suite at a time and was not part of it.

### V5 - T5 red-then-green

Each restored or rewritten test is run against a mutation that removes what it guards, then against
the committed code. Mutations are applied to the working tree only and restored before any commit.
A mutation is read only after its output shows it compiled.

#### T8 - health-check contract tests, 2026-11-15 12:19 UTC

Command:
`cargo nextest run -p torrust-tracker-axum-health-check-api-server --all-features --no-fail-fast -E 'test(/refuses_the_check|never_answers_the_check/)'`

| Test (`server::contract::…`) | Mutation: the probed address answers | Red | Green |
| --- | --- | --- | --- |
| `api::it_should_return_error_when_the_registered_api_service_refuses_the_check` | A listening socket answers every request with `200 OK`. | `report.status`: left `Ok`, right `Error` | pass |
| `http::it_should_return_error_when_the_registered_http_service_refuses_the_check` | Same. | `report.status`: left `Ok`, right `Error` | pass |
| `udp::it_should_return_error_when_the_registered_udp_service_never_answers_the_check` | The held socket answers every connect request with a valid connect response. | `report.status`: left `Ok`, right `Error` | pass |

A first UDP mutation answered with junk bytes. The test failed, but by a panic in the health-check
handler, not by its assertion, so it was replaced by the valid connect response above. That panic is
noted under [Failures and Follow-up](#failures-and-follow-up).

#### T9 - the loopback helper, 2026-11-15 12:20 UTC

Command:
`cargo nextest run -p torrust-tracker-test-helpers --all-features --no-fail-fast -E 'test(/loopback|127_0_0_1/)'`

| Mutation of `random::loopback_ipv4` | Failed tests (`random::tests::…`) |
| --- | --- |
| A: always returns `127.0.0.1` | `it_should_return_a_loopback_address_other_than_127_0_0_1` ("127.0.0.1 should have its last three octets in 1..=254"); `it_should_return_an_address_that_can_bind_a_port_another_socket_holds_on_127_0_0_1` (`AddrInUse`) |
| B: octets drawn from `0..=255` | `it_should_return_a_loopback_address_other_than_127_0_0_1` ("127.226.175.255 should have its last three octets in 1..=254") |

Green: both tests pass on the committed code. Mutation B leaves the port-sharing test passing, as
expected: a random address in the wider range still shares the port.

#### T9 - server registration-failure tests, 2026-11-15 12:22 UTC

Tests: `server::tests::it_should_release_the_listener_when_token_aware_startup_registration_fails`
in `axum-http-server`, `axum-rest-api-server` and `axum-health-check-api-server`.

Command:
`cargo nextest run --all-features --no-fail-fast -p torrust-tracker-axum-rest-api-server -p torrust-tracker-axum-http-server -p torrust-tracker-axum-health-check-api-server -E 'test(/it_should_release_the_listener_when_token_aware_startup_registration_fails/)'`

- Mutation, in each server's registration-failure branch: drop the server task and the drain
  controller (detaching them) instead of aborting and joining them. The REST API branch still
  cancels the caller's token, so only the release clause is broken.
- Red: 5 of 5 runs, 3 of 3 tests failed each time, each on its release assertion, for example
  "REST API listener should be released after token-aware registration failure: … AddrInUse".
- Green: 20 of 20 runs passed on the committed code.

#### T9 - UDP server tests, 2026-11-15 12:26 UTC

Tests in `udp-server`:

- C: `server::tests::token_aware_start::it_should_stop_the_receive_loop_and_release_the_socket_when_its_cancellation_token_is_cancelled`
- R: `server::tests::token_aware_start::it_should_release_the_socket_when_registration_fails`
- O: `server::launcher::tests::receive_loop_shutdown::it_should_release_the_socket_once_the_processors_orphaned_by_the_request_ring_are_joined`

Command: `cargo nextest run --all-features --no-fail-fast -p torrust-tracker-udp-server -E 'test(/…/)'`
with the three test names.

| Mutation | Failed | Passed |
| --- | --- | --- |
| U1: the receive loop's cancellation branch leaks a clone of its socket (`std::mem::forget`) | C ("the UDP socket should be released after the receive loop stops: … AddrInUse"); O ("the socket should be released once the aborted processors are joined") | R, whose failure path aborts the loop instead of cancelling it |
| U2: the registration-failure branch in `states.rs` drops the receive loop (detaching it) instead of aborting and joining it | R ("the UDP socket should be released after registration failure: … AddrInUse") | C, O |
| U3: the drain detaches the processors left at the deadline instead of aborting and joining them | O (after its 5 s wait) | C, R |

Green: 20 of 20 runs of the three tests passed on the committed code.

#### T9 - application bootstrap tests, 2026-11-15 12:39 UTC

Tests in the `torrust-tracker` crate:

- U: `bootstrap::jobs::udp_tracker::tests::it_should_release_the_socket_when_the_component_is_dropped_before_it_runs`
- H: `bootstrap::jobs::health_check_api::tests::it_should_release_the_listener_when_the_component_is_dropped_before_it_runs`
- A: `app::tests::it_should_release_udp_listener_before_returning_from_start_after_setup_when_later_http_startup_fails`

Command: `cargo nextest run --all-features --no-fail-fast -p torrust-tracker -E 'test(/component_is_dropped_before_it_runs|it_should_release_udp_listener_before_returning/)'`

| Mutation | Failed | Passed |
| --- | --- | --- |
| J1: `OwnedTask::drop` no longer aborts its task | U ("dropping the unpolled UDP tracker component must release its socket at 127.253.175.146:35418 within 5s") | H, A |
| J2: `TokenAwareServerTask::drop` no longer aborts the server task and drain controller | H ("dropping the unpolled health check API component must release its listener at 127.134.231.197:34239 within 5s") | U, A |
| J3: `start_jobs` returns a startup error without cancelling the job manager and waiting for its components | A ("UDP listener should be released before startup returns: … AddrInUse") | U, H |

The failure messages show the random loopback addresses the tests bound. Green: 20 of 20 runs of
the three tests passed on the committed code.

### V7 - Test-design review per increment

Each review follows the prose-first Arrange-Act-Assert step of the `write-unit-test` skill: the
prose is written first, compared with the code, and removed once the code says it.

#### T8 - health-check contract tests

Prose, shared by the three tests:

- Arrange: a service of one role is registered with its own production health check at an address
  the test holds for the whole test, where nothing answers: a TCP socket bound but not listening
  refuses the connection, and a UDP socket never replies.
- Act: a client asks the health-check API for its report.
- Assert: the report is an error, "health check failed", and its one detail names the probed
  binding, the service type, the probe description, and the check's failure.

Comparison and changes:

- The causal state (an address that does not answer) was hidden in the old tests behind starting and
  stopping a real service, which is also what released the port to other tests. It is now named by
  `hold_a_tcp_address_that_refuses_connections()` and by the `silent_socket` binding, and the probed
  `binding` is read from the held socket and used again in the assertions.
- Registering the service and starting the health-check API are incidental mechanics, so they moved
  to two named helpers, `registar_with_one_service` and `start_health_check_api`. The production
  health check passed to the registration (`check_fn`, `Launcher::check`) stays visible, because it
  selects the behavior.
- The Act, the request to `/health_check`, stays in the test body.
- The expected values are literals in the test, not derived from production code. The assertions
  describe one observable report, so they stay in one test.
- The old names said "was stopped after registration", which no longer happens. The tests are renamed
  for the state they set up: "refuses the check" (TCP) and "never answers the check" (UDP).
- Prose kept: one comment in the UDP test, because a held socket that never answers is not evident
  from `UdpSocket::bind`.

#### T9 - the loopback helper

Prose:

- `it_should_return_a_loopback_address_other_than_127_0_0_1`. Arrange: nothing; the helper takes no
  input. Act: draw many addresses. Assert: each is a loopback address whose last three octets are in
  `1..=254`, which is what rules out `127.0.0.1`.
- `it_should_return_an_address_that_can_bind_a_port_another_socket_holds_on_127_0_0_1`. Arrange:
  sockets on `127.0.0.1` hold a TCP port and a UDP port, as another test could after the code under
  test releases them. Act: bind the helper's address on those same ports. Assert: both binds succeed.

Comparison and changes:

- The first test has no Arrange section; the loop is the Act, repeated so that an out-of-range
  octet would show up. Its message names the sampled address and the range, so a failure needs no
  file lookup.
- The second test states the property the restored tests rely on, and the reason the helper exists.
  The two sockets on `127.0.0.1` are the causal state and stay in the test body. TCP and UDP stay in
  one test, because the rule is the same for both; each assertion message names its protocol. It is
  ignored off Linux, like the restored tests.
- No prose was kept; the test names and messages say it.

#### T9 - server registration-failure tests

Prose, shared by the three tests:

- Arrange: an address that no server holds, on a loopback address no other test uses, is already
  claimed in the registry by another registration.
- Act: start the server on that address with a cancellation token.
- Assert: the start fails with the duplicate-binding registration error for that address (the REST
  API test also checks that the caller's token is cancelled), and the address can be bound again as
  soon as the start returns.

Comparison and changes:

- The only change is where the address comes from: `random::loopback_ipv4()` instead of
  `127.0.0.1`. It is test isolation, not part of the causal state, so in the HTTP and health-check
  tests it stays inside the `ServerStartWithDuplicateRegistration` fixture. A comment there now says
  why the address is not `127.0.0.1`, next to the existing note that the listener is released before
  the start.
- The Act and the assertions are unchanged and visible.
- Not changed: the REST API test builds the same scenario inline (address, configuration,
  registration), a Complex Arrange compared with the other two. Extracting a fixture is a refactor
  outside this change, so it is left for the maintainer's review rather than done here.
- Prose kept: the comment on the address choice, because the reason is not evident from the call.

#### T9 - UDP server tests

Prose:

- C. Arrange: a token-aware UDP server runs on port `0` of a loopback address no other test uses.
  Act: cancel its token and join the receive loop. Assert: the loop stops without an error, and its
  address can be bound again.
- R. Arrange: an address that no server holds, on a loopback address no other test uses, is already
  claimed in the registry. Act: start the server on it. Assert: the start fails with the
  duplicate-binding error for that address, and the address can be bound again.
- O. Arrange: a receive loop on a loopback address no other test uses, whose full request ring has
  dropped the only handles of processors that are still running, with a drain deadline shorter than
  their hold. Act: cancel and join the receive loop. Assert: the address becomes bindable within the
  lifecycle deadline.

Comparison and changes:

- C no longer needs to know its address before the start, so it now starts on port `0` and reads
  the bound address back (`bound_to`). This removes the handoff between choosing the port and the
  server binding it, which the old released-address fixture had.
- R needs the address before the start, to claim it in the registry, so it keeps
  `on_a_released_free_address`. That fixture's documentation said only ignored tests used it; it
  now says why the address is safe and when to prefer `bound_to`.
- O: the shared `ReceiveLoopHoldingRequests` fixture now takes the server address instead of reading
  it from `UdpLauncherTestContext`, whose `bind_address` field had no other use and is removed. The
  orphaned-processor scenario gains `start_on(server_ip)`, used by O. Its companion test keeps
  `start()`, which binds `127.0.0.1`, because it does not rebind and must keep running on every
  system. The client sockets stay on `127.0.0.1`.
- The Acts and the assertions are unchanged.
- Prose kept: in C and O, the comment on the address choice.

#### T9 - application bootstrap tests

Prose:

- U and H. Arrange: a component (UDP tracker, health-check API) is started on an address chosen
  before the start, on a loopback address no other test uses; its tasks are already spawned but the
  component future is not polled. Act: drop the component. Assert: the address becomes bindable
  within the test deadline.
- A. Arrange: the UDP tracker is configured on such an address, and the HTTP tracker on a port the
  test keeps occupied, so the HTTP start fails after the UDP tracker started. Act: run the
  application startup. Assert: it fails, and the UDP address can be bound as soon as it returns.

Comparison and changes:

- In each test the address helper (`available_udp_address`, `available_health_check_address`,
  `reserve_udp_address`) now binds `random::loopback_ipv4()`. Its documentation says why; the test
  bodies, Acts and assertions are unchanged.
- U and H must know the address before the start, because the unpolled component does not expose
  it, so they keep choosing and releasing it first; the random loopback address removes that handoff
  race too.
- A keeps its occupied HTTP port on `127.0.0.1`: the test holds it for the whole run, so it cannot
  race.
- Prose kept: the helpers' documentation.

## Failures and Follow-up

- No production listener failed. The affected behavior is test address ownership and test-only
  configuration.
- The source inventory found other port-selection fixtures in `axum-rest-api-server`,
  `axum-health-check-api-server`, `udp-server`, and bootstrap tests. Their disposition belongs to
  T5; do not assume they are unsafe merely because they bind port `0`.
- Found during V5 (out of scope for #2393): when a UDP service answers the health check with a
  datagram that is not a valid tracker response, `check` in `tracker-client/src/udp/client.rs`
  unwraps the parse error and panics, and the health-check handler's `expect` on the joined job
  panics in turn, so the client gets no report at all instead of an error detail.
