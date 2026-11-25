---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3045 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/3045>.

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

Round 1 covers review 6045674009 (Greptile, COMMENTED) of the first head of the PR (the commit `docs(issues): [#2392] mark order 9 (#2924) done`), with one inline finding, F1. Greptile marks it P2; this audit maps that badge to `Minor (inferred)`. The CodeRabbit comment on the PR is a rate-limit notice, not a review, and has no finding.

Round 2 covers review 6053927729 (da2ce7, CHANGES_REQUESTED) of the head `docs(pr-reviews): [#2392] audit review round 1 of PR 3045`, with the reviewer's findings F1 to F5, as audit findings F2 to F6. F1 is Major and blocking; the others are Minor or Nit.

Round 3 covers review 6056979124 (CodeRabbit, COMMENTED) of the head `docs(issues): [#2392] record order 9 as done in the AC2 evidence`, with one inline finding, F7. CodeRabbit marks it Minor.

Round 4 covers review 6057293370 (da2ce7, APPROVED at the head `docs(pr-reviews): [#2392] say the round 3 thread was already resolved`) with three inline Nits, as audit findings F8 to F10. The review body says none blocks.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3045-f1 | Unknown | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3045-f2 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3045-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3045-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3045-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3045-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3045-f7 | Unknown | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3045-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3045-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-3045-f10 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The recheck count of 6 looked suite-wide

- PR number: 3045
- Source review ID: 6045674009
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4685188502
- Concern: The new progress-log entry said the recheck still counts 6 `data_*.db` files without saying the count comes from running only `torrust-tracker-http-core`, so a reader could take it as the suite-wide baseline for order 10.
- Solution: The finding is valid. The entry now says the `http-core` recheck still counts 6, which matches the scope of V1 and V2 in the archived #2924 spec.
- Current-tree verification: `grep -c 'The .http-core. recheck still counts 6' docs/issues/open/2392-test-isolation/EPIC.md` prints 1.
- Resolution reference: `docs(issues): [#2392] scope the recheck count of order 9 to http-core`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4685257658

### F2 - The description's AC1 sentence carried a closing keyword

- PR number: 3045
- Source review ID: 6053927729
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4692050049
- Concern: The description ended its AC1 sentence with a closing keyword directly before the #2394 reference, so merging would have closed #2394, which row 2 of the EPIC keeps open until the maintainer closes it. The sentence also added a condition the AC1 evidence does not state.
- Solution: The description now says AC1 waits for the maintainer to link #2924, #2925 and #2943 as sub-issues, with no closing keyword. The description is outside the tree, so this fix has no commit.
- Current-tree verification: `gh-bot pr view 3045 --repo torrust/torrust-tracker --json closingIssuesReferences --jq '.closingIssuesReferences|length'` prints 0 (the description is not in the tree; this ran at 11:35 UTC on 2026-11-25).
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4694400456
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4694400456

### F3 - The AC2 evidence still listed order 9 as pending

- PR number: 3045
- Source review ID: 6053927729
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4692050057
- Concern: Row 9 is DONE, but the AC2 row of Acceptance Verification still ended with the condition that orders 6, 9 and 10 be done.
- Solution: The AC2 evidence says orders 7 and 9 are done (PRs #2938 and #3029) and stays TODO until orders 6 and 10 are done. A new progress-log entry says AC2 now waits on orders 6 and 10; the earlier entry keeps its text.
- Current-tree verification: `grep -c 'Stays TODO until orders 6 and 10 are done' docs/issues/open/2392-test-isolation/EPIC.md` prints 1.
- Resolution reference: `docs(issues): [#2392] record order 9 as done in the AC2 evidence`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4694400817

### F4 - The log pointed at a validator result that no commit carries

- PR number: 3045
- Source review ID: 6053927729
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4692050063
- Concern: The second log entry said the validator result is in the commit that adds the entry, and no commit carries one.
- Solution: The entry now quotes the validator's summary line, which it printed on the worktree record before the commit.
- Current-tree verification: `grep -c '^- 2026-11-24 19:18 UTC - Resolved the thread.*validator printed' docs/pr-reviews/pr-3045-review/PR-REVIEW.md` prints 1.
- Resolution reference: `docs(pr-reviews): [#2392] correct the round 1 log of PR 3045`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4694401220

### F5 - The audit start was logged after the first reply, with no late-start note

- PR number: 3045
- Source review ID: 6053927729
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4692050068
- Concern: The first entry says the audit started at 19:18 UTC, while the reply it records was posted at 19:17:49 UTC.
- Solution: A correction entry says the record was created after the first reply, so the first entry is a late start stamped when it was written. The first entry keeps its text.
- Current-tree verification: `grep -c '^- 2026-11-25 [0-9:]* UTC - Correction for round 1.*is a late start' docs/pr-reviews/pr-3045-review/PR-REVIEW.md` prints 1.
- Resolution reference: `docs(pr-reviews): [#2392] correct the round 1 log of PR 3045`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4694401759

### F6 - The newest review in the list was not the one named

- PR number: 3045
- Source review ID: 6053927729
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4692050072
- Concern: The review list holds a later entry, 6045762270, which only wraps this PR author's own reply, so the sentence naming the newest review was not literally true.
- Solution: The same correction entry says the newest review means the newest review by a reviewer, and names review 6045762270 as the one that wraps the author's own reply.
- Current-tree verification: `grep -c '^- 2026-11-25 [0-9:]* UTC - Correction for round 1.*newest review by a reviewer' docs/pr-reviews/pr-3045-review/PR-REVIEW.md` prints 1.
- Resolution reference: `docs(pr-reviews): [#2392] correct the round 1 log of PR 3045`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4694402155

### F7 - The six-file attribution was stated as fact

- PR number: 3045
- Source review ID: 6056979124
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4694371011
- Concern: The progress-log entry said all six `http-core` `data_*.db` files come from `ephemeral*()` users that order 10 moves, but the archived #2924 evidence supports it only by elimination of the one direct caller, and says per-test attribution is not done.
- Solution: The finding is valid. The 2026-11-24 18:57 entry now says the one direct caller creates none, so the six come from `ephemeral*()` users that order 10 moves, and that the per-test attribution is not done. A new entry records the change, and the PR description says the same.
- Current-tree verification: `grep -c 'the per-test attribution is not done' docs/issues/open/2392-test-isolation/EPIC.md` prints 1.
- Resolution reference: `docs(issues): [#2392] say the six-file attribution follows from excluding the direct caller`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4694473643

### F8 - The Round 3 paragraph named the wrong head

- PR number: 3045
- Source review ID: 6057293370
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4694624057
- Concern: The paragraph said review 6056979124 is of the head named by the round 1 correction commit, but GitHub records its commit as the one that records order 9 in the AC2 evidence.
- Solution: The paragraph names the head `docs(issues): [#2392] record order 9 as done in the AC2 evidence`.
- Current-tree verification: `grep -c '^Round 3 covers review 6056979124 .*of the head .docs(issues): \[#2392\] record order 9 as done' docs/pr-reviews/pr-3045-review/PR-REVIEW.md` prints 1.
- Resolution reference: `docs(pr-reviews): [#2392] restore the rewritten log entries of PR 3045 and name the right head`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4694758661

### F9 - The record called its logs append-only while entries were rewritten in place

- PR number: 3045
- Source review ID: 6057293370
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4694624075
- Concern: Two Solutions gave the append-only log as a reason, while the EPIC entry and two Processing Log entries had been rewritten in place after they were pushed.
- Solution: The two reasons are dropped. The two rewritten Processing Log entries are restored to their first text, and an appended correction names the rewritten content and the EPIC entry that the F7 fix also rewrote.
- Current-tree verification: `grep -c 'because the log is append[-]only' docs/pr-reviews/pr-3045-review/PR-REVIEW.md` prints 0, and `grep -c '^- 2026-11-25 [0-9:]* UTC - Correction after da2ce7 review 6057293370' docs/pr-reviews/pr-3045-review/PR-REVIEW.md` prints 1.
- Resolution reference: `docs(pr-reviews): [#2392] restore the rewritten log entries of PR 3045 and name the right head`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4694759256

### F10 - The description's second bullet counted one entry and misplaced the elimination

- PR number: 3045
- Source review ID: 6057293370
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4694624096
- Concern: The bullet said the PR adds a progress-log entry, while it adds three, and its clause read as if order 10 moves the users by elimination.
- Solution: The description says it adds three progress-log entries and puts the elimination on the clause about where the files come from. The description is outside the tree, so this fix has no commit.
- Current-tree verification: `gh-bot pr view 3045 --repo torrust/torrust-tracker --json body --jq .body | grep -c 'Adds three progress-log entries'` prints 1 (the description is not in the tree; this ran at 12:18 UTC on 2026-11-25).
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4694759726
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3045#discussion_r4694759726

## Processing Log

- 2026-11-24 19:18 UTC - Started audit. Round 1: review 6045674009 (Greptile, COMMENTED) with one finding, F1, valid. Committed and pushed the fix (`docs(issues): [#2392] scope the recheck count of order 9 to http-core`), then replied on the thread. The CodeRabbit comment is a rate-limit notice and needs no action.
- 2026-11-24 19:18 UTC - Resolved the thread. A GraphQL refresh shows no unresolved thread, and the newest review in the list is 6045674009. The audit validator result is in the commit that adds this entry.
- 2026-11-25 11:40 UTC - Correction for round 1, after da2ce7 review 6053927729 (F4 and F5 of that review). The record was created after the first reply (posted at 19:17:49Z), so the first entry above is a late start, stamped when the entry was written. In the second entry, "the newest review in the list" means the newest review by a reviewer: the list also holds review 6045762270, which only wraps this PR author's own reply.
- 2026-11-25 11:53 UTC - Round 2: da2ce7 review 6053927729 (CHANGES_REQUESTED, at the first audit head) with findings F2 to F6 of this record, all valid. Edited the PR description for F2 (11:35Z), then committed and pushed the fixes (`docs(issues): [#2392] record order 9 as done in the AC2 evidence` and `docs(pr-reviews): [#2392] correct the round 1 log of PR 3045`) and replied to the five threads (about 11:46Z). The `ACK` comments of the maintainer at 19:38Z and 11:32Z name the first audit head.
- 2026-11-25 11:53 UTC - Round 3: CodeRabbit review 6056979124 (COMMENTED) with one finding, F7, valid. Committed and pushed the fix (`docs(issues): [#2392] say the six-file attribution follows from excluding the direct caller`, then `docs(issues): [#2392] log the attribution change as its own entry`) and replied (about 11:53Z). Resolved the five round 2 threads at about 11:47Z.
- 2026-11-25 11:53 UTC - The validator on the worktree record printed `{"status": "ok", "rows": 7, "log_entries": 5, "failures": 0}`. Resolved the round 3 thread (about 11:53Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread, and the newest review by a reviewer in the review list was 6056979124. Committed these entries.
- 2026-11-25 12:12 UTC - Correction after da2ce7 review 6057293370, which found that two entries above were rewritten in place after they were pushed. Both are restored to their first text. The first one, the second entry of 2026-11-24 19:18, was rewritten by `docs(pr-reviews): [#2392] correct the round 1 log of PR 3045`: its last sentence said the validator result is in the commit that adds the entry, and it now has no such result; the validator printed `{"status": "ok", "rows": 1, "log_entries": 2, "failures": 0}` on the worktree record before that commit. The second one, the validator entry of 2026-11-25 11:53, was rewritten by `docs(pr-reviews): [#2392] say the round 3 thread was already resolved`: the round 3 thread was already resolved when I refreshed, so I did not resolve it. Also, the EPIC entry of 2026-11-24 18:57 was rewritten in place by `docs(issues): [#2392] say the six-file attribution follows from excluding the direct caller`, as the F7 solution says.
- 2026-11-25 12:18 UTC - Round 4: da2ce7 review 6057293370 (APPROVED) with three Nits, F8 to F10, all valid. Edited the PR description for F10, committed and pushed the fix for F8 and F9 (`docs(pr-reviews): [#2392] restore the rewritten log entries of PR 3045 and name the right head`), and replied to the three threads (about 12:17Z). The validator on the worktree record printed `{"status": "ok", "rows": 10, "log_entries": 7, "failures": 0}`. Resolved the three threads, selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread, and the newest review by a reviewer in the review list was 6057293370. Committed these entries.

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
