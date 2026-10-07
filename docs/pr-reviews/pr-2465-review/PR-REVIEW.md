---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - .github/skills/dev/planning/cleanup-completed-issues/SKILL.md
    - docs/issues/closed/2448-1488-si-17-migrate-standalone-udp-environment/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2465 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2465>.

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

Copilot review 5439917780 ("Balanced" effort) left one inline finding, `[Minor][F1]`; its badge
markup rates it `Low severity`. da2ce7 review 5440064083 (round 1, CHANGES_REQUESTED) left two
inline findings numbered F1-F2 in the reviewer's series. F1 already belongs to Copilot's finding,
so they take the next free audit IDs, F2-F3, with their original IDs kept in the detail entries.
da2ce7's F2 raises the same stale date as Copilot's F1, which was fixed first.

da2ce7 review 5440236519 (round 2, CHANGES_REQUESTED, at the F1 fix head) confirmed F1 and
re-raised its round-1 F1 inline before the F2 fix was pushed; that re-raise is F4. da2ce7 review
5440392814 (round 3, approval at the F2 fix head, later dismissed by the push of this record)
found nothing new.

da2ce7 review 5440707809 (round 4, CHANGES_REQUESTED, at the first audit-update head) left three
inline findings on this record, numbered F4-F6 in the reviewer's series. F4 is taken, so they
become F5-F7, with their original IDs kept in the detail entries.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2465-f1` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2465-f2` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2465-f3` | Human | Suggestion | documentation | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |
| F4 | `review-finding:pr-2465-f4` | Human | Minor | correctness | RE_RAISE_OF:F2 | NO_ACTION | SUPERSEDED |
| F5 | `review-finding:pr-2465-f5` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2465-f6` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2465-f7` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - EPIC findings lead-in dates every status to 2026-10-02

- PR number: 2465
- Source review ID: 5439917780
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2465#discussion_r4204943246>
- Concern: the reworded findings 2 and 8 of EPIC #1488 describe the tree on 2026-10-07, but the
  list's lead-in still says each status is current as of 2026-10-02.
- Solution: change the lead-in date to 2026-10-07 and refresh the EPIC's `last-updated-utc`.
- Current-tree verification: findings 1-9 re-read against `develop` at `7836471b3`; the only
  merge since #2459 is #2461 (unrelated), and #2410, #2413-#2416, #2449, and #2450 are open, so
  every status still holds on that date.
- Resolution reference: `docs(issues): refresh EPIC #1488 status snapshot date`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2465#discussion_r4205102654>

### F2 - The archive log entry misstates #2459's review history

- PR number: 2465
- Source review ID: 5440064083
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2465#discussion_r4205054961>
- Concern: the closing progress-log entry said #2459 merged "after da2ce7 approved review rounds
  2 to 4", but the #2459 audit records an approval dismissed by a push (round 2), a change
  request (round 3), and an approval (round 4); the merge rests on da2ce7's later ACK of the
  final head `888b83b41`.
- Solution: use the reviewer's wording: #2459 merged after da2ce7 approved its final head, with
  rounds 1 to 4 audited in the #2459 record.
- Current-tree verification: `docs/pr-reviews/pr-2459-review/PR-REVIEW.md` lines 57-61 and the
  `604b26c6c` merge message's ACK match the new entry.
- Resolution reference: `docs(issues): [#2448] correct the archive entry's #2459 review history`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2465#discussion_r4205276416>

### F3 - The EPIC findings lead-in date (re-raise)

- PR number: 2465
- Source review ID: 5440064083
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2465#discussion_r4205054974>
- Concern: the same stale "as of 2026-10-02" lead-in as F1.
- Solution: no change of its own; the F1 fix covers it.
- Current-tree verification: same as F1.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2465#discussion_r4205276688>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2465#discussion_r4205276688>

### F4 - The archive log entry's review history (re-raise)

- PR number: 2465
- Source review ID: 5440236519
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2465#discussion_r4205192350>
- Concern: the same misstated #2459 review history as F2, re-raised at the F1 fix head, where
  the line was still unchanged.
- Solution: no change of its own; the F2 fix, pushed after this re-raise, covers it.
- Current-tree verification: same as F2; da2ce7 round 3 confirmed the new line at the bytes.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2465#discussion_r4205454707>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2465#discussion_r4205454707>

### F5 - Status Values and Completion Rules are not the template's verbatim text

- PR number: 2465
- Source review ID: 5440707809
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2465#discussion_r4205575732>
- Concern: the template marks both blocks "Copied verbatim", but this record dropped the `OPEN`
  sentence and the duplicate-rule bullet from Status Values, and the outdated/duplicate and
  consolidated-response bullets from Completion Rules.
- Solution: copy both blocks from `docs/templates/PR-REVIEW-TEMPLATE.md` byte for byte; they had
  been copied from the older #2443 record.
- Current-tree verification: a script extracting both template blocks finds each as a substring
  of this record.
- Resolution reference: `docs(pr-reviews): copy the template's Status Values and Completion Rules verbatim into the #2465 record`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2465#discussion_r4205671671>

### F6 - Rows F3 and F4 break the duplicate rule

- PR number: 2465
- Source review ID: 5440707809
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2465#discussion_r4205575747>
- Concern: F3 and F4 are duplicates, which take `Disposition=NO_ACTION`,
  `Thread state=SUPERSEDED`, and the reply URL as the resolution reference; both cited a commit
  subject and were recorded as `SUPERSEDED`/`RESOLVED`.
- Solution: change the three cells per row. The validator had rejected a reply URL while the
  disposition was `SUPERSEDED`; the commit subject was the wrong fix for that failure.
- Current-tree verification: rows F3 and F4 read `NO_ACTION | SUPERSEDED`, their detail entries
  cite their reply URLs, and the validator passes.
- Resolution reference: `docs(pr-reviews): record the #2465 duplicates F3 and F4 as NO_ACTION/SUPERSEDED`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2465#discussion_r4205671938>

### F7 - F1's reviewer finding ID should be N/A

- PR number: 2465
- Source review ID: 5440707809
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2465#discussion_r4205575763>
- Concern: Copilot's F1 kept its ID, so the skill records `N/A`, not the original ID.
- Solution: set F1's `Reviewer finding ID` to `N/A`.
- Current-tree verification: the F1 entry reads `Reviewer finding ID: N/A`; F2-F7 keep the
  reviewer's original IDs.
- Resolution reference: `docs(pr-reviews): record N/A as the #2465 F1 reviewer finding ID`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2465#discussion_r4205672181>

## Processing Log

- 2026-10-07 09:11 UTC - Fetched Copilot review 5439917780; committed the F1 fix (09:00); the
  branch was 24 commits behind `develop`, so rebased onto `7836471b3` and force-pushed with
  lease; replied on the F1 thread (09:11:20). The reply says the other findings still hold; that
  check was completed just after posting it, and it confirmed the claim.
- 2026-10-07 09:30 UTC - Fetched da2ce7 review 5440064083 (09:06:02, CHANGES_REQUESTED);
  committed the F2 fix (09:12), pushed, and replied on both threads (09:30:26 and 09:30:27).
  Recorded this audit; threads are resolved only after it is pushed.
- 2026-10-07 09:50 UTC - The pushed record missed two da2ce7 reviews found by the post-push
  reply guard: round 2 (09:21:16), whose re-raise is F4, and round 3 (09:34:49, approval, no
  findings, dismissed by that push). da2ce7 had already resolved the F2-F4 threads. Replied on the
  F4 thread (09:50:19) and added F4; the Copilot F1 thread is resolved after this update is
  pushed.
- 2026-10-07 10:14 UTC - Rebased onto `develop` at `30e6b7127` (9 commits behind, no overlap).
  da2ce7 round 4 (5440707809, 10:04:07) arrived after a 09:59 check had reported nothing new.
  Committed the F5, F6, and F7 fixes separately, pushed, and replied on the three threads
  (10:14:18-10:14:22). F3 and F4 rows and the F1 reviewer ID were edited in place by those fixes,
  as the findings required.

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
