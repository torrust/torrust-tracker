---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2629 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2629>.

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

## Findings

Round 1: da2ce7's review 5631559010 (APPROVED, on the opening head, the commit whose subject is "docs(issues): [#2628] specify moving the test-helpers protocol helpers") carries four inline threads, F1 to F4, and an ACK of that head, which is stale after the fix commits. Copilot's review 5631574526 (COMMENTED, same head) says "No open issues" and opens no thread, so it raises no actionable assertion and adds no row. Severities are the labels of the source comments.

Round 2: da2ce7's review 5631788587 (APPROVED, submitted 2026-10-23T17:04:09Z, after the round 1 fixes and audit, dismissed by the 17:25:57Z push) carries three nit threads, F5 to F7, next to an ACK of the head that carried the round 1 audit. Severities are the labels of the source comments.

Round 3: da2ce7's review 5632201391 (APPROVED, submitted 2026-10-23T17:40:24Z, on the head that carried the round 2 audit, dismissed by the 18:02:11Z push of the F8 to F10 fixes) carries three nit threads, F8 to F10, about this audit record, next to an ACK of that head. Severities are the labels of the source comments.

Round 4: da2ce7's review 5633236549 (APPROVED, submitted 2026-10-23T19:07:54Z, on the head that carried the round 3 audit, dismissed by the 2026-10-24T08:13:53Z push of the F11 fix) carries one nit thread, F11, about this audit record, next to an ACK of that head. Severities are the labels of the source comments.

Round 5: da2ce7's review 5637522963 (APPROVED, submitted 2026-10-24T08:28:21Z, on the head that carried the round 4 audit, dismissed by the 2026-10-24T12:20:39Z push of the F12 fix) carries one nit thread, F12, about this audit record, next to an ACK of that head. Severities are the labels of the source comments.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2629-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2629-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2629-f3 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2629-f4 | Human | Nit | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2629-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2629-f6 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2629-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2629-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2629-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2629-f10 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2629-f11 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2629-f12 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Progress Log line filed under References

- PR number: 2629
- Source review ID: 5631559010
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4361996049>
- Concern: the EPIC row 2 Progress Log line sat under `## References`, not under `### Progress Log`.
- Solution: the entry is under `### Progress Log`.
- Current-tree verification: read the EPIC headings and the entry's position.
- Resolution reference: docs(issues): [#2628] file the EPIC row 2 progress-log line under Progress Log
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362035742>

### F2 - Check scope wider than AC3

- PR number: 2629
- Source review ID: 5631559010
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4361996055>
- Concern: the spec said ADR rule B.1 becomes a check, which is wider than the entries AC3 adds.
- Solution: the spec states the check covers `client-lib` and the three domain crates and that B.1 stays a convention for the others.
- Current-tree verification: re-read the spec's deny.toml scope and AC3.
- Resolution reference: docs(issues): [#2628] state which crates the deny.toml check covers
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362036104>

### F3 - Spec frontmatter branch and status

- PR number: 2629
- Source review ID: 5631559010
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4361996061>
- Concern: the spec frontmatter `branch:` should name the spec branch and `status:` should be `planned` while the spec PR is open.
- Solution: `branch:` is the `-spec` branch and `status:` is `planned`.
- Current-tree verification: read the frontmatter.
- Resolution reference: docs(issues): [#2628] name the spec branch and keep status planned until the spec PR merges
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362036447>

### F4 - Drafted checkpoint lost the drafts path

- PR number: 2629
- Source review ID: 5631559010
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4361996069>
- Concern: the ticked checkpoint about drafting the spec named the `open/` path though it was drafted under `drafts/`.
- Solution: the checkpoint names the `drafts/` path again.
- Current-tree verification: read the checkpoint line.
- Resolution reference: docs(issues): [#2628] restore the draft path in the drafted checkpoint
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362036692>

### F5 - Rule B.3 misread

- PR number: 2629
- Source review ID: 5631788587
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362177235>
- Concern: the spec said "as rule B.3 already is", which misreads B.3.
- Solution: the sentence now says B.1 stays a convention, as rule B.3 already says.
- Current-tree verification: read the sentence and ADR rule B.3.
- Resolution reference: docs(issues): [#2628] say that rule B.3 already keeps B.1 a convention
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362373141>

### F6 - EPIC and inventory not restamped

- PR number: 2629
- Source review ID: 5631788587
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362177245>
- Concern: the EPIC and the inventory were edited in this PR but their `last-updated-utc` was not restamped.
- Solution: both files carry a fresh stamp.
- Current-tree verification: read the frontmatter of both files.
- Resolution reference: docs(issues): [#2628] restamp the EPIC and inventory edited by the spec PR
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362373424>

### F7 - Processing Log misstates the order of events

- PR number: 2629
- Source review ID: 5631788587
- Reviewer finding ID: F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362177253>
- Concern: the single round 1 entry misstates the order of fixes, pushes and replies and omits the bot review.
- Solution: the log is append-only, so a new entry records the actual order, see the second correction entry in the Processing Log below.
- Current-tree verification: compared the entry with the review and comment timestamps from the GitHub API.
- Resolution reference: docs(pr-reviews): [#2628] audit review round 2 of the spec PR
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362373718>

### F8 - Correction entry repeats the defects it corrects

- PR number: 2629
- Source review ID: 5632201391
- Reviewer finding ID: F8
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362501793>
- Concern: the 17:27 correction entry is stamped later than its own commit and misstates the round 2 order.
- Solution: the log is append-only, so a second correction entry records the real stamp and order, and the F7 row points to it.
- Current-tree verification: compared the entry with the commit time and the timeline and reply timestamps from the GitHub API.
- Resolution reference: docs(pr-reviews): [#2628] correct the round 2 order in the audit processing log
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362721896>

### F9 - Round 2 review recorded as COMMENTED

- PR number: 2629
- Source review ID: 5632201391
- Reviewer finding ID: F9
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362501810>
- Concern: review 5631788587 was an approval, not COMMENTED, as the timeline's `review_dismissed` event shows.
- Solution: the round 2 paragraph says APPROVED, submitted 17:04:09Z, dismissed by the 17:25:57Z push.
- Current-tree verification: read the timeline event and the paragraph.
- Resolution reference: docs(pr-reviews): [#2628] record the round 2 review as approved
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362722180>

### F10 - F7 cites a reply URL for a fix in the tree

- PR number: 2629
- Source review ID: 5632201391
- Reviewer finding ID: F10
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362501818>
- Concern: F7's fix lives in the tree, so its resolution reference must be a commit subject, not a reply URL.
- Solution: the F7 resolution reference is the commit subject; the reply URL stays in the Reply URL field.
- Current-tree verification: read the F7 details against the skill's resolution-reference rule.
- Resolution reference: docs(pr-reviews): [#2628] cite the commit subject for the F7 fix
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362722449>

### F11 - Round 3 review state missing and loose log list

- PR number: 2629
- Source review ID: 5633236549
- Reviewer finding ID: F11
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4363287210>
- Concern: the round 3 paragraph gave review 5632201391 no state although it was APPROVED, and a blank line made the Processing Log a loose list.
- Solution: the paragraph says APPROVED, submitted 17:40:24Z, dismissed by the 18:02:11Z push, and the blank line is removed.
- Current-tree verification: read the timeline `review_dismissed` event and the file lines.
- Resolution reference: docs(pr-reviews): [#2628] record the round 3 review state and tighten the log
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4366497200>

### F12 - Round 4 entry has the F11 reply and push in the wrong order

- PR number: 2629
- Source review ID: 5637522963
- Reviewer finding ID: F12
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4366531797>
- Concern: the 08:15 entry says the F11 fix was pushed and then replied to, but the reply (08:13:52Z) came one second before the push (08:13:53Z).
- Solution: the log is append-only, so a new correction entry records the real order.
- Current-tree verification: compared the entry with the reply timestamp and the timeline `review_dismissed` event.
- Resolution reference: docs(pr-reviews): [#2628] correct the F11 reply and push order in the audit log
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4367098194>

## Processing Log

- 2026-10-23 16:49 UTC - Started audit. da2ce7 reviewed at 16:44 UTC (4 threads, F1 to F4). Verified each claim, committed one fix each, rebased onto `develop`, pushed and replied on every thread; resolved all 4 threads at 16:48 UTC (GraphQL showed 0 unresolved). The ACK names the head before the fixes, so a new ACK is needed on the head that carries this audit record.
- 2026-10-23 17:27 UTC - Correction of the entry above, from the GitHub API timestamps. da2ce7's review 5631559010 was submitted at 16:43:57Z and Copilot's review 5631574526 at 16:45:24Z. The reply on F1 was posted at 16:48:23Z, one second before the force-push at 16:48:24Z, and the replies on F2 to F4 at 16:48:25Z to 16:48:29Z, after it. Round 2: da2ce7's review 5631788587 (17:04:09Z) raised F5 to F7; each got its own fix commit, a plain push, a reply (17:25:56Z to 17:26:00Z) and was then resolved; this audit commit records it. A new ACK is needed on the head that carries it.
- 2026-10-23 17:59 UTC - Second correction, from the GitHub timeline and reply timestamps; the 17:27 entry above is stamped later than its commit (17:26:45Z) and misstates the round 2 order. Round 2 order: the F5 and F6 fix commits (17:23:02Z, 17:24:12Z) were pushed together at 17:25:57Z, which dismissed da2ce7's approval 5631788587; the F5 reply (17:25:56Z) came one second before that push and the F6 reply (17:25:58Z) after it; the F7 reply (17:26:00Z) came 45 seconds before its own fix commit (17:26:45Z), which was pushed before the 17:28:43Z re-request of da2ce7.
- 2026-10-23 18:02 UTC - Round 3. da2ce7's review 5632201391 (17:40:24Z) raised F8 to F10 about this audit record. Verified each claim against the timeline and the file, committed one fix each (F9, F10, F8 in that order), pushed the three together, replied on each thread and resolved all three (GraphQL showed 0 unresolved afterwards); this audit commit records it. A new ACK is needed on the head that carries it.
- 2026-10-24 08:15 UTC - Round 4. da2ce7's review 5633236549 (2026-10-23T19:07:54Z) raised F11 about this audit record. Verified both claims against the timeline and the file, committed one fix, pushed, replied and resolved the thread (GraphQL showed 0 unresolved afterwards); this audit commit records it. A new ACK is needed on the head that carries it.
- 2026-10-24 12:18 UTC - Correction of the 08:15 entry, from the GitHub timeline and reply timestamps. That entry says the F11 fix was committed, pushed, replied to and resolved in that order; the F11 reply (08:13:52Z, on the head whose subject is `docs(pr-reviews): [#2628] audit review round 3 of the spec PR`) came one second before the push of the fix commit (08:13:53Z, which dismissed review 5633236549). The fix commit itself was made at 08:11:47Z.
- 2026-10-24 12:22 UTC - Round 5. da2ce7's review 5637522963 (2026-10-24T08:28:21Z) raised F12 about this audit record. Verified the claim against the timeline and reply timestamps and committed one fix (12:18:41Z). The push dismissed the review at 12:20:39Z, the reply followed at 12:20:59Z and the thread was then resolved (GraphQL showed 0 unresolved); this audit commit records it. A new ACK is needed on the head that carries it.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
