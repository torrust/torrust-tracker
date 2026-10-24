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
Round 3 comes from review 5633175715 (da2ce7, submitted as `APPROVED`, now `DISMISSED`)
with four inline findings. Its reviewer-provided IDs F3 to F6 collide with F3 of this audit, so they
are recorded as F4 to F7 with the original ID in the detail entry. F4 repeats F3 and is a re-raise.
Round 4 comes from review 5637566834 (da2ce7, submitted as `APPROVED`, now `DISMISSED`) with three
inline findings, F8 to F10. Its IDs do not collide with the audit, so they are kept. Copilot review
5637532312 (`COMMENTED`) is a summary with no inline thread, so it has no row. Round 5 comes from
Copilot review 5637658378 (`COMMENTED`) on the head that round 4 reviewed, requested before the
round 4 fixes were pushed, with three inline findings, F11 to F13. A warning at the top of that
review says Copilot could not run its full agentic review before the timeout. Copilot review
5637742895 (`COMMENTED`), on the head after the round 4 fixes, is a summary that lists F11 to F13 as
open, so it has no row. Round 6 comes from review 5638563262 (da2ce7, `APPROVED`) on the head after
the round 5 fixes, with four inline findings, F14 to F17. Its IDs do not collide with the audit, so
they are kept. Copilot review 5638250321 (`COMMENTED`) on the same head has no open finding, so it
has no row.
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
| F8 | review-finding:pr-2630-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2630-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2630-f10 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2630-f11 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2630-f12 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2630-f13 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2630-f14 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2630-f15 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2630-f16 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2630-f17 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

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

### F8 - The rule sentence does not name the title

- PR number: 2630
- Source review ID: 5637566834
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4366565812>
- Concern: The merge tool writes the title after the pull request reference in the merge commit
  subject, and the rule sentence still named only the body and the commit messages, so the title
  paragraph below it was a second rule. The description said the title "is the subject".
- Solution: The rule sentence now names the title and the scan before creating or editing it. The
  title paragraph says the merge commit subject carries the title. The description says the same.
- Current-tree verification: Read the rule sentence, the title paragraph and the description after
  the change. `linter all` exits 0.
- Resolution reference: docs(skills): name the title in the closing keyword rule sentence
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4366609403>

### F9 - The audit and the reply say the rule paragraph names the title

- PR number: 2630
- Source review ID: 5637566834
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4366565816>
- Concern: The F7 entry said the rule paragraph names the title, but the title was in a new
  paragraph below the commit-message block.
- Solution: The F7 entry now states what that commit changed and that F8 corrected the rule
  sentence.
- Current-tree verification: Read the F7 entry and the skill. The rule sentence names the title after
  the F8 fix.
- Resolution reference: docs(pr-reviews): say where the PR 2630 skill change names the title
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4366609531>

### F10 - The conversation response cited for F5 calls the finding by the reviewer's id

- PR number: 2630
- Source review ID: 5637566834
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4366565823>
- Concern: The response opened with "finding F4", the reviewer's id, which is a different finding in
  this audit, so a reader following the F5 resolution reference landed on another id.
- Solution: Edited the response so that it names both ids for every finding it mentions and says it
  was edited. The fix is outside the repository tree, so the resolution reference is the edited
  response.
- Current-tree verification: Read the edited comment through the API (last edited at 2026-10-24
  08:56 UTC). It names both ids for each finding.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2630#issuecomment-6381971943>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4366609614>

### F11 - The scan pattern uses a word boundary that is not portable

- PR number: 2630
- Source review ID: 5637658378
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4366639415>
- Concern: The pattern began with `\b` under `grep -E`, which some implementations, such as BSD and
  macOS `grep`, may handle differently, so the documented scan could miss matches. The review gave
  the severity as Medium with an icon and no bracket.
- Solution: Replaced `\b` with the explicit boundary `(^|[^[:alnum:]_])` and stated the reason in the
  skill. The class name `alnum` was added to `project-words.txt` because the spell checker rejected
  it.
- Current-tree verification: Loaded the pattern from the skill and ran 13 cases. It matched the
  plain, negated, past tense, `owner/repo#N`, URL, colon and wrapped forms, and it did not match
  `Related to`, `closure`, `closing`, a word between the keyword and the reference, or a prefixed
  word. It was not run on BSD or macOS `grep`, so portability rests on the pattern using only POSIX
  constructs.
- Resolution reference: docs(skills): make the closing keyword scan portable and commit-safe
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4367080421>

### F12 - Folding all commit messages can match across two commits

- PR number: 2630
- Source review ID: 5637658378
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4366639440>
- Concern: Folding the newlines of all messages into one line removed the commit boundaries, so a
  keyword ending one message could match a reference starting the next, a false positive.
- Solution: The command now prints a `--COMMIT--` line after each message, so the pattern cannot
  match across two messages.
- Current-tree verification: For a message ending in `close` followed by a message starting with a
  reference, the command printed nothing with the separator and printed a match without it. The
  branch messages printed nothing, and the old bad message still printed its two matches.
- Resolution reference: docs(skills): make the closing keyword scan portable and commit-safe
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4367080570>

### F13 - The title scan instruction is not a valid command

- PR number: 2630
- Source review ID: 5637658378
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4366639459>
- Concern: The skill told readers to pipe the title into the first command in place of
  `<body-file>`, which is not a valid command, so readers could skip the title scan. Round 4 did not
  report it. Round 3 could not have, because the sentence came after that review.
- Solution: Replaced the sentence with a command that runs, and defined the pattern once in a
  `PATTERN=` line that the body, commit and title commands reuse.
- Current-tree verification: With the pattern loaded from the skill, the title command printed
  nothing for this PR and matched a sample title that pairs a keyword with an issue.
- Resolution reference: docs(skills): make the closing keyword scan portable and commit-safe
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4367080665>

### F14 - The audit omits Copilot's summary review 5637742895

- PR number: 2630
- Source review ID: 5638563262
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4367387635>
- Concern: Copilot review 5637742895 is a summary that re-lists F11 to F13 as open and has no new
  thread. The audit named the earlier summary 5637532312 but not this one, although it came before
  the head the reviewer checked.
- Solution: Added a sentence to the introduction and a log entry. The review needs no row.
- Current-tree verification: The introduction and the Processing Log name review 5637742895. The
  audit validator reports 0 failures.
- Resolution reference: docs(pr-reviews): record the Copilot summary review 5637742895 in the PR 2630 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4367619090>

### F15 - Round 3 could not have reported the title scan sentence

- PR number: 2630
- Source review ID: 5638563262
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4367387641>
- Concern: The F13 Concern said rounds 3 and 4 did not report the sentence, but the sentence reached
  the branch after round 3's review.
- Solution: The F13 Concern now says that round 4 did not report it and that round 3 could not have.
- Current-tree verification: The review API gives review 5633175715 a commit before the sentence and
  review 5637566834 a commit after it.
- Resolution reference: docs(pr-reviews): correct the round and review statements in the PR 2630 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4367619186>

### F16 - Round 5 is on the head round 4 reviewed, and two review states are stale

- PR number: 2630
- Source review ID: 5638563262
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4367387646>
- Concern: The introduction said review 5637658378 was on the head after round 4, but it was on the
  head round 4 reviewed and was requested before the round 4 fixes were pushed. It also gave round 4
  as `APPROVED` and round 3 as `DISMISSED`, although both reviews are now dismissed.
- Solution: The introduction names the head round 5 reviewed, names review 5637742895 as the one on
  the head after the round 4 fixes, and gives the submitted and the present state of both reviews.
- Current-tree verification: The timeline's `review_dismissed` events give the submitted state of
  each dismissed review, and the review API gives its present state and its commit. The
  introduction matches both.
- Resolution reference: docs(pr-reviews): correct the round and review statements in the PR 2630 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4367619311>

### F17 - The description does not name the dictionary entry

- PR number: 2630
- Source review ID: 5638563262
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4367387650>
- Concern: The description said the PR changes skill documentation and the audit only, but
  `project-words.txt` also gains `alnum`, and the description goes into the merge commit.
- Solution: Edited the description to name the dictionary entry. The fix is outside the repository
  tree, so the resolution reference is the conversation response that states it.
- Current-tree verification: The edited description names the entry, prints no match for the skill's
  closing-keyword scan and cites no commit id.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2630#issuecomment-6386192343>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2630#discussion_r4367619433>

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
- 2026-10-24 08:32 UTC - Copilot submitted review 5637532312 (`COMMENTED`), a summary with no inline
  thread.
- 2026-10-24 08:42 UTC - da2ce7 submitted review 5637566834 (`APPROVED`) with threads 4366565812
  (F8), 4366565816 (F9) and 4366565823 (F10), and an ACK comment on the head that carried the
  round 3 audit.
- 2026-10-24 08:55 UTC - Committed the F8 fix.
- 2026-10-24 08:56 UTC - Committed the F9 fix, and edited the conversation response 6381971943 for
  F10.
- 2026-10-24 08:58 UTC - Replied to and resolved the F8 thread (reply 4366609403), the F9 thread
  (reply 4366609531) and the F10 thread (reply 4366609614).
- 2026-10-24 09:10 UTC - Copilot submitted review 5637658378 (`COMMENTED`) with threads 4366639415
  (F11), 4366639440 (F12) and 4366639459 (F13).
- 2026-10-24 09:26 UTC - Copilot submitted review 5637742895 (`COMMENTED`), a summary with no new
  thread that lists F11 to F13 as open.
- 2026-10-24 12:10 UTC - Committed the F11, F12 and F13 fix in one commit, because the three findings
  change the same command block.
- 2026-10-24 12:13 UTC - Replied to and resolved the F11 thread (reply 4367080421), the F12 thread
  (reply 4367080570) and the F13 thread (reply 4367080665).
- 2026-10-24 12:34 UTC - Copilot submitted review 5638250321 (`COMMENTED`), a summary with no open
  finding.
- 2026-10-24 14:16 UTC - da2ce7 submitted review 5638563262 (`APPROVED`) with threads 4367387635
  (F14), 4367387641 (F15), 4367387646 (F16) and 4367387650 (F17), and an ACK comment on the head
  that carried the round 5 audit.
- 2026-10-24 15:40 UTC - Committed the F15 and F16 fix in one commit, because both correct
  statements about the reviews in the audit.
- 2026-10-24 15:42 UTC - Committed the F14 fix.
- 2026-10-24 15:44 UTC - Posted the conversation response 6386192343 for F17, after editing the
  description.
- 2026-10-24 15:45 UTC - Replied to and resolved the F14 thread (reply 4367619090), the F15 thread
  (reply 4367619186), the F16 thread (reply 4367619311) and the F17 thread (reply 4367619433).

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
