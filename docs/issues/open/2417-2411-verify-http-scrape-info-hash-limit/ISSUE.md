---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p2
epic: 2411
github-issue: 2417
spec-path: docs/issues/open/2417-2411-verify-http-scrape-info-hash-limit/ISSUE.md
branch: "2410-process-queued-events-before-listeners-stop-spec"
related-pr: null
last-updated-utc: "2026-10-02 18:45"
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
  response fits in the conventional UDP packet budget (BEP 15). It comes from
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

Questions T2 must answer:

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

- Related ADRs: none known.
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

## Bug-Fix Process

Follow `.github/skills/dev/debugging/fix-bug/SKILL.md`. The first step is M1.
The reproduced defect is the mismatch between documentation and behavior; the
correction may change the code (A/B) or only the documentation (C), depending
on the T2 decision.

M1 was reproduced during the requested draft review. M2 remains pending;
record it before selecting the final behavior. V1 classifies the observed
documentation mismatch separately from the unmeasured overload risk.

## Regression Test Strategy

Preferred boundary: a unit test of HTTP scrape request parsing in
`packages/http-protocol` with 75 or more `info_hash` parameters, if the limit
is applied there. Otherwise, the smallest boundary where the limit is applied.

## Implementation Plan

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | IN_PROGRESS | M1 and M2 manual verification | M1 confirms 75/1000 HTTP results; M2 UDP control remains pending |
| T2 | TODO | Reconsider whether HTTP should cap scrape requests | Answers to the four T2 questions agreed with the maintainer |
| T3 | TODO | ADR for the decision | Root ADR with per-protocol reasons and rejected options; index updated |
| T4 | TODO | Regression test for the agreed contract | Red before a behavior fix; characterization if retaining current behavior; justify no new test for a documentation-only correction |
| T5 | TODO | Implement and fix docs | Test green; doc comments and user docs match behavior, state each protocol's reason, and link the ADR |
| T6 | TODO | Final recheck | M1 repeated |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T3 | ADR | `docs(adrs): ...` commit after maintainer review of the decision. |
| T4-T5 | Test and limit | Commit after focused validation and review. |
| T5 | Documentation | Separate `docs(...)` commit if not part of the code change. |

## Acceptance Criteria

- [x] AC1: Whether HTTP scrape enforces the documented 74-hash cap is shown by
  recorded manual evidence (V1: it does not on the tested requests).
- [ ] AC2: HTTP scrape behavior with more than 74 info hashes matches the
      decided option and the documentation.
- [ ] AC4: An ADR records whether HTTP caps scrape requests, the value and
      behavior if so, and the reason per protocol (UDP packet size; HTTP
      policy), noting that parallel requests bypass a per-request cap and that
      rate limiting is EPIC #2411. Doc comments and user docs link it.
- [ ] AC3: A maintained test covers the decided behavior, or a documentation-only
  outcome explains why existing coverage suffices without a new test.
- [ ] `linter all` exits with code `0`; relevant tests pass.

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
| M2 | UDP scrape control | Send one 75-hash request with a valid connection ID to the local tracker; verify actual transmitted hash count so client truncation cannot hide server behavior | Records accepted hashes or transport rejection; parser control separately verifies its 74-hash cap | TODO |

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
- [ ] Manual baseline, reconsidered decision, and ADR recorded
- [ ] Implementation, automatic checks, and manual recheck completed
- [ ] Acceptance criteria re-reviewed and independent Task Reviewer report recorded

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

### Acceptance Verification

| AC ID | Status | Evidence |
| --- | --- | --- |
| AC1 | DONE | [V1: HTTP baseline](manual-verification-evidence.md#v1-documented-74-hash-cap); final implementation acceptance remains pending |
| AC2 | TODO | Approved decision, docs, and final recheck |
| AC3 | TODO | Maintained test or justified documentation-only outcome |
| AC4 | TODO | ADR, plus doc comments and user docs linking it |

## Implementation Completion Review

Record material discoveries in issue-local `implementation-retrospective.md`,
or explain in the progress log why none is needed. An independent Task Reviewer
checks acceptance criteria before the implementation PR; record its report in
`agent-review-reports.md`. Commit reproduction/recheck evidence with spec
updates after the required gates; a justified no-change outcome needs no empty
code commit.

## References

- Related issues: spam and abuse EPIC (draft)
- Related code: `MAX_SCRAPE_TORRENTS`
