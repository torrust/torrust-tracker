---
doc-type: file-test-plan
issue: 1349
package: torrust-tracker-http-core
target-file: packages/http-core/src/services/scrape.rs
status: done
semantic-links:
  related-artifacts:
    - packages/http-core/src/services/scrape.rs
    - packages/http-core/src/services/error_mapping.rs
    - packages/tracker-core/src/scrape_handler.rs
    - packages/tracker-core/src/databases/traits/torrent_metrics.rs
    - docs/issues/open/1349-1347-add-tests-http-core/coverage-evidence.md
---

# HTTP Core Scrape Service File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/http-core/src/services/scrape.rs`
and one shared test helper (see [Shared Helper](#shared-helper)). It reuses the approach of
[services-announce-tests.md](services-announce-tests.md) and the lessons recorded there.

## Current State

- **Unit-only coverage:** 507 / 540 lines (93.89%), 678 / 711 regions, 36 / 43 functions, measured at the
  S2 baseline (`develop` at the merge of #2575; the file did not change in S2). The totals include the
  test module.
- **Existing tests:** 8 in the file, in two modules: five in `with_real_data` (the reverse-proxy policy,
  the scrape data of a torrent, the persisted downloads of a torrent absent from memory, and one tcp4 and
  one tcp6 event test) and three in `with_zeroed_data` (the zeroed data for an unauthenticated peer on a
  private tracker, and a second tcp4 and tcp6 event pair).
- **Module-owned decisions:** on a private tracker an unauthenticated peer, including one with a key that
  does not authenticate, gets zeroed data for every requested torrent instead of an error; the scrape
  handler runs before the client IP is resolved; the event is published only after both succeed, carrying
  the resolved remote client, the server binding and the instance's public URL; the failure reason of each
  `HttpScrapeError`. Scrape aggregation, the whitelist and key validity are `tracker-core`'s; header and
  client-IP resolution are `http-protocol`'s.
- **T1 hypothesis:** test candidate (error paths).

## Current Tests Review

Reviewed against `write-unit-test`. Conclusion: **refactor**. The behavior is right; the smells make the
suite hard to extend.

| Test | Smells found |
| --- | --- |
| `it_should_use_the_http_tracker_reverse_proxy_policy` | Over 50 lines of Arrange, the expected event is built inside a `mockall` matcher, and the only assertion is `result.is_ok()`. |
| `it_should_return_the_scrape_data_for_a_torrent` | No `// Arrange`, `// Act`, `// Assert`; builds an event bus with a disabled sender that nothing reads; `Arc` wraps a service used once. The announce is a seeding step of the Arrange. |
| `it_should_return_the_persisted_downloads_of_a_torrent_absent_from_memory` | Clear AAA, but the Arrange creates a real SQLite database through `initialize_database` for a decision that `tracker-core`'s exported `MockTorrentMetricsStore` can express without one (#1348 lesson). |
| `it_should_send_the_tcp_4_scrape_event_when_the_peer_uses_ipv4` and `..._tcp_6_..._ipv6` (in `with_real_data`) | Near-identical 40-60 line bodies differing in the client address; no AAA markers; no explicit assertion (the mock verifies on drop). |
| `it_should_return_the_zeroed_scrape_data_when_the_tracker_is_running_in_private_mode_and_the_peer_is_not_authenticated` | No AAA markers; the expected value is `ScrapeData::zeroed(&info_hashes)`, a helper of the code under test's collaborator, not a literal; one info hash only, so it cannot show that every requested hash is zeroed; builds the same unused event bus. |
| `it_should_send_the_tcp_4_scrape_event_when_the_peer_uses_ipv4` and `..._tcp_6_..._ipv6` (in `with_zeroed_data`) | Exact duplicates of the two event tests above. The module name claims zeroed data, but they use `configuration::ephemeral()`, which is not private, so nothing is zeroed: the name describes an Arrange the tests do not set up. |

The file also keeps a `mock!` double (line 318, uncovered) and a fixture that creates a database for every
test.

## Coverage and Decision Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Zeroed data on a private tracker for no key (line 110) | 3A unit | Kept as R1, with a literal expected value and two requested hashes (R2b). |
| A key that does not authenticate (line 128) and a key that does | Collaboration | R2a: a private tracker returns zeroed data for an unregistered key and the real data for a registered one. Key validity is `tracker-core`'s; the test shows how the service uses its answer. Covers the uncovered `is_authenticated` path. |
| A public tracker with a presented key | 3A unit | R2a: planned as a test that a public tracker returns real data and never consults the key. **Dropped on 2026-10-20 while verifying**: on a public tracker `AuthenticationService::authenticate` accepts any key, so no mutation of this file can make that test fail; it would only repeat the plain public scrape test. |
| Every requested torrent is zeroed, including an unknown one | 3A unit | R2b: two requested hashes, one with a swarm. |
| Persisted-downloads store fails (line 112, `?`) | Collaboration | R2c: `MockTorrentMetricsStore` returns an error, and `handle_scrape` reports the tracker-core scrape failure and publishes no event. |
| Client IP cannot be resolved (line 115, `?`) | 3A unit | R2d: with a reverse proxy and no forwarded address, and without it and no connection address, the resolution failure is returned and no event is published. |
| Order: the scrape handler before the client-IP resolution | 3A unit | R2e: a failing store and an unresolvable client IP together return the tracker-core failure. Mutation M3 confirms it. |
| Event: remote client, binding, public URL | 3A unit | R1 (IPv4 and IPv6 client table and the reverse-proxy case) and R2f (the instance public URL, the absent sender). |
| `From` conversions and failure reasons (lines 164-209) | 3A unit | R2g: the conversions from `ScrapeError` and `PeerIpResolutionError` are reached through R2c and R2d. `WhitelistError`, `authentication::key::Error` and `TrackerCoreError` have no caller in the repository; as for `HttpAnnounceError` (S2) they are public conversions and are pinned by one table, not filed as findings. The protocol responses delegate to `http-protocol` (resolution) and `error_mapping.rs` (tracker core): the tests check delegation, not text. |
| Debug logging in `send_event` | logging | Not tested. |
| `mock!` macro (line 318) | test support | Removed by R1. |

No property candidate: the file has no input-by-state decision beyond the private-mode table above.

## Test Bootstrap

An in-file `Fixture` builds only what each scenario needs: the configuration, the in-memory torrent and key
repositories, the scrape handler (public, or reading from a given persisted-downloads store), the
authentication service and the announce handler used to seed a peer. There is no database and no event
listener. Named constructors express the one state difference (`new`, `with_persisted_downloads_from`), and
one method per service constructor. The production container is not used.

## Shared Helper

Both `services/announce.rs` (S2) and this file need an event sender that records what the service
publishes, so a test can assert on the events explicitly. **Proposal (R0):** move the announce tests'
`RecordingSender` unchanged into the existing `crate::tests` module of `lib.rs` and import it from both
files. One responsibility, named by what it does: record the HTTP core events a service publishes. The
alternative is a second copy in this file. Needs the maintainer's approval (shared helpers do).

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R0 | DONE | Move `RecordingSender` into `crate::tests`; update the announce tests' imports. | The 79 existing tests still passed; no production code changed. |
| R1 | DONE | Refactor the 8 existing tests: the in-file fixture, the recording sender, AAA markers, literal expected values, the four duplicate event tests as one IPv4 and IPv6 table (dropping the misnamed module), `MockTorrentMetricsStore` instead of a database, and no `mock!` double. | Focused tests, `cargo +nightly fmt --all -- --check`, `git diff --check`, prose-first AAA review. Maintainer review of the result. |
| R2 | DONE | Add R2a-R2g. | Same validation; the named mutations M1-M4 each make their test fail. |
| R3 | DONE | Select an integration increment, or `none`. | `none`: the service has no network boundary of its own, and every decision is observable at its unit API with in-memory collaborators. |
| R4 | DONE | Re-run unit-only coverage; record before and after. | Results, below. |

## Named Mutations

Each is a temporary hand-made change that its test must catch; it is reverted after the run and recorded in
Results. Lines are those of `services/scrape.rs` at the S2 baseline, which the test changes do not move. The
workspace denies `unused` and `warnings`, so a mutation that leaves code unused does not compile and proves
nothing: run it with `RUSTFLAGS="--cap-lints=warn"` only when the plain run fails to compile, and count it
only when its test runs and fails.

| ID | Test | Site | Temporary change |
| --- | --- | --- | --- |
| M1 | R2a (public tracker) | `handle_scrape`, line 109 | Make the authentication check unconditional: `(self.authentication_is_required() \|\| true) &&`, so every unauthenticated peer is zeroed, also on a public tracker. Removing the call outright does not compile (`unused`). |
| M2 | R2a (unregistered and registered key) | `is_authenticated`, line 128 | Return `... .is_ok() \|\| true` (any key authenticates); and `... .is_ok() && false` (no key does). |
| M3 | R2e | `handle_scrape`, lines 109-115 | Move the `resolve_remote_client_addr` call above the scrape. |
| M4 | R2f (public URL) | `send_event`, line 138 | Delete `.with_public_url(self.public_url.clone())`; needs `--cap-lints=warn` (the field becomes unread). |

Gates: this plan needs approval first (including the R0 helper); R1 needs the maintainer's review of its
result; the completed file needs the maintainer's completed-file review before its one signed commit.

## Prose-First AAA Review

Temporary prose, written before comparing with the code. The fixture builds one scenario's collaborators (no
database, listener or statistics repository); every event assertion compares the whole recorded event list
with literal expected values; the connection port (54,321) is a constant the tests choose, so it is not
confused with any other port.

| Test (cases) | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_return_the_scrape_data_for_a_torrent` | A public tracker with one seeder announced for the sample torrent, no sender. | Scrape that torrent. | One file: one complete peer, no downloads, no incomplete peer, written as a literal. | Matches the code. R1; the old test had no markers and an unused event bus. |
| `it_should_return_the_persisted_downloads_of_a_torrent_absent_from_memory` | A tracker whose persisted-downloads store holds 7 downloads for the sample torrent, which has no swarm. | Scrape that torrent. | One file: no peers and 7 downloads. | Matches the code. R1; a `MockTorrentMetricsStore` replaces the SQLite database (collaboration test: the rule is `tracker-core`'s). |
| `it_should_zero_every_requested_torrent_for_an_unauthenticated_peer_on_a_private_tracker` | A private tracker; the sample torrent has a seeder and a second requested torrent is unknown; no key. | Scrape both. | Both files are zeroed, written as literals. | Matches the code. R1/R2b; mutations M1 and P1 fail it. |
| `it_should_zero_the_scrape_data_for_an_unregistered_key_on_a_private_tracker` | A private tracker with a seeder and no registered key; a random key is presented. | Scrape the torrent. | The file is zeroed. | Matches the code. R2a; mutation M2 fails it. |
| `it_should_return_the_scrape_data_for_a_registered_key_on_a_private_tracker` | A private tracker with a seeder and one registered key; that key is presented. | Scrape the torrent. | The file shows the seeder. | Matches the code. R2a (collaboration: key validity is `tracker-core`'s); mutation M2b fails it. |
| `it_should_publish_the_connection_address_of_the_client_in_the_scrape_event` (IPv4 client, IPv6 client) | A public tracker, a recording sender, a client at the case's address. | Scrape the sample torrent. | Exactly one event, with that address as the remote client, the connection port and the server binding. | Matches the code. R1; one table replaces four duplicate tests. |
| `it_should_use_the_http_tracker_reverse_proxy_policy` | An HTTP tracker behind a reverse proxy, a forwarded address that differs from the connection address, a recording sender. | Scrape the sample torrent. | One event whose remote client is the forwarded address, with the connection port. | Matches the code. R1; probe P2 fails it. |
| `it_should_publish_the_public_url_of_the_http_tracker_in_the_scrape_event` | An HTTP tracker configured with a public URL, a recording sender. | Scrape the sample torrent. | One event whose connection context carries that URL as text. | Matches the code. R2f; mutation M4 fails it. |
| `it_should_scrape_without_publishing_when_there_is_no_event_sender` | A public tracker and no sender. | Scrape the sample torrent. | The scrape succeeds. | Matches the code. R2f. |
| `it_should_report_a_failing_persisted_downloads_lookup_as_a_tracker_core_scrape_error` | A tracker whose persisted-downloads store fails, a recording sender. | Scrape a torrent absent from memory. | The tracker-core scrape failure wrapping a database error; no event. | Matches the code. R2c (collaboration, no database). |
| `it_should_report_an_unresolvable_client_ip_without_publishing_an_event` (without a reverse proxy, behind a reverse proxy) | An HTTP tracker of the case's kind, client IP sources that name no address, a recording sender. | Scrape the sample torrent. | The resolution failure; no event. | Matches the code. R2d. |
| `it_should_scrape_before_resolving_the_client_ip` | A failing persisted-downloads store and client IP sources that name no address. | Scrape. | The tracker-core failure, not the resolution failure; no event. | Matches the code. R2e; mutation M3 fails it. |
| `it_should_wrap_a_whitelist_error_in_the_tracker_core_variant` | A not-whitelisted error. | Convert it into `HttpScrapeError`. | The tracker-core variant wrapping a whitelist error. | Matches the code. R2g; a public conversion with no caller in the repository. |
| `it_should_wrap_an_authentication_error_in_the_tracker_core_variant` | A missing-key error. | Convert it. | The tracker-core variant wrapping an authentication error. | Matches the code. R2g; same remark. |
| `it_should_wrap_a_tracker_core_error_without_changing_it` | A tracker-core scrape error. | Convert it. | The tracker-core variant, still a scrape error. | Matches the code. R2g; same remark. |
| `it_should_report_a_peer_ip_resolution_error_with_the_protocol_failure_reason` | A resolution error and the protocol crate's own response for it. | Convert it through `HttpScrapeError`. | The failure reason equals the protocol crate's. | Matches the code. R2g; delegation, not text (`http-protocol`'s). The expectation is derived from that crate on purpose. |
| `it_should_report_a_tracker_core_error_with_the_tracker_core_failure_reason` | A tracker-core error and `error_mapping.rs`'s own response for it. | Convert it through `HttpScrapeError`. | The failure reason equals that response's. | Matches the code. R2g; delegation, the text is pinned once in `error_mapping.rs`. |

## Results

### R0-R4 (2026-10-20, stable Rust 1.99.0, nightly `rustfmt`)

- `cargo test -p torrust-tracker-http-core`: 90 unit tests pass and the 3 doc tests pass. The file's 8 tests
  are now 19 cases (17 functions); the announce and `error_mapping.rs` tests are unchanged (the announce
  tests only import `RecordingSender` from `crate::tests` now). `cargo +nightly fmt --all -- --check`,
  `cargo clippy -p torrust-tracker-http-core --all-targets --all-features -- -D warnings`,
  `cargo machete --with-metadata` and `git diff --check` pass. A comparison of everything before
  `#[cfg(test)]` with `HEAD` shows production code identical.
- `crate::tests::RecordingSender` (R0) is the announce tests' sender moved unchanged; `mockall` stays a
  dev-dependency because the Criterion bench helpers use it.
- Mutations (each applied by hand, run, and reverted):

| Mutation | Result |
| --- | --- |
| M1: authentication always required | Fails four tests that scrape a public tracker without a key: the scrape data, the persisted downloads, the failing store and the order test. |
| M2: any key authenticates | Fails `it_should_zero_the_scrape_data_for_an_unregistered_key_on_a_private_tracker` only. |
| M2b: no key authenticates | Fails `it_should_return_the_scrape_data_for_a_registered_key_on_a_private_tracker` only. |
| M3: resolve the client IP before scraping | Fails `it_should_scrape_before_resolving_the_client_ip` only. |
| M4: delete `with_public_url` | Does not compile by default (`field public_url is never read`). With `RUSTFLAGS="--cap-lints=warn"`, fails `it_should_publish_the_public_url_of_the_http_tracker_in_the_scrape_event` only. |
| P1 (probe): private zeroing returns an empty set | Fails the two zeroing tests. |
| P2 (probe): the HTTP tracker config's reverse-proxy mode is ignored | Fails `it_should_use_the_http_tracker_reverse_proxy_policy` only. |

- The planned test of a public tracker that ignores a presented key was dropped: no mutation of this file
  can fail it, because `AuthenticationService::authenticate` accepts any key on a public tracker. It would
  only repeat the plain public scrape test.
- Conversions: `From<ScrapeError>` and `From<PeerIpResolutionError>` are reached through `handle_scrape`.
  `From<WhitelistError>`, `From<authentication::key::Error>` and `From<TrackerCoreError>` have no caller in
  the repository; they are public conversions and are pinned by one test each, not filed as findings (as for
  `HttpAnnounceError` in S2). Removing them would be a breaking API change.
- No bug was found, so no hand-off was needed.

Unit-only coverage of the file (`cargo llvm-cov clean --workspace`, then `cargo llvm-cov -p
torrust-tracker-http-core --all-features --lib`, `cargo-llvm-cov 0.6.16`, stable Rust 1.99.0):

| Scope | Before (S2 baseline) | After |
| --- | ---: | ---: |
| `services/scrape.rs` lines | 507 / 540 (93.89%) | 488 / 492 (99.19%) |
| `services/scrape.rs` regions | 678 / 711 (95.36%) | 659 / 664 (99.25%) |
| `services/scrape.rs` functions | 36 / 43 (83.72%) | 66 / 66 (100.00%) |

The file's counted lines fell by 48 because the refactored test module has fewer instrumented lines than
the 8 old tests (four duplicate event tests became one table, and the fixture shrank).
Every production line, region and function of the file is covered: the four uncovered lines and five
uncovered regions are the failure branches of `matches!` assertions in the test module (lines 705, 785, 807
and 827, plus 766). The package totals after slice S3 are in the
[coverage evidence](../coverage-evidence.md#package-after-slice-s3).

## Approval

Approved by Jose Celano on 2026-10-20 (16:54 UTC): R0 (the shared `RecordingSender`), R1, R2a-R2g, R3 `none` and mutations M1-M4. The three conversions with no repository caller are pinned as public API, not filed as findings.

## Progress Log

- 2026-10-20 16:52 UTC - GitHub Copilot - Created the plan from `services/scrape.rs`, the S2 baseline and the
  announce plan. No Rust test or production code changed. The plan awaits maintainer approval, including
  the shared `RecordingSender` helper.
- 2026-10-20 16:54 UTC - Jose Celano - Approved the plan, including the shared helper. R0 starts.
- 2026-10-20 17:05 UTC - GitHub Copilot - R0-R4 done: 19 cases, mutations M1-M4 and probes P1 and P2 each
  fail only the tests named for them, and the file is at 488 / 492 lines with only `matches!` failure
  branches of the test module uncovered. R1 and R2 were coded together; the planned test of a public
  tracker ignoring a presented key was dropped because no mutation can fail it (see Results). The
  completed file awaits the maintainer's review.
- 2026-10-20 17:09 UTC - Jose Celano - Approved R1 and the completed file. The plan is `done`.
