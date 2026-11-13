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
Round 2 is the approving review `5881088057` of `da2ce7` (F13 to F16, reviewer-provided IDs), on
the head that carried the round 1 audit.
Round 3 is Copilot's review `5881522270` (F17 to F19, audit-local IDs), on the head that carried
the round 2 audit. It labels its three findings "Low severity"; the severity `Minor` is inferred from that label.
Round 4 is the approving review `5887584344` of `da2ce7` (F20 to F22, reviewer-provided IDs), on the head that carried
the round 3 audit.
Round 5 is the approving review `5887875398` of `da2ce7` (F23 and F24, reviewer-provided IDs) and Copilot's review
`5888919892` (F25 and F26, audit-local IDs; Copilot labels them F24 and F25), both on the head that carried the round 4 audit.
Round 6 is the review `5889368194` of `da2ce7` (COMMENTED, F27 to F31, audit-local IDs; the reviewer's own numbering,
which collides with F25 and F26 above, is recorded as the original), on the head that carried the round 5 spec changes
before the round 5 audit record; and the review `5889728498` of `da2ce7` (COMMENTED, F32 to F34, audit-local IDs; same
collision, with the reviewer's own F30 to F32 recorded as the original), on the head that carried the round 5 audit.

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
| F13 | review-finding:pr-2851-f13 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2851-f14 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2851-f15 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2851-f16 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2851-f17 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2851-f18 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-2851-f19 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2851-f20 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2851-f21 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-2851-f22 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-2851-f23 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F24 | review-finding:pr-2851-f24 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F25 | review-finding:pr-2851-f25 | Copilot | Minor | security | ORIGINAL | FIXED | RESOLVED |
| F26 | review-finding:pr-2851-f26 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F27 | review-finding:pr-2851-f27 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F28 | review-finding:pr-2851-f28 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F29 | review-finding:pr-2851-f29 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F30 | review-finding:pr-2851-f30 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F31 | review-finding:pr-2851-f31 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F32 | review-finding:pr-2851-f32 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F33 | review-finding:pr-2851-f33 | Human | Minor | process | ORIGINAL | FIXED | RESOLVED |
| F34 | review-finding:pr-2851-f34 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

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

### F13 - The audit cites three commit ids that will never be on `develop`

- PR number: 2851
- Source review ID: 5881088057
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555575897
- Concern: The processing log cited the ids of three commits of the branch as it was before the history rewrite. A review audit may not cite a PR-branch commit by id.
- Solution: The entry cites the commits by subject only, and a correction entry records that it was rewritten in place.
- Current-tree verification: Ran the commit-id check of the `open-pull-request` skill in a clone that still holds the first head; it printed nothing.
- Resolution reference: docs(pr-reviews): drop the commit ids of the rewritten branch from the audit of PR #2851
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555647329

### F14 - The force push is stamped 19:30; the forge records 19:34:12Z

- PR number: 2851
- Source review ID: 5881088057
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555575908
- Concern: The log stamped the force push before it happened and named the maintainer as the actor; the forge timeline records 19:34:12Z and the PR author's account.
- Solution: A correction entry gives the forge time and actor and also corrects the "and pushed" of the next entry. The original entries are not rewritten.
- Current-tree verification: Read `head_ref_force_pushed` in the forge timeline of the pull request: 2026-11-12T19:34:12Z, actor `josecelano-bot`.
- Resolution reference: docs(pr-reviews): correct the time and actor of the force push in the audit of PR #2851
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555647720

### F15 - Scenario M2 still reads DONE for the fixed build

- PR number: 2851
- Source review ID: 5881088057
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555575927
- Concern: The manual scenario M2 of the #2838 specification still read DONE for a build that is not on `develop`.
- Solution: M2 is TODO and its evidence cell carries the superseded mark.
- Current-tree verification: Read the Manual Verification Scenarios table of the #2838 specification in the tree.
- Resolution reference: docs(issues): [#2838] mark scenario M2 of the unmerged patch as superseded
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555648132

### F16 - `last-updated-utc` is unchanged by the edits

- PR number: 2851
- Source review ID: 5881088057
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555575950
- Concern: The evidence file of #2838 and the specifications of #2838 and #2850 were edited in round 1 and kept their earlier `last-updated-utc`.
- Solution: The evidence file and the #2850 specification carry the time of their last content commits; the #2838 specification was stamped with its M2 edit in the commit for F15.
- Current-tree verification: Compared the three `last-updated-utc` values with `git log --format=%cI` of each file.
- Resolution reference: docs(issues): [#2850] set last-updated-utc to the time of the last content edits
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555648558

### F17 - The probe neither checks nor shows the transaction id of the response

- PR number: 2851
- Source review ID: 5881522270
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555935594
- Concern: The UDP probe decoded the transaction id of each response and ignored it, so the planned recheck could not detect a response with a wrong id.
- Solution: The probe compares the id of the connect and the announce response with the id it sent and exits with status 1 on a mismatch. Its output for a correct response is unchanged.
- Current-tree verification: Ran the probe against a local fake UDP server: with the right id it printed the error response and exited 0; with a wrong id it printed a mismatch message on stderr and exited 1.
- Resolution reference: test(udp-probe): [#2838] reject a response with the wrong transaction id in the UDP probe
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4556040577

### F18 - Replaying the evidence fails because the database directory is not created

- PR number: 2851
- Source review ID: 5881522270
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555935674
- Concern: The recorded configuration stores the database under a directory that no step creates, and the tracker creates only the file.
- Solution: The prerequisites have a bullet, marked as added after the recorded runs, that creates the directory. The record of the runs is unchanged.
- Current-tree verification: Read the Environment and Prerequisites section of the evidence file and `path` in `evidence/tracker.toml` in the tree.
- Resolution reference: docs(issues): [#2838] add the database directory as a replay prerequisite of the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4556040964

### F19 - A REST check after all announces cannot show the earlier byte counts

- PR number: 2851
- Source review ID: 5881522270
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4555935730
- Concern: The probe fixes the info hash and the peer endpoint, so each announce replaces the previous peer; scenario M7 read the REST output only at the end.
- Solution: M7 reads the single-torrent detail right after each accepted announce, and the scenarios say why and that the torrent list has no peers.
- Current-tree verification: Read the Manual Verification Scenarios of the #2850 specification in the tree, and the fixed info hash and port in the probe.
- Resolution reference: docs(issues): [#2850] read the REST output after each accepted announce in scenario M7
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4556041507

### F20 - The round-3 review does carry a severity label

- PR number: 2851
- Source review ID: 5887584344
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4560762446
- Concern: This audit said Copilot's round-3 review gives no severity label; its body marks each of the three findings "Low severity".
- Solution: The round overview now says the review labels its three findings "Low severity" and that `Minor` is inferred from that label.
- Current-tree verification: Read the round overview of this record in the tree.
- Resolution reference: docs(pr-reviews): record round 4 in the review audit of PR #2851
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4560877430

### F21 - `last-updated-utc` again trails the edits of two files

- PR number: 2851
- Source review ID: 5887584344
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4560762450
- Concern: The evidence file and the #2850 specification carried times earlier than their last content edits (the same defect as F16).
- Solution: Both files now read 2026-11-12 20:32, the time of their last content edits in `git log`.
- Current-tree verification: Read the frontmatter of both files in the tree and `git log -1 --format=%aI` of each before this fix.
- Resolution reference: docs(issues): [#2850] carry the time of the last content edit in last-updated-utc of two files
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4560877678

### F22 - After the rebase, committer times no longer match the times this log attributes

- PR number: 2851
- Source review ID: 5887584344
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4560762460
- Concern: The rebase reset the committer time of the 24 commits up to the round-2 audit to 20:28:16Z to 20:28:32Z, so `git log --format=%cI` no longer gives the times that earlier entries cite; `%aI` keeps the original times, except for the last fix of the 19:36 range (19:35:03Z there).
- Solution: An appended entry in the processing log says so; no earlier entry was rewritten.
- Current-tree verification: Read the processing log in the tree.
- Resolution reference: docs(pr-reviews): record round 4 in the review audit of PR #2851
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4560877891

### F23 - `Reviewer finding ID` repeats IDs that were not reassigned

- PR number: 2851
- Source review ID: 5887875398
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4560991862
- Concern: The entries of F20, F21 and F22 set `Reviewer finding ID` to the reviewer's own IDs, which are not reassigned.
- Solution: The three entries read `N/A`, as those of F1 to F7 and F13 to F16.
- Current-tree verification: Read the three entries in the tree.
- Resolution reference: docs(pr-reviews): record round 5 in the review audit of PR #2851
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4561860121

### F24 - The correction for F22 does not name the entry stamped 20:01

- PR number: 2851
- Source review ID: 5887875398
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4560991869
- Concern: The correction for F22 names the entries stamped 18:11 and 19:36 and the F16 verification, not the one stamped 20:01, which also cites `%cI` for commits the rebase re-stamped.
- Solution: An appended log entry names the entry stamped 20:01; the entry of the correction for F22 stays as it is.
- Current-tree verification: Read the processing log in the tree and `git log --format=%cI` and `%aI` of the four round 2 fixes.
- Resolution reference: docs(pr-reviews): record round 5 in the review audit of PR #2851
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4561860498

### F25 - The replay configuration binds an unused listener on every interface

- PR number: 2851
- Source review ID: 5888919892
- Reviewer finding ID: F24
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4561799031
- Concern: `evidence/tracker.toml` bound a UDP listener on `0.0.0.0:6969` that no probe used, exposing the replay tracker beyond localhost.
- Solution: The listener is removed, and the evidence file records the removal as a replay-only adjustment.
- Current-tree verification: Read `evidence/tracker.toml` and the Environment section of the evidence file in the tree; every probe targets `127.0.0.1:6868`.
- Resolution reference: docs(issues): [#2838] remove the unused second UDP listener from the replay configuration
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4561860930

### F26 - The plan has no baseline and no UDP announce benchmark

- PR number: 2851
- Source review ID: 5888919892
- Reviewer finding ID: F25
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4561799123
- Concern: T2 changes every UDP announce but the plan only compiled the benches; `docs/benchmarking.md` requires a relevant microbenchmark, the load test, a baseline before production changes and a one-sided pass rule agreed before implementation.
- Solution: New task T0 adds the missing UDP announce benchmark in its own commit and records the baseline before T2; T6 repeats the measurement after T3; a Performance Evidence section states the instruments, the one-sided rule and the tolerance to agree for the UDP announce; evidence goes to `performance-evidence.md`.
- Current-tree verification: Read the Implementation Plan and the Performance Evidence section of the #2850 specification; checked that no `udp-core` or `udp-server` benchmark times an announce (`udp-core` times the connect, the connection cookie and the ban service; `udp-server` a scrape).
- Resolution reference: docs(issues): [#2850] plan the performance baseline and the missing UDP announce benchmark
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4561861311

### F27 - The audit record stops at round 4, and two replies cite a commit that is not on the branch

- PR number: 2851
- Source review ID: 5889368194
- Reviewer finding ID: F25
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4562135511
- Concern: The record had no row or log entry for the two reviews after round 4; two replies said a fix was "pushed right after this reply" in a commit subject that was not yet on the branch.
- Solution: The round-5 audit commit landed (rows F23 to F26, log entry at 09:50); this round adds rows for this review's own five findings, which the round-5 commit had not covered.
- Current-tree verification: Read the round 5 and round 6 sections of this record and `git log` of the branch.
- Resolution reference: docs(pr-reviews): record round 6 in the review audit of PR #2851
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4562440525

### F28 - `udp-core` has three benchmarks, not only `connect_once`

- PR number: 2851
- Source review ID: 5889368194
- Reviewer finding ID: F26
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4562135512
- Concern: The T0 row said "`udp-core` has only `connect_once`", omitting `ban_service_benchmark` and `connection_cookie_benchmark`; the conclusion (no announce benchmark) held regardless.
- Solution: The T0 row now reads "the `udp-core` benchmarks time only the connect, the connection cookie and the ban service"; the audit's own verification text for F26 above is corrected the same way (sweep).
- Current-tree verification: Read the T0 row of the #2850 specification and `packages/udp-core/Cargo.toml` lines 50 to 60 in the tree.
- Resolution reference: docs(issues): [#2850] correct benchmark wording, add a Commit Points row, and record the tolerance write-back
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4562440881

### F29 - T2 adds one check per counter, not one comparison per announce

- PR number: 2851
- Source review ID: 5889368194
- Reviewer finding ID: F27
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4562135522
- Concern: The Performance Evidence section said T2 "deliberately adds one comparison on every UDP announce"; the design converts each of `bytes_left`, `bytes_uploaded` and `bytes_downloaded` with `u64::try_from`, so it adds three checks.
- Solution: The section now reads "one sign check per counter — three checks on every UDP announce", naming the three counters and that `peer_builder.rs` copies them unchecked today.
- Current-tree verification: Read the Performance Evidence section and `packages/udp-core/src/peer_builder.rs` lines 23 to 25 in the tree.
- Resolution reference: docs(issues): [#2850] correct benchmark wording, add a Commit Points row, and record the tolerance write-back
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4562441227

### F30 - Commit Points has no row for T0

- PR number: 2851
- Source review ID: 5889368194
- Reviewer finding ID: F28
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4562135516
- Concern: The Commit Points table started at T1, with no row for the new T0 task, and `bench` is not a Conventional Commit type the repository recognizes.
- Solution: A T0 row was added: `test(udp-core)` for the benchmark, `docs(issues)` for the baseline in `performance-evidence.md`.
- Current-tree verification: Read the Commit Points table of the #2850 specification in the tree.
- Resolution reference: docs(issues): [#2850] correct benchmark wording, add a Commit Points row, and record the tolerance write-back
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4562441604

### F31 - Say where the agreed tolerance is written

- PR number: 2851
- Source review ID: 5889368194
- Reviewer finding ID: F29
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4562135525
- Concern: The Expected cost bullet asked the maintainer to agree a tolerance after the baseline but did not say where that figure would be recorded, so the after measurement could be judged against a figure the plan never states.
- Solution: The bullet now ends "...and before the after measurement, and is then added to this section".
- Current-tree verification: Read the Performance Evidence section of the #2850 specification in the tree.
- Resolution reference: docs(issues): [#2850] correct benchmark wording, add a Commit Points row, and record the tolerance write-back
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4562442024

### F32 - The round-5 log entry places the F23 and F24 fix before its replies

- PR number: 2851
- Source review ID: 5889728498
- Reviewer finding ID: F30
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4562413702
- Concern: The round-5 entry said the F23/F24 fix was committed and pushed before the 09:46 replies, and gave a placeholder time "09:5x"; in fact that fix is `c237cca8b`, committed at 10:38:07Z, 52 minutes after those replies.
- Solution: An appended correction entry gives the forge time of the replies and the actual commit time of the fix; the original entry stays as it is, per the Completion Rules' correction convention.
- Current-tree verification: Read the round-5 log entry and the appended correction, and `git log --format=%cI` of `c237cca8b`.
- Resolution reference: docs(pr-reviews): record round 6 in the review audit of PR #2851
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4562443275

### F33 - The five threads of review `5889368194` were resolved with no reply and no row

- PR number: 2851
- Source review ID: 5889728498
- Reviewer finding ID: F31
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4562413688
- Concern: A blanket GraphQL resolve call closed those five threads before they were read or replied to, against the skill's rule that a reply precedes resolution; the record had no row for them.
- Solution: All five threads were reopened, replied to with their disposition and verification (this round's F27 to F31), and resolved again after the replies.
- Current-tree verification: Read the five threads on the forge: each now has a reply before its resolution, and rows F27 to F31 above.
- Resolution reference: docs(pr-reviews): record round 6 in the review audit of PR #2851
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4562442447

### F34 - This verification repeats the `udp-core` benchmark claim of F26

- PR number: 2851
- Source review ID: 5889728498
- Reviewer finding ID: F32
- Source URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4562413698
- Concern: The audit's own current-tree verification for F26 (round 5) repeated the inaccurate "only `connect_once`" claim that F28 (this round) corrects in the specification.
- Solution: The verification text for F26 above now reads "no `udp-core` or `udp-server` benchmark times an announce", matching the corrected specification wording.
- Current-tree verification: Read the F26 section of this record in the tree.
- Resolution reference: docs(issues): [#2850] correct benchmark wording, add a Commit Points row, and record the tolerance write-back
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2851#discussion_r4562442841

## Processing Log

- 2026-11-12 18:05 UTC - Started round 1 for review `5879517733` (submitted 18:02:39Z, forge `submitted_at`). Checked F1 to F7 against the tree.
- 2026-11-12 18:07 UTC - F1: rewrote the branch history to reword two commit messages. The four commits after "docs(adrs): [#2842] cite the client evidence for the byte counts" were replayed in the same order; `git diff` between the old head and the rewritten head was empty. The first review cites two of them and its reviewed head by their earlier ids: "docs(issues): [#2838] carry over the spec and evidence from PR #2839", "docs(adrs): [#2842] point the inbound tolerance ADR at issue #2850" and, as the reviewed head, "docs(issues): [#2850] cite the commits of PR #2839 by subject".
- 2026-11-12 18:11 UTC - Fixes of F2 to F7 committed between 18:08:03Z and 18:11:27Z (`git log --format=%cI`), one commit each.
- 2026-11-12 19:30 UTC - The maintainer pushed the rewritten branch with `--force-with-lease`. Fetched the threads again and found Copilot's review `5879649645` (submitted 18:11:09Z) with five threads. Checked F8 to F12 against the tree; F10 is a re-raise of F2.
- 2026-11-12 19:36 UTC - Fixes of F8, F9, F11 and F12 committed between 19:33:34Z and 19:36:24Z (`git log --format=%cI`), one commit each, and pushed.
- 2026-11-12 19:37 UTC - Edited the pull request description for F1 and stated the history rewrite in it.
- 2026-11-12 19:38 UTC - Replied on the twelve threads between 19:38:23Z and 19:38:49Z (forge `created_at`).
- 2026-11-12 19:40 UTC - Started this audit record, after the replies, because it holds their URLs.
- 2026-11-12 19:41 UTC - `validate-audit-record.py --pr-number 2851` exited 0 (12 rows, 0 failures). Resolved the twelve threads; a GraphQL refresh showed 0 unresolved threads.
- 2026-11-12 19:57 UTC - Round 2: review `5881088057` of `da2ce7` (APPROVED, submitted 19:53:43Z, forge `submitted_at`) with four findings, F13 to F16, on the audit and the #2838 records.
- 2026-11-12 19:57 UTC - Correction for F13: the entry stamped 18:07 was rewritten in place to remove three commit ids of the branch as it was before the history rewrite; it now cites those commits by subject only. No other entry was rewritten.
- 2026-11-12 19:57 UTC - Correction for F14: the entry stamped 19:30 has the wrong time and actor. The forge timeline records the force push (`head_ref_force_pushed`) at 19:34:12Z with `josecelano-bot` as the actor; the maintainer ran `git-bot-push --force-with-lease`, which pushes as that account. The entry stamped 19:36 says "and pushed": the commits are stamped up to 19:36:24Z and their push landed later, before the first reply at 19:38:23Z, whose forge record names that head.
- 2026-11-12 20:01 UTC - Round 2: the four fixes were committed between 19:55:16Z and 19:57:12Z (`git log --format=%cI`) and their push had landed by 20:00:04Z (`git ls-remote` of the fork matched the local head). Replied on the four threads between 20:00:05Z and 20:00:12Z (forge `created_at`). `validate-audit-record.py --pr-number 2851` exited 0 (16 rows, 0 failures). Resolved the four threads; a GraphQL refresh showed 0 unresolved threads.
- 2026-11-12 20:38 UTC - Round 3: Copilot's review `5881522270` (submitted 20:28:18Z, forge `submitted_at`) with three findings, F17 to F19. Before it was read, the branch had been rebased onto `develop`, because the pull request conflicted in the sub-issue table of EPIC #1347 after PR #2841 merged; every commit was replayed in the same order. The three fixes were committed between 20:31:34Z and 20:32:42Z (`git log --format=%cI`). The forge timeline records the force push of the rebased branch at 20:37:18Z (`head_ref_force_pushed`, actor `josecelano-bot`; the maintainer ran `git-bot-push --force-with-lease`). Edited the pull request description to state the rebase. Replied on the three threads between 20:37:44Z and 20:37:50Z (forge `created_at`). `validate-audit-record.py --pr-number 2851` exited 0 (19 rows, 0 failures). Resolved the three threads; a GraphQL refresh showed 0 unresolved threads.
- 2026-11-13 07:44 UTC - Round 4: the approving review `5887584344` of `da2ce7` (submitted 2026-11-13 07:28:03Z, forge `submitted_at`) at the head that carried the round 3 audit, with three nits, F20 to F22. The F21 fix was committed and pushed first; the replies on the three threads follow it (forge `created_at`). Correction for F22: the rebase set the committer time of the 24 commits up to the round 2 audit to 20:28:16Z to 20:28:32Z, so `git log --format=%cI` does not give the times that the entries stamped 18:11, 19:36 and the F16 verification cite. `git log --format=%aI` keeps the original times (18:08:03Z, 18:11:27Z, 19:33:34Z, 19:55:16Z, 19:57:12Z), except that the last fix of the 19:36 range shows 19:35:03Z there, not 19:36:24Z. No earlier entry was rewritten.
- 2026-11-13 09:50 UTC - Round 5: the approving review `5887875398` of `da2ce7` (forge `submitted_at` 2026-11-13 08:00:46Z) with two nits, F23 and F24, and Copilot's review `5888919892` (submitted 09:39:31Z, forge `submitted_at`) with two findings, F25 and F26, on the head that carried the round 4 audit. Their fixes were committed and pushed before the replies on the four threads (forge `created_at` 09:5x). Correction for F24: the entry stamped 20:01 cites `git log --format=%cI` for the four round 2 fixes (19:55:16Z to 19:57:12Z); after the rebase `%cI` shows 20:28:31Z for them, and `%aI` keeps 19:55:16Z to 19:57:12Z. No earlier entry was rewritten.
- 2026-11-13 11:10 UTC - Round 6: two reviews of `da2ce7` that had not yet been processed. `5889368194` (submitted 10:21:06Z, forge `submitted_at`) with five findings in the reviewer's own numbering F25 to F29, on the head that carried the round-5 spec commits before the round-5 audit landed; its five threads had been resolved earlier by a blanket GraphQL call with no reply, against the skill's rule. `5889728498` (submitted 10:54:09Z, forge `submitted_at`) with three findings, reviewer's own F30 to F32, on the head that carried the round-5 audit. Both sets of reviewer IDs collide with existing audit IDs (F25, F26, F30, F31), so new audit-local IDs F27 to F34 were assigned, recording the originals. The five wrongly-resolved threads were reopened (`unresolveReviewThread`), then all eight threads were replied to between 11:04:40Z and 11:07:55Z (forge `created_at`), apologizing on the five for the process error. The spec fix (F28, F29, F30, F31, F34 wording) was committed as `docs(issues): [#2850] correct benchmark wording, add a Commit Points row, and record the tolerance write-back` and the audit rows and sections as `docs(pr-reviews): record round 6 in the review audit of PR #2851`, then pushed. `validate-audit-record.py --pr-number 2851` exited 0 (34 rows, 0 failures). Resolved all eight threads, in order, after their replies; a GraphQL refresh showed 0 unresolved threads.
- 2026-11-13 11:10 UTC - Correction for F32: the round-5 log entry (stamped 09:50) says the F23/F24 fix was committed and pushed before the four replies, with a placeholder time "09:5x". The fix is commit `c237cca8b`, authored and committed at 10:38:07Z (`git log --format=%aI`/`%cI`), 52 minutes after the four replies (forge `created_at` 09:46:22Z to 09:46:28Z). The listener and benchmark-plan fixes (`9b4f2de6d`, pushed by 09:46:13Z) are unaffected by this correction. No earlier entry is rewritten.

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
