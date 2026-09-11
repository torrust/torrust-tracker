---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/container.rs
status: proposed
semantic-links:
  related-artifacts:
    - packages/udp-server/src/container.rs
    - packages/udp-server/src/event.rs
    - packages/events/src/bus.rs
    - packages/udp-server/src/server/launcher.rs
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/ISSUE.md
---

# UDP Server Container Test Refactor Plan

Follow the shared [purpose, quality goals, plan structure, and required two-phase
sequence](README.md). This plan applies only to `packages/udp-server/src/container.rs`.

## Phase 1 - Clean Current Tests

### Current state

`container.rs` has no direct tests. `UdpTrackerServerServices::initialize` constructs the package
`Broadcaster`, an explicitly enabled server `EventBus`, its optional event sender, and a statistics
repository. `UdpTrackerServerContainer::initialize` exposes cloned handles from those services.

The aggregate package baseline reports `container.rs` as 19/19 lines, 29/29 regions, and 2/2
functions covered, but that global result does not show which test level provides the coverage.
Existing launcher unit tests construct the real container and observe server events, while root
integration tests own multi-listener metrics and banning policy.

### Decision

No cleanup increment is proposed because no local test code exists. Preserve the concise explicit
container composition. Do not treat indirect aggregate or integration coverage as a reason to
skip a feasible focused unit test: the issue's unit-only coverage objective requires this package
composition decision to be assessed at the unit boundary.

## Phase 2 - Add Missing Behavior Tests

### Strengths to preserve

1. `UdpTrackerServerServices::initialize` owns selection of an enabled UDP-server event-publication
   path.
2. The container owns package-local coherence between its `event_bus` and `stats_event_sender`.
3. `packages/events` owns generic enabled/disabled event-bus behavior.
4. Launcher tests own server admission facts, and root integration tests own multi-listener metrics
   filtering and banning outcomes.

### Problems and opportunities

#### P1 - The package-selected enabled publication path has no direct unit contract

**Problem.** Indirect launcher coverage proves that the container is exercised, but a failure does
not isolate the container's own composition decision. Aggregate/global coverage cannot establish
that this package responsibility has focused unit protection.

**Why it matters.** A future change can disable the server event bus, omit its sender, or wire the
sender to a different bus while higher-level tests fail less locally or only under a specific
listener configuration.

**Opportunity.** Add one deterministic asynchronous unit test that initializes
`UdpTrackerServerServices`, creates a receiver from its `event_bus`, publishes one representative
UDP-server event through `stats_event_sender`, and asserts that exact event is received. Use one
absolute timeout solely as a diagnostic failure bound; do not use a delay, socket, spawned server,
or lifecycle fixture.

The event and its `ConnectionContext` are setup mechanics for observing the container-owned
publication path. Keep the sender, receiver, exact event, production publication Act, and received
event assertion visible. Do not derive the expected event through production code.

#### P2 - Multi-listener policy is not a container-unit responsibility

**Decision.** Do not test metrics-enabled/disabled listener policy, root statistics aggregation,
REST exposure, or banning outcomes here. Those require root composition and retain their existing
higher-level ownership. This direct unit test protects the package's unconditional event
publication, not a consumer's policy.

#### P3 - Generic event-bus variants are not this package's responsibility

**Decision.** Do not add a disabled-sender test or a generic `EventBus` matrix. The events package
owns that implementation. This package needs one contract proving its explicit selection of the
enabled mode is observable through its own composed services.

## Proposed Refactorings

Apply items in order. Complete one approved increment—including prose-first comparison, focused
validation, review, and its mapped commit point—before beginning the next item.

### R1 - Record the Phase 1 no-change decision

- **Status:** DONE
- **Priority:** High impact / trivial effort
- **Addresses:** Phase 1
- **Change:** Confirm `container.rs` has no direct tests to clean and that direct unit coverage,
  not indirect global or integration coverage, is required for the package-owned enabled publication
  decision.
- **Guardrails:** Do not move launcher or root tests, change production composition, or introduce a
  fixture before a specific test requires it.
- **Decision:** `container.rs` has no direct tests to clean. Its current composition is concise and
  explicit, so no test-code refactor applies. Indirect aggregate/global and higher-level coverage
  do not substitute for assessing the feasible focused unit contract in R2.
- **Done when:** The no-cleanup decision is recorded before adding a test.

### R2 - Cover the enabled server event-publication path

- **Status:** DONE
- **Priority:** High impact / low effort
- **Addresses:** P1
- **Change:** Add one direct asynchronous unit test for `UdpTrackerServerServices::initialize`.
  Publish a representative event through its available sender and assert that its own event-bus
  receiver obtains that exact event.
- **Guardrails:** Keep the causal enabled sender, publication Act, and expected event visible. Use
  only an absolute diagnostic timeout. Do not assert optional-sender implementation details, test
  generic disabled behavior, add sockets/tasks, or assert metrics/banning/root policy.
- **Prose-first review:** The temporary prose specified that newly initialized services publish a
  server event through their enabled sender to their own event-bus receiver. The final code makes
  the initialized services, available sender, exact representative event, sender publication Act,
  and received-event assertion visible. `sample_udp_request_received_event` names only incidental
  valid event construction; no fixture derives the expected event. The timeout is an absolute
  diagnostic failure bound. Temporary prose is redundant and removed.
- **Done when:** Disabling or disconnecting the package-composed publication path has one direct,
  deterministic unit-test failure.

### R3 - Review residual composition coverage and ownership

- **Status:** DONE
- **Priority:** Low impact / low effort
- **Change:** Measure unit-only coverage for `container.rs`, separately retain aggregate/global and
  integration-only evidence, and record ownership for residual paths.
- **Guardrails:** Do not add tests merely to increase percentages. Do not claim unit coverage from
  aggregate/global or integration-only results.
- **Decision:** The separately measured reports show `container.rs` at 59/59 lines, 72/72 regions,
  and 5/5 functions (all 100%) for aggregate/global and unit-only execution; integration-only
  execution independently covers its 19 production lines, 29 regions, and 2 functions (all 100%).
  The R2 unit test provides the direct package-owned enabled-publication contract. Do not add a
  coverage-only test for `UdpTrackerServerContainer::initialize` cloning service handles, empty
  repository state, generic disabled-bus behavior, or root consumer policy: those would test an
  internal allocation detail, `Repository`, `packages/events`, or root composition respectively.
- **Done when:** Unit-only measurement and each residual ownership decision are recorded.

## Progress Tracking

### Plan Checklist

- [x] Container source, event-bus responsibility, indirect package coverage, and root-policy
      boundaries reviewed.
- [x] Maintainer approved R1.
- [x] R1 implemented, reviewed, validated, and committed.
- [x] Maintainer approved R2.
- [x] R2 implemented, reviewed, validated, and committed.
- [x] R3 coverage/ownership review completed and decision recorded.
- [ ] Maintainer reviewed all approved changes.
- [ ] Plan completed and ready for final verification.

### Progress Log

- 2026-09-10 - GitHub Copilot - Created this proposed plan after reviewing package container
  composition, event-bus ownership, existing launcher unit tests, root policy tests, and the
  clarified unit-first coverage objective. No test or production change has been made.
- 2026-09-11 - User/maintainer - Approved R1. Record that `container.rs` has no direct test code
  to clean; do not use indirect aggregate/global or higher-level coverage to avoid the R2 unit-test
  assessment.
- 2026-09-11 - User/maintainer - Approved R2. Add the direct deterministic services event-bus
  publication test only, retaining the visible sender, event, publication Act, and received-event
  assertion.
- 2026-09-11 - User/maintainer - Approved R3. Measure aggregate/global, unit-only, and
  integration-only coverage separately and record residual composition ownership without adding a
  percentage-only test.

### Validation Evidence

| Increment | Status | Evidence |
| --- | --- | --- |
| Plan documentation | TODO | Run Markdown and spelling checks after maintainer review changes. |
| R1 | DONE | The reviewed source has no direct test code or concrete cleanup opportunity. The explicit no-cleanup decision preserves the feasible R2 unit-test assessment under the unit-first coverage policy. |
| R2 | DONE | `cargo fmt --all -- --check`, `cargo test -p torrust-tracker-udp-server container::tests::should_publish_events_through_the_enabled_server_event_bus`, and `git diff --check` passed. Prose-first review keeps the enabled sender, exact event, publication Act, and received-event assertion visible; the timeout is diagnostic only. |
| R3 | DONE | Separate clean reports passed: aggregate/global package source is 5,423/5,548 lines (97.75%), 7,088/7,340 regions (96.57%), and 551/565 functions (97.52%); unit-only is 5,317/5,548 lines (95.84%), 6,961/7,340 regions (94.84%), and 535/565 functions (94.69%); integration-only is 1,118/1,469 lines (76.11%), 1,190/1,654 regions (71.95%), and 148/187 functions (79.14%). `container.rs` unit-only coverage is 59/59 lines, 72/72 regions, and 5/5 functions (all 100%); integration-only separately covers 19/19 production lines, 29/29 regions, and 2/2 functions (all 100%). |

## Non-Goals

- Do not change container production code, event-bus implementation, metrics aggregation, banning,
  REST exposure, or multi-listener policy.
- Do not duplicate generic enabled/disabled `EventBus` tests owned by `packages/events`.
- Do not start sockets, listeners, server tasks, cancellation, shutdown, sleeps, polling, or a
  lifecycle fixture; those concerns remain owned by #1488.
- Do not replace root integration tests or use their coverage to claim this direct unit contract.

## Validation Per Approved Increment

- Apply the mandatory prose-first Arrange-Act-Assert comparison before maintainer review.
- Run the focused `container` unit test and then the package `--lib` target when the increment is
  approved for broader validation.
- Run `cargo fmt --all -- --check` and `git diff --check`.
- Run `linter markdown` and `linter cspell` when this plan changes.
- Measure and record aggregate/global, unit-only, and integration-only coverage separately whenever
  coverage informs a decision.

## Completion Criteria

- The package-selected enabled event-publication path has one direct, deterministic unit contract.
- The test keeps its causal enabled sender, production publication Act, and exact received event
  visible without a generic fixture.
- Aggregate/global, unit-only, and integration-only coverage are recorded in separate tables and
  used only for their respective claims.
- Generic event-bus behavior, root consumer policy, and lifecycle concerns remain at their existing
  ownership boundaries.
- The maintainer reviews every approved increment before the next increment and before final
  verification.
