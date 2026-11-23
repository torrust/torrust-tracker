---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3009 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3009.

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

Round 1 is two reviews of the head whose last commit is
`docs(issues): [#2902] record the S2 pull request`: the review `6025339962` of the automated
reviewer Greptile (2026-11-23 16:30:18 UTC, comment; one inline comment without an ID, recorded as F9
with the severity of its `P2` badge) and da2ce7's review `6025674705` (2026-11-23 16:51:51 UTC,
approval; inline findings F1 to F8 with the reviewer's IDs). F6 asks for the reply to the Greptile
thread and for this record. Copilot's review `6025721446` (2026-11-23 16:54:40 UTC, comment) is of the
same head and arrived while the fixes were being made; its three inline comments carry the IDs R1,
R2 and F3 (which collides with da2ce7's F3) and are recorded as F10, F11 and F12.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3009-f1 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3009-f2 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3009-f3 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3009-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3009-f5 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3009-f6 | Human | Minor | other | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3009-f7 | Human | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3009-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3009-f9 | Unknown | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-3009-f10 | Copilot | Minor | testing | ORIGINAL | NO_ACTION | SUPERSEDED |
| F11 | review-finding:pr-3009-f11 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-3009-f12 | Copilot | Nit | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The shared-database test checked one store after the drop

- PR number: 3009
- Source review ID: 6025674705
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4668977631
- Concern: After dropping the tables through the schema migrator the test checked only the whitelist store, so a factory that gave the key store or the metrics store its own migrated database would pass.
- Solution: After the drop the test also asserts that `load_keys` and `load_global_downloads` fail; four mutants that give one store its own migrated database all fail it.
- Current-tree verification: `grep -c 'must share one database' packages/tracker-core/src/databases/setup.rs` prints 3.
- Resolution reference: test(tracker-core): [#2902] check that all four stores lose their tables when the migrator drops them
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4669095496

### F2 - The spec front matter named the S1 branch

- PR number: 3009
- Source review ID: 6025674705
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4668977645
- Concern: `related-pr` names this pull request but `branch` still named the S1 branch.
- Solution: `branch` is `2902-1347-add-tests-tracker-core-sqlite`.
- Current-tree verification: `grep -n '^branch:' docs/issues/open/2902-1347-add-tests-tracker-core/ISSUE.md` shows the S2 branch.
- Resolution reference: docs(issues): [#2902] name the S2 branch in the spec front matter
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4669096014

### F3 - The slice table showed S1 in progress

- PR number: 3009
- Source review ID: 6025674705
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4668977659
- Concern: The S1 row still said `IN_PROGRESS` although S1 was merged and this change logs it.
- Solution: The S1 row is `DONE`.
- Current-tree verification: `grep -n '^| S1 ' docs/issues/open/2902-1347-add-tests-tracker-core/ISSUE.md | grep -c DONE` prints 1.
- Resolution reference: docs(issues): [#2902] mark slice S1 done in the slice table
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4669096483

### F4 - Line numbers taken before the rebase

- PR number: 3009
- Source review ID: 6025674705
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4668977666
- Concern: The Results of four plans quoted lines as they were before the rebase onto #2916, which moved them.
- Solution: They give the line numbers at the S2 head after the rebase and say so.
- Current-tree verification: `grep -c 'at the S2 head' docs/issues/open/2902-1347-add-tests-tracker-core/test-refactor-plans/databases-error-tests.md` prints 1.
- Resolution reference: docs(issues): [#2902] refresh the line numbers of the plans after the rebase on develop
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4669096949

### F5 - The description said the gate ran on every commit

- PR number: 3009
- Source review ID: 6025674705
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4668977683
- Concern: After the rebase the commits that build the row 15 table with the removed `location` fields do not compile their test target until the adapting commit, so the validation line overstated.
- Solution: The PR description says the gate ran as each commit was made, before the rebase, names the commits that do not compile by subject, and says the history was not rewritten.
- Current-tree verification: `gh pr view 3009 --json body --jq .body | grep -c 'history was not rewritten'` prints 1.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4669097345
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4669097345

### F6 - The automated review thread had no reply and the head had no audit record

- PR number: 3009
- Source review ID: 6025674705
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4668977699
- Concern: The Greptile thread on `setup.rs` had no reply and the pull request had no review audit record.
- Solution: The thread has a reply (F9), and this record has a row for every finding of the round, the automated one included.
- Current-tree verification: `docs/pr-reviews/pr-3009-review/PR-REVIEW.md` exists and its checker passes.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4669099455
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4669097830

### F7 - `tempfile::TempDir` written in full

- PR number: 3009
- Source review ID: 6025674705
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4668977722
- Concern: The helper wrote `tempfile::TempDir` where the sibling test modules import it.
- Solution: `use tempfile::{TempDir, tempdir};` in the test module.
- Current-tree verification: `grep -c 'tempfile::tempdir' packages/tracker-core/src/databases/setup.rs` prints 0.
- Resolution reference: test(tracker-core): [#2902] import the temporary directory names in the database setup tests
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4669098387

### F8 - The plan row promised a message naming the key

- PR number: 3009
- Source review ID: 6025674705
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4668977743
- Concern: The plan said the bad-key cases assert the driver and a message naming the key; the test asserts only the driver.
- Solution: The clause is dropped; the row says the cases carry the driver they were given.
- Current-tree verification: `grep -c 'message naming the key' docs/issues/open/2902-1347-add-tests-tracker-core/test-refactor-plans/sqlite-auth-key-store-tests.md` prints 0.
- Resolution reference: docs(issues): [#2902] say what the bad-key cases of the shared parsing test assert
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4669098863

### F9 - Imports of the new test helpers (Greptile)

- PR number: 3009
- Source review ID: 6025339962
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4668705638
- Concern: The helper uses `tempfile::TempDir` and `tempfile::tempdir()` without importing them, and `::sqlx::query` without a naming conflict; the repository guide asks for short imported names.
- Solution: The `tempfile` names are imported (same change as F7). The `::sqlx::query` half is declined: every query of the driver files is written with the leading `::sqlx::query` path, and the new code follows that style.
- Current-tree verification: `grep -c 'use tempfile::' packages/tracker-core/src/databases/setup.rs` prints 1.
- Resolution reference: test(tracker-core): [#2902] import the temporary directory names in the database setup tests
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4669099455

### F10 - The post-drop assertion covered one store (Copilot)

- PR number: 3009
- Source review ID: 6025721446
- Reviewer finding ID: R1
- Source URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4669011501
- Concern: The same as F1: after the drop only the whitelist store was checked, so a key or metrics store on another migrated database would pass.
- Solution: Superseded by F1: the test also asserts that `load_keys` and `load_global_downloads` fail after the drop.
- Current-tree verification: `grep -c 'must share one database' packages/tracker-core/src/databases/setup.rs` prints 3.
- Resolution reference: test(tracker-core): [#2902] check that all four stores lose their tables when the migrator drops them
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4669260082

### F11 - The plans omitted the Prose-First AAA Review (Copilot)

- PR number: 3009
- Source review ID: 6025721446
- Reviewer finding ID: R2
- Source URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4669011565
- Concern: The eight new test-producing plans omit the Prose-First AAA Review rows that the shared guardrails require before the completed-file review.
- Solution: Each of the eight plans has the section, with the prose for every test, the comparison with the code and the smells found; it says the prose was written after the fact. The S1 plans that #2988 merged have the same gap, raised with the maintainer.
- Current-tree verification: `grep -l 'Prose-First AAA Review' docs/issues/open/2902-1347-add-tests-tracker-core/test-refactor-plans/*.md | wc -l` prints 10 (the eight plans, the repository plan of S1 and the README that states the rule).
- Resolution reference: docs(issues): [#2902] record the Prose-First AAA review of the eight S2 test plans
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4669260697

### F12 - The panic tests lacked the it_should prefix (Copilot)

- PR number: 3009
- Source review ID: 6025721446
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4669011629
- Concern: The two tests of the legacy factory panics were named `the_legacy_factory_should_*` where the plans and the `write-unit-test` skill ask for `it_should_*`.
- Solution: They are `it_should_panic_when_the_legacy_factory_gets_a_configuration_without_a_database` and `it_should_panic_when_the_legacy_factory_cannot_open_the_database`.
- Current-tree verification: `grep -rc 'the_legacy_factory_should' packages/tracker-core/src/databases/setup.rs` prints 0.
- Resolution reference: test(tracker-core): [#2902] name the legacy factory panic tests with the it_should prefix
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3009#discussion_r4669261412

## Processing Log

- 2026-11-23 17:01 UTC - Read both round 1 reviews. The `OS Compatibility` check of the reviewed head failed only because its `macos-latest (nightly)` build was cancelled (the other five builds pass), not because of a test; re-running it is the maintainer's. Fixed F1, F7 (with F9), F2, F3, F4 and F8 in their own commits and F5 in the PR description; replied on all nine threads at 17:01:31 to 17:01:51 UTC, before pushing the fixes and this record together; the threads are resolved after the push.
- 2026-11-23 17:14 UTC - Copilot's review of the same head (F10 to F12) was read after that push: F10 duplicates F1 and is superseded by it; F11 and F12 fixed in their own commits; replied at 17:14:51 to 17:14:56 UTC, before pushing the fixes and this update together; the threads are resolved after the push.

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
