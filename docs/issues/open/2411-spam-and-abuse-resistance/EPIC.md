---
schema-version: 1
doc-type: epic
status: planned
epic: null
github-issue: 2411
spec-path: docs/issues/open/2411-spam-and-abuse-resistance/EPIC.md
epic-owner: josecelano
last-updated-utc: "2026-10-02 17:28"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# EPIC #2411 - Spam and Abuse Resistance

## Goal

Keep one inventory of the ways clients can overload the tracker or its APIs,
then evaluate a shared rate-limiting design and any complementary controls.

## Why This Is Needed

The tracker lacks a shared configurable request-rate policy across its services.
Existing protections, including UDP connection-ID validation, banning, and
bounded active-request handling, must be inventoried rather than assumed absent.
Per-request limits bound individual work but do not bound aggregate work from
parallel requests. This EPIC collects the cases before selecting controls.

## Scope

### In Scope

- An inventory of abuse cases, updated whenever a new one is found.
- Linking existing issues as sub-issues.
- Later: a rate-limiting design (likely an ADR) and its implementation.

### Out of Scope

- Security vulnerabilities that must be reported privately (see
  `SECURITY.md`). Do not add undisclosed vulnerabilities to this public
  inventory.
- Implementing controls in this spec-only PR; implementation belongs to later
  child issues, after the inventory and design are reviewed.

## Abuse Case Inventory

Add a row for every new case. "Source" says where it was found.

| ID | Case | Affected | Effect | Status | Source |
| --- | --- | --- | --- | --- | --- |
| A1 | No shared configurable request-rate policy | UDP, HTTP, REST API, health check | Aggregate work can exceed capacity despite existing local protections | To inventory | Known limitation |
| A2 | Connections without a client timeout | HTTP, REST API | Idle connections hold resources | Open, needs research | #324 |
| A3 | HTTP scrape exceeds the documented 74 info-hash limit | HTTP tracker | Local requests returned 75 and 1000 entries; overload impact not measured | Count mismatch reproduced; whether HTTP should cap (and why) to be reconsidered and recorded in an ADR | [HTTP limit evidence](../2417-2411-verify-http-scrape-info-hash-limit/manual-verification-evidence.md) |
| A4 | Announce for new info hashes adds torrents to memory when policy permits | UDP, HTTP tracker | Memory is retained while peers remain active; expiry and optional peerless cleanup affect retention | To inventory | SI-22 session, 2026-10-02 |
| A5 | First announce of an unknown torrent reads the database when persistence is enabled | UDP, HTTP tracker | Database load from random info hashes | Open | `AnnounceHandler::load_downloads_metric_if_needed` |
| A6 | Scrape may read the database (and, with option B, grow memory) after the scrape bug fix | UDP, HTTP tracker | Same as A4 and A5, via scrape | Pending the fix | #2406 |

Notes:

- A3: the UDP scrape limit is enforced (`MAX_SCRAPE_TORRENTS` in
  `Request::parse_bytes`).
- A5: sequential announces avoid a database read while the torrent remains in
  memory. Eviction permits another read, and concurrent first announces may
  both observe a miss. A6's caching policy remains an implementation decision
  in #2406; do not assume it already exists.
- Case status means an inventory hypothesis unless reproduced. Confirm current
  behavior and existing controls before choosing a mitigation or publishing
  newly discovered security-sensitive details.

## Subissues

| Order | Issue | Local Spec | Status | Notes |
| --- | --- | --- | --- | --- |
| 1 | #324 - Denial of Service attack factor | None (pre-dates specs) | TODO | Existing open issue; linked as a sub-issue |
| 2 | #2417 - Verify whether HTTP scrape enforces the 74 info-hash limit | [ISSUE.md](../2417-2411-verify-http-scrape-info-hash-limit/ISSUE.md) | TODO | A3 |
| 3 | #[To be assigned] - Rate-limiting design | Not drafted | TODO | After the inventory is reviewed |

## Delivery Strategy

### Phase 1 - Inventory

- Outcome: the inventory lists known cases; existing issues are linked.
- Exit criteria: the maintainer reviews the inventory.

### Phase 2 - Design

- Outcome: a rate-limiting design (per IP, per service, per request type)
  evaluated against the inventory, recorded in a root ADR. Rate limiting is
  not assumed sufficient for idle connections, request sizes, retained state,
  distributed traffic, or shared-IP clients; assess complementary controls.
- Exit criteria: the ADR is accepted.

### Phase 3 - Implementation

- Outcome: sub-issues implement the design and close the inventory cases.

## Progress Tracking

### Workflow Checkpoints

- [x] Epic spec drafted in `docs/issues/drafts/`
- [ ] Epic spec reviewed and approved by user/maintainer
- [ ] GitHub epic issue created and issue number added to this spec
- [ ] #324 linked as a sub-issue
- [ ] Subissues created and linked in this spec

### Progress Log

- 2026-10-02 10:49 UTC - GitHub Copilot - Drafted the EPIC with the first
  inventory (A1-A6), at the maintainer's request during the #1488 SI-22
  session.

## Acceptance Criteria

- [ ] AC1: The inventory is reviewed, distinguishes hypotheses from evidence,
  and records existing controls.
- [ ] AC2: #324 and approved public child issues are linked; related issues
  owned elsewhere, including #2406, remain cross-references.
- [ ] AC3: The design maps every case to a mitigation or an explicit deferral.
- [ ] AC4: Each implemented child includes automatic checks, local manual
  evidence, and post-implementation acceptance and completion review.

### Acceptance Verification

| AC ID | Status | Evidence |
| --- | --- | --- |
| AC1 | TODO | Reviewed inventory |
| AC2 | TODO | GitHub parent-child links |
| AC3 | TODO | Accepted ADR and case-to-control mapping |
| AC4 | TODO | Child verification and completion records |

## Architectural Decisions

No existing ADR is selected yet. Phase 2 creates a root ADR because the policy
crosses protocol and API boundaries. Do not prescribe one algorithm before
measuring the cases and reviewing client identity behind proxies and NAT.

## Verification and Completion

Each child runs `linter all`, relevant tests, and required pre-push checks.
Manual checks use an isolated local tracker, never unsolicited public load,
and record commands, toolchain, limits, outcomes, and logs in issue-local
`manual-verification-evidence.md`. Review acceptance criteria after each child.
At EPIC closure, record material discoveries in an implementation retrospective
or state why none is needed. Keep future discoveries in this inventory until
closure, then assign a follow-up owner rather than preventing closure forever.

## Risks and Trade-offs

- Rate limiting can block legitimate heavy users, for example behind a NAT;
  the design must make limits configurable.
- A public inventory tells attackers what is weak; it lists only cases that
  are already public or generic, never private vulnerabilities.

## References

- Related issues: #324, #1510
- Security policy: `SECURITY.md`
