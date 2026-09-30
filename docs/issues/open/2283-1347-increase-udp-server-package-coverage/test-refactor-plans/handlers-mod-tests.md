---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/handlers/mod.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/handlers/mod.rs
    - packages/udp-server/src/server/processor.rs
    - packages/udp-server/src/handlers/connect.rs
    - packages/udp-server/src/handlers/announce.rs
    - packages/udp-server/src/handlers/scrape.rs
    - docs/issues/open/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Handler Dispatch File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/handlers/mod.rs`.

## Current State

- **Baseline unit-only coverage:** 184 / 214 lines (85.98%).
- **R4 unit-only coverage:** 241 / 250 lines, 277 / 287 regions, and 38 / 40 functions (96.40%,
  96.52%, and 95.00%) from `cargo llvm-cov -p torrust-tracker-udp-server --all-features --lib
  --json`.
- **Uncovered executable lines:** `CookieTimeValues::new`, which reads the current clock once and
  computes the cookie issue time plus its lower and upper validation bounds. The first R2 test
  then exposed the untested `Request::Connect` dispatch arm in the fresh report.
- **Module-owned decisions:** parsing and error-routing orchestration, dispatch to the connect,
  announce, and scrape handlers, and cookie-time window calculation.
- **Existing test:** one asynchronous `handle_packet` collaboration test preserves a sendable
  parse-error transaction ID and correctly reports no request kind.
- **T1 hypothesis:** no change. Fresh unit-only evidence falsifies it for `CookieTimeValues::new`.

## Current Tests Review

The existing parse-routing test has a state-centred invalid raw scrape request, visible
`handle_packet` Act, and typed response/request-kind assertions. It is a collaboration test for
the public packet boundary and needs no refactor. It deliberately does not duplicate the specific
request dispatch semantics owned by the specialized handler modules.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Cookie-time range calculation | 3A unit | R2 add a direct `CookieTimeValues::new` test that compares the range with the returned issue time and fixed lifetime. |
| Connect request dispatch | Collaboration | R2 amendment proposes one `handle_request` test that verifies the returned connect response and `UdpRequestKind::Connect`. |
| Raw parse-error response routing | Collaboration | Existing `handle_packet` test owns the observable packet-boundary contract. |
| Connect, announce, and scrape behavior | Collaborator-owned | Specialized handler files own their service calls, events, and responses. |
| Packet processor UDP I/O | Integration | Not selected: server/processor owns the socket boundary. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Reviewed the existing parse-routing test; no refactor is justified. | Focused `handlers::tests`, nightly formatting, and prose-first review passed. |
| R2 | DONE | Added the cookie-window unit contract and the approved connect-dispatch collaboration contract. No further test is selected for fixture-only persistent-statistics setup or generated mock declarations. | Focused handler tests, coverage ownership review, and two direct contracts passed. |
| R3 | SKIPPED | No integration increment: this file has no UDP socket Act. | Explicit no-integration rationale. |
| R4 | DONE | Re-ran unit-only coverage and recorded the resulting coverage increase. | 241 / 250 lines, 277 / 287 regions, and 38 / 40 functions. |

## Approval Requested

Approve the completed-file result before its test and documentation commit. R2 covers the direct
cookie-window calculation and connect-dispatch result without duplicating connect-handler behavior
or the server socket boundary.

## Progress Log

- 2026-09-22 16:43 UTC - GitHub Copilot - Created the plan from `handlers/mod.rs`, its existing
  packet-boundary test, direct callers, and fresh unit-only missing-line evidence. One
  deterministic cookie-window unit contract is proposed. No Rust tests or production code changed.
  R1 and R2 await maintainer approval.
- 2026-09-22 16:46 UTC - User/maintainer - Approved R1 and R2. R1 found the existing
  parse-routing test clean, so no refactor was made. R2 added
  `it_should_build_a_cookie_validation_range_around_the_issue_time`; focused handler tests,
  nightly Rust formatting, and `git diff --check` passed. Fresh unit-only coverage then revealed
  the untested `Request::Connect` dispatch arm. Its additional collaboration test awaits maintainer
  approval as an R2 amendment.
- 2026-09-22 17:11 UTC - User/maintainer - Approved the R2 connect-dispatch amendment.
- 2026-09-22 17:11 UTC - GitHub Copilot - Completed R2-R4. Added
  `it_should_dispatch_a_connect_request`, which verifies a connect response preserves its request
  transaction ID and returns `UdpRequestKind::Connect`. `cargo test -p
  torrust-tracker-udp-server handlers::tests` passed three tests; nightly Rust formatting and
  `git diff --check` passed. Fresh unit-only coverage increased from 184 / 214 lines (85.98%) to
  241 / 250 lines, 277 / 287 regions, and 38 / 40 functions (96.40%, 96.52%, and 95.00%). The
  remaining uncovered entries are the persistent-statistics branch of test fixture construction
  and generated mock declarations, neither a package-owned production decision. Completed-file
  review is requested.
- 2026-09-22 17:14 UTC - User/maintainer - Approved the completed-file result. The plan is done;
  its test and documentation increment awaits the mandatory validation and signed commit.
