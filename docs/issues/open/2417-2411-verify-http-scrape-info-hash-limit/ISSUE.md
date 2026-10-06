---
schema-version: 1
doc-type: issue
issue-type: bug
status: in-progress
priority: p2
epic: 2411
github-issue: 2417
spec-path: docs/issues/open/2417-2411-verify-http-scrape-info-hash-limit/ISSUE.md
branch: "2417-2411-verify-http-scrape-info-hash-limit"
related-pr: null
last-updated-utc: "2026-10-06 09:58"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #2417 - Verify Whether HTTP Scrape Enforces the 74 Info-Hash Limit

Parent: [EPIC #2411 - Spam and abuse resistance](../2411-spam-and-abuse-resistance/EPIC.md).

## Goal

Find out whether an HTTP scrape request is limited to 74 info hashes, as the
documentation says. Then reconsider, without assuming the answer, whether HTTP
should have a per-request cap at all and with which value. Record the decision
and its reasons in an ADR, then apply the correction (code, documentation, or
both) that the decision requires.

## Background

`torrust_tracker_core::MAX_SCRAPE_TORRENTS` is `74`
(`packages/tracker-core/src/lib.rs`), following the usual note that "up to
about 74 torrents can be scraped at once" (a limit that comes from the UDP
packet size).

- UDP: enforced. `packages/udp-server/src/handlers/mod.rs` passes
  `MAX_SCRAPE_TORRENTS` to `Request::parse_bytes`
  (`packages/udp-protocol/src/request.rs`), which keeps only the first 74.
- HTTP: only documented. `packages/axum-http-server/src/lib.rs` says the
  maximum is 74, but a code search on 2026-10-02 found no cap in
  `extract_info_hashes` (`packages/http-protocol/src/v1/requests/scrape.rs`),
  the HTTP scrape service (`packages/http-core/src/services/scrape.rs`), or
  `ScrapeHandler::handle_scrape`. The local review probe now confirms 75 and
  1000 distinct hashes are returned in full; the latter logged HTTP 200.
  See [manual-verification-evidence.md](manual-verification-evidence.md), V1.
  A heavily encoded 1000-hash URL failed in the client; a compact ASCII URL
  succeeded. This is not evidence that all HTTP transport limits are absent.

If the hash-count limit is missing, requests may exceed the documented count;
transport size limits do not establish a 74-hash contract. That is a case for
the spam and abuse EPIC draft
(`docs/issues/open/2411-spam-and-abuse-resistance/EPIC.md`).

### Why 74 Has Different Reasons per Protocol

- **UDP: protocol constraint.** 74 is the number of info hashes whose scrape
  request fits in the tracker's UDP packet budget: (1496 - 16) / 20 (BEP 15's
  "up to about 74 torrents"). It comes from
  the transport, not from a policy choice.
- **HTTP: no such constraint.** The response is a TCP stream and the request
  URL has no protocol-level length limit relevant here (V1 returned 1000
  entries). Any HTTP cap is therefore a policy choice, and its only reason is
  to bound the work one request can cause (abuse mitigation).
- **Limit of that reason.** A per-request cap does not stop abuse: a client
  can send many capped requests in parallel and cause the same total work.
  It only bounds per-request cost and response size; total-load protection
  belongs to rate limiting in EPIC #2411.

The documentation must state these reasons separately. Reusing the UDP
explanation for HTTP is the documentation defect this issue found.

## Scope

### In Scope

- Manual verification against a locally running tracker with 1000 distinct
  info hashes in one HTTP scrape (M1), plus boundary probes at 74 and 75.
  Use 75 hashes for the UDP control (M2), not an oversized 1000-hash datagram.
- Reconsider whether HTTP should cap scrape requests (T2), with the
  maintainer, and record the outcome in an ADR (T3).
- Apply the correction the decision requires, with a regression test if the
  behavior changes.
- Make the documentation match the behavior and link the ADR.

### Out of Scope

- Rate limiting (spam and abuse EPIC).
- Making the limit configurable, unless the maintainer asks for it.

## Behavior Options

To decide in T2, with the maintainer. Do not assume a cap is right because UDP
has one; the reasons differ (see above):

| Option | Behavior | Pros | Cons |
| --- | --- | --- | --- |
| A | Truncate to the first 74, like UDP | Same behavior on both protocols; clients get a response | Silent; the client does not learn that hashes were dropped |
| B | Reject the request with a bencoded failure | Explicit | Breaks clients that send more than 74 today |
| C | Keep it unlimited and fix the documentation | No behavior change | Leaves the per-request cost unbounded |

Maintainer input (2026-10-02): a 74 cap on HTTP for abuse mitigation is a
plausible outcome, but it is a hypothesis to reconsider in T2, not a decision.

### T2 Decision (Maintainer, 2026-10-05)

1. **Cap HTTP scrape: yes, option A.** Keep the first N info hashes and
   silently ignore the rest, like UDP. No log or metric for truncation.
2. **HTTP value: 100.** A standalone policy value, not derived from the UDP
   limit nor from the database batch size. That 100 hashes fit one persisted
   downloads query (`MAX_INFO_HASHES_PER_QUERY`) is a consequence, not a
   coupling: no shared constant.
3. **Enforcement point: the HTTP request parser** (`Scrape::try_from` in
   `http-protocol`). It stops decoding at the cap, so `info_hash` values past
   the cap are ignored without validation, mirroring the UDP parser.
4. **Constants: one per protocol, symmetric names, owned by the protocol
   package whose reason they carry.**
   - `udp-protocol`: `MAX_SCRAPE_INFO_HASHES = 74` (UDP packet budget, BEP 15).
   - `http-protocol`: `MAX_SCRAPE_INFO_HASHES = 100` (HTTP abuse-mitigation
     policy).
   - Remove `torrust_tracker_core::MAX_SCRAPE_TORRENTS` (public API change,
     accepted for `3.0.0-develop`); rename the UDP parser parameter
     `max_scrape_torrents` to `max_scrape_info_hashes`.
5. **Tests:** an HTTP parser unit test, an HTTP server test with 101 hashes,
   and UDP parser and server tests pinning 74 (no test references the UDP
   limit today).
6. **M2:** run the UDP 75-hash control manually with `tracker_client`,
   verifying the transmitted hash count.
7. **No external research:** the decision rests on the project's own reasons.
8. **One PR** for the spec update, ADR, tests, fix, and documentation, in
   separate commits.

### Edge-Case Decisions (Maintainer, 2026-10-05)

Every decision below is pinned by a test in the matrix under "Regression Test
Strategy", so the tests state where each limit lives and why.

1. **HTTP counts `info_hash` parameters as sent.** Duplicates count toward
   the cap, like UDP's raw 20-byte slots, and counting stops before decoding.
2. **Parameters past the HTTP cap are not validated.** An invalid value after
   the 100th parameter is ignored; an invalid value within the first 100 still
   fails the request.
3. **The UDP value is computed from the packet size in code:**
   `(MAX_PACKET_SIZE - 16-byte scrape request header) / 20-byte info hash`,
   which is 74 for `MAX_PACKET_SIZE = 1496`. The reason then lives in the
   expression, not in a comment next to a literal.
4. **UDP has two truncation points.** The server reads datagrams into a
   `MAX_PACKET_SIZE` buffer, so the kernel already drops the 75th hash of a
   datagram sent over a socket. A socket-level test alone cannot prove that the
   server passes the cap to the parser; a `handle_packet` unit test sends a
   75-hash payload directly to cover that wiring.
5. **tracker-core has no cap.** A `ScrapeHandler` test with more than 100
   hashes shows that the caps live in the protocol parsers.
6. **Boundary tests use literal counts** (100/101, 74/75), not the constants,
   so changing a limit fails the tests and forces an ADR update. Each test's
   doc comment names the reason and links the ADR.

### Client Decision (Maintainer, 2026-10-05)

The unified `tracker_client` stops capping UDP scrape arguments at 74
(`num_args = 1..=74`). The limit belongs to a tracker, not to the protocol
client: other trackers may accept more or fewer, and a diagnostic client must be
able to probe truncation (it blocked M2). Because trackers truncate silently,
the client instead warns when a scrape returns fewer entries than requested:
one NDJSON record on stderr (`kind: scrape_response_truncated`), stdout
unchanged, per the global CLI output contract ADR. UDP compares entries with
requested hashes; HTTP compares files with distinct requested hashes, because
the response dictionary collapses duplicates. The frozen legacy
`udp_tracker_client` is left unchanged until it is removed.

History (from the #2003 specifications discussion): the original Warp HTTP
tracker rejected scrapes above `MAX_SCRAPE_TORRENTS` (option B); the Axum
rewrite dropped the check without a recorded decision.

Questions T2 had to answer (answered above):

1. Should HTTP cap scrape requests at all (A/B versus C)? Weigh the bounded
   per-request cost against the fact that parallel requests bypass any
   per-request cap, and against compatibility with existing clients.
2. If capped, which value? 74 gives the same limit on both protocols, but its
   UDP origin is not a reason for HTTP; another value needs its own reason.
3. If capped, truncate (A) or reject (B)? Initial recommendation: A, for
   consistency with UDP.
4. Where does the value live? `MAX_SCRAPE_TORRENTS` carries the UDP reason.
   Either document both reasons on the shared constant, or give HTTP its own
   named constant whose doc comment states its reason, so changing one
   protocol's limit cannot silently change the other.

## Architectural Decisions

- Related ADRs: [20261005124222](../../../adrs/20261005124222_cap_scrape_info_hashes_per_protocol.md),
  created by this issue (T3).
- ADRs to create: one root ADR in `docs/adrs/` (the decision spans
  `udp-protocol`, `http-protocol`, `http-core`, and `tracker-core`), following
  `.github/skills/dev/planning/create-adr/SKILL.md`. It records the T2
  decision, the per-protocol reasons for each limit (or for no HTTP limit),
  the rejected options, and that a per-request cap is not a substitute for
  rate limiting (EPIC #2411). Code doc comments and user documentation link it
  instead of repeating the rationale.

## Design and Ownership Review

The manual runner owns its tracker process, temporary database, and ports.
Bound startup, requests, and stop waits; clean up owned processes on failure.
Do not interpret HTTP transport rejection as evidence of a hash-count limit.

### Fake Trackers (T8, T9)

Added after review (PR #2444, finding F3), describing the fixtures in
`console/tracker-client/tests/common/fake_trackers/`:

- **Ownership.** Each fake owns one socket bound to `127.0.0.1:0` and one
  server thread. The socket moves into the thread; `ServerThread` owns the
  stop flag and the join handle. A test owns the fake for its whole scope and
  starts its own, so no socket or port is shared between tests.
- **Normal and drop-path lifetime.** Dropping the fake sets the stop flag and
  joins the thread, on success and on a failing assertion alike. A panic in the
  fake thread fails the test through the join (unless the test is already
  panicking), so fake errors are never silently ignored.
- **Deadlines.** The UDP fake polls the stop flag through a 10 ms read
  timeout; the HTTP fake polls its non-blocking listener every 10 ms and reads
  each accepted request under a 5 s timeout, so a silent peer fails the test
  instead of blocking the join. The `tracker_client` runs have no deadline of
  their own: they rely on the binary's 5 s network timeout and, for the
  monitor, its `--duration`.
- **Design review.** No separate checkpoint was held after the first passing
  slice: K5-K8 were written together, then mutation-checked (T8). This
  section and the read timeout were added in review instead; the prose-first
  review records the accepted timing exception.

## Bug-Fix Process

Follow `.github/skills/dev/debugging/fix-bug/SKILL.md`. The first step is M1.
The reproduced defect is the mismatch between documentation and behavior; the
correction may change the code (A/B) or only the documentation (C), depending
on the T2 decision.

M1 was reproduced during the requested draft review, and M2 was recorded
before the final behavior was selected (V2). V1 classifies the observed
documentation mismatch separately from the unmeasured overload risk.

## Regression Test Strategy

The limit is applied in the HTTP request parser, so the causal seam is a unit
test of `Scrape::try_from` in `packages/http-protocol` with 101 distinct
`info_hash` parameters, proven red before the fix. Because the HTTP limit was
once lost silently in a server rewrite, an HTTP server test in
`packages/axum-http-server` also sends 101 hashes through the public interface
and expects 100 entries. UDP parser and server tests pin the 74 limit with 75
hashes; they are characterization tests (UDP already behaves correctly).

Test matrix (one test per row; "red" rows must fail before the fix):

| ID | Package / seam | Input | Expected | Pins | Red first |
| --- | --- | --- | --- | --- | --- |
| H1 | `http-protocol` `Scrape::try_from` | 100 distinct hashes | All 100, in request order | Cap is inclusive | No |
| H2 | `http-protocol` `Scrape::try_from` | 101 distinct hashes | First 100, in request order | Truncation (option A), value 100 | Yes |
| H3 | `http-protocol` `Scrape::try_from` | 100 valid + invalid 101st | Ok, first 100 | Values past the cap are not validated | Yes |
| H4 | `http-protocol` `Scrape::try_from` | Invalid value within the first 100, 101 total | Error | Validation unchanged within the cap | No |
| H5 | `http-protocol` `Scrape::try_from` | 101 params, first one repeated | First 100 params (99 distinct) | Duplicates count | Yes |
| S1 | `axum-http-server` contract | 101 distinct hashes | 100 files: the first 100 | End-to-end HTTP contract | Yes |
| U1 | `udp-protocol` `Request::parse_bytes` | 74 hashes, `MAX_SCRAPE_INFO_HASHES` | All 74 | Cap is inclusive | No |
| U2 | `udp-protocol` `Request::parse_bytes` | 75 hashes, `MAX_SCRAPE_INFO_HASHES` | First 74 | UDP value 74 | No |
| U3 | `udp-protocol` | Largest scrape request fitting `MAX_PACKET_SIZE` | Carries exactly `MAX_SCRAPE_INFO_HASHES` hashes | Value derived from the packet size | No |
| W1 | `udp-server` `handle_packet` | 75-hash payload, no socket | 74 entries | Server passes the cap to the parser | No |
| US1 | `udp-server` contract (socket) | 75 hashes | 74 entries | End-to-end UDP contract (receive buffer and parser) | No |
| C1 | `tracker-core` `ScrapeHandler` | 101 hashes | 101 entries | Core has no cap | No |
| K1 | `tracker-client` CLI parsing | `udp scrape` with 75 hashes | Accepted | Client does not cap | Yes |
| K2 | `tracker-client` truncation check | 75 requested, 74 returned | Warning record with both counts | Client reports silent truncation | Yes |
| K3 | `tracker-client` truncation check | 74 requested, 74 returned | No warning | No false warning | No |
| K4 | `tracker-client` HTTP distinct count | 3 params, 2 distinct | 2 | Duplicates do not cause a false warning | Yes |
| K5 | `tracker_client` binary vs fake UDP tracker keeping 2 | 3 hashes | Exit 0; stdout 2 entries; stderr exactly one warning (3 requested, 2 returned) | `run()` emits the warning for any tracker limit | No |
| K6 | `tracker_client` binary vs fake UDP tracker keeping all | 3 hashes | Exit 0; stdout 3 entries; stderr empty | No false warning end to end | No |
| K7 | `tracker_client` binary vs fake HTTP tracker keeping 2 | 3 hashes | Exit 0; stdout 2 files; stderr exactly one warning (3 requested, 2 returned) | Same, over HTTP | No |
| K8 | `tracker_client` binary vs fake HTTP tracker keeping all | 3 params, 2 distinct | Exit 0; stdout 2 files; stderr empty | Duplicates do not warn end to end | No |

## Implementation Plan

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | M1 and M2 manual verification | M1 (V1) and M2 (V2) recorded |
| T2 | DONE | Reconsider whether HTTP should cap scrape requests | See "T2 Decision": truncate at 100 in the HTTP parser |
| T3 | DONE | ADR for the decision | `docs/adrs/20261005124222_cap_scrape_info_hashes_per_protocol.md`; index updated |
| T4 | DONE | Regression tests for the agreed contract | Matrix H1-H5, S1, U1-U3, W1, US1, C1 added; H2, H3, H5, S1 proven red before the fix; W1 proven to catch a mutated cap |
| T5 | DONE | Implement and fix docs | Per-protocol `MAX_SCRAPE_INFO_HASHES`, UDP value computed from `MAX_PACKET_SIZE`, tracker-core constant removed; server docs and EPIC A3 updated |
| T6 | DONE | Final recheck | V3: 74, 75, and 1000 hashes returned 74, 75, and 100 entries |
| T7 | DONE | Client: no cap, truncation warning | K1 proven red before removing the cap; K1-K4 green; V4 shows the warning for UDP 75 and HTTP 1000, none for HTTP 74 |
| T8 | DONE | Binary-level client tests with fake trackers | K5-K8 green; mutation-checked (dropping the warning fails K5 and K7; counting raw HTTP params fails K8); fakes originally in `console/tracker-client/tests/tracker_client/fake_trackers/`, moved to `tests/common/fake_trackers/` by T9 |
| T9 | DONE | Ride-along: monitor success-path test | Deferred item 14 of the closed refactor plan #1178, unblocked by T8's fakes. Fakes moved to `console/tracker-client/tests/common/fake_trackers/`; `FakeUdpTracker` answers announces and has `start_silent()` (named `silent()` when T9 was done), replacing the ad-hoc sink; the success-path test is mutation-checked; the refactor plan records the resolution. Not an acceptance criterion of this issue |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T3 | ADR | `docs(adrs): ...` commit after maintainer review of the decision. |
| T4-T5 | Test and limit | Commit after focused validation and review. |
| T5 | Documentation | Separate `docs(...)` commit if not part of the code change. |

## Acceptance Criteria

- [x] AC1: Whether HTTP scrape enforces the documented 74-hash cap is shown by
  recorded manual evidence (V1: it does not on the tested requests).
- [x] AC2: An HTTP scrape with more than 100 info hashes returns entries for
      only the first 100, matching the documentation; a UDP scrape keeps the
      first 74.
- [x] AC3: Maintained tests cover every decision and edge case in the test
      matrix (H1-H5, S1, U1-U3, W1, US1, C1, K1-K8), using literal counts and doc
      comments that name each limit's reason and link the ADR.
- [x] AC4: An ADR records whether HTTP caps scrape requests, the value and
      behavior if so, and the reason per protocol (UDP packet size; HTTP
      policy), noting that parallel requests bypass a per-request cap and that
      rate limiting is EPIC #2411. Doc comments and user docs link it.
- [x] `linter all` exits with code `0`; relevant tests pass.

## Verification Plan

### Automatic Checks

- `linter all`, focused tests at the chosen boundary, and required pre-push checks.
- Follow `write-unit-test`: smallest behavior increment, focused validation,
  prose-first Arrange-Act-Assert design review, then maintainer review before
  the next increment. Keep the causal input, production Act, and independent
  expected result visible; helpers hide only incidental mechanics.

### Manual Verification Scenarios

| ID | Scenario | Steps | Expected Result | Status |
| --- | --- | --- | --- | --- |
| M1 | HTTP scrape with 1000 info hashes (repeated in T6) | Maintained client sent 74/75 numeric hashes and 1000 compact ASCII hashes; decoded result counted with jq; tracker response log checked | Returned 74/75/1000 respectively; 1000 logged HTTP 200; longer numeric URL failed client-side; URL length not captured (V1) | DONE |
| M2 | UDP scrape control | Send one 75-hash request with a valid connection ID to the local tracker; verify actual transmitted hash count so client truncation cannot hide server behavior | 1516 bytes (75 hashes) sent; 74 entries returned (V2). Maintained client caps CLI args at 74, so an inline Python snippet sent the datagram; rerun with the maintained client in V4 after T7 | DONE |

A small disposable script may generate the 1000-hash URL; record its path and
removal owner in this spec before creating it.

Record actual commands, toolchain/runtime, responses, tracker logs, initial
classification, and the like-for-like final recheck in issue-local
`manual-verification-evidence.md`. Duplicate hashes are not a valid count probe
because the response dictionary can collapse them.

## Progress Tracking

### Workflow Checkpoints

- [x] Draft reviewed; HTTP mismatch reproduced, final behavior choice still pending
- [x] GitHub issue created and parent linked
- [x] Manual baseline, reconsidered decision, and ADR recorded
- [x] Implementation, focused checks, and manual recheck completed
- [x] Acceptance criteria re-reviewed and independent Task Reviewer report recorded

### Progress Log

- Maintainer approved a separate verification issue. V1 now records the
  1000-hash local result; no limit policy or implementation is implied by it.
- 2026-10-02 17:42 UTC - Maintainer clarified that 74 is a UDP packet-size
  limit with no HTTP equivalent; keeping 74 on HTTP for abuse mitigation is
  a good candidate if the different reason and its limit are documented.
  Added the per-protocol rationale section and AC4.
- 2026-10-02 17:51 UTC - Maintainer: do not assume HTTP needs the cap;
  reconsider first (T2), record the decision in an ADR (T3), then apply any
  correction. #2406 is a separate scrape issue (persisted download counts for
  torrents with no swarm), unrelated to this limit; the UDP control M2 stays
  here.
- 2026-10-05 12:35 UTC - T2 decided with the maintainer (see "T2
  Decision"): truncate HTTP scrapes at 100 in the parser, per-protocol
  `MAX_SCRAPE_INFO_HASHES` constants, tracker-core constant removed. Fixed
  the `branch` frontmatter, which pointed at an unrelated spec branch.
- 2026-10-05 13:10 UTC - Maintainer asked for tests covering every edge case
  and decision. Found that the UDP receive buffer (`MAX_PACKET_SIZE` = 1496 =
  16 + 74 x 20) already truncates a 75-hash datagram, so a socket test cannot
  prove the parser cap. Recorded the edge-case decisions and the test matrix;
  the UDP value is now computed from `MAX_PACKET_SIZE`.
- 2026-10-05 13:20 UTC - ADR, tests, and fix done. The UDP parser parameter
  became `max_scrape_info_hashes: usize` so the computed constant needs no
  truncating cast. M2 and T6 recorded (V2, V3). Open question for the
  maintainer: the UDP client CLI hardcodes `num_args = 1..=74` in two places, a
  third copy of the UDP limit, which blocked M2 with the maintained client.
- 2026-10-05 13:40 UTC - Maintainer chose to remove the client cap and warn on
  truncation, in this PR (see "Client Decision"); added T7 and rows K1-K4.
- 2026-10-05 15:11 UTC - T7 done: client cap removed, truncation warning
  added for UDP and HTTP; M2 rerun with the maintained client (V4).
- 2026-10-05 15:45 UTC - Maintainer asked for automated end-to-end client
  tests instead of relying on V4 alone: added T8 and rows K5-K8 (binary-level
  tests against fake trackers).
- 2026-10-05 15:54 UTC - T8 done: K5-K8 run the `tracker_client` binary
  against fake UDP and HTTP trackers and are mutation-checked.
- 2026-10-05 18:07 UTC - Maintainer asked to implement the monitor
  success-path test deferred in refactor plan #1178 (item 14) on top of the
  new fakes, in this PR; added T9.
- 2026-10-05 18:33 UTC - T9 done: the monitor tests run against
  `FakeUdpTracker::answering()` and `silent()`; `keeping_all()` became
  `answering()` on both fakes.
- 2026-10-05 21:05 UTC - The first Task Reviewer report failed: the
  prose-first Arrange-Act-Assert comparison was planned but not recorded. It
  is now done and recorded in
  [prose-first-test-review.md](prose-first-test-review.md); it led to small
  test changes (specific H4 error, visible U1/U2 Acts, W1 context, K1 ADR
  link). The reviewer's spec findings are fixed. Reports are in
  [agent-review-reports.md](agent-review-reports.md).
- 2026-10-06 06:14 UTC - The Task Reviewer re-review passed (21:07 UTC);
  its optional retrospective and review-record findings are applied. Pre-push
  is next.
- 2026-10-06 07:11 UTC - Rebased on the latest `develop`; pre-commit
  (including `linter all`) passed on every commit and the full pre-push suite
  passed (nightly format, check, and docs; all stable tests).
- 2026-10-06 09:58 UTC - Renamed the fake tracker constructors to `start()`,
  `start_with_scrape_limit(n)`, and `start_silent()` after maintainer review;
  pre-push passed again. Opened PR #2444.

### Acceptance Verification

| AC ID | Status | Evidence |
| --- | --- | --- |
| AC1 | DONE | [V1: HTTP baseline](manual-verification-evidence.md#v1-documented-74-hash-cap) |
| AC2 | DONE | S1 and US1 tests; [V2](manual-verification-evidence.md#v2-udp-control-m2) and [V3](manual-verification-evidence.md#v3-http-recheck-after-the-fix-t6) |
| AC3 | DONE | Test matrix rows implemented in `http-protocol`, `axum-http-server`, `udp-protocol`, `udp-server`, `tracker-core`, and `tracker-client` (K1-K4 unit, K5-K8 binary-level against fake trackers; [V4](manual-verification-evidence.md#v4-maintained-client-without-the-74-cap-t7)) |
| AC4 | DONE | ADR 20261005124222; HTTP and UDP constant docs and server crate docs link it |

## Implementation Completion Review

Record material discoveries in issue-local `implementation-retrospective.md`,
or explain in the progress log why none is needed. An independent Task Reviewer
checks acceptance criteria before the implementation PR; record its report in
`agent-review-reports.md`. Commit reproduction/recheck evidence with spec
updates after the required gates; a justified no-change outcome needs no empty
code commit.

## References

- Related issues: spam and abuse EPIC (draft)
- Related code: `MAX_SCRAPE_TORRENTS` (removed by this issue), the
  per-protocol `MAX_SCRAPE_INFO_HASHES` constants that replace it
