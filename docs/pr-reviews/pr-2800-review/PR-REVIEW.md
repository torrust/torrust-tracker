---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2800 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2800.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2800-f1 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2800-f2 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2800-f3 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2800-f4 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2800-f5 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2800-f6 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2800-f7 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2800-f8 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2800-f9 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2800-f10 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2800-f11 | Copilot | Minor (inferred) | correctness | RE_RAISE_OF:F2 | NO_ACTION | SUPERSEDED |
| F12 | review-finding:pr-2800-f12 | Copilot | Minor (inferred) | correctness | RE_RAISE_OF:F3 | NO_ACTION | SUPERSEDED |
| F13 | review-finding:pr-2800-f13 | Copilot | Minor | testing | RE_RAISE_OF:F4 | NO_ACTION | SUPERSEDED |
| F14 | review-finding:pr-2800-f14 | Copilot | Minor | documentation | RE_RAISE_OF:F5 | NO_ACTION | SUPERSEDED |
| F15 | review-finding:pr-2800-f15 | Human | Minor | documentation | RE_RAISE_OF:F6 | FIXED | RESOLVED |
| F16 | review-finding:pr-2800-f16 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2800-f17 | Human | Nit | maintainability | ORIGINAL | NO_ACTION | OPEN |

## Finding Details

### F1 - Correct the T1 history of the peer-id-only Ord

- PR number: 2800
- Source review ID: 5842317457
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525209694
- Concern: The spec said the peer_id-only Ord predates 00af70f25 and was already present after the 9a43815d0 move; it was introduced by 00af70f25, replacing an all-field derive added by e18cae46e.
- Solution: Replaced the history line with the three commits and what each did, and said T1 finds why 00af70f25 changed the ordering.
- Current-tree verification: Ran `git show 00af70f25`, `git show e18cae46e` and `git log -S'impl Ord for Peer'`; read the T1 History bullet.
- Resolution reference: docs(issues): [#2798] address review round 1 of the Peer Ord spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525271988

### F2 - Name the production swarm and its SocketAddr key

- PR number: 2800
- Source review ID: 5842317457
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525209706
- Concern: The spec called the PeerId-keyed benchmark map the swarm; production keys by SocketAddr in the Coordinator.
- Solution: Named Coordinator in swarm-coordination-registry and its key in the impact bullet and the audit, moved the benchmark map to the PeerList/Torrent bullet, and added the same-id-different-address case to the Intent section.
- Current-tree verification: Read `coordinator.rs` (`peers: BTreeMap<SocketAddr, Arc<PeerAnnouncement>>`) and the edited bullets.
- Resolution reference: docs(issues): [#2798] address review round 1 of the Peer Ord spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525274208

### F3 - State the projection of the hand-written Ord variant

- PR number: 2800
- Source review ID: 5842317457
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525209714
- Concern: A tuple of the seven raw fields is not Ord because NumberOfBytes and AnnounceEvent lack Ord.
- Solution: Specified comparing the inner i64 of the byte counts and a private rank per AnnounceEvent variant, and aligned row C with both variants.
- Current-tree verification: Read `number_of_bytes.rs` and `announce.rs` derives and the edited recommendation.
- Resolution reference: docs(issues): [#2798] address review round 1 of the Peer Ord spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525272332

### F4 - Add the progressive test-design review to the plan

- PR number: 2800
- Source review ID: 5842317457
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525209725
- Concern: The plan omitted the write-unit-test loop, the recorded design review and the maintainer stop that create-issue requires.
- Solution: Added them to T4, the Regression Test Strategy and the Commit Points.
- Current-tree verification: Read T4, the strategy paragraph and the Commit Points table.
- Resolution reference: docs(issues): [#2798] address review round 1 of the Peer Ord spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525275811

### F5 - Restore template rows and completion-review conditions

- PR number: 2800
- Source review ID: 5842317457
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525209734
- Concern: The checkpoint list dropped template rows, the completion review lost its conditions, and Commit Points had no T1 row.
- Solution: Restored the rows and conditions from docs/templates/ISSUE.md and added a T1 commit point.
- Current-tree verification: Compared the checkpoint list and completion review with the template.
- Resolution reference: docs(issues): [#2798] address review round 1 of the Peer Ord spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525273430

### F6 - Rebuild the evidence file from its template

- PR number: 2800
- Source review ID: 5842317457
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525209742
- Concern: The evidence file departed from the template structure and had no step timestamps.
- Solution: Rebuilt it from the template, added symptom, hypothesis and test boundary, and re-ran the reproduction with stamps. A reviewer then found the stamps still overlapped (F15).
- Current-tree verification: Read the evidence file headings against the template.
- Resolution reference: docs(issues): [#2798] address review round 1 of the Peer Ord spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525271616

### F7 - Resolve the unpublished F1 and review-note references

- PR number: 2800
- Source review ID: 5842317457
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525209754
- Concern: The references to the primitives test finding and to the duplicate-peers review note resolved to nothing in the tree.
- Solution: Said the primitives test spec is unpublished and summarized the finding, and cited the code-review comment above PeerList.
- Current-tree verification: Read References, Out of Scope and the `peer_list.rs` comment.
- Resolution reference: docs(issues): [#2798] address review round 1 of the Peer Ord spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525274628

### F8 - Add Files touched and Validation performed to the PR description

- PR number: 2800
- Source review ID: 5842317457
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525209766
- Concern: The PR description lacked the Files touched and Validation performed items.
- Solution: Edited the PR description to add both sections.
- Current-tree verification: `gh pr view 2800 --json body` shows both headings.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2800#issuecomment-6768744960
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525272688

### F9 - Use real times in last-updated-utc and the log

- PR number: 2800
- Source review ID: 5842317457
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525209775
- Concern: The 12:45 stamp was later than the commit carrying it.
- Solution: Set the earlier entry to the commit time and stamped the new entries with the time they were written.
- Current-tree verification: Compared the log stamps with `git log --format=%cI`.
- Resolution reference: docs(issues): [#2798] address review round 1 of the Peer Ord spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525275433

### F10 - Drop the sort and remove-equal-neighbors example

- PR number: 2800
- Source review ID: 5842317457
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525209786
- Concern: The example does not lose different peers; it fails the other way.
- Solution: Dropped it; the remaining examples fit.
- Current-tree verification: Read the Contract violated paragraph.
- Resolution reference: docs(issues): [#2798] address review round 1 of the Peer Ord spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525275039

### F11 - Production Coordinator keys by SocketAddr (repeat of F2)

- PR number: 2800
- Source review ID: 5842388394
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525254965
- Concern: The production Coordinator stores a SocketAddr-keyed map, not the PeerId-keyed one cited.
- Solution: None beyond F2.
- Current-tree verification: Re-read the impact bullet; it names Coordinator and its SocketAddr key.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525506273
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525506273

### F12 - Hand-written variant needs a projection (repeat of F3)

- PR number: 2800
- Source review ID: 5842388394
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525255033
- Concern: The handwritten alternative cannot compile as described; specify byte counts and an event rank.
- Solution: None beyond F3.
- Current-tree verification: Re-read the recommendation; it states the projection.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525506721
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525506721

### F13 - Include the test-design review (repeat of F4)

- PR number: 2800
- Source review ID: 5842388394
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525255072
- Concern: The regression plan omits the per-increment test-design review.
- Solution: None beyond F4.
- Current-tree verification: Re-read T4 and the Regression Test Strategy.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525507066
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525507066

### F14 - Define how the completion review is resolved (repeat of F5)

- PR number: 2800
- Source review ID: 5842388394
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525255121
- Concern: The completion review only says Not yet assessed.
- Solution: None beyond F5.
- Current-tree verification: Re-read Implementation Completion Review; it states the retrospective and progress-log conditions.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525507399
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525507399

### F15 - Give each reproduction step a start and an end stamp

- PR number: 2800
- Source review ID: 5842657896
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525443691
- Concern: Step 1 had one stamp equal to step 2 start; no step had an end time.
- Solution: Re-ran the reproduction and recorded a start and an end stamp for each of the three steps.
- Current-tree verification: Read V1 Steps Performed; the stamps are strictly ordered.
- Resolution reference: docs(issues): [#2798] stamp start and end of each reproduction step
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525507788

### F16 - Add the review audit record

- PR number: 2800
- Source review ID: 5842657896
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525443699
- Concern: The head has no docs/pr-reviews/pr-2800-review/PR-REVIEW.md; the second review reused IDs; repeats need Superseded by replies; F8 needs a PR conversation response.
- Solution: Added this record with new audit-local IDs, the exact Superseded by replies, and the F8 conversation response.
- Current-tree verification: Ran `validate-audit-record.py --pr-number 2800` (see the Processing Log).
- Resolution reference: docs(pr-reviews): [#2798] record the PR 2800 review audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525510533

### F17 - One commit per independent fix

- PR number: 2800
- Source review ID: 5842657896
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525443706
- Concern: One commit carried every fix of round 1.
- Solution: Not changed for pushed commits: rewriting history is not offered by the push wrapper. Round 2 used one commit per fix.
- Current-tree verification: `git log` shows the F15 fix and this record as separate commits.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525508087
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2800#discussion_r4525508087

## Processing Log

- 2026-11-10 13:07 UTC - Review 5842317457 (`da2ce7`) submitted with findings F1 to F10; review 5842388394 (Copilot) submitted at 13:12 UTC.
- 2026-11-10 13:10 UTC - Committed the round 1 fixes as "docs(issues): [#2798] address review round 1 of the Peer Ord spec" (commit time 13:10:38 UTC).
- 2026-11-10 13:14 UTC - Replied on the 14 threads of the first two reviews (`created_at` 13:13:55 to 13:14:25 UTC) and resolved them.
- 2026-11-10 13:31 UTC - Review 5842657896 (`da2ce7`) submitted with F15 to F17 (its IDs F6, F11, F12), after the round 1 push.
- 2026-11-10 13:34 UTC - Committed the F15 fix as "docs(issues): [#2798] stamp start and end of each reproduction step" (commit time 13:34:15 UTC) and pushed it.
- 2026-11-10 13:35 UTC - Posted the PR conversation response for F8 (`created_at` 13:35:42 UTC).
- 2026-11-10 13:38 UTC - Posted the `Superseded by` replies on the threads of F11 to F14 (13:37:33 to 13:37:40 UTC), the replies on F15, F17 (13:37:43 and 13:37:45 UTC) and F16 (13:38:00 UTC).
- 2026-11-10 13:44 UTC - Pushed the audit record (commit "docs(pr-reviews): [#2798] record the PR 2800 review audit", commit time 13:40:53 UTC; `validate-audit-record.py --pr-number 2800` printed `"status": "ok"` with 0 failures) and then resolved the F15 and F16 threads, each returning `isResolved: true`. The F17 thread stays open for the reviewer.

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
