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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| --- | --- | --- | --- | --- | --- | --- | --- |
| F1 | review-finding:pr-2884-f1 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2884-f2 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2884-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2884-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2884-f5 | Copilot | Suggestion (inferred) | link-integrity | ORIGINAL | NO_ACTION | RESOLVED |

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
- Current-tree verification: `grep -rn "2026-11-16 14:10"` finds none of the changed files.
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
- Solution: no change. The entry records where the spec was at that time, and step 4 of
  `cleanup-completed-issues` keeps such historical records as they are, as F3 asked for the run
  script. The live references to #2850 in the same spec (the `related-artifacts` frontmatter and
  the note at the top, which links `../2850-non-negative-number-of-bytes/ISSUE.md`) point to the
  archived spec.
- Current-tree verification: the frontmatter of the #2838 spec lists
  `docs/issues/closed/2850-non-negative-number-of-bytes/ISSUE.md`, and its top note's relative link
  resolves inside `docs/issues/closed/`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588096540
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2884#discussion_r4588096540

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
