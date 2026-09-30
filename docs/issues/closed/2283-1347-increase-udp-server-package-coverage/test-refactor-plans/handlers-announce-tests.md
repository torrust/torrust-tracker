---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/handlers/announce.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/handlers/announce.rs
    - packages/udp-server/src/handlers/mod.rs
    - packages/udp-server/src/handlers/error.rs
    - packages/udp-core/src/services/announce.rs
    - docs/issues/closed/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Announce Handler File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/handlers/announce.rs`.

## Current State

- **Fresh unit-only coverage:** 781 / 800 lines (97.63%) from the T1 report.
- **Uncovered executable lines:** the `map_err` adaptation from a failed `AnnounceService` call to
  `HandlerError`: converted `Error`, request transaction ID, and original `UdpRequestKind::Announce`.
- **Module-owned decisions:** emit accepted/error facts, select strict or disabled cookie validation,
  preserve service failures for packet error routing, and shape response peers by address family.
- **Existing tests:** IPv4/IPv6 announce collaboration scenarios cover peer registration, address
  family filtering, accepted-request event publication, and configured external-IP behavior.
- **T1 hypothesis:** no change. Fresh unit-only evidence falsifies it for service-failure adaptation.

## Current Tests Review

The existing tests provide distinct IPv4/IPv6 and policy scenarios. Their event-publication names
contain only terminology defects: `upd4`/`upd6` should be `udp4`/`udp6`. R1 should correct those
two names without reorganizing the broader, scenario-heavy collaboration fixtures.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Announce-service failure adaptation | Collaboration | R2 add a strict invalid-cookie scenario that asserts the returned error preserves the converted error, transaction ID, and original announce request kind. |
| Accepted-request event publication | 3A unit | Existing IPv4/IPv6 mock contracts cover it; R1 corrects their names only. |
| Disabled-validation cookie observability | Collaboration | Existing service/event flows own the policy behavior; no additional test selected without a clear untested branch. |
| Peer registration | Collaborator-owned | `udp-core` and tracker-core own business semantics; current collaboration scenarios remain sufficient. |
| Response peer shaping | 3A unit | R5 directly covers the UDP adapter's concrete IPv4/IPv6 protocol peer conversion and filtering, without exercising peer registration. |
| UDP socket transport | Integration | Not selected: this adapter has no socket Act. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Correct `upd4`/`upd6` to `udp4`/`udp6` in accepted-event test names. | Focused `handlers::announce` tests, nightly formatting, and prose-first review passed. |
| R2 | DONE | Add one strict invalid-cookie collaboration test for `HandlerError` adaptation. | Fixed invalid connection ID, direct handler Act, and complete error tuple assertion passed. |
| R3 | SKIPPED | No integration increment: this handler controls no UDP socket boundary. | Explicit no-integration rationale. |
| R4 | DONE | Re-run unit-only coverage and record before/after file results. | `781 / 800` (97.63%) to `903 / 905` (99.78%) lines; 55 / 55 functions; 1,273 / 1,278 regions. |
| R5 | DONE | Add a direct mixed-address-family response-shaping unit test. | Explicit `AnnounceData`, one IPv4 and one IPv6 peer, and independently selected protocol responses passed. |

## Results

- **Unit-only coverage:** 781 / 800 (97.63%) to 903 / 905 (99.78%) lines; 53 / 53 to 55 / 55
  functions; 1,193 / 1,208 to 1,273 / 1,278 regions.
- **Tests refactored:** 2 terminology corrections.
- **Tests added:** 2 focused unit/collaboration contracts: strict service-failure routing and
  address-family response conversion.
- **Integration selected:** No. The handler has no socket Act.
- **Residual executable lines:** two impossible alternate response match arms in legacy exclusion
  tests and one port-zero test-fixture guard; none is an untested production decision.
- **Completed-file review:** Approved by Jose Celano on 2026-09-22.

## Approval Requested

Approve R1 and R2 before changing Rust tests. The proposed test covers the handler-owned error
tuple without duplicating `udp-core` cookie semantics or packet-to-wire error response behavior.

## Progress Log

- 2026-09-22 17:25 UTC - GitHub Copilot - Created the plan from `handlers/announce.rs`, current
  IPv4/IPv6 scenarios, and fresh unit-only missing-line evidence. The plan proposes two
  terminology-only R1 name corrections and one R2 service-failure adaptation contract. No Rust
  tests or production code changed. R1 and R2 await maintainer approval.
- 2026-09-22 18:10 UTC - Jose Celano - Approved R5 after R4 showed that the existing exclusion
  scenarios leave both peer-conversion arms uncovered.
- 2026-09-22 18:15 UTC - GitHub Copilot - Completed R1/R2 and focused validation: 13 announce
  tests passed with nightly formatting and `git diff --check` clean. R4 measured 831 / 841
  (98.81%) unit-only lines, leaving peer-conversion arms uncovered.
- 2026-09-22 18:25 UTC - GitHub Copilot - Completed R5 and focused validation: 14 announce tests
  passed with nightly formatting and `git diff --check` clean. Fresh R4 measurement recorded 903
  / 905 (99.78%) unit-only lines, 55 / 55 functions, and 1,273 / 1,278 regions. The plan awaits
  completed-file review.
- 2026-09-22 18:30 UTC - Jose Celano - Approved completed-file review. The file is ready for
  package validation, pre-commit, and its signed per-file commit.
