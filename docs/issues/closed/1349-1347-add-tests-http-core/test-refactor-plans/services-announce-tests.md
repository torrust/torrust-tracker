---
doc-type: file-test-plan
issue: 1349
package: torrust-tracker-http-core
target-file: packages/http-core/src/services/announce.rs
status: done
semantic-links:
  related-artifacts:
    - packages/http-core/src/services/announce.rs
    - packages/http-core/src/services/error_mapping.rs
    - packages/tracker-core/src/databases/traits/torrent_metrics.rs
    - docs/issues/closed/1349-1347-add-tests-http-core/coverage-evidence.md
---

# HTTP Core Announce Service File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/http-core/src/services/announce.rs`. The maintainer approved this plan on 2026-10-16 and
confirmed every later amendment, listed in
[Post-Approval Amendments](#post-approval-amendments), at the R1 review and the completed-file
review on 2026-10-20. The plan is done.

## Post-Approval Amendments

The approval below covers R1, R2a-R2j, the fixture approach and `rstest`. These later changes were
made in response to review findings or during implementation. Each was confirmed by the maintainer at
the review of the step it touches, and each is also in the Progress Log.

| Date (UTC) | Amendment | Touches | Confirmed |
| --- | --- | --- | --- |
| 2026-10-17 08:22 | Rename or reconfigure the tcp6 event test so its name matches its Arrange | R1 | Confirmed 2026-10-20 12:43 (R1 review) |
| 2026-10-17 09:49 | Give the precedence test a policy with overrides enabled and a distinct external IP | R1 | Confirmed 2026-10-20 12:43 (R1 review) |
| 2026-10-19 11:23 | Add R2k (the instance public URL in the announce event); R2 reads R2a-R2k | R2 | Confirmed 2026-10-20 12:57 (completed-file review) |
| 2026-10-19 11:24 | Add the Named Mutations table (M1-M5) and point R1, R2 and the order row at it | R1, R2 | R1 confirmed 2026-10-20 12:43; R2 confirmed 2026-10-20 12:57 (completed-file review) |
| 2026-10-19 12:44 | Run M5 with `RUSTFLAGS="--cap-lints=warn"` and count it only when R2k runs and fails | R2 | Confirmed 2026-10-20 12:57 (completed-file review) |
| 2026-10-20 12:40 | R1 keeps the plain IPv6 case under an accurate name and adds a case that matches the old tcp6 name (an IPv6 loopback client with an IPv4 external IP). That case also covers R2e's "IPv6 loopback is substituted", so R2e keeps only the non-loopback case. | R1, R2e | Confirmed 2026-10-20 12:43 (R1 review) |
| 2026-10-20 12:41 | The announce tests record events with an in-file recording sender and assert on them, instead of a `mockall` double with a predicate. R2d proves "no event was sent" with the recorded list being empty, not with an expectation-free double. | R1, R2d | Confirmed 2026-10-20 12:43 (R1 review) |

Update the Confirmed column when the maintainer confirms an amendment.

## Current State

- **Unit-only coverage:** 675 / 727 lines (92.85%), 878 / 938 regions, 56 / 63 functions
  ([coverage evidence](../coverage-evidence.md), `87690b15e`). The totals include the test module.
- **Existing tests:** 8 in the file (2 synchronous peer-IP decision tests and 3 asynchronous service
  tests, plus 3 event tests). `axum-http-server` exercises the service through HTTP, which EPIC #1347 does not accept as
  evidence for this package's unit seam.
- **Module-owned decisions:** the peer-IP selection under the instance policy (override on or off,
  `ip` literal, DNS name, invalid value, loopback substitution by the configured external IP);
  private-mode authentication; the order authenticate, authorize, resolve client IP, select peer
  IP, announce; the protocol-to-domain announcement mapping; `numwant` to `PeersWanted`; event
  publication with and without a sender, carrying the instance's public URL; and the failure reason
  of every `HttpAnnounceError`.
- **T1 hypothesis:** property candidate.

## Current Tests Review

Reviewed against `write-unit-test`. Conclusion: **refactor**. Behavior is right; the smells make the
suite hard to extend, and the new tests need a smaller fixture.

| Test | Smells found |
| --- | --- |
| `it_should_select_the_connection_address_for_absent_or_empty_peer_ip` | Clean. Loops over two inputs with one assertion; the loop is data, not logic. |
| `it_should_reject_or_select_non_empty_peer_ip_according_to_policy` | Logic in the test: a `match` re-derives the expected result from the input, with an `unreachable!()` arm. Replace with a table of independently written expectations. |
| `it_should_return_the_announce_data` | No `// Arrange`, `// Act`, `// Assert` structure; the expected `policy` is read from the fixture used by the Act. |
| `it_should_use_the_http_tracker_policy_for_query_string_and_reverse_proxy_ips` | Complex Arrange (over 50 lines); the expected event is built inside a mock predicate, and the assertion is only `result.is_ok()`. |
| `it_should_prefer_a_query_string_peer_ip_over_the_external_ip_for_a_loopback_client_...` | Same predicate-built expectation and `is_ok()` assertion. It also cannot prove its name: it builds the service with `PeerIpSelectionPolicy::enabled()`, whose `external_ip` is `None`, so the external IP configured in its fixture never reaches the service, and a service that wrongly preferred the external IP would still pass. |
| the three `it_should_send_the_tcp_{4,6}_announce_event_...` tests | Three near-identical 40-70 line bodies differing in two inputs; none has AAA markers or an explicit assertion (the mock verifies on drop), and the expected event is built inside a mock predicate. The tcp6 test's name says the tracker changes the peer IP to IPv4, but it uses the default configuration with no external IP, so the peer IP is never changed and the name describes an Arrange the test does not set up; R1 renames it or gives it the configuration its name claims. |

The shared fixture `initialize_core_tracker_services_with_config` also creates a real database,
a statistics repository and bus, and (when usage statistics are on) spawns an event listener whose
handle is discarded, although no announce test reads the statistics. The new tests need only the configuration, the in-memory key
repository, the whitelist, the torrent repository, and the announce handler.

## Coverage and Decision Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Private mode: no key, an unknown key, and a valid key (lines 307-311) | Collaboration | R2a: the `private` flag selects the check and a missing or rejected key returns the authentication failure; key validity is `tracker-core`'s, so the valid-key case only shows that the service proceeds. |
| Public mode ignores a presented key | 3A unit | R2b: a public tracker with an arbitrary key still announces; the flag is the one Arrange difference. |
| Listed mode rejects a torrent that is not whitelisted | Collaboration | R2c: `TorrentNotWhitelisted` surfaces as the whitelist failure. |
| Order of the checks (regions at lines 211, 213, 215, 217, 226) | 3A unit | R2d: with several failures present at once, the earliest check's error is returned, no event is sent (an expectation-free sender double), and no peer is registered. Confirmed by mutation M2. |
| Non-loopback connection keeps its address when an external IP is configured; IPv6 loopback is substituted | 3A unit | R2e: the substitution applies only to loopback connections. This is untested today and is a spoofing-relevant decision. |
| Request-to-announcement mapping (lines 260-262) | 3A unit | R2f: every event variant and an absent event, absent byte counters defaulting to zero, the port, and the stopped-clock timestamp, against independently written expected values. |
| `numwant` to `PeersWanted` | 3A unit | R2g: absent gives as many as possible; a number gives that limit (`PeersWanted` is `PartialEq`). |
| Absent event sender (line 350) | 3A unit | R2h: an announce without a sender succeeds. |
| Instance public URL in the announce event (lines 74, 192, 342) | 3A unit | R2k: an instance configured with a public URL publishes it in the event's connection context. `HttpTrackerPolicy::from(&HttpTracker)` maps it, the service stores it, and `send_event` attaches it with `with_public_url`; `announce_events_match` compares it through `ConnectionContext`'s `PartialEq`. Every existing announce test runs with no URL, so dropping that call, or mapping the URL to `None`, passes all eight tests today. |
| Failure reasons of the package-owned variants (lines 420-430) | 3A unit | R2i: the text of `PeerIpOverrideDisabled`, `PeerIpDnsNameUnsupported`, and `PeerIpInvalid`. `PeerIpResolutionError` text is `http-protocol`'s and `TrackerCoreError` text is the `error_mapping.rs` plan's. |
| Tracker-core announce failure (region at line 226) | Collaboration | R2j: `tracker-core` exports `MockTorrentMetricsStore`, so a failing persisted-downloads lookup reaches `TrackerCoreError::AnnounceError` without a database (#1348 lesson). |
| `From` conversions (lines 380-416) | Covered by R2a, R2c, R2d, R2j | A conversion no test reaches is a finding candidate (dead code); record it in `lessons.md`. |
| Debug logging in `send_event` | logging | Not tested. |
| Peer-IP selection property | Assess after R2 | Apply hand-made mutations to `select_peer_ip_with_policy` and `external_ip_for_loopback_connection`. Add a property only if a mutation survives the examples (the udp-core cookie did); otherwise record that examples suffice and add no dependency. |
| Announce rules, whitelist policy, key validity, peer selection | Collaborator-owned | `tracker-core`; no test here. |

Test bootstrap: an in-file fixture that constructs only the collaborators each scenario needs, with
named scenario constructors for the one state difference (private, listed, with an external IP,
with a failing downloads store). The fixture keeps the torrent repository for state assertions and
builds the request with a small in-file builder. The production container is not used.

**Decision (approved 2026-10-16).** Table-driven tests use `rstest`, which is not yet a
dev-dependency of this package (the workspace uses `rstest = "0"` elsewhere). Adding it follows the
`add-rust-dependency` skill and happens in S2, with the version rationale recorded.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Refactor the existing tests: replace the `match` in the non-empty-IP test with a data table, give the three event tests one scenario fixture, add AAA structure and explicit assertions, rename or reconfigure the tcp6 event test so its name matches its Arrange, give the precedence test a policy that enables overrides and carries a distinct external IP (and confirm it fails under mutation M1), and shrink the fixture. | Focused tests, `cargo +nightly fmt --all -- --check`, `git diff --check`, prose-first AAA review. Code done on 2026-10-20 (see Results); the maintainer approved the result on 2026-10-20 12:43 UTC. |
| R2 | DONE | Add R2a-R2k. | Same validation; the named mutations M2-M5 each make their test fail. Done on 2026-10-20; the maintainer approved the completed file on 2026-10-20 12:57 UTC (see Results). |
| R3 | DONE | Select an integration increment, or `none`. | `none`: the service has no network boundary of its own, and every decision is observable at its unit API with in-memory collaborators. `axum-http-server` covers transport. |
| R4 | DONE | Re-run unit-only coverage; record before and after. | Results, below. `coverage-evidence.md` is updated with the completed-file review. |

## Named Mutations

Each proof is a temporary, hand-made change to production code that the named test must catch; it is
reverted after the check, and the result is recorded in the Results section. Lines are those of
`services/announce.rs` at `87690b15e`, which the test changes do not move. The lines of M1-M5 are unchanged since `a3810b813`: `#[non_exhaustive]` (#2454) was added after them.

The workspace denies `unused` and `warnings`, so a mutation that leaves code unused fails to compile
and proves nothing. Compile-checked on 2026-10-19: M1-M4 compile under the default lints; M5 does not,
because deleting the call removes the only read of the private `public_url` field. Run M5 with
`RUSTFLAGS="--cap-lints=warn"` (as T6 does), and count it only when R2k runs and fails; a compile
error is not a caught mutant. Run a new mutation without the cap first, and use the cap only when
that fails to compile.

| ID | Test | Site | Temporary change |
| --- | --- | --- | --- |
| M1 | R1 precedence test | `select_peer_ip_with_policy`, the `PeerIp::Literal(ip)` arm (line 288) | Return `peer_ip_selection_policy.external_ip.filter(\|_\| connection_peer_ip.is_loopback()).unwrap_or(*ip)` instead of `*ip`, so a loopback connection prefers the external IP over the literal. |
| M2 | R2d order | `handle_announce`, lines 211 and 213 (and 215) | Swap `self.authenticate(maybe_key).await?;` and `self.authorize(announce_request.info_hash).await?;`; repeat with the `resolve_remote_client_addr` call moved above `authenticate`. |
| M3 | R2e loopback | `external_ip_for_loopback_connection`, the `is_loopback()` guard (line 298) | Remove the guard so any connection takes the configured external IP. |
| M4 | R2f mapping | `peer_from_request`, the `ProtocolAnnounceEvent::Stopped` arm (line 260) | Map `Stopped` to `AnnounceEvent::None`. |
| M5 | R2k public URL | `send_event`, the `with_public_url` call (line 342) | With `RUSTFLAGS="--cap-lints=warn"`, delete `.with_public_url(self.public_url.clone())`. |

Gates: this plan needs approval first; R1 needs maintainer review of its result; the completed
file needs the maintainer's completed-file review before its one signed commit.

## Prose-First AAA Review

Temporary prose for each changed test, written before comparing with the code. R2 adds its rows. The
fixture builds the collaborators of one scenario (no database, listener or statistics repository);
`freeze_the_clock` makes the expected `updated` a literal; the connection port (54,321) differs from
the announced port (8080), so a mix-up between them fails.

| Test (cases) | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_select_the_connection_address_for_an_absent_or_empty_peer_ip` (absent, empty) | A request whose `ip` is absent or empty, a connection from loopback, overrides enabled, no external IP. | Select the peer IP. | The connection address is selected. | Matches the code. Unchanged behavior; the loop became a table. |
| `it_should_select_a_literal_peer_ip_when_the_policy_allows_the_override` (IPv4, IPv6) | A request carrying a literal `ip`, overrides enabled. | Select the peer IP. | The literal is selected. | Matches the code. Replaces the `match` that re-derived expectations. |
| `it_should_reject_a_literal_peer_ip_when_the_policy_disables_the_override` (IPv4, IPv6) | The same request, overrides disabled. | Select the peer IP. | The override-disabled error is returned. | Matches the code. |
| `it_should_reject_a_dns_name_peer_ip_under_any_override_policy` (enabled, disabled) | A request whose `ip` is a DNS name. | Select the peer IP. | The DNS-name error, whatever the policy. | Matches the code. |
| `it_should_reject_an_invalid_peer_ip_under_any_override_policy` (enabled, disabled) | A request whose `ip` is invalid. | Select the peer IP. | The invalid-IP error, whatever the policy. | Matches the code. |
| `it_should_return_the_announce_data` | A public tracker configured with a non-default announce policy (1800 s, 900 s, 50 peers), one started seeder announcing from loopback, no sender. | Handle the announce. | The data holds no peers, one complete and no incomplete peer, and the configured policy written as a literal. | Matches the code. The old expectation read the policy from the fixture the Act used. |
| `it_should_use_the_http_tracker_policy_for_query_string_and_reverse_proxy_ips` | An HTTP tracker behind a reverse proxy that accepts the query-string `ip`; a request with a literal `ip`; a forwarded address and a different connection address; a recording sender; a frozen clock. | Handle the announce. | The announce succeeds, and exactly one event carries the forwarded address as the remote client, the connection port, the literal peer IP and the frozen timestamp. | Matches the code. The old assertion was `is_ok()` plus a mock predicate. |
| `it_should_prefer_a_query_string_peer_ip_over_the_external_ip_for_a_loopback_client_when_overrides_are_enabled` | A policy that enables overrides **and** carries a distinct external IP, a loopback client, a literal `ip`, a recording sender. | Handle the announce. | The event announces the literal IP, not the external IP. | Matches the code. Mutation M1 fails it (Results). The old test never gave the service an external IP. |
| `it_should_publish_the_connection_address_and_the_selected_peer_ip_in_the_announce_event` (IPv4 client, IPv6 client, IPv4 loopback with an IPv6 external IP, IPv6 loopback with an IPv4 external IP) | A tracker with or without an external IP, a client at the case's connection address, a recording sender. | Handle an announce with no `ip`. | Exactly one event carries the connection address as the remote client and the case's peer IP: the external IP only for a loopback client. | Matches the code. One table replaces three 40-70 line bodies. |
| `it_should_not_replace_a_loopback_literal_peer_ip_with_the_external_ip` | Overrides enabled, an external IP configured, the client supplies the loopback literal `127.0.0.2`. | Select the peer IP. | The literal is selected, not the external IP. | Matches the code. Added after mutant P1 survived (Results). |
| `it_should_replace_a_loopback_connection_address_with_the_external_ip_for_an_absent_or_empty_peer_ip` (absent, empty) | A tracker with an external IP, a request with an absent or an empty `ip`, a loopback connection. | Select the peer IP. | The external IP is selected, for both. | Matches the code. Added after the S2 Task Reviewer's mutant P2 survived: an empty `ip=` must get the same loopback substitution as an absent one. |
| `it_should_keep_the_connection_address_of_a_non_loopback_client_when_an_external_ip_is_configured` (IPv4, IPv6) | A tracker with an external IP, a request with no `ip`, a non-loopback connection. | Select the peer IP. | The connection address is kept. | Matches the code. R2e; mutation M3 fails both cases. |
| `it_should_reject_an_announce_without_a_key_on_a_private_tracker` | A private tracker, no key. | Handle the announce. | The authentication failure for a missing key; no event; no peer registered. | Matches the code. R2a. |
| `it_should_reject_an_announce_with_an_unregistered_key_on_a_private_tracker` | A private tracker with no registered key, a random key presented. | Handle the announce. | The authentication failure for an unreadable key; no event; no peer. | Matches the code. R2a. Key validity is `tracker-core`'s; the test shows only that the service surfaces the failure. |
| `it_should_accept_an_announce_with_a_registered_key_on_a_private_tracker` | A private tracker with one registered key, that key presented. | Handle the announce. | The announce succeeds, the peer is registered, one event is published. | Matches the code. R2a (collaboration). |
| `it_should_ignore_a_presented_key_on_a_public_tracker` | A public tracker, a random key presented. | Handle the announce. | The announce succeeds, the peer is registered, one event is published. | Matches the code. R2b. |
| `it_should_reject_a_torrent_that_is_not_whitelisted_on_a_listed_tracker` | A listed tracker with an empty whitelist. | Handle the announce. | The whitelist failure; no event; no peer. | Matches the code. R2c (collaboration). |
| `it_should_authenticate_before_authorizing_the_torrent` | A private and listed tracker; the key is missing and the torrent is not whitelisted. | Handle the announce. | The authentication failure, not the whitelist failure; nothing announced. | Matches the code. R2d; mutation M2a fails it. |
| `it_should_authenticate_before_resolving_the_client_ip` | A private tracker; the key is missing and no client address is available. | Handle the announce. | The authentication failure; nothing announced. | Matches the code. R2d; mutation M2b fails it. |
| `it_should_authorize_the_torrent_before_resolving_the_client_ip` | A listed tracker, the torrent is not whitelisted and no client address is available. | Handle the announce. | The whitelist failure; nothing announced. | Matches the code. R2d; mutations M2b and M2c fail it. |
| `it_should_resolve_the_client_ip_before_selecting_the_peer_ip` | A public tracker, no client address, and an `ip` that is a DNS name. | Handle the announce. | The resolution failure, not the DNS-name failure; nothing announced. | Matches the code. R2d. |
| `it_should_select_the_peer_ip_before_announcing` | A public tracker that rejects a literal `ip`, and a request that carries one. | Handle the announce. | The override-disabled failure; no event, no peer registered. | Matches the code. R2d: the announce handler never ran. |
| `it_should_map_the_announce_event_to_the_announcement_event` (started, stopped, completed, empty, absent) | A request with the case's protocol event. | Map the request to an announcement. | The case's announcement event; an empty or absent event is `None`. | Matches the code. R2f; mutation M4 fails the `stopped` case. |
| `it_should_default_absent_byte_counters_to_zero_in_the_announcement` | A request with no byte counters. | Map the request. | Uploaded, downloaded and left are all zero. | Matches the code. R2f. |
| `it_should_copy_the_peer_id_address_byte_counters_and_time_into_the_announcement` | A request with counters 1, 2 and 3, a frozen clock, a peer IP. | Map the request. | One announcement equal to a literal: the sample peer id, the peer IP and announced port, the frozen time, the counters and the started event. | Matches the code. R2f. |
| `it_should_translate_numwant_into_the_number_of_peers_wanted` (absent, a number) | A request with the case's `numwant`. | Compute the peers wanted. | As many as possible when absent; otherwise exactly that number. | Matches the code. R2g. |
| `it_should_announce_without_publishing_when_there_is_no_event_sender` | A public tracker and no sender. | Handle the announce. | The announce succeeds and the peer is registered. | Matches the code. R2h. |
| `it_should_report_a_failing_persisted_downloads_lookup_as_a_tracker_core_announce_error` | A tracker whose persisted-downloads store fails on lookup, a torrent not yet in memory. | Handle the announce. | The tracker-core announce failure wrapping a database error; nothing announced. | Matches the code. R2j (collaboration, `MockTorrentMetricsStore`, no database). |
| `it_should_publish_the_public_url_of_the_http_tracker_in_the_announce_event` | An HTTP tracker configured with a public URL, a frozen clock. | Handle the announce. | One event whose connection context carries that URL as text. | Matches the code. R2k; mutations M5 (with `--cap-lints=warn`) and the URL mapped to `None` fail it. |
| `it_should_report_a_rejected_peer_ip_with_its_failure_reason` (override disabled, DNS name, invalid) | One of the three package-owned errors. | Convert it to a protocol response. | The failure reason is the literal client-visible text. | Matches the code. R2i. |
| `it_should_report_a_peer_ip_resolution_error_with_the_protocol_failure_reason` | A resolution error. | Convert it through `HttpAnnounceError`. | The reason equals the protocol crate's own conversion of the same error. | Matches the code. R2i. It checks delegation, not text, which is `http-protocol`'s; the expectation is derived from that crate on purpose. |
| `it_should_report_a_tracker_core_error_with_the_tracker_core_failure_reason` | A tracker-core error, and the protocol response that `error_mapping.rs` gives it. | Convert it through `HttpAnnounceError`. | The failure reason equals that response's. | Matches the code. R2i. It checks only delegation: the text is pinned once, in `error_mapping.rs`. The expectation is derived from that function on purpose. |
| `it_should_wrap_a_tracker_core_error_without_changing_it` | A tracker-core error. | Convert it into `HttpAnnounceError`. | It is the tracker-core variant, still holding the same error kind. | Matches the code. The only conversion no other test reaches; it is a public conversion with no caller in the repository (Results). |

## Results

### R1 (2026-10-20, stable Rust 1.99.0, nightly `rustfmt`)

- `cargo test -p torrust-tracker-http-core --lib`: 42 passed. The announce tests are 9 functions that
  run as 17 cases, where the file had 8 tests. The package had 33 tests before; the other 25 are
  unchanged.
- `cargo +nightly fmt --all -- --check`, `cargo clippy -p torrust-tracker-http-core --all-targets
  --all-features -- -D warnings`, `cargo machete --with-metadata` and `git diff --check` pass.
- Production code did not change. The diff starts inside the test module.
- Mutation M1 (the `PeerIp::Literal` arm prefers the external IP for a loopback connection): exactly
  one test failed, `it_should_prefer_a_query_string_peer_ip_over_the_external_ip_for_a_loopback_client_when_overrides_are_enabled`;
  the other 16 cases passed. The change was reverted.
- Dependency: `rstest = "0.28.0"` is a dev-dependency of `torrust-tracker-http-core` (the latest stable
  version on crates.io on 2026-10-20). The workspace uses `rstest = "0"` elsewhere, which would have
  locked 0.27.0 here because Cargo reuses an existing compatible lock entry; the exact requirement
  gives this package the latest version. It is used only for the case tables; `cargo machete` passes.
- The fixture no longer creates a database or an event listener, and the `mock!` double of the
  announce tests is gone. `crate::event::test::announce_events_match` has no caller in the announce
  tests now; the `event.rs` plan (S4) decides whether to remove it.

### R2-R4 (2026-10-20, stable Rust 1.99.0, nightly `rustfmt`)

- `cargo test -p torrust-tracker-http-core`: 79 unit tests pass (the file's 8 tests are now 50
  cases; the 4 `error_mapping.rs` cases and the other 25 tests are unchanged), and the 3 doc tests
  pass. `cargo +nightly fmt --all -- --check`, `cargo clippy -p torrust-tracker-http-core
  --all-targets --all-features -- -D warnings` and `git diff --check` pass. A comparison of everything
  before `#[cfg(test)]` with `HEAD` shows production code identical. The file had 48 cases at the
  completed-file review; two more were added after the S2 Task Reviewer found a surviving mutant
  (see the property assessment).
- Named mutations (each applied by hand, run, and reverted):

| Mutation | Result |
| --- | --- |
| M1 (R1, above) | Fails exactly the precedence test. |
| M2a: authorize before authenticate | Fails `it_should_authenticate_before_authorizing_the_torrent` only. |
| M2b: resolve the client IP before authenticating | Fails `it_should_authenticate_before_resolving_the_client_ip` and `it_should_authorize_the_torrent_before_resolving_the_client_ip`. |
| M2c: resolve the client IP between authenticate and authorize | Fails `it_should_authorize_the_torrent_before_resolving_the_client_ip` only. |
| M3: remove the `is_loopback()` guard | Fails both cases of `it_should_keep_the_connection_address_of_a_non_loopback_client_when_an_external_ip_is_configured`. |
| M4: map `Stopped` to `None` | Fails the `stopped` case of `it_should_map_the_announce_event_to_the_announcement_event` only. |
| M5: delete `with_public_url` | Does not compile by default (`field public_url is never read`). With `RUSTFLAGS="--cap-lints=warn"`, fails `it_should_publish_the_public_url_of_the_http_tracker_in_the_announce_event` only. |
| M5b: map the URL to `None` in `HttpTrackerPolicy::from` | Fails the same test only. |
| P1: the `PeerIp::Literal` arm routes the literal through `external_ip_for_loopback_connection` | Survived the first examples; killed by `it_should_not_replace_a_loopback_literal_peer_ip_with_the_external_ip` only (see below). |
| P2: `PeerIp::Empty` returns the connection address, skipping the external-IP substitution | Survived all 77 tests (found by the S2 Task Reviewer); killed by the `empty` case of `it_should_replace_a_loopback_connection_address_with_the_external_ip_for_an_absent_or_empty_peer_ip` only. |

- Peer-IP property assessment: besides M1 and M3, two mutants of `select_peer_ip_with_policy`
  **survived** the examples and were each fixed with one example, so a property test is still not
  needed. P1 (the `PeerIp::Literal` arm routes the literal through
  `external_ip_for_loopback_connection`, so a client-supplied loopback literal is replaced by the
  external IP) was found by my own probe and fixed by
  `it_should_not_replace_a_loopback_literal_peer_ip_with_the_external_ip`. P2 (an empty `ip=` skips
  the loopback substitution that an absent `ip` gets) was found by the S2 Task Reviewer, and my
  claim that no mutation survived the examples was wrong until then; the two-case test
  `it_should_replace_a_loopback_connection_address_with_the_external_ip_for_an_absent_or_empty_peer_ip`
  fixed it. Both mutants are a small input-by-state grid that examples cover; the dependency stays
  out. The mutations of the selection decision that were actually run are M1, M3, P1 and P2, plus
  one the reviewer ran (passing the remote client address instead of the selected peer IP to the
  announce handler, which four tests catch); only P1 and P2 survived, and each is now caught.
- Order proofs: the five order tests assert that no event was published and no peer registered when
  an earlier check fails. A `RecordingSender` with an empty list replaces the expectation-free
  `mockall` double (amendment confirmed 2026-10-20).
- Collaborators: R2a (valid key), R2c (whitelist) and R2j (failing store) are collaboration tests;
  they show how the service surfaces `tracker-core`'s failures, not the rules behind them. R2j uses
  `MockTorrentMetricsStore`, so no database is created.
- Conversions: all five `From` conversions into `HttpAnnounceError` are reached. `From<TrackerCoreError>`
  was reached by no other test; it is a public conversion with no caller in this repository
  (`axum-http-server` uses only `HttpAnnounceError` as a return type). It is kept and pinned by one
  test. Removing it would be a breaking change to a public API, so it is not a finding for this
  issue; the maintainer may remove it in a separate change.
- `it_should_report_a_tracker_core_error_with_the_tracker_core_failure_reason` checks only that the
  announce error delegates to `error_mapping.rs`: its expectation is that function's own result for
  the same error. The text itself is pinned once, in `error_mapping.rs` (the S2 Task Reviewer noted
  that a literal prefix here would duplicate it).

Unit-only coverage of the file (`cargo llvm-cov clean --workspace`, then `cargo llvm-cov -p
torrust-tracker-http-core --all-features --lib`, `cargo-llvm-cov 0.6.16`, stable Rust 1.99.0). The
package totals after slice S2 are in the [coverage evidence](../coverage-evidence.md#per-file-results).

| Scope | Before (`87690b15e`) | After |
| --- | ---: | ---: |
| `services/announce.rs` lines | 675 / 727 (92.85%) | 843 / 851 (99.06%) |
| `services/announce.rs` regions | 878 / 938 (93.60%) | 1,039 / 1,049 (99.05%) |
| `services/announce.rs` functions | 56 / 63 (88.89%) | 92 / 92 (100.00%) |

The totals include the test module, so a file with more tests has more covered lines; the
meaningful signal is that every production decision above is pinned by a mutation. The eight
uncovered lines of `announce.rs` are the failure branches of the `matches!` assertions in the test
module (lines 1046, 1079, 1159, 1191, 1221, 1251, 1458 and 1571), and the ten uncovered regions are
those same branches (the two extra regions are at lines 1285 and 1314): test code that runs only when
a test fails. Every production line, region and function of the service is covered.

## Approval

Approved by Jose Celano on 2026-10-16 (16:02 UTC), before any amendment in [Post-Approval Amendments](#post-approval-amendments): R1, R2a-R2j, the fixture approach, and adding `rstest` as a dev-dependency of `torrust-tracker-http-core` for the table-driven tests, following the `add-rust-dependency` skill in the announce slice (S2). The peer-IP property stays conditional on the mutation check.

## Progress Log

- 2026-10-16 15:40 UTC - GitHub Copilot - Created the plan from `services/announce.rs`, the T1 coverage
  evidence and uncovered regions, and the `tracker-core` collaborators. No Rust test or production
  code changed. The plan awaits maintainer approval.
- 2026-10-16 16:02 UTC - Jose Celano - Approved the plan, including `rstest`. R1 and R2 start in S2,
  after S1 merges.
- 2026-10-17 08:22 UTC - GitHub Copilot - After the approval, the S1 Task Reviewer found that the
  tcp6 event test's name describes a peer-IP change its Arrange does not set up. The finding was
  added to the Current Tests Review and to R1. This amends the approved plan: the maintainer
  confirms the rename when reviewing the R1 result.
- 2026-10-17 09:49 UTC - GitHub Copilot - PR #2558 review finding F1 (Copilot): the precedence test
  never gives the service an external IP. Verified against `PeerIpSelectionPolicy::enabled()`, which
  sets `external_ip: None`. Added the smell to the Current Tests Review and a step to R1. This
  amends the approved plan a second time; the maintainer confirms it when reviewing the R1 result.
- 2026-10-19 11:23 UTC - GitHub Copilot - PR #2558 review finding F2 (da2ce7): the plan names no case
  for the instance public URL that the announce event carries. Verified at `announce.rs` lines 74,
  192 and 342 and in `announce_events_match`. Added the decision row and R2k, and R2 now reads
  R2a-R2k. This amends the approved plan a third time, adding one test to R2; the maintainer
  confirms it when reviewing the R2 result. The approval record above keeps its original wording.
- 2026-10-19 11:24 UTC - GitHub Copilot - PR #2558 review finding F3 (da2ce7): the temporary
  changes were not located. Added the Named Mutations table (M1-M5), checked against the source
  lines, and pointed R1, R2 and the order row at it.
- 2026-10-19 12:44 UTC - GitHub Copilot - PR #2558 review finding F5 (Copilot): mutation M5 does not
  compile under the workspace lints. Verified by applying it: `field public_url is never read`, exit
  101 by default and 0 with `--cap-lints=warn`; M1-M4 compile by default. Added the compile note and
  the cap to M5.
- 2026-10-19 16:25 UTC - GitHub Copilot - PR #2558 review finding F8 (Copilot): `status: approved` and
  the approval text read as full authorization although the plan was amended five times. Added the
  Post-Approval Amendments table (date, scope, step, confirmation) near the top and linked it from
  the approval record, whose wording is unchanged otherwise.
- 2026-10-20 12:45 UTC - GitHub Copilot - R1 coded in S2 (not committed): the existing tests were
  refactored onto an in-file fixture, a recording sender and `rstest` tables; the precedence test now
  gets an external IP and fails under M1. Two implementation choices are recorded as amendments
  (the IPv6-loopback case and the recording sender). The Prose-First AAA Review and the R1 results are
  above. Next: the maintainer's review of the R1 result, then R2.
- 2026-10-20 12:58 UTC - GitHub Copilot - R1 was approved by the maintainer at 12:43 UTC, with both
  implementation amendments. R2a-R2k, R3 (`none`) and R4 are done: 48 cases, every named mutation fails
  its test, one surviving mutant (a loopback `ip` literal) was found and fixed with one example, and
  announce coverage is 99.06% of lines. The maintainer approved the completed file at 12:57 UTC.
  The plan is `done`.
- 2026-10-20 16:05 UTC - GitHub Copilot - The S2 Task Reviewer found a surviving mutant in
  `select_peer_ip_with_policy`: an empty `ip=` that skips the loopback substitution (P2). It
  contradicted this plan's claim that no mutation survived the examples. Added a two-case test (absent
  and empty `ip`, loopback connection, external IP) that kills it, and reworked the tracker-core
  delegation test to compare with `error_mapping.rs`'s own result instead of repeating its prefix. The
  file now has 50 cases; the Prose-First AAA Review, the mutation table and the coverage numbers above
  are updated. The earlier entries keep their original wording.
