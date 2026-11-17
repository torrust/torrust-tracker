---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2919 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2919.

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
| F1 | review-finding:pr-2919-f1 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2919-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2919-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2919-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2919-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2919-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2919-f7 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2919-f8 | Copilot | Minor | correctness | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - The sub-issue specs still record themselves as unapproved and without an issue

- PR number: 2919
- Source review ID: 5943374190
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2919#discussion_r4603304294
- Concern: The six sub-issue specs carried `github-issue` and an `Issue #<n>` heading but left "Spec reviewed and approved" and "GitHub issue created" unticked and logged only "Not yet approved"; the #384 spec left "Spec reviewed and approved" unticked; the "drafted in" checkpoint named the `open/` path while the EPIC named `drafts/`.
- Solution: Ticked both checkpoints in the six sub-issue specs and the approval checkpoint in the #384 spec (its issue checkpoint was already ticked), added one Progress Log line per spec for the approval, the issue and the move, and changed the "drafted in" checkpoint to name both folders, as the EPIC does.
- Current-tree verification: `grep -n 'Spec reviewed and approved\|GitHub issue created' docs/issues/open/29*-2912-si-*/ISSUE.md docs/issues/open/384-*/ISSUE.md` shows `[x]` on every line.
- Resolution reference: `docs(issues): [#2912] address review of PR 2919: approval checkpoints, audit status and option wording`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2919#discussion_r4603382559

### F2 - The #2804 audit spec still calls the EPIC a draft with no issue

- PR number: 2919
- Source review ID: 5943374190
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2919#discussion_r4603304297
- Concern: The repaired sentence in the #2804 audit spec still said "EPIC draft", and the paragraph after the table said the EPIC and its sub-issues were drafts with no issue, which was false once #2912 to #2918 existed.
- Solution: The sentence names EPIC #2912, the table says "Sub-issues", and the paragraph says the issues exist, that the closing condition is met and that the issue's own session does the close-out.
- Current-tree verification: `grep -n '#2912' docs/issues/open/2804-location-caller-track-caller-audit/ISSUE.md` finds the heading (after F5, `EPIC #2912`), the sentence (`EPIC #2912`) and the paragraph (`issue #2912`).
- Resolution reference: `docs(issues): [#2912] address review of PR 2919: approval checkpoints, audit status and option wording`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2919#discussion_r4603383063

### F3 - Option 3 reads as both adopted and rejected

- PR number: 2919
- Source review ID: 5943374190
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2919#discussion_r4603304307
- Concern: The EPIC said it implements option 4 together with option 3 and, in the next sentence, that option 4 was chosen over option 3; decision 1 called option 3 rejected.
- Solution: Used the reviewer's wording: option 4 keeps option 3's context-instead-of-location wording and also removes the field; only option 3 alone (keeping the field) was rejected. Decision 1 says "option 3 alone" too.
- Current-tree verification: `grep -n 'option 3' docs/issues/open/2912-remove-source-locations-from-errors/EPIC.md` shows only the two new phrasings.
- Resolution reference: `docs(issues): [#2912] address review of PR 2919: approval checkpoints, audit status and option wording`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2919#discussion_r4603383547

### F4 - No review audit record for the answered and resolved threads

- PR number: 2919
- Source review ID: 5943554323
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2919#discussion_r4603450558
- Concern: The round 1 threads were answered and resolved without the audit record that `process-pr-review` requires, and the replies cite the branch commit id `280121c6a` instead of a commit subject; the three fixes share one commit.
- Solution: Created this record from `docs/templates/PR-REVIEW-TEMPLATE.md`, logged the late start, and mapped the commit id cited in the round 1 replies to its subject. The three round 1 fixes were made in one commit; the log records that deviation from step 6.
- Current-tree verification: This file exists; `validate-audit-record.py --pr-number 2919 --base torrust/develop` run as the log states.
- Resolution reference: `docs(pr-reviews): [#2912] record the review audit of PR 2919`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2919#discussion_r4603525639

### F5 - The audit spec dates #384 to this week

- PR number: 2919
- Source review ID: 5943554323
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2919#discussion_r4603450567
- Concern: "its sub-issues are #2913 to #2918 and #384, created on 2026-11-17" reads as dating #384 (opened in 2023) to this week; the heading still said "Tasks moved to the EPIC draft".
- Solution: The sentence now reads "#2913 to #2918, created on 2026-11-17, and the existing #384", and the heading reads "Tasks moved to EPIC #2912". The acceptance-criteria labels that say "EPIC draft" stay: they record where the work moved at the time, as the reviewer noted.
- Current-tree verification: `grep -n 'existing #384\|### Tasks moved to EPIC #2912' docs/issues/open/2804-location-caller-track-caller-audit/ISSUE.md` finds both lines.
- Resolution reference: `docs(issues): [#2912] date only the new sub-issues in the #2804 audit spec and name the EPIC in its heading`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2919#discussion_r4603526079

### F6 - The F2 verification claims a grep match it does not produce

- PR number: 2919
- Source review ID: 5943851942
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2919#discussion_r4603665151
- Concern: The F2 verification said `grep -n 'EPIC #2912'` finds the sentence, the paragraph and the heading, but it prints only the heading and the sentence; the paragraph says `issue #2912`.
- Solution: The F2 verification now uses `grep -n '#2912'` and names what each of the three matches says.
- Current-tree verification: `grep -n '#2912' docs/issues/open/2804-location-caller-track-caller-audit/ISSUE.md` prints lines 215, 220 and 228 (heading, sentence, paragraph), plus other lines that cite the EPIC.
- Resolution reference: `docs(pr-reviews): [#2912] correct the F2 verification and drop a branch commit id from the review audit of PR 2919`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2919#discussion_r4603731183

### F7 - The Processing Log cites a branch commit id as its own reference

- PR number: 2919
- Source review ID: 5943851942
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2919#discussion_r4603665136
- Concern: The 17:48 Processing Log entry named the head by its branch commit id, which a rebase changes; `open-pull-request` forbids such ids in durable references, except where the record maps the id cited by uneditable replies.
- Solution: The entry now names that commit by its subject, and a correction entry at the end of the log records the change.
- Current-tree verification: the commit-id check of `open-pull-request` prints only `280121c6a`, the id the round 1 replies cite and the Processing Log maps to its subject.
- Resolution reference: `docs(pr-reviews): [#2912] correct the F2 verification and drop a branch commit id from the review audit of PR 2919`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2919#discussion_r4603731569

### F8 - Decision 1 calls every production location wrong

- PR number: 2919
- Source review ID: 5943970292
- Reviewer finding ID: F6 (reassigned: audit F6 is an earlier finding)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2919#pullrequestreview-5943970292
- Concern: Decision 1 of the EPIC said the recorded value is wrong at every production site, but the audit inventory classifies the five `#[track_caller]` `From` conversions into `LocatedError` as correct for their call chain (class C), so the rationale contradicts its evidence. The finding is in the review body (a "previously missed issue"), with no inline thread.
- Solution: Decision 1 now says that each of the 38 production `Location::caller()` sites records a fixed library line, and that the five class C conversions record a correct location but are removed as well, because that location is still an implementation path.
- Current-tree verification: `sed -n 70,76p docs/issues/open/2912-remove-source-locations-from-errors/EPIC.md` shows the new wording; `inventory.md` names the five class C conversions in its `LocatedError` conversions table.
- Resolution reference: `docs(issues): [#2912] separate the wrong direct captures from the correct LocatedError conversions in decision 1`
- Follow-up PR URL: N/A
- Reply URL: N/A

## Processing Log

- 2026-11-17 17:35 UTC - Replied to F1 to F3 (replies created from 17:35:15Z to 17:35:20Z) and resolved their threads after pushing the round 1 fixes, before this record existed: a late start of the audit. The replies cite the branch commit id `280121c6a`, which is the commit `docs(issues): [#2912] address review of PR 2919: approval checkpoints, audit status and option wording`. The three independent fixes share that one commit instead of one commit each (step 6). The F1 reply says the #384 spec now ticks both checkpoints; only the approval checkpoint changed there, because its issue checkpoint was already ticked.
- 2026-11-17 17:44 UTC - Started audit: created this record from `docs/templates/PR-REVIEW-TEMPLATE.md` after da2ce7's round 2 review 5943554323 (F4 asked for it). Normalized round 1 (F1 to F3, review 5943374190) and round 2 (F4 and F5); committed the F5 fix (`docs(issues): [#2912] date only the new sub-issues in the #2804 audit spec and name the EPIC in its heading`) before this record. Copilot review 5943384927 is an overview with no actionable assertion, so it has no row.
- 2026-11-17 17:48 UTC - Pushed the F5 fix and this record (head commit `docs(pr-reviews): [#2912] record the review audit of PR 2919`), then replied to F4 and F5 (replies created at 17:48:01Z and 17:48:04Z) and filled in their reply URLs. `validate-audit-record.py --pr-number 2919 --base torrust/develop`, run from a scratchpad copy whose `gh` calls go through `gh-bot`, reported 5 rows, 0 failures on this worktree record.
- 2026-11-17 18:02 UTC - Round 3: normalized da2ce7 review 5943851942 (APPROVED) with findings F6 and F7. Correction (F7) of the 2026-11-17 17:48 entry: its words naming the head by branch commit id were replaced in place by the commit subject `docs(pr-reviews): [#2912] record the review audit of PR 2919`; the rest of that entry is unchanged. F6 corrected the F2 verification line.
- 2026-11-17 18:06 UTC - Pushed the commit `docs(pr-reviews): [#2912] correct the F2 verification and drop a branch commit id from the review audit of PR 2919`, then replied to F6 and F7 (replies created at 18:06:21Z and 18:06:23Z) and filled in their reply URLs. `validate-audit-record.py --pr-number 2919 --base torrust/develop` (scratchpad copy, `gh` calls through `gh-bot`) on this worktree record reported 7 rows, 0 failures.
- 2026-11-17 18:11 UTC - Round 4: normalized Copilot review 5943970292, whose body reports one previously missed issue (Copilot's own ID F6) as audit finding F8, and committed its fix. It has no inline thread, so its row is `NON_RESOLVABLE` and its reply goes to the PR conversation.

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
