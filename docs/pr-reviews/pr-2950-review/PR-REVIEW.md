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

The round-8 rewrite of the specification as a decision record (`docs(issues): [#2949] record the decision to keep the strict ACK`) removed the text that the "Current-tree verification" lines of F2, F3, F5, F9, F12, F13, F14, F23 and F24, and the M4 part of F1's, describe. Those lines were true when written; the rows stay as written.

Round 1 is Copilot review `5963231037` (`COMMENTED`, on head `docs(pr-reviews): [#2949] add the PR #2950 review audit record`), with two inline comments, F1 and F2 (audit-local IDs). Copilot labels both "Medium severity" in its overview; the severity `Minor` is inferred from that label, as in the PR #2905 audit. Its body says the full agentic review did not run because it did not start before the timeout.

Round 2 is Copilot review `5963780219` (`COMMENTED`, on head `docs(pr-reviews): [#2949] record PR #2950 review round 1`), with one inline comment, F3. Its overview counts one open issue and two resolved since the last review, and labels F3 "Medium severity", so the severity `Minor` is inferred from that label, as for F1 and F2. Its body carries the same timeout warning.

Round 3 is Copilot review `5964359494` (`COMMENTED`, on head `docs(pr-reviews): [#2949] record PR #2950 review round 2`), with two inline comments, F4 and F5. Its overview counts two open issues and one resolved since the last review, and labels both F4 and F5 "Medium severity", so `Minor` is inferred for both. Its body carries the same timeout warning. An earlier version of this record said rounds 2 and 3 gave no severity label and recorded F3 and F4 as `Nit`; da2ce7's F2 in review 5970994174 corrected it.

Round 4 is Copilot review `5964972845` (`COMMENTED`, on head `docs(pr-reviews): [#2949] record PR #2950 review round 3`) and da2ce7's review `5970994174` (`CHANGES_REQUESTED`, on the same head). Copilot's review has no inline comment and its overview counts no open issue, but its body asserts two independently actionable concerns in prose: the acceptance criteria conflict with the endpoint diff (F13), and the test plan omits the required test-design review (F14). Following the template's rule for a review-body finding without a thread, each is a row with the review URL as `Source URL` and `Thread state=NON_RESOLVABLE`; no badge rates them, so the severity `Minor` is inferred from the concern. da2ce7's F3 and F7 later asked for the same changes, so F8 and F12 are `RE_RAISE_OF:F13` and `RE_RAISE_OF:F14`. An earlier version of this record said a body assertion makes no row; da2ce7's F15 in review 5971317192 corrected it. da2ce7's review has seven inline comments with explicit severity labels, recorded as F6 to F12 with their reviewer finding IDs F1 to F7.

Round 5 is da2ce7's review `5971317192` (`APPROVED`, on head `docs(pr-reviews): [#2949] record PR #2950 review round 4`), with two inline comments that carry their own finding IDs, F15 and F16, and explicit severity labels. It is followed by Copilot review `5971362548` (`COMMENTED`, on head `docs(pr-reviews): [#2949] record Copilot's Medium label for F3 and F4 as Minor`), with two inline comments, recorded as F20 and F21 because da2ce7's next review uses F17 to F19. Copilot labels F20 "Medium severity" and F21 "Low severity"; the severities `Minor` and `Nit` are inferred from those labels. Its body carries the same timeout warning as rounds 1 to 3.

Round 6 is da2ce7's review `5971496221` (`COMMENTED`, on head `docs(pr-reviews): [#2949] record Copilot's two body findings as non-resolvable rows`), with three inline comments carrying their own finding IDs, F17 to F19, and explicit severity labels.

Round 7 is da2ce7's review `5971904090` (`APPROVED`, on head `docs(pr-reviews): [#2949] record PR #2950 review round 6`), with one inline comment carrying its own finding ID, F22, and an explicit severity label, and Copilot review `5972058838` (`COMMENTED`, on the same head), with two inline comments that label themselves `[Minor][F22]` and `[Minor][F23]`. Because da2ce7's comment already uses F22, Copilot's two are recorded as F23 and F24, with Copilot's own IDs as their reviewer finding IDs.

Round 8 is da2ce7's review `5976843973` (`APPROVED`, on head `docs(pr-reviews): [#2949] record PR #2950 review round 7`), with two inline comments carrying their own finding IDs, F26 and F27, and explicit severity labels. The review uses no F25, so this record has no F25 row either. The maintainer decided the specification's D1 in the working session before the fixes: the strict ACK stays. The specification was rewritten as the record of that decision, which is why F26 needs no change.

Round 9 is da2ce7's review `5981273810` (`CHANGES_REQUESTED`, on head `docs(pr-reviews): [#2949] record PR #2950 review round 8`), with three inline comments carrying their own finding IDs, F28 to F30, and explicit severity labels. F28 is about the pull request description, which is outside the diff, so no commit carries its fix.

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
| F15 | `review-finding:pr-2950-f15` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | `review-finding:pr-2950-f16` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | `review-finding:pr-2950-f17` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | `review-finding:pr-2950-f18` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F19 | `review-finding:pr-2950-f19` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F20 | `review-finding:pr-2950-f20` | Copilot | Minor (inferred) | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |
| F21 | `review-finding:pr-2950-f21` | Copilot | Nit (inferred) | formatting | ORIGINAL | FIXED | RESOLVED |
| F22 | `review-finding:pr-2950-f22` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F23 | `review-finding:pr-2950-f23` | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F24 | `review-finding:pr-2950-f24` | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F26 | `review-finding:pr-2950-f26` | Human | Nit | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |
| F27 | `review-finding:pr-2950-f27` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F28 | `review-finding:pr-2950-f28` | Human | Major | workflow | ORIGINAL | FIXED | RESOLVED |
| F29 | `review-finding:pr-2950-f29` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F30 | `review-finding:pr-2950-f30` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

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

### F15 - A body assertion said to make no row, against the template

- PR number: 2950
- Source review ID: 5971317192
- Reviewer finding ID: F15
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625888791
- Concern: The round-4 paragraph said "A body assertion is not a thread, so it makes no row", which contradicts `docs/templates/PR-REVIEW-TEMPLATE.md` and `process-pr-review`: an independently actionable review-body finding gets a `NON_RESOLVABLE` row with the review URL as its source.
- Solution: Copilot review 5964972845's two body concerns are F13 and F14, `NON_RESOLVABLE`, and F8 and F12 are their re-raises; the round-4 paragraph states the template's rule and what the earlier version said.
- Current-tree verification: The validator's own `main()`, run with its gh calls routed through the contributor account, reported 14 rows and no failures after the fix.
- Resolution reference: `docs(pr-reviews): [#2949] record Copilot's two body findings as non-resolvable rows`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625956424

### F16 - The 2026-11-18 event described two ways

- PR number: 2950
- Source review ID: 5971317192
- Reviewer finding ID: F16
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625888803
- Concern: EPIC row 24 and its log line said the maintainer proposed the rule on 2026-11-18, while Background said the maintainer decided it then and D1 reopened it.
- Solution: Background, EPIC row 24, both log lines and the description use one wording: the decision of 2026-11-18 was reopened under D1. The log lines were reworded in place; F19 records the correction.
- Current-tree verification: `grep -n 'was proposed\|proposed by the maintainer'` on the specification and the EPIC prints only the two 08:20 correction entries, which quote the old wording; EPIC row 24 and Background do not match. (Scoped in round 7 after da2ce7's F22; it first said the grep prints nothing.)
- Resolution reference: `docs(issues): [#2949] describe the 2026-11-18 decision one way in the spec and EPIC`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625956795

### F20 - The empty `--format=` value is easy to misread

- PR number: 2950
- Source review ID: 5971362548
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625921959
- Concern: The command's `--format= X..H` relies on an empty `--format` value followed by a space; Copilot suggests `--pretty=format:`.
- Solution: No change. `--format=<format>` is Git's documented form and the one the rule was tested with; `--pretty=format:` would change the separator semantics for no benefit.
- Current-tree verification: On Git 2.53.0, `git log --no-renames --diff-merges=first-parent --name-only --format= e01212151..1d27a67e9` printed only `docs/pr-reviews/pr-2947-review/PR-REVIEW.md`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4626034114
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4626034114

### F21 - "behaviour" and "behavior" both used in the specification

- PR number: 2950
- Source review ID: 5971362548
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4625922011
- Concern: The specification used "behaviour" twice and "behavior" three times.
- Solution: Both occurrences of "behaviour" are now "behavior", the spelling of the issue template.
- Current-tree verification: `grep -c behaviour` on the specification prints 0.
- Resolution reference: `docs(issues): [#2949] spell behavior one way in the specification`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4626034520

### F17 - Round-5 threads answered but not recorded

- PR number: 2950
- Source review ID: 5971496221
- Reviewer finding ID: F17
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4626029117
- Concern: da2ce7's F15 and F16 were answered and resolved, but the audit had no row for either and no log entry for review 5971317192.
- Solution: F15 and F16 are rows with their own finding IDs, source comments and reply URLs, and a round-5 log entry covers the review.
- Current-tree verification: The validator's own `main()`, run with its gh calls routed through the contributor account, reported 18 rows, 6 log entries and no failures after the fix.
- Resolution reference: `docs(pr-reviews): [#2949] record PR #2950 review round 5`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4626178697

### F18 - Copilot's two round-5 threads without reply, row or resolution

- PR number: 2950
- Source review ID: 5971496221
- Reviewer finding ID: F18
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4626029138
- Concern: Copilot review 5971362548's two threads had no reply, no row and no resolution.
- Solution: Both are rows (F20 `NO_ACTION`/`SUPERSEDED`, F21 `FIXED`), each thread has a reply, and both are resolved; the round-5 entry records that they were first resolved before any reply and reopened.
- Current-tree verification: The GraphQL thread listing shows both threads resolved, each with a reply by the contributor account.
- Resolution reference: `docs(pr-reviews): [#2949] record PR #2950 review round 5`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4626179113

### F19 - Dated log entries reworded in place

- PR number: 2950
- Source review ID: 5971496221
- Reviewer finding ID: F19
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4626029151
- Concern: The F16 fix reworded the EPIC's 07:39 entry and the specification's 07:40 entry in place, so they described a wording first made at 08:06, and neither log had an 08:06 entry.
- Solution: Each log has an appended 08:20 entry that says when and in which commit the earlier entry was reworded and what it said before.
- Current-tree verification: Read both progress logs on the current tree: the 08:20 entry follows the 07:39 entry in the EPIC and the 07:40 entry in the specification.
- Resolution reference: `docs(issues): [#2949] log the in-place rewording of the 07:39 and 07:40 entries`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4626179554

### F22 - The 08:20 corrections say Background said "proposed"

- PR number: 2950
- Source review ID: 5971904090
- Reviewer finding ID: F22
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4626342004
- Concern: The 08:20 correction entries said the first wording "was the wording of row 24 and Background at that time", but Background never said "proposed"; the same clause made F16's verification line inaccurate.
- Solution: Each log has an appended correction saying only row 24 said "proposed"; F16's verification line is scoped to what the grep prints. The fix was first deferred (reply of 2026-11-19 08:56:52 UTC) and made in round 7.
- Current-tree verification: The grep in F16's verification prints only the two 08:20 entries; both logs have the appended correction after them.
- Resolution reference: `docs(issues): [#2949] correct the 08:20 log entries about what said "proposed"`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4629683773

### F23 - The D2 (b) design leaves the Git-history step unspecified

- PR number: 2950
- Source review ID: 5972058838
- Reviewer finding ID: F22
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4626459790
- Concern: Design and Ownership Review called the work not applicable because D2 (b) changes "one pure function", but `get_acks_from_comments` receives only the head hash and the comments, so ancestry and changed-path data need Git I/O that nothing specifies, including what happens when it fails.
- Solution: The section is not applicable only for D2 (a). For D2 (b) it says `main()` acquires the history after fetching `refs/pull/<N>/*`, passes it to the pure matching function, and fails closed when a commit is missing, not an ancestor, ambiguous, or a Git command fails.
- Current-tree verification: Read Design and Ownership Review on the current tree: it has the D2 (a) exemption and the four D2 (b) points.
- Resolution reference: `docs(issues): [#2949] specify the merge tool's Git-history step and its failures for D2 (b)`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4629684234

### F24 - Unit tests allowed to replace the manual scenarios

- PR number: 2950
- Source review ID: 5972058838
- Reviewer finding ID: F23
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4626459836
- Concern: The note let the merge tool's tests replace M1 to M5 "instead of by hand", which the issue template forbids (running automated tests is not manual verification).
- Solution: The note says the tests supplement the manual runs; M1 to M5 are still run by hand with the documented command and their output recorded.
- Current-tree verification: `grep -n 'instead of by hand'` on the specification prints nothing; the note names the template rule.
- Resolution reference: `docs(issues): [#2949] keep the manual runs of M1 to M5 alongside the merge-tool tests`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4629684802

### F26 - The D2 (b) design omits lifetime, deadline and design-review items

- PR number: 2950
- Source review ID: 5976843973
- Reviewer finding ID: F26
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4630098689
- Concern: For D2 (b), Design and Ownership Review covered responsibilities and failure handling but not the template's lifetime invariants, readiness deadline and post-slice design-review checkpoint.
- Solution: No change. The maintainer decided D1 on 2026-11-19 to keep the strict ACK, so the merge tool does not change and the D2 (b) design was removed with the rewrite of the specification as a decision record.
- Current-tree verification: `grep -n 'D2 (b)' docs/issues/open/2949-2003-keep-reviewer-ack-after-audit-only-commits/ISSUE.md` prints only Progress Log entries; Design and Ownership Review says "Not applicable".
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4633369542
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4633369542

### F27 - The unverified point under D1 (c) is answered by GitHub's documentation

- PR number: 2950
- Source review ID: 5976843973
- Reviewer finding ID: F27
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4630098670
- Concern: The specification marked as not verified whether GitHub marks a pull request merged through a three-parent merge, and proposed a spike for it; GitHub's documentation on indirect merges answers it, and the point the tree leaves open is ruleset push acceptance.
- Solution: The maintainer rejected (c) on 2026-11-19. Its record now cites the Indirect merges section (read on 2026-11-19), says the merge tool and Merge Readiness enforce the review and not GitHub, and names ruleset 87085 push acceptance with a third parent as not tested. The "Not verified" sentence and the spike are gone from the specification and the description.
- Current-tree verification: `grep -n 'Not verified\|spike' docs/issues/open/2949-2003-keep-reviewer-ack-after-audit-only-commits/ISSUE.md` prints only Progress Log entries; the section "D1" has the bullet list "Facts about (c)".
- Resolution reference: `docs(issues): [#2949] record the decision to keep the strict ACK`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4633369020

### F28 - The description has a closing keyword before the issue reference

- PR number: 2950
- Source review ID: 5981273810
- Reviewer finding ID: F28
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4633520845
- Concern: The round-8 description said the follow-up work lands "in one small pull request that closes #2949". The merge tool copies the description into the merge commit, so merging this spec-only pull request would have closed the issue.
- Solution: The sentence now reads "Skill text only, in one small follow-up pull request for #2949:". The description is outside the diff, so no commit carries this fix. The sentence was written in round 8 without running the closing-keyword scan of `open-pull-request`.
- Current-tree verification: The pattern of `open-pull-request` (Issue Linking Rules) prints nothing for the live title and description (`gh pr view 2950 --json title,body`) or for the commit messages since the merge base.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4633548092
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4633548092

### F29 - Rows verify specification text that the D1 rewrite removed

- PR number: 2950
- Source review ID: 5981273810
- Reviewer finding ID: F29
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4633520862
- Concern: After the rewrite of the specification as a decision record, the "Current-tree verification" lines of F2, F3, F5, F9, F12, F13, F14, F23 and F24, and the M4 part of F1's, describe text the specification no longer has, and the record did not say so.
- Solution: The Findings intro names those rows and says their verification lines were true when written and stay as written.
- Current-tree verification: `grep -n 'removed the text' docs/pr-reviews/pr-2950-review/PR-REVIEW.md` prints the new paragraph of the Findings intro and this line.
- Resolution reference: `docs(pr-reviews): [#2949] say which audit rows verify text the D1 rewrite removed`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4633630021

### F30 - Commit Points has no row for T6

- PR number: 2950
- Source review ID: 5981273810
- Reviewer finding ID: F30
- Source URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4633520877
- Concern: T6 writes `manual-verification-evidence.md` and updates the EPIC row, but Commit Points mapped it to no commit.
- Solution: Commit Points has a T6 row: the evidence file and the EPIC row, committed after the manual runs of M1 and M2.
- Current-tree verification: `grep -n '^| T6' docs/issues/open/2949-2003-keep-reviewer-ack-after-audit-only-commits/ISSUE.md` prints two rows, one in Implementation Plan and one in Commit Points.
- Resolution reference: `docs(issues): [#2949] add the T6 row to Commit Points`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2950#discussion_r4633630432

## Processing Log

- 2026-11-18 18:34 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - Started audit. The poller reported da2ce7's conversation comment [6987829006](https://github.com/torrust/torrust-tracker/pull/2950#issuecomment-6987829006) (2026-11-18 18:31:31 UTC) on head `docs(issues): add issue specification for #2949`; the pull request had no review. The comment does not agree to the rule as written, points to their proposal on #2949 ([issue comment 6987828129](https://github.com/torrust/torrust-tracker/issues/2949#issuecomment-6987828129), 18:31:29 UTC: a metadata branch folded in with a three-parent merge), and asks that the two be compared before deciding. A conversation comment is not a review, so it makes no row. The comparison was added to the specification in `docs(issues): [#2949] compare da2ce7's metadata-branch alternative under D1`; no decision was made, the maintainer decides D1.
- 2026-11-18 19:13 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - Round 1: the poller reported Copilot review 5963231037 (submitted 2026-11-18 19:06:19 UTC). Normalized its two inline comments as F1 and F2, fixed each in its own commit, pushed both, and replied on both threads, then resolved both threads.
- 2026-11-18 19:48 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - Round 2: the poller reported Copilot review 5963780219 (submitted 2026-11-18 19:41:53 UTC). Normalized its inline comment as F3, fixed it in its own commit, pushed it, replied on the thread, then resolved the thread.
- 2026-11-18 20:26 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - Round 3: the poller reported Copilot review 5964359494 (submitted 2026-11-18 20:19:07 UTC). Normalized its two inline comments as F4 and F5, fixed each in its own commit, pushed both, replied on both threads, then resolved both threads.
- 2026-11-19 07:48 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - Round 4: the poller reported da2ce7's review 5970994174 (submitted 2026-11-19 07:32:57 UTC, `CHANGES_REQUESTED`). Normalized its seven inline comments as F6 to F12, fixed each in its own commit (F11 in the description, which has no commit), pushed the fixes, replied on all seven threads, then resolved them. Copilot review 5964972845 (2026-11-18 20:55:36 UTC) on the round-3 head was first read as clean; its body raised the concerns of F8 and F12 in prose, which da2ce7's review pointed out. F9 corrects the premise of F5's Concern and of the F5 fix commit's body, which stay as history.
- 2026-11-19 08:26 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - Round 5: the poller reported da2ce7's review 5971317192 (`APPROVED`, submitted 2026-11-19 08:03:58 UTC) and their `ACK` on the round-4 head (08:04:02 UTC); the maintainer posted the same `ACK` at 08:09:20 UTC. Normalized the review's two inline comments as F15 and F16 and fixed each in its own commit; the push at 08:11:49 UTC moved the head, so both `ACK`s became stale and GitHub dismissed the approval. Replied on both threads (08:11:56 and 08:11:59 UTC) and resolved them. The same resolve loop also resolved the two threads of Copilot review 5971362548 (submitted 08:07:51 UTC) before any reply; they were unresolved again at once. That push was the fifth round of fixes, so the maintainer was asked before a sixth and approved it in the working session. F21 was fixed in its own commit; F20 was answered with no change (the reply was edited at 08:20:39 UTC to open with the prescribed `Superseded by` form); both threads were replied to (08:19:28 and 08:19:31 UTC) and then resolved. The fixes for F21 and da2ce7's later F19 were pushed together. This entry and the rows for F15, F16, F20 and F21 address da2ce7's F17 and F18 in review 5971496221.
- 2026-11-19 08:33 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - Round 6: the poller reported da2ce7's review 5971496221 (`COMMENTED`, submitted 2026-11-19 08:19:03 UTC). Normalized its three inline comments as F17 to F19. F19 was fixed in its own commit and pushed with F21's fix; F17 and F18 were fixed by the round-5 record, pushed before the replies. Replied on all three threads (08:33:20 to 08:33:25 UTC) and resolved them.
- 2026-11-19 13:54 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - Round 7: the poller reported da2ce7's review 5971904090 (`APPROVED`, submitted 2026-11-19 08:50:35 UTC) and their `ACK` on the round-6 head (08:50:39 UTC); the maintainer posted the same `ACK` at 08:56:15 UTC. The maintainer chose to leave da2ce7's Nit F22 for the next commit, and a reply deferring it was posted at 08:56:52 UTC with the thread left open. Copilot review 5972058838 (submitted 09:02:26 UTC) then raised two Minor comments; the maintainer pointed to them in the working session. Fixed F23, F24 and the deferred F22 in their own commits, pushed them together (head `docs(issues): [#2949] correct the 08:20 log entries about what said "proposed"`), which made both `ACK`s stale; replied on the three threads (13:53:38 to 13:53:43 UTC) and resolved them.
- 2026-11-19 18:40 UTC - AI assistant (Claude Code, Claude Fable 5.2, issue #2949 session) - Round 8: da2ce7's review 5976843973 (`APPROVED`, submitted 2026-11-19 14:25:34 UTC) and their `ACK` on the round-7 head (14:25:39 UTC) were read in the working session. Normalized its two inline comments as F26 and F27. The maintainer approved an eighth round and decided D1 in the working session: the strict ACK stays. Rewrote the specification as the decision record and updated EPIC row 24, each in its own commit, and pushed them (head `docs(issues): [#2949] update EPIC row 24 for the strict-ACK decision`), which made da2ce7's `ACK` stale. Updated the titles and descriptions of the pull request and of issue #2949, replied on both threads (18:39:16 and 18:39:19 UTC), posted the comment that tells da2ce7 the decision (issue comment 7018461081, text approved by the maintainer) and resolved both threads.
- 2026-11-19 19:02 UTC - AI assistant (Claude Code, Claude Fable 5.2, issue #2949 session) - Round 9: the poller reported da2ce7's review 5981273810 (`CHANGES_REQUESTED`, submitted 2026-11-19 18:52:27 UTC) on the round-8 head. Normalized its three inline comments as F28 to F30. F28 was fixed at once by editing the description, with no push, and answered at 18:54:50 UTC; that reply said the thread would stay open until this record was pushed, but the thread was resolved with the other two before this entry, so that the row states what had happened. The maintainer posted `ACK` on the round-8 head at 18:56:06 UTC and approved a ninth round in the working session. Fixed F30 and F29 in their own commits and pushed them (head `docs(pr-reviews): [#2949] say which audit rows verify text the D1 rewrite removed`), which made that `ACK` stale; replied on the two threads (19:01:55 and 19:01:58 UTC) and resolved all three.

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
