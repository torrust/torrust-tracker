---
schema-version: 1
doc-type: epic
status: blocked
epic: null
github-issue: 2491
spec-path: docs/issues/open/2491-i2p-peer-support/EPIC.md
epic-owner: null
last-updated-utc: "2026-10-08 16:45"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/discussions/AGENTS.md
    - docs/discussions/2491-i2p-peer-support/20261008-i2p-peer-support-design/README.md
    - "issue #1659"
    - "issue #144"
    - "issue #2411"
    - docs/issues/closed/1987-add-config-option-to-use-ip-from-announce-query-string/ISSUE.md
---

<!-- skill-link: create-issue -->

# EPIC #2491 - I2P Peer Support

## Goal

Let the tracker serve BitTorrent swarms whose peers use the
[I2P anonymity network](https://i2p.net/en/docs/applications/bittorrent/), without
weakening the tracker's clearnet behavior or its trust boundaries.

## Why This Is Needed

Jose Celano supports adding I2P peer support. A contributor proposed an implementation in
PR #2050, and Jose reviewed it in the rebased draft PR #2059. His review found that it needs a
careful design first: it changes the core peer address model, the HTTP announce contract, the
compact response format, swarm statistics, the REST API, and the tracker's trust boundary. The
review recorded seven required actions, including a merge blocker: a plain HTTP announce cannot
prove that a client owns the I2P Destination it supplies.

The maintainers are focused on releasing v4.0.0 (#1659). This EPIC is blocked until that release
ships and the roadmap is redefined. It exists so the work and its history are not lost when
PRs #2050 and #2059 are closed: closing them does not reject the feature.

## Scope

### In Scope

- The [design discussion](../../../discussions/2491-i2p-peer-support/20261008-i2p-peer-support-design/README.md)
  that collects the proposal, the review findings, and the open questions.
- Design decisions, with ADRs where a decision is architectural (for example, how the tracker
  trusts a peer's I2P Destination).
- I2P peer support in the HTTP tracker: announce parsing, network-isolated matchmaking, and the
  compact and non-compact response formats from the I2P BitTorrent specification.
- The required actions A1 to A7 from the PR #2059 review, kept as source material in the
  discussion folder.
- Follow-up work the review identified: tracker client support for I2P Destinations, and the
  documented placeholder port.

### Out of Scope

- Implementation before v4.0.0 (#1659) is released and the roadmap includes this EPIC.
- The REST API representation of I2P peers. It belongs to the REST API overhaul (#144) and needs
  an ADR there; until then, I2P peers must not change the existing `peer_addr` field.
- UDP over I2P, I2P PEX, and I2P DHT. They are separate I2P specifications; a later EPIC may add
  them.
- General rate limiting. The I2P design must account for fabricated-Destination spam, but shared
  rate-limiting controls belong to EPIC #2411.

## Subissues

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

No subissues yet. They are created after the design discussion records its Outcome. The review
actions in the discussion are the expected starting point.

| Order | Issue | Local Spec | Status | Notes |
| ----- | ----- | ---------- | ------ | ----- |
| -     | -     | -          | -      | -     |

## Delivery Strategy

For each subissue implementation in this EPIC, the default completion policy is:

1. Run automatic checks (`linter all`, relevant tests, pre-push checks when applicable).
2. Run manual verification scenarios and record evidence.
3. Re-review acceptance criteria after implementation and update verification evidence.
4. Complete an evidence-based implementation review. Create or update an
   issue-local retrospective for reusable lessons, material design changes, or
   meaningful deviations from the plan; otherwise record why one was unnecessary
   in the issue progress log.

### Phase 0 - Preserve the Proposal (now)

- Outcome: this EPIC and the design discussion are merged; PRs #2050 and #2059 are closed with a
  link to them.
- Exit criteria: the discussion links both PRs and keeps the review documents as source material.

### Phase 1 - Design Discussion (after v4.0.0)

- Outcome: maintainers and contributors add their positions in rounds, and the decision owner
  records the Outcome.
- Exit criteria: every topic has a decision, and each decision links the document that carries
  it (this EPIC, an ADR, or a subissue specification).

### Phase 2 - Subissues and ADRs

- Outcome: the decisions become ADRs and subissue specifications in implementation order.
- Exit criteria: the `Subissues` table lists every subissue and its dependencies.

### Phase 3 - Implementation

- Outcome: I2P peer support ships behind the agreed trust and abuse controls.
- Exit criteria: all subissues are done and the EPIC acceptance criteria are met.

## Progress Tracking

### Workflow Checkpoints

- [x] Epic spec drafted in `docs/issues/drafts/`
- [x] Epic spec reviewed and approved by user/maintainer
- [x] GitHub epic issue created and issue number added to this spec
- [ ] Design discussion merged and PRs #2050 and #2059 closed
- [ ] Discussion Outcome recorded (after v4.0.0)
- [ ] Subissues created and linked in this spec
- [ ] Subissue statuses kept up to date in the `Subissues` table
- [ ] For each implemented subissue: automatic checks completed and recorded
- [ ] For each implemented subissue: manual verification completed and recorded
- [ ] For each implemented subissue: acceptance criteria reviewed post-implementation
- [ ] For each implemented subissue: implementation completion review recorded
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update.

- 2026-10-08 15:45 UTC - Jose Celano/AI assistant using Copilot SDK in VS Code - Drafted the
  EPIC to keep the I2P proposal and its review after closing PRs #2050 and #2059; blocked until
  after v4.0.0 (#1659).
- 2026-10-08 15:45 UTC - Jose Celano/AI assistant using Copilot SDK in VS Code - Created
  #2491 and opened the design discussion with the PR #2059 review documents as source material.
- 2026-10-08 16:45 UTC - Jose Celano/AI assistant using Copilot SDK in VS Code - Corrected the
  review attribution after PR #2492 review finding F2.

## Acceptance Criteria

- [ ] AC1: The design discussion has a recorded Outcome, and each decision links its canonical
      document.
- [ ] AC2: An ADR records how the tracker establishes a peer's I2P Destination (review action A7).
- [ ] AC3: All required subissues are created and linked, in an explicit and justified order.
- [ ] AC4: Every review action (A1 to A7) is implemented by a subissue or explicitly rejected in
      the discussion Outcome.
- [ ] AC5: I2P and clearnet peers never receive each other in announce responses, and announce
      statistics describe only the peers reachable by the requester.
- [ ] AC6: Every completed subissue includes automated and manual verification evidence, a
      post-implementation acceptance criteria review, and an implementation completion review.
- [ ] AC7: User documentation describes how to deploy and operate the tracker for I2P peers.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   | -        |
| AC2   | TODO                   | -        |
| AC3   | TODO                   | -        |
| AC4   | TODO                   | -        |
| AC5   | TODO                   | -        |
| AC6   | TODO                   | -        |
| AC7   | TODO                   | -        |

## Risks and Trade-offs

- **Destination spoofing.** A plain HTTP announce does not prove ownership of a Destination, so
  an attacker can impersonate a peer or flood swarms with fabricated Destinations. Mitigation:
  decide the trust model in an ADR before any I2P announce is accepted.
- **Wide change surface.** Replacing `SocketAddr` with a peer address type touches most packages.
  Mitigation: decide the domain model first, and split delivery into reviewable subissues.
- **Stale proposal.** PRs #2050 and #2059 were written against `develop` in August 2026 and will
  need rework. Mitigation: treat them as reference, not as a branch to rebase blindly.

## References

- Related issues: #1659 (v4.0.0 release), #144 (REST API overhaul), #2411 (spam and abuse
  resistance), #1987 (`use_ip_from_query_string`)
- Related PRs: #2050 (original proposal), #2059 (rebased draft with the maintainer review)
- Related ADRs: none yet
- I2P BitTorrent specification: <https://i2p.net/en/docs/applications/bittorrent/>
