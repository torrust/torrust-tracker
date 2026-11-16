---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

# PR #2872 Review Audit

<!-- cspell:ignore peerslee -->

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2872.

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

Round 1 is Copilot's review `5916561572` (commented, "Changes recommended") at the head whose last
commit is `docs(issues): [#2850] point #2838 and the EPIC #1347 table to PR #2872`. Its two inline
findings keep the reviewer's IDs F1 and F2; the review body only summarizes them.

Round 2 is da2ce7's review `5916791851` (approved) at the head whose last commit is
`docs(pr-reviews): [#2850] audit review round 1 of PR #2872`. Its five inline findings are numbered
F1 to F5 by the reviewer; F1 and F2 collide with round 1, so they get the audit IDs F3 to F7 in
source order, with the reviewer's ID kept in each detail entry. The review body only summarizes them.

Round 3 is da2ce7's second review `5917052481` (approved) at the head whose last commit is
`docs(pr-reviews): [#2850] audit review round 2 of PR #2872`. Its three inline findings F6 to F8
collide with existing audit IDs, so they get F8 to F10 in source order, with the reviewer's ID kept.

Round 4 is da2ce7's third review `5917226013` (approved) at the head whose last commit is
`docs(pr-reviews): [#2850] audit review round 3 of PR #2872`. Its one inline finding F9 collides
with an existing audit ID, so it gets F11, with the reviewer's ID kept.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2872-f1 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2872-f2 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2872-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2872-f4 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2872-f5 | Human | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2872-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2872-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2872-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2872-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2872-f10 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2872-f11 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The domain `NumberOfBytes` has no compile-fail test of its construction boundary

- PR number: 2872
- Source review ID: 5916561572
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583142394
- Concern: the positive doctest and the UDP rejection tests would still pass if the field of
  `torrust_tracker_primitives::NumberOfBytes` became public or a signed constructor were added, so
  nothing pins AC1 for this type. Add compile-fail doctests for a signed argument and for the
  tuple constructor, next to the existing successful example.
- Solution: added two doctests to `packages/primitives/src/number_of_bytes.rs`:
  `compile_fail,E0308` passes an `i64` to `NumberOfBytes::new`, and `compile_fail,E0423` builds
  the value with the tuple constructor from outside the crate. The existing successful example
  stays first.
- Current-tree verification: `cargo test --doc -p torrust-tracker-primitives number_of_bytes`
  ran 3 doctests, all passed. With the field changed to `pub u64` for a moment, the same command
  reported the tuple-constructor doctest as `FAILED`; the file was then restored.
- Resolution reference: test: [#2850] pin that a number of bytes cannot be built from a signed integer
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583186952

### F2 - The HTTP `NumberOfBytes` has no compile-fail test of its construction boundary

- PR number: 2872
- Source review ID: 5916561572
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583142335
- Concern: the new HTTP range tests exercise query parsing inside the module, so they cannot detect
  a tuple constructor exposed to other crates, and they do not pin the refusal of a signed
  constructor argument. Add compile-fail doctests for both, plus a successful unsigned
  construction example.
- Solution: added a doc comment to `NumberOfBytes` in
  `packages/http-protocol/src/v1/requests/announce.rs` with a successful construction doctest, a
  `compile_fail,E0308` doctest that passes an `i64` to `new`, and a `compile_fail,E0423` doctest
  that uses the tuple constructor from outside the crate.
- Current-tree verification: `cargo test --doc -p torrust-tracker-http-protocol NumberOfBytes` ran
  3 doctests, all passed; the pre-commit hook's `cargo test --doc --workspace` passed.
- Resolution reference: test: [#2850] pin that a number of bytes cannot be built from a signed integer
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583187197

### F3 - The branch's red and green regression runs are not recorded

- PR number: 2872
- Source review ID: 5916791851
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583310003
- Concern: the spec's Bug-Fix Process and the fix-bug skill ask for the failing run of the regression test on this branch before the fix and the passing runs after it; the issue's evidence held only the manual recheck.
- Solution: added the section "Regression Tests: Red and Green Runs" to `docs/issues/open/2850-non-negative-number-of-bytes/manual-verification-evidence.md`, with the raw outputs in `evidence/regression-red-run.txt` and `evidence/regression-green-run.txt`. The red run was made at the red commit in its own detached worktree; the green run at the branch head after the F4 to F6 fixes.
- Current-tree verification: red: `cargo test -p torrust-tracker-udp-server --lib it_should_reject_an_announce_with_a_negative_number_of_bytes_left` at `test(udp-server): [#2850] reject a negative left in an announce (red)` exited 101 with the test `FAILED` and `seeders: NumberOfPeers(I32(1))` in the response (2026-11-16 08:16:06 to 08:16:41 UTC). Green: the eight commands of the table passed, 72 tests, 0 failed (08:16:11 to 08:16:38 UTC).
- Resolution reference: docs(issues): [#2850] record the red and green runs of the regression tests
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583390892

### F4 - Progress-log and frontmatter stamps are later than the commits that carry them

- PR number: 2872
- Source review ID: 5916791851
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583310011
- Concern: the 21:40 UTC stamps (2850 spec entry and frontmatter, 2838 spec, EPIC) come from a commit authored at 21:23 UTC, after the PR was opened at 21:22 UTC; the 21:25 UTC entry and `performance-evidence.md`'s 21:20 come from a commit authored at 21:17 UTC.
- Solution: set the 21:40 stamps to 21:22 and the 21:25 and 21:20 stamps to 21:17. A sweep of every stamp the branch adds against its commit's author time found one more: `manual-verification-evidence.md` at 21:05 in a commit authored at 21:04, set to 21:04.
- Current-tree verification: a script listed each `last-updated-utc` and progress-log stamp added by each branch commit next to that commit's author time; after the fix, `grep` finds no `21:40` and no `21:25 UTC` in the three specs and the evidence.
- Resolution reference: docs(issues): [#2850] set each stamp to no later than its own commit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583391235

### F5 - `peerslee` is a bencode fragment, not a term

- PR number: 2872
- Source review ID: 5916791851
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583310019
- Concern: `peerslee` is `5:peers`, an empty list and a closing `e`, so a dictionary entry admits a non-word in every file; the tree already ignores it per file.
- Solution: removed it from `project-words.txt`, added it to the spelling directive of `manual-verification-evidence.md`, and added `docs/issues/*/2850-non-negative-number-of-bytes/evidence/*.txt` to the `ignorePaths` of `cspell.json`.
- Current-tree verification: `grep -c peerslee project-words.txt` printed `0`; `linter cspell` passed ("All files passed spell checking!").
- Resolution reference: chore(cspell): [#2850] ignore the bencode fragment `peerslee` per file
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583391581

### F6 - A draft spec still lists the removed `NumberOfBytesOverflow`

- PR number: 2872
- Source review ID: 5916791851
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583310025
- Concern: `docs/issues/drafts/error-locations-si-3-http-tracker-request-errors/ISSUE.md` line 54 maps `NumberOfBytesOverflow`, which no longer exists, and its line numbers were already off.
- Solution: dropped the variant from the row and pointed the three remaining variants at their current lines.
- Current-tree verification: `grep -n` on the `ParseAnnounceQueryError` enum shows `MissingParams` at 219, `MissingParam` at 221 and `InvalidParam` at 227; the row now cites `:219`, `:221`, `:227`.
- Resolution reference: docs(issues): [#2850] drop the removed overflow variant from the SI-3 draft
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583391911

### F7 - The description does not mention the compile-fail doctests or the audit

- PR number: 2872
- Source review ID: 5916791851
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583310033
- Concern: the PR description, which the merge tool copies into the merge commit, did not list the review-round commits or a `cargo test --doc` run that covers the compile-fail doctests.
- Solution: edited the description: a "Review rounds" list names the commits of both rounds by subject, and the validation list names the `cargo test --doc --workspace` run of the pre-commit hook and the red and green runs. The fix is outside the tree, so the reference is the PR conversation response that states it.
- Current-tree verification: `gh-bot pr edit 2872 --body-file` succeeded; the description shows the "Review rounds" list.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2872#issuecomment-6911164755
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583392250

### F8 - The SI-3 draft still counts 11 variants after its row lost one

- PR number: 2872
- Source review ID: 5917052481
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583490622
- Concern: with `NumberOfBytesOverflow` gone the draft's table lists 10 variants, but its text says 11 on lines 49, 84 and 152.
- Solution: changed the count to 10 and said on line 49 why fact 1 of `options.md` counts 11. The sweep found a fourth count, line 73 ("Six of the 11 formats"), changed to 10 as well.
- Current-tree verification: `grep -n` for `11` in the draft finds no variant count left; the table lists 1 + 3 + 2 + 2 + 2 = 10 variants.
- Resolution reference: docs(issues): [#2850] count 10 located variants in the SI-3 draft
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583544077

### F9 - The two regression-run captures are excerpts, not the full text

- PR number: 2872
- Source review ID: 5917052481
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583490625
- Concern: the record called both captures the full text, but the red one starts at cargo's `Finished` line and the green one keeps only selected lines.
- Solution: both lines of `manual-verification-evidence.md` now say "excerpt" and what each capture keeps.
- Current-tree verification: the red capture's first line is cargo's `Finished` line; the green capture has no `Finished` or `Running` line; the record describes both.
- Resolution reference: docs(issues): [#2850] call the regression-run captures excerpts
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583544352

### F10 - The green runs passed 72 tests, not 73

- PR number: 2872
- Source review ID: 5917052481
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583490631
- Concern: row F3's verification and the reply on its thread said 73 results; the eight `test result` lines add up to 72.
- Solution: row F3 now says 72 tests, and a correcting reply was posted on F3's thread (https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583545035).
- Current-tree verification: `grep -c "\.\.\. ok" evidence/regression-green-run.txt` prints 72.
- Resolution reference: docs(pr-reviews): [#2850] count 72 tests in the green regression run
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583544693

### F11 - The round-3 log entry is stamped before the replies it reports

- PR number: 2872
- Source review ID: 5917226013
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583618217
- Concern: the round-3 log entry was stamped 08:38 UTC, but the replies it reports were posted at
  08:39:19 to 08:39:27 UTC, so the entry recorded events after its own stamp.
- Solution: the round-3 entry is now stamped 08:39 UTC, the minute of the replies.
- Current-tree verification: the processing log has no `08:38 UTC` stamp; the round-3 entry and
  the replies share the minute 08:39.
- Resolution reference: docs(pr-reviews): [#2850] audit review round 4 of PR #2872
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4584227007

## Processing Log

- 2026-11-16 07:51 UTC - Started this audit for Copilot's review `5916561572` (submitted
  2026-11-16T07:47:32Z), before any reply. The fix for F1 and F2 is one commit, because the two
  findings ask for the same doctests on the two types.
- 2026-11-16 07:54 UTC - Round 1 processed. The fix was pushed by `git-bot-push` (head
  `test: [#2850] pin that a number of bytes cannot be built from a signed integer`); each of the
  two threads has a reply (posted 2026-11-16T07:54 UTC) and was resolved (`isResolved: true`); a
  fresh GraphQL fetch shows 0 unresolved threads. `validate-audit-record.py --pr-number 2872`
  returned `status: ok`, 2 rows, 0 failures, before the resolution.
- 2026-11-16 08:21 UTC - Round 2: da2ce7's review `5916791851` (submitted 2026-11-16T08:11:06Z,
  approved, five findings). The fixes of F3 to F6 were committed and pushed by `git-bot-push` (head
  `docs(issues): [#2850] record the red and green runs of the regression tests`), the description
  was edited for F7 and comment
  https://github.com/torrust/torrust-tracker/pull/2872#issuecomment-6911164755 states it, and the
  five threads were replied to (2026-11-16T08:20:34Z to 08:20:43Z). Late start: the round-2 rows
  were added to this record after those replies, not before them.
- 2026-11-16 08:21 UTC - Round 2 processed. `validate-audit-record.py --pr-number 2872` returned
  `status: ok`, 7 rows, 3 log entries, 0 failures; then the five threads were resolved
  (`isResolved: true`) and a fresh GraphQL fetch shows 0 unresolved threads.
- 2026-11-16 08:35 UTC - Copilot's review `5916992112` (submitted 2026-11-16T08:27:42Z) reports
  no open issues; it only restates the HTTP benchmark decision of the description; no row.
- 2026-11-16 08:39 UTC - Round 3: da2ce7's review `5917052481` (submitted 2026-11-16T08:32:27Z,
  approved, three Nit findings). The fixes of F8 to F10 were pushed by `git-bot-push` (head
  `docs(pr-reviews): [#2850] count 72 tests in the green regression run`) and the three threads
  were replied to, plus a correcting reply on F3's thread; the rows were added after the replies.
- 2026-11-16 08:39 UTC - Round 3 processed. `validate-audit-record.py --pr-number 2872` returned
  `status: ok`, 10 rows, 6 log entries, 0 failures; then the three threads were resolved and a
  fresh GraphQL fetch shows 0 unresolved threads.
- 2026-11-16 09:51 UTC - Started round 4 for da2ce7's review `5917226013` (submitted
  2026-11-16T08:48:26Z, approved, one Nit finding), before any reply. Copilot's review `5917198610`
  (submitted 2026-11-16T08:45:58Z) reports no open issues and only restates the HTTP benchmark
  decision, which the maintainer accepted; no row.
- 2026-11-16 09:56 UTC - Round 4 processed. The fix was pushed by `git-bot-push` (head
  `docs(pr-reviews): [#2850] audit review round 4 of PR #2872`) and the thread was replied to
  (2026-11-16T09:55:09Z). `validate-audit-record.py --pr-number 2872` returned `status: ok`, 11
  rows, 8 log entries, 0 failures; then the thread was resolved and a fresh GraphQL fetch shows 0
  unresolved threads.

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
