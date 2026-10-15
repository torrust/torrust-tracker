---
doc-type: file-test-plan
issue: 1348
package: torrust-tracker-udp-core
target-file: packages/udp-core/src/services/scrape.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-core/src/services/scrape.rs
    - packages/tracker-core/src/scrape_handler.rs
    - packages/tracker-core/src/error.rs
    - packages/udp-server/src/handlers/scrape.rs
    - docs/issues/closed/1348-1347-add-tests-udp-core/coverage-evidence.md
---

# UDP Core Scrape Service File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-core/src/services/scrape.rs`.

## Current State

- **Unit-only coverage:** 0 / 77 lines, 0 / 78 regions, 0 / 14 functions, at T1 and unchanged
  after the announce and peer-builder files. PR #2423 changed only this file's module doc comment.
- **Existing tests:** none in this file. `udp-server` handler tests exercise the service through
  the handler, which EPIC #1347 does not accept as evidence for this package's unit seam.
- **Module-owned decisions:** apply the connection-ID validation flag; validate the cookie against
  the client socket fingerprint and the given range before tracker core does any work; convert
  wire info hashes to domain info hashes; adapt tracker-core failures to `UdpScrapeError`; publish
  `UdpScrape` with the connection context (instance ID, client address, server binding, public URL)
  only after a successful scrape.
- **Collaborator change from #2423:** `ScrapeError` gained a `Database` variant. Tracker core
  returns it when the batched persisted-downloads lookup fails. This makes
  `TrackerCoreScrapeError` reachable. `tracker-core` publicly exports `MockTorrentMetricsStore`,
  so a test can construct a failing store without a database.
- **T1 hypothesis:** test candidate.

## Current Tests Review

No existing tests. R1 is skipped.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Cookie policy: strict validation rejects a cookie issued outside the valid range | 3A unit | S1: expired cookie with validation enabled returns `ConnectionCookieError`. |
| Cookie policy: disabled validation accepts the same cookie | 3A unit | S2: identical request with validation disabled succeeds; the flag is the one Arrange difference. |
| Authentication before tracker-core work | 3A unit | S3: with persistent statistics backed by a store that has no expectations, an expired cookie returns the cookie error and causes no persisted-downloads lookup. An unauthenticated packet must not cause database reads. |
| Wire info-hash conversion | Collaboration | S4: scraping a torrent with an in-memory swarm returns its statistics under the requested info hash. The statistics rule is `tracker-core`'s; the test protects the conversion. |
| Tracker-core failure adaptation | Collaboration | S5: a failing persisted-downloads lookup returns `TrackerCoreScrapeError` wrapping `ScrapeError::Database`. |
| Event publication | 3A unit | S6: a successful scrape sends exactly one `UdpScrape` event with the full connection context including the public URL. |
| Absent event sender | 3A unit | No dedicated test: S1-S5 run without a sender and therefore cover it. |
| `TrackerCoreWhitelistError` and `From<WhitelistError>` | Unreachable here | Not testable: this service never authorizes, and tracker core reports non-whitelisted torrents as zeroed statistics. See the finding below; no production change in this issue. |
| Per-torrent statistics, whitelist zeroing, persisted downloads | Collaborator-owned | `tracker-core` owns them and tests them in `scrape_handler.rs`. |
| Property-based testing | Not selected | No natural property: the decisions are enumerable branches. |

Test bootstrap: an in-file fixture constructs only the whitelist authorization, the in-memory
torrent repository, and the `ScrapeHandler`. Scenario constructors name the one state difference:
a public tracker, or a tracker whose persisted downloads come from a given store mock. Requests
are built in-file, as in the announce service tests.

**Finding:** the two whitelist variants have different statuses.

- `ScrapeError::Whitelist` is reserved on purpose. The #1270 refactor (2025-02-17, merged commit
  `da1353bf1`) made the scrape
  handler return `Result` so that a future option (for example, rejecting non-whitelisted torrents
  instead of zeroing them) would not break callers. `ScrapeError` is not `#[non_exhaustive]`, and
  `udp-server` matches it exhaustively, so pre-declaring the variant also avoids that break. The
  current `handle_scrape` doc no longer states this intent.
- `UdpScrapeError::TrackerCoreWhitelistError` stays unreachable even if that option ships: a
  tracker-core whitelist failure would arrive as `TrackerCoreScrapeError` wrapping
  `ScrapeError::Whitelist`. It appears to mirror the announce service, which authorizes itself.

No action in this issue. Both points are recorded as follow-up candidates in
[lessons.md](../lessons.md).

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests to refactor. | Recorded above. |
| R2 | DONE | Add S1-S6. | 6 focused `services::scrape` tests passed; nightly formatting, pedantic Clippy, and `git diff --check` clean; S3 failed (store mock panicked on a read) under a temporary move of the cookie check after the tracker-core call. |
| R3 | SKIPPED | No integration increment: the service has no network boundary. | Explicit rationale. |
| R4 | DONE | Re-run unit-only coverage; record before/after. | 0 / 77 to 264 / 273 lines; see Results. |

## Results

- **Unit-only coverage (test-inclusive file totals):** 0 / 77 (0.00%) to 264 / 273 (96.70%)
  lines; 0 / 78 to 303 / 316 regions; 0 / 14 to 37 / 40 functions.
- **Residual uncovered lines:** the `configuration_instance_id()` and `public_url()` accessors and
  the unreachable `From<WhitelistError>` adaptation. No untested service decision remains.
- **Tests added:** 6 (S1-S6), using an in-file fixture that optionally backs persisted downloads
  with a `tracker-core` store mock.
- **Integration selected:** No.
- **Completed-file review:** Approved by Jose Celano on 2026-10-05.

## Prose-First AAA Review

Recorded on 2026-10-15, late: the PR #2538 Task Reviewer found no record of it (AC7). For each
test, temporary Arrange, Act, and Assert prose was compared with the code and then removed.

| Test | Arrange → Act → Assert | Outcome |
| --- | --- | --- |
| `..._reject_an_expired_connection_id_when_cookie_validation_is_enabled` | Expired ID → `handle_scrape` validating → `ValueExpired` | No change |
| `..._accept_an_expired_connection_id_when_cookie_validation_is_skipped` | Expired ID → `handle_scrape` skipping → `Ok` | No change |
| `..._validate_the_connection_id_before_reading_any_persisted_downloads` | Store that fails if read, expired ID → `handle_scrape` → cookie error | No change |
| `..._return_the_swarm_statistics_under_the_requested_info_hash` | One seeder announced → `handle_scrape` → one complete peer | Changed: the expected `SwarmMetadata` is a struct literal, so `complete: 1` visibly matches the one seeder |
| `..._report_a_failing_persisted_downloads_lookup_as_a_tracker_core_scrape_error` | Failing downloads store → `handle_scrape` → `Database` error | No change |
| `..._publish_a_scrape_event_with_the_connection_context` | Mock expects one event → `handle_scrape` → mock verifies on drop | Changed: the `// Assert` marker now says the mock verifies the event |

## Approval

Approved by Jose Celano on 2026-10-05: S1-S6 and the fixture approach. The whitelist finding is
recorded above as a `lessons.md` candidate, with no action in this issue.

## Progress Log

- 2026-10-05 15:04 UTC - GitHub Copilot - Rebased onto `develop` (9 commits; none touched
  `udp-core` or the tracker-core scrape handler). Created the plan from `services/scrape.rs`, the
  post-#2423 `ScrapeHandler`, and the latest unit-only report. No Rust tests or production code
  changed. R2 awaits maintainer approval.
- 2026-10-05 18:03 UTC - GitHub Copilot - Plan approved after the whitelist finding was revised
  with git-history evidence. Implemented R2 and measured R4 (stable Rust 1.99.0, nightly 1.101.0,
  `cargo-llvm-cov 0.6.16`). The plan awaits completed-file review.
- 2026-10-05 18:44 UTC - Jose Celano - Approved completed-file review. The file is ready for
  pre-commit and its signed per-file commit.
- 2026-10-15 10:30 UTC - GitHub Copilot - Recorded the prose-first AAA review late, after the
  PR #2538 Task Reviewer found no record (AC7). Two changes: the expected swarm statistics as a
  struct literal and the event test's `// Assert` marker.
