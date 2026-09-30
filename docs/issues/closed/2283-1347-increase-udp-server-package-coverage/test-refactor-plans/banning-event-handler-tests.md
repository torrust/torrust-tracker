---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/banning/event/handler.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/banning/event/handler.rs
    - packages/udp-server/src/banning/event/listener.rs
    - packages/udp-server/src/statistics/repository.rs
    - packages/udp-core/src/services/banning.rs
    - docs/issues/closed/2149-1347-add-focused-udp-server-package-tests/test-refactor-plans/banning-event-handler-tests.md
    - docs/issues/closed/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Banning Event Handler File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/banning/event/handler.rs`.

## Current State

- **Fresh unit-only coverage:** 81 / 82 lines (98.78%), 12 / 12 functions, and 108 / 110 regions
  from the stable Rust toolchain report `cargo llvm-cov -p torrust-tracker-udp-server --all-features
  --lib --json`. No package source has changed since this report was generated for row 18.
- **Module-owned decisions:** a classified connection-cookie event increments the `BanService`
  counter for the context client IP, then publishes the post-update distinct-IP total to the
  package statistics repository.
- **Existing tests:** two direct handler tests added by #2149 cover client-IP forwarding and the
  post-update distinct-IP gauge separately.
- **T1 hypothesis:** probably no change. The current tests and coverage support the approved
  no-change conclusion.

## Current Tests Review

| Test | Smells | Response |
| --- | --- | --- |
| `it_should_record_the_connection_cookie_error_for_its_client_ip` | Clean: visible cookie-error IP, direct `handle_event` Act, one counter assertion. | R1 no change. |
| `it_should_publish_the_distinct_client_ip_total_after_a_connection_cookie_error` | Clean: visible unrelated and causal IPs, named expected total, direct Act, one gauge assertion. | R1 no change. |

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Cookie-error client-IP forwarding and post-update gauge publication | 3A unit | Already directly covered by the two focused tests. |
| Non-cookie event filtering | Collaboration | The listener owns the ignored-event boundary; a local matrix would repeat the pattern guard without a clearer handler contract. |
| Statistics repository `set_gauge` failure | Diagnostics / collaborator-owned | Do not add an injection seam only to assert logging; repository failure mechanics and logging are not a handler contract. |
| Ban threshold, counter storage, and reset policy | Collaborator-owned | `udp-core` `BanService` owns these rules. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Confirmed current tests require no refactor. | Focused handler tests pass; current-tests review above. |
| R2 | SKIPPED | No package-owned unit contract remains unprotected. | Coverage analysis above. |
| R3 | SKIPPED | No real UDP boundary contract is selected. | The handler has no socket Act. |
| R4 | DONE | Recorded the completed no-change result. No code change, so no re-measurement is needed. | Current unit-only result is 81 / 82 lines (98.78%). |

## Results

Unit-only coverage remained **81 / 82 lines (98.78%)** before and after review. No Rust test or
production-code change was appropriate.

## Completed-File Review

Approved by Jose Celano on 2026-09-29.

## Progress Log

- 2026-09-29 - GitHub Copilot - Created the plan from the handler, its two tests, the completed
  #2149 plan, and the current unit-only coverage report. No Rust tests or production code changed.
- 2026-09-29 - Jose Celano - Approved the plan's no-change conclusion.
- 2026-09-29 - GitHub Copilot - Completed R1 after
  `cargo test -p torrust-tracker-udp-server banning::event::handler::tests` passed two tests on
  the stable Rust toolchain. R2 and R3 are skipped; R4 reuses the current unit-only report because
  no package source changed. Completed-file review is requested.
- 2026-09-29 - Jose Celano - Approved the completed-file result.
