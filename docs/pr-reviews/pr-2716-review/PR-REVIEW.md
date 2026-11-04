---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2712-2003-wait-for-pr-review-pipe-wait-after-exit/ISSUE.md
    - docs/issues/open/2713-2003-wait-for-pr-review-timeout-overflow-panic/ISSUE.md
    - docs/issues/open/2714-2003-wait-for-pr-review-head-race-between-requests/ISSUE.md
    - docs/issues/open/2715-cli-output-contract-scope-and-exit-code-policy/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2716 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2716>.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2716-f1 | Copilot | Minor (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2716-f2 | Copilot | Minor (inferred) | metadata | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |
| F3 | review-finding:pr-2716-f3 | Copilot | Minor (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2716-f4 | Copilot | Minor (inferred) | metadata | RE_RAISE_OF:F3 | NO_ACTION | SUPERSEDED |
| F5 | review-finding:pr-2716-f5 | Copilot | Minor (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2716-f6 | Copilot | Minor (inferred) | metadata | RE_RAISE_OF:F5 | NO_ACTION | SUPERSEDED |
| F7 | review-finding:pr-2716-f7 | Copilot | Minor (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2716-f8 | Copilot | Minor (inferred) | metadata | RE_RAISE_OF:F7 | NO_ACTION | SUPERSEDED |
| F9 | review-finding:pr-2716-f9 | Copilot | Minor (inferred) | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2716-f10 | Copilot | Minor (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2716-f11 | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2716-f12 | Copilot | Nit (inferred) | maintainability | RE_RAISE_OF:F11 | NO_ACTION | SUPERSEDED |
| F13 | review-finding:pr-2716-f13 | Copilot | Nit (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - The frontmatter timestamp of the #2712 specification is out of sync with its progress log

- PR number: 2716
- Source review ID: 5768334587
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4467528271>
- Concern: `last-updated-utc` in the #2712 specification does not match the latest progress-log entry.
- Solution: On the reviewed head the two values matched (12:12), so the report was only partly accurate. It exposed a real related defect: the drafting entry was stamped 12:00, before the reproduction commands ran (11:49 to 11:55 UTC). The earlier entries are kept, a correction entry is appended, and `last-updated-utc` holds the time of the edit (13:07 UTC).
- Current-tree verification: The frontmatter validator accepted the changed files and the pre-commit checks passed (`./contrib/dev-tools/git/hooks/pre-commit.sh`).
- Resolution reference: docs(issues): correct forward-dated stamps in the 2712-2715 specs
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4467918351>

### F2 - The #2712 progress-log entry stamp

- PR number: 2716
- Source review ID: 5768334587
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4467528320>
- Concern: The same stamp concern as F1, raised on the progress-log line of the #2712 specification.
- Solution: Superseded by F1; the same commit fixes both.
- Current-tree verification: Covered by F1.
- Resolution reference: docs(issues): correct forward-dated stamps in the 2712-2715 specs
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4467918916>

### F3 - The frontmatter timestamp of the #2713 specification is out of sync with its progress log

- PR number: 2716
- Source review ID: 5768334587
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4467528379>
- Concern: `last-updated-utc` in the #2713 specification does not match the latest progress-log entry.
- Solution: Same as F1: the values matched on the reviewed head, the drafting entry was stamped 12:10 before the commands ran, and the correction entry plus the new `last-updated-utc` (13:07 UTC) fix it.
- Current-tree verification: The frontmatter validator accepted the changed files and the pre-commit checks passed.
- Resolution reference: docs(issues): correct forward-dated stamps in the 2712-2715 specs
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4467919469>

### F4 - The #2713 progress-log entry stamp

- PR number: 2716
- Source review ID: 5768334587
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4467528410>
- Concern: The same stamp concern as F3, raised on the progress-log line of the #2713 specification.
- Solution: Superseded by F3; the same commit fixes both.
- Current-tree verification: Covered by F3.
- Resolution reference: docs(issues): correct forward-dated stamps in the 2712-2715 specs
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4467920139>

### F5 - The frontmatter timestamp of the #2714 specification is older than its latest log entry

- PR number: 2716
- Source review ID: 5768334587
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4467528468>
- Concern: `last-updated-utc` is older than the 12:20 progress-log entry in the #2714 specification.
- Solution: Valid. The log listed 12:20 before 12:12. A correction entry records that the commands ran between 11:49 and 11:56 UTC, and `last-updated-utc` holds the time of the edit (13:07 UTC).
- Current-tree verification: The frontmatter validator accepted the changed files and the pre-commit checks passed.
- Resolution reference: docs(issues): correct forward-dated stamps in the 2712-2715 specs
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4467920610>

### F6 - The #2714 progress-log entry order

- PR number: 2716
- Source review ID: 5768334587
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4467528512>
- Concern: The same concern as F5, raised on the progress-log line of the #2714 specification.
- Solution: Superseded by F5; the same commit fixes both.
- Current-tree verification: Covered by F5.
- Resolution reference: docs(issues): correct forward-dated stamps in the 2712-2715 specs
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4467921067>

### F7 - The frontmatter timestamp of the #2715 specification is older than its latest log entry

- PR number: 2716
- Source review ID: 5768334587
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4467528562>
- Concern: `last-updated-utc` is older than the 12:30 progress-log entry in the #2715 specification.
- Solution: Valid. The log listed 12:30 before 12:12. A correction entry records that the drafting finished by 11:56 UTC, and `last-updated-utc` holds the time of the edit. The F9 commit later moved it to 13:09 UTC with its own log entry.
- Current-tree verification: The frontmatter validator accepted the changed files and the pre-commit checks passed after each commit.
- Resolution reference: docs(issues): correct forward-dated stamps in the 2712-2715 specs
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4467921592>

### F8 - The #2715 progress-log entry order

- PR number: 2716
- Source review ID: 5768334587
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4467528602>
- Concern: The same concern as F7, raised on the progress-log line of the #2715 specification.
- Solution: Superseded by F7; the same commit fixes both.
- Current-tree verification: Covered by F7.
- Resolution reference: docs(issues): correct forward-dated stamps in the 2712-2715 specs
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4467922173>

### F9 - The #2715 Evidence column cites a file this pull request does not add

- PR number: 2716
- Source review ID: 5768334587
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4467528634>
- Concern: The manual scenarios M1 and M2 of #2715 cite sections of `manual-verification-evidence.md`, which does not exist in the pull request.
- Solution: Valid. The Evidence column now reads `TBD, written when the scenario runs`, because the file is written when the scenarios run. A progress-log entry records the change.
- Current-tree verification: Inspected the table after the edit; the frontmatter validator and the pre-commit checks passed.
- Resolution reference: docs(issues): mark the 2715 evidence file as to be written
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4467922737>

### F10 - The #2715 progress log is not in chronological order

- PR number: 2716
- Source review ID: 5769278008
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4468206945>
- Concern: The 12:30 drafting entry appears before the 12:12 approval entry; reorder the entries or mark the out-of-order ones.
- Solution: The file order matches the order of the events and only the drafting stamp is wrong, so reordering by stamp would be wrong. The drafting entry is marked `OUT-OF-ORDER` and a new log entry records the marking. The same marking was applied to the #2714 specification, which has the same defect.
- Current-tree verification: The frontmatter validator accepted the changed files and the pre-commit checks passed.
- Resolution reference: docs(issues): mark the out-of-order drafting entries in the 2714 and 2715 specs
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4468265278>

### F11 - HEAD_B names a commit the pull request head never reaches

- PR number: 2716
- Source review ID: 5769278008
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4468207047>
- Concern: In the #2714 reproduction script `HEAD_B` is the reviewed commit id, but the script keeps the pull request head at A for every read, so the name is misleading.
- Solution: Valid. The variable is renamed to `REVIEW_COMMIT_B` at its definition and its use.
- Current-tree verification: `bash -n` on the script passed, and the pre-commit checks passed.
- Resolution reference: docs(issues): rename HEAD_B in the 2714 head-race reproduction script
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4468266052>

### F12 - The HEAD_B name at its use in the script

- PR number: 2716
- Source review ID: 5769278008
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4468207107>
- Concern: The same concern as F11, raised on the line that uses `HEAD_B`.
- Solution: Superseded by F11; the same commit fixes both.
- Current-tree verification: Covered by F11.
- Resolution reference: docs(issues): rename HEAD_B in the 2714 head-race reproduction script
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4468266689>

### F13 - The Evidence column of the #2715 table has trailing whitespace

- PR number: 2716
- Source review ID: 5769278008
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4468207157>
- Concern: The Evidence column values include trailing whitespace; trim it.
- Solution: No change. The row ends at the closing `|` with no trailing whitespace on the line; the spaces before it pad the cell to the column width, as in the other tables of the specification, and trimming them would break the alignment.
- Current-tree verification: Inspected the line with `cat -A`: it ends with `|$`. The markdown linters in `linter all` passed in the pre-commit checks.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4468267312>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2716#discussion_r4468267312>

## Processing Log

- 2026-11-04 13:05 UTC - Started audit: fetched the nine unresolved threads of Copilot's review 5768334587 (submitted 12:34:59 UTC) on the head at that time.
- 2026-11-04 13:09 UTC - Fixed in two signed commits (commit times from `git log --format=%cI`): the stamp correction at 13:07 and the F9 evidence column at 13:09 UTC.
- 2026-11-04 13:12 UTC - After the push, each thread was replied to and resolved with `reply-and-resolve-thread.sh`; the GitHub times of the nine replies run from 13:12:13 to 13:12:39 UTC.
- 2026-11-04 13:13 UTC - A GraphQL fetch at 13:13:05 UTC, after the last reply, reported no unresolved thread.
- 2026-11-04 13:22 UTC - Copilot was re-requested at 13:22:55 UTC on the head at that time (after everything was pushed).
- 2026-11-04 13:40 UTC - The `wait-for-pr-review` poller returned Copilot's second review 5769278008, submitted at 13:39:26 UTC (kind `review`, state `COMMENTED`); four unresolved threads, F10 to F13, were recorded.
- 2026-11-04 13:42 UTC - F10 and F11 were fixed in two signed commits (commit times from `git log --format=%cI`): F10 at 13:41 and F11 at 13:42 UTC. F12 is a duplicate of F11 and F13 needed no change.
- 2026-11-04 13:44 UTC - After the push, each thread was replied to and resolved with `reply-and-resolve-thread.sh`; the GitHub times of the four replies are 13:44:37, 13:44:42, 13:44:45 and 13:44:48 UTC.
- 2026-11-04 13:44 UTC - A GraphQL fetch at 13:44:59 UTC, after the last reply, reported no unresolved thread.

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
