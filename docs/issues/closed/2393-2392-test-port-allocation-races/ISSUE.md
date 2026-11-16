---
schema-version: 1
doc-type: issue
issue-type: bug
status: done
priority: p2
epic: 2392
github-issue: 2393
spec-path: docs/issues/closed/2393-2392-test-port-allocation-races/ISSUE.md
branch: "2392-test-isolation-spec"
related-pr: 2397
last-updated-utc: "2026-11-16 15:37"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - write-unit-test
  related-artifacts:
    - docs/issues/open/2392-test-isolation/EPIC.md
    - docs/issues/open/2395-2392-e2e-runner-dynamic-host-ports/ISSUE.md
    - .claude/skills/fix-bug/SKILL.md
    - .claude/skills/write-unit-test/SKILL.md
    - packages/axum-http-server/src/server.rs
    - tests/configuration/cli_configuration/base_source_precedence.rs
    - docs/issues/closed/2298-rust-dev-tool-container-integration/manual-verification-evidence.md
    - docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md
    - docs/issues/closed/2386-1840-self-hosted-runner-minimum-capacity/ISSUE.md
    - docs/issues/closed/2132-add-sigterm-to-main/native-shutdown-test-plan.md
    - tests/metrics/fixed_ports.rs
    - src/console/ci/e2e/runner.rs
---

<!-- skill-link: create-issue -->

<!-- markdownlint-disable MD003 -->

# Issue #2393 - Eliminate Parallel Test Port Allocation Races

Parent EPIC: #2392 - Test Isolation (`docs/issues/open/2392-test-isolation/EPIC.md`)

## Goal

Remove the intermittent `Address already in use` failures from the tracker test suite with a quick
patch. A flaky test is worse for development than no test, so each racing test is disabled with
`#[ignore]`, then deleted, rewritten without a real port, or kept disabled with a recorded reason.

The first phase (PR 2859, merged) disabled twenty-one racing tests and rewrote four. The second
phase (T5) applies the maintainer's decision for the twenty-one: delete nine, rewrite three to
probe an address the test holds, and restore nine on a unique loopback address so they no longer
race on `127.0.0.1`. When it is done, no test is left ignored for #2393 on Linux.

## Background

Several tests choose a TCP or UDP address by binding loopback port `0`, reading the address, and
then dropping the socket before the system under test binds it. Between those operations, another
parallel test or process can claim the released port. Other tests use fixed ports in Linux's
configurable ephemeral-port range, so a concurrent port-`0` bind can receive the same port.

The wrong outcome is an intermittent `Address already in use` failure unrelated to the behavior a
test intends to verify. The failure makes the suite nondeterministic and can hide regressions behind
infrastructure noise.

Real-artifact reproduction is recorded in `manual-verification-evidence.md`. A full nextest run
reproduced both observed failure modes: the `axum-http-server` duplicate-registration test failed
when an unrelated process claimed its released address, and a CLI configuration test failed because
its fixed health-check port was already in use.

### Related Issues

| Issue | Relationship |
| --- | --- |
| #1419 (open) | Same problem class for main-application integration tests. It moved most suites to port `0` with registry-based address discovery; `tests/metrics/fixed_ports.rs` still uses fixed ports `17091`-`17094`, below Linux's default ephemeral range (`32768`-`60999`). The inventory classifies that suite as safe; it does not change #1419's remaining #1488 shutdown work. |
| #2386 (open) | Documents fixed E2E host ports (`6969/udp`, `7070`, `1212`, `1313`) in `src/console/ci/e2e/runner.rs` that prevent concurrent E2E jobs on one Docker host. Different failure domain (Docker host, not nextest); sibling subissue #2395 owns it. |
| #2132 (closed) | Its native-process fixture design explicitly rejected "find a free port, release it, then spawn" and chose port `0` plus startup-log discovery. It is the precedent for port `0` with address read-back, which the rewritten UDP health-check test follows. |

Of these, only #1419 is itself a subissue of the parent EPIC; #2386 and #2132 are references.

## Scope

### In Scope

- Inventory the Rust tests that bind port `0`, drop the socket, and reuse the address, or that use
  fixed ports in the ephemeral range.
- Disable each racing test with `#[ignore = "flaky port race, see #2393"]`.
- Record a decision for every disabled test in this issue, so nothing stays ignored without one.
- Rewrite a test without a real port when it checks the tracker's own logic.
- T5: delete the nine ignored tests that only check test-support code, rewrite the three
  health-check contract tests to probe an address the test holds, and restore the nine tests of
  the tracker's socket-release contract on a unique loopback address.
- T5: add one shared test helper that returns a random loopback address in `127.0.0.0/8` (see
  Architectural Decisions).

### Out of Scope

- A server that takes over an already-bound socket (listener handoff). It was considered and
  rejected for now: it needs a production API change on four server crates.
- Any new or extended shared port helper. The #2581 helper (`reserve_free_ports`) stays as it is.
  The T5 loopback helper chooses an IP address, not a port, and the maintainer approved it on
  2026-11-15.
- Moving the other tests in the suite to unique loopback addresses. Only the nine restored tests
  use the helper.
- Serializing the entire test suite or lowering nextest concurrency.
- Changing tracker protocol behavior, default service ports, or deployment configuration.
- Fixed E2E container host ports in `src/console/ci/e2e/runner.rs` (sibling subissue #2395).
- #1419's remaining cooperative-shutdown acceptance criterion (AC8a, blocked on #1488).

## Inventory

The original spec is partly stale. On `develop`, the CLI configuration tests no longer use fixed
ephemeral ports: #2581 moved them to `reserve_free_ports`, which draws from a range below `32768`.
The duplicate-registration test the spec names is now
`it_should_release_the_listener_when_token_aware_startup_registration_fails`.

Three shapes race:

- **Bind, drop, reuse.** The test binds port `0`, reads the address, drops the socket and hands the
  address to the server. Another test can claim the port in between.
- **Release, then rebind.** The server binds port `0`, the test stops or fails it, then rebinds the
  released port (directly or in a wait loop) to prove it was released. A concurrent port-`0` bind can
  receive the same port in between. The V1 failure (`... should be released after registration
  failure: Address already in use`) has this assertion.
- **Released port handed back within one test.** A service binds port `0` and registers it, the test
  stops it, then starts a second service on port `0`. The kernel can hand the second service the
  port just released, which the registry still holds, so its registration fails with
  `DuplicateBinding`. The first V4 recheck found this shape (2 failures in 150 runs).

Safe, left unchanged: `tests/configuration/` (ports below the ephemeral range), `tests/metrics/fixed_ports.rs`
(`17091`-`17094`), the `src/bootstrap/config.rs` and `packages/configuration` tests (the ports are only
parsed, never bound), and tests that keep their listener open until the end (`src/app.rs` HTTP
listener, `src/bootstrap/jobs/http_tracker.rs`, the occupied-port tests).

### Per-test decisions

| Test | Package or file | Decision | Reason |
| --- | --- | --- | --- |
| `it_should_release_the_listener_when_token_aware_startup_registration_fails` | `axum-http-server` `server.rs` | T5: restore on a unique loopback address | The server is already serving when registration fails, so the error path must cancel and join the spawned task. Needs a pre-known address for the registry and a rebind to prove release. |
| `it_should_release_the_listener_when_token_aware_startup_registration_fails` | `axum-rest-api-server` `server.rs` | T5: restore on a unique loopback address | Same contract and shape. |
| `it_should_release_the_listener_when_token_aware_startup_registration_fails` | `axum-health-check-api-server` `server.rs` | T5: restore on a unique loopback address | Same contract and shape. |
| `it_should_stop_the_receive_loop_and_release_the_socket_when_its_cancellation_token_is_cancelled` | `udp-server` `server/mod.rs` | T5: restore on a unique loopback address | The cancellation path must stop the receive loop and drop its socket. Rebinds the released socket. |
| `it_should_release_the_socket_when_registration_fails` | `udp-server` `server/mod.rs` | T5: restore on a unique loopback address | Same contract as the server registration-failure tests. Needs a pre-known address and a rebind. |
| `it_should_release_the_socket_when_the_component_is_dropped_before_it_runs` | `src/bootstrap/jobs/udp_tracker.rs` | T5: restore on a unique loopback address | `start_job` spawns tasks before it returns the component future; dropping a `JoinHandle` detaches a task, so the release depends on the tracker's abort-on-drop code. Rebinds the released socket. |
| `it_should_release_the_listener_when_the_component_is_dropped_before_it_runs` | `src/bootstrap/jobs/health_check_api.rs` | T5: restore on a unique loopback address | Same contract as the UDP job test. Rebinds the released listener. |
| `it_should_release_udp_listener_before_returning_from_start_after_setup_when_later_http_startup_fails` | `src/app.rs` | T5: restore on a unique loopback address | Application startup must release the already-started UDP tracker before it returns a later HTTP failure. Rebinds the released UDP socket. |
| `it_should_release_the_socket_once_the_processors_orphaned_by_the_request_ring_are_joined` | `udp-server` `server/launcher.rs` | T5: restore on a unique loopback address | The socket must not be released while orphaned request processors still run. Waits until the released port can be rebound. |
| `it_should_release_the_health_check_api_binding_when_stopped` | `axum-health-check-api-server` `environment.rs` | T5: delete | Checks the `testing` feature's environment, not the tracker. Every user of the `testing` environments (the package tests and examples) binds port `0`, and the fixed-port suites start the main application, not these environments, so nothing depends on an environment releasing its port; the tracker's own release contract is covered by the restored tests above. |
| `it_should_release_the_health_check_api_binding_when_dropped_without_being_stopped` | `axum-health-check-api-server` `environment.rs` | T5: delete | Same reason. |
| `it_should_serve_requests_after_being_stopped_and_started_again_on_a_fixed_port` | `axum-health-check-api-server` `environment.rs` | T5: delete | Checks that the test environment restarts on its configured address. No test relies on that, and `it_should_serve_requests_after_being_stopped_and_started_again` still covers restart on port `0`. |
| `it_should_release_the_http_binding_when_stopped` | `axum-http-server` `testing/environment.rs` | T5: delete | Checks test-support code, as above. |
| `it_should_release_the_http_binding_when_dropped_without_being_stopped` | `axum-http-server` `testing/environment.rs` | T5: delete | Same reason. |
| `it_should_release_the_rest_api_binding_when_stopped` | `axum-rest-api-server` `testing/environment.rs` | T5: delete | Same reason. |
| `it_should_release_the_rest_api_binding_when_dropped_without_being_stopped` | `axum-rest-api-server` `testing/environment.rs` | T5: delete | Same reason. |
| `it_should_release_the_udp_socket_when_stopped` | `udp-server` `testing/environment.rs` | T5: delete | Same reason. |
| `it_should_release_the_udp_socket_when_dropped_without_being_stopped` | `udp-server` `testing/environment.rs` | T5: delete | Same reason. |
| `it_should_return_error_when_api_service_was_stopped_after_registration` | `axum-health-check-api-server` `tests/server/contract.rs` | T5: rewrite with an address the test holds | Checks that the health-check API reports an unreachable REST API as failing. The rewrite registers the REST API's public `check_fn` against a TCP socket the test binds to port `0` and never listens on, so the check is refused and no other test can take the port. |
| `it_should_return_error_when_http_service_was_stopped_after_registration` | `axum-health-check-api-server` `tests/server/contract.rs` | T5: rewrite with an address the test holds | Same rewrite with the HTTP tracker's public `check_fn`. |
| `it_should_return_error_when_udp_service_was_stopped_after_registration` | `axum-health-check-api-server` `tests/server/contract.rs` | T5: rewrite with an address the test holds | Same rewrite with the UDP tracker's `Launcher::check` against a UDP socket the test binds and never answers on, which gives the `Timed Out` result the test already expects. |
| `it_should_not_return_while_processors_orphaned_by_the_request_ring_are_still_running` | `udp-server` `server/launcher.rs` | Rewritten | Its main assertion (processors joined before the loop returns) needs no port, so it runs again; its socket-release assertion moved to the companion above. |
| `it_should_register_the_service_with_a_health_check_that_reaches_the_running_server` | `udp-server` `server/mod.rs` | Rewritten | Binds `127.0.0.1:0` and reads the bound address; it never needed a pre-selected port. |
| `it_should_cancel_the_udp_tracker_component_through_the_job_manager` | `src/app.rs` | Rewritten | Uses the configured port `0`; it never needed a reserved address. |
| `it_should_report_an_error_with_the_checked_url_when_nothing_listens_on_the_checked_address` | `axum-rest-api-server` `server.rs` | Rewritten | Checks the address of a socket the test binds to port `0` and never listens on, so nothing else can listen there. |

The maintainer decided the twenty-one ignored tests on 2026-11-15 (T5):

- **Delete nine.** They check the `testing` feature's environments, which is test-support code.
- **Rewrite three** health-check contract tests to probe an address the test holds open, without
  a helper and without a production change.
- **Restore nine** tests of the tracker's socket-release contract on a unique loopback address.
  Each covers a path where the socket belongs to a spawned task. Rust's ownership does not free a
  socket held like that, because dropping a `JoinHandle` detaches the task, so the release depends
  on the tracker's own cancel, join or abort code. A leak here would mostly hurt code that embeds
  the tracker and retries startup in one process, and the test suite itself; the binary usually
  exits on a startup failure.

## Architectural Decisions

- Related ADRs: none. Neither phase changes a production API. T5 adds one test helper, used only
  by the nine restored tests, so it sets no repository-wide test networking policy.
- A test that races on a port is disabled first. A flaky test is worse for development than a missing
  one, and some tests that need a fixed or pre-selected port may never have been worth having.
- The listener-handoff design (the server takes over an already-bound socket) was considered and
  rejected for now.
- **T5 unique loopback address (approved by the maintainer on 2026-11-15).** On Linux every address
  in `127.0.0.0/8` is loopback, and two sockets on different specific addresses can share a port
  (checked for TCP and UDP). A restored test binds port `0` on a random loopback address instead of
  `127.0.0.1`, reads the port back, and later rebinds that address to prove the release. Another
  test, in this run or in another agent's run, that binds `127.0.0.1:0` and happens to receive the
  released port no longer blocks the rebind.
  - The helper lives in `torrust-tracker-test-helpers` (it already depends on `rand`, and every
    affected crate already depends on it). It returns an `Ipv4Addr` of the form `127.a.b.c` with
    `a`, `b` and `c` in `1..=254`, so it never returns `127.0.0.1`. It chooses an address, not a
    port; the tests still bind port `0`.
  - Linux only. macOS and the BSDs configure only `127.0.0.1` on `lo0` (Windows answers on the
    whole range, but CI runs no tests off Linux), so each restored test conservatively carries
    `#[cfg_attr(not(target_os = "linux"), ignore = "needs the Linux 127.0.0.0/8 loopback range, see #2393")]`.
    CI runs the tests on Linux only; the macOS and Windows jobs only build.
  - It does not stop a socket bound to the wildcard address `0.0.0.0` on the same port. Some
    integration-test configurations bind `0.0.0.0:0`, so a small chance of collision remains; V6
    measures it.
- T5 rewrites the health-check contract tests without the helper: a TCP socket bound to port `0`
  and never listening refuses the check, and a UDP socket the test holds and never answers makes
  the check time out. The test owns the address for the whole test, so nothing else can take it.

## Bug-Fix Process

The defect is in the tests, not in tracker behavior. This patch disables the racing tests instead of
repairing them, so the red-then-green steps of `.claude/skills/fix-bug/SKILL.md` do
not apply to it: a disabled test has no regression boundary to prove red. The steps that still
apply:

1. Reproduce against a real artifact and record it (V1).
2. Inventory every test with a racing shape and record its decision (this spec).
3. Recheck the full parallel suite after the change, sized for the claim it supports (V4).

Each test restored or rewritten in T5 follows the full `fix-bug` process: it must still detect the
defect it guards. Before the change is committed, temporarily remove the cleanup the test guards
(or, for a contract test, make the probed address answer), confirm the test fails, restore the
code, and confirm it passes. Record each red-then-green pair in V5.

T8 and T9 produce tests, so they follow the progressive test-development loop of the
`write-unit-test` skill:

- Work in the smallest behavior-focused increments: the helper's unit test, the three contract
  tests, then the restored tests one crate or small family at a time.
- After each increment passes focused validation, and before the next one starts, review its
  design with the mandatory prose-first Arrange-Act-Assert comparison. Write temporary prose for
  each section, refactor until the code expresses it, and remove the redundant prose. The review
  confirms that the test exposes the one causal initial-state difference, that its fixture owns
  only incidental mechanics (judged by named actions and abstraction level, not caller count), and
  that the production Act and the independently specified expected result stay visible. Use the
  test refactoring-pattern catalog where it applies.
- Record each review in `manual-verification-evidence.md` (V7, one entry per increment).
- Stop for maintainer review after the final increment, before T10, the final commit or the pull
  request.

## Regression Test Strategy

The first phase adds no regression test: disabling the racing tests removes the race rather than
fixing it. The guard against recurrence is the 150-run full-suite recheck (V4): at the V1 rate of 1
failure in 50 runs on the `axum-http-server` assertion, 150 green runs would happen with no change
at a probability of about 0.05.

T5 restores twelve tests as the regression guards for the tracker's socket-release and
health-check contracts. V5 proves each still detects its defect; V6 repeats the 150-run full-suite
recheck with none of the #2393 tests ignored.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Reproduce and inventory the observed failures | Evidence records the 200 isolated, 100 package, and 51 full-suite runs. |
| T2 | DONE | Re-check the inventory on `develop` | The inventory above, including the stale CLI finding and the three racing shapes. |
| T3 | DONE | Disable each racing test with `#[ignore]` | Twenty-one tests ignored with the reason `flaky port race, see #2393`. |
| T4 | DONE | Rewrite the tests that never needed a racing port | Four tests rewritten. |
| T5 | DONE | Decide the fate of the twenty-one ignored tests | The maintainer's decisions in the per-test table, recorded by this spec-only change. |
| T6 | DONE | Full-suite recheck | V4 in `manual-verification-evidence.md`: the first 150 runs found a third racing shape (2 failures), and 150 runs on the fixed head all passed. |
| T7 | DONE | Delete the nine test-support release tests | The nine tests are gone, with any helper left unused removed too. |
| T8 | DONE | Rewrite the three health-check contract tests | They run without `#[ignore]` and probe an address the test holds; V5 red-then-green and V7 design review recorded. |
| T9 | DONE | Add the loopback helper and restore the nine release tests | The helper with its own unit test; the nine tests run without `#[ignore]` on Linux; V5 red-then-green and V7 design review recorded per increment; maintainer review after the final increment. |
| T10 | DONE | Full-suite recheck with no #2393 test ignored | V6: 150 full-suite runs pass, and `rg -t rust 'see #2393'` finds no `#[ignore]` outside the non-Linux `cfg_attr`. |
| T11 | DONE | Close-out | This spec archived to `docs/issues/closed/` and this issue's row in the EPIC set to DONE, after the implementation PR 2874 and the EPIC sync PR 2862 merged. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 to T4 | Ignored and rewritten tests, with their rows in this spec | One commit per racing shape or rewrite. |
| T5 | The per-test decisions | Spec-only PR, merged before T7 starts. |
| T6 | Recheck evidence | Documentation-only commit. |
| T7 | Deleted tests | One commit. |
| T8 | Contract-test rewrites | One commit. |
| T9 | Helper, then restored tests | One commit for the helper, then one per crate or small cohesive family. |
| T10 | Recheck evidence | Documentation-only commit. |

## Progress Tracking

### Workflow Checkpoints

- [x] Initial real-artifact reproduction recorded.
- [x] Draft reviewed and approved by the maintainer.
- [x] GitHub issue created and issue number added to this specification.
- [x] Racing tests inventoried, ignored or rewritten, and focused checks pass.
- [x] Full parallel recheck recorded (V4).
- [x] Fate of every ignored test decided by the maintainer (T5).
- [x] T5 decisions applied: tests deleted, rewritten or restored (T7 to T9), red-then-green recorded (V5), test-design review recorded per increment (V7).
- [x] Full parallel recheck with no #2393 test ignored (V6).
- [x] Acceptance criteria reviewed after implementation.
- [x] Implementation completion review recorded.

### Progress Log

- 2026-09-30 13:29 UTC - GitHub Copilot - Drafted after an `axum-http-server`
  listener-release failure was reproduced in the full local nextest suite. The same run exposed a
  fixed health-check port collision in a CLI configuration test. No implementation has started.
- 2026-09-30 13:34 UTC - GitHub Copilot - Linked related fixed-port issues #1419, #2386, and
  #2132; excluded E2E host ports (#2386) from scope; recorded that the CLI precedence test needs
  distinct predictable ports.
- 2026-09-30 13:38 UTC - GitHub Copilot - Placed under the new test isolation EPIC
  draft, with the E2E host-port work as a sibling subissue.
- 2026-09-30 14:49 UTC - GitHub Copilot - Consistency review: corrected which related issues are
  EPIC subissues and stated the port-inventory boundary with the shared-resource audit subissue.
- 2026-09-30 14:58 UTC - GitHub Copilot - Maintainer approved the draft. Created #2393 as a
  sub-issue of EPIC #2392 and moved the specification to `docs/issues/open/`.
- 2026-11-14 09:14 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer narrowed the scope through the EPIC coordinator: a quick patch,
  no owned-listener seam, no new port helper. Reverted the shared-helper change, re-checked the
  inventory on `develop`, ignored nine racing tests, rewrote three tests without a real port, and
  recorded the per-test decisions. Nine tests still await a maintainer decision.
- 2026-11-14 10:45 UTC - Claude Code (Claude Opus 5.5) - Addressed the findings of the two review rounds on PR 2859.
  Ignored nine more tests that rebind a released port, rewrote the dead-address test and two
  health-check contract tests, and restated the bug-fix sections. The first 150-run recheck found
  the health-check self-collision (2 failures); the second, on the head ending in `fix(tests): [#2393] start the health-check API before stopping the probed service`, passed 150 of 150
  runs. Eighteen ignored tests await the maintainer's decision (T5).
- 2026-11-14 11:45 UTC - Claude Code (Claude Opus 5.5) - Addressed the third review round on PR 2859: ignored
  the three health-check contract tests that probe a released port, let the launcher test's main
  contract run again with its socket-release assertion moved to an ignored companion, counted three
  racing shapes, and replaced PR-branch commit ids with commit subjects. Twenty-one ignored tests
  await the maintainer's decision (T5); four tests are rewritten. Check 3 in V4: 20 full-suite runs
  passed.
- 2026-11-15 07:50 UTC - Claude Code (Claude Opus 5.5) - PR 2859 merged. The maintainer decided T5:
  delete the nine test-support release tests, rewrite the three health-check contract tests on an
  address the test holds, and restore the nine tracker release tests on a unique loopback address,
  through a new loopback helper he approved. The EPIC coordinator agreed T5 stays under #2393.
  Recorded the decisions and the T7 to T11 plan in this spec-only change; the implementation starts
  after it merges.
- 2026-11-15 08:22 UTC - Claude Code (Claude Opus 5.5) - Addressed the first review round on PR
  2863 (five da2ce7 findings, one Copilot finding): qualified the Goal, narrowed the deletion
  premise, corrected the non-Linux reason, scoped the T10 check to Rust sources, and added the
  `write-unit-test` design review per test increment (V7, M7, AC7) with a maintainer stop after the
  final increment.
- 2026-11-15 12:40 UTC - Claude Code (Claude Opus 5.5) - Implemented T7 to T9 after PR 2863 merged:
  deleted the nine test-support release tests, rewrote the three health-check contract tests on an
  address the test holds, added `random::loopback_ipv4`, and restored the nine release tests on a
  random loopback address (ignored off Linux). V5 and V7 recorded per increment. The first UDP
  mutation found an unrelated panic in the UDP health check, now #2867.
- 2026-11-16 08:18 UTC - Claude Code (Claude Opus 5.5) - The maintainer reviewed the T8 and T9
  tests. T10: 150 of 150 full-suite runs passed with no #2393 test skipped (V6), and the source
  check finds `see #2393` only in the non-Linux `cfg_attr`. Recorded the completion review; T11
  waits for the implementation PR to merge.
- 2026-11-16 08:55 UTC - Claude Code (Claude Opus 5.5) - Addressed da2ce7's first review of PR 2874
  (approved, six non-blocking findings): corrected the guessed times of the post-rebase run and of
  this file's previous log entry, recorded repeated green runs for T8 and the helper, recorded the
  maintainer's review of the final increment in V7, ticked the last acceptance criterion with its
  evidence, restored the no-retries warning in the HTTP tracker test, and gave the contract tests
  one registration path.
- 2026-11-16 13:54 UTC - Claude Code (Claude Opus 5.5) - PR 2874 merged and #2393 is closed. T11:
  archived this spec to `docs/issues/closed/` and set order 1 to DONE in the EPIC.

## Acceptance Criteria

- [x] AC1: Every test with a racing shape (bind, drop, reuse; release, then rebind; or a released
  port handed back within one test) is either
  rewritten without a racing port or disabled with `#[ignore = "flaky port race, see #2393"]`.
- [x] AC2: Every ignored test has a recorded decision in the per-test table: deleted, rewritten, or
  kept disabled with a reason.
- [x] AC3: The full parallel suite passes repeatedly with no port-allocation failure (V4, 150 of
  150 runs on the head ending in `fix(tests): [#2393] start the health-check API before stopping the probed service`).
- [x] AC4: No test is ignored for #2393 on Linux: the nine test-support tests are deleted, and the
  other twelve run (the nine restored ones are ignored only on other systems).
- [x] AC5: Each restored or rewritten T5 test fails when the cleanup or contract it guards is
  removed, and passes when it is restored (V5).
- [x] AC6: The full parallel suite passes 150 of 150 runs with the restored tests running (V6).
- [x] AC7: Every test-producing increment of T8 and T9 has a recorded prose-first
  Arrange-Act-Assert design review, and the maintainer reviewed the final increment (V7, "Maintainer
  review of the final increment").
- [x] `linter all` exits with code `0`.
- [x] Relevant focused tests and pre-push checks pass: the focused runs of V5, the pre-commit hook on
  every commit, and the repository's pre-push hook on each push of the branch on 2026-11-16, which
  `git-bot-push` runs and which blocks the push when it fails.

## Verification Plan

- `cargo nextest run --tests --all-features --no-fail-fast`, repeated, with the ignored tests
  listed in the run summary.
- `linter all` and the pre-push checks.
- `manual-verification-evidence.md`: V1 (original reproduction) and V4 (150-run recheck after the
  change); for T5, V5 (red-then-green per restored or rewritten test), V6 (150-run recheck with
  the restored tests running) and V7 (test-design review per increment).

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`, `NOT_APPLICABLE`.

| ID | Scenario | Command | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Original failure reproduction | Run the full suite repeatedly with `--no-fail-fast`. | The race is observed. | DONE | `manual-verification-evidence.md#v1---original-parallel-suite-reproduction` |
| M2 | Duplicate-registration repair | None: the test is ignored, not repaired. | Not applicable. | NOT_APPLICABLE | `manual-verification-evidence.md#v2---duplicate-registration-repair` |
| M3 | CLI configuration repair | None: #2581 already moved these tests below the ephemeral range. | Not applicable. | NOT_APPLICABLE | `manual-verification-evidence.md#v3---cli-configuration-repair` |
| M4 | Full recheck | Run the full suite 150 times with `--no-fail-fast`. | Every run passes, with the ignored tests skipped. | DONE | `manual-verification-evidence.md#v4---full-recheck` |
| M5 | T5 red-then-green | For each restored or rewritten test: remove the guarded cleanup (or make the probed address answer), run it, restore, run it again. | Fails without the cleanup, passes with it. | DONE | `manual-verification-evidence.md#v5---t5-red-then-green` |
| M6 | T5 full recheck | Run the full suite 150 times with `--no-fail-fast`. | Every run passes, and no #2393 test is skipped. | DONE | `manual-verification-evidence.md#v6---t5-full-recheck` |
| M7 | T5 test-design reviews | After each passing test-producing increment of T8 and T9, compare the test with its prose-first Arrange-Act-Assert description. | Each increment has a recorded review before the next starts, and the maintainer reviews the final increment. | DONE | `manual-verification-evidence.md#v7---test-design-review-per-increment` |

### Acceptance Verification

Re-review every acceptance criterion against the recorded evidence after the recheck. Do not mark
AC3 done from a focused test or a single run.

## Risks and Trade-offs

- 150 green runs give about 95% confidence that the `axum-http-server` failure rate observed in V1
  (1 in 50) is gone; fewer runs would be weak evidence.
- Ignored tests are lost coverage of the socket-release contract until T9 restores it.
- The unique loopback address does not protect against a socket bound to `0.0.0.0` on the same
  port. V6 measures the remaining failure rate; if it is not zero, the affected test goes back to
  ignored with that reason, and the maintainer decides again.
- The nine restored tests do not run on macOS or Windows. CI only builds there, so CI coverage is
  unchanged; a developer on those systems does not run them locally.
- Deleting the nine environment tests removes the check that the `testing` environments free their
  ports. That matters only if a test starts depending on a fixed port again.
- Repeated parallel runs are probabilistic evidence.
- The local machine has 32 logical processors, so its nextest scheduling may differ from CI.

## Implementation Completion Review

After T10, compare the result with this specification. Record which ignored tests were deleted,
rewritten or kept disabled, any tests intentionally left unchanged, and reusable lessons. Create `implementation-retrospective.md` only if the work changes repository-wide test
networking policy; otherwise add a concise progress-log entry explaining why no separate
retrospective is needed.

### Result

- Matches the specification: the nine test-support release tests are deleted (T7), the three
  health-check contract tests are rewritten on an address the test holds (T8), and the nine
  tracker release tests are restored on a random loopback address (T9). No test stays disabled for
  #2393 on Linux; the nine restored tests and the helper's own test are ignored on other systems.
- Deviations: the three contract tests are renamed for the state they set up, since they no longer
  stop a service; and the UDP launcher's shared fixture now takes its bind address per scenario,
  so only the restored test uses the random loopback address.
- Left unchanged on purpose: the long inline Arrange of the REST API registration-failure test,
  which T9 only moved to the new address.
- Lessons: a test that proves a release by rebinding cannot hold its address, but it can choose one
  no other test picks; a test that only needs a failing address should hold it for the whole test.
  A mutation that sends junk bytes found a real panic (#2867), so red-then-green also tests the code
  under test, not only the test.
- No separate `implementation-retrospective.md`: the random loopback address is a test helper for
  this family of tests, not a repository-wide networking policy.

## References

- Issue #2298 manual verification follow-up:
  `docs/issues/closed/2298-rust-dev-tool-container-integration/manual-verification-evidence.md`
- `axum-http-server` duplicate-registration tests: `packages/axum-http-server/src/server.rs`
- CLI configuration failure: `tests/configuration/cli_configuration/base_source_precedence.rs`
- Test fixture refactoring patterns: `docs/testing/refactoring-patterns/scenario-fixtures-for-causal-initial-state.md`
- Parallel main-application integration tests: #1419
- E2E fixed host ports as a self-hosted runner concurrency limit: #2386
- Native-process port `0` discovery precedent: #2132 `native-shutdown-test-plan.md`
