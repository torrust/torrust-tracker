---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2950 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2950.

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

da2ce7's conversation comment 6987829006 is not a review, so, as in the PR #2430 audit, it is recorded in the Processing Log, not as a row.

Round 1 is Copilot review `5963231037` (`COMMENTED`, on head `docs(pr-reviews): [#2949] add the PR #2950 review audit record`), with two inline comments, F1 and F2 (audit-local IDs). Copilot labels both "Medium severity" in its overview; the severity `Minor` is inferred from that label, as in the PR #2905 audit. Its body says the full agentic review did not run because it did not start before the timeout.

Round 2 is Copilot review `5963780219` (`COMMENTED`, on head `docs(pr-reviews): [#2949] record PR #2950 review round 1`), with one inline comment, F3. Its overview counts one open issue and two resolved since the last review, and labels F3 "Medium severity", so the severity `Minor` is inferred from that label, as for F1 and F2. Its body carries the same timeout warning.

Round 3 is Copilot review `5964359494` (`COMMENTED`, on head `docs(pr-reviews): [#2949] record PR #2950 review round 2`), with two inline comments, F4 and F5. Its overview counts two open issues and one resolved since the last review, and labels both F4 and F5 "Medium severity", so `Minor` is inferred for both. Its body carries the same timeout warning. An earlier version of this record said rounds 2 and 3 gave no severity label and recorded F3 and F4 as `Nit`; da2ce7's F2 in review 5970994174 corrected it.

Round 4 is Copilot review `5964972845` (`COMMENTED`, on head `docs(pr-reviews): [#2949] record PR #2950 review round 3`) and da2ce7's review `5970994174` (`CHANGES_REQUESTED`, on the same head). Copilot's review has no inline comment and its overview counts no open issue, but its body asserts two independently actionable concerns in prose: the acceptance criteria conflict with the endpoint diff (F13), and the test plan omits the required test-design review (F14). Following the template's rule for a review-body finding without a thread, each is a row with the review URL as `Source URL` and `Thread state=NON_RESOLVABLE`; no badge rates them, so the severity `Minor` is inferred from the concern. da2ce7's F3 and F7 later asked for the same changes, so F8 and F12 are `RE_RAISE_OF:F13` and `RE_RAISE_OF:F14`. An earlier version of this record said a body assertion makes no row; da2ce7's F15 in review 5971317192 corrected it. da2ce7's review has seven inline comments with explicit severity labels, recorded as F6 to F12 with their reviewer finding IDs F1 to F7.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| --- | --- | --- | --- | --- | --- | --- | --- |
| F1 | `review-finding:pr-2950-f1` | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2950-f2` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2950-f3` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2950-f4` | Copilot | Minor (inferred) | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2950-f5` | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2950-f6` | Human | Major | formatting | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2950-f7` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2950-f8` | Human | Minor | correctness | RE_RAISE_OF:F13 | FIXED | RESOLVED |
| F9 | `review-finding:pr-2950-f9` | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2950-f10` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | `review-finding:pr-2950-f11` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | `review-finding:pr-2950-f12` | Human | Minor | testing | RE_RAISE_OF:F14 | FIXED | RESOLVED |
| F13 | `review-finding:pr-2950-f13` | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | NON_RESOLVABLE |
| F14 | `review-finding:pr-2950-f14` | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - The check command misses a file renamed into the audit folder

- PR number: 2950
- Source review ID: 5963231037
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4619336035
- Concern: `git diff --name-only` follows Git's rename detection, so a file moved from outside into the audit folder may be listed only by its new path and pass the "only files under the folder" check.
- Solution: The rule's command is now `git diff --no-renames --name-only X..H`, with a sentence saying why, and scenario M4 covers a file moved into the audit folder.
- Current-tree verification: On a throwaway repository with Git 2.53.0, after `git mv src/lib.rs docs/pr-reviews/pr-7-review/lib.rs`, `git diff --name-only X..HEAD` printed only `docs/pr-reviews/pr-7-review/lib.rs`, and `git diff --no-renames --name-only X..HEAD` printed both paths. The specification's rule under In Scope names `--no-renames`, and the M4 row is in the manual scenarios table.
- Resolution reference: `docs(issues): [#2949] require --no-renames in the ACK check command`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4619414676

### F2 - Regression Test Strategy contradicts the merge-tool tests

- PR number: 2950
- Source review ID: 5963231037
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4619336104
- Concern: Regression Test Strategy said "Not applicable" while the automatic checks, T5 and AC6 require merge-tool tests when D2 is (b).
- Solution: The section now says not applicable unless D2 is (b), and names the merge-tool unit tests for that case, matching M1 to M4.
- Current-tree verification: Read the Regression Test Strategy section of the specification on the current tree: it names the five test cases and the D2 (b) condition, consistent with T5, AC6 and Automatic Checks.
- Resolution reference: `docs(issues): [#2949] name the merge-tool tests in the regression test strategy`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4619415354

### F3 - Five test cases said to match four manual scenarios

- PR number: 2950
- Source review ID: 5963780219
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4619770882
- Concern: Regression Test Strategy listed five unit-test cases, including an ACK on the head, and then said they match M1 to M4.
- Solution: The paragraph now calls the ACK-on-head case the baseline with no manual scenario and maps the other four cases to M1 to M4 one by one (Copilot's option (a)).
- Current-tree verification: Read the Regression Test Strategy section on the current tree: it names M1, M2, M3 and M4 against one case each and says the ACK-on-head case has no manual scenario.
- Resolution reference: `docs(issues): [#2949] map the regression test cases to M1 to M4 one by one`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4619832826

### F4 - Checked facts cite line numbers that shift

- PR number: 2950
- Source review ID: 5964359494
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4620220580
- Concern: The checked facts cited `merge-pull-request/SKILL.md` line 63 and `wait-for-pr-review/SKILL.md` line 178, which shift as the skills change.
- Solution: The fact now names the sections: Merge Readiness, item Reviewer ACK; and The Review Loop, step 4.
- Current-tree verification: `grep -n "line 63\|line 178"` on the specification prints nothing; the fact names both sections.
- Resolution reference: `docs(issues): [#2949] cite skill sections instead of line numbers in the checked facts`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4620307218

### F5 - The folder match has no explicit form

- PR number: 2950
- Source review ID: 5964359494
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4620220646
- Concern: "Matched as a whole path segment" gives no explicit matching form, so an implementation could use substring matching and let `pr-29-review` match pull request 2947.
- Solution: The rule and D3 now state the match as the exact prefix `docs/pr-reviews/pr-<N>-review/`, trailing slash included, with the `pr-29-review` / `pr-2947-review` example.
- Current-tree verification: Read the rule under In Scope and D3 on the current tree: both name the exact prefix with its trailing slash.
- Resolution reference: `docs(issues): [#2949] state the audit-folder match as an exact prefix with its trailing slash`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4620307853

### F6 - The Findings table is column-padded, so the audit validator parses no tracking row

- PR number: 2950
- Source review ID: 5970994174
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625634074
- Concern: The validator's `ROW_RE` wants one space between the finding ID and the next pipe; prettier had padded every cell, so no row parsed and the record failed with 6 failures, which Merge Readiness treats as blocking.
- Solution: The Findings table is written without cell padding, as recent audits on `develop` are (MD060 is off), and prettier is no longer run on this file.
- Current-tree verification: The validator's own `main()`, run with its gh calls routed through the contributor account, reported 5 rows, 4 log entries and no failures after the fix; this round's record is checked the same way before it is pushed.
- Resolution reference: `docs(pr-reviews): [#2949] collapse the findings table padding so the validator parses it`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625748617

### F7 - Rounds 2 and 3 recorded as having no severity label

- PR number: 2950
- Source review ID: 5970994174
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625634081
- Concern: The record said Copilot reviews 5963780219 and 5964359494 gave no severity label and recorded F3 and F4 as `Nit (inferred)`, but each overview item has the alt text "Medium severity", the label this record maps to Minor.
- Solution: Rounds 2 and 3 state the label, F3 and F4 are `Minor (inferred)`, and the text says what the earlier version recorded.
- Current-tree verification: `gh api .../reviews/5964359494 --jq .body` shows "Medium severity" on all three overview items; the rows for F3 and F4 read `Minor (inferred)`.
- Resolution reference: `docs(pr-reviews): [#2949] record Copilot's Medium label for F3 and F4 as Minor`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625748897

### F8 - AC7 and the Goal judge each commit, but the command compares only the two ends

- PR number: 2950
- Source review ID: 5970994174
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625634089
- Concern: `git diff X..H` diffs two trees, so a change outside the folder that a later commit reverts leaves only the audit file listed and the ACK counts, while AC7 and the Goal say it is stale.
- Solution: The rule now means every commit: `git log --no-renames --diff-merges=first-parent --name-only --format= X..H`, with `--diff-merges=first-parent` so a change made in a merge commit is listed. AC7 says a reverted change still makes the ACK stale; M5 and its test case cover it.
- Current-tree verification: With Git 2.53.0 on a throwaway repository, the end-to-end diff of an edit-then-revert listed only the audit file and the new command also listed `src/lib.rs`; a merge commit with an extra change was listed only with `--diff-merges`. On `e01212151..1d27a67e9` the new command lists only `docs/pr-reviews/pr-2947-review/PR-REVIEW.md`.
- Resolution reference: `docs(issues): [#2949] check every commit after the ACK, not only the two ends`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625749244

### F9 - The prefix example does not tell the matching forms apart

- PR number: 2950
- Source review ID: 5970994174
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625634097
- Concern: Every candidate form rejects `pr-2947-review` for N = 29, so the example in the rule, D3 and M3 distinguishes no form and a test built from it passes a wrong implementation. This also corrects the premise of F5's Concern in this record.
- Solution: The rule, D3, M3 and its test case use `docs/pr-reviews/pr-29-review-old/x` and `docs/pr-reviews/pr-29-reviewer.md` (accepted by a prefix without the slash) and `src/docs/pr-reviews/pr-29-review/x` (accepted by a substring test).
- Current-tree verification: Read the rule under In Scope, D3 and the M3 row on the current tree: all three name the sibling and nested paths.
- Resolution reference: `docs(issues): [#2949] show the folder match with paths that tell the forms apart`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625749529

### F10 - Background, EPIC row 24 and the description still say the rule was decided

- PR number: 2950
- Source review ID: 5970994174
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625634104
- Concern: Background, EPIC row 24 and the description's Summary said the maintainer decided to adopt the rule, while D1 leaves it open after da2ce7 disagreed.
- Solution: Background says the decision of 2026-11-18 was reopened under D1; row 24 says the rule was proposed and its adoption is open, with an EPIC progress-log line. The description's Summary and the body of #2949 say the same.
- Current-tree verification: `grep -n 'decided by the maintainer' EPIC.md` prints nothing; Background and the description read as stated.
- Resolution reference: `docs(issues): [#2949] say the rule was proposed and reopened under D1, not decided`; https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625749826
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625749826

### F11 - The description states the rule's command without `--no-renames`

- PR number: 2950
- Source review ID: 5970994174
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625634111
- Concern: The description's Summary gave `git diff --name-only X..H`, the form the specification calls insufficient, and the merge tool copies the description into the merge commit.
- Solution: The Summary gives the command as `git log --no-renames --diff-merges=first-parent --name-only --format= X..H`, the form after F8. The description is outside the diff, so no commit carries this fix.
- Current-tree verification: `gh pr view 2950 --json body` shows the new command in the Summary.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625750111
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625750111

### F12 - Commit Points leaves out the test-design review for T5

- PR number: 2950
- Source review ID: 5970994174
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625634115
- Concern: `create-issue` requires a progressive test-development loop when the plan adds tests, and the template carries it in Commit Points; T5 adds merge-tool tests when D2 is (b) but said only to commit after the tests pass.
- Solution: Commit Points has the template's opening sentence and, conditional on D2 (b), the loop: `write-unit-test`, one increment per test case, a recorded prose-first Arrange-Act-Assert design review after each, and a maintainer stop after the last one.
- Current-tree verification: Read Commit Points on the current tree: the paragraph follows the table and T5's policy points to it.
- Resolution reference: `docs(issues): [#2949] add the test-development loop to Commit Points for T5`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625750772

### F13 - The acceptance criteria conflict with the endpoint-diff rule

- PR number: 2950
- Source review ID: 5964972845
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2950#pullrequestreview-5964972845>
- Concern: The review body says human assessment is needed because the ACK acceptance criteria conflict with the endpoint-diff rule: AC7 and the Goal judge each commit after the ACK, while the command compared only the two ends.
- Solution: Fixed by the change F8 records: the rule checks every commit after the ACK.
- Current-tree verification: F8's checks hold at the branch head: the rule under In Scope gives `git log --no-renames --diff-merges=first-parent --name-only --format= X..H`, and AC7 says a reverted change still makes the ACK stale.
- Resolution reference: `docs(issues): [#2949] check every commit after the ACK, not only the two ends`
- Follow-up PR URL: N/A
- Reply URL: N/A (no thread; F8's reply answers the same concern)

### F14 - The test plan omits the required test-design review

- PR number: 2950
- Source review ID: 5964972845
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2950#pullrequestreview-5964972845>
- Concern: The review body says the test plan omits the required test-design review gates for the merge-tool tests that T5 adds when D2 is (b).
- Solution: Fixed by the change F12 records: Commit Points carries the test-development loop for T5.
- Current-tree verification: F12's check holds at the branch head: the paragraph follows the Commit Points table, conditional on D2 (b).
- Resolution reference: `docs(issues): [#2949] add the test-development loop to Commit Points for T5`
- Follow-up PR URL: N/A
- Reply URL: N/A (no thread; F12's reply answers the same concern)

## Processing Log

- 2026-11-18 18:34 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - Started audit. The poller reported da2ce7's conversation comment [6987829006](https://github.com/torrust/torrust-tracker/pull/2950#issuecomment-6987829006) (2026-11-18 18:31:31 UTC) on head `docs(issues): add issue specification for #2949`; the pull request had no review. The comment does not agree to the rule as written, points to their proposal on #2949 ([issue comment 6987828129](https://github.com/torrust/torrust-tracker/issues/2949#issuecomment-6987828129), 18:31:29 UTC: a metadata branch folded in with a three-parent merge), and asks that the two be compared before deciding. A conversation comment is not a review, so it makes no row. The comparison was added to the specification in `docs(issues): [#2949] compare da2ce7's metadata-branch alternative under D1`; no decision was made, the maintainer decides D1.
- 2026-11-18 19:13 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - Round 1: the poller reported Copilot review 5963231037 (submitted 2026-11-18 19:06:19 UTC). Normalized its two inline comments as F1 and F2, fixed each in its own commit, pushed both, and replied on both threads, then resolved both threads.
- 2026-11-18 19:48 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - Round 2: the poller reported Copilot review 5963780219 (submitted 2026-11-18 19:41:53 UTC). Normalized its inline comment as F3, fixed it in its own commit, pushed it, replied on the thread, then resolved the thread.
- 2026-11-18 20:26 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - Round 3: the poller reported Copilot review 5964359494 (submitted 2026-11-18 20:19:07 UTC). Normalized its two inline comments as F4 and F5, fixed each in its own commit, pushed both, replied on both threads, then resolved both threads.
- 2026-11-19 07:48 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - Round 4: the poller reported da2ce7's review 5970994174 (submitted 2026-11-19 07:32:57 UTC, `CHANGES_REQUESTED`). Normalized its seven inline comments as F6 to F12, fixed each in its own commit (F11 in the description, which has no commit), pushed the fixes, replied on all seven threads, then resolved them. Copilot review 5964972845 (2026-11-18 20:55:36 UTC) on the round-3 head was first read as clean; its body raised the concerns of F8 and F12 in prose, which da2ce7's review pointed out. F9 corrects the premise of F5's Concern and of the F5 fix commit's body, which stay as history.

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
