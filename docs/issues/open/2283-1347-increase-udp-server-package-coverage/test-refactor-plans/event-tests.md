---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/event.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/event.rs
    - packages/udp-server/src/error.rs
    - packages/udp-core/src/services/scrape.rs
    - docs/issues/open/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Event File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/udp-server/src/event.rs`.

## Current State

- **Baseline unit-only coverage:** 125 / 131 lines (95.42%).
- **R4 unit-only coverage:** 190 / 190 lines, 214 / 214 regions, and 17 / 17 functions (100.00%)
  from `cargo llvm-cov -p torrust-tracker-udp-server --all-features --lib --json`.
- **Uncovered executable lines:** the nested announce whitelist branch plus `ErrorKind::from(
  Error::ScrapeFailed)` and all three scrape source variants: connection-cookie, tracker-core
  `ScrapeError::Whitelist`, and direct tracker-core `WhitelistError`.
- **Module-owned decisions:** this module classifies package errors as objective `ErrorKind`
  facts and maps each `UdpRequestKind` to a stable display and metrics-label value.
- **Existing tests:** six focused error-classification tests and one table-driven test that covers
  both the display and metrics-label contracts for all request kinds.
- **T1 hypothesis:** no change. Fresh unit-only evidence falsifies it for the nested announce and
  scrape-error classification branches.

## Current Tests Review

The six error-classification tests have state-centred Arrange sections, a direct `ErrorKind::from`
Act, and one independently specified result. They do not need refactoring. The table-driven
`it_should_convert_request_kinds_to_metric_labels_and_display_values` test combines two distinct
contracts and has multiple semantic assertions per case. R1 should split it into one display test
and one metrics-label test while retaining the three explicit request-kind cases in each.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Request-kind display and metrics-label mapping | 3A unit | R1 refactor the existing combined test into one test per public conversion contract. |
| Nested announce whitelist classification | 3A unit | R2 add a direct typed `Error::AnnounceFailed` to `ErrorKind::Whitelist` contract. |
| Scrape connection-cookie classification | 3A unit | R2 add a direct typed `Error::ScrapeFailed` to `ErrorKind::ConnectionCookie` contract. |
| Scrape whitelist classification | 3A unit | R2 add direct contracts for the nested `ScrapeError::Whitelist` and direct `WhitelistError` forms. |
| Event type aliases | Type wiring | Do not add tests; `events` owns sender, receiver, and bus behavior. |
| Handler event emission | Collaboration boundary | Handler tests own emission and consumer tests own interpretation. |
| Real UDP behavior | Integration | Not selected: event classification has no socket or protocol Act. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Split the combined request-kind display/label test into one test per contract. | Focused `event::tests`, nightly formatting, and prose-first review passed. |
| R2 | DONE | Added direct typed unit tests for the nested announce whitelist branch and each uncovered `ScrapeFailed` classification branch. | Fixed source error, direct conversion Act, and complete expected `ErrorKind` result in four tests. |
| R3 | SKIPPED | No integration increment: no real UDP boundary is controlled by this module. | Explicit no-integration rationale. |
| R4 | DONE | Re-ran unit-only coverage and recorded full file coverage. | 190 / 190 lines, 214 / 214 regions, and 17 / 17 functions (100.00%). |

## Approval Requested

Approve the completed-file result before its test and documentation commit. R1 separated the two
existing public contracts, and R2 covers the four previously untested package-owned classification
branches without changing event schemas, handlers, or generic event-bus behavior.

## Progress Log

- 2026-09-22 16:27 UTC - GitHub Copilot - Created the plan from `event.rs`, its current tests,
  callers, and fresh unit-only missing-line evidence. The plan selects one test-only R1 refactor
  and three direct R2 unit contracts. No Rust tests or production code changed. R1 and R2 await
  maintainer approval.
- 2026-09-22 16:30 UTC - User/maintainer - Approved R1 and R2, then approved the R2 amendment
  after fresh coverage identified the nested announce whitelist branch as a fourth missing
  classification contract.
- 2026-09-22 16:37 UTC - GitHub Copilot - Completed R1-R4. R1 split the mixed display/label test
  into `it_should_render_request_kinds` and `it_should_convert_request_kinds_to_metric_labels`.
  R2 added direct classification contracts for nested announce whitelist, scrape connection-cookie,
  nested scrape whitelist, and direct scrape whitelist errors. `cargo test -p
  torrust-tracker-udp-server event::tests` passed 12 tests; nightly Rust formatting and `git diff
  --check` passed. Fresh unit-only coverage increased from 125 / 131 lines (95.42%) to 190 / 190
  lines, 214 / 214 regions, and 17 / 17 functions (100.00%). Completed-file review is requested.
- 2026-09-22 16:38 UTC - User/maintainer - Approved the completed-file result. The plan is done;
  its test and documentation increment awaits the mandatory validation and signed commit.
