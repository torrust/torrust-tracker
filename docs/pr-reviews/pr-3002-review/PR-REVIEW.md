---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3002 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/3002>.

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

Round 1 covers three reviews of the head commit `docs(issues): [#2904] record how the stop hook of the task reviewer works`, all COMMENTED. Review 6022610166 (CodeRabbit, submitted 2026-11-23T13:35:49Z) has three inline comments, F1 to F3; its body is a summary and a prompt that repeats them, and makes no row. Its comments carry a "Major" or "Minor" label, not a `[Severity]` bracket; the labels are recorded as severities. Review 6022825332 (`da2ce7`, submitted 13:50:57Z) has five inline comments with `[Severity][FindingId]` first lines, F1 to F5; its body is a verdict and a checks summary and makes no row. Its IDs collide with F1 to F3, so they are F4 to F8 here, in source order, with the reviewer's ID in each detail entry; its F1 and F2 corroborate the CodeRabbit threads and are re-raises of F1 and F3 that add evidence, fixed by the same commits. Review 6022913275 (`copilot-pull-request-reviewer`, submitted 13:57:02Z, a lite review) has three inline comments, F9 to F11; its body is an overview that repeats them, with a warning that the full agentic review did not start before the timeout, and makes no row. The overview marks all three Medium; they are recorded as `Minor (inferred)`. F9 and F11 are the same text on two lines and ask for the change of F7.

Round 2 covers review 6023199677 (Greptile, submitted 2026-11-23T14:15:50Z, COMMENTED) on the intermediate head commit `docs(pr-reviews): [#3002] record review round 1`, whose audit record still had placeholder Reply URLs. It has one inline comment, F12, marked P2 and recorded as `Minor (inferred)`; its body repeats it and makes no row. The next commit had already filled the URLs, so F12 needed no change.

Commit ids in posted replies: this record names commits by subject only. Two replies posted before that rule was applied cite a commit by its abbreviated id, and a reply cannot be edited. In both, the id is that of the commit `docs(pr-reviews): [#3002] record review round 1`: the F1 reply (<https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666976606>, "Checked at ...") and the F12 reply (<https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4667230477>, "The placeholders you saw at ...").

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3002-f1 | Unknown | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3002-f2 | Unknown | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3002-f3 | Unknown | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3002-f4 | Human | Minor | correctness | RE_RAISE_OF:F1 | FIXED | RESOLVED |
| F5 | review-finding:pr-3002-f5 | Human | Minor | correctness | RE_RAISE_OF:F3 | FIXED | RESOLVED |
| F6 | review-finding:pr-3002-f6 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3002-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3002-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3002-f9 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F7 | NO_ACTION | SUPERSEDED |
| F10 | review-finding:pr-3002-f10 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-3002-f11 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F7 | NO_ACTION | SUPERSEDED |
| F12 | review-finding:pr-3002-f12 | Unknown | Minor (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - A continued reviewer is held to the first request's reply file

- PR number: 3002
- Source review ID: 6022610166
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666536345
- Concern: A caller can continue a finished reviewer with a new task. D17 read the `Reply file:` line from the first transcript message, so a continued reviewer whose new task names a reply file could finish on the old task. CodeRabbit asks to read the current request or to require a fresh reviewer per review.
- Solution: Applied the maintainer's decision of 2026-11-23: the caller writes the line in every request, a continuation included, naming the new round's file (D18), and the hook reads the line from the last message from the caller that carries one. The hook tells the caller's messages apart from tool results and from its own refusal text, which never contains `Reply file:`, and its tests cover a continued run.
- Current-tree verification: `grep -c` on `docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/ISSUE.md` finds "last message from the caller that carries one" in D17 (and in the new Progress Log entry), "refusal text never contains `Reply file:`" once, and "first message of the agent" 0 times.
- Resolution reference: `docs(issues): [#2904] hold a continued task reviewer to its latest reply file`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666976606

### F2 - A continued run inherits the earlier run's refusal count

- PR number: 3002
- Source review ID: 6022610166
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666536360
- Concern: A continued agent keeps its `agent_id`, so the count file named after it carries the earlier run's refusals and the new run can pass without its own 3 refusals. CodeRabbit asks for a run-specific key or a reset policy.
- Solution: Applied the maintainer's decision of 2026-11-23: one count file per request, keyed by the `agent_id` together with the reply file (for example a file named from both). A continued request names a new round file under D18, so it gets its own 3 refusals. The Requirements paragraph says "3 refusals per request, that is per run of the agent and reply file".
- Current-tree verification: `grep -c` on `docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/ISSUE.md` finds "keyed by the run's `agent_id` together with" once and "3 refusals per request" once.
- Resolution reference: `docs(issues): [#2904] hold a continued task reviewer to its latest reply file`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666977020

### F3 - A refused draft reply path has no counter location

- PR number: 3002
- Source review ID: 6022610166
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666536364
- Concern: A draft `Reply file:` path has no issue number, but the counter lives under `.agents/issues/<issue-number>/`, so the refusal of a draft path has no counter and the 3-refusal limit cannot hold. CodeRabbit asks for a fallback counter location.
- Solution: Applied the maintainer's decision of 2026-11-23 instead of a fallback counter: for a path under `docs/issues/drafts/` the hook lets the reviewer finish with a message saying the reply was not enforced and why (a draft has no implemented task to review), and keeps no count. The issue number is taken only from a path under `docs/issues/open/` or `docs/issues/closed/`.
- Current-tree verification: `grep -c` on `docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/ISSUE.md` finds the bullet "A draft specification" once, "taken only from a path under" once, and "so the hook refuses it" 0 times.
- Resolution reference: `docs(issues): [#2904] let the task reviewer finish on a draft reply path`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666977377

### F4 - A continued reviewer keeps the first task's reply file and refusal count

- PR number: 3002
- Source review ID: 6022825332
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666708327
- Concern: The same concern as F1 and F2, raised by `da2ce7` with the spec's own rows (a continued agent keeps its history; D18 writes a new file per call): the hook reads the first message's line, which names an earlier valid file, and the count file named after the unchanged `agent_id` carries the earlier refusals, against "3 refusals per run".
- Solution: Fixed by the commit of F1 and F2: the hook reads the last caller message that carries the line, and the count is keyed by the `agent_id` and the reply file, per request.
- Current-tree verification: As for F1 and F2.
- Resolution reference: `docs(issues): [#2904] hold a continued task reviewer to its latest reply file`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666977884

### F5 - A refused draft path has no counter, and the draft folder number is unreliable

- PR number: 3002
- Source review ID: 6022825332
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666708337
- Concern: The concern of F3, plus a second point: the stated reason "has no issue number" is loose, because 11 of the 18 draft folders start with a number (their parent's), and "the hook cannot loop forever" rested on the cap of eight, which is stated only for `Stop`.
- Solution: Fixed by the commit of F3. The draft bullet says the issue number is never taken from a draft path, because draft folders often start with their EPIC's number, and gives the count and an example. A draft path is no longer refused, so the bound does not depend on the cap of eight (F7).
- Current-tree verification: Before writing the count, `ls -d docs/issues/drafts/*/ | wc -l` printed 18 and `ls -d docs/issues/drafts/[0-9]*/ | wc -l` printed 11; `docs/issues/drafts/2003-separate-ai-harness-cargo-workspace` exists. `grep -c` on the spec finds "never taken" once.
- Resolution reference: `docs(issues): [#2904] let the task reviewer finish on a draft reply path`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666978395

### F6 - The plan does not carry the verification D17 promises

- PR number: 3002
- Source review ID: 6022825332
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666708347
- Concern: D17 says the hook's tests pin the transcript format and that M7 shows the hook reads the `Reply file:` line, but the T5 row, Commit Points and Automatic Checks name only tests of the 3-refusal limit, M7's steps do not say the task carries the line, and the let-finish branch has no test.
- Solution: The T5 row, the Commit Points test sentence and Automatic Checks name the tests of the limit of 3 refusals, of reading the latest `Reply file:` line from a transcript including a continued run, and of the let-finish branches without a line and for a draft path. M7's steps say the task carries the `Reply file:` line (D17).
- Current-tree verification: `grep -c` on `docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/ISSUE.md` finds "carries the `Reply file:` line (D17)" once (M7), "letting the agent finish without a line" twice (Commit Points, Automatic Checks), and "including a continued run" 4 times (D17, the T5 row, Commit Points, Automatic Checks).
- Resolution reference: `docs(issues): [#2904] name the stop hook's tests in the plan and checks`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666978919

### F7 - The cap of eight is stated for stop hooks in general

- PR number: 3002
- Source review ID: 6022825332
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666708364
- Concern: D17 states the cap of eight continuations as documented for stop hooks in general, while the facts row says the page states it only for `Stop` and is silent on `SubagentStop`.
- Solution: The sentence now says that the bound of 3 does not rely on the cap, that the hooks page states the cap for `Stop` and does not say whether it applies to `SubagentStop`, and that the bound is below the cap in either case.
- Current-tree verification: `grep -c` on `docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/ISSUE.md` finds "documents for stop hooks" 0 times and "does not rely on Claude Code's cap" once.
- Resolution reference: `docs(issues): [#2904] state the cap of eight continuations only for Stop hooks`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666979362

### F8 - The facts introduction dates every row 2026-11-15

- PR number: 3002
- Source review ID: 6022825332
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666708375
- Concern: The facts table's introduction says every fact was checked on 2026-11-15 and names that day's installed version, while the two new rows say they were checked on 2026-11-23 and give no version.
- Solution: The introduction names the two rows that start with "Checked on 2026-11-23" as checked on that day, and says that `claude --version` still printed 2.1.318 on 2026-11-23.
- Current-tree verification: `claude --version` run at 13:58 UTC on 2026-11-23 printed `2.1.318 (Claude Code)`. `grep -c` on the spec finds "Checked on 2026-11-23", which" once and "still printed" once.
- Resolution reference: `docs(issues): [#2904] date the two facts rows of 2026-11-23 in the table's introduction`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666979916

### F9 - Same cap-of-eight wording comment, anchored on the facts row

- PR number: 3002
- Source review ID: 6022913275
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666779318
- Concern: Copilot notes that the facts row states the cap only for `Stop` while D17 treats it as applying to stop hooks generally, and asks to reword D17. The same request as F7.
- Solution: No separate change: the D17 sentence was reworded under F7.
- Current-tree verification: As for F7.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666980445
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666980445

### F10 - `agent_transcript_path` reads as the transcript content, not a path

- PR number: 3002
- Source review ID: 6022913275
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666779386
- Concern: D17 said the hook reads the line "from the first message of the agent's transcript, `agent_transcript_path`", which reads as if the field were the transcript text; Copilot asks to say the hook reads the transcript file at that path.
- Solution: Fixed in the rewrite of the "Which file" bullet (F1): the hook opens the transcript file at `agent_transcript_path` and reads the line from the last message from the caller that carries one.
- Current-tree verification: `grep -c` on `docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/ISSUE.md` finds "opens the transcript file at `agent_transcript_path`" once.
- Resolution reference: `docs(issues): [#2904] hold a continued task reviewer to its latest reply file`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666980835

### F11 - Same cap-of-eight wording comment, on the D17 sentence

- PR number: 3002
- Source review ID: 6022913275
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666779451
- Concern: The same text as F9, anchored on the D17 sentence itself. The same request as F7.
- Solution: No separate change: the D17 sentence was reworded under F7.
- Current-tree verification: As for F7.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666981257
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4666981257

### F12 - The committed audit record has placeholder Reply URLs

- PR number: 3002
- Source review ID: 6023199677
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4667004905
- Concern: At the head commit `docs(pr-reviews): [#3002] record review round 1` every `Reply URL` field of F1 to F11 held a placeholder, so `validate-audit-record.py` could not pass.
- Solution: No further change: the rows were committed before the replies, as the workflow orders, and the next commit filled the eleven URLs after the replies were posted.
- Current-tree verification: `grep -c "^- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r"` on this file printed 11 before this row was added, and `grep -i pending` printed nothing; the validator exited 0 (11 rows, 0 failures).
- Resolution reference: `docs(pr-reviews): [#3002] record the replies and resolutions of review round 1`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4667230477

## Processing Log

- 2026-11-23 14:05 UTC - Started audit. Round 1: reviews 6022610166 (CodeRabbit), 6022825332 (`da2ce7`) and 6022913275 (Copilot) on the head commit `docs(issues): [#2904] record how the stop hook of the task reviewer works`, eleven inline comments recorded as F1 to F11. Fetched the eleven review threads with GraphQL as `josecelano-bot`: all unresolved, `viewerCanResolve` true for each. Applied the maintainer's two design answers of 2026-11-23 (F1, F2, F4; F3, F5) and the mechanical fixes in five signed commits (committer times 13:59:23Z to 14:03:03Z) and logged the round in the spec (14:03:55Z); each commit's pre-commit hook passed. Checked each disposition against the current tree with the commands in the detail entries. Not pushed yet.
- 2026-11-23 14:14 UTC - Pushed the seven commits with `git-bot-push` (pre-push hook passed); `git ls-remote josecelano-bot refs/heads/2904-t5-task-reviewer-spec` shows the commit `docs(pr-reviews): [#3002] record review round 1`. Replied on the eleven threads as `josecelano-bot` (reply `created_at` 14:13:28Z to 14:13:52Z) and recorded the reply URLs.
- 2026-11-23 14:14 UTC - The copy of the audit validator that calls `gh-bot` exits 0 (11 rows, 0 failures). A GraphQL fetch showed every unresolved thread with one reply of `josecelano-bot` and `viewerCanResolve` true; the F3 thread had already been resolved by the CodeRabbit bot after the reply. The other ten threads were then resolved as `josecelano-bot`. A new GraphQL fetch shows 11 threads, 0 unresolved.
- 2026-11-23 14:22 UTC - Round 2: review 6023199677 (Greptile) on the head commit `docs(pr-reviews): [#3002] record review round 1`, one inline comment recorded as F12. Checked against the current tree: the Reply URLs it asks for were filled by the next commit, so no change. Row committed before the reply.
- 2026-11-23 14:34 UTC - Pushed the F12 row with `git-bot-push` (pre-push hook passed); `git ls-remote josecelano-bot refs/heads/2904-t5-task-reviewer-spec` showed the commit `docs(pr-reviews): [#3002] record review round 2 before replying`. Replied on the F12 thread as `josecelano-bot` and resolved it (`isResolved` true).
- 2026-11-23 16:01 UTC - Correction for round 3 (finding F15 below): in the two round paragraphs, the F12 concern and the four entries above stamped 14:05, 14:14, 14:22 and 14:34, each abbreviated commit id was replaced by that commit's subject; nothing else in them changed. The ids of the two posted replies that cite one are mapped to a commit subject after the round paragraphs. The `open-pull-request` commit-id check, run on this file and the spec, prints nothing.

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
