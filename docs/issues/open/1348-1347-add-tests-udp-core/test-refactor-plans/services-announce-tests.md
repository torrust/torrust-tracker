---
doc-type: file-test-plan
issue: 1348
package: torrust-tracker-udp-core
target-file: packages/udp-core/src/services/announce.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-core/src/services/announce.rs
    - packages/udp-core/src/peer_builder.rs
    - packages/udp-core/src/connection_cookie.rs
    - packages/udp-server/src/handlers/announce.rs
    - docs/issues/open/1348-1347-add-tests-udp-core/coverage-evidence.md
---

# UDP Core Announce Service File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-core/src/services/announce.rs`.

## Current State

- **Unit-only coverage:** 0 / 101 lines (0.00%) from the T1 report.
- **Existing tests:** none in this file. `udp-server` handler tests exercise the service through
  the handler, which EPIC #1347 does not accept as evidence for this package's unit seam.
- **Module-owned decisions:** apply the connection-ID validation flag; validate the cookie against
  the client socket fingerprint and the given validity range; authenticate before whitelist
  authorization; announce the peer with the socket IP, not the request's `ip_address` field; pass
  the listener's external IP to tracker core; publish `UdpAnnounce` with the connection context
  (instance ID, client address, server binding, public URL), info hash, and announced peer; adapt
  cookie, whitelist, and tracker-core failures to `UdpAnnounceError`.
- **T1 hypothesis:** test candidate.

## Current Tests Review

No existing tests. R1 is skipped.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Cookie policy: strict validation rejects a cookie issued outside the valid range | 3A unit | R2a: expired cookie with validation enabled returns `ConnectionCookieError`. |
| Cookie policy: disabled validation accepts the same cookie | 3A unit | R2b: identical request with validation disabled succeeds; the flag is the one Arrange difference. |
| Whitelist rejection | Collaboration | R2c: listed tracker, torrent not whitelisted, valid cookie returns `TrackerCoreWhitelistError` for that info hash. |
| Authentication before authorization | 3A unit | R2d: listed tracker, torrent not whitelisted, expired cookie returns the cookie error, so whitelist status is not revealed to unauthenticated clients. |
| Peer address from the socket | 3A unit | R2e: request `ip_address` names another IP; the registered peer uses the client socket IP and request port (observed through the test-owned torrent repository). |
| External-IP pass-through | Collaboration | R2f: loopback client with a configured external IP registers the peer with the external IP. The assignment rule is `tracker-core`'s; the test protects only that the service forwards its configuration. |
| Event publication | 3A unit | R2g: a successful announce sends exactly one `UdpAnnounce` event with the full connection context including public URL, the info hash, and the announced peer. |
| Absent event sender | 3A unit | No dedicated test: R2a-R2f run without a sender and therefore cover it. |
| `TrackerCoreAnnounceError` adaptation | Collaborator-owned | Not selected. Reachable only through a `tracker-core` database failure with persistent statistics, or a whitelist mismatch between the service and handler that the container never wires. The `From` impl is a one-field wrap. |
| `peers_wanted` limit, peer selection, swarm statistics | Collaborator-owned | `tracker-core` owns them; no test here. |
| Property-based testing | Not selected | No natural property: the decisions are enumerable branches. |

Test bootstrap: an in-file fixture constructs only `Core` (public or listed), the in-memory
whitelist, `WhitelistAuthorization`, `InMemoryTorrentRepository`, and `AnnounceHandler`, and keeps
the repository handle for state assertions. Scenario constructors name the one state difference
(for example listed without whitelisted torrents, or with an external IP). Requests use an
in-file announce request builder; `udp-server`'s builder is not shared across packages.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests to refactor. | Recorded above. |
| R2 | DONE | Add R2a-R2g. | 7 focused `services::announce` tests passed; nightly formatting, pedantic Clippy, and `git diff --check` clean; R2d failed under a temporary authenticate/authorize order swap. |
| R3 | SKIPPED | No integration increment: the service has no network boundary. | Explicit rationale. |
| R4 | DONE | Re-run unit-only coverage; record before/after. | 0 / 101 to 338 / 347 lines; see Results. |

## Results

- **Unit-only coverage (test-inclusive file totals):** 0 / 101 (0.00%) to 338 / 347 (97.41%)
  lines; 0 / 14 to 41 / 44 functions; 0 / 102 to 376 / 389 regions. The denominator grows because the
  measurement includes the new test module.
- **Residual uncovered lines:** the `configuration_instance_id()` and `public_url()` accessors
  and the deliberately unselected `From<AnnounceError>` adaptation. No untested service decision
  remains.
- **Tests added:** 7 (R2a-R2g), using an in-file fixture that owns the torrent repository for
  state assertions.
- **Integration selected:** No.
- **Completed-file review:** Approved by Jose Celano on 2026-10-04.

## Approval

Approved by Jose Celano on 2026-10-02: R2a-R2g, the fixture approach, and not testing
`TrackerCoreAnnounceError` adaptation.

## Progress Log

- 2026-10-02 18:34 UTC - GitHub Copilot - Created the plan from `services/announce.rs`, the
  `tracker-core` announce and whitelist collaborators, and the T1 report. No Rust tests or
  production code changed. R2 awaits maintainer approval.
- 2026-10-02 18:43 UTC - Jose Celano - Approved the plan. R2 is paused until the maintainer's
  bug-fix PR, which adds tests to this package, merges. After rebasing, re-check R2a-R2g against
  those tests and drop any they already cover.
- 2026-10-04 09:48 UTC - GitHub Copilot - Rebased onto `develop` after PR #2423 merged. #2423
  changed only doc comments in `udp-core` and added no tests overlapping R2a-R2g, so all seven
  were kept. Implemented R2 and measured R4 (stable Rust 1.99.0, nightly 1.101.0,
  `cargo-llvm-cov 0.6.16`). The plan awaits completed-file review.
- 2026-10-04 10:35 UTC - Jose Celano - Approved completed-file review. The file is ready for
  pre-commit and its signed per-file commit.
