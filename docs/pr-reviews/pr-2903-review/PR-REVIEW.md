---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2903 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2903.

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

Round 1 is da2ce7's review `5938095803` of `690cfb5f5` (submitted as an approval, later dismissed by the
push), round 2 his review `5938377458` of `2b0c3fe36` (comment). Their inline findings keep the
reviewer's IDs F1 to F10; the review bodies only summarize them.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2903-f1 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2903-f2 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2903-f3 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2903-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2903-f5 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2903-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2903-f7 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2903-f8 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2903-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2903-f10 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The async, cleanup and fixture rules of `create-issue` are missing

- PR number: 2903
- Source review ID: 5938095803
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599332949
- Concern: The `create-issue` skill requires, for asynchronous I/O, resource cleanup or reusable fixtures, a responsibility and ownership map, normal and failure-path lifetime invariants, absolute deadlines and a design-review checkpoint after the first vertical slice; the spec has none, and its mitigation offers `ephemeral_sqlite_database`, which returns a bare `PathBuf` with no cleanup owner, the leak #2637 fixed in its S5.
- Solution: Added the section `Asynchronous Work, Cleanup and Test Fixtures` to `ISSUE.md`: an ownership map (listener tasks, channels, SQLite files, fakes, containers, the `events` drain), the normal and failure-path invariants, a 5-second absolute deadline for every wait on another task, and the design-review checkpoint T2b in the task table. New tests own a `tempfile::TempDir`. The existing leak is recorded as finding F5 with question Q7 (recommendation: fix the package's shared test helpers in S1), and the risk line no longer offers the helper as a mitigation.
- Current-tree verification: `grep -n 'Asynchronous Work, Cleanup and Test Fixtures\|T2b\|^| F5 |\|^| Q7 |' docs/issues/open/2902-1347-add-tests-tracker-core/ISSUE.md` shows the section, the task, the finding and the question; `packages/test-helpers/src/configuration.rs:85-89` returns `temp_directory.join(...)` with no guard.
- Resolution reference: docs(issues): [#2902] define the async, cleanup and fixture rules
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599795368

### F2 - The scope without the feature runs 155 unit tests, not 157

- PR number: 2903
- Source review ID: 5938095803
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599332966
- Concern: The two driver tests sit in modules gated on `db-compatibility-tests`, so a plain `--lib` run compiles 155 tests; `coverage-evidence.md:34` and `ISSUE.md:80` said 157.
- Solution: Both rows now say 155 and why.
- Current-tree verification: `grep -n '^running' .tmp/cov/log.txt` (the local log of the four baseline runs) prints `running 155 tests` for the `--lib` run without the feature; `grep -n '155 unit' docs/issues/open/2902-1347-add-tests-tracker-core/*.md` shows both rows.
- Resolution reference: docs(issues): [#2902] count 155 unit tests in the scope without the feature
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599796065

### F3 - Uncovered-line groups mixed test-module lines and production regions

- PR number: 2903
- Source review ID: 5938095803
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599332974
- Concern: Several groups classed test-module lines as production regions (`whitelist/authorization.rs:139`, `authentication/key/repository/persisted.rs:96`, `sqlite/mod.rs:103` and `:119`, `listener.rs:283-284`, `databases/error.rs:160` and `:173`) and one production region as a test assertion (`statistics/persisted/downloads.rs:133`).
- Solution: Each group now lists production lines first and test-module lines after a slash, with the class `test code` for the latter; ledger rows 6, 7, 11, 13, 28, 29, 37, 41 and 42 follow. The sweep of every group against the `#[cfg(test)]` line of its file found three more production regions the first table had called `no decision` or `one region`: `statistics/mod.rs:49` (the usage-statistics-disabled branch), `peer_key.rs:64` (the mixed-expiry `PartialEq` arm) and `torrent/manager.rs:66` (a failing-database `?`).
- Current-tree verification: Read each cited line at `5833cdae9`-equivalent source (`packages/` unchanged on this branch) and the first `#[cfg(test)]` line of each file (`grep -n 'cfg(test)\]'`), for example `authentication/handler.rs` tests start at 288, `databases/error.rs` at 150, `statistics/event/listener.rs` at 237.
- Resolution reference: docs(issues): [#2902] separate production and test-module uncovered lines
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599796661

### F4 - `parse_valid_until` line ranges were off

- PR number: 2903
- Source review ID: 5938095803
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599332988
- Concern: The function is at lines 119-125 in the MySQL and PostgreSQL copies and 122-128 in the SQLite copy, not 119-128 in all three.
- Solution: The F2 evidence cites both ranges.
- Current-tree verification: `grep -n 'fn parse_valid_until' -A6 packages/tracker-core/src/databases/driver/*/auth_key_store.rs` shows 119-125, 122-128 and 119-125.
- Resolution reference: docs(issues): [#2902] cite the exact parse_valid_until line ranges
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599797283

### F5 - The EPIC log entry followed the blank line that closed the list

- PR number: 2903
- Source review ID: 5938095803
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599332994
- Concern: Appending the 10:57 entry after the blank line before `## Acceptance Criteria` split the progress log into a loose list.
- Solution: The blank line is removed, so the entries form one tight list again.
- Current-tree verification: `git diff 690cfb5f5 -- docs/issues/open/1347-overhaul-packages-testing/EPIC.md` shows the removed blank line and no other change to the earlier entries.
- Resolution reference: docs(issues): [#1347] restore the 10:57 EPIC log entry and append T1b
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599797846

### F6 - The ranking rested on a git-ignored assessment

- PR number: 2903
- Source review ID: 5938095803
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599333003
- Concern: `ISSUE.md:65` and the EPIC row 14 note cited the Phase 3 assessment, whose only source is the git-ignored `.tmp/phase3-assessment.md`.
- Solution: The spec now states the ranking from the EPIC coverage table (the unit-only figures of the five packages that have one), the EPIC row says the same, and the reference to the git-ignored file is removed.
- Current-tree verification: `grep -rn -i 'phase3\|largest remaining' docs/issues/open/2902-1347-add-tests-tracker-core docs/issues/open/1347-overhaul-packages-testing/EPIC.md` prints nothing.
- Resolution reference: docs(issues): [#2902] state the coverage ranking from the EPIC table
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599798461

### F7 - A log entry said no review arrived after one had

- PR number: 2903
- Source review ID: 5938377458
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599550865
- Concern: The 12:16 UTC entry says no review of #2903 arrived within 60 minutes, eight minutes after a review with six findings was submitted (12:08:23 UTC), and the six threads had no reply.
- Solution: Appended a correction entry to the spec's progress log (the log is append-only): the poller started when the PR opened (11:05:26 UTC) timed out with no review, the first review arrived at 12:08:23 UTC before the entry was written, and the findings are processed in this record. The sweep for statements that went further than what ran found one more: the F1 evidence claimed a per-test time of 0.00 s that no log shows; it now cites the run times of both logs.
- Current-tree verification: `gh-bot api repos/torrust/torrust-tracker/pulls/2903 --jq .created_at` prints `2026-11-17T11:05:26Z`; review `5938095803` was submitted at `2026-11-17T12:08:23Z`; the two `test result` lines of the coverage logs read `finished in 0.13s` and `finished in 9.32s`.
- Resolution reference: docs(issues): [#2902] correct the log entry that missed the first review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599799088

### F8 - The container-gated scope had no command and no per-file figures

- PR number: 2903
- Source review ID: 5938377458
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599550883
- Concern: The fifth scope existed only as row text, with no reproducible command and per-file results only as prose, and the reading note still said the figures above excluded the drivers.
- Solution: `coverage-evidence.md` now gives the command (both variables, after `cargo llvm-cov clean --workspace`, the separate target directory, the image tags) and the measured commit (`690cfb5f5`, whose `packages/` tree equals `5833cdae9`), adds a container-gated column to all 51 inventory rows, and the note says the four default rows exclude the drivers.
- Current-tree verification: `git diff --stat 5833cdae9 690cfb5f5 -- packages` prints nothing; the per-file inventory rows each have five columns (`awk -F'|'` counts 7 fields on all 51 rows).
- Resolution reference: docs(issues): [#2902] add the container-gated command and per-file column
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599799711

### F9 - The EPIC 10:57 entry was rewritten instead of a new entry appended

- PR number: 2903
- Source review ID: 5938377458
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599550894
- Concern: The T1b commit edited the 10:57 EPIC entry so that an entry dated before the measurement reported it.
- Solution: The original 10:57 text is restored byte for byte from `690cfb5f5`, and a dated entry records T1b with a correction note.
- Current-tree verification: `git show 690cfb5f5:docs/issues/open/1347-overhaul-packages-testing/EPIC.md | sed -n 362p` equals the restored line; `git diff 690cfb5f5` shows only the row 14 note, the removed blank line and the appended entry in the log area.
- Resolution reference: docs(issues): [#1347] restore the 10:57 EPIC log entry and append T1b
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599800327

### F10 - The PR description was stale

- PR number: 2903
- Source review ID: 5938377458
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599550887
- Concern: The description still said the container-gated scope was not measured and that Q1-Q6 were open; the merge tool copies it into the merge commit.
- Solution: The description was edited after the push: it states the T1b result, the async and fixture rules, findings F1-F5 and questions Q2-Q7 (Q1 answered), and names this audit record.
- Current-tree verification: `gh-bot pr view 2903 --repo torrust/torrust-tracker --json body` after the edit.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2903#issuecomment-6948544053
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599800899

## Processing Log

- 2026-11-17 12:48 UTC - Started audit late: the round 1 threads (12:08:23 UTC) and round 2 threads (12:28:40 UTC) had no reply before this record; the fixes were committed first, between 12:37 and 12:46 UTC, one commit per finding (F5 and F9 share one).
- 2026-11-17 12:51 UTC - Pushed the fix commits (head `37d9b1218`), edited the PR description and posted the F10 note at 12:51:13 UTC (issue comment 6948544053); replied to the ten threads between 12:51:24 and 12:51:57 UTC and resolved each after its reply.

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
