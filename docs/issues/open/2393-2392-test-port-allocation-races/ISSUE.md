---
schema-version: 1
doc-type: issue
issue-type: bug
status: in-progress
priority: p2
epic: 2392
github-issue: 2393
spec-path: docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md
branch: "2392-test-isolation-spec"
related-pr: 2397
last-updated-utc: "2026-11-14 10:45"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - write-unit-test
  related-artifacts:
    - docs/issues/open/2392-test-isolation/EPIC.md
    - docs/issues/open/2395-2392-e2e-runner-dynamic-host-ports/ISSUE.md
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
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

### Out of Scope

- A server that takes over an already-bound socket (listener handoff). It was considered and
  rejected for now: it needs a production API change on four server crates.
- Any new or extended shared port helper. The #2581 helper (`reserve_free_ports`) stays as it is.
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
| `it_should_release_the_listener_when_token_aware_startup_registration_fails` | `axum-http-server` `server.rs` | Ignored, needs maintainer decision | Needs a pre-known address for the registry and a rebind to prove release. |
| `it_should_release_the_listener_when_token_aware_startup_registration_fails` | `axum-rest-api-server` `server.rs` | Ignored, needs maintainer decision | Same shape. |
| `it_should_release_the_listener_when_token_aware_startup_registration_fails` | `axum-health-check-api-server` `server.rs` | Ignored, needs maintainer decision | Same shape. |
| `it_should_serve_requests_after_being_stopped_and_started_again_on_a_fixed_port` | `axum-health-check-api-server` `environment.rs` | Ignored, needs maintainer decision | Restart on one address needs a known port. |
| `it_should_stop_the_receive_loop_and_release_the_socket_when_its_cancellation_token_is_cancelled` | `udp-server` `server/mod.rs` | Ignored, needs maintainer decision | Rebinds the released socket. |
| `it_should_release_the_socket_when_registration_fails` | `udp-server` `server/mod.rs` | Ignored, needs maintainer decision | Needs a pre-known address and a rebind. |
| `it_should_release_the_socket_when_the_component_is_dropped_before_it_runs` | `src/bootstrap/jobs/udp_tracker.rs` | Ignored, needs maintainer decision | Rebinds the released socket. |
| `it_should_release_the_listener_when_the_component_is_dropped_before_it_runs` | `src/bootstrap/jobs/health_check_api.rs` | Ignored, needs maintainer decision | Rebinds the released listener. |
| `it_should_release_udp_listener_before_returning_from_start_after_setup_when_later_http_startup_fails` | `src/app.rs` | Ignored, needs maintainer decision | Rebinds the released UDP socket. |
| `it_should_release_the_health_check_api_binding_when_stopped` | `axum-health-check-api-server` `environment.rs` | Ignored, needs maintainer decision | Rebinds the released ephemeral port. |
| `it_should_release_the_health_check_api_binding_when_dropped_without_being_stopped` | `axum-health-check-api-server` `environment.rs` | Ignored, needs maintainer decision | Waits until the released port can be rebound. |
| `it_should_release_the_http_binding_when_stopped` | `axum-http-server` `testing/environment.rs` | Ignored, needs maintainer decision | Rebinds the released ephemeral port. |
| `it_should_release_the_http_binding_when_dropped_without_being_stopped` | `axum-http-server` `testing/environment.rs` | Ignored, needs maintainer decision | Waits until the released port can be rebound. |
| `it_should_release_the_rest_api_binding_when_stopped` | `axum-rest-api-server` `testing/environment.rs` | Ignored, needs maintainer decision | Rebinds the released ephemeral port. |
| `it_should_release_the_rest_api_binding_when_dropped_without_being_stopped` | `axum-rest-api-server` `testing/environment.rs` | Ignored, needs maintainer decision | Waits until the released port can be rebound. |
| `it_should_release_the_udp_socket_when_stopped` | `udp-server` `testing/environment.rs` | Ignored, needs maintainer decision | Rebinds the released ephemeral port. |
| `it_should_release_the_udp_socket_when_dropped_without_being_stopped` | `udp-server` `testing/environment.rs` | Ignored, needs maintainer decision | Waits until the released port can be rebound. |
| `it_should_not_return_while_processors_orphaned_by_the_request_ring_are_still_running` | `udp-server` `server/launcher.rs` | Rewritten | Its main assertion (processors joined before the loop returns) needs no port, so it runs again; its socket-release assertion moved to the companion below. |
| `it_should_release_the_socket_once_the_processors_orphaned_by_the_request_ring_are_joined` | `udp-server` `server/launcher.rs` | Ignored, needs maintainer decision | New companion holding the socket-release assertion, which waits until the released port can be rebound. |
| `it_should_return_error_when_api_service_was_stopped_after_registration` | `axum-health-check-api-server` `tests/server/contract.rs` | Ignored, needs maintainer decision | It now starts the health-check API before stopping the REST API, so the two never share a port, but it still expects the released REST API port to be dead, and another test can bind that port and answer. |
| `it_should_return_error_when_http_service_was_stopped_after_registration` | `axum-health-check-api-server` `tests/server/contract.rs` | Ignored, needs maintainer decision | Same shape and reorder as the REST API variant; it still expects the released HTTP port to be dead. |
| `it_should_return_error_when_udp_service_was_stopped_after_registration` | `axum-health-check-api-server` `tests/server/contract.rs` | Ignored, needs maintainer decision | It cannot collide with the health-check API (a UDP and an HTTP binding on one port are different registrations), but it expects the released UDP port to be dead. |
| `it_should_register_the_service_with_a_health_check_that_reaches_the_running_server` | `udp-server` `server/mod.rs` | Rewritten | Binds `127.0.0.1:0` and reads the bound address; it never needed a pre-selected port. |
| `it_should_cancel_the_udp_tracker_component_through_the_job_manager` | `src/app.rs` | Rewritten | Uses the configured port `0`; it never needed a reserved address. |
| `it_should_report_an_error_with_the_checked_url_when_nothing_listens_on_the_checked_address` | `axum-rest-api-server` `server.rs` | Rewritten | Checks the address of a socket the test binds to port `0` and never listens on, so nothing else can listen there. |

No test was deleted: none of them only checks the operating system. Eighteen of the twenty-one ignored
tests prove that the tracker releases its socket after a stop, a drop or a failure, which is the
tracker's own cleanup contract. The other three prove that the health-check report shows a stopped
service as failing, which needs a port nothing can listen on. Each needs a real bound port, so it
can only be rewritten once a decision is made.

## Architectural Decisions

- Related ADRs: none. This patch adds no shared abstraction and changes no production API.
- A test that races on a port is disabled first. A flaky test is worse for development than a missing
  one, and some tests that need a fixed or pre-selected port may never have been worth having.
- The listener-handoff design (the server takes over an already-bound socket) was considered and
  rejected for now. It stays available if the maintainer later wants the ignored tests restored.

## Bug-Fix Process

The defect is in the tests, not in tracker behavior. This patch disables the racing tests instead of
repairing them, so the red-then-green steps of `.github/skills/dev/debugging/fix-bug/SKILL.md` do
not apply to it: a disabled test has no regression boundary to prove red. The steps that still
apply:

1. Reproduce against a real artifact and record it (V1).
2. Inventory every test with a racing shape and record its decision (this spec).
3. Recheck the full parallel suite after the change, sized for the claim it supports (V4).

If the maintainer restores an ignored test in T5, that test follows the full `fix-bug` process.

## Regression Test Strategy

No regression test is added. The ignored tests are the racing code; disabling them removes the
race rather than fixing it. The guard against recurrence is the 150-run full-suite recheck (V4):
at the V1 rate of 1 failure in 50 runs on the `axum-http-server` assertion, 150 green runs would
happen with no change at a probability of about 0.05.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Reproduce and inventory the observed failures | Evidence records the 200 isolated, 100 package, and 51 full-suite runs. |
| T2 | DONE | Re-check the inventory on `develop` | The inventory above, including the stale CLI finding and the three racing shapes. |
| T3 | DONE | Disable each racing test with `#[ignore]` | Twenty-one tests ignored with the reason `flaky port race, see #2393`. |
| T4 | DONE | Rewrite the tests that never needed a racing port | Four tests rewritten. |
| T5 | BLOCKED | Decide the fate of the twenty-one ignored tests | Waiting for the maintainer. |
| T6 | DONE | Full-suite recheck | V4 in `manual-verification-evidence.md`: the first 150 runs found a third racing shape (2 failures), and 150 runs on the fixed head all passed. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 to T4 | Ignored and rewritten tests, with their rows in this spec | One commit per racing shape or rewrite. |
| T5 | Each decided test | One commit per decided test or small cohesive family. |
| T6 | Recheck evidence | Documentation-only commit. |

## Progress Tracking

### Workflow Checkpoints

- [x] Initial real-artifact reproduction recorded.
- [x] Draft reviewed and approved by the maintainer.
- [x] GitHub issue created and issue number added to this specification.
- [x] Racing tests inventoried, ignored or rewritten, and focused checks pass.
- [x] Full parallel recheck recorded (V4).
- [ ] Fate of every ignored test decided by the maintainer (T5).
- [ ] Acceptance criteria reviewed after implementation.
- [ ] Implementation completion review recorded.

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
  the health-check self-collision (2 failures); the second, on head `363c30ac2`, passed 150 of 150
  runs. Eighteen ignored tests await the maintainer's decision (T5).

## Acceptance Criteria

- [ ] AC1: Every test with a racing shape (bind, drop, reuse; release, then rebind; or a released
  port handed back within one test) is either
  rewritten without a racing port or disabled with `#[ignore = "flaky port race, see #2393"]`.
- [ ] AC2: Every ignored test has a recorded decision in the per-test table: deleted, rewritten, or
  kept disabled with a reason.
- [x] AC3: The full parallel suite passes repeatedly with no port-allocation failure (V4, 150 of
  150 runs on head `363c30ac2`).
- [ ] `linter all` exits with code `0`.
- [ ] Relevant focused tests and pre-push checks pass.

## Verification Plan

- `cargo nextest run --tests --all-features --no-fail-fast`, repeated, with the ignored tests
  listed in the run summary.
- `linter all` and the pre-push checks.
- `manual-verification-evidence.md`: V1 (original reproduction) and V4 (150-run recheck after the
  change).

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`, `NOT_APPLICABLE`.

| ID | Scenario | Command | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Original failure reproduction | Run the full suite repeatedly with `--no-fail-fast`. | The race is observed. | DONE | `manual-verification-evidence.md#v1---original-parallel-suite-reproduction` |
| M2 | Duplicate-registration repair | None: the test is ignored, not repaired. | Not applicable. | NOT_APPLICABLE | `manual-verification-evidence.md#v2---duplicate-registration-repair` |
| M3 | CLI configuration repair | None: #2581 already moved these tests below the ephemeral range. | Not applicable. | NOT_APPLICABLE | `manual-verification-evidence.md#v3---cli-configuration-repair` |
| M4 | Full recheck | Run the full suite 150 times with `--no-fail-fast`. | Every run passes, with the ignored tests skipped. | DONE | `manual-verification-evidence.md#v4---full-recheck` |

### Acceptance Verification

Re-review every acceptance criterion against the recorded evidence after the recheck. Do not mark
AC3 done from a focused test or a single run.

## Risks and Trade-offs

- 150 green runs give about 95% confidence that the `axum-http-server` failure rate observed in V1
  (1 in 50) is gone; fewer runs would be weak evidence.
- Ignored tests are lost coverage of the socket-release contract until the maintainer decides how
  to restore it.
- Repeated parallel runs are probabilistic evidence.
- The local machine has 32 logical processors, so its nextest scheduling may differ from CI.

## Implementation Completion Review

After T5, compare the result with this specification. Record which ignored tests were deleted,
rewritten or kept disabled, any tests intentionally left unchanged, and reusable lessons. Create `implementation-retrospective.md` only if the work changes repository-wide test
networking policy; otherwise add a concise progress-log entry explaining why no separate
retrospective is needed.

## References

- Issue #2298 manual verification follow-up:
  `docs/issues/closed/2298-rust-dev-tool-container-integration/manual-verification-evidence.md`
- `axum-http-server` duplicate-registration tests: `packages/axum-http-server/src/server.rs`
- CLI configuration failure: `tests/configuration/cli_configuration/base_source_precedence.rs`
- Test fixture refactoring patterns: `docs/testing/refactoring-patterns/scenario-fixtures-for-causal-initial-state.md`
- Parallel main-application integration tests: #1419
- E2E fixed host ports as a self-hosted runner concurrency limit: #2386
- Native-process port `0` discovery precedent: #2132 `native-shutdown-test-plan.md`
