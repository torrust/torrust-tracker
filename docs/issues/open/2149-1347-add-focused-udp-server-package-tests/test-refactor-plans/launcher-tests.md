---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/server/launcher.rs
status: proposed
semantic-links:
  related-artifacts:
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/src/server/processor.rs
    - packages/udp-server/src/server/request_buffer.rs
    - packages/udp-server/tests/server/contract.rs
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/issues/drafts/1488-si-14-migrate-udp-receive-reset-token-lifecycle/ISSUE.md
    - docs/issues/drafts/1488-si-15-define-udp-active-request-policy/ISSUE.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/ISSUE.md
---

# UDP Launcher Test Refactor Plan

Follow the shared [purpose, quality goals, plan structure, and required two-phase
sequence](README.md). This plan applies only to `packages/udp-server/src/server/launcher.rs`.

## Phase 1 - Clean Current Tests

### Current state

`launcher.rs` has one direct test: the startup-notification receiver is dropped, so
`run_with_graceful_shutdown` must return `BrokenPipe` and release its socket. The test protects a
valuable failure cleanup contract, but its Arrange block manually composes configuration, clocks,
logging, UDP-core services, server services, a bound socket, and two oneshot channels. The causal
state—the startup receiver is absent—is difficult to see among ordinary infrastructure.

The separate coverage reports at commit `81f5edbc` show 72/135 lines (53.33%), 83/144 regions
(57.64%), and 7/13 functions (53.85%) for unit-only `--lib`; integration-only execution gives
68/91 lines (74.73%), 46/75 regions (61.33%), and 9/11 functions (81.82%) for its smaller
production-only slice. Neither report identifies an uncovered source line through the generic
line-entry data, so the measurements are navigation evidence, not a reason to force tests into
lifecycle-owned branches.

### Decision

Start with a mandatory prose-first Arrange-Act-Assert comparison of the existing test. Its temporary
prose must distinguish ordinary valid launcher dependencies from the causal dropped startup receiver
and the independently observed socket address. Refactor only to make those concepts visible. A
focused scenario fixture may own ordinary launcher construction and the dropped receiver condition,
but it must not run the launcher, receive its outcome, or assert socket release.

## Phase 2 - Add Missing Behavior Tests

### Strengths to preserve

1. `run_with_graceful_shutdown` owns startup notification and releases the listener when startup
   reporting fails.
2. `should_discard_request` owns deterministic pre-processing admission decisions for source port
   zero and currently banned source IPs.
3. `server/processor.rs` already protects source-port-zero defense in depth, while
   `statistics/event/handler` modules own the corresponding counter effects.
4. The #1488 shutdown EPIC and SI-14/SI-15 own receive-loop cancellation, child-task joining,
   request-abort behavior, and active-request shutdown policy.

### Problems and opportunities

#### P1 - Startup-receiver failure setup is harder to read than the contract

**Problem.** The one existing test makes readers reconstruct the causal dropped-receiver state from
the last lines of a long setup sequence.

**Opportunity.** Apply the Phase 1 prose-first refactor before considering any behavior additions.

#### P2 - Admission decisions may have direct deterministic unit seams

**Problem.** The source-port-zero and banned-IP paths are package-owned decisions before processing,
but direct evidence at this boundary is limited.

**Opportunity.** After Phase 1, assess one direct `should_discard_request` contract at a time only
if it can observe the Boolean admission decision and its immediate event without starting a receive
loop, spawning request tasks, using sleeps/polling, or duplicating processor/statistics tests.

#### P3 - Lifecycle and active-request behavior is not owned by this issue

**Decision.** Do not add tests for receive-loop completion, `None`/I/O receiver outcomes, spawned
request-task lifecycle, shutdown aborts, task joining, or active-request eviction. These are owned
by #1488 SI-14 and SI-15 and require their approved cancellation and deadline policy.

## Proposed Refactorings

Apply items in order. Complete one approved increment—including prose-first comparison, focused
validation, review, and its mapped commit point—before beginning the next item.

### R1 - Express startup-receiver failure causally

- **Status:** DONE
- **Priority:** High impact / low effort
- **Addresses:** P1
- **Change:** Write temporary prose for the existing test's Arrange, Act, and Assert sections. Then
  refactor its setup until the code visibly states a valid launcher with a dropped startup receiver,
  the `run_with_graceful_shutdown` Act, and the independent `BrokenPipe`/rebind assertions.
- **Guardrails:** Keep the launcher call and both observable assertions in the test body. Do not
  generalize a fixture for future shutdown cases or change production lifecycle behavior.
- **Prose-first review:** The temporary Arrange prose was “a valid UDP launcher has a dropped
  startup-notification receiver.” `UdpLauncherDependencies::new()` now names ordinary valid
  construction, while the test visibly creates and drops only the startup receiver. The Act remains
  the direct `run_with_graceful_shutdown` call with strict validation, and the Assert retains
  independent `BrokenPipe` and rebind results. The temporary prose is redundant and removed.
- **Done when:** redundant prose can be removed because names and structure express the causal
  state and contract.

### R2 - Assess source-port-zero admission at the launcher boundary

- **Status:** DONE
- **Priority:** Medium impact / low effort
- **Addresses:** P2
- **Change:** Determine whether a direct test can call `should_discard_request` with a source-port-
  zero raw request and observe only its Boolean decision plus immediate `UdpRequestDiscarded` fact.
  Add one unit test only if it adds a clearer contract than `Processor::process_request` and the
  existing statistics handler tests.
- **Guardrails:** Do not start `run_udp_server_main`, receive real UDP traffic, spawn tasks, use a
  listener, sleep, poll, or assert later counter consumption. Do not test source-port-zero wire
  transport, which standard sockets cannot produce.
- **Prose-first review:** The temporary Arrange prose was “a valid launcher evaluates a request
  whose source port is zero.” The final code makes the port-zero client address and raw request
  visible, while `UdpLauncherDependencies`, `sample_udp_service_binding`, and `TEST_LOG_TARGET`
  own ordinary setup. The direct `should_discard_request` Act and strict-policy input remain
  visible. The Assert independently specifies both discard decision and exact immediate event;
  only the event-await comment remains because its deadline failure-bound rationale is not evident
  from syntax alone. The temporary prose is redundant and removed.
- **Done when:** the admission seam has either one unique direct contract or a documented
  no-change decision assigning it to processor/statistics boundaries.

### R3 - Assess banned-IP admission at the launcher boundary

- **Status:** DONE
- **Priority:** Medium impact / low effort
- **Addresses:** P2, P3
- **Change:** Determine whether one deterministic unit test can seed a banned IP, call
  `should_discard_request`, and assert only the Boolean decision plus immediate `UdpRequestBanned`
  fact. Add it only if it does not duplicate ban-service policy or listener counter behavior.
- **Guardrails:** Keep validation-policy choice visible. Do not cover ban threshold accumulation,
  receive-loop lifecycle, or disabled-mode tracker behavior unless the direct admission choice is
  uniquely obscured elsewhere.
- **Prose-first review:** The temporary Arrange prose was “a strict launcher receives a nonzero-
  port request from an already-banned IP.” The final `ban_client_ip` setup operation expresses the
  causal state while deriving its counter increments from the configured threshold; it does not
  assert or test the UDP-core ban algorithm. The direct strict-policy admission Act and the
  independent discard/exact-event assertions remain visible. The shared event-publication deadline
  retains its concise failure-bound rationale; the temporary prose is redundant and removed.
- **Done when:** the strict-mode admission choice has a unique direct contract or a documented
  no-change ownership decision.

### R3a - Split admission decision and event contracts

- **Status:** DONE
- **Priority:** High impact / low effort
- **Addresses:** R2/R3 assertion specificity
- **Change:** Refactor each admission condition into two tests with one observable reason to fail:
  one asserts only `should_discard_request`'s Boolean decision; the other asserts only its immediate
  event. Implement and review the source-port-zero pair first. Assess the already-banned-IP pair
  only after that review.
- **Guardrails:** Each test must retain the direct `should_discard_request` Act. Decision tests do
  not subscribe to or assert events. Event tests do not assert the Boolean decision. Preserve direct
  event-bus observation and its absolute deadline in event tests. Do not change production behavior,
  start a receive loop, or duplicate processor/statistics behavior.
- **Prose-first review:** The source-port-zero and banned-IP tests initially combined their decision
  and event assertions, giving each two unrelated failure causes. The final four test names state
  either `require_discarding` or `publish` and retain one assertion accordingly. Each Arrange keeps
  its causal source-port-zero or `with_banned_client_ip` state visible; each Act is the direct
  `should_discard_request` call. Event tests retain only the bounded direct event receive, while
  decision tests do not subscribe. The temporary prose is redundant and removed.
- **Done when:** a failing decision assertion identifies admission-policy behavior, and a failing
  event assertion identifies immediate observability behavior without conflating the two.

### R4 - Review design and residual test-level coverage

- **Status:** TODO
- **Priority:** Low impact / low effort
- **Change:** After each approved test, apply and record prose-first AAA verification. Measure
  unit-only and integration-only coverage separately; assign every remaining relevant branch to the
  launcher, processor, request buffer, integration contract, or #1488 shutdown work.
- **Guardrails:** Do not use combined coverage to claim either boundary, and do not add
  percentage-only tests.
- **Done when:** remaining lifecycle-sensitive gaps have explicit ownership and all approved tests
  are readable, deterministic, and unit-first where appropriate.

## Progress Tracking

### Plan Checklist

- [x] Existing launcher test, admission branches, separate coverage, and #1488 ownership reviewed.
- [x] Maintainer approved R1.
- [x] R1 implemented, reviewed, validated, and committed.
- [x] Maintainer approved R2.
- [x] R2 implemented, reviewed, validated, and committed.
- [x] Maintainer approved R3.
- [x] R3 implemented, reviewed, validated, and committed.
- [x] Maintainer approved R3a source-port-zero split.
- [x] R3a source-port-zero split implemented, reviewed, validated, and committed.
- [x] Maintainer approved R3a banned-IP split.
- [x] R3a banned-IP split implemented, reviewed, validated, and committed.
- [ ] R4 design/coverage review completed and decision recorded.
- [ ] Maintainer reviewed all approved changes.
- [ ] Plan completed and ready for final verification.

### Progress Log

- 2026-09-09 - GitHub Copilot - Created this proposed two-phase plan after reviewing the existing
  launcher test, `should_discard_request`, processor and request-buffer boundaries, separate
  unit-only/integration-only coverage, and #1488 shutdown ownership. No test or production change
  has been made.
- 2026-09-09 - User/maintainer - Reviewed and approved R1. The final test retains the direct
  launcher Act and observable failure/rebind assertions, while `UdpLauncherDependencies` owns only
  ordinary construction and the visible dropped startup receiver identifies the causal state.
- 2026-09-09 - User/maintainer - Approved R2. Add one direct unit test proving the launcher
  rejects a source-port-zero raw request and immediately emits `UdpRequestDiscarded`, without
  starting the receive loop, spawning a processor, or asserting later metrics consumption.
- 2026-09-09 - User/maintainer - Reviewed and approved R2. The unit-first direct admission test
  makes the source-port-zero causal state, strict policy, dispatcher-independent Act, and exact
  immediate event visible; helpers hide only tracing/server metadata and ordinary dependencies.
- 2026-09-09 - User/maintainer - Approved R3. Add one direct strict-mode unit test for an
  already-banned nonzero-port client IP. Seed the existing ban service past its configured limit,
  then assert only the launcher discard decision and exact immediate `UdpRequestBanned` event.
- 2026-09-09 - User/maintainer - Reviewed and approved R3. The named banned-client setup, visible
  strict-policy admission Act, and exact immediate event retain the launcher boundary without
  duplicating UDP-core threshold behavior or integration-level network handling.
- 2026-09-09 - User/maintainer - Identified that the R2/R3 tests assert both admission decision
  and event publication, giving each test two unrelated reasons to fail. R3a splits each condition
  into a decision contract and an immediate-event contract, starting with the source-port-zero pair.
- 2026-09-09 - User/maintainer - Approved the R3a source-port-zero split. Commit this plan update
  before replacing the combined test with separate decision and event contracts.
- 2026-09-09 - User/maintainer - Reviewed and approved the source-port-zero split: one test asserts
  only the discard decision and the other only the immediate discard event. Also approved applying
  the same single-fact split to the banned-IP admission test.
- 2026-09-09 - User/maintainer - Reviewed and approved R3a. Both admission conditions now have a
  decision-only contract and an event-only contract, preserving one behavioral reason to fail per
  test without receive-loop, processor, or metrics-listener setup.
- 2026-09-09 - User/maintainer - Approved the final naming refinement. The test context is named
  `UdpLauncherTestContext`, its variable is `launcher`, and
  `with_banned_client_ip(client_socket_addr.ip())` states the causal already-banned-client state
  directly in Arrange. The shared event-publication deadline records its failure-bound rationale.

### Validation Evidence

| Increment | Status | Evidence |
| --- | --- | --- |
| Plan documentation | TODO | Run Markdown and spelling checks after maintainer review changes. |
| R1 | DONE | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server launcher::tests::it_should_release_the_socket_when_the_startup_notification_receiver_is_dropped`, and `git diff --check` passed. The prose-first review separates ordinary launcher construction from the visible dropped receiver state. |
| R2 | DONE | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server launcher::tests::it_should_discard_a_request_when_its_source_port_is_zero`, and `git diff --check` passed. The prose-first review retains the direct admission Act, causal source port, strict policy, and exact immediate event. |
| R3 | DONE | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server launcher::tests::it_should_discard_a_request`, and `git diff --check` passed. The prose-first review uses `UdpLauncherTestContext::with_banned_client_ip` to keep the causal state, strict Act, and independent discard/event assertions visible. |
| R3a | DONE | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server launcher::tests`, and `git diff --check` passed. Source-port-zero and banned-IP behavior are each split into one decision-only and one event-only test after prose-first review. |
| R4 | TODO | Awaiting approved increments. |

## Non-Goals

- Do not change UDP launcher, admission, request-buffer, processor, or shutdown production behavior.
- Do not test receive-loop termination, task lifecycle, cancellation, joining, request draining, or
  active-request shutdown policy owned by #1488 SI-14/SI-15.
- Do not add a real-loopback integration test when a deterministic unit contract can express the
  selected behavior more directly.
- Do not duplicate ban-service policy, processor source-port-zero defense, event-listener counter
  consumption, or protocol transport constraints.

## Validation Per Approved Increment

- Apply the mandatory prose-first Arrange-Act-Assert comparison before maintainer review.
- Run focused launcher tests.
- Run `cargo fmt --all -- --check` and `git diff --check`.
- Run `linter markdown` and `linter cspell` when this plan changes.
- Record unit-only and integration-only coverage separately when coverage informs a decision.

## Completion Criteria

- The existing startup-receiver failure test expresses its causal state without obscuring the Act or
  independent assertions.
- Any new admission test is deterministic, unit-first, and protects a unique immediate launcher
  decision.
- Lifecycle-sensitive gaps remain assigned to #1488 until its cancellation and active-request
  policies are implemented.
- The maintainer reviews every approved increment before the next increment and before final
  verification.
