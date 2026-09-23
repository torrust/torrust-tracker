---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/handlers/scrape.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/handlers/scrape.rs
    - packages/udp-server/src/handlers/mod.rs
    - packages/udp-server/src/handlers/error.rs
    - packages/udp-core/src/services/scrape.rs
    - docs/issues/open/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Scrape Handler File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/handlers/scrape.rs`.

## Current State

- **Fresh unit-only coverage:** 319 / 321 lines (99.38%), 27 / 28 functions, and 385 / 390 regions.
- **Uncovered executable lines:** the `map_err` adaptation from `ScrapeService` failure to
  `HandlerError`, and the impossible fallback arm in the test-only `match_scrape_response` helper.
- **Module-owned decisions:** emit accepted/error facts, select strict or disabled cookie validation,
  preserve service failures for packet error routing, encode scrape statistics, and saturate UDP
  counters.
- **Existing tests:** public/whitelisted scrape collaboration scenarios, IPv4/IPv6 accepted-event
  publication, and direct counter-saturation coverage.
- **T1 hypothesis:** no change. Fresh unit-only evidence falsifies it for the service-failure
  adaptation.

## Current Tests Review

The existing collaboration contracts remain distinct. R1 should correct the `upd4`/`upd6`
terminology and replace the optional `match_scrape_response` helper with a strict response
extractor, so an unexpected protocol response fails at the test boundary rather than becoming an
unexplained `None`. No scenario fixtures need restructuring.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Scrape-service failure adaptation | Collaboration | R2 add a strict invalid-cookie scenario that asserts the returned error preserves the converted error, transaction ID, and `UdpRequestKind::Scrape`. |
| Accepted-request event publication | 3A unit | Existing IPv4/IPv6 mock contracts cover it; R1 corrects their names only. |
| Disabled-validation cookie observability | Collaboration | Existing service/event flows own the policy behavior; no additional test selected without a clear untested branch. |
| Scrape response encoding and counter saturation | 3A unit | Existing direct and collaboration contracts cover the adapter boundary and saturation guard. |
| UDP socket transport | Integration | Not selected: this handler has no socket Act. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Correct `upd4`/`upd6` names and make scrape-response extraction strict. | Focused `handlers::scrape` tests, nightly formatting, and prose-first review passed. |
| R2 | DONE | Add one strict invalid-cookie collaboration test for `HandlerError` adaptation. | Fixed invalid connection ID, direct handler Act, and complete error tuple assertion passed. |
| R3 | SKIPPED | No integration increment: this handler controls no UDP socket boundary. | Explicit no-integration rationale. |
| R4 | DONE | Re-run unit-only coverage and record before/after file results. | `319 / 321` (99.38%) to `356 / 357` (99.72%) lines; 30 / 30 functions; 425 / 426 regions. |

## Results

- **Unit-only coverage:** 319 / 321 (99.38%) to 356 / 357 (99.72%) lines; 27 / 28 to 30 / 30
  functions; 385 / 390 to 425 / 426 regions.
- **Tests refactored:** two terminology corrections and strict scrape-response extraction.
- **Tests added:** one strict service-failure routing contract.
- **Integration selected:** No. The handler has no socket Act.
- **Residual executable line:** the explicit panic for an impossible non-scrape `Response` in the
  test-only strict extractor; it is not an untested production decision.
- **Completed-file review:** Approved by Jose Celano on 2026-09-23.

## Approval Requested

Approve R1 and R2 before changing Rust tests. The proposed test covers the handler-owned error
tuple without duplicating `udp-core` cookie semantics or packet-to-wire error response behavior.

## Progress Log

- 2026-09-23 - GitHub Copilot - Created the plan from `handlers/scrape.rs`, its current tests,
  and fresh unit-only missing-line evidence. The plan proposes R1 test cleanups and one R2
  service-failure adaptation contract. No Rust tests or production code changed. R1 and R2 await
  maintainer approval.
- 2026-09-23 - Jose Celano - Approved R1 and R2.
- 2026-09-23 - GitHub Copilot - Completed R1/R2 and focused validation: eight scrape tests passed
  with nightly formatting and `git diff --check` clean. Fresh R4 measurement recorded 356 / 357
  (99.72%) unit-only lines, 30 / 30 functions, and 425 / 426 regions. The plan awaits
  completed-file review.
- 2026-09-23 - Jose Celano - Approved completed-file review. The file is ready for package
  validation, pre-commit, and its signed per-file commit.
