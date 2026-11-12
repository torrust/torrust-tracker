---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2851 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2851.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

Round 1 is the review `5879517733` of `da2ce7` (F1 to F7, reviewer-provided IDs) and Copilot's
review `5879649645` (F8 to F12, audit-local IDs). Both reviewed the first head of the pull request.
Copilot labels its five findings "Low severity"; the severity `Minor` is inferred from that label.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2851-f1 | Human | Major | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2851-f2 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2851-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2851-f4 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2851-f5 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2851-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2851-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2851-f8 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2851-f9 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2851-f10 | Copilot | Minor (inferred) | correctness | RE_RAISE_OF:F2 | NO_ACTION | SUPERSEDED |
| F11 | review-finding:pr-2851-f11 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2851-f12 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Closing keywords for #2838 and #2850 in two commit messages and in the description

- PR number: 2851
- Source review ID: 5879517733
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4554334119
- Concern: Two commit messages and two sentences of the description paired a closing keyword with #2838 or #2850, so the merge would have closed both issues before the implementation.
- Solution: Reworded the two commit messages (a history rewrite that kept every commit and the tree) and edited the description, which now also states the rewrite. The fix is outside the tree, so the resolution reference is the reply that states it.
- Current-tree verification: The closing-keyword scan of the `open-pull-request` skill printed nothing for the commit messages of `torrust/develop..HEAD`, only `Closes #2842` for the description, and nothing for the title.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555413190
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555413190

### F2 - The inventory says the UDP byte-count bug is fixed and UDP now rejects it

- PR number: 2851
- Source review ID: 5879517733
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4554334138
- Concern: The inventory row of the ADR said the bug was fixed in PR #2839, which was not merged, and called the current HTTP `i64` type old.
- Solution: The Note column says UDP still accepts the value and that the planned change is issue #2850; the HTTP column names both limits of the current parser.
- Current-tree verification: Read `packages/udp-core/src/peer_builder.rs`: `from_request` returns `peer::Peer` and copies the three counters with `.0.get()`.
- Resolution reference: docs(adrs): [#2842] state in the inventory that UDP still accepts a negative byte count
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555413695

### F3 - Point 6 describes the `udp-core` conversion as if it existed

- PR number: 2851
- Source review ID: 5879517733
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4554334155
- Concern: Point 6 of the ADR used the present tense for a fallible conversion that is not in the tree.
- Solution: Point 6 says that issue #2850 plans it.
- Current-tree verification: Read point 6 of the ADR in the tree; it reads "as issue #2850 plans for a negative `left` with a fallible conversion in `udp-core`".
- Resolution reference: docs(adrs): [#2842] describe the udp-core conversion as planned, not as existing
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555414073

### F4 - The Description attributes a sign to BEP 15 that the ADR later says BEP 15 does not state

- PR number: 2851
- Source review ID: 5879517733
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4554334169
- Concern: The Description said BEP 15 encodes `left` as a signed 64-bit integer, against the research paragraph of the same ADR.
- Solution: The Description says BEP 15 encodes a 64-bit integer, which the implementation document it cites types as `int64_t`.
- Current-tree verification: `grep -n -i signed` over the ADR and the specifications of #2842 and #2838 showed no other sentence that attributes the sign to BEP 15.
- Resolution reference: docs(adrs): [#2842] stop attributing the sign of left to BEP 15 in the description
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555414457

### F5 - Checkpoints and acceptance criteria report the unmerged patch as done

- PR number: 2851
- Source review ID: 5879517733
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4554334182
- Concern: The carried-over specification of #2838 reported implementation, tests and verification as done and cited tests that are not in the tree; the evidence file recorded an after-fix run without a note.
- Solution: The checkpoints and criteria are unchecked and marked superseded, the verification rows are TODO with the same mark, and V2 of the evidence file says the run used the branch of PR #2839.
- Current-tree verification: Read the three places in the tree after the commit; `grep -c '\[x\]'` in the Acceptance Criteria section of the #2838 specification is 0.
- Resolution reference: docs(issues): [#2838] mark the checkpoints, criteria and recheck of the unmerged patch as superseded
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555414853

### F6 - Section reference with no matching heading

- PR number: 2851
- Source review ID: 5879517733
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4554334200
- Concern: The #2850 specification referred to a section "Why This Is Needed", which is not a heading, and one row still said "outside this EPIC".
- Solution: The reference names "Background" and the row says "outside this issue".
- Current-tree verification: Compared every quoted section name after "see" in the #2850 specification with its headings; the only one without a heading there is a heading of the #2838 specification, cited as such.
- Resolution reference: docs(issues): [#2850] correct a section reference and a leftover mention of the EPIC
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555415274

### F7 - Section reference with no matching heading

- PR number: 2851
- Source review ID: 5879517733
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4554334213
- Concern: The #2838 specification referred to a section "Decision to approve", which does not exist.
- Solution: The sentence points to the Progress Log, where the decision is recorded.
- Current-tree verification: Read the sentence and the progress-log entry of 2026-11-12 09:15 UTC in the tree.
- Resolution reference: docs(issues): [#2838] point the field decision at the progress log
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555415700

### F8 - A negative `left` inflates the seeder count, not completed downloads

- PR number: 2851
- Source review ID: 5879649645
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4554433111
- Concern: The Description said a negative value counts as a completed download.
- Solution: The Description says it counts the peer as a seeder (the `complete` count).
- Current-tree verification: Read `Peer::is_seeder` in `packages/primitives/src/peer.rs` and the completed counter in `packages/swarm-coordination-registry/src/swarm/coordinator.rs`, which is incremented on a transition to `Completed`.
- Resolution reference: docs(adrs): [#2842] name the seeder count as the statistic a negative left inflates
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555416071

### F9 - Require parity at the final inbound acceptance boundary, not at the parser

- PR number: 2851
- Source review ID: 5879649645
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4554433196
- Concern: Point 7 asked for a parity test of both parsers, while point 6 lets the UDP parser accept a value that the conversion refuses.
- Solution: Point 7 asks for a parity test up to the point where the request is accepted or refused, including the wire-to-domain conversion.
- Current-tree verification: Read points 6 and 7 of the ADR in the tree after the commit.
- Resolution reference: docs(adrs): [#2842] require verdict parity at the acceptance boundary, not at the parser
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555416464

### F10 - Reference pending #2850; #2839 was closed without merging

- PR number: 2851
- Source review ID: 5879649645
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4554433267
- Concern: The inventory row said "UDP now rejects it" although PR #2839 was not merged. This is the concern of F2 on the same row.
- Solution: No further change: the fix of F2 already corrected the row.
- Current-tree verification: Read the inventory row in the tree; it says "UDP still accepts it; the planned change is issue #2850".
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555416881
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555416881

### F11 - UDP treats a non-positive `num_want` as the maximum; HTTP zero requests none

- PR number: 2851
- Source review ID: 5879649645
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4554433336
- Concern: The inventory said the tracker ignores the UDP value and did not record the leniency below `-1` or the different meaning of zero.
- Solution: The row describes both protocols as the code behaves.
- Current-tree verification: Read `PeersWanted::from_client_request` and `PeersWanted::only` in `packages/tracker-core/src/announce_handler.rs`, the UDP call in `packages/udp-core/src/services/announce.rs` and `peers_wanted` in `packages/http-core/src/services/announce.rs`.
- Resolution reference: docs(adrs): [#2842] describe how num_want is handled on each protocol in the inventory
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555417314

### F12 - HTTP rejects a non-IP announce `ip`; distinguish this from the ADR fallback

- PR number: 2851
- Source review ID: 5879649645
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4554433400
- Concern: The inventory listed a non-IP `ip` as ignored on HTTP, while the code rejects it.
- Solution: The row says HTTP rejects it, that it is not a leniency today, and that the linked `ip` ADR records an older silent fallback whose difference from the code needs its own follow-up.
- Current-tree verification: Read the `PeerIp` match in `packages/http-core/src/services/announce.rs`: `DnsName` and `Invalid` return errors; `Absent` and `Empty` use the connection IP.
- Resolution reference: docs(adrs): [#2842] record that HTTP rejects an announce ip that is not an IP address
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555417701

## Processing Log

- 2026-11-12 18:05 UTC - Started round 1 for review `5879517733` (submitted 18:02:39Z, forge `submitted_at`). Checked F1 to F7 against the tree.
- 2026-11-12 18:07 UTC - F1: rewrote the branch history to reword two commit messages. The four commits after "docs(adrs): [#2842] cite the client evidence for the byte counts" were replayed in the same order; `git diff` between the old head and the rewritten head was empty. The first review cites the old ids of two of them: `f0209ce97` is "docs(issues): [#2838] carry over the spec and evidence from PR #2839" and `8c28f2ccb` is "docs(adrs): [#2842] point the inbound tolerance ADR at issue #2850". Its reviewed head `4e77dd74b` is "docs(issues): [#2850] cite the commits of PR #2839 by subject".
- 2026-11-12 18:11 UTC - Fixes of F2 to F7 committed between 18:08:03Z and 18:11:27Z (`git log --format=%cI`), one commit each.
- 2026-11-12 19:30 UTC - The maintainer pushed the rewritten branch with `--force-with-lease`. Fetched the threads again and found Copilot's review `5879649645` (submitted 18:11:09Z) with five threads. Checked F8 to F12 against the tree; F10 is a re-raise of F2.
- 2026-11-12 19:36 UTC - Fixes of F8, F9, F11 and F12 committed between 19:33:34Z and 19:36:24Z (`git log --format=%cI`), one commit each, and pushed.
- 2026-11-12 19:37 UTC - Edited the pull request description for F1 and stated the history rewrite in it.
- 2026-11-12 19:38 UTC - Replied on the twelve threads between 19:38:23Z and 19:38:49Z (forge `created_at`).
- 2026-11-12 19:40 UTC - Started this audit record, after the replies, because it holds their URLs.
- 2026-11-12 19:41 UTC - `validate-audit-record.py --pr-number 2851` exited 0 (12 rows, 0 failures). Resolved the twelve threads; a GraphQL refresh showed 0 unresolved threads.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work. When the current identity cannot resolve a thread
  (`viewerCanResolve` is `false`), still reply, keep the disposition established from the current
  tree, and record `Thread state=OPEN`; the reviewer or PR author resolves it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
