---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/server/launcher.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/src/server/states.rs
    - packages/udp-server/tests/server/contract.rs
    - packages/axum-health-check-api-server/tests/server/contract.rs
    - docs/issues/closed/2149-1347-add-focused-udp-server-package-tests/test-refactor-plans/launcher-tests.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/issues/closed/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Launcher File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/server/launcher.rs`.

## Current State

- **Fresh unit-only coverage:** 278 / 290 lines (95.86%), 24 / 26 functions, and 269 / 292
  regions from `cargo llvm-cov -p torrust-tracker-udp-server --all-features --lib --json` (stable
  Rust toolchain).
- **Uncovered:**
  - line 81: end of the spawned receive task; line 245: end of the receive loop;
  - lines 117-125: `Launcher::check` (never called by a unit test);
  - line 303/305: the `false` (admit) exit of `should_discard_request`;
  - line 312: the `publish_event_if_sender_available` path with no sender.
- **Module-owned decisions:**
  1. Release the listener when startup notification fails.
  2. Admission: discard source-port-zero requests; discard banned client IPs only under strict
     connection-ID validation; admit everything else.
  3. Publish the matching `UdpRequestDiscarded` or `UdpRequestBanned` fact.
- **Existing tests:** five, all from #2149: socket release on a dropped startup receiver, and
  one decision test plus one event test for each discard condition.
- **T1 hypothesis:** lifecycle (#1488). Confirmed for the loop and task paths; the admit path is
  a package-owned gap.

## Current Tests Review

The #2149 tests are clean: `it_should_*` names, visible AAA, a named `UdpLauncherTestContext`
scenario (`new`, `with_banned_client_ip`) that keeps the causal state visible, the direct
`should_discard_request` Act, one assertion each, and an absolute event deadline. No change.

Their gap is structural: every admission test expects `true`. If `should_discard_request` always
returned `true`, every test would pass while the tracker dropped all traffic.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Admit an ordinary request | 3A unit | R1 add a decision test: a nonzero source port from a client that is not banned, strict mode, is admitted. |
| Ban not enforced when validation is disabled | 3A unit | R2 add a decision test: a banned client IP with `ConnectionIdValidationPolicy::Disabled` is admitted. The real-listener contract `many_invalid_connection_ids_do_not_cause_ban_in_disabled_mode` covers this at integration level; #2149 R3 deferred it here, but #2283 is unit-first and the decision is a one-token policy gate at this seam. |
| Discard decisions and events | 3A unit | Covered by the existing four tests. |
| Startup-notification failure cleanup | 3A unit | Covered by the existing test. |
| `Launcher::check` | Integration | Runs a real UDP health check; covered by `axum-health-check-api-server` contracts `it_should_return_good_health_for_udp_service` and `..._when_udp_service_was_stopped_after_registration`. Not selected. |
| No-sender publication (line 312) | Not selected | The only effect is that nothing is sent; there is no observable result to assert. |
| Receive task and loop completion (lines 81, 245) | Lifecycle (#1488) | SI-14/SI-15 own receive-loop termination and task joining (#2149 P3 retained). |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Added `it_should_admit_a_request_from_a_client_that_is_not_banned`. | Passes; fails when `should_discard_request` always returns `true`. |
| R2 | DONE | Added `it_should_admit_a_request_from_a_banned_client_when_connection_id_validation_is_disabled`. | Passes; fails when ban enforcement ignores the validation policy, while R1 still passes. |
| R3 | SKIPPED | No integration increment. | Explicit rationale above. |
| R4 | DONE | Re-ran unit-only coverage and recorded before/after file results. | 278 / 290 (95.86%) to 321 / 331 (96.98%) lines; 24 / 26 to 28 / 30 functions; 269 / 292 to 317 / 338 regions. |

## Results

- **Unit-only coverage:** 278 / 290 (95.86%) to 321 / 331 (96.98%) lines; 24 / 26 to 28 / 30
  functions; 269 / 292 to 317 / 338 regions.
- **Tests refactored:** none. **Tests added:** two admit-path decision tests.
- **Residual:** lines 81 and 245 (#1488 lifecycle), 117-125 (`Launcher::check`, integration),
  and 312 (no-sender publication). See Coverage Analysis.
- **Mutation evidence:** each test was run against a hand-applied mutation of the decision it
  guards and the mutation was reverted by hand; before R1 and R2, the always-discard mutation
  passed every existing test.

## Completed-File Review

Approved by Jose Celano on 2026-09-25.

## Post-Commit Readability Plan

Requested by the maintainer after the completed-file commit: re-read the `write-unit-test` skill
and make the tests read as prose, showing only what decides the outcome. Approve the steps below
as a separate launcher increment.

### Prose Specification

1. A request whose source port is zero is discarded.
2. A request from a banned client is discarded under strict connection-ID validation.
3. A request from a client that is not banned is admitted.
4. A request from a banned client is admitted when connection-ID validation is disabled.
5. Discarding a source-port-zero request publishes a "request discarded" fact naming the client.
6. Discarding a banned client's request publishes a "request banned" fact naming the client.
7. When nobody receives the startup notification, the launcher reports a broken pipe.
8. When nobody receives the startup notification, the launcher releases its socket.

### Smell Audit

The only values that decide facts 1-6 are the client address (its port, or whether its IP is
banned) and the validation policy. Everything else is plumbing.

| Smell | Where | Effect |
| --- | --- | --- |
| Wide Act | Six admission tests call `should_discard_request` with six arguments; four (`udp_tracker_core_container`, `udp_tracker_server_container`, service binding, `TEST_LOG_TARGET`) never vary. | ~10 lines per test; the causal policy argument is lost among them. |
| Irrelevant request detail | `RawRequest { payload: Vec::new(), from: .. }` in every admission test. | Payload looks meaningful but admission never reads it. |
| Repeated full paths | `torrust_tracker_udp_core::ConnectionIdValidationPolicy::` seven times; `std::io::ErrorKind::` once. | Line noise; `AGENTS.md` prefers short imported names. |
| Receive plumbing in Assert | `tokio::time::timeout(..).await.expect(..).expect(..)` in both event tests. | Four calls and two messages before the expected fact appears. |
| Fixture-derived expectation built by hand | Event tests rebuild `ConnectionContext::new(instance_id, client, binding)`. | Only the client is causal; instance ID and binding come from the fixture. |
| Inconsistent names | Decision tests say `require_discarding`; the new ones say `admit`. | A reader has to map two vocabularies to one decision. |
| Two behaviors, one name | The startup test asserts the `BrokenPipe` error and the socket release, but its name mentions only the release. | Two reasons to fail; the error-report fact is unnamed. |

### Target Design

Extend the existing `UdpLauncherTestContext` scenario fixture (it already owns container setup and
`with_banned_client_ip`). Following the accepted `handlers/error.rs` and `handlers/scrape.rs`
precedent, add one named action that hides only the arguments no test varies:

- `launcher.should_discard(&request_from(client), policy)`: calls the production
  `should_discard_request` with the fixture's containers, service binding, and log target. The
  policy stays a visible argument because it is causal in facts 2 and 4.
- `launcher.connection_context(client)`: the expected event context, derived from the fixture's
  instance ID and binding with the visible client.
- `launcher.subscribe_to_events()` and `next_published_event(&mut events)`: the event receiver
  and the bounded receive, keeping `EVENT_PUBLICATION_TIMEOUT` as the failure bound.
- `request_from(client)`: a raw request whose only relevant property is its source address.
- `sample_client()` (`203.0.113.1:8080`); the port-zero case stays visible as
  `SocketAddr::new(sample_client().ip(), 0)`.

Target shape for fact 4:

```rust
#[tokio::test]
async fn it_should_admit_a_request_from_a_banned_client_when_connection_id_validation_is_disabled() {
    // Arrange
    let client = sample_client();
    let launcher = UdpLauncherTestContext::with_banned_client_ip(client.ip()).await;

    // Act
    let should_discard = launcher.should_discard(&request_from(client), ConnectionIdValidationPolicy::Disabled).await;

    // Assert
    assert!(!should_discard);
}
```

Target shape for fact 5:

```rust
#[tokio::test]
async fn it_should_publish_a_request_discarded_event_when_its_source_port_is_zero() {
    // Arrange
    let launcher = UdpLauncherTestContext::new().await;
    let client = SocketAddr::new(sample_client().ip(), 0);
    let mut events = launcher.subscribe_to_events();

    // Act
    launcher.should_discard(&request_from(client), ConnectionIdValidationPolicy::Strict).await;

    // Assert
    assert_eq!(
        next_published_event(&mut events).await,
        Event::UdpRequestDiscarded { context: launcher.connection_context(client) }
    );
}
```

The source-port-zero tests keep their one-line comment that the policy is inert, because that fact
is not visible from the code.

### Steps

| ID | Status | Work | Boundary |
| --- | --- | --- | --- |
| L1 | DONE | Imported `ConnectionIdValidationPolicy`, `ErrorKind`, and the event `Receiver` alias; added `sample_client`, `request_from`, `next_published_event`, and the `should_discard`, `connection_context`, and `subscribe_to_events` fixture methods. Rewrote the six admission tests on them. | Behavior-preserving: the same uncovered production lines (81, 117-125, 245, 312) before and after. |
| L2 | DONE | Renamed the decision tests to `it_should_discard_a_request_whose_source_port_is_zero` and `it_should_discard_a_request_from_a_banned_client_in_strict_mode`. | Names only. |
| L3 | DONE | Split the startup test into `it_should_report_a_broken_pipe_..` and `it_should_release_the_socket_..`, one assertion each, with the Arrange inline. | Eight tests pass. |

### Prose-First Review

- **Arrange** now states only the causal state: a client (`sample_client()`, or the same IP on
  port zero) and, where relevant, `with_banned_client_ip(client.ip())`. Containers, binding, log
  target, and payload are fixture mechanics.
- **Act** is one line naming the production decision, `launcher.should_discard(..)`, with the
  validation policy visible because facts 2 and 4 depend on it.
- **Assert** is one semantic assertion: the Boolean decision, or the complete published fact whose
  only causal field, the client, is visible.
- **Retained comments:** the source-port-zero "policy is inert" note (not expressible in code),
  the event deadline rationale, and the `request_from` payload note.
- **Validation:** `cargo test -p torrust-tracker-udp-server launcher::tests` passed eight tests
  (stable Rust toolchain); nightly Rust formatting, Clippy, and `git diff --check` passed. Unit-only
  coverage 288 / 298 lines, 38 / 40 functions, 325 / 346 regions; the lower denominator is shorter
  test code.

## Progress Log

- 2026-09-24 - GitHub Copilot - Created the plan from `launcher.rs`, its five tests, the #2149
  plan, the health-check API contracts, and fresh unit-only coverage. No Rust tests or production
  code changed. R1 and R2 await maintainer approval.
- 2026-09-25 - Jose Celano - Approved R1 and R2.
- 2026-09-25 - GitHub Copilot - Completed R1, R2, and R4. `cargo test -p torrust-tracker-udp-server
  launcher::tests` passed seven tests (stable Rust toolchain); nightly Rust formatting, Clippy, and
  `git diff --check` passed. Completed-file review is requested.
- 2026-09-25 - Jose Celano - Approved the completed-file result.
- 2026-09-25 - Jose Celano - Asked for a prose-first readability review against the
  `write-unit-test` skill before the next file.
- 2026-09-25 - GitHub Copilot - Added the Post-Commit Readability Plan (prose specification, smell
  audit, target design, L1-L3). No Rust changed; L1-L3 await approval.
- 2026-09-25 - Jose Celano - Approved L1-L3, including the named `should_discard` action.
- 2026-09-25 - GitHub Copilot - Completed L1-L3. Refactor review is requested.
