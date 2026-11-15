---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md
last-updated-utc: "2026-11-15 12:19"
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
