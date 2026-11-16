---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/closed/2867-udp-health-check-panics-on-invalid-response/ISSUE.md
    - docs/pr-reviews/pr-2868-review/PR-REVIEW.md
---

<!-- skill-link: process-pr-review -->

# PR #2885 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2885>.

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

F1 to F4 come from the first da2ce7 review (COMMENTED, two Minors and two Nits, none Major), on the head ending in `docs(pr-reviews): [#2867] close the PR 2868 follow-up findings after PR 2879 merged`. The F2 fix reworded a commit and replayed the branch on the same base, so the review's commit ids map to subjects: `5bb40f69c` is `chore(issues): archive closed issue #2867 spec to docs/issues/closed` and `cb8ceec73` is `docs(pr-reviews): [#2867] close the PR 2868 follow-up findings after PR 2879 merged`. F5 comes from the first Copilot review (COMMENTED), on the same head; it asks for the change F1 already made.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2885-f1 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2885-f2 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2885-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2885-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2885-f5 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - The PR 2868 follow-up resolution references fail the audit validator

- PR number: 2885
- Source review ID: 5923256767
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2885#discussion_r4588235454>
- Concern: F7's and F8's new resolution references in the PR 2868 audit cited the PR 2879 merge commit with a parenthetical and a closing reply, which the validator rejects.
- Solution: Both cite the follow-up's fix commit `docs(issues): [#2867] record the regression tests, the fix recheck and the deferred PR 2868 findings`; the merge moved to Solution with the deferral reply, and the closing replies are the Reply URLs.
- Current-tree verification: `validate-audit-record.py` with its `gh` calls sent through `gh-bot` (`--pr-number 2868 --base torrust/develop`) reports status ok, 8 rows, 5 log entries, 0 failures.
- Resolution reference: docs(pr-reviews): [#2867] cite the fix commit as the resolution of the PR 2868 follow-ups
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2885#discussion_r4588303926>

### F2 - The archive commit's body matches the closing-keyword pattern

- PR number: 2885
- Source review ID: 5923256767
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2885#discussion_r4588235462>
- Concern: The archive commit's body said the merge "closed #2867", a match of the `open-pull-request` closing-keyword scan that would stay in `develop` history.
- Solution: Reworded the body to "PR 2879 merged, which closed issue #2867." and replayed the branch on the same base, keeping every commit; pushed with `--force-with-lease`.
- Current-tree verification: The skill's scan over the folded commit messages of the branch prints nothing; `git diff` between the old and new tips is empty.
- Resolution reference: chore(issues): archive closed issue #2867 spec to docs/issues/closed
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2885#discussion_r4588304428>

### F3 - The reviewer checkpoint was ticked though no reviewer edited the spec

- PR number: 2885
- Source review ID: 5923256767
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2885#discussion_r4588235478>
- Concern: The spec ticked "Reviewer validated acceptance criteria and updated checkboxes" although only the author account edited it.
- Solution: The checkpoint is open again, and a progress-log entry says the approving reviews validated the criteria but no reviewer edited the boxes.
- Current-tree verification: `grep -n "\[ \] Reviewer validated" docs/issues/closed/2867-udp-health-check-panics-on-invalid-response/ISSUE.md` prints the open checkpoint.
- Resolution reference: docs(issues): [#2867] leave the reviewer checkpoint open
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2885#discussion_r4588305015>

### F4 - The F7 and F8 verifications name a tree where the command cannot run

- PR number: 2885
- Source review ID: 5923256767
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2885#discussion_r4588235487>
- Concern: The PR 2868 audit said the greps run on `develop` after the follow-up merged, where the closed spec folder does not exist yet.
- Solution: Both verifications say they run at the head of the #2867 close-out PR, which moves the spec folder.
- Current-tree verification: At this head, the F7 grep prints lines 24 and 152 and the F8 grep prints line 39 of the evidence file.
- Resolution reference: docs(pr-reviews): [#2867] cite the fix commit as the resolution of the PR 2868 follow-ups
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2885#discussion_r4588305514>

### F5 - The Reply URL still points to the earlier thread reply

- PR number: 2885
- Source review ID: 5923323989
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2885#discussion_r4588289374>
- Concern: In the PR 2868 audit, F7's and F8's Resolution reference cited the closing reply while their Reply URL named the earlier deferral reply.
- Solution: None needed; F1's fix already made the closing replies the Reply URLs, cites the fix commit as the Resolution reference, and names the deferral replies in Solution.
- Current-tree verification: `grep -n "Reply URL" docs/pr-reviews/pr-2868-review/PR-REVIEW.md` prints `discussion_r4587758733` and `discussion_r4587759257` for F7 and F8.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2885#discussion_r4588479333>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2885#discussion_r4588479333>

## Processing Log

- 2026-11-16 16:17 UTC - Started audit. da2ce7 reviewed the head ending in `docs(pr-reviews): [#2867] close the PR 2868 follow-up findings after PR 2879 merged` at 16:11:50Z (COMMENTED, four inline threads), recorded as F1 to F4. F1 and F4 were fixed in one commit, F3 in another, and F2 by rewording the archive commit and replaying the branch. Replied on all four threads before the push.
- 2026-11-16 16:31 UTC - Copilot reviewed the same head at 16:15:52Z (COMMENTED, one inline thread), recorded as F5, a re-raise of F1 that F1's fix already covers. After the force-push, all five threads were resolved in one pass, the Copilot one before its reply; it was unresolved again, given its `Superseded by F1` reply, and resolved.

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
