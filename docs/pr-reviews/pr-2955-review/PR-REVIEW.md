---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2955 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2955.

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

F1 to F5 come from da2ce7's review 5971033415 (APPROVED on the first head, with an ACK comment),
and F7 and F8 from da2ce7's review 5971375457 (APPROVED on the round-1 head, with an ACK comment); the
reviewer's IDs are kept. da2ce7's F6 was never posted (the review says it was raised against an
intermediate head and fixed by the next push), so it has no row. F9 to F11 come from Copilot's review 5971580124 on the round-1 head; Copilot gives
no IDs, so they get the next audit-local ordinals after the reviewer's F6 to F8. F12 and F13 come from
da2ce7's review 5971881789 (APPROVED on the head that recorded F9 to F11, with an ACK comment). Copilot's review 5963497070 on the same head had no findings and gets no
row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2955-f1 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2955-f2 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2955-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2955-f4 | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2955-f5 | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2955-f7 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2955-f8 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2955-f9 | Copilot | Minor (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F10 | review-finding:pr-2955-f10 | Copilot | Minor (inferred) | maintainability | RE_RAISE_OF:F9 | NO_ACTION | SUPERSEDED |
| F11 | review-finding:pr-2955-f11 | Copilot | Nit (inferred) | testing | ORIGINAL | NO_ACTION | SUPERSEDED |
| F12 | review-finding:pr-2955-f12 | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2955-f13 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The spec's last-updated line conflicts with develop

- PR number: 2955
- Source review ID: 5971033415
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4625664458
- Concern: The `last-updated-utc` line of the SI-2 spec conflicts with `develop` at `7a4c16eaf`, where the archive of the #2804 audit set it to `"2026-11-18 19:11"` and moved the spec's audit paths to `docs/issues/closed/`; the open-pull-request skill requires an update when the branch conflicts.
- Solution: Merged `develop` at `f212ae33b` into the PR head (an earlier local rebase onto `7a4c16eaf` was not pushed), keeping `develop`'s `"2026-11-18 19:11"` for the conflicting line; the next commit set the field to `"2026-11-19 07:27"`, newer than `develop`'s, and later commits moved it on. The four audit paths are `docs/issues/closed/` from `develop`.
- Current-tree verification: `git merge-base --is-ancestor f212ae33b HEAD` exits 0; the spec's `last-updated-utc` is later than `"2026-11-18 19:11"`; `grep -n 'open/2804'` on the spec finds only the 2026-11-11 progress-log entry, which is history.
- Resolution reference: Merge torrust/develop into 2914-primitives
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4625809666

### F2 - The recorded red output is not from the tests-only commit

- PR number: 2955
- Source review ID: 5971033415
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4625664468
- Concern: The red output recorded `peer.rs:680:13`, but at the tests-only commit the helper's `assert!` is on line 688; the run at 18:52:30 preceded the commit at 18:52:43 and was of the unformatted tree, yet the evidence said it ran "on the tests-only commit".
- Solution: The evidence now names the first run's state (the working tree before `cargo +nightly fmt` and the commit) and adds a re-run at the commit itself, in a detached temporary worktree: all three tests fail and the panics name `peer.rs:688:13`. The same sweep corrected the environment line, which said the V1 and V2 "Before" runs used the tests-only commit; they ran on the unchanged `develop` tree.
- Current-tree verification: Re-run on 2026-11-19 07:39:10 to 07:39:11 UTC at `test(primitives): [#2914] add red tests that peer role and peer id errors name no source path`: 3 failed, panics at `peer.rs:688:13`; output recorded in `manual-verification-evidence.md`, section Red.
- Resolution reference: docs(issues): [#2914] record the red re-run at the tests-only commit and the review-fix evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4625809994

### F3 - The publish plan still gives SI-2 as TODO

- PR number: 2955
- Source review ID: 5971033415
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4625664477
- Concern: Line 109 of the first-publish plan says SI-2 (#2914, `TODO`) must land before `primitives` 4.0.0, while the new bullet records the change as made and the PR closes the issue on merge.
- Solution: The state line drops `TODO` and says PR #2955 implements SI-2 and closes the issue when it merges; row 1 of the checklist names PR #2955 too.
- Current-tree verification: `grep -c "2914, .TODO" <file>` prints 0 for `docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md`; lines 109 to 111 and row 1 name PR #2955.
- Resolution reference: docs(issues): [#2914] name PR #2955 in the primitives state line of the publish plan
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4625810337

### F4 - No test pins the new peer-role wording

- PR number: 2955
- Source review ID: 5971033415
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4625664485
- Concern: The tests check the rejected text and the absence of a path, so a return to `invalid param {raw_param}` would stay green although the publish plan records the exact new text.
- Solution: New test `it_should_name_the_accepted_roles_when_it_rejects_a_name` compares the whole `Display` of `"x".parse::<PeerRole>()` with `invalid peer role "x", expected "seeder" or "leecher"`.
- Current-tree verification: Mutation at 07:41:32 UTC: with the format set back to `"invalid param {raw_param}"`, only the new test failed (`left: "invalid param x"`); restored, `git diff --quiet HEAD` exited 0; `cargo test -q -p torrust-tracker-primitives` passes 129 unit tests and 8 doctests.
- Resolution reference: test(primitives): [#2914] pin the full message of a rejected peer role
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4625810653

### F5 - Mark the assertion helper `#[track_caller]`

- PR number: 2955
- Source review ID: 5971033415
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4625664493
- Concern: Without `#[track_caller]` every failure of `assert_names_no_source_path` reports the helper's line; ADR 20261118045443 rule 6 allows it on assertion helpers.
- Solution: The helper is `#[track_caller]`, and its doc comment says a failure is reported at the caller's line.
- Current-tree verification: Mutation at 07:40:15 UTC: with a path appended to the `NotEnoughBytes` format, the failing test panicked at `peer.rs:1449:13`, its own `assert_names_no_source_path(&message);` line; restored, `git diff --quiet HEAD` exited 0.
- Resolution reference: test(primitives): [#2914] report source-path assertion failures at the test line
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4625810980

### F7 - The description still describes the first head

- PR number: 2955
- Source review ID: 5971375457
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4625932008
- Concern: The PR description still said "128 unit tests and 8 doctests", listed three commits and four files, and did not mention the `develop` merge, the review-fix commits or the audit; the merge tool copies the description into the merge commit.
- Solution: Edited the PR description (no commit): it lists every non-merge commit, the merge "Merge torrust/develop into 2914-primitives", the audit file, and 129 unit tests and 8 doctests. A first draft wrote "closed #2804", which the closing-keyword scan caught; it was reworded before the edit.
- Current-tree verification: `gh-bot pr view 2955 --json body` contains "129 unit tests"; the closing-keyword scan of the description matches only `Closes #2914`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4625943446
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4625943446

### F8 - An evidence sentence claims more than the recorded command ran

- PR number: 2955
- Source review ID: 5971375457
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4625932015
- Concern: The F4 mutation's evidence said all three no-source-path tests stayed green, but its command filtered on `peer_role`, which selects 9 tests and only one of the three.
- Solution: The evidence now says what the filter selected and records the same mutation run over every unit test of the crate (`cargo test -q -p torrust-tracker-primitives --lib`, 2026-11-19 08:10:15 UTC): 128 passed, 1 failed (the new test), so all three no-source-path tests ran and stayed green.
- Current-tree verification: the run's output was read from its log before the sentence was written; `git diff --quiet HEAD -- packages/primitives/src/peer.rs` exited 0 after the restore.
- Resolution reference: docs(issues): [#2914] say which tests the F4 mutation run selected and add a full-suite run
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4626078892

### F9 - The accepted role names are written in two places

- PR number: 2955
- Source review ID: 5971580124
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4626095741
- Concern: `"seeder"` and `"leecher"` appear in the `FromStr` match arms and in the new `#[error]` text, so they could drift; Copilot suggests one shared source. Severity inferred from Copilot's "Medium" marker.
- Solution: No change. Centralizing the names would change how `PeerRole` is parsed, which the SI-2 spec puts out of scope; the duplication between `FromStr` and `Display for PeerRole` predates this PR. `PeerRole` is a closed two-variant enum, and the F4 test pins the full message.
- Current-tree verification: at the head "Merge torrust/develop into 2914-primitives after the #1488 EPIC close-out", `grep -n '"seeder"\|"leecher"' packages/primitives/src/peer.rs` shows the names in `Display` (lines 59-60), `FromStr` (lines 70-71) and the tests; the spec's Out of Scope list has "Any other change to `PeerRole` or `peer::Id` parsing behavior" (line 68).
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4626211484
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4626211484

### F10 - The accepted role names are written in two places (second comment)

- PR number: 2955
- Source review ID: 5971580124
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4626095804
- Concern: The same suggestion as F9, posted on the `#[error]` line.
- Solution: No change, for the reasons recorded under F9.
- Current-tree verification: as for F9.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4626211828
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4626211828

### F11 - The no-source-path guard matches raw substrings

- PR number: 2955
- Source review ID: 5971580124
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4626095873
- Concern: Matching `.rs:` and `src/` as substrings could give false positives if a message legitimately contained them, for example from user input; Copilot suggests tighter path patterns. Severity inferred from Copilot's "Low" marker.
- Solution: No change. The fragment list is the one the SI-2 spec prescribes (Regression Test Strategy, "Portable path check"), and the guard only checks messages built from fixed test inputs, none of which contains a listed fragment.
- Current-tree verification: line 107 of the SI-2 spec lists the fragments; the three callers of `assert_names_no_source_path` render errors for `"uploader"`, `vec![0; 19]` and `vec![0; 21]`, and the three passing tests show none of the rendered messages contains a fragment.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4626212269
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4626212269

### F12 - The record cites a branch commit by id

- PR number: 2955
- Source review ID: 5971881789
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4626325385
- Concern: F9's Current-tree verification and the 08:38 log entry named the second `develop` merge by its commit id, which the `open-pull-request` commit-id check reports as a PR-branch id; the F9 reply on the forge cites the same id.
- Solution: Both places now name the merge by its subject, "Merge torrust/develop into 2914-primitives after the #1488 EPIC close-out". The 08:38 entry was edited in place for this, because the commit-id rule and the append-only rule cannot both hold otherwise; the correction entry of this round says so. The F9 reply cannot be edited from the audit's side and keeps the id; the id it cites is that merge commit. The commit-id check was skipped before the pushes of the second merge and of the F9 to F11 audit; it now runs before every push.
- Current-tree verification: the skill's commit-id check against `develop` prints nothing.
- Resolution reference: docs(pr-reviews): [#2914] cite the second develop merge by subject and name the reviewer by account
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4626395952

### F13 - Refer to the reviewer by account, not with a pronoun

- PR number: 2955
- Source review ID: 5971881789
- Reviewer finding ID: F13
- Source URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4626325396
- Concern: The Findings introduction and the 08:24 log entry referred to the reviewer with a gendered pronoun, for which the record has no source.
- Solution: Those three places now say "da2ce7's". A search of every file this PR changes found no other pronoun for a person.
- Current-tree verification: `grep -n -wiE "he|his|him|she|her"` over the audit, the SI-2 spec and evidence and the first-publish plan matches only two lines of this audit: this verification line, which spells the pattern, and the round-3 correction entry, which quotes the replaced text.
- Resolution reference: docs(pr-reviews): [#2914] cite the second develop merge by subject and name the reviewer by account
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2955#discussion_r4626396327

## Processing Log

- 2026-11-19 07:44 UTC - Started audit for da2ce7's review 5971033415 (APPROVED, submitted 07:36:51Z on the first head, whose last commit was "docs(issues): [#2914] record the evidence of SI-2 and its breaking change for primitives 4.0.0"). Five findings, F1 to F5 with the reviewer's IDs. The branch had already been rebased onto `develop` at `7a4c16eaf` (F1) before the review arrived. Fixes committed: F5, F4 (test code), F3, F2 (docs). Not pushed yet; replies and thread resolution follow the push, and the Reply URL and Thread state fields are updated then.
- 2026-11-19 07:51 UTC - Correction to the 07:44 entry: the rebase onto `7a4c16eaf` it mentions was local only and was replaced, on the maintainer's request, by a merge of `develop` at `f212ae33b` into the PR head ("Merge torrust/develop into 2914-primitives"); the later commits were carried over with the same content. F1's Solution, verification and resolution reference now name the merge.
- 2026-11-19 07:55 UTC - Pushed "docs(issues): [#2914] record that develop was merged into the PR instead of rebased" as the new head (a fast-forward of the PR branch, pre-push hook passed). Replied on the five threads (replies created 07:54:47Z to 07:54:56Z), recorded the reply URLs, ran `validate-audit-record.py --pr-number 2955 --base torrust/develop` through a scratch copy that calls `gh-bot` (status ok, 5 rows, 0 failures), then resolved the five threads by 07:55:31 UTC (a `date -u` run after the resolve commands). GraphQL refresh: zero unresolved threads on PR #2955.
- 2026-11-19 08:24 UTC - Round 2: da2ce7's review 5971375457 (APPROVED at 08:09:02Z on the round-1 head, "docs(pr-reviews): [#2914] record the replies and resolved threads of PR #2955 round 1"), with F7 (Minor) and F8 (Nit); da2ce7's F6 was never posted. F7: edited the PR description and replied at 08:10:25Z. On the maintainer's choice to fix in this PR rather than in the close-out, F8 was fixed in "docs(issues): [#2914] say which tests the F4 mutation run selected and add a full-suite run", pushed as the new head, and replied at 08:23:21Z. Ran `validate-audit-record.py --pr-number 2955 --base torrust/develop` through the `gh-bot` scratch copy (status ok, 7 rows, 0 failures), then resolved both threads by 08:23:52 UTC (a `date -u` run after the resolve commands). GraphQL refresh: zero unresolved threads on PR #2955.
- 2026-11-19 08:38 UTC - After round 2, `develop` moved and the PR conflicted again on the `last-updated-utc` line of the first-publish plan; merged `develop` at `3aa88bf19` in "Merge torrust/develop into 2914-primitives after the #1488 EPIC close-out" (a distinct subject, so F1's reference stays unique) and pushed it as the new head. Copilot's review 5971580124 (COMMENTED at 08:24:55Z on the round-1 head; its full agentic review timed out) left three comments, recorded as F9 to F11: no change for all three (F10 repeats F9). Replied at 08:37:00Z, 08:37:02Z and 08:37:05Z, ran `validate-audit-record.py --pr-number 2955 --base torrust/develop` through the `gh-bot` scratch copy (status ok, 10 rows, 0 failures), then resolved the three threads by 08:37:52 UTC (a `date -u` run after the resolve commands). GraphQL refresh: zero unresolved threads on PR #2955.
- 2026-11-19 08:52 UTC - Round 3: da2ce7's review 5971881789 (APPROVED at 08:48:52Z, with an ACK comment) with F12 (Minor) and F13 (Suggestion). Correction, made in place: in the 08:38 entry, "pushed it as head" followed by the merge's commit id became "pushed it as the new head" (F12), and in the 08:24 entry "his F6" became "da2ce7's F6" (F13); the Findings introduction and F9's Current-tree verification were corrected the same way. No other content of earlier entries changed.
- 2026-11-19 08:57 UTC - Pushed "docs(pr-reviews): [#2914] cite the second develop merge by subject and name the reviewer by account" as the new head after the commit-id check printed nothing. Replied on F12 and F13 (replies created 08:56:08Z and 08:56:10Z), recorded the reply URLs, ran `validate-audit-record.py --pr-number 2955 --base torrust/develop` through the `gh-bot` scratch copy (status ok, 12 rows, 0 failures), then resolved both threads by 08:56:50 UTC (a `date -u` run after the resolve commands). GraphQL refresh: zero unresolved threads on PR #2955.

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
