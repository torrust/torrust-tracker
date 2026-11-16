---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

# PR #2884 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2884.

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

Round 1 is da2ce7's review `5922865820` (approved) at the head whose only commit is
`chore(issues): archive closed issue #2850 and #2838 specs to docs/issues/closed`. Its four inline findings keep the reviewer's IDs F1 to F4; the review body only
summarizes the verification.

Round 2 is Copilot's review `5922943922` (commented) at the same head. Its one inline finding has
no reviewer ID, so it gets F5.

Round 3 is da2ce7's review `5923302003` (changes requested) at the rewritten head whose only
commit is the reworded archive commit. Its four inline findings F5 to F8 collide with existing
audit IDs, so they get F6 to F9 in source order, with the reviewer's ID kept.

The branch history was rewritten after round 2 (see F1 and F7), so the round 1 and round 2 review
bodies cite a commit id that no longer exists on the branch. That id named the first version of the
commit with the subject `chore(issues): archive closed issue #2850 and #2838 specs to docs/issues/closed`. The id cited by the round 3 and round 4 review bodies names its
reworded, squashed and rebased version, which has the same subject.

Round 4 is Copilot's review `5923416979` (commented) at the rewritten head. Its one inline finding
has no reviewer ID, so it gets F10.

Round 5 is da2ce7's review `5923522198` (now `DISMISSED`) at the head whose last commit is
`docs(pr-reviews): [#2850] audit review round 3 of PR #2884`. Its three inline findings F9 to F11
collide with existing audit IDs, so they get F11 to F13 in source order, with the reviewer's ID kept.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| --- | --- | --- | --- | --- | --- | --- | --- |
| F1 | review-finding:pr-2884-f1 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2884-f2 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2884-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2884-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2884-f5 | Copilot | Suggestion (inferred) | link-integrity | ORIGINAL | NO_ACTION | SUPERSEDED |
| F6 | review-finding:pr-2884-f6 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2884-f7 | Human | Minor | other | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2884-f8 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2884-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2884-f10 | Copilot | Nit (inferred) | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2884-f11 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2884-f12 | Human | Minor | other | RE_RAISE_OF:F10 | NO_ACTION | SUPERSEDED |
| F13 | review-finding:pr-2884-f13 | Human | Nit | metadata | ORIGINAL | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - The commit body's `closed #2850` matches the closing-keyword pattern

- PR number: 2884
- Source review ID: 5922865820
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4587934600
- Concern: the body of the archive commit said "PR #2872 merged and closed #2850 and #2838"; `closed #2850` matches the closing-keyword scan of `open-pull-request`, which wants every match to be an issue this PR closes, and a commit message cannot be fixed after the merge.
- Solution: with the maintainer's approval, the commit was reworded ("PR #2872 merged; #2850 and #2838 are closed."), squashed with the fixes of F2 to F4 and force-pushed by `git-bot-push --force-with-lease`.
- Current-tree verification: `git log -1 --format=%B` has no `close`, `fix` or `resolve` keyword followed by an issue number.
- Resolution reference: chore(issues): archive closed issue #2850 and #2838 specs to docs/issues/closed
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588094671

### F2 - The 14:10 UTC stamps are later than the commit that carries them

- PR number: 2884
- Source review ID: 5922865820
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4587934611
- Concern: the archive entries of the two specs and the EPIC, and six `last-updated-utc` values, read 14:10 UTC, after the commit's time of 14:07:38 UTC.
- Solution: the stamps of the archive work are 14:07 UTC; the two files changed again for F3 and F4 carry the time of that change.
- Current-tree verification: `grep -rn` of the 14:10 stamp over the six documents that carried it prints nothing.
- Resolution reference: chore(issues): archive closed issue #2850 and #2838 specs to docs/issues/closed
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588095135

### F3 - The run script now names paths that did not exist when it ran

- PR number: 2884
- Source review ID: 5922865820
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4587934623
- Concern: the archive had repointed the `probe` and `cfg` paths of the recorded run script of #2850 to `closed/`, so the script no longer matched `evidence/manual-run-output.txt`, which it produced.
- Solution: the two lines are back to the `open/` paths that existed when the script ran; the script is a record, not a link.
- Current-tree verification: lines 51 and 52 of `manual-verification-evidence.md` name `docs/issues/open/`, as the run output does.
- Resolution reference: chore(issues): archive closed issue #2850 and #2838 specs to docs/issues/closed
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588095584

### F4 - This ticked criterion and the checkpoints above disagree

- PR number: 2884
- Source review ID: 5922865820
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4587934633
- Concern: the #2838 spec ticked its criteria as met by #2850 while four implementation checkpoints stayed unticked as superseded.
- Solution: the four checkpoints are ticked, each naming #2850 and PR #2872, and the manual-verification checkpoint and criterion name section V1 of the #2850 evidence as the recheck of the fix.
- Current-tree verification: the #2838 checkpoints and criteria are all ticked and name where each was met.
- Resolution reference: chore(issues): archive closed issue #2850 and #2838 specs to docs/issues/closed
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588096023

### F5 - A progress-log entry of #2838 names the old `open/` path of #2850

- PR number: 2884
- Source review ID: 5922943922
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4587995593
- Concern: the #2838 progress-log entry of 2026-11-12 17:41 UTC names
  `docs/issues/open/2850-non-negative-number-of-bytes/ISSUE.md`, which this PR archives; Copilot
  suggests pointing it to `closed/`.
- Solution: no change; the first reply (https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588096540) was not in the prescribed form, so a second reply gives it (see F8). The entry records where the spec was at that time, and step 4 of
  `cleanup-completed-issues` keeps such historical records as they are, as F3 asked for the run
  script. The live references to #2850 in the same spec (the `related-artifacts` frontmatter and
  the note at the top, which links `../2850-non-negative-number-of-bytes/ISSUE.md`) point to the
  archived spec.
- Current-tree verification: the frontmatter of the #2838 spec lists
  `docs/issues/closed/2850-non-negative-number-of-bytes/ISSUE.md`, and its top note's relative link
  resolves inside `docs/issues/closed/`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588303994
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588303994

### F6 - The description still claims the run-script change that F3 reverted

- PR number: 2884
- Source review ID: 5923302003
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588270939
- Concern: the description said the commands of the #2850 manual-verification evidence were repointed, which F3 reverted; it did not say that the history was rewritten, nor that the PR adds this audit and ticks four #2838 checkpoints. The merge tool copies the description into the merge commit.
- Solution: the description was edited: the run script is listed with the historical records that keep their paths, the audit and the #2838 checkpoints are listed, and a "History rewritten" section says what the rewrite did.
- Current-tree verification: the description no longer claims the run-script change and has the "History rewritten" section.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588304632
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588304632

### F7 - The round-1 fixes and this audit were folded into the archive commit

- PR number: 2884
- Source review ID: 5923302003
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588270958
- Concern: the fixes of F2 to F4 and this audit were squashed into the archive commit, while the skills keep each fix in its own commit and the audit apart; the amend kept the 14:07:38 author date, older than the 15:48 to 16:00 stamps it carries.
- Solution: the maintainer's approval covered the squash; the correction entry of 16:17 UTC below quotes the question and the answer, and the PR description states the rewrite. The commit's committer date is 16:01:42 UTC, after its stamps; its author date is that of the original commit. Round 3 and later are separate commits.
- Current-tree verification: `git log -1 --format='%ad %cd'` on the archive commit shows the two dates.
- Resolution reference: docs(pr-reviews): [#2850] audit review round 3 of PR #2884
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588305190

### F8 - A no-change thread is recorded as `RESOLVED`, and its reply is not in the prescribed form

- PR number: 2884
- Source review ID: 5923302003
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588270976
- Concern: row F5 declined the suggestion but recorded `RESOLVED`, and its reply opened "No change:" instead of `Superseded by <FindingId>: <reason>.`
- Solution: row F5 now records `Thread state=SUPERSEDED`, and a reply in the prescribed form, citing F3, was added to its thread.
- Current-tree verification: row F5 reads `NO_ACTION | SUPERSEDED`; the F5 thread has a reply that starts `Superseded by F3:`.
- Resolution reference: docs(pr-reviews): [#2850] audit review round 3 of PR #2884
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588305709

### F9 - This verification's grep finds this line

- PR number: 2884
- Source review ID: 5923302003
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588270983
- Concern: the F2 verification said a grep for the 14:10 stamp finds none of the changed files, but the sentence itself contained the string.
- Solution: the verification is scoped to the six documents that carried the stamp, and no longer quotes it.
- Current-tree verification: the F2 verification no longer contains the stamp, and a grep for it over the six documents prints nothing.
- Resolution reference: docs(pr-reviews): [#2850] audit review round 3 of PR #2884
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588306286

### F10 - The link text names a folder but the link targets `ISSUE.md`

- PR number: 2884
- Source review ID: 5923416979
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588361472
- Concern: in the carry-over table of the #2850 spec, the visible text of the link to the #2838
  spec ended in `/`, as if it pointed to a folder, while the link targets its `ISSUE.md`.
- Solution: the visible text now ends in `ISSUE.md`, matching the target.
- Current-tree verification: line 487 of the #2850 spec shows the same path as text and target.
- Resolution reference: docs(issues): [#2850] name ISSUE.md in the link text to the #2838 spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588409983

### F11 - The audit cites branch heads by commit id

- PR number: 2884
- Source review ID: 5923522198
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588443247
- Concern: the mapping of the review bodies' commit ids named the round-1 head and the rewritten head by id, so the commit-id check of `open-pull-request` printed both.
- Solution: the mapping now names the commits by subject only, in the wording of the PR #2630 audit, and the round 4 paragraph says "the rewritten head".
- Current-tree verification: the commit-id check of `open-pull-request` prints nothing for this record.
- Resolution reference: docs(pr-reviews): [#2850] audit review round 5 of PR #2884
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588465977

### F12 - The bot thread on the link line had no reply or audit row

- PR number: 2884
- Source review ID: 5923522198
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588443261
- Concern: Copilot's thread of review `5923416979` had no reply and no row when the round 3 audit was pushed.
- Solution: no further change: the thread is F10 of this record, fixed in its own commit, replied to and resolved before this review arrived. Reply: `Superseded by F10: ...`.
- Current-tree verification: row F10 and its thread are `FIXED`/`RESOLVED`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588466618
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588466618

### F13 - The commit body calls review 5923302003 the reviewer's third

- PR number: 2884
- Source review ID: 5923522198
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588443274
- Concern: the body of `docs(pr-reviews): [#2850] audit review round 3 of PR #2884` says it records da2ce7's "third review"; it is da2ce7's second review and the PR's third round.
- Solution: no change: the commit is pushed, and rewording it needs another force-push, which the maintainer approved only for the round-1 rewrite (F7). The reviewer marked it a slip, not a wrong fact about the tree; this row records the exact count. Reply: `Superseded by F7: ...`.
- Current-tree verification: N/A; the commit message is unchanged.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588467270
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588467270

## Processing Log

- 2026-11-16 15:48 UTC - Started this audit for da2ce7's review `5922865820` (submitted
  2026-11-16T15:46:38Z, approved, four findings), before any reply. F1 needs a rewritten commit
  message, so a force-push; the maintainer approved it at about 15:55 UTC, and the fixes of F2 to F4
  are squashed into the same reworded commit so the branch keeps one commit.
- 2026-11-16 16:00 UTC - Copilot's review `5922943922` (submitted 2026-11-16T15:51:42Z, one
  finding) and a conflict with `develop` in the EPIC #1347 table, reported by the maintainer. The
  branch was rebased onto `develop`, keeping row 13 (#2876) and row 14 of `develop` with row 12 of
  this PR and the later `last-updated-utc`.
- 2026-11-16 16:00 UTC - Both rounds processed. The five threads were replied to
  (2026-11-16T15:59 UTC), then the reworded, squashed and rebased commit was force-pushed by
  `git-bot-push --force-with-lease`; the threads are resolved after the push and the validator
  run on the pushed head.
- 2026-11-16 16:17 UTC - Started round 3 for da2ce7's review `5923302003` (submitted
  2026-11-16T16:14:46Z, changes requested, four findings), before any reply. Correction to the
  entry of 15:48 UTC: the maintainer's approval covered the squash as well as the force-push. The
  question was "May I squash the fixes into the reworded commit and force-push the bot's branch?"
  and the answer "Yes, force-push". The description was edited for F6 before this commit. Row F5
  changed from `RESOLVED` to `SUPERSEDED`, its reply and resolution reference now point to the
  second reply, and the F2 verification was reworded (F8 and F9).
- 2026-11-16 16:17 UTC - Round 3 replies posted (2026-11-16T16:16 to 16:17 UTC), including the
  second reply on the F5 thread. The threads are resolved after the push of this commit and a
  validator run on the pushed head.
- 2026-11-16 16:26 UTC - Round 4: Copilot's review `5923416979` (submitted 2026-11-16T16:21:55Z,
  one finding). The fix was committed as its own commit and the thread was replied to
  (2026-11-16T16:25:55Z). Late start: this entry and row F10 were added after the reply, not before
  it. The thread is resolved after the push and a validator run on the pushed head.
- 2026-11-16 16:29 UTC - Started round 5 for da2ce7's review `5923522198` (submitted
  2026-11-16T16:28:46Z, now `DISMISSED`, three findings), before any reply. The mapping of review
  commit ids was reworded to cite subjects only (F11).
- 2026-11-16 16:30 UTC - Round 5 replies posted (2026-11-16T16:30 UTC). The threads
  are resolved after the push of this commit and a validator run on the pushed head.

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
