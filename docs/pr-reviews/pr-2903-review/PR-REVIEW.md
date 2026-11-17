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

Round 1 is da2ce7's review `5938095803` of the head whose last commit is `docs(issues): [#2902] specify the tracker-core package tests` (submitted as an
approval, later dismissed by the push), round 2 his review `5938377458` of the head whose last commit is
`docs(issues): [#2902] record the container-gated coverage measurement` (comment). Their inline findings keep the
reviewer's IDs F1 to F10; the review bodies only summarize them. Round 3 is Copilot's review `5938522893`
of the head whose last commit is `docs(issues): [#2902] specify the tracker-core package tests` (submitted 12:38:27 UTC as a comment; its body is an overview
with no request, so it needs no row), found only after the first reply push; its three
inline comments have no IDs and get F11 to F13 in source order. Round 4 is da2ce7's review `5939100172`
(approval) of the head whose last commit is `docs(pr-reviews): [#2902] record Copilot's review of #2903`;
its inline findings keep the reviewer's IDs F14 to F16. Round 5 is Copilot's review `5939369230`
(13:30:51 UTC, comment) of the head whose last commit is `docs(pr-reviews): [#2902] record the fourth
review round of #2903`; its one inline finding keeps the reviewer's ID F17. Round 6 is da2ce7's review `5939412622` (13:33:52 UTC,
submitted as an approval and dismissed by the next push) of the head whose last commit is
`docs(pr-reviews): [#2902] record the round 4 replies of #2903`; its findings keep the reviewer's IDs F18
and F19.

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
| F11 | review-finding:pr-2903-f11 | Copilot | Nit (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2903-f12 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2903-f13 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F12 | NO_ACTION | SUPERSEDED |
| F14 | review-finding:pr-2903-f14 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2903-f15 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2903-f16 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2903-f17 | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2903-f18 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-2903-f19 | Human | Nit | metadata | RE_RAISE_OF:F17 | NO_ACTION | SUPERSEDED |

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
- Current-tree verification: `git diff $(git log --format=%H --fixed-strings --grep='specify the tracker-core package tests' -1) -- docs/issues/open/1347-overhaul-packages-testing/EPIC.md` (the diff against the S0 commit `docs(issues): [#2902] specify the tracker-core package tests`) shows the removed blank line and no other change to the earlier entries.
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
- Solution: `coverage-evidence.md` now gives the command (both variables, after `cargo llvm-cov clean --workspace`, the separate target directory, the image tags) and the measured commit (the S0 commit `docs(issues): [#2902] specify the tracker-core package tests`; the branch changes nothing under `packages/`, so the source is that of `5833cdae9`), adds a container-gated column to all 51 inventory rows, and the note says the four default rows exclude the drivers.
- Current-tree verification: `git diff --stat 5833cdae9 <S0 commit> -- packages` prints nothing (the S0 commit found by its subject; the first version of this line used `HEAD`, which holds only on this branch, F18); the per-file inventory rows each have five columns (`awk -F'|'` counts 7 fields on all 51 rows).
- Resolution reference: docs(issues): [#2902] add the container-gated command and per-file column
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599799711

### F9 - The EPIC 10:57 entry was rewritten instead of a new entry appended

- PR number: 2903
- Source review ID: 5938377458
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599550894
- Concern: The T1b commit edited the 10:57 EPIC entry so that an entry dated before the measurement reported it.
- Solution: The original 10:57 text is restored byte for byte from the S0 commit `docs(issues): [#2902] specify the tracker-core package tests`, and a dated entry records T1b with a correction note.
- Current-tree verification: line 362 of the EPIC at the S0 commit (`git show <S0 commit>:docs/issues/open/1347-overhaul-packages-testing/EPIC.md`) equals the restored line; the diff against that commit shows only the row 14 note, the removed blank line and the appended entry in the log area.
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

### F11 - The measured commits were cited by short SHA

- PR number: 2903
- Source review ID: 5938522893
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599655402
- Concern: Coverage evidence meant to be reproducible should record the full 40-character SHA, since a short one can become ambiguous.
- Solution: `coverage-evidence.md` records the full SHA of `5833cdae9` (also in its `measured-commit` frontmatter). It first also recorded the full SHA of the branch commit of the container-gated run; F14 replaced that with the commit subject, since a rebase can rewrite a branch id.
- Current-tree verification: `git rev-parse 5833cdae9` prints the SHA the file cites.
- Resolution reference: docs(issues): [#2902] record the full commit SHAs of the measurements
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599904133

### F12 - The slash in the uncovered-lines column was ambiguous

- PR number: 2903
- Source review ID: 5938522893
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599655482
- Concern: In the combined rows, `304 / 64` reads like a fraction and the slash does not say which file each set of lines belongs to; split the rows per file or label them.
- Solution: The slash came from the first table and spread with the F3 fix, which used it between production and test-module lines. Every lines cell of a file with a test module now labels its `production:` and `test module:` lines, the two multi-file rows are split into one row per file (`peer_key.rs`, `key/mod.rs`; `whitelist/manager.rs`, `repository/persisted.rs`, `authorization.rs`), and the class column names alternatives in words instead of a slash.
- Current-tree verification: `grep -n ' / [0-9]' docs/issues/open/2902-1347-add-tests-tracker-core/coverage-evidence.md` prints nothing after the `## Uncovered Line Groups` heading.
- Resolution reference: docs(issues): [#2902] label production and test-module uncovered lines
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599905007

### F13 - The same slash ambiguity on a second row

- PR number: 2903
- Source review ID: 5938522893
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599655543
- Concern: The same comment as F12, on a second combined row.
- Solution: Same change as F12; no separate fix.
- Current-tree verification: As F12.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599905897
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4599905897

### F14 - Durable text cited branch commit ids

- PR number: 2903
- Source review ID: 5939100172
- Reviewer finding ID: F14
- Source URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4600053985
- Concern: The evidence, the spec log and the audit cited branch commit ids (`2b0c3fe36`, `37d9b1218`, `690cfb5f5` short and full, `8f42a03a6`) that a rebase can rewrite.
- Solution: `coverage-evidence.md` names the container-gated run's commit by subject and the measured source by the full id of `5833cdae9` on `develop`; the audit's round descriptions and finding details name heads and commits by subject. The ids left are in append-only progress and processing log entries (`ISSUE.md:414`, `PR-REVIEW.md` processing log), which are not rewritten; they resolve at this head and stay reachable after the merge.
- Current-tree verification: `grep -n -E '2b0c3fe36|37d9b1218|690cfb5f5|8f42a03a6' docs/issues/open/2902-1347-add-tests-tracker-core/coverage-evidence.md` prints nothing; in `PR-REVIEW.md` the ids appear only in Processing Log entries and in this entry.
- Resolution reference: docs(issues): [#2902] cite the container-gated run by commit subject
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4600182264

### F15 - Two lines cells in files with a test module had no label

- PR number: 2903
- Source review ID: 5939100172
- Reviewer finding ID: F15
- Source URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4600053992
- Concern: `error.rs` and `statistics/repository.rs` have test modules but their cells listed production lines without a `production:` label, against the rule above the table and the F12 entry.
- Solution: Both cells are labeled `production:`. Checking every row against its file found one more: `test_helpers.rs` (test support), now labeled `test module:`.
- Current-tree verification: a loop over the table rows that checks each file for a `cfg(test)` attribute and an unlabeled cell prints nothing.
- Resolution reference: docs(issues): [#2902] label the last unlabeled uncovered-line cells
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4600182958

### F16 - The record described Copilot's review wrongly

- PR number: 2903
- Source review ID: 5939100172
- Reviewer finding ID: F16
- Source URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4600054002
- Concern: The record said Copilot's review was of the second head and had an empty body; it was of the first head (`commit_id` of the S0 commit) and has a 3,254-character overview with no request.
- Solution: The Findings introduction now gives the reviewed head (by commit subject), the submission time 12:38:27 UTC and the body as an overview with no request. The processing log entry of 12:53 UTC, which gives 12:38:26 UTC (the time of the inline comments), is corrected by an appended entry.
- Current-tree verification: `gh-bot api repos/torrust/torrust-tracker/pulls/2903/reviews/5938522893 --jq '[.commit_id,.submitted_at,(.body|length)]'` prints the S0 commit's full id, `2026-11-17T12:38:27Z` and `3254`.
- Resolution reference: docs(pr-reviews): [#2902] record the fourth review round of #2903
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4600183671

### F17 - Pending reply markers in the round 4 rows

- PR number: 2903
- Source review ID: 5939369230
- Reviewer finding ID: F17
- Source URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4600249246
- Concern: F14-F16 were marked `FIXED`/`RESOLVED` while their Reply URL fields held `PENDING_REPLY_*` markers, which fail the validator's same-thread reply check.
- Solution: The markers were in the audit commit made before the replies existed; the next commit replaced them with the three reply URLs, which are the ones the comment lists. No further change was needed beyond recording this row.
- Current-tree verification: `grep -c PENDING docs/pr-reviews/pr-2903-review/PR-REVIEW.md` prints 0; the audit validator reports no failure.
- Resolution reference: docs(pr-reviews): [#2902] record the round 4 replies of #2903
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4600261767

### F18 - The unchanged-source check compared with HEAD

- PR number: 2903
- Source review ID: 5939412622
- Reviewer finding ID: F18
- Source URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4600282774
- Concern: `git diff --stat 5833cdae9 HEAD -- packages` prints nothing only while `HEAD` is on this branch; after the merge, later slices change `packages/` and the check stops holding.
- Solution: `coverage-evidence.md` and F8's verification name the range by the S0 commit, found by its subject, instead of `HEAD`.
- Current-tree verification: `git diff --stat 5833cdae9 $(git log --format=%H --fixed-strings --grep='specify the tracker-core package tests' -1) -- packages | wc -l` prints 0.
- Resolution reference: docs(issues): [#2902] name the S0 commit in the unchanged-source check
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4600363938

### F19 - Copilot's F17 comment had no reply, row or resolution

- PR number: 2903
- Source review ID: 5939412622
- Reviewer finding ID: F19
- Source URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4600282784
- Concern: Copilot's comment of 13:30:51 UTC had no reply, row or resolution when this review was written.
- Solution: Same concern as F17, which was recorded, replied to and resolved in the commit after the head this review saw; no separate change.
- Current-tree verification: the F17 row and detail entry exist and its thread is resolved.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4600364534
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2903#discussion_r4600364534

## Processing Log

- 2026-11-17 12:48 UTC - Started audit late: the round 1 threads (12:08:23 UTC) and round 2 threads (12:28:40 UTC) had no reply before this record; the fixes were committed first, between 12:37 and 12:46 UTC, one commit per finding (F5 and F9 share one).
- 2026-11-17 12:51 UTC - Pushed the fix commits (head `37d9b1218`), edited the PR description and posted the F10 note at 12:51:13 UTC (issue comment 6948544053); replied to the ten threads between 12:51:24 and 12:51:57 UTC and resolved each after its reply.
- 2026-11-17 12:53 UTC - Found Copilot's review `5938522893` (12:38:26 UTC, three threads), which the 12:51 replies had missed: the thread check right after pushing the audit commit (committed 12:53:03 UTC) showed three unresolved threads. Recorded as F11-F13.
- 2026-11-17 13:01 UTC - Pushed the F11 and F12 fixes (head `8f42a03a6`); replied to the three threads between 13:01:41 and 13:01:48 UTC and resolved each after its reply (F13 superseded by F12).
- 2026-11-17 13:16 UTC - Round 4 (da2ce7's review `5939100172`, 13:13:35 UTC, approval with F14-F16) processed: F15 and F14 fixed in their own commits; the round descriptions and finding details now cite commits by subject. Correction of the 12:53 UTC entry above: Copilot's review was submitted at 12:38:27 UTC (12:38:26 UTC is the time of its first inline comments) and was of the first head, not the second (F16).
- 2026-11-17 13:25 UTC - Pushed the round 4 fixes and audit commits; replied to the F14-F16 threads between 13:24:53 and 13:25:01 UTC and resolved each after its reply.
- 2026-11-17 13:31 UTC - Round 5 (Copilot's review `5939369230`, 13:30:51 UTC, F17) was about the head before the round 4 replies were recorded; replied at 13:31:57 UTC and resolved the thread.
- 2026-11-17 13:41 UTC - Round 6 (da2ce7's review `5939412622`, 13:33:52 UTC, F18-F19): F18 fixed in its own commit and pushed; replied to both threads at 13:41:03 and 13:41:07 UTC and resolved each after its reply (F19 superseded by F17).

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
