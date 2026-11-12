---
schema-version: 1
doc-type: issue
issue-type: bug
status: blocked
priority: p2
epic: 1347
github-issue: 2838
spec-path: docs/issues/open/2838-1347-udp-reject-negative-announce-bytes/ISSUE.md
branch: "2838-udp-reject-negative-announce-bytes"
related-pr: null
last-updated-utc: "2026-11-12 17:41"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #2838 - UDP tracker accepts announces with a negative `left` (and `uploaded`/`downloaded`)

> **Superseded plan (2026-11-12).** The tactical patch described here (PR #2839) was not merged.
> The bug is fixed at its root by
> [issue #2850](../2850-non-negative-number-of-bytes/ISSUE.md), whose implementation closes this
> issue. The analysis, the field audit, the client evidence and the reproduction evidence below
> stay valid and are used by that issue.

## Goal

The UDP tracker refuses an announce whose `left`, `uploaded` or `downloaded` is negative with a
BEP 15 error response, as the HTTP tracker already does, so one announce gets the same verdict on
both protocols.

## Background

Found during the EPIC #1347 `primitives` tests work (issue #2824), whose session tracks it as
finding F6. That finding is not recorded in the #2824 spec at the time of writing.

What happens today, step by step:

1. A UDP client sends an announce packet. `bytes_left`, `bytes_uploaded` and `bytes_downloaded` are
   decoded as signed 64-bit integers (request fields in `packages/udp-protocol/src/announce.rs`;
   the type `NumberOfBytes(I64)` is defined in `packages/udp-protocol/src/common.rs`).
2. `packages/udp-core/src/peer_builder.rs` copies them unchanged into `peer::Peer`
   (`NumberOfBytes(i64)`).
3. `Peer::is_seeder` (`packages/primitives/src/peer.rs`) is `left.0 <= 0 && event != Stopped`, so a
   negative `left` makes the peer a **seeder**.
4. The tracker stores the peer, counts it in `complete` (seeders) and returns a normal announce
   response.

The HTTP path refuses the same value: `extract_number_of_bytes_from_param`
(`packages/http-protocol/src/v1/requests/announce.rs`) parses `u64` and returns `InvalidParam`
for `uploaded`, `downloaded` and `left`. So the same announce is refused over HTTP and accepted
over UDP.

Contract violated: a byte count cannot be negative (BEP 3 defines these as counts of bytes; BEP 15
only gives the wire width, 64-bit integer, and is silent about negatives). Impact: a client can
register itself as a seeder without having anything (`left = -5`), inflating `complete` in
announce and scrape responses and in the metrics; nothing else is observably wrong.

Maintainer decision (2026-11-11): be strict and reject it.

### Field audit (decided here, per field)

| Field (UDP wire)                   | Wire type | Legitimate negative? | Decision                                    |
| ---------------------------------- | --------- | -------------------- | ------------------------------------------- |
| `left`                             | i64       | No                   | Reject if `< 0` (maintainer decision)       |
| `uploaded`                         | i64       | No                   | Reject if `< 0` (recommended, same as HTTP) |
| `downloaded`                       | i64       | No                   | Reject if `< 0` (recommended, same as HTTP) |
| `num_want` (`peers_wanted`)        | i32       | Yes: `-1` = default  | Keep signed; already handled                |
| `connection_id`, `transaction_id`  | i64, i32  | Opaque identifiers   | Unchanged                                   |
| `key`                              | i32       | Opaque identifier    | Unchanged                                   |
| `event`                            | i32       | Enum                 | Unchanged (out of scope)                    |

`uploaded` and `downloaded` are only stored on the peer today; they drive no logic, so rejecting
them is a consistency choice (same verdict as HTTP), not a defect fix. It is split as a separate
decision for the maintainer (see "Decision to approve").

### Evidence on real clients and other trackers

Collected by a research subagent reading raw sources (links verified by it, not re-fetched by the
author, except where noted). Nothing here is claimed from memory.

| Finding | Source | Verdict |
| ------- | ------ | ------- |
| BEP 15: `downloaded`, `left`, `uploaded` are "64-bit integer"; `num_want` "-1 // default"; errors are action 3. Silent on negatives and unknown size. | <https://www.bittorrent.org/beps/bep_0015.html> | Spec does not forbid or define negatives |
| libtorrent, no metadata yet: `req.left = value_or(bytes_left(), 16*1024)`; writes int64 | `src/torrent.cpp`, `src/udp_tracker_connection.cpp` (RC_2_0), <https://github.com/arvidn/libtorrent> | Sends positive 16384, never negative |
| Transmission, no metainfo: `leftUntilComplete = has_metainfo() ? total_size() - has_total() : INT64_MAX` | `libtransmission/announcer.cc` ~l.940, `announcer-udp.cc` ~l.248, <https://github.com/transmission/transmission> | Sends INT64_MAX, never negative |
| opentracker: only tests whether `left` is zero | `ot_udp.c` l.113-139 (a mirror, <https://github.com/masroore/opentracker>; upstream not reachable) | Accepts; negative = leecher |
| aquatic_udp: parses i64; Seeding if `left == 0` else Leeching; no validation found | `crates/udp/src/swarm.rs` l.741-752, <https://github.com/greatest-ape/aquatic> | Accepts; negative = leecher |
| chihaya: reads `left` as unsigned (`Uint64`) then `SanitizeAnnounce` (not read) | `frontend/udp/parser.go`, <https://github.com/chihaya/chihaya> | Negative becomes a huge number; rejection unverified |
| webtorrent bittorrent-tracker: parses to a double, no range check | `lib/server/parse-udp.js`, <https://github.com/webtorrent/bittorrent-tracker> | Accepts |

Reading of the evidence: no client checked sends a negative `left` (the two magnet-link cases send
`16384` and `INT64_MAX`, both non-negative and accepted by the strict rule). No tracker checked
rejects negatives, so strict is stricter than the ecosystem; opentracker and aquatic treat a
negative as a leecher, which is the lenient alternative. Not verified: qBittorrent, Deluge,
rtorrent, aria2 (qBittorrent uses libtorrent, an unverified inference); BEP 3 text; chihaya's
sanitizer; any known bug report (none found).

## Scope

### In Scope

- Reject, in the UDP announce path, a request with negative `left` (and, if approved, `uploaded`
  and `downloaded`) with an error response carrying the request's transaction id.
- The rejected announce must not register a peer or change any swarm statistic.
- Emit the existing UDP error event/log for the rejection, so operators can see how often real
  clients trip it (the evidence-collection mechanism; no new metric type).
- Tests at the UDP handler boundary and at the `udp-core` service boundary.

### Out of Scope

- Changing `peer::Peer::is_seeder` or `NumberOfBytes` (signed domain type; stays as is). The
  primitives tests work (issue #2824) plans a characterization test for it; that test is not in
  the tree at the time of writing.
- `num_want`, ids, `key`, `event` handling.
- The repo-wide "strict by default on inbound protocol values" ADR (separate work; this issue is
  its first case and cites it once it exists).

## Architectural Decisions

- Related ADRs: `docs/adrs/20261007082938_bound_protocol_agnostic_values_by_the_tightest_delivery_protocol.md`
  (outbound values); the planned inbound-tolerance ADR (`inbound-tolerance-adr`, not yet merged).
- ADRs to create: None known.

Proposed design: validate in `udp-core` `AnnounceService::handle_announce`, after cookie validation
and before whitelist authorization (so a bad request reveals nothing about the whitelist), with a
new `UdpAnnounceError` variant (`NegativeNumberOfBytes { field, value }`). The error maps to an
`ErrorResponse` through the existing handler error path.

Alternatives considered:

- (a) Validate in the `udp-protocol` decoder. Not chosen for this patch. An earlier version of this
  spec said the decoder's parse errors drop the transaction id; that was wrong:
  `Request::parse_bytes` already returns sendable errors for port 0 and invalid events. The
  maintainer accepted the service-level check as a tactical patch, and a follow-up EPIC will
  decide the layer.
- (b) Make the domain `NumberOfBytes` non-negative. Wide blast radius (HTTP, REST and UDP use the
  signed type); deferred to the same follow-up EPIC.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Follows [fix-bug](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md):

1. Analysis: hypothesis above (no validation between wire decode and `Peer`).
2. Reproduction against the real tracker over a UDP socket, then at the handler boundary
   (`evidence/udp_announce_probe.py`; results in `manual-verification-evidence.md`).
3. Regression-test boundary: the `udp-server` handler test (UDP boundary the maintainer asked for)
   plus a `udp-core` service test for the exact error variant.
4. Red commit (`test(udp-server): ...`), then a separate `fix(udp-core): ...` commit.
5. Green, then the like-for-like real-artifact recheck.

## Regression Test Strategy

- Handler test
  `handlers::announce::tests::announce_request::using_ipv4::it_should_reject_an_announce_with_a_negative_number_of_bytes_left`
  (already written, red: the announce returns `Ok` with `seeders: 1`).
- At the fix: the handler test asserts the exact error variant and that the failure carries the
  request's transaction id (the shared handler error path turns it into the `ErrorResponse`);
  `udp-core` tests cover `uploaded` and `downloaded`, the boundaries (`0`, `16384` and `i64::MAX`
  accepted; `-1` and `i64::MIN` rejected) and the placement (after cookie validation, before the
  whitelist). No IPv6 case: the check runs before any use of the client address.

## Implementation Plan

| ID  | Status | Task                                                          | Notes / Expected Output                      |
| --- | ------ | ------------------------------------------------------------- | -------------------------------------------- |
| T1  | DONE   | Reproduce over UDP and record evidence                        | `manual-verification-evidence.md` V1         |
| T2  | DONE   | Add red handler test for negative `left`                      | Test fails: announce returns `Ok`            |
| T3  | DONE   | Fix: validation + error variant in `udp-core`                 | Red test green; exact variant asserted       |
| T4  | DONE   | Extend tests (uploaded/downloaded, boundaries, placement)     | Per approved field scope; no IPv6 case       |
| T5  | DONE   | Green run and like-for-like recheck over UDP                  | V2 in evidence file                          |
| T6  | DONE   | Document the behavior                                         | Error text and spec; no UDP docs list per-field validation |

## Commit Points

| Task | Coherent change set                                  | Commit policy                                        |
| ---- | ---------------------------------------------------- | ---------------------------------------------------- |
| T1-2 | Spec, evidence and red test                          | Commit after focused validation and required review. |
| T3   | `fix(udp-core)`: reject negative bytes               | Commit after focused validation and required review. |
| T4   | Extra tests                                          | Commit after focused validation and required review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2838-1347-udp-reject-negative-announce-bytes/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-11 17:30 UTC - Claude Code (Claude Sonnet 5.5) - Spec drafted, defect reproduced, red test added; awaiting maintainer decision.
- 2026-11-12 09:15 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved all three fields; issue #2838 created; fix, tests and recheck done.
- 2026-11-12 16:40 UTC - Claude Code (Claude Fable 5.2) - Review round 1 on PR #2839 addressed: typed field enum, `BadRequest` event classification, placement tests, exact-variant handler test, V2 repeated on the committed tree. The maintainer accepted the service-level check as a tactical patch; a follow-up EPIC (draft, no issue yet) will make the domain byte count non-negative by construction.
- 2026-11-12 17:41 UTC - Claude Code (Fable 5.2) - The maintainer decided not to merge the tactical patch of PR #2839. The bug is fixed at its root by issue #2850 (`docs/issues/open/2850-non-negative-number-of-bytes/ISSUE.md`), whose implementation closes this issue. This specification and its evidence were carried over from the branch of PR #2839 as the record of the analysis, the reproduction and the client evidence. Its implementation plan and commit points describe the patch and are superseded; the tests of that branch are taken over by #2850.

## Acceptance Criteria

- [x] AC1: a UDP announce with `left < 0` gets an error response (action 3) with the request's
      transaction id, and no peer is registered.
- [x] AC2: same for `uploaded < 0` and `downloaded < 0` (if approved).
- [x] AC3: `left = 0`, `left = i64::MAX`, and `num_want = -1` are still accepted.
- [x] AC4: the rejection emits the existing UDP error event and log.
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-udp-core -p torrust-tracker-udp-server`

### Manual Verification Scenarios

| ID  | Scenario                                   | Human-oriented command/steps                                   | Expected Result                       | Status | Evidence                                     |
| --- | ------------------------------------------ | -------------------------------------------------------------- | ------------------------------------- | ------ | -------------------------------------------- |
| M1  | Negative `left` over UDP (before the fix)  | Run the tracker; `python3 -I evidence/udp_announce_probe.py`   | Reproduced: response with seeders = 1 | DONE   | `manual-verification-evidence.md` section V1 |
| M2  | Same probe after the fix                   | Same command on the fixed build                                | Error response, seeders = 0           | DONE   | `manual-verification-evidence.md` section V2 |

### Disposable Verification Scripts

- `evidence/udp_announce_probe.py`: sends one connect and one announce with chosen `left`. Python
  because it is a throwaway raw-socket probe of one packet, not product behavior; the durable check
  is the Rust handler test. Owner: this issue's author; remove at close-out.

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | DONE   | udp-core `it_should_reject_a_negative_number_of_bytes`; udp-server handler test; V2 |
| AC2   | DONE   | same tests, `uploaded`/`downloaded` cases; V2 |
| AC3   | DONE   | `it_should_accept_a_non_negative_number_of_bytes_left` (0, 16384, i64::MAX) |
| AC4   | DONE   | V2 log lines; `it_should_classify_a_negative_number_of_bytes_in_an_announce_as_a_bad_request` |

## Risks and Trade-offs

- A client that sends a negative `left` for an unknown size would start failing. Evidence shows
  libtorrent (16384) and Transmission (INT64_MAX) do not; other clients are unverified. Mitigation:
  the rejection is logged and counted through the existing UDP error event, so a regression report
  is visible, and the check is a single revertible condition.
- Stricter than every tracker examined; accepted by the maintainer's decision.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- EPIC #1347 (primitives tests, finding F6)
- Related ADRs: `docs/adrs/20261007082938_bound_protocol_agnostic_values_by_the_tightest_delivery_protocol.md`
