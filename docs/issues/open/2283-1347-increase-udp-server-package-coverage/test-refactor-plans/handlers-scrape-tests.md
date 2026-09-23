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

## Post-Commit Improvement Plan

Identified after the completed-file commit by applying the `write-unit-test` skill with the goal
that every test reads as prose a tracker operator can follow. Nothing here reopens the completed
increment or authorizes test changes; approve the steps below as a separate scrape-focused
increment.

### Prose Specification

Each test must state one of these facts and nothing else. This is the reader's contract; the code
is judged against it during the prose-first review.

1. Scraping a torrent the tracker does not know returns zeroed statistics for it.
2. Scraping a torrent with one seeder reports one seeder, no leechers, and no completed downloads.
3. On a listed tracker, scraping a whitelisted torrent returns its statistics.
4. On a listed tracker, scraping a torrent that is not whitelisted returns zeroed statistics.
5. Handling a scrape publishes an "accepted scrape request" fact naming the client and the
   listener; one case for an IPv4 client, one for an IPv6 client.
6. Under strict validation, a scrape with an invalid connection ID fails, and the failure keeps the
   client's transaction ID and the scrape kind so the error response can be routed back.
7. With validation disabled, an invalid connection ID is published as a connection-cookie error
   fact, but the scrape is still answered.
8. The wire response for one torrent echoes the request's transaction ID and encodes its seeders,
   completed downloads, and leechers.
9. Counters that fit in a signed 32-bit integer are encoded as-is; larger counters saturate.

### Smell Audit

Signal is the share of lines that state causal state, the production action, or the expected
result; the rest is plumbing the reader must skip.

| Current test | Signal | Smells |
| --- | ---: | --- |
| `should_return_no_stats_when_the_tracker_does_not_have_any_torrent` | ~6 / 35 | Complex Arrange (service tuple, listener binding literal, cookie forging); six-argument Act; legacy name; no AAA markers. |
| `it_should_preserve_a_scrape_service_failure_for_packet_error_routing` | ~8 / 30 | Same Arrange plumbing; nested `matches!` reads as code, not prose. |
| `with_a_public_tracker::should_return_torrent_statistics_when_...` | ~4 / 15 | Hidden production Act and hidden causal state: the seeder that justifies `seeders: 1` is created inside `add_a_sample_seeder_and_scrape`. |
| `with_a_whitelisted_tracker::*` (two tests) | ~5 / 40 each | The one causal line (`in_memory_whitelist.add`) is buried in 20 duplicated plumbing lines; both tests are copies. |
| `using_ipv4` / `using_ipv6::should_send_the_*_scrape_event` | ~6 / 35 each | `mockall` DSL (`expect_send().with(eq(..)).times(1).returning(..)`) is unreadable to non-developers; `ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0)` duplicates a fixture-derived value; IPv4 test uses an IPv6 listener; two modules exist for one test each; `sample_scrape_request` is declared after its callers. |
| `should_saturate_large_download_counts_for_udp_protocol` | 3 / 3 | Three unlabeled assertions; name says "download counts" but the rule applies to every counter; no AAA. |
| Helpers | — | `build_scrape_request` and `sample_scrape_request` are near-duplicates; `scrape_response` is fine. |

Root cause: `handle_scrape` takes six arguments of which four (`scrape_service`, listener
`ServiceBinding`, event sender, `CookieValidationContext`) describe the listener, not the request.
Every test re-supplies them, so the plumbing is structural, not accidental.

### Target Design

Reuse the two patterns this package already accepted in
[`handlers/error.rs`](../../../../../packages/udp-server/src/handlers/error.rs) tests: a named
helper that calls the production function with the context no test varies, and event capture via a
`Broadcaster` subscription instead of `mockall`.

Test-only fixture catalog, all local to `scrape::tests`:

- `Tracker` scenario fixture: `Tracker::public().await`, `Tracker::listed().await`, then
  `.with_seeder_for(&info_hash).await` and `.whitelisting(&info_hash).await`. It owns the service
  bootstrap, listener binding, and event bus, and exposes them for the Act. It never calls
  `handle_scrape`.
- `ScrapeRequestBuilder` (mirrors the announce builder): `scrape_request_for(&info_hash)` yields a
  valid-cookie request from the default client; `.with_connection_id(..)` and
  `.with_transaction_id(..)` name the deviations.
- `scrape(&tracker, &request) -> Result<Response, HandlerError>`: documented as "calls the
  production `handle_scrape` with the tracker's listener context and strict cookie validation".
  Tests that vary the cookie policy or the client address call `handle_scrape` directly so the
  varied value stays visible.
- `published_events(&tracker) -> Vec<Event>` drains the broadcaster subscription so the Assert
  compares against a complete expected event whose context is built from the same client and
  listener values used in the Act.

Target shape for fact 4:

```rust
#[tokio::test]
async fn it_should_return_zeroed_statistics_when_the_torrent_is_not_whitelisted() {
    // Arrange
    let info_hash = sample_info_hash();
    let tracker = Tracker::listed().await.with_seeder_for(&info_hash).await;
    let request = scrape_request_for(&info_hash);

    // Act
    let response = scrape(&tracker, &request).await.unwrap();

    // Assert
    assert_eq!(response, scrape_response(request.transaction_id, vec![zeroed_torrent_statistics()]));
}
```

Trade-off to decide before F1: `scrape(..)` hides the production call behind a named action, which
the skill lists as a smell but which `handlers/error.rs` accepted for the same reason (a wide
context signature). Alternative: expose `tracker.scrape_service()`, `tracker.service_binding()`,
and `tracker.event_sender()` and keep the six-argument `handle_scrape(..)` in every test. The first
reads as prose; the second keeps the Act literal. Maintainer decision required.

### Steps

| ID | Status | Work | Boundary |
| --- | --- | --- | --- |
| F1 | PROPOSED | Introduce the `Tracker` fixture, `ScrapeRequestBuilder`, `scrape`, and `published_events`; delete `add_a_seeder`, `build_scrape_request`, `sample_scrape_request`, and `add_a_sample_seeder_and_scrape`; flatten `using_ipv4`/`using_ipv6`. Rewrite facts 1-6 on the new fixtures with `it_should_*` names and AAA markers. Reorganisation only: no fact added or removed. | One commit, behavior-preserving; verified by the unchanged 8-test count and unit-only coverage. |
| F2 | PROPOSED | Replace `mockall` in the two accepted-event tests with `published_events`; build the expected `ConnectionContext` from the fixture's instance ID and the visible client/listener values; use an IPv4 listener for the IPv4 client. | Removes fixture-derived duplication and the mock DSL. |
| F3 | PROPOSED | Add fact 7: disabled validation, invalid connection ID, strict-free `handle_scrape` call visible, assert one `Event::UdpError { kind: Some(Scrape), error: ConnectionCookie(_) }` and a returned scrape response. | Handler-owned policy decision; cookie semantics stay in `udp-core`. |
| F4 | PROPOSED | Add fact 8 as a direct `build_response` test with one `ScrapeData` entry and a non-default transaction ID; assert the complete `ScrapeResponse`. | Pure adapter seam; one entry avoids `HashMap` order. |
| F5 | PROPOSED | Split the saturation test into fact 9's two cases: `it_should_encode_counters_that_fit_in_i32_as_is` and `it_should_saturate_counters_above_i32_max`, each with AAA. | No new dependency; `rstest` is not in this package. |
| F6 | BLOCKED | Treat multi-torrent response order as a defect, not a test gap. Source analysis found `tracker-core::ScrapeHandler` inserts request-order hashes into `ScrapeData.files: HashMap`; `udp-server::build_response` serializes that map's iteration order into positional `torrent_stats`. | Follow `fix-bug`: reproduce against a real UDP scrape artifact, add and prove red a two-torrent regression test, then fix the layer where order is lost. Do not add an order-insensitive test. F1-F5 wait so the refactor does not obscure red evidence. |
| F7 | PROPOSED | Record design feedback: group the four listener arguments of `handle_scrape` and `handle_announce` into one context type so tests and `handlers/mod.rs` stop re-threading them. | Production change outside #2283; propose as a follow-up issue, not in this branch. |

No integration candidate: every fact above is observable without a UDP socket.

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
- 2026-09-23 - GitHub Copilot - Re-read the `write-unit-test` skill after the signed completion
  commit and recorded F1-F3 as unapproved post-commit improvement candidates. No Rust tests or
  production code changed.
- 2026-09-23 - GitHub Copilot - Expanded the candidates into a prose specification, smell audit,
  fixture design, and F1-F7 execution plan. Source analysis then confirmed F6's causal path:
  request order is discarded by `ScrapeData.files: HashMap` before the UDP handler serializes
  positional scrape statistics. F6 is blocked for the repository bug-fix workflow; F1-F5 remain
  unimplemented to preserve a clear red regression investigation. No Rust code changed.
