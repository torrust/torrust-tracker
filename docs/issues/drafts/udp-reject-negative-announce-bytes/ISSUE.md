---
schema-version: 1
doc-type: issue
issue-type: bug
status: draft
priority: p2
epic: null
github-issue: null
spec-path: docs/issues/drafts/udp-reject-negative-announce-bytes/ISSUE.md
branch: "{issue-number}-udp-reject-negative-announce-bytes"
related-pr: null
last-updated-utc: "2026-11-11 00:00"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - UDP tracker accepts announces with a negative `left` (and `uploaded`/`downloaded`)

## Goal

The UDP tracker refuses an announce whose `left`, `uploaded` or `downloaded` is negative with a
BEP 15 error response, as the HTTP tracker already does, so one announce gets the same verdict on
both protocols.

## Background

Found as finding F6 by the EPIC #1347 `primitives` tests work.

What happens today, step by step:

1. A UDP client sends an announce packet. `bytes_left`, `bytes_uploaded` and `bytes_downloaded` are
   decoded as signed 64-bit integers (`packages/udp-protocol/src/announce.rs`, `NumberOfBytes(I64)`).
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

- Changing `peer::Peer::is_seeder` or `NumberOfBytes` (signed domain type; stays as is, pinned by
  the primitives characterization test).
- `num_want`, ids, `key`, `event` handling.
- The repo-wide "strict by default on inbound protocol values" ADR (separate work; this issue is
  its first case and cites it once it exists).

## Architectural Decisions

- Related ADRs: `docs/adrs/20261007082938_bound_protocol_agnostic_values_by_the_tightest_delivery_protocol.md`
  (outbound values); the planned inbound-tolerance ADR (`inbound-tolerance-adr`, not yet merged).
- ADRs to create: None known.

Proposed design: validate in `udp-core` `AnnounceService::handle_announce`, after cookie validation
and before whitelist authorization (so a bad request reveals nothing about the whitelist), with a
new `UdpAnnounceError` variant (e.g. `InvalidNumberOfBytes { field, value }`). The error maps to an
`ErrorResponse` through the existing handler error path.

Alternatives considered: (a) validate in the `udp-protocol` decoder: the codec is a copy of
aquatic's wire layer and must stay a faithful decoder, and its parse errors drop the transaction
id; (b) change `NumberOfBytes` to unsigned: wide blast radius, and HTTP/REST already rely on the
signed domain type.

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
- At the fix: assert the exact error variant and an `ErrorResponse` with the request's transaction
  id; same cases for `uploaded` and `downloaded` if approved; a boundary case (`0` and `i64::MAX`
  still accepted; `-1` and `i64::MIN` rejected); IPv6 variant.

## Implementation Plan

| ID  | Status | Task                                                          | Notes / Expected Output                      |
| --- | ------ | ------------------------------------------------------------- | -------------------------------------------- |
| T1  | DONE   | Reproduce over UDP and record evidence                        | `manual-verification-evidence.md` V1         |
| T2  | DONE   | Add red handler test for negative `left`                      | Test fails: announce returns `Ok`            |
| T3  | TODO   | Fix: validation + error variant in `udp-core`                 | Red test green; exact variant asserted       |
| T4  | TODO   | Extend tests (uploaded/downloaded, boundaries, IPv6)          | Per approved field scope                     |
| T5  | TODO   | Green run and like-for-like recheck over UDP                  | V2 in evidence file                          |
| T6  | TODO   | Document the behavior (UDP tracker docs/changelog if any)     | Docs mention rejection                       |

## Commit Points

| Task | Coherent change set                                  | Commit policy                                        |
| ---- | ---------------------------------------------------- | ---------------------------------------------------- |
| T1-2 | Spec, evidence and red test                          | Commit after focused validation and required review. |
| T3   | `fix(udp-core)`: reject negative bytes               | Commit after focused validation and required review. |
| T4   | Extra tests                                          | Commit after focused validation and required review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/udp-reject-negative-announce-bytes/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-11 UTC - Claude Code (Claude Sonnet 5.5) - Spec drafted, defect reproduced, red test added; awaiting maintainer decision.

## Acceptance Criteria

- [ ] AC1: a UDP announce with `left < 0` gets an error response (action 3) with the request's
      transaction id, and no peer is registered.
- [ ] AC2: same for `uploaded < 0` and `downloaded < 0` (if approved).
- [ ] AC3: `left = 0`, `left = i64::MAX`, and `num_want = -1` are still accepted.
- [ ] AC4: the rejection emits the existing UDP error event and log.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-udp-core -p torrust-tracker-udp-server`

### Manual Verification Scenarios

| ID  | Scenario                                   | Human-oriented command/steps                                   | Expected Result                       | Status | Evidence                                     |
| --- | ------------------------------------------ | -------------------------------------------------------------- | ------------------------------------- | ------ | -------------------------------------------- |
| M1  | Negative `left` over UDP (before the fix)  | Run the tracker; `python3 -I evidence/udp_announce_probe.py`   | Reproduced: response with seeders = 1 | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | Same probe after the fix                   | Same command on the fixed build                                | Error response, seeders = 0           | TODO   | `manual-verification-evidence.md` section V2 |

### Disposable Verification Scripts

- `evidence/udp_announce_probe.py`: sends one connect and one announce with chosen `left`. Python
  because it is a throwaway raw-socket probe of one packet, not product behavior; the durable check
  is the Rust handler test. Owner: this issue's author; remove at close-out.

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | TODO   |          |
| AC2   | TODO   |          |
| AC3   | TODO   |          |
| AC4   | TODO   |          |

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
