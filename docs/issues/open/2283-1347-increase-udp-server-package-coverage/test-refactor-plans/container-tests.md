---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/container.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/container.rs
    - packages/udp-server/src/event.rs
    - packages/udp-server/src/server/launcher.rs
    - docs/issues/open/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
    - docs/issues/closed/2149-1347-add-focused-udp-server-package-tests/test-refactor-plans/container-tests.md
---

# UDP Server Container File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/udp-server/src/container.rs`.

## Current State

- **Baseline unit-only coverage:** 59 / 59 lines, 72 / 72 regions, and 5 / 5 functions (100.00%)
  from the T1 report in `coverage-evidence.md`.
- **Module-owned decision:** `UdpTrackerServerServices::initialize` composes an enabled
  package-local event bus, its sender, and a statistics repository. `UdpTrackerServerContainer`
  exposes those service handles for server composition.
- **Existing test:** one asynchronous test publishes an exact `UdpRequestReceived` event through
  the initialized services sender and observes it on the same event bus.
- **T1 hypothesis:** probably no change. #2149 already added the direct unit contract for the
  package-selected enabled publication path.

## Current Tests Review

R1 will review `it_should_publish_events_through_the_enabled_server_event_bus` against the
`write-unit-test` skill. Its initial reading is clean: the enabled sender, exact event, publication
Act, and received-event assertion are visible; `sample_udp_request_received_event` hides only
ordinary valid event construction; and the one-second timeout bounds diagnostic receive failure
without controlling behavior. R1 confirmed that conclusion: no readability, duplication, fixture,
assertion, or Arrange-Act-Assert refactor is justified.

## Coverage Analysis

| Behavior group | Classification | Decision to review |
| --- | --- | --- |
| Enabled package event-publication path | 3A unit | Already directly protected by the current exact-event test. Review its readability before deciding no change. |
| Container handle exposure | Internal allocation detail | Do not add an identity/cloning test; the services composition contract is the maintained behavior. |
| Disabled event bus variants | Collaborator-owned | `packages/events` owns generic bus semantics. |
| Metrics/banning consumer policy | Root composition | Consumers decide filtering and policy; do not duplicate them here. |
| Real UDP behavior | Integration | Not selected: the container has no socket or protocol act. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Reviewed the existing event-publication test; no refactor is justified. | Focused container test, nightly formatting, and prose-first review passed. |
| R2 | DONE | No additional unit test: enabled publication is already directly protected; handle identity is an internal allocation detail, and disabled behavior is owned by `events`. | Code-path and collaborator-ownership review. |
| R3 | DONE | No integration increment: this module has no socket or protocol Act; real UDP behavior belongs to server integration boundaries. | Explicit no-integration rationale. |
| R4 | DONE | Re-ran unit-only coverage and recorded unchanged full coverage. | 59 / 59 lines, 72 / 72 regions, and 5 / 5 functions (100.00%). |

## Approval Requested

Approve the completed-file result before its documentation-only commit. No refactor or additional
test is proposed: the direct publication contract covers the only package-owned decision, while the
other candidates are internal details or collaborator/root-owned behavior.

## Progress Log

- 2026-09-22 12:50 UTC - GitHub Copilot - Created the plan from the T1 baseline, current test,
  package consumers, and #2149 plan. No tests or production code changed. R1 awaits maintainer
  approval.
- 2026-09-22 16:15 UTC - User/maintainer - Approved the file test plan and R1 review.
- 2026-09-22 16:15 UTC - GitHub Copilot - Completed R1 with no code change. The existing test
  has a state-centred Arrange, visible publication Act, one complete received-event assertion, and
  a narrowly scoped diagnostic timeout. `cargo test -p torrust-tracker-udp-server
  container::tests::it_should_publish_events_through_the_enabled_server_event_bus` passed; nightly
  formatting and `git diff --check` passed.
- 2026-09-22 16:22 UTC - GitHub Copilot - Completed R2-R4 with no code change. `container.rs`
  selects `SenderStatus::Enabled`, and the current exact-event test proves that package-owned
  decision. Generic enabled/disabled sender behavior is tested in `packages/events`; container
  handle cloning is an internal allocation detail; and this module has no socket or protocol Act
  for an integration increment. Fresh `cargo llvm-cov --lib` coverage is unchanged: 59 / 59 lines,
  72 / 72 regions, and 5 / 5 functions (100.00%). Completed-file review is now requested.
- 2026-09-22 16:25 UTC - User/maintainer - Approved the completed-file result. The plan is done;
  its documentation-only increment awaits the mandatory validation and signed commit.
