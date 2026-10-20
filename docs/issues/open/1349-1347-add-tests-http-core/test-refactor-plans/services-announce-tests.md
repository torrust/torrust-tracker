---
doc-type: file-test-plan
issue: 1349
package: torrust-tracker-http-core
target-file: packages/http-core/src/services/announce.rs
status: approved
semantic-links:
  related-artifacts:
    - packages/http-core/src/services/announce.rs
    - packages/http-core/src/services/error_mapping.rs
    - packages/tracker-core/src/databases/traits/torrent_metrics.rs
    - docs/issues/open/1349-1347-add-tests-http-core/coverage-evidence.md
---

# HTTP Core Announce Service File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/http-core/src/services/announce.rs`. The maintainer approved this plan on 2026-10-16; the
plan has since been amended in five places that await his confirmation (see
[Post-Approval Amendments](#post-approval-amendments)), so `status: approved` covers the approved
scope only.

## Post-Approval Amendments

The approval below covers R1, R2a-R2j, the fixture approach and `rstest`. These later changes were
made in response to review findings and are **pending maintainer confirmation**, at the review of
the step each one touches. Each is also in the Progress Log.

| Date (UTC) | Amendment | Touches | Confirmed |
| --- | --- | --- | --- |
| 2026-10-17 08:22 | Rename or reconfigure the tcp6 event test so its name matches its Arrange | R1 | Pending (R1 review) |
| 2026-10-17 09:49 | Give the precedence test a policy with overrides enabled and a distinct external IP | R1 | Pending (R1 review) |
| 2026-10-19 11:23 | Add R2k (the instance public URL in the announce event); R2 reads R2a-R2k | R2 | Pending (R2 review) |
| 2026-10-19 11:24 | Add the Named Mutations table (M1-M5) and point R1, R2 and the order row at it | R1, R2 | Pending (R1 and R2 reviews) |
| 2026-10-19 12:44 | Run M5 with `RUSTFLAGS="--cap-lints=warn"` and count it only when R2k runs and fails | R2 | Pending (R2 review) |

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
| R1 | TODO | Refactor the existing tests: replace the `match` in the non-empty-IP test with a data table, give the three event tests one scenario fixture, add AAA structure and explicit assertions, rename or reconfigure the tcp6 event test so its name matches its Arrange, give the precedence test a policy that enables overrides and carries a distinct external IP (and confirm it fails under mutation M1), and shrink the fixture. | Focused tests, `cargo +nightly fmt --all -- --check`, `git diff --check`, prose-first AAA review. |
| R2 | TODO | Add R2a-R2k. | Same validation; the named mutations M2-M5 each make their test fail. |
| R3 | TODO | Select an integration increment, or `none`. | Expected: `none`. The service has no network boundary of its own. |
| R4 | TODO | Re-run unit-only coverage; record before and after. | Plan Results and `coverage-evidence.md`. |

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

To be recorded per added or changed test during R1 and R2.

## Results

To be recorded after R4.

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
