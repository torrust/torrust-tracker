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
    - docs/issues/open/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
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

## Progress Log

- 2026-09-24 - GitHub Copilot - Created the plan from `launcher.rs`, its five tests, the #2149
  plan, the health-check API contracts, and fresh unit-only coverage. No Rust tests or production
  code changed. R1 and R2 await maintainer approval.
- 2026-09-25 - Jose Celano - Approved R1 and R2.
- 2026-09-25 - GitHub Copilot - Completed R1, R2, and R4. `cargo test -p torrust-tracker-udp-server
  launcher::tests` passed seven tests (stable Rust toolchain); nightly Rust formatting, Clippy, and
  `git diff --check` passed. Completed-file review is requested.
- 2026-09-25 - Jose Celano - Approved the completed-file result.
