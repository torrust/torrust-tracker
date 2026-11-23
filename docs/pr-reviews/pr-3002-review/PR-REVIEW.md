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

Round 3 covers five reviews. Review 6023349775 (`da2ce7`, submitted 2026-11-23T14:25:45Z, CHANGES_REQUESTED) and review 6023402273 (`copilot-pull-request-reviewer`, 14:29:09Z, COMMENTED) are on the head commit `docs(pr-reviews): [#3002] record the replies and resolutions of review round 1`; review 6023553046 (CodeRabbit, 14:38:58Z, COMMENTED) and review 6023574266 (`da2ce7`, 14:40:16Z, CHANGES_REQUESTED) are on the head commit `docs(pr-reviews): [#3002] record review round 2 before replying`; review 6023654876 (`da2ce7`, 14:45:11Z, CHANGES_REQUESTED) is on the head commit `docs(pr-reviews): [#3002] record the reply and resolution of review round 2`. They have thirteen inline comments, F13 to F25 here in source order. The `da2ce7` comments carry `[Severity][FindingId]` first lines whose IDs (F6, F7, F8, F10, F11; F12, F13, F14; F15, F16) collide with this record's, so each detail entry keeps the reviewer's ID; the Copilot comment carries `[Minor][F12]`, which collides too. The CodeRabbit comments carry a "Major" or "Minor" label, recorded as the severity. The `da2ce7` review bodies are verdicts, checks summaries and the status of earlier findings (one of them says its F9 of an earlier round is fixed; it has no inline comment and makes no row); the Copilot body is an overview of its one comment; the CodeRabbit body only counts its comments. None makes a row. Reviews 6023191005 and 6023194907 (CodeRabbit, 14:15:17Z and 14:15:32Z) are its replies on the F1 and F2 threads, which accept the fixes and request nothing; they make no row. The contributor answered the design questions of this round on 2026-11-23 (a continuation without a line, preloaded skill text, a path outside the three places, the M7 step, the PR description and the commit ids); the rows apply those answers.

Round 4 covers four reviews. Review 6025120305 (CodeRabbit, submitted 2026-11-23T16:16:10Z), review 6025142141 (`da2ce7`, 16:17:34Z) and review 6025374584 (`copilot-pull-request-reviewer`, 16:32:40Z) are on the head commit `docs(pr-reviews): [#3002] record review round 3`; review 6025460812 (`da2ce7`, 16:38:05Z) is on the head commit `docs(pr-reviews): [#3002] record the replies and resolutions of review round 3`. All are COMMENTED. They have twenty-two inline comments, F26 to F47 here in source order. The `da2ce7` comments carry `[Severity][FindingId]` first lines whose IDs (F17, F18, F19; F26, F27) collide with this record's, so each detail entry keeps the reviewer's ID. The CodeRabbit comment carries a "Minor" label, recorded as the severity. The Copilot comments carry no severity; its overview marks the thirteen placeholder threads High, the log-entry thread Medium and the two long-line threads Low, recorded as `Major (inferred)`, `Minor (inferred)` and `Nit (inferred)`. The CodeRabbit body is a count of its comment and a prompt that repeats it; the `da2ce7` bodies are verdicts, checks summaries and the status of earlier findings; the Copilot body is an overview of its comments, which also lists the resolved thread of F18. None makes a row. Reviews 6025149790 and 6025153395 (CodeRabbit, 16:18:05Z and 16:18:18Z) are its replies on the resolved F19 and F20 threads: they note the `PENDING` Reply URLs of F13 to F25, which the commit `docs(pr-reviews): [#3002] record the replies and resolutions of review round 3` filled, take no resolution action and request nothing new; they make no row. Two reviews arrived on the head commit `docs(issues): [#2904] log the fourth review round of pull request 3002` while the round's replies were posted: review 6025904627 (CodeRabbit, 17:06:45Z) is its reply on the F26 thread, which keeps one point of F26 and is recorded as F48; review 6025931039 (`da2ce7`, 17:08:28Z, COMMENTED) reports no new finding and the status of F17, F18, F26 and F27, and makes no row. The contributor answered the questions of this round on 2026-11-23 (the missing reply folder, the description sentence, the F19 count, the old log entry and the long lines); the rows apply those answers.

Round 5 covers two reviews. Review 6026236623 (`copilot-pull-request-reviewer`, submitted 2026-11-23T17:25:26Z, COMMENTED) is on the intermediate head commit `docs(issues): [#2904] log the fourth review round of pull request 3002`, which did not yet contain the round-4 rows; it has one inline comment, F49, whose first line carries `[Minor][F46]`, an ID that collides with this record's. Review 6026291754 (`da2ce7`, 17:28:36Z, APPROVED) is on the head commit `docs(pr-reviews): [#3002] cite the review of F48 as its source`; it has one inline comment, F50 (`[Nit][F28]`). Neither body makes a row: the Copilot body is an overview of its comment, and the `da2ce7` body is the verdict and the status of earlier findings.

Round 6 covers review 6026507151 (`da2ce7`, submitted 2026-11-23T17:41:34Z, COMMENTED) on the head commit `docs(pr-reviews): [#3002] say that no field holds a placeholder instead of no PENDING text`, which did not yet contain the round-5 rows. It has one inline comment, F51 (`[Minor][F49]`); its body is the verdict and the status of earlier findings and makes no row. The issue comment of `da2ce7` at 17:28:40Z is an ACK of the head commit `docs(pr-reviews): [#3002] cite the review of F48 as its source` and makes no row.

Round 7 covers two reviews of the head commit `docs(pr-reviews): [#3002] record review round 5`. Review 6026731021 (`da2ce7`, submitted 2026-11-23T17:55:28Z) approved it and was dismissed when the next commit was pushed; it has no inline comment and its body is the verdict, so it makes no row. Review 6026746874 (`copilot-pull-request-reviewer`, 17:56:29Z, COMMENTED) has one inline comment, F52, whose first line carries `[Minor][F51]`, an ID that collides with this record's; its body is an overview of that comment and makes no row. The issue comments of the maintainer (17:50:12Z) and of `da2ce7` (17:55:31Z) are ACKs of the same head and make no row. The maintainer chose the fix of F52 on 2026-11-23: a scenario that observes the transcript format before any of T5's code.

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
| F13 | review-finding:pr-3002-f13 | Human | Major | metadata | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-3002-f14 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-3002-f15 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-3002-f16 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-3002-f17 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-3002-f18 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-3002-f19 | Unknown | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-3002-f20 | Unknown | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-3002-f21 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-3002-f22 | Human | Minor | metadata | RE_RAISE_OF:F20 | FIXED | RESOLVED |
| F23 | review-finding:pr-3002-f23 | Human | Minor | correctness | RE_RAISE_OF:F18 | FIXED | RESOLVED |
| F24 | review-finding:pr-3002-f24 | Human | Minor | testing | RE_RAISE_OF:F19 | FIXED | RESOLVED |
| F25 | review-finding:pr-3002-f25 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F26 | review-finding:pr-3002-f26 | Unknown | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F27 | review-finding:pr-3002-f27 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F28 | review-finding:pr-3002-f28 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F29 | review-finding:pr-3002-f29 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F30 | review-finding:pr-3002-f30 | Copilot | Major (inferred) | metadata | RE_RAISE_OF:F29 | FIXED | RESOLVED |
| F31 | review-finding:pr-3002-f31 | Copilot | Major (inferred) | metadata | RE_RAISE_OF:F29 | FIXED | RESOLVED |
| F32 | review-finding:pr-3002-f32 | Copilot | Major (inferred) | metadata | RE_RAISE_OF:F29 | FIXED | RESOLVED |
| F33 | review-finding:pr-3002-f33 | Copilot | Major (inferred) | metadata | RE_RAISE_OF:F29 | FIXED | RESOLVED |
| F34 | review-finding:pr-3002-f34 | Copilot | Major (inferred) | metadata | RE_RAISE_OF:F29 | FIXED | RESOLVED |
| F35 | review-finding:pr-3002-f35 | Copilot | Major (inferred) | metadata | RE_RAISE_OF:F29 | FIXED | RESOLVED |
| F36 | review-finding:pr-3002-f36 | Copilot | Major (inferred) | metadata | RE_RAISE_OF:F29 | FIXED | RESOLVED |
| F37 | review-finding:pr-3002-f37 | Copilot | Major (inferred) | metadata | RE_RAISE_OF:F29 | FIXED | RESOLVED |
| F38 | review-finding:pr-3002-f38 | Copilot | Major (inferred) | metadata | RE_RAISE_OF:F29 | FIXED | RESOLVED |
| F39 | review-finding:pr-3002-f39 | Copilot | Major (inferred) | metadata | RE_RAISE_OF:F29 | FIXED | RESOLVED |
| F40 | review-finding:pr-3002-f40 | Copilot | Major (inferred) | metadata | RE_RAISE_OF:F29 | FIXED | RESOLVED |
| F41 | review-finding:pr-3002-f41 | Copilot | Major (inferred) | metadata | RE_RAISE_OF:F29 | FIXED | RESOLVED |
| F42 | review-finding:pr-3002-f42 | Copilot | Major (inferred) | metadata | RE_RAISE_OF:F29 | FIXED | RESOLVED |
| F43 | review-finding:pr-3002-f43 | Copilot | Minor (inferred) | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |
| F44 | review-finding:pr-3002-f44 | Copilot | Nit (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F45 | review-finding:pr-3002-f45 | Copilot | Nit (inferred) | formatting | ORIGINAL | FIXED | RESOLVED |
| F46 | review-finding:pr-3002-f46 | Human | Minor | correctness | RE_RAISE_OF:F26 | FIXED | RESOLVED |
| F47 | review-finding:pr-3002-f47 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F48 | review-finding:pr-3002-f48 | Unknown | Minor (inferred) | correctness | RE_RAISE_OF:F26 | NO_ACTION | SUPERSEDED |
| F49 | review-finding:pr-3002-f49 | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F50 | review-finding:pr-3002-f50 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F51 | review-finding:pr-3002-f51 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F52 | review-finding:pr-3002-f52 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |

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

### F13 - The PR description describes the replaced design

- PR number: 3002
- Source review ID: 6023349775
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4667121510
- Concern: The author-written part of the pull request description was unchanged since the first head: it begins "One commit" and says the hook reads the line from the first message of the transcript, that the count is one file per run, and that a draft reply path is refused. The merge tool copies the description into the merge commit, so a merge would put the replaced design into permanent history. `da2ce7` asks to update it to the design at the head.
- Solution: Applied the contributor's answer of 2026-11-23: the description was rewritten with `gh-bot pr edit 3002 --body-file` to the design at the head, with no commit count and no commit id: the latest-line rule, the per-request count keyed by the `agent_id` and the reply file, the issue number only from `open/` or `closed/`, the let-finish branches, preloaded skill text, the M7 continued-run step, the binary `agent-reply-guard`, the two facts rows, and this audit record as part of the pull request.
- Current-tree verification: After the edit, `grep -c` on the body that `gh-bot pr view 3002 --json body` returned printed 0 for "One commit", "first message", "One file per run" and "is refused", and the body was the edited file apart from a trailing newline.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668482269
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668482269

### F14 - Preloaded skill text can carry the reply-file line

- PR number: 3002
- Source review ID: 6023349775
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4667121532
- Concern: The hook reads the last caller message that carries the `Reply file:` line, and V1 records the preloaded skills' text right after the task in the transcript. `task-reviewer` preloads `review-task`, where T5 writes the reply rule; if that rule shows the line literally, the last message carrying it is the skill's example. D17 named tool results and the refusal text as non-caller text, not preloaded skills.
- Solution: Applied the contributor's answer: both of the reviewer's options. D17 names preloaded skill text with tool results and the refusal text, and says that neither the refusal text nor `review-task` ever shows the literal line, both referring to the reply-file line of D17. The T5 row says the reply rule refers to that line without showing it literally, and T5's tests include a transcript where skill text follows the task.
- Current-tree verification: `grep -c` on `docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/ISSUE.md` finds "preloaded skills and from its own refusal text" once, "ever shows the literal line" once, "without showing it literally" once and "skill text that follows the task" 3 times (the T5 row, the Commit Points and Automatic Checks).
- Resolution reference: `docs(issues): [#2904] keep the reply file line out of preloaded skill text`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668482927

### F15 - The record and two replies cite PR-branch commit ids

- PR number: 3002
- Source review ID: 6023349775
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4667121546
- Concern: The `open-pull-request` commit-id check printed PR-branch commit ids in this record: the round paragraphs, the F12 concern and the Processing Log entries named heads by id, and the F1 reply cites one. In its later reviews `da2ce7` widened the finding to a third id in the round-2 log entry and to the F12 reply.
- Solution: Applied the contributor's answer: every PR-branch commit id in this record was replaced by the commit's subject, with a correction entry in the Processing Log, and the two posted replies that cite an id are mapped to the commit subject after the round paragraphs. New text and replies of this round name commits by subject only.
- Current-tree verification: The `open-pull-request` commit-id check, with `torrust/develop` fetched, prints nothing for this file and the spec.
- Resolution reference: `docs(pr-reviews): [#3002] name commits by subject instead of commit id`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668483549

### F16 - A reply path outside open/, closed/ and drafts/ has no stated behaviour

- PR number: 3002
- Source review ID: 6023349775
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4667121583
- Concern: A `Reply file:` path outside `docs/issues/open/`, `closed/` and `drafts/` had no stated behaviour: the validator rejects its place, so the hook would refuse it with no issue number to key the count, which is the round-1 draft case for a mistyped path.
- Solution: Applied the contributor's answer: the reviewer cannot change the path the caller wrote, so the hook lets it finish, as for a draft, with a message that names the path and says it is outside the places D18 allows, and keeps no count. The caller then finds no valid reply file and stops. The T5 row, the Commit Points and Automatic Checks name the test of the branch.
- Current-tree verification: `grep -c` on `docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/ISSUE.md` finds "A path outside the specification folders" once, "outside the places D18 allows" once, and "for a path outside the specification folders" twice (the T5 row and the Commit Points; the Automatic Checks item wraps the phrase).
- Resolution reference: `docs(issues): [#2904] let the task reviewer finish on a reply path outside the specification folders`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668484157

### F17 - The Greptile thread had no reply, row or resolution

- PR number: 3002
- Source review ID: 6023349775
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4667121559
- Concern: The thread opened by Greptile at 14:15:49 UTC on the head commit `docs(pr-reviews): [#3002] record review round 1` had no reply, row or resolution when `da2ce7` reviewed; the reviewer notes the concern itself was already fixed.
- Solution: No further change: round 2 added the row F12, replied on that thread and resolved it, and the next commit recorded the reply URL.
- Current-tree verification: The F12 detail entry has its Reply URL on the thread of `discussion_r4667004905`; a GraphQL fetch as `josecelano-bot` before this round showed that thread resolved with the reply.
- Resolution reference: `docs(pr-reviews): [#3002] record the reply and resolution of review round 2`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668484834

### F18 - A continuation without a folder-style specification inherits the earlier line

- PR number: 3002
- Source review ID: 6023402273
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4667163452
- Concern: If a reviewer handles a folder-style issue and is then continued for a task without one, the lookup selects the earlier request's line, so the no-line branch cannot apply and the hook checks the earlier reply and count. Copilot asks for a request boundary or a no-reply marker, and a test.
- Solution: Applied the contributor's answer: no new marker. D17 states the case and why it is harmless: the latest line found is then the earlier request's, and the earlier run could only have finished with that file valid or with its 3 refusals used, so the hook lets the agent finish, with a message naming the earlier file. T5's tests include a continued transcript whose last request has no line.
- Current-tree verification: `grep -c` on `docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/ISSUE.md` finds "A continuation without a line" once, "its message names the earlier file" once and "whose last request has no line" 3 times (the T5 row, the Commit Points and Automatic Checks).
- Resolution reference: `docs(issues): [#2904] state that a continuation without a reply file line keeps the earlier file`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668485342

### F19 - M7 does not observe a continued run

- PR number: 3002
- Source review ID: 6023553046
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4667281929
- Concern: The hook reads the latest `Reply file:` line from an undocumented transcript format, M7 covers only a fresh task, and the continued-run test uses fixtures, so nothing observes how Claude Code records a resumed request. CodeRabbit asks for a continued-run step in M7 that records the message carrying the line, the Claude Code version and whether the hook reads the new path, and for D17 to be revised before T5 if the line cannot be told apart.
- Solution: Applied the contributor's answer: M7 continues a finished `task-reviewer` with a new request carrying a new `Reply file:` line (the next round's file), and V7 records the Claude Code version, which transcript message carries the line, where the preloaded skill text sits and whether the hook reads the new path; if the line cannot be told apart from tool results, skill text or the refusal text, D17 is revised before T5's code. D17 no longer says that M7 already shows this: the tests pin only the format their fixtures assume, and M7 observes the real one. The T5 row says M7 is recorded with its continued-run step.
- Current-tree verification: `grep -c` on `docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/ISSUE.md` finds "continue the finished reviewer with a new request" once (M7), "D17 is revised before T5's code" three times (D17, M7 and the Progress Log entry of 16:02 UTC), "including its continued-run step" once (the T5 row) and "scenario M7 shows that the hook" 0 times.
- Resolution reference: `docs(issues): [#2904] add a continued-run step to scenario M7`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668485899

### F20 - The F12 Reply URL was a placeholder

- PR number: 3002
- Source review ID: 6023553046
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4667281942
- Concern: At the head commit `docs(pr-reviews): [#3002] record review round 2 before replying` the F12 row's Reply URL was a placeholder, so the validator reported one failure. CodeRabbit asks to post the reply, record its URL and resolve only after the reply.
- Solution: No further change: the reply was posted before the thread was resolved, and the next commit recorded its URL. The thread was already resolved by the CodeRabbit account (GraphQL `resolvedBy`); it gets a reply now and stays resolved.
- Current-tree verification: The F12 detail entry's Reply URL is `discussion_r4667230477`; review 6023654876 reports that the validator's logic passes at the head commit `docs(pr-reviews): [#3002] record the reply and resolution of review round 2` (12 rows, 0 failures).
- Resolution reference: `docs(pr-reviews): [#3002] record the reply and resolution of review round 2`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668486615

### F21 - Review 6023349775 was left out of the record

- PR number: 3002
- Source review ID: 6023574266
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4667299022
- Concern: Review 6023349775 was submitted three minutes before the round-2 audit commit, but the record had no row for its five comments and none of its threads had a reply or resolution. In its next review `da2ce7` widened the finding to review 6023402273 of Copilot.
- Solution: This round records both reviews (F13 to F18), and each of their threads is replied to and resolved.
- Current-tree verification: The rows F13 to F18 cite reviews 6023349775 and 6023402273; the Reply URLs and the final GraphQL fetch are recorded at the end of the round.
- Resolution reference: `docs(pr-reviews): [#3002] record review round 3`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668487209

### F22 - The F12 Reply URL was still a placeholder

- PR number: 3002
- Source review ID: 6023574266
- Reviewer finding ID: F13
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4667299037
- Concern: The same concern as F20: the F12 Reply URL was a placeholder although the reply existed, so the validator failed once.
- Solution: Fixed by the commit of F20.
- Current-tree verification: As for F20.
- Resolution reference: `docs(pr-reviews): [#3002] record the reply and resolution of review round 2`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668487811

### F23 - The Copilot continuation thread had no reply, row or resolution

- PR number: 3002
- Source review ID: 6023574266
- Reviewer finding ID: F14
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4667299056
- Concern: The thread of F18 had no reply, row or resolution. `da2ce7` adds that by D17's own bound the result is harmless, since the earlier run could only have finished with that file valid or its 3 refusals used, and asks that the text say so or mark the request boundary.
- Solution: Fixed by the commit of F18: D17 says so.
- Current-tree verification: As for F18.
- Resolution reference: `docs(issues): [#2904] state that a continuation without a reply file line keeps the earlier file`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668488441

### F24 - M7 lacks the continued-run step

- PR number: 3002
- Source review ID: 6023654876
- Reviewer finding ID: F15
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4667361461
- Concern: The D17 sentence said that M7 shows the hook reads the line and that the tests pin the format including a continued run, while M7 had no continuation step and fixtures can only pin the format they assume. The same request as F19.
- Solution: Fixed by the commit of F19.
- Current-tree verification: As for F19.
- Resolution reference: `docs(issues): [#2904] add a continued-run step to scenario M7`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668489016

### F25 - Five later threads had no reply, row or resolution

- PR number: 3002
- Source review ID: 6023654876
- Reviewer finding ID: F16
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4667361489
- Concern: The two threads of review 6023553046 and the three of review 6023574266 postdate the round-2 audit commit and had no reply, row or resolution.
- Solution: This round records the five threads (F19 to F23), replies on each and resolves it; the F20 thread was already resolved and stays so.
- Current-tree verification: The rows F19 to F23 cite reviews 6023553046 and 6023574266; the Reply URLs and the final GraphQL fetch are recorded at the end of the round.
- Resolution reference: `docs(pr-reviews): [#3002] record review round 3`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668489611

### F26 - A continuation can reuse a reply path that no longer exists

- PR number: 3002
- Source review ID: 6025120305
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668527336
- Concern: The continuation without a `Reply file:` line reuses the earlier request's line and was called harmless, which holds only while the earlier file is in place. Close-out moves the specification folder to `closed/` with its `agent-replies/` files, so the earlier path under `open/` no longer exists, and no path-not-found case was stated. CodeRabbit also names an earlier run that used its 3 refusals without a valid file, and asks that omitting the line cannot bypass a required reply.
- Solution: Applied the contributor's decision of 2026-11-23: D17 adds the case of a missing specification folder. For a path under `open/` or `closed/` whose specification folder does not exist (moved to `closed/` at close-out, or mistyped by the caller) the hook lets the agent finish with a message naming the path and saying that its specification folder does not exist, and keeps no count, because the agent cannot create a specification folder legitimately and must not recreate one under `open/` to write a reply. The continuation sentence no longer assumes the earlier file is still in place. The T5 row, the Commit Points and Automatic Checks name the test. No change for the other two points: an earlier run that used its 3 refusals is already covered by the count, which stays at 3 for the same `agent_id` and reply file, so the hook lets the agent finish; and a caller cannot skip a required reply by omitting the line, because D17 and D18 require the caller to write the line in every request that has a folder-style specification.
- Current-tree verification: `grep -c -F` on `docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/ISSUE.md` finds "A missing specification folder." once (D17), "must not recreate a folder under `open/`" once, "case of a missing specification folder below applies" once (the continuation without a line), and "path whose specification folder does not exist" 3 times (the T5 row, the Commit Points and Automatic Checks).
- Resolution reference: `docs(issues): [#2904] let the task reviewer finish when the reply file's specification folder is missing`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669144270

### F27 - The description says the caller stops in the no-line case too

- PR number: 3002
- Source review ID: 6025142141
- Reviewer finding ID: F17
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668544775
- Concern: Under "When the hook lets the agent finish" the description listed three cases (no line, a draft path, a path outside the three folders) and then said "In these cases it keeps no count, and the caller then finds no valid reply file and stops." D17 states that consequence only for the path cases; with no line, the caller named no reply file. The merge tool copies the description into the merge commit.
- Solution: Applied the contributor's decision: the description was edited with `gh-bot pr edit 3002 --body-file` after the fix push. The sentence on stopping now covers only the three path cases (a draft, a path outside the three folders, a missing specification folder); the keeping of no count still covers every let-finish case. The description also gains the missing-folder case of F26. Nothing else in it changed.
- Current-tree verification: After the edit, the body that `gh-bot pr view 3002 --json body` returned was the edited file apart from a trailing newline; `grep -c` on it printed 0 for "In these cases it keeps no count, and the caller then finds", 1 for "For the three path cases" and 2 for "missing specification folder".
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669144651
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669144651

### F28 - F19's verification counts a phrase twice instead of three times

- PR number: 3002
- Source review ID: 6025142141
- Reviewer finding ID: F18
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668544782
- Concern: F19's Current-tree verification said `grep -c` finds "D17 is revised before T5's code" twice (D17 and M7); at the head it printed 3, because the Progress Log entry of 16:02 UTC, added after the fix, carries the phrase too.
- Solution: Applied the contributor's decision: F19's verification now says three times and names the Progress Log entry of 16:02 UTC, as F14's verification names its third match. The Processing Log records the correction. The other verification counts of rows F13 to F25 were checked again at the head commit `docs(pr-reviews): [#3002] record the replies and resolutions of review round 3` and reproduce.
- Current-tree verification: At the head commit `docs(pr-reviews): [#3002] record the replies and resolutions of review round 3`, `grep -n -F "D17 is revised before T5's code"` on `docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/ISSUE.md` printed lines 373 (D17), 670 (the 16:02 Progress Log entry) and 740 (M7); the F19 entry now says three times and names them.
- Resolution reference: `docs(pr-reviews): [#3002] record review round 4`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669145147

### F29 - The round-3 rows had placeholder Reply URLs

- PR number: 3002
- Source review ID: 6025142141
- Reviewer finding ID: F19
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668544786
- Concern: At the head commit `docs(pr-reviews): [#3002] record review round 3` the Reply URLs of F13 to F25, and F13's verification and resolution reference, were placeholders, the rows said `RESOLVED` while eleven of the thirteen threads were unresolved, and the validator's logic reported 14 failures.
- Solution: No further change: the next commit filled the thirteen Reply URLs and F13's fields after the replies, and the threads were then resolved. Review 6025460812 of `da2ce7` reports F19 fixed at that commit.
- Current-tree verification: This record has no `PENDING` text for rows F1 to F25; the copy of the audit validator that calls `gh-bot` exited 0 at the end of round 3 (25 rows, 0 failures), and review 6025460812 reports the same result (25 rows, 0 failures).
- Resolution reference: `docs(pr-reviews): [#3002] record the replies and resolutions of review round 3`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669145760

### F30 - The F13 Reply URL was a placeholder

- PR number: 3002
- Source review ID: 6025374584
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668734671
- Concern: On the head commit `docs(pr-reviews): [#3002] record review round 3`, the Reply URL of F13 was `PENDING`. Copilot asks to fill the placeholders of durable references before merge, or to set a final value. The overview marks it High, recorded as `Major (inferred)`. The same request as F29.
- Solution: Fixed by the commit of F29, which filled the Reply URL of F13 with the reply on its own thread.
- Current-tree verification: The F13 detail entry's Reply URL is a `discussion_r` URL; no Reply URL, Resolution reference or Current-tree verification field of this record holds a placeholder, and the audit validator confirms it.
- Resolution reference: `docs(pr-reviews): [#3002] record the replies and resolutions of review round 3`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669146315

### F31 - The F14 Reply URL was a placeholder

- PR number: 3002
- Source review ID: 6025374584
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668734746
- Concern: On the head commit `docs(pr-reviews): [#3002] record review round 3`, the Reply URL of F14 was `PENDING`. Copilot asks to fill the placeholders of durable references before merge, or to set a final value. The overview marks it High, recorded as `Major (inferred)`. The same request as F29.
- Solution: Fixed by the commit of F29, which filled the Reply URL of F14 with the reply on its own thread.
- Current-tree verification: The F14 detail entry's Reply URL is a `discussion_r` URL; no Reply URL, Resolution reference or Current-tree verification field of this record holds a placeholder, and the audit validator confirms it.
- Resolution reference: `docs(pr-reviews): [#3002] record the replies and resolutions of review round 3`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669147040

### F32 - The F15 Reply URL was a placeholder

- PR number: 3002
- Source review ID: 6025374584
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668734798
- Concern: On the head commit `docs(pr-reviews): [#3002] record review round 3`, the Reply URL of F15 was `PENDING`. Copilot asks to fill the placeholders of durable references before merge, or to set a final value. The overview marks it High, recorded as `Major (inferred)`. The same request as F29.
- Solution: Fixed by the commit of F29, which filled the Reply URL of F15 with the reply on its own thread.
- Current-tree verification: The F15 detail entry's Reply URL is a `discussion_r` URL; no Reply URL, Resolution reference or Current-tree verification field of this record holds a placeholder, and the audit validator confirms it.
- Resolution reference: `docs(pr-reviews): [#3002] record the replies and resolutions of review round 3`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669147837

### F33 - The F16 Reply URL was a placeholder

- PR number: 3002
- Source review ID: 6025374584
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668734855
- Concern: On the head commit `docs(pr-reviews): [#3002] record review round 3`, the Reply URL of F16 was `PENDING`. Copilot asks to fill the placeholders of durable references before merge, or to set a final value. The overview marks it High, recorded as `Major (inferred)`. The same request as F29.
- Solution: Fixed by the commit of F29, which filled the Reply URL of F16 with the reply on its own thread.
- Current-tree verification: The F16 detail entry's Reply URL is a `discussion_r` URL; no Reply URL, Resolution reference or Current-tree verification field of this record holds a placeholder, and the audit validator confirms it.
- Resolution reference: `docs(pr-reviews): [#3002] record the replies and resolutions of review round 3`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669148577

### F34 - The F17 Reply URL was a placeholder

- PR number: 3002
- Source review ID: 6025374584
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668734904
- Concern: On the head commit `docs(pr-reviews): [#3002] record review round 3`, the Reply URL of F17 was `PENDING`. Copilot asks to fill the placeholders of durable references before merge, or to set a final value. The overview marks it High, recorded as `Major (inferred)`. The same request as F29.
- Solution: Fixed by the commit of F29, which filled the Reply URL of F17 with the reply on its own thread.
- Current-tree verification: The F17 detail entry's Reply URL is a `discussion_r` URL; no Reply URL, Resolution reference or Current-tree verification field of this record holds a placeholder, and the audit validator confirms it.
- Resolution reference: `docs(pr-reviews): [#3002] record the replies and resolutions of review round 3`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669149234

### F35 - The F18 Reply URL was a placeholder

- PR number: 3002
- Source review ID: 6025374584
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668734958
- Concern: On the head commit `docs(pr-reviews): [#3002] record review round 3`, the Reply URL of F18 was `PENDING`. Copilot asks to fill the placeholders of durable references before merge, or to set a final value. The overview marks it High, recorded as `Major (inferred)`. The same request as F29.
- Solution: Fixed by the commit of F29, which filled the Reply URL of F18 with the reply on its own thread.
- Current-tree verification: The F18 detail entry's Reply URL is a `discussion_r` URL; no Reply URL, Resolution reference or Current-tree verification field of this record holds a placeholder, and the audit validator confirms it.
- Resolution reference: `docs(pr-reviews): [#3002] record the replies and resolutions of review round 3`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669149969

### F36 - The F19 Reply URL was a placeholder

- PR number: 3002
- Source review ID: 6025374584
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668735020
- Concern: On the head commit `docs(pr-reviews): [#3002] record review round 3`, the Reply URL of F19 was `PENDING`. Copilot asks to fill the placeholders of durable references before merge, or to set a final value. The overview marks it High, recorded as `Major (inferred)`. The same request as F29.
- Solution: Fixed by the commit of F29, which filled the Reply URL of F19 with the reply on its own thread.
- Current-tree verification: The F19 detail entry's Reply URL is a `discussion_r` URL; no Reply URL, Resolution reference or Current-tree verification field of this record holds a placeholder, and the audit validator confirms it.
- Resolution reference: `docs(pr-reviews): [#3002] record the replies and resolutions of review round 3`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669150848

### F37 - The F20 Reply URL was a placeholder

- PR number: 3002
- Source review ID: 6025374584
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668735072
- Concern: On the head commit `docs(pr-reviews): [#3002] record review round 3`, the Reply URL of F20 was `PENDING`. Copilot asks to fill the placeholders of durable references before merge, or to set a final value. The overview marks it High, recorded as `Major (inferred)`. The same request as F29.
- Solution: Fixed by the commit of F29, which filled the Reply URL of F20 with the reply on its own thread.
- Current-tree verification: The F20 detail entry's Reply URL is a `discussion_r` URL; no Reply URL, Resolution reference or Current-tree verification field of this record holds a placeholder, and the audit validator confirms it.
- Resolution reference: `docs(pr-reviews): [#3002] record the replies and resolutions of review round 3`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669151474

### F38 - The F21 Reply URL was a placeholder

- PR number: 3002
- Source review ID: 6025374584
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668735139
- Concern: On the head commit `docs(pr-reviews): [#3002] record review round 3`, the Reply URL of F21 was `PENDING`. Copilot asks to fill the placeholders of durable references before merge, or to set a final value. The overview marks it High, recorded as `Major (inferred)`. The same request as F29.
- Solution: Fixed by the commit of F29, which filled the Reply URL of F21 with the reply on its own thread.
- Current-tree verification: The F21 detail entry's Reply URL is a `discussion_r` URL; no Reply URL, Resolution reference or Current-tree verification field of this record holds a placeholder, and the audit validator confirms it.
- Resolution reference: `docs(pr-reviews): [#3002] record the replies and resolutions of review round 3`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669152049

### F39 - The F22 Reply URL was a placeholder

- PR number: 3002
- Source review ID: 6025374584
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668735209
- Concern: On the head commit `docs(pr-reviews): [#3002] record review round 3`, the Reply URL of F22 was `PENDING`. Copilot asks to fill the placeholders of durable references before merge, or to set a final value. The overview marks it High, recorded as `Major (inferred)`. The same request as F29.
- Solution: Fixed by the commit of F29, which filled the Reply URL of F22 with the reply on its own thread.
- Current-tree verification: The F22 detail entry's Reply URL is a `discussion_r` URL; no Reply URL, Resolution reference or Current-tree verification field of this record holds a placeholder, and the audit validator confirms it.
- Resolution reference: `docs(pr-reviews): [#3002] record the replies and resolutions of review round 3`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669152725

### F40 - The F23 Reply URL was a placeholder

- PR number: 3002
- Source review ID: 6025374584
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668735274
- Concern: On the head commit `docs(pr-reviews): [#3002] record review round 3`, the Reply URL of F23 was `PENDING`. Copilot asks to fill the placeholders of durable references before merge, or to set a final value. The overview marks it High, recorded as `Major (inferred)`. The same request as F29.
- Solution: Fixed by the commit of F29, which filled the Reply URL of F23 with the reply on its own thread.
- Current-tree verification: The F23 detail entry's Reply URL is a `discussion_r` URL; no Reply URL, Resolution reference or Current-tree verification field of this record holds a placeholder, and the audit validator confirms it.
- Resolution reference: `docs(pr-reviews): [#3002] record the replies and resolutions of review round 3`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669153396

### F41 - The F24 Reply URL was a placeholder

- PR number: 3002
- Source review ID: 6025374584
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668735314
- Concern: On the head commit `docs(pr-reviews): [#3002] record review round 3`, the Reply URL of F24 was `PENDING`. Copilot asks to fill the placeholders of durable references before merge, or to set a final value. The overview marks it High, recorded as `Major (inferred)`. The same request as F29.
- Solution: Fixed by the commit of F29, which filled the Reply URL of F24 with the reply on its own thread.
- Current-tree verification: The F24 detail entry's Reply URL is a `discussion_r` URL; no Reply URL, Resolution reference or Current-tree verification field of this record holds a placeholder, and the audit validator confirms it.
- Resolution reference: `docs(pr-reviews): [#3002] record the replies and resolutions of review round 3`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669153964

### F42 - The F25 Reply URL was a placeholder

- PR number: 3002
- Source review ID: 6025374584
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668735362
- Concern: On the head commit `docs(pr-reviews): [#3002] record review round 3`, the Reply URL of F25 was `PENDING`. Copilot asks to fill the placeholders of durable references before merge, or to set a final value. The overview marks it High, recorded as `Major (inferred)`. The same request as F29.
- Solution: Fixed by the commit of F29, which filled the Reply URL of F25 with the reply on its own thread.
- Current-tree verification: The F25 detail entry's Reply URL is a `discussion_r` URL; no Reply URL, Resolution reference or Current-tree verification field of this record holds a placeholder, and the audit validator confirms it.
- Resolution reference: `docs(pr-reviews): [#3002] record the replies and resolutions of review round 3`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669154471

### F43 - The 13:16 Progress Log entry says per run

- PR number: 3002
- Source review ID: 6025374584
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668735427
- Concern: The Progress Log entry of 13:16 UTC says the count of refusals is kept per run, while D17 and the entry of 14:03 UTC describe a count per request keyed by the `agent_id` and the reply file. Copilot asks to update the old entry. The overview marks it Medium, recorded as `Minor (inferred)`.
- Solution: No change, by the contributor's decision of 2026-11-23: Progress Log entries record history, and the entry of 14:03 UTC records that the per-request count replaced the per-run count of the entry above it. The old entry is not edited.
- Current-tree verification: `grep -n` on `docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/ISSUE.md`: the 13:16 entry carries "per run", the 14:03 entry carries "replacing the per-run count of the entry above", and D17 says "One file per request".
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669155004
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669155004

### F44 - The T5 table row is a long line

- PR number: 3002
- Source review ID: 6025374584
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668735474
- Concern: The T5 row of the Implementation Plan (line 580 at the reviewed head) is a long line; Copilot asks to reflow it into wrapped lines or sub-bullets. The overview marks it Low, recorded as `Nit (inferred)`.
- Solution: No change, by the contributor's decision of 2026-11-23: a Markdown table row cannot wrap across lines, and the other rows of the table are single lines too.
- Current-tree verification: The T5 row of `docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/ISSUE.md` is one table row on a single line, as every row of the Implementation Plan table is.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669155505
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669155505

### F45 - The test paragraph of the Commit Points is one long line

- PR number: 3002
- Source review ID: 6025374584
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668735532
- Concern: The paragraph that lists the tests of T3 and T5 in the Commit Points (line 612 at the reviewed head) began with a 390-character line in a file that wraps prose near 100 characters. The overview marks it Low, recorded as `Nit (inferred)`.
- Solution: Applied the contributor's decision: the paragraph is rewrapped near 100 characters; `git diff --word-diff=porcelain` showed no changed words.
- Current-tree verification: `awk` on `docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/ISSUE.md` prints no line longer than 100 characters in the paragraph that begins "Tests are added in T3".
- Resolution reference: `docs(issues): [#2904] wrap the test paragraph of the Commit Points`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669156072

### F46 - The missing reply path thread had no reply

- PR number: 3002
- Source review ID: 6025460812
- Reviewer finding ID: F26
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668803202
- Concern: The CodeRabbit thread of F26, opened 51 seconds before the head's audit commit, had no reply, row or resolution, and its concern holds: after close-out moves the specification to `closed/`, a no-line continuation finds no file at the earlier path. `da2ce7` suggests letting the agent finish with a message naming the missing file and keeping no count.
- Solution: Fixed by the commit of F26, which takes that approach for a missing specification folder; this round replies on the F26 thread, adds its row and resolves it.
- Current-tree verification: As for F26.
- Resolution reference: `docs(issues): [#2904] let the task reviewer finish when the reply file's specification folder is missing`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669156625

### F47 - Nineteen later threads had no reply, row or resolution

- PR number: 3002
- Source review ID: 6025460812
- Reviewer finding ID: F27
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4668803212
- Concern: The three threads of review 6025142141 and the sixteen of review 6025374584, opened after the head's audit commit, had no reply, row or resolution.
- Solution: This round records the nineteen threads (F27 to F45), replies on each and resolves it.
- Current-tree verification: The rows F27 to F45 cite reviews 6025142141 and 6025374584; the Reply URLs and the final GraphQL fetch are recorded in the Processing Log of this round.
- Resolution reference: `docs(pr-reviews): [#3002] record review round 4`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669157220

### F48 - The hook does not enforce the line in a folder-style continuation

- PR number: 3002
- Source review ID: 6025904627
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#pullrequestreview-6025904627
- Concern: CodeRabbit's reply on the F26 thread (comment `discussion_r4669161099`, the only comment of its review; the row cites the review because the comment is not the first of its thread, and the audit validator anchors a discussion row to the thread's first comment) accepts the missing-folder case and the three-refusal case, and keeps one point of F26: if a caller omits the required line in a folder-style continuation, the hook reuses the earlier line and, when that file is valid, lets the agent finish without a new reply, so that part relies on the caller. It does not mark F26 addressed. The reply carries no severity label; recorded as `Minor (inferred)`, as F26.
- Solution: No change, by the contributor's decision of 2026-11-23 recorded in F26: a skipped reply by an omitted line is covered by the caller rule of D17 and D18, not by the hook. The thread was already resolved after the F26 reply and stays resolved.
- Current-tree verification: D17 in `docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/ISSUE.md` says that a caller that continues a finished reviewer writes the line again in its new request, and that for a continuation without a folder-style specification "no other marker is added".
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669191731
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669191731

### F49 - The record had no rows for the fourth round

- PR number: 3002
- Source review ID: 6026236623
- Reviewer finding ID: F46
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669406210
- Concern: On the head commit `docs(issues): [#2904] log the fourth review round of pull request 3002`, the specification's Progress Log recorded four addressed reviews, while this record ended at round 3 and the replies already posted cited rows F29 to F45 that it did not contain.
- Solution: The commit `docs(pr-reviews): [#3002] record review round 4`, pushed after that head, added the round-4 paragraph, rows F26 to F48, their detail entries and Reply URLs, and the Processing Log entries of that round.
- Current-tree verification: The Findings table has rows F26 to F48, each detail entry has a `discussion_r` Reply URL, and the audit validator exits 0.
- Resolution reference: `docs(pr-reviews): [#3002] record review round 4`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669575184

### F50 - The placeholder check matched its own line

- PR number: 3002
- Source review ID: 6026291754
- Reviewer finding ID: F28
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669448063
- Concern: The verification lines of F30 to F42 said that the record had no `PENDING` text, but a plain count of that word on the record found 28 matches, among them those lines themselves. The claim meant is that no field holds a placeholder.
- Solution: The thirteen lines now say that no Reply URL, Resolution reference or Current-tree verification field of this record holds a placeholder, and that the audit validator confirms it. F29's verification, limited to rows F1 to F25, is unchanged.
- Current-tree verification: The thirteen verification lines of F30 to F42 carry the new wording; the audit validator exits 0.
- Resolution reference: `docs(pr-reviews): [#3002] say that no field holds a placeholder instead of no PENDING text`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669575652

### F51 - Two answered threads had no rows

- PR number: 3002
- Source review ID: 6026507151
- Reviewer finding ID: F49
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669618099
- Concern: On the head commit `docs(pr-reviews): [#3002] say that no field holds a placeholder instead of no PENDING text`, the threads of reviews 6026236623 and 6026291754 had replies announcing rows F49 and F50 and were resolved, but the record still ended at F48. The comment of review 6026236623 carries `[Minor][F46]`, so its row's severity must read `Minor`.
- Solution: The commit `docs(pr-reviews): [#3002] record review round 5`, pushed after that head, added rows F49 (severity `Minor`) and F50 with their detail entries and Reply URLs.
- Current-tree verification: The Findings table has rows F49 and F50, and their detail entries have `discussion_r` Reply URLs; the audit validator exits 0.
- Resolution reference: `docs(pr-reviews): [#3002] record review round 5`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669700478

### F52 - The transcript check could not run before T5's code

- PR number: 3002
- Source review ID: 6026746874
- Reviewer finding ID: F51
- Source URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4669805683
- Concern: D17 said that if scenario M7 showed a continued request's `Reply file:` line could not be told apart in the transcript, D17 would be revised before T5's code. M7 starts `task-reviewer` with its stop hook, which T5 creates, and D18 says M7 runs on the T5 branch, so M7 could not give that evidence before the code it gates.
- Solution: By the maintainer's decision, a new scenario M8 observes the transcript format before any of T5's code, without the new agent or the hook: an agent that already exists and preloads skills is started with a `Reply file:` line and continued with a new one, and V8 records how each message is marked, which message carries each line and where the preloaded skill text sits. D17 is revised before T5's code if the lines cannot be told apart; M7 keeps the checks of the hook itself. T5's row and the Commit Points put M8 first, and the pull request description says the same.
- Current-tree verification: D17 in `docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/ISSUE.md` names scenario M8 as the check before T5's code; the Manual Verification Scenarios table has the row M8 with evidence section V8; T5's row says "scenario M8 recorded before the hook's code"; the Commit Points say that M8 and its evidence are committed before any of the hook's code.
- Resolution reference: `docs(issues): [#2904] probe the transcript format before the stop hook's code`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3002#discussion_r4670040743

## Processing Log

- 2026-11-23 14:05 UTC - Started audit. Round 1: reviews 6022610166 (CodeRabbit), 6022825332 (`da2ce7`) and 6022913275 (Copilot) on the head commit `docs(issues): [#2904] record how the stop hook of the task reviewer works`, eleven inline comments recorded as F1 to F11. Fetched the eleven review threads with GraphQL as `josecelano-bot`: all unresolved, `viewerCanResolve` true for each. Applied the maintainer's two design answers of 2026-11-23 (F1, F2, F4; F3, F5) and the mechanical fixes in five signed commits (committer times 13:59:23Z to 14:03:03Z) and logged the round in the spec (14:03:55Z); each commit's pre-commit hook passed. Checked each disposition against the current tree with the commands in the detail entries. Not pushed yet.
- 2026-11-23 14:14 UTC - Pushed the seven commits with `git-bot-push` (pre-push hook passed); `git ls-remote josecelano-bot refs/heads/2904-t5-task-reviewer-spec` shows the commit `docs(pr-reviews): [#3002] record review round 1`. Replied on the eleven threads as `josecelano-bot` (reply `created_at` 14:13:28Z to 14:13:52Z) and recorded the reply URLs.
- 2026-11-23 14:14 UTC - The copy of the audit validator that calls `gh-bot` exits 0 (11 rows, 0 failures). A GraphQL fetch showed every unresolved thread with one reply of `josecelano-bot` and `viewerCanResolve` true; the F3 thread had already been resolved by the CodeRabbit bot after the reply. The other ten threads were then resolved as `josecelano-bot`. A new GraphQL fetch shows 11 threads, 0 unresolved.
- 2026-11-23 14:22 UTC - Round 2: review 6023199677 (Greptile) on the head commit `docs(pr-reviews): [#3002] record review round 1`, one inline comment recorded as F12. Checked against the current tree: the Reply URLs it asks for were filled by the next commit, so no change. Row committed before the reply.
- 2026-11-23 14:34 UTC - Pushed the F12 row with `git-bot-push` (pre-push hook passed); `git ls-remote josecelano-bot refs/heads/2904-t5-task-reviewer-spec` showed the commit `docs(pr-reviews): [#3002] record review round 2 before replying`. Replied on the F12 thread as `josecelano-bot` and resolved it (`isResolved` true).
- 2026-11-23 16:01 UTC - Correction for round 3 (finding F15 below): in the two round paragraphs, the F12 concern and the four entries above stamped 14:05, 14:14, 14:22 and 14:34, each abbreviated commit id was replaced by that commit's subject; nothing else in them changed. The ids of the two posted replies that cite one are mapped to a commit subject after the round paragraphs. The `open-pull-request` commit-id check, run on this file and the spec, prints nothing.
- 2026-11-23 16:03 UTC - Round 3: reviews 6023349775, 6023574266 and 6023654876 (`da2ce7`), 6023402273 (Copilot) and 6023553046 (CodeRabbit), thirteen inline comments recorded as F13 to F25; a new listing of the reviews before recording showed none later than 6023654876. Fetched the threads with GraphQL as `josecelano-bot`: twelve of the thirteen unresolved with `viewerCanResolve` true, the F20 thread already resolved. Applied the contributor's answers of 2026-11-23 in four signed spec commits and the commit-id fix in one audit commit, and logged the round in the spec; each commit's pre-commit hook passed. Rows committed before the replies, so their Reply URLs, and the F13 verification and resolution reference, are placeholders that the next audit commit fills after the replies and the description edit; not pushed yet.
- 2026-11-23 16:14 UTC - Pushed the seven commits of round 3 with `git-bot-push` (pre-push hook passed); `git ls-remote josecelano-bot refs/heads/2904-t5-task-reviewer-spec` showed the commit `docs(pr-reviews): [#3002] record review round 3`. Rewrote the pull request description with `gh-bot pr edit 3002 --body-file` (F13). Replied on the thirteen threads as `josecelano-bot` (reply `created_at` 16:12:35Z to 16:13:07Z) and recorded the reply URLs and the F13 resolution reference. Corrected the F14 verification, which named the Progress Log entry where the third match is the Automatic Checks item. The copy of the audit validator that calls `gh-bot` exited 0 (25 rows, 0 failures). A GraphQL fetch then showed eleven unresolved threads, each with one reply of `josecelano-bot` and `viewerCanResolve` true; the F19 and F20 threads had been resolved by the CodeRabbit account. The eleven were resolved as `josecelano-bot` (`isResolved` true for each); a new GraphQL fetch shows 25 threads, 0 unresolved.
- 2026-11-23 17:09 UTC - Round 4: reviews 6025120305 (CodeRabbit), 6025142141 and 6025460812 (`da2ce7`) and 6025374584 (Copilot), twenty-two inline comments recorded as F26 to F47; a listing of the reviews before recording showed none later than 6025460812 apart from this round's replies of `josecelano-bot`. Fetched the threads with GraphQL as `josecelano-bot`: the twenty-two unresolved with `viewerCanResolve` true. Applied the contributor's answers of 2026-11-23 in two signed spec commits and logged the round in the spec; each commit's pre-commit hook passed. Corrected the F19 verification, which said twice where `grep -n -F` at the head commit `docs(pr-reviews): [#3002] record the replies and resolutions of review round 3` printed three lines (F28). Pushed the three spec commits with `git-bot-push` (pre-push hook passed); `git ls-remote josecelano-bot refs/heads/2904-t5-task-reviewer-spec` showed the commit `docs(issues): [#2904] log the fourth review round of pull request 3002`. Edited the pull request description with `gh-bot pr edit 3002 --body-file` (F27). Replied on the twenty-two threads as `josecelano-bot` (reply `created_at` 17:05:28Z to 17:06:25Z) and recorded the reply URLs before committing this record. The copy of the audit validator that calls `gh-bot` then reported 47 rows and 2 failures, the rows F28 and F47, whose resolution reference is this record's own commit, not made yet. A GraphQL fetch showed the twenty-two threads unresolved, each with a reply of `josecelano-bot` and `viewerCanResolve` true; they were resolved as `josecelano-bot` (`isResolved` true for each), and a new GraphQL fetch showed 47 threads, 0 unresolved.
- 2026-11-23 17:09 UTC - A new listing of the reviews showed review 6025904627 (CodeRabbit, 17:06:45Z), its reply on the F26 thread, recorded as F48, and review 6025931039 (`da2ce7`, 17:08:28Z), which has no new finding. Replied on the F26 thread with the `Superseded by F26` form (reply `created_at` 17:09:13Z); the thread was already resolved and stays resolved.
- 2026-11-23 17:14 UTC - Correction for F48: the validator reported 1 failure at the commit `docs(pr-reviews): [#3002] record review round 4`, because F48's Source URL was CodeRabbit's comment inside the F26 thread, while the reply's `in_reply_to_id` is the thread's first comment. F48 now cites its review 6025904627 as the Source URL and names the comment in its Concern; nothing else changed.
- 2026-11-23 17:38 UTC - Round 5: review 6026236623 (Copilot) and review 6026291754 (`da2ce7`, APPROVED), one inline comment each, recorded as F49 and F50; a listing of the reviews before recording showed none later. Fixed F50 in the commit `docs(pr-reviews): [#3002] say that no field holds a placeholder instead of no PENDING text`, which also logs the round in the specification; its pre-commit hook passed. Pushed it with `git-bot-push` (pre-push hook passed) and confirmed the head with `git ls-remote`. Replied on both threads as `josecelano-bot` and resolved them.
- 2026-11-23 17:48 UTC - Round 6: review 6026507151 (`da2ce7`), one inline comment recorded as F51, already fixed by the commit `docs(pr-reviews): [#3002] record review round 5`; a listing of the reviews before recording showed none later. Replied on its thread as `josecelano-bot` and resolved it.
- 2026-11-23 18:24 UTC - Round 7: reviews 6026731021 (`da2ce7`, approval dismissed by the next push) and 6026746874 (Copilot), one inline comment recorded as F52; a listing of the reviews before recording showed none later. Fixed it in the commit `docs(issues): [#2904] probe the transcript format before the stop hook's code`, whose pre-commit hook passed. The first push died with exit 141 after the pre-push hook passed and left the fork's branch unchanged; the description edit and the reply were posted before that was noticed. The retried push succeeded, and `git ls-remote` showed the fix commit. Updated the description, replied on the thread as `josecelano-bot` and resolved it.

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
