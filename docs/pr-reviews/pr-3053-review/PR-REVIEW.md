---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3053 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3053.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`
- Reviewer: the author's GitHub login
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`, `documentation`, `maintainability`, `security`, `other`

## Findings

F1 is from CodeRabbit's inline comment. F2 to F10 are from da2ce7's review 6059881188: F2 to F8 are its
inline comments and F9 and F10 its review-body findings. F11 is the Minor in the body of da2ce7's approval 6060214481, and F12 to F14 are the Nits in the body of da2ce7's approval 6060859817. The reviews of `claude[bot]` (6059427646, 6059672293)
and the summary body of CodeRabbit's review 6059546450 contain no actionable assertion and have no rows.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3053-f1 | coderabbitai[bot] | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3053-f2 | da2ce7 | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3053-f3 | da2ce7 | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3053-f4 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3053-f5 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3053-f6 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3053-f7 | da2ce7 | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3053-f8 | da2ce7 | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3053-f9 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F10 | review-finding:pr-3053-f10 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F11 | review-finding:pr-3053-f11 | da2ce7 | Minor | testing | ORIGINAL | FIXED | NON_RESOLVABLE |
| F12 | review-finding:pr-3053-f12 | da2ce7 | Nit | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |
| F13 | review-finding:pr-3053-f13 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F14 | review-finding:pr-3053-f14 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - The reproduction test counts failures and never asserts; clarify which test T3 uses

- PR number: 3053
- Source review ID: 6059546450
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3053#discussion_r4696389790
- Concern: The evidence file's reproduction test only prints failure counts, so it passes even when every call fails; if T3 reused it, its red run would pass. The steps also did not say which version T3 uses.
- Solution: The evidence file and ISSUE.md now say the temporary test only measures and that T3 is a separate asserting test (both concurrent adds Ok and the hash whitelisted, both concurrent removes Ok and the hash gone, every round), which supplies the red evidence.
- Current-tree verification: Read both files after the edit: step 3 of the performed actions and the Disposable Verification Scripts note state it.
- Resolution reference: docs(issues): [#3052] clarify the reproduction test is not the regression test
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3053#discussion_r4696444158

### F2 - `branch:` names the implementation branch, not the spec-only branch

- PR number: 3053
- Source review ID: 6059881188
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3053#discussion_r4696652186
- Concern: ISSUE.md `branch:` is `3052-whitelist-add-remove-race` while the PR head branch is `3052-whitelist-add-remove-race-spec`.
- Solution: Set `branch:` to the `-spec` name; it changes back to the base name when implementation starts.
- Current-tree verification: `grep -n '^branch:' ISSUE.md` prints the `-spec` value.
- Resolution reference: docs(issues): [#3052] name the spec-only branch in the frontmatter
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3053#discussion_r4696783774

### F3 - AC1 cites the MySQL and PostgreSQL suites but no planned test there runs concurrent add

- PR number: 3053
- Source review ID: 6059881188
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3053#discussion_r4696652206
- Concern: The shared harness cases are sequential, so on MySQL and PostgreSQL concurrent add is exercised only by the temporary V2.
- Solution: Planned maintained concurrent store-level cases in the shared harness (T5), and made AC1, AC2 and the Regression Test Strategy cite them.
- Current-tree verification: `grep -n 'concurrent' ISSUE.md` shows the strategy, T5 and AC1/AC2 consistent.
- Resolution reference: docs(issues): [#3052] plan maintained concurrent cases for MySQL and PostgreSQL
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3053#discussion_r4696784460

### F4 - Template parts missing: test-development loop and completion-review conditions

- PR number: 3053
- Source review ID: 6059881188
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3053#discussion_r4696652221
- Concern: The Commit Points paragraph, the completion-review conditions and the agent-review-reports checkpoint were dropped from the spec.
- Solution: Restored them from docs/templates/ISSUE.md.
- Current-tree verification: Compared the restored text with the template; `linter all` exits 0.
- Resolution reference: docs(issues): [#3052] restore the test-development loop and completion-review parts
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3053#discussion_r4696785341

### F5 - The quoted store error drops `driver: Sqlite3`

- PR number: 3053
- Source review ID: 6059881188
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3053#discussion_r4696652235
- Concern: ISSUE.md line 45 quoted the error without its `driver` field.
- Solution: Quoted it whole.
- Current-tree verification: Line 45 of ISSUE.md ends with `driver: Sqlite3 }`.
- Resolution reference: docs(issues): [#3052] quote the recorded store error whole
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3053#discussion_r4696785835

### F6 - The test result line is shortened, not as printed

- PR number: 3053
- Source review ID: 6059881188
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3053#discussion_r4696652254
- Concern: The evidence file recorded a shortened `test ... repro_concurrent_add_and_remove ... ok` line.
- Solution: Recorded the full line as the harness printed it.
- Current-tree verification: The evidence file contains the full `test whitelist::repository::persisted::tests::...` line.
- Resolution reference: docs(issues): [#3052] record the reproduction test line as printed
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3053#discussion_r4696786343

### F7 - `DONE (before fix)` is not a template status value

- PR number: 3053
- Source review ID: 6059881188
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3053#discussion_r4696652264
- Concern: The V1 status cell used a value outside TODO/IN_PROGRESS/DONE/FAILED/BLOCKED.
- Solution: Set it to `IN_PROGRESS` and named the before-fix result in the Evidence cell.
- Current-tree verification: The V1 row of ISSUE.md shows `IN_PROGRESS`.
- Resolution reference: docs(issues): [#3052] use a template status value for V1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3053#discussion_r4696786957

### F8 - `last-updated-utc` of the evidence file lags ISSUE.md

- PR number: 3053
- Source review ID: 6059881188
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3053#discussion_r4696652275
- Concern: The evidence file kept the first version's timestamp after an edit.
- Solution: Set the same `last-updated-utc` in both files.
- Current-tree verification: `grep -n '^last-updated-utc'` of both files prints the same value.
- Resolution reference: docs(issues): [#3052] move the last-updated-utc of both spec files
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3053#discussion_r4696787651

### F9 - The PR description lacks the "Validation performed" part

- PR number: 3053
- Source review ID: 6059881188
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3053#pullrequestreview-6059881188
- Concern: `open-pull-request` requires a "Validation performed" part in the description, and it was missing.
- Solution: Edited the description to add "Files touched" and "Validation performed" sections.
- Current-tree verification: Re-read the description after the edit with `gh pr view 3053`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3053#issuecomment-7166150355
- Follow-up PR URL: N/A
- Reply URL: N/A

### F10 - The audit record was missing before the first reply, and that reply cites a branch commit id

- PR number: 3053
- Source review ID: 6059881188
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3053#pullrequestreview-6059881188
- Concern: `process-pr-review` requires the audit record before the first reply; the reply to the CodeRabbit thread was posted without it and cites the branch commit id instead of the commit subject.
- Solution: Created this record, with the late start logged below. The earlier reply stays as posted; this record maps it to the commit subject in F1.
- Current-tree verification: This file exists at the PR head after the audit commit.
- Resolution reference: docs(pr-reviews): [#3053] record the review audit
- Follow-up PR URL: N/A
- Reply URL: N/A

### F11 - The concurrent store-level cases come after the fix, so no red run is planned for them

- PR number: 3053
- Source review ID: 6060214481
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3053#pullrequestreview-6060214481
- Concern: T5 added the concurrent store-level harness cases after T4 made the drivers idempotent, while T3 and AC4 planned a red run only for the regression test, so nothing would show that the MySQL and PostgreSQL cases detect the race.
- Solution: T3 now adds the concurrent harness cases and observes them red on the unfixed code (SQLite, and MySQL and PostgreSQL via the Docker suites); T5 only replaces the three failure-documenting tests; AC4 names the harness cases.
- Current-tree verification: `grep -n 'T3\|T5\|AC4' ISSUE.md` shows the three places consistent.
- Resolution reference: docs(issues): [#3052] plan the concurrent harness cases red before the fix
- Follow-up PR URL: N/A
- Reply URL: N/A

### F12 - `ISSUE.md` keeps an old `last-updated-utc` after a change

- PR number: 3053
- Source review ID: 6060859817
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3053#pullrequestreview-6060859817
- Concern: `987e8257c` changed ISSUE.md but left `last-updated-utc` at 14:57.
- Solution: Moved `last-updated-utc` after the T3 change, and in the next ISSUE.md edit as well.
- Current-tree verification: `sed -n 12p ISSUE.md` shows the new value, not earlier than the last commit that changed the file.
- Resolution reference: docs(issues): [#3052] move the last-updated-utc after the T3 change
- Follow-up PR URL: N/A
- Reply URL: N/A

### F13 - T3's single commit point covers two test files

- PR number: 3053
- Source review ID: 6060859817
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3053#pullrequestreview-6060859817
- Concern: T3 covers `persisted.rs` and the shared harness, while the Commit Points paragraph commits each reviewed test-design increment before the next planned file.
- Solution: Split the T3 commit point in two rows, one per file, and noted the per-file commit.
- Current-tree verification: The Commit Points table shows two T3 rows.
- Resolution reference: docs(issues): [#3052] commit the T3 tests per file
- Follow-up PR URL: N/A
- Reply URL: N/A

### F14 - The audit commits cite the PR, not the issue, in their subjects

- PR number: 3053
- Source review ID: 6060859817
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3053#pullrequestreview-6060859817
- Concern: The two audit commits cite `[#3053]`, where the repository's `docs(pr-reviews)` subjects cite the issue; for later audit commits only.
- Solution: Cite the issue in this and later audit commits; the earlier two are not rewritten, as the reviewer asked.
- Current-tree verification: `git log --format=%s` shows the new audit commit citing `[#3052]`.
- Resolution reference: docs(pr-reviews): [#3052] record the third review round
- Follow-up PR URL: N/A
- Reply URL: N/A

## Processing Log

- 2026-11-25 14:30 UTC - CodeRabbit's inline comment (F1) arrived.
- 2026-11-25 14:34 UTC - Replied to F1 and resolved its thread, before this record existed.
- 2026-11-25 14:50 UTC - da2ce7's review 6059881188 arrived (F2 to F10).
- 2026-11-25 15:00 UTC - Replied to F2 to F8 after pushing their fixes, and edited the PR description for F9 (comment 7166150355).
- 2026-11-25 15:01 UTC - Created this record from the template; the late start is the subject of F10.
- 2026-11-25 15:10 UTC - da2ce7's approval 6060214481 arrived with one Minor (F11); the maintainer's ACK of the previous head followed.
- 2026-11-25 15:12 UTC - Committed the F11 fix (`docs(issues): [#3052] plan the concurrent harness cases red before the fix`) and held the push until the maintainer chose between pushing it and merging as is.
- 2026-11-25 15:58 UTC - da2ce7's approval 6060859817 arrived with three Nits (F12 to F14).
- 2026-11-25 16:03 UTC - Committed the F12 and F13 fixes; recorded F12 to F14 in this audit.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
