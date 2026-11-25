---
pr-number: 3049
pr-url: https://github.com/torrust/torrust-tracker/pull/3049
last-updated-utc: "2026-11-25 14:29"
---

<!-- skill-link: process-pr-review -->

# PR #3049 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3049.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A (pre-merge review).

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

Round 1: CodeRabbit review 6058202792 (F1, F2), answered before the audit record existed.
Round 2: CodeRabbit review 6058425125 on the head of commit `docs(issues): [#2904] cite commits by subject in the task reviewer reply files` (F3) and `da2ce7`'s review 6058630615 on the head of commit `docs(issues): [#2904] reopen AC10 until scenario M5 has its reply files` (F4 to F12). F6 repeats F3. F11 is undecided: it waits on the maintainer's answer about the D17 exact-path-shape consequence. The two reviews of `josecelano-bot` (6058508511, 6058509193) carry the replies of round 1 and no finding.

Round 3: `claude[bot]` review 6058972240 (F14), CodeRabbit review 6059084436 (F15 to F17) and `da2ce7`'s review 6059158630 (F18 to F21). F13 is the nit of review 6058630615's body that round 2 did not record. F19 repeats F14 and F21 repeats F13. F17 is declined with a reason and its thread stays open for the reviewer.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3049-f1 | coderabbitai[bot] | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3049-f2 | coderabbitai[bot] | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3049-f3 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3049-f4 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3049-f5 | da2ce7 | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3049-f6 | da2ce7 | Minor | documentation | RE_RAISE_OF:F3 | FIXED | RESOLVED |
| F7 | review-finding:pr-3049-f7 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3049-f8 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3049-f9 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-3049-f10 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-3049-f11 | da2ce7 | Suggestion | correctness | ORIGINAL | FOLLOW_UP | OPEN |
| F12 | review-finding:pr-3049-f12 | da2ce7 | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-3049-f13 | da2ce7 | Nit | maintainability | ORIGINAL | NO_ACTION | NON_RESOLVABLE |
| F14 | review-finding:pr-3049-f14 | claude[bot] | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-3049-f15 | coderabbitai[bot] | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-3049-f16 | coderabbitai[bot] | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-3049-f17 | coderabbitai[bot] | Minor | correctness | ORIGINAL | NO_ACTION | OPEN |
| F18 | review-finding:pr-3049-f18 | da2ce7 | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-3049-f19 | da2ce7 | Suggestion | correctness | RE_RAISE_OF:F14 | FIXED | RESOLVED |
| F20 | review-finding:pr-3049-f20 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-3049-f21 | da2ce7 | Nit | maintainability | RE_RAISE_OF:F13 | NO_ACTION | NON_RESOLVABLE |

## Finding Details

### F1 - Define the overall result of `review-task` when a criterion is pending

- PR number: 3049
- Source review ID: 6058202792
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695361559
- Concern: "Output" item 6 of `.claude/skills/review-task/SKILL.md` defined `REVIEW PASSED` only as every required criterion passing, which contradicts the task-reviewer practice of leaving a criterion `PENDING` with a reason and still returning `REVIEW_PASSED`.
- Solution: Item 6 now says `REVIEW PASSED` when no required criterion fails and no blocking issue remains, and that a criterion left `PENDING` for evidence from a later task is listed with the reason and does not by itself fail the review; step 9 reports the overall result instead of pass/fail.
- Current-tree verification: read the skill after the edit; `grep` for the old wording in `.claude/agents/task-reviewer.md` and `contrib/dev-tools/checks` found no copy (the agent file already says "the overall result"); the commit passed the pre-commit hook.
- Resolution reference: docs(skills): [#2904] define the review-task result when a criterion is pending
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695609915

### F2 - AC10 ticked done although scenario M5 has no reply files

- PR number: 3049
- Source review ID: 6058202792
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695361573
- Concern: AC10 of `ISSUE.md` was ticked and its acceptance-verification row was DONE, but it also covers scenario M5 (task T9), which has not produced its reply files.
- Solution: The checkbox is back to `[ ]` and the row to `TODO`, with a note that the M7 evidence (section V7 of `manual-verification-evidence.md`) is recorded and M5 is pending. AC11 is unchanged; a Progress Log entry records the fix.
- Current-tree verification: read AC10 and its row in `ISSUE.md` after the edit; the commit passed the pre-commit hook.
- Resolution reference: docs(issues): [#2904] reopen AC10 until scenario M5 has its reply files
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695610470

### F3 - The `external-protocol` override must cover section 5

- PR number: 3049
- Source review ID: 6058425125
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695541075
- Concern: Section 5 of the CLI output contract forbids plain text on stdout and stderr, while the `agent-reply-guard` row requires a plain-text refusal on stderr; the amendment note and the class definition lifted only sections 1 and 2.
- Solution: Both now say sections 1, 2 and 5; the class definition adds that section 5 stays binding for every other class.
- Current-tree verification: read both places in `docs/adrs/20260519000000_define_global_cli_output_contract.md` after the edit; the new ADR's section 2 already names the replaced parts by topic (the content of stdout and stderr), so it needed no change; the commit passed the pre-commit hook.
- Resolution reference: docs(adrs): [#2904] extend the external-protocol override to section 5 of the CLI output contract
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695984260

### F4 - No audit record at this head for the two answered threads

- PR number: 3049
- Source review ID: 6058630615
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695708405
- Concern: The progress-log entry says two findings were answered and resolved, but `docs/pr-reviews/pr-3049-review/PR-REVIEW.md` did not exist at the reviewed head.
- Solution: The record was created by the previous handler round, after the reviewed head, with rows F1 and F2; this round extends it. No further edit was needed for this finding.
- Current-tree verification: `docs/pr-reviews/pr-3049-review/PR-REVIEW.md` exists on the branch with F1 and F2 and their reply URLs; the validator exits 0 on it (run after the replies of this round are recorded).
- Resolution reference: docs(pr-reviews): [#2904] record the audit of the first review round on pull request 3049
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695984803

### F5 - The review-task result passes any `PENDING` criterion

- PR number: 3049
- Source review ID: 6058630615
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695708415
- Concern: `REVIEW PASSED` needed only that no required criterion fails, so a criterion that is `PENDING` for a reason other than deferred evidence passed too; the inventory row in `removed-profile-rules.md` said the profile's rule moved unchanged.
- Solution: Item 6 of `review-task` now says only a criterion left `PENDING` for evidence from a later task does not fail the review and any other `PENDING` required criterion gives `REVIEW FAILED`; the inventory row says the rule changed when it moved.
- Current-tree verification: read item 6 of the skill and the inventory row after the edit; `grep` for the old wording found no other copy; the agent file only says each `FAIL` or `PENDING` gets a remediation line.
- Resolution reference: docs(skills): [#2904] fail the review-task result on a pending criterion that is not deferred
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695985435

### F6 - The override names sections 1 and 2, but section 5 also forbids plain text

- PR number: 3049
- Source review ID: 6058630615
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695708424
- Concern: Same point as F3: the amendment note (line 22) and the class definition lift only sections 1 and 2.
- Solution: The same edit as F3; each thread still has its own row and reply.
- Current-tree verification: as F3: both places now name sections 1, 2 and 5.
- Resolution reference: docs(adrs): [#2904] extend the external-protocol override to section 5 of the CLI output contract
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695985961

### F7 - Section 3 of the new ADR quotes the row as a `Stop` hook only

- PR number: 3049
- Source review ID: 6058630615
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695708429
- Concern: The binary-table row names a `Stop` and a `PreToolUse` hook; section 3 of the new ADR named only the first.
- Solution: The sentence now names `Stop` (run as `SubagentStop`) and the `PreToolUse` hook on the tool that hands the report back.
- Current-tree verification: compared the sentence with the `agent-reply-guard` row in the amended ADR after the edit.
- Resolution reference: docs(adrs): [#2904] name the Stop and PreToolUse hooks in the external-protocol ADR and list its affected code
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695986693

### F8 - The guard's documentation links neither the hooks reference nor the ADR

- PR number: 3049
- Source review ID: 6058630615
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695708441
- Concern: The module documentation of `agent-reply-guard` did not link the hooks reference or the ADR, and the ADR had no Affected Code section (`create-adr` step 3.5).
- Solution: The module documentation links the ADR and the hooks reference; the ADR gets an Affected Code section listing the binary table, the program, `guard.rs` and the agent file. The ADR part is in the commit `docs(adrs): [#2904] name the Stop and PreToolUse hooks in the external-protocol ADR and list its affected code`.
- Current-tree verification: checked that the relative path in the module documentation resolves to the ADR file; read the new section; the commits passed the pre-commit hook.
- Resolution reference: docs(agent-reply-validator): [#2904] link the hooks reference and the ADR from the guard's module docs
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695987572

### F9 - M7 is `DONE` but V7 omits two of the four items its row says it records

- PR number: 3049
- Source review ID: 6058630615
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695708450
- Concern: The M7 row says V7 records which transcript message carries the new line and where the preloaded skill text sits; V7 did not.
- Solution: V7 now records both, read from the reviewer's own transcript of the second execution's run A (line 1 the first request, line 2 the preloaded skill text, line 103 the continued request with `origin.kind` `coordinator`); the row is unchanged.
- Current-tree verification: read the transcript `agent-<agent id>.jsonl` of that run with a short Python script that prints, per line, `type`, `isMeta`, `origin`, the content shape and the matches of `^Reply file: (.+)$`; the facts in V7 are those it printed.
- Resolution reference: docs(issues): [#2904] record in V7 which records carry the reply-file line and where the skill text sits
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695988671

### F10 - AC10's exceptions do not cover the continued run that ended without its file

- PR number: 3049
- Source review ID: 6058630615
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695708458
- Concern: V7 records that the continuation naming `task-reviewer-t5-r4.json` was refused 3 times and finished with no file, but AC10 listed only the forced run as an exception.
- Solution: AC10 (still unticked, pending scenario M5) now names that continuation as a second exception, with the reason (a name D18 rejects, a prompt mistake) and V7 as its evidence; its acceptance-verification row says the same. Not rerun: the run only showed the hook reading the new path, and a rerun would repeat that without changing the files.
- Current-tree verification: read AC10 and its row after the edit; V7 describes the run.
- Resolution reference: docs(issues): [#2904] name the continued M7 run as a second AC10 exception and update the timestamp
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695989256

### F11 - The guard holds an agent to a file name the validator rejects

- PR number: 3049
- Source review ID: 6058630615
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695708467
- Concern: `guard.rs` accepts any file name in `agent-replies/`, so a name the validator's name rule rejects keeps the agent refused until the limit; the reviewer proposes treating such a name like a non-exact path, with a message that names the problem.
- Solution: No change. It depends on the maintainer's answer about the D17 exact-path-shape consequence, which the pull request asks for; recorded as undecided and returned to the contributor as a question. The thread stays open.
- Current-tree verification: read the arm in `contrib/dev-tools/checks/agent-reply-validator/src/guard.rs`: it accepts any name under `agent-replies/`, as the reviewer says; V7 shows the three refusals for `task-reviewer-t5-r4.json`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695989904
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695989904

### F12 - `last-updated-utc` predates the newest progress-log entry

- PR number: 3049
- Source review ID: 6058630615
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695708477
- Concern: The frontmatter said `2026-11-24 13:40`, older than the entry of `2026-11-25 13:23`.
- Solution: The stamp is the output of `date -u` at the edit, which also adds this round's progress-log entry.
- Current-tree verification: read the frontmatter and the last progress-log entry after the edit; the stamp equals the entry's.
- Resolution reference: docs(issues): [#2904] name the continued M7 run as a second AC10 exception and update the timestamp
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695990612

### F13 - Commits without a body

- PR number: 3049
- Source review ID: 6058630615
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#pullrequestreview-6058630615
- Concern: Three commits of the branch have no body, and the review lists them as a nit.
- Solution: No change. No rule in the tree requires a body, the reviewer says so, and the branch history is kept as it is (`AGENTS.md`, Commit history); a commit cannot gain a body without rewriting history.
- Current-tree verification: Read `AGENTS.md` and the `commit-changes` skill: a Conventional Commit subject is required, a body is not.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3049#issuecomment-7165325317
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#issuecomment-7165325317

### F14 - A reply path in backticks or quotes is never enforced

- PR number: 3049
- Source review ID: 6058972240
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695980846
- Concern: `reply_file_in` trimmed only whitespace, so a path written as inline code kept its backticks, `classify` returned `Outside` and the hook let the reviewer finish with no reply file.
- Solution: The reader removes one matching pair of backticks or quotes around the path before the path is classified; the unit tests cover backticks, double and single quotes, a trailing space and a lone mark that stays. The agent file now says the line is written bare, because a whole line in inline code is still not read (an existing test keeps that behaviour).
- Current-tree verification: Read `reply_file_in` and `unwrap_pair` in `contrib/dev-tools/checks/agent-reply-validator/src/transcript.rs` after the edit; `cargo test -p agent-reply-validator` passed with the new cases and clippy was clean; the pre-commit hook passed.
- Resolution reference: fix(agent-reply-validator): [#2904] read a reply path written in backticks or quotes and tell the caller to write the line bare
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4696350026

### F15 - The round limit is not stated

- PR number: 3049
- Source review ID: 6059084436
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4696071031
- Concern: After round 3 the instructions imply `r4`, which the validator refuses; the skill states no limit and no recovery.
- Solution: The skill says the rounds go from 1 to 3, that the validator refuses a higher round, and that for a fourth review the reviewer writes no file, says so and leaves the caller to stop and ask the maintainer.
- Current-tree verification: Round limit read from `Round::MAX` in `contrib/dev-tools/checks/agent-reply-validator/src/name.rs`, whose tests refuse `r4`; read the paragraph in the skill after the edit; the pre-commit hook passed.
- Resolution reference: docs(skills): [#2904] state the round limit, keep the criteria matrix in the reply and say one kind of pending criterion
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4696350665

### F16 - The reply drops the acceptance-criteria matrix

- PR number: 3049
- Source review ID: 6059084436
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4696071042
- Concern: A passing reply can have an empty `findings` list and no mapping from criteria to evidence, although the earlier report recorded the matrix.
- Solution: The `evidence` field of the reply now also carries the matrix of Output item 2, one text per criterion with its result and evidence, passing criteria included. No schema change: `evidence` is a list of texts.
- Current-tree verification: Read the `evidence` bullet after the edit; the reply schema in the validator takes a list of texts for `evidence`, so the matrix validates; the pre-commit hook passed.
- Resolution reference: docs(skills): [#2904] state the round limit, keep the criteria matrix in the reply and say one kind of pending criterion
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4696351189

### F17 - An unreadable transcript lets the agent finish unvalidated

- PR number: 3049
- Source review ID: 6059084436
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4696071057
- Concern: When the guard cannot read the transcript it exits 1, which does not hold the agent; the reviewer asks that a task with a named reply file stays held, with a count, and the tests changed.
- Solution: No change. The ADR of this pull request decides it: failures of the program itself (unreadable input, an unreadable transcript) are errors and not refusals, exit `1` and cannot keep an agent working; the caller then finds no valid reply file and stops (fails closed). Holding on an error the guard cannot attribute to a reply would also have no count to key on when the issue number cannot be read. Reversing it changes the decision and the tests together, so it is the maintainer's call. The thread stays open for the reviewer.
- Current-tree verification: Read the guard's `fail` path in `contrib/dev-tools/checks/agent-reply-validator/src/bin/agent-reply-guard/main.rs` and the sentence of the new ADR (section 2) that says such failures exit 1 and hold nothing; the tests at the lines the reviewer names assert exactly that.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4696351824
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4696351824

### F18 - The audit record lags the forge

- PR number: 3049
- Source review ID: 6059158630
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4696122714
- Concern: At the reviewed head the record had `Reply URL: pending` on ten rows, a `pending` resolution reference, no rows for the thread of review 6058972240 and the body nit of review 6058630615, an old `last-updated-utc`, and a verification that claimed an exit 0 the validator could not give.
- Solution: The reply URLs, the resolution reference and the stamp were already recorded by the commit `docs(pr-reviews): [#2904] record the replies and resolved threads of the second review round on pull request 3049`, after the head the reviewer read. This round adds the missing rows (F13 and F14) and every row of round 3. Per the maintainer's rule, the commit that records the reply URLs of round 3 stays local and unpushed until he decides, so the pushed record has `pending` on this round's rows until then.
- Current-tree verification: Ran the validator at the head before this round: exit 0, 12 rows and 0 failures; the rows and log entries of this round are checked by it again after the replies.
- Resolution reference: docs(pr-reviews): [#2904] record the third review round on pull request 3049
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4696352434

### F19 - The reply-line reader drops a wrapped path or line

- PR number: 3049
- Source review ID: 6059158630
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4696122730
- Concern: A path in backticks keeps them and is classified outside, and a whole line in inline code, as the agent file showed it, is not found; the hook's three chances are lost.
- Solution: Same edit as F14 for the path; the agent file now says the line is written bare as plain text, the alternative the reviewer offers for the whole-line case.
- Current-tree verification: As F14; the agent file shows `Reply file: <path>` with the sentence that it is written bare and not as inline code.
- Resolution reference: fix(agent-reply-validator): [#2904] read a reply path written in backticks or quotes and tell the caller to write the line bare
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4696353236

### F20 - "Only one `PENDING` criterion" reads as a count

- PR number: 3049
- Source review ID: 6059158630
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4696122737
- Concern: The sentence can be read as at most one deferred criterion per review.
- Solution: It now reads "Only one kind of `PENDING` criterion does not fail the review".
- Current-tree verification: Read item 6 of the Output in `.claude/skills/review-task/SKILL.md` after the edit; no other copy of the old wording (`grep`).
- Resolution reference: docs(skills): [#2904] state the round limit, keep the criteria matrix in the reply and say one kind of pending criterion
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4696353768

### F21 - Commits without a body, again

- PR number: 3049
- Source review ID: 6059158630
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#pullrequestreview-6059158630
- Concern: The review body repeats the nit of F13: six of the nine new commits have no body, and no rule requires one.
- Solution: No change, for the reason of F13.
- Current-tree verification: As F13.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3049#issuecomment-7165325317
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#issuecomment-7165325317

## Processing Log

- 2026-11-25 13:25 UTC - Started audit for review `6058202792` of `coderabbitai[bot]`; the review `6058085652` of `claude[bot]` (review disabled for forks) and the CodeRabbit walkthrough comment `7163196060` carry no finding.
- 2026-11-25 13:25 UTC - Verified both findings against the current tree and committed the two fixes.
- 2026-11-25 13:30 UTC - Pushed the fixes and replied on both threads, then recorded the reply URLs.
- 2026-11-25 13:31 UTC - A refresh showed both threads already resolved (not by this session) after the replies; no resolve mutation was needed.
- 2026-11-25 13:49 UTC - Round 2: started on reviews `6058425125` of `coderabbitai[bot]` and `6058630615` of `da2ce7`, nine inline comments plus one; verified every finding against the current tree and committed the fixes (F3 to F10, F12). F11 is undecided: no code change.
- 2026-11-25 13:49 UTC - Correction: the `## Findings` introduction written in round 1 described findings F1 to F19 of another pull request's record (reviews 6045345701 to 6054114265) and not this one; it was replaced by one that describes rounds 1 and 2 of this record. The Processing Log entries above were not changed.
- 2026-11-25 14:00 UTC - Pushed the fixes and the record of this round, replied on the ten threads (GitHub `created_at` 13:59:39 to 14:00:06 UTC) and recorded the reply URLs; the validator exited 0.
- 2026-11-25 14:01 UTC - Resolved nine threads (F3 to F10 and F12) with the GraphQL mutation, each returning `isResolved: true`. F11 stays open as undecided, waiting on the maintainer.
- 2026-11-25 14:18 UTC - Round 3: started on reviews `6058972240` of `claude[bot]`, `6059084436` of `coderabbitai[bot]` and `6059158630` of `da2ce7`; verified every finding against the current tree and committed two fixes (F14 to F16, F19, F20). F17 is declined with a reason, F13 and F21 need no change, F18 is answered by the record itself.
- 2026-11-25 14:22 UTC - Pushed the two fixes and the record of this round with `git-bot-push`, no force.
- 2026-11-25 14:28 UTC - Replied on the seven threads (GitHub `created_at` 14:27:47 to 14:28:03 UTC) and with one conversation comment for the body nits F13 and F21 (14:28:10 UTC), then resolved five threads (F14, F16, F18 to F20) with the GraphQL mutation; the thread of F15 was already resolved by `coderabbitai[bot]`. F17 stays open for its reviewer and F11 for the maintainer.
- 2026-11-25 14:29 UTC - Recorded the reply URLs of this round; this commit is kept local and not pushed on its own, by the maintainer's rule.

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
