---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2630 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2630>.

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

Round 1 comes from review 5632165728 (da2ce7, `CHANGES_REQUESTED`) on the first head of the branch.
Review 5632190535 (Copilot, `COMMENTED`) holds a summary with no actionable assertion, so it has no
row. Round 2 comes from Copilot review 5632626163 (`COMMENTED`), which has one inline finding (F3).
Round 3 comes from review 5633175715 (da2ce7, listed by GitHub as `DISMISSED`) with four inline
findings. Its reviewer-provided IDs F3 to F6 collide with F3 of this audit, so they are recorded as
F4 to F7 with the original ID in the detail entry. F4 repeats F3 and is a re-raise.
The branch history was rewritten in round 1 (see F1), so the round 1 review bodies cite a commit id
that no longer exists on the branch. That id named the commit with the subject
`docs(skills): state that a negated closing keyword still closes the issue`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2630-f1 | Human | Blocker | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2630-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2630-f3 | Copilot | Nit (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2630-f4 | Human | Minor | metadata | RE_RAISE_OF:F3 | FIXED | RESOLVED |
| F5 | review-finding:pr-2630-f5 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2630-f6 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2630-f7 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The commit message of the first head would close the reopened issue when it lands on develop

- PR number: 2630
- Source review ID: 5632165728
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4362473538>
- Concern: The message of the commit with the subject
  `docs(skills): state that a negated closing keyword still closes the issue` contained two
  closing-keyword and issue-reference pairs, which the skill's own scan matches. The maintainer
  merge tool keeps branch commits, and GitHub acts on the keyword in a commit message, so the merge
  would have closed issue 2606 again.
- Solution: Reworded the message of that commit with a signed `git commit --amend`, keeping the
  subject. The new message has no issue reference next to a keyword. The tree is unchanged. The
  history was rewritten, so the branch was pushed with `--force-with-lease`.
- Current-tree verification: The scan `git log --format=%B torrust/develop..HEAD | tr '\n' ' ' |
  grep -oiE <skill pattern>` prints nothing on the head with the subject
  `docs(skills): scan branch commit messages for closing keywords` (exit 1). The same command prints
  the two matches for the old message. Both commits have tree `a5b4a1969cc5`. The PR body scan also
  prints nothing.
- Resolution reference: docs(skills): state that a negated closing keyword still closes the issue
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4362556930>

### F2 - The scan covers the PR body but not the branch commit messages

- PR number: 2630
- Source review ID: 5632165728
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4362473557>
- Concern: The rule paragraph and the checklist line named only the PR body, although commit
  messages reach develop and reproduce the same accident. A line-based `grep` also misses a keyword
  whose reference wraps to the next line.
- Solution: The skill now covers the commit messages in the rule paragraph and the checklist line,
  gives a scan that folds the newlines and prints with `-o`, states the wrapped-line limit, and says
  to reword a matching message before the merge.
- Current-tree verification: The command printed nothing on the current history and printed the two
  matches for the old message. On a two-line sample whose reference wraps to the second line, the
  folded command printed a match and the line-based `grep` printed nothing.
- Resolution reference: docs(skills): scan branch commit messages for closing keywords
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4362558054>

### F3 - The audit names a branch commit by its id instead of its subject

- PR number: 2630
- Source review ID: 5632626163
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4362832351>
- Concern: The F1 detail entry cited the verification head by its commit id. The skill requires
  review audits to cite Conventional Commit subjects, because a history rewrite invalidates an id.
  The review marked the finding with a low-severity icon and no bracket.
- Solution: Replaced that id with the commit subject. A search of the audit and the PR body for
  commit ids found two more, one in the Findings introduction and one in the PR body, and both were
  replaced with descriptions that name the commit by its subject.
- Current-tree verification: Searching the audit for 9 to 12 character hex words finds only review,
  comment and tree ids. Searching the PR body finds none.
- Resolution reference: docs(pr-reviews): cite commit subjects instead of ids in the PR 2630 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4366492105>

### F4 - The audit verification cites a branch commit by its id (re-raise of F3)

- PR number: 2630
- Source review ID: 5633175715
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4363238973>
- Concern: The F1 verification named a branch commit by its id, which the `open-pull-request`
  commit-id check prints. This is the change F3 asked for, raised on a head that still carried it.
- Solution: Fixed by the F3 change before this review was processed. The reply says so and cites that
  commit subject.
- Current-tree verification: Searching the audit for 9 to 12 character hex words finds only review,
  comment and tree ids.
- Resolution reference: docs(pr-reviews): cite commit subjects instead of ids in the PR 2630 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4366525341>

### F5 - The description's Changes list omits the commit-message scan

- PR number: 2630
- Source review ID: 5633175715
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4363239000>
- Concern: The PR description listed only the negation sentence and the body scan, while the branch
  also adds the commit-message scan, and the merge commit copies the description.
- Solution: Edited the description. Its Changes section now lists the commit-message scan, the title
  scan, the post-creation checklist scan and the review audit. The fix is outside the repository
  tree, so the resolution reference is the conversation response that states it.
- Current-tree verification: The edited description printed no match for the skill's closing-keyword
  scan and cites no commit id.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2630#issuecomment-6381971943>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4366525435>

### F6 - Reviewer finding ID should be N/A and the template frontmatter is missing

- PR number: 2630
- Source review ID: 5633175715
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4363239013>
- Concern: The first two findings recorded the reviewer's own ID, although no ID was reassigned, and
  the audit dropped the frontmatter and the skill-link comment of the template.
- Solution: Set `Reviewer finding ID` to N/A in F1 and F2, and added the template frontmatter and the
  skill-link comment.
- Current-tree verification: The audit validator reports 0 failures and `linter all` exits 0, which
  includes the frontmatter check.
- Resolution reference: docs(pr-reviews): align the PR 2630 audit with the review template
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4366525530>

### F7 - The merge commit also carries the PR title

- PR number: 2630
- Source review ID: 5633175715
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4363239023>
- Concern: The merge commit subject is the PR title, and no scan covered it, so a title pairing a
  keyword with an issue would close it.
- Solution: The skill gained a paragraph below the commit-message block that says the merge commit
  subject carries the title and how to scan it, and the checklist line now names the title. The rule
  paragraph itself did not name the title in that commit, which F8 later corrected.
- Current-tree verification: `gh pr view 2630 --json title --jq .title` piped into the skill pattern
  prints nothing for this PR. A sample title that pairs a keyword with an issue matches.
- Resolution reference: docs(skills): scan the pull request title for closing keywords
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4366525665>

## Processing Log

- 2026-10-23 17:37 UTC - da2ce7 submitted review 5632165728 (`CHANGES_REQUESTED`) with threads
  4362473538 (F1) and 4362473557 (F2).
- 2026-10-23 17:39 UTC - Copilot submitted review 5632190535 (`COMMENTED`), a summary with no
  actionable assertion.
- 2026-10-23 17:43 UTC - Reworded the commit message for F1 (tree unchanged).
- 2026-10-23 17:44 UTC - Committed the F2 fix.
- 2026-10-23 17:46 UTC - Pushed the branch with `--force-with-lease`. The pre-push hooks passed.
- 2026-10-23 17:46 UTC - Replied to and resolved the F1 thread (reply 4362556930) and the F2
  thread (reply 4362558054).
- 2026-10-23 18:14 UTC - Copilot submitted review 5632626163 (`COMMENTED`) with thread 4362832351
  (F3).
- 2026-10-23 19:01 UTC - da2ce7 submitted review 5633175715 with threads 4363238973 (F4),
  4363239000 (F5), 4363239013 (F6) and 4363239023 (F7), and an ACK comment on the earlier head.
- 2026-10-24 08:09 UTC - Committed the F3 fix.
- 2026-10-24 08:11 UTC - Replied to and resolved the F3 thread (reply 4366492105).
- 2026-10-24 08:21 UTC - Committed the F7 fix.
- 2026-10-24 08:22 UTC - Committed the F6 fix.
- 2026-10-24 08:25 UTC - Posted the conversation response 6381971943 for F5, after editing the
  description.
- 2026-10-24 08:25 UTC - Replied to and resolved the F4 thread (reply 4366525341), the F5 thread
  (reply 4366525435), the F6 thread (reply 4366525530) and the F7 thread (reply 4366525665).

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
