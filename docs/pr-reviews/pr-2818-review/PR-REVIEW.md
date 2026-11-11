---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2818 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2818.

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

Round 1: da2ce7's review `5856655819` (F1 to F3), processed after the replies were posted. The record is therefore late, which is the pattern this pull request describes; the replies were posted first because I read the validator's "before replying" line as requiring reply URLs that do not exist yet. The line now says to run the validator once the reply URLs are recorded. Round 2: Copilot's review `5856716279` (F4, F5), on the first commit, processed with the record already present. Round 3: da2ce7's review `5857046891`, whose reviewer IDs F4 to F7 collide with existing rows and are recorded as F6 to F9. Round 4: da2ce7's review `5857359407`, whose IDs F9 to F11 are recorded as F10 to F12. Round 5: da2ce7's review `5857689660`; his F12 is recorded as F13 because F12 exists, and his F15 to F17 keep their IDs. Round 6: da2ce7's review `5857985681`; his F12 is recorded as F18 because F12 exists, and his F19 to F21 keep their IDs.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2818-f1 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2818-f2 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2818-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2818-f4 | Copilot | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2818-f5 | Copilot | Suggestion (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2818-f6 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2818-f7 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2818-f8 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2818-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2818-f10 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2818-f11 | Human | Nit | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F12 | review-finding:pr-2818-f12 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2818-f13 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2818-f15 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2818-f16 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2818-f17 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2818-f18 | Human | Minor | correctness | RE_RAISE_OF:F13 | FIXED | RESOLVED |
| F19 | review-finding:pr-2818-f19 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2818-f20 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2818-f21 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - A late start is forbidden rather than handled

- PR number: 2818
- Source review ID: 5856655819
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536225666
- Concern: The preface said "Do not start it after a later round", which tells an agent that takes over after round 1, or one told the record is missing, not to create the record at all.
- Solution: Replaced the sentence with the remedy: if a reply was already posted without the record, create it now from the template and log the late start in the Processing Log.
- Current-tree verification: Read the new preface in process-pr-review/SKILL.md.
- Resolution reference: docs(skills): scope the audit record start to events that need action
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536269270

### F2 - A clean first event now creates a record whose push the loop does not settle

- PR number: 2818
- Source review ID: 5856655819
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536225718
- Concern: The record was created on the first review or comment whatever it carried, so a clean review, informational comment or ACK left an uncommitted record or an extra push that moves the head.
- Solution: Scoped the trigger in `process-pr-review` to the first review or comment that needs a reply or a fix; `wait-for-pr-review` points there. An earlier wording of the clean-event sentence in `wait-for-pr-review` was removed by a later commit.
- Current-tree verification: Read the process-pr-review preface ("that needs a reply or a fix").
- Resolution reference: docs(skills): scope the audit record start to events that need action
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536269640

### F3 - The description patterns cover four of the six PRs

- PR number: 2818
- Source review ID: 5856655819
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536225734
- Concern: The description said all six started the record after round 2 or 3, after a reviewer cited the skill, or from an earlier audit, which the audits and retrospectives of 2809 and 2810 do not show.
- Solution: Edited the PR description to say only what holds for all six: each audit record was first committed after the first replies.
- Current-tree verification: gh api repos/torrust/torrust-tracker/pulls/2818 shows the edited body.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536270103
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536270103

### F4 - The trigger terms review and comment are ambiguous and unformatted

- PR number: 2818
- Source review ID: 5856716279
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536274300
- Concern: The new preface used unformatted "review or comment", which could mean several kinds of comment.
- Solution: Used backticks and named the kinds that count: a submitted review, or a PR, review or inline comment.
- Current-tree verification: Read the preface in process-pr-review/SKILL.md.
- Resolution reference: docs(skills): point wait-for-pr-review to the audit start rule
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536321407

### F5 - The rule is stated in full in two skills

- PR number: 2818
- Source review ID: 5856716279
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536274496
- Concern: The same rule was spelled out in wait-for-pr-review and process-pr-review, which can drift.
- Solution: Shortened wait-for-pr-review to a pointer to the rule in process-pr-review.
- Current-tree verification: Read step 2 of the review loop in wait-for-pr-review/SKILL.md.
- Resolution reference: docs(skills): point wait-for-pr-review to the audit start rule
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536321795

### F6 - The retrospective says the new start rule cannot end validated

- PR number: 2818
- Source review ID: 5857046891
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536535928
- Concern: Root cause 3, the Findings introduction and the 09:41 entry said the rule cannot end validated, but the validator only fails rows whose replies do not exist; the conflict is the "before replying" line of the validation step.
- Solution: Said in the start rule that reply URLs are filled in after replying, scoped the validator step to recorded reply URLs, and corrected the introduction and retrospective.
- Current-tree verification: Read the preface and Validation Script paragraph in process-pr-review/SKILL.md.
- Resolution reference: docs(skills): fill in reply URLs after replying in the audit start rule
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536568853

### F7 - The F2 Solution cites a sentence that a later commit removed

- PR number: 2818
- Source review ID: 5857046891
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536535943
- Concern: The F2 Solution and verification described a sentence in wait-for-pr-review that a later commit removed.
- Solution: Restated the F2 Solution and verification as the trigger scope in process-pr-review.
- Current-tree verification: Read the F2 detail entry and the process-pr-review preface.
- Resolution reference: docs(skills): fill in reply URLs after replying in the audit start rule
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536569250

### F8 - The 09:45 entry stamps the second review arrival late

- PR number: 2818
- Source review ID: 5857046891
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536535955
- Concern: The arrival of review 5856716279 was stamped 09:45, the time of the replies.
- Solution: The arrival of review 5856716279 at 09:40 is stated by the correction entry at 10:38 in the Processing Log, appended in the commit `docs(pr-reviews): record round 4 for PR 2818`. An arrival entry inserted earlier in the commit `docs(skills): fill in reply URLs after replying in the audit start rule` was removed in the commit `docs(pr-reviews): restore the PR 2818 log and update the retrospective narrative` because it rewrote the log.
- Current-tree verification: Read the Processing Log.
- Resolution reference: docs(pr-reviews): record round 4 for PR 2818
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536569598

### F9 - The description omits the two new files

- PR number: 2818
- Source review ID: 5857046891
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536535973
- Concern: The description did not name the audit record and retrospective files the branch adds.
- Solution: Edited the PR description to name them.
- Current-tree verification: gh pr view 2818 shows the edited body.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536569994
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536569994

### F10 - The F6 fix rewrites the Processing Log in place

- PR number: 2818
- Source review ID: 5857359407
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536790786
- Concern: The previous commit inserted a log entry between existing ones and reworded the 09:45 entry, which the skill treats as a rewrite to undo.
- Solution: Restored the 09:45 entry and removed the inserted 09:40 entry in the commit `docs(pr-reviews): restore the PR 2818 log and update the retrospective narrative`; the correction entry was appended in the commit `docs(pr-reviews): record round 4 for PR 2818`.
- Current-tree verification: Read the Processing Log.
- Resolution reference: docs(pr-reviews): restore the PR 2818 log and update the retrospective narrative
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536843499

### F11 - One commit mixes the skill fix with record edits

- PR number: 2818
- Source review ID: 5857359407
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536790803
- Concern: The commit `docs(skills): fill in reply URLs after replying in the audit start rule` changes a skill and the record files and its body omits some record edits; the reviewer suggests splitting it before merge.
- Solution: No change: splitting a pushed, reviewed commit needs a force-push that moves every reviewed commit and invalidates the earlier ACK. Later commits keep skill and record edits apart.
- Current-tree verification: git log shows the commits `docs(pr-reviews): restore the PR 2818 log and update the retrospective narrative` and `docs(pr-reviews): record round 4 for PR 2818` touch only docs/pr-reviews.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536843915
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536843915

### F12 - The retrospective's round-3 update changes counts but not narrative

- PR number: 2818
- Source review ID: 5857359407
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536790814
- Concern: The duration said a few minutes for 36, the line count was stale, and round 3 was not described.
- Solution: Gave the exact duration, the current skill diff size and a paragraph on round 3.
- Current-tree verification: Read the Review Summary and What Made the Review Costly sections.
- Resolution reference: docs(pr-reviews): restore the PR 2818 log and update the retrospective narrative
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4536844415

### F13 - Row F8's Solution still describes the removed 09:40 entry

- PR number: 2818
- Source review ID: 5857689660
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4537053538
- Concern: The F8 Solution described an arrival entry that was removed, and the F10 Solution named a correction that landed in another commit.
- Solution: Restated both Solutions as the current tree and named the commits by subject.
- Current-tree verification: Read the F8 and F10 detail entries.
- Resolution reference: docs(pr-reviews): cite subjects and state the current tree in the PR 2818 record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4537106365

### F15 - The record cites two branch commit ids

- PR number: 2818
- Source review ID: 5857689660
- Reviewer finding ID: F15
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4537053552
- Concern: The record cited two branch commit ids, which a rebase changes.
- Solution: Cited the commits by subject, in the log correction and the F11 entry.
- Current-tree verification: The commit-id check prints nothing for the record.
- Resolution reference: docs(pr-reviews): cite subjects and state the current tree in the PR 2818 record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4537106768

### F16 - Row F11 pairs NO_ACTION with RESOLVED

- PR number: 2818
- Source review ID: 5857689660
- Reviewer finding ID: F16
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4537053559
- Concern: Row F11 recorded NO_ACTION with RESOLVED, and its reply was not in the prescribed form.
- Solution: Recorded F11 as SUPERSEDED; the reply wording gap is in the skill and is left unchanged.
- Current-tree verification: Read the F11 row.
- Resolution reference: docs(pr-reviews): cite subjects and state the current tree in the PR 2818 record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4537107196

### F17 - The round-4 update times resolutions that had not happened

- PR number: 2818
- Source review ID: 5857689660
- Reviewer finding ID: F17
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4537053564
- Concern: The retrospective stamped a resolution time before the threads were resolved and its narrative stopped at round 3.
- Solution: Reported the resolution time as not captured and added a paragraph on rounds 4 and 5.
- Current-tree verification: Read the Review Summary and What Made the Review Costly sections.
- Resolution reference: docs(pr-reviews): cite subjects and state the current tree in the PR 2818 record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4537107463

### F18 - Row F8's Resolution reference names a change that was reverted

- PR number: 2818
- Source review ID: 5857985681
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4537286151
- Concern: The F8 Resolution reference named a commit whose F8 change was removed; the correction landed in another commit.
- Solution: Pointed the Resolution reference at the round 4 commit and named the three commits in the F8 Solution.
- Current-tree verification: Read the F8 detail entry.
- Resolution reference: docs(pr-reviews): align the PR 2818 record references and counts
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4537330005

### F19 - The count of findings about the record omits F16

- PR number: 2818
- Source review ID: 5857985681
- Reviewer finding ID: F19
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4537286157
- Concern: The retrospective counted 8 findings about the record and left out F16.
- Solution: Counted F16 (9 findings) and added it to the round 5 sentence.
- Current-tree verification: Read the Review Summary and What Made the Review Costly sections.
- Resolution reference: docs(pr-reviews): align the PR 2818 record references and counts
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4537330359

### F20 - Row F16's Concern states the rule as if it held

- PR number: 2818
- Source review ID: 5857985681
- Reviewer finding ID: F20
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4537286166
- Concern: The F16 Concern described the rule as holding for row F11, which was the opposite of the finding.
- Solution: Restated the Concern as the finding.
- Current-tree verification: Read the F16 detail entry.
- Resolution reference: docs(pr-reviews): align the PR 2818 record references and counts
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4537330628

### F21 - Two 11:05 entries stamp acts committed at 11:06:37

- PR number: 2818
- Source review ID: 5857985681
- Reviewer finding ID: F21
- Source URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4537286174
- Concern: Two entries stamped 11:05 describe acts committed at 11:06:37.
- Solution: Appended a correction entry instead of editing the two entries.
- Current-tree verification: Read the Processing Log.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4537330943
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2818#discussion_r4537330943

## Processing Log

- 2026-11-11 09:35 UTC - Review 5856655819 arrived with three findings.
- 2026-11-11 09:40 UTC - Replied to the three threads after pushing the fix and editing the description.
- 2026-11-11 09:41 UTC - Started audit late, after the replies, because the validator needs the reply URLs.
- 2026-11-11 09:45 UTC - Copilot review 5856716279 arrived with two findings; fixed, replied, and added F4 and F5.
- 2026-11-11 10:07 UTC - Review 5857046891 arrived with four findings.
- 2026-11-11 10:10 UTC - Fixed, replied, and added F6 to F9.
- 2026-11-11 10:33 UTC - Review 5857359407 arrived with three findings.
- 2026-11-11 10:38 UTC - Fixed, replied, and added F10 to F12.
- 2026-11-11 10:38 UTC - Correction: the commit `docs(skills): fill in reply URLs after replying in the audit start rule` rewrote this log in place (it inserted a 09:40 entry and reworded the 09:45 entry); the entries were restored, review 5856716279 arrived at 09:40 and was processed at 09:45.
- 2026-11-11 10:59 UTC - Review 5857689660 arrived with four findings.
- 2026-11-11 11:05 UTC - Fixed, replied, and added F13 and F15 to F17.
- 2026-11-11 11:05 UTC - Correction: the commit-id replacement in the 10:38 correction entry, made in the commit `docs(pr-reviews): cite subjects and state the current tree in the PR 2818 record`, edited that entry in place; its content is otherwise unchanged.
- 2026-11-11 11:25 UTC - Review 5857985681 arrived with four findings.
- 2026-11-11 11:30 UTC - Fixed, replied, and added F18 to F21.
- 2026-11-11 11:30 UTC - Correction: the two 11:05 entries above ("Fixed, replied, and added F13 and F15 to F17" and the correction) were made in the commit `docs(pr-reviews): record round 5 for PR 2818`, committed at 11:06; the fix was at 11:02 and the replies at 11:05.

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
