---
pr-number: 2926
pr-url: https://github.com/torrust/torrust-tracker/pull/2926
last-updated-utc: "2026-11-17 21:39"
---

<!-- skill-link: process-pr-review -->

# PR #2926 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2926.

<!-- Copied verbatim into each audit record after placeholder replacement. -->

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

<!-- Copied verbatim into each audit record. -->

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

Round 1: Copilot review 5946669412 (COMMENTED) with three findings without reviewer IDs, as audit findings F1 to F3. Round 2: Copilot review 5946979677 (COMMENTED, at the round 1 fix head) with two findings without reviewer IDs, as audit findings F4 and F5.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2926-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2926-f2 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2926-f3 | Copilot | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2926-f4 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2926-f5 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Order 7 title in the EPIC table differs from the narrowed #2907 title

- PR number: 2926
- Source review ID: 5946669412
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2926#discussion_r4605825785
- Concern: The order 7 row of the EPIC table still says "Remove the temporary files tests leave behind", while the #2907 spec title is now "Remove the Temporary TOML and PEM Files Tests Leave Behind".
- Solution: The row now reads "#2907 - Remove the temporary TOML and PEM files tests leave behind", the title of GitHub issue #2907.
- Current-tree verification: `grep -n '#2907 - Remove the temporary TOML and PEM files tests leave behind' docs/issues/open/2392-test-isolation/EPIC.md` finds the order 7 row.
- Resolution reference: docs(issues): [#2392] align the order 7 title and widen #2925's SQLite leftover check
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2926#discussion_r4605878269

### F2 - #2925 checks only for `data_*.db` files

- PR number: 2926
- Source review ID: 5946669412
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2926#discussion_r4605825865
- Concern: The goal, acceptance criteria and verifications of #2925 target `data_*.db` files, but the guard of #2924 keeps the database as `data.db` inside a directory, so the check could miss a leftover guard directory.
- Solution: The goal, the scope item, T5, AC1 and V2 now exclude a leftover directory that holds a database file as well as `data_*.db` files; a progress-log entry records it. V1 still counts `data_*.db`, which is what the tests leave before the change, and AC5 keeps the check that the fresh `TMPDIR` is empty.
- Current-tree verification: `grep -n 'no directory that holds a database file\|SQLite database file or directory' docs/issues/open/2925-*/ISSUE.md` finds AC1 and T5.
- Resolution reference: docs(issues): [#2392] align the order 7 title and widen #2925's SQLite leftover check
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2926#discussion_r4605878716

### F3 - Function name written without `()`

- PR number: 2926
- Source review ID: 5946669412
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2926#discussion_r4605825963
- Concern: The leak-sites table of #2907 writes `ephemeral_sqlite_database` without `()`, unlike the other specs in this PR.
- Solution: The row now writes `ephemeral_sqlite_database()` and `ephemeral*()`.
- Current-tree verification: `grep -n 'configuration::ephemeral_sqlite_database()' docs/issues/open/2907-*/ISSUE.md` finds the name with `()`; the F4 fix moved it from the table row to the paragraph after the table.
- Resolution reference: docs(issues): [#2392] align the order 7 title and widen #2925's SQLite leftover check
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2926#discussion_r4605879199

### F4 - Out-of-scope SQLite row in the #2907 leak table

- PR number: 2926
- Source review ID: 5946979677
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2926#discussion_r4606079586
- Concern: The Background leak table of #2907 still lists the SQLite helper although the issue is narrowed and the SQLite work moved to #2924 and #2925, which makes the in-scope inventory ambiguous.
- Solution: The table now holds only the two in-scope helpers. A paragraph after it names `ephemeral_sqlite_database()` as the third leak the audit found, out of scope and moved to #2924 and #2925.
- Current-tree verification: `grep -n 'Two of them stay in this issue\|The third is out of scope here' docs/issues/open/2907-*/ISSUE.md` finds both sentences, and the table has two rows.
- Resolution reference: docs(issues): [#2392] keep #2907's leak table in scope and reword the #2924 guard goal
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2926#discussion_r4606129122

### F5 - Redundant wording in the #2924 goal

- PR number: 2926
- Source review ID: 5946979677
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2926#discussion_r4606079655
- Concern: The goal says the guard removes the directory "on drop, on success, failure and drop paths", which mentions drop twice and reads unclear.
- Solution: The goal now says the guard removes the directory when it is dropped, so the directory goes whether the test passes or fails, and keeps the EPIC rule 1 reference.
- Current-tree verification: `grep -n 'removes it when the guard is dropped' docs/issues/open/2924-*/ISSUE.md` finds the goal.
- Resolution reference: docs(issues): [#2392] keep #2907's leak table in scope and reword the #2924 guard goal
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2926#discussion_r4606129559

## Processing Log

- 2026-11-17 21:15 UTC - Round 1: Copilot review 5946669412 with F1 to F3. Committed and pushed the fix (`docs(issues): [#2392] align the order 7 title and widen #2925's SQLite leftover check`), then replied to the three threads (21:14:38Z to 21:14:43Z).
- 2026-11-17 21:15 UTC - The validator on the worktree record reported 3 rows, 1 log entry, 0 failures. Resolved the three threads (about 21:15:31Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread. Committed this record.
- 2026-11-17 21:39 UTC - Round 2: Copilot review 5946979677 (at the round 1 fix head) with F4 and F5. Committed and pushed the fix (`docs(issues): [#2392] keep #2907's leak table in scope and reword the #2924 guard goal`), then replied to the two threads (21:38:39Z and 21:38:42Z). Updated F3's verification, because the F4 fix moved the function name out of the table.
- 2026-11-17 21:39 UTC - The validator on the worktree record reported 5 rows, 3 log entries, 0 failures. Resolved the F4 and F5 threads (about 21:39:20Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread. Committed these entries.
<!-- Copied verbatim into each audit record. -->

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
