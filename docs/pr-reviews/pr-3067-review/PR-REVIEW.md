# PR #3067 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3067.

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
- Reviewer: the author's GitHub login, for example `da2ce7` or `coderabbitai[bot]`; `unknown`
  when GitHub shows no author
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

Reviews processed: CodeRabbit 6070552226 (F1) and da2ce7 6071125410, 6075566793 and 6075682135. Review 6070518939 (claude[bot], fork notice) contains no finding and has no row. Findings without an inline thread come from review bodies and are `NON_RESOLVABLE`.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3067-f1 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3067-f2 | da2ce7 | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3067-f3 | da2ce7 | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3067-f4 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3067-f5 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3067-f6 | da2ce7 | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3067-f7 | da2ce7 | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3067-f8 | da2ce7 | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3067-f9 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-3067-f10 | da2ce7 | Suggestion | metadata | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-3067-f11 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F12 | review-finding:pr-3067-f12 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-3067-f13 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-3067-f14 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F15 | review-finding:pr-3067-f15 | da2ce7 | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-3067-f16 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-3067-f17 | da2ce7 | Nit | maintainability | ORIGINAL | NO_ACTION | NON_RESOLVABLE |
| F18 | review-finding:pr-3067-f18 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - V1b count must read the directory the run used

- PR number: 3067
- Source review ID: 6070552226
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4705493357
- Concern: The inline `TMPDIR=$(mktemp -d)` applied to `cargo` only, so the later `ls "$TMPDIR"` count did not describe the run's directory.
- Solution: Reran V1b in one script: create and export `TMPDIR` once, run the suite, count that same directory, and record the new count.
- Current-tree verification: Read `manual-verification-evidence.md` V1b steps 2 to 4 and its observed result (`entries=0 data_db=0`).
- Resolution reference: docs(issues): [#2925] rerun the no-database classification with a stamped and exported TMPDIR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4710988384

### F2 - AC4 coverage list names tests that never start SQLite

- PR number: 3067
- Source review ID: 6071125410
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4705938517
- Concern: The AC4 list named three groups whose tests never open SQLite, omitted the `axum-rest-api-server` contexts that stop opening it, and understated the helper-only handler cases.
- Solution: Rewrote the AC4 section: only tests that open a database count as coverage, the non-opening groups are named as not coverage, and every package whose tests stop opening a database is named. The list stays a proposal until the maintainer approves it.
- Current-tree verification: Read the AC4 section of `ISSUE.md`; checked the named files with grep for `Database::Sqlite3` and `sqlite3`.
- Resolution reference: docs(issues): [#2925] correct the SQLite start-up coverage list and keep T1 in progress
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4710988890

### F3 - Recorded times later than the commit that records them

- PR number: 3067
- Source review ID: 6071125410
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4705938525
- Concern: The evidence file and ISSUE.md recorded 08:30 and 08:35 stamps, later than commit 3bb76d813 (08:09:21 UTC), while claiming nothing is invented.
- Solution: The rerun commit replaced the evidence-file times with the actual script times and set both `last-updated-utc` values from the clock. The remaining 08:35 progress-log entry is marked as an estimate by an appended correction entry; the log is not rewritten.
- Current-tree verification: Read the Environment section of the evidence file, the frontmatter of both files and the last progress-log entry of `ISSUE.md` (stamped 16:12, not later than its commit at 16:13:34 UTC).
- Resolution reference: docs(issues): [#2925] mark the 08:35 progress log stamp as an estimate
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4710989383

### F4 - V1b and V1c lack stamps and the revert command

- PR number: 3067
- Source review ID: 6071125410
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4705938529
- Concern: V1b and V1c carried no start and end stamps, and the revert of the `ephemeral()` edit named no command.
- Solution: V1b has start and end stamps and the revert command with its check (`git diff --quiet HEAD -- ...` exit 0). V1c states that its start and end were not stamped and gives the output-file time.
- Current-tree verification: Read V1b steps 2 to 5 and V1c steps in the evidence file.
- Resolution reference: docs(issues): [#2925] rerun the no-database classification with a stamped and exported TMPDIR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4710989911

### F5 - Doc comment of ephemeral() claim reads as present fact

- PR number: 3067
- Source review ID: 6071125410
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4705938536
- Concern: A reason for keeping the names said the doc comment of `ephemeral()` mentions a database, which is not true at this head.
- Solution: The sentence was removed when the naming proposal was replaced by the approved decision.
- Current-tree verification: `grep -n 'doc comment' ISSUE.md` finds nothing.
- Resolution reference: docs(issues): [#2925] record the approved naming decision
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4710990405

### F6 - Only two of three app tests set persistent_torrent_completed_stat

- PR number: 3067
- Source review ID: 6071125410
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4705938540
- Concern: The table and the V1b conclusion treated the three `app` tests alike, while one uses `ephemeral_private()` and expects persistence.
- Solution: The `torrust-tracker --lib` row separates the two tests that set `persistent_torrent_completed_stat` from the third, and the V1b conclusion names the `app` module apart from the REST API and HTTP tests.
- Current-tree verification: Read the table row in `ISSUE.md` and the V1b conclusion in the evidence file.
- Resolution reference: docs(issues): [#2925] make the classification table and its causes precise
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4710990863

### F7 - Bench row makes the table sum differ from the prose

- PR number: 3067
- Source review ID: 6071125410
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4705938544
- Concern: The bench row counted in the Failing and Need persistence columns, so the sums were 78 and 49 against 77 and 48 in the prose.
- Solution: The row reads `1 bench` and a Total row reads 77 tests and 1 bench, 48 tests and 1 bench, 29.
- Current-tree verification: Summed the columns: 25+5+9+2+31+2+3=77, 1+0+9+2+31+2+3=48, 24+5=29.
- Resolution reference: docs(issues): [#2925] make the classification table and its causes precise
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4710991381

### F8 - The drop from 100 failures attributed to tracker-core alone

- PR number: 3067
- Source review ID: 6071125410
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4705938557
- Concern: The drop was attributed to `tracker-core` alone although `axum-rest-api-server` moved from 36 to 31 as its test files changed.
- Solution: The wording is now 'mostly because' and the `axum-rest-api-server` move is stated.
- Current-tree verification: Read the second Classification bullet of `ISSUE.md`.
- Resolution reference: docs(issues): [#2925] make the classification table and its causes precise
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4710991826

### F9 - Template section Failures and Follow-up is missing

- PR number: 3067
- Source review ID: 6071125410
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4705938558
- Concern: The evidence file lacked the template's closing `Failures and Follow-up` section.
- Solution: Added it: no process failed, V1b was rerun once, V1c has no stamps.
- Current-tree verification: Read the end of the evidence file before the appendix.
- Resolution reference: docs(issues): [#2925] rerun the no-database classification with a stamped and exported TMPDIR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4710992270

### F10 - Frontmatter status stays planned while T1 is in progress

- PR number: 3067
- Source review ID: 6071125410
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4705938569
- Concern: The frontmatter `status` was `planned` while T1 is `IN_PROGRESS`.
- Solution: Set `status: in-progress`, which is allowed in `open/`.
- Current-tree verification: Read line 5 of `ISSUE.md`.
- Resolution reference: docs(issues): [#2925] make the classification table and its causes precise
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4710992728

### F11 - PR description should say Related to #2925

- PR number: 3067
- Source review ID: 6071125410
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3067#pullrequestreview-6071125410
- Concern: The skill's table asks for `Related to #2925` where the description wrote `Issue: #2925`.
- Solution: Edited the PR description; the fix is outside the tree.
- Current-tree verification: Read the PR body with `gh-bot pr view 3067`: it contains `Related to #2925`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3067#issuecomment-7197444126
- Follow-up PR URL: N/A
- Reply URL: N/A

### F12 - Approval recorded only as given in this session

- PR number: 3067
- Source review ID: 6075566793
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4709536207
- Concern: The approval of the naming was cited as given 'in this session', which no reader can follow.
- Solution: The spec cites the durable PR comment that quotes the approvals and their source.
- Current-tree verification: Read Architectural Decisions, the decisions section and the progress log of `ISSUE.md`; the comment exists and was posted at 2026-11-26T16:07:16Z.
- Resolution reference: docs(issues): [#2925] carry the approved decisions to the places that track them
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4710993169

### F13 - Decision not carried to the places that track it

- PR number: 3067
- Source review ID: 6075566793
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4709536227
- Concern: Architectural Decisions listed the naming as open, and T4 and AC2 did not mention the rename of `ephemeral_with_no_services()`.
- Solution: Architectural Decisions records the decision, the T4 row names the rename in every caller, and AC2 checks it.
- Current-tree verification: Read lines for Architectural Decisions, the T4 row and AC2 in `ISSUE.md`.
- Resolution reference: docs(issues): [#2925] carry the approved decisions to the places that track them
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4710993694

### F14 - PR description lists the names as a proposal and omits the rename

- PR number: 3067
- Source review ID: 6075566793
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3067#pullrequestreview-6075566793
- Concern: The description still listed keeping the names as a proposal and did not mention the approved rename.
- Solution: Edited the PR description to record the naming as approved with the rename; the fix is outside the tree.
- Current-tree verification: Read the PR body with `gh-bot pr view 3067`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3067#issuecomment-7197444126
- Follow-up PR URL: N/A
- Reply URL: N/A

### F15 - T1 marked DONE without approval of the coverage list

- PR number: 3067
- Source review ID: 6075682135
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4709629275
- Concern: T1 was `DONE` although its expected output includes the maintainer's approval of the SQLite start-up coverage, which was not logged and listed non-opening tests.
- Solution: T1 is `IN_PROGRESS` again, a correction entry is logged, and the corrected AC4 list is a proposal awaiting approval (reported to the maintainer as a question).
- Current-tree verification: Read the T1 row, the 16:05 progress-log entry and the AC4 heading of `ISSUE.md`.
- Resolution reference: docs(issues): [#2925] correct the SQLite start-up coverage list and keep T1 in progress
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4710994218

### F16 - Heading still says Proposals for the maintainer

- PR number: 3067
- Source review ID: 6075682135
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4709629288
- Concern: The heading read 'Proposals for the maintainer (T1 approval)' above items labelled approved.
- Solution: Renamed it `Decisions approved by the maintainer`.
- Current-tree verification: Read the heading in `ISSUE.md`.
- Resolution reference: docs(issues): [#2925] carry the approved decisions to the places that track them
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3067#discussion_r4710994708

### F17 - Commit subject names two approvals but not the T1 DONE change

- PR number: 3067
- Source review ID: 6075682135
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3067#pullrequestreview-6075682135
- Concern: The subject of 4ec424810 names the two approvals but not the change of T1 to `DONE`.
- Solution: No change: history is not rewritten. T1 is back to `IN_PROGRESS` and the correction is in the spec progress log.
- Current-tree verification: `git log` shows the subject unchanged; the T1 row is `IN_PROGRESS` and the 16:05 progress-log entry records the correction.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3067#issuecomment-7197444126
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3067#issuecomment-7197444126

### F18 - PR description presents all three decisions as proposals

- PR number: 3067
- Source review ID: 6075682135
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3067#pullrequestreview-6075682135
- Concern: The description presented the naming, package order and helper pattern as awaiting approval.
- Solution: Edited the PR description to list the three decisions as approved, cite the record, and keep the AC4 list open; the fix is outside the tree.
- Current-tree verification: Read the PR body with `gh-bot pr view 3067`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3067#issuecomment-7197444126
- Follow-up PR URL: N/A
- Reply URL: N/A

## Processing Log

- 2026-11-26 16:18 UTC - Started audit for reviews 6070552226, 6071125410, 6075566793 and 6075682135.
- 2026-11-26 16:18 UTC - Pushed head `674a5e225` (`git ls-remote` showed it) before any reply; the fix commit added in this round is the 08:35 estimate correction. The audit record was created after the replies, and the log of the start is therefore late.
- 2026-11-26 16:18 UTC - Replies on the 14 inline threads were posted from 16:16:38 to 16:17:10 UTC, then the consolidated response https://github.com/torrust/torrust-tracker/pull/3067#issuecomment-7197444126 at 16:17:22 UTC (GitHub `created_at`). Thread resolution follows the audit commit push.

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
