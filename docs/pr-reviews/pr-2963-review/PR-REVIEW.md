---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2963 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2963>.

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

Round 1 has two reviews at the head whose last commit is
`docs(issues): [#2904] record the task review of T4 and check AC2`:

- `5972182565` by `da2ce7` (APPROVED, 09:11:11 UTC): six inline findings, F1 to F6, with the
  reviewer's own IDs. Its body lists the same six findings and holds no other request, so it adds
  no row.
- `5972197600` by `copilot-pull-request-reviewer[bot]` (COMMENTED, 09:12:18 UTC): an overview
  with "No open issues" and no inline comment. Its note that scenario M3 is still to run restates
  the pull request's own plan and holds no request, so it adds no row.

Round 2 has one review at the head whose last commit is
`docs(pr-reviews): [#2904] add the audit record of PR #2963 review round 1`:

- `5974337471` by `da2ce7` (11:42:04 UTC; shown as DISMISSED because a later push dismissed it):
  six inline findings, F8 to F13, with the reviewer's own IDs (the reviewer's F7 was never
  posted). Its body lists the same six findings and holds no other request, so it adds no row.

Round 3 has two reviews, both shown as DISMISSED because a later push dismissed them:

- `5974503813` by `da2ce7` (11:54:19 UTC, at the head whose last commit is
  `docs(issues): [#2904] record scenario M3 and the first round count of AC9`): three inline
  findings, F14 to F16, with the reviewer's own IDs. Its body lists the same three findings and
  holds no other request, so it adds no row.
- `5974634671` by `da2ce7` (12:03:59 UTC, at the head whose last commit is
  `docs(pr-reviews): [#2904] count six sections in the F1 verification of PR #2963`): two inline
  findings, F17 and F18, with the reviewer's own IDs. Its body lists them and F14 to F16 as still
  open, and a point checked with no finding; it holds no other request, so it adds no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2963-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2963-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2963-f3 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2963-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2963-f5 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2963-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2963-f8 | Human | Nit | metadata | ORIGINAL | NO_ACTION | SUPERSEDED |
| F9 | review-finding:pr-2963-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2963-f10 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2963-f11 | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2963-f12 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2963-f13 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2963-f14 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F15 | review-finding:pr-2963-f15 | Human | Nit | documentation | ORIGINAL | FIXED | OPEN |
| F16 | review-finding:pr-2963-f16 | Human | Nit | metadata | ORIGINAL | FIXED | OPEN |
| F17 | review-finding:pr-2963-f17 | Human | Minor | metadata | ORIGINAL | FIXED | OPEN |
| F18 | review-finding:pr-2963-f18 | Human | Nit | correctness | ORIGINAL | FIXED | OPEN |

## Finding Details

### F1 - AC2 is checked on evidence that exists only outside the repository

- PR number: 2963
- Source review ID: 5972182565
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4626549863>
- Concern: V2 summarizes the transcripts' `model` values in a table without the recorded lines,
  names no command for reading the transcripts, the metadata file, the request-cache record or the
  JSON result, and does not say which variable `printenv` read or when, so AC2 cannot be
  recomputed from the tree.
- Solution: added `evidence/m1m2-run-records.md` in the specification's folder with the `model`
  lines of both transcripts, the agent's metadata file, the cited fields of the request-cache
  record, the JSON result's model list, the debug-log lines and `printenv CLAUDE_CODE_SUBAGENT_MODEL`,
  each with the command that printed it, where and when it ran, and local path parts replaced.
  V1 and V2 link the file, and V2's conclusion names the `printenv` command, its time and its exit
  code.
- Current-tree verification: the file `evidence/m1m2-run-records.md` of the fixing commit holds six
  sections, each with its command (`grep -c '^## '` on it prints 6); F10 of round 2 added a
  seventh. `grep -n "m1m2-run-records" manual-verification-evidence.md` finds the links in V1 and
  V2.
- Resolution reference: docs(issues): [#2904] store the records of the M1 and M2 run in the repository
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628055234>

### F2 - The Progress Log ends before the task review

- PR number: 2963
- Source review ID: 5972182565
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4626549877>
- Concern: the last Progress Log entry (08:39) predates the task review recorded at 08:44 and the
  commits it produced; no entry names the review, its verdict or the AC2 check.
- Solution: appended a Progress Log entry stamped 09:18 that names the review of 08:44, its
  verdict and findings, the changes made before the pull request, AC2, what was not changed, and
  that the entry was added in this round.
- Current-tree verification: `grep -n "2026-11-19 09:18 UTC" ISSUE.md` finds the entry. It was the
  last line of the Progress Log at the fixing commit; later rounds appended the entries of 11:37 and
  12:17 after it. (Corrected in round 3, in the sweep for finding F18: the first version said the
  entry is the last line, which stopped holding when the 11:37 entry was added.)
- Resolution reference: docs(issues): [#2904] log the task review of T4 in the Progress Log
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628055518>

### F3 - `last-updated-utc` predates the last change to the evidence file

- PR number: 2963
- Source review ID: 5972182565
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4626549885>
- Concern: the evidence file's `last-updated-utc` read `2026-11-19 08:38`, but a commit changed the
  file at 08:45.
- Solution: set `last-updated-utc` to `2026-11-19 09:19`, after the round's other changes to the
  file.
- Current-tree verification: `git log --format='%cI %s' -S'last-updated-utc: "2026-11-19 09:19"'`
  on the file lists the fixing commit at `2026-11-19T09:19:07Z`, which set the value to the time of
  its own commit. Later commits that changed the file moved it again: at this head it reads
  `2026-11-19 12:16`, and `git log -1 --format=%cI` on the file prints `2026-11-19T12:16:54Z`.
  (Corrected in round 3, in the sweep for finding F18: the first version said the file reads
  `2026-11-19 09:19`, which stopped holding at the next change to the file.)
- Resolution reference: docs(issues): [#2904] update last-updated-utc of the manual verification evidence
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628055944>

### F4 - V1 does not say that the probed agent file changed after the run

- PR number: 2963
- Source review ID: 5972182565
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4626549894>
- Concern: the run used the agent file of the first T4 commit, and a later commit changed its
  `description` value and body; V1 does not say so.
- Solution: V1's initial state names the later commit, what it changed, that the keys and the
  values of `name`, `model`, `tools` and `skills` did not change, and that the run did not observe
  the `description`.
- Current-tree verification: lines 52 to 55 of the evidence file hold the added sentences.
- Resolution reference: docs(issues): [#2904] say in V1 that the agent file changed after the run
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628056394>

### F5 - The inventory note on loading `AGENTS.md` is too broad

- PR number: 2963
- Source review ID: 5972182565
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4626549907>
- Concern: the note said every agent Claude Code starts loads `AGENTS.md`, while the
  specification's own facts say the built-in Explore and Plan agents skip the instruction files.
- Solution: the note now says that `pr-review-handler`, a project agent, loads `AGENTS.md` through
  the root `CLAUDE.md`, as scenario M4 showed for a started agent.
- Current-tree verification: the `Follow AGENTS.md.` row of `removed-profile-rules.md` reads as
  above; `grep -n "Every agent Claude Code starts" removed-profile-rules.md` finds nothing.
- Resolution reference: docs(issues): [#2904] narrow the inventory note on loading AGENTS.md to the handler
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628056779>

### F6 - The description gives too few rows as not already held

- PR number: 2963
- Source review ID: 5972182565
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4626549912>
- Concern: the pull-request description said every rule but two was already held, while the
  inventory has five rows already held, three in the agent file and two dropped.
- Solution: edited the pull-request description at 09:24:08 UTC to give the ten rows as five
  already held, three in the agent file (identity, tools, inputs) and two dropped. The fix is
  outside the repository tree, so its reference is the pull-request comment that states it.
- Current-tree verification: `gh-bot pr view 2963 --json body` shows the corrected sentence; the
  description's edit history shows an edit by `josecelano-bot` at 2026-11-19T09:24:08Z; the
  inventory table has ten rows with those destinations.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2963#issuecomment-7007780542>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628057104>

### F8 - A commit message cites a branch commit by id

- PR number: 2963
- Source review ID: 5974337471
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628195278>
- Concern: the body of `docs(issues): [#2904] say in V1 that the agent file changed after the run`
  gives a `git diff` from a commit of this branch by its id, which a rebase rewrites; the
  reviewer says the message can be reworded if the branch is ever rewritten for another reason.
- Solution: no change. The message can change only by rewriting the branch's history, which this
  round does not do; the file already cites the commit by its subject. If the branch is rewritten
  for another reason, the message should name
  `docs(issues): [#2904] mark task T4 in progress with M1 and M2 done` instead.
- Current-tree verification: `git log --format=%B` of that commit shows the id in its body; the
  evidence file's V1 cites `docs(agents): [#2904] align pr-review-handler with the review of task T4`
  by subject; the PR-branch commit-id check of `open-pull-request` prints nothing for the files.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628369297>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628369297>

### F9 - The T4 review log entry says "one commit each"

- PR number: 2963
- Source review ID: 5974337471
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628195298>
- Concern: the Progress Log entry of 09:18 says the review items were addressed "one commit each",
  but the branch-sync item took two commits and other items share commits.
- Solution: the entry now says "in four commits" and that the words were corrected in the second
  round, with the reason.
- Current-tree verification: `grep -n "one commit each" ISSUE.md` prints one line, the 09:18 entry,
  where the words are quoted in the sentence that records their correction; no other line uses
  them. `grep -c "in four commits" ISSUE.md` prints 1, in the same entry. The four commits are the ones that fix the V1 skills sentence, the agent file,
  the `wait-for-pr-review` paragraph, and AC2 with the review record. The first version of this
  verification, and the F9 reply, said `grep -c "one commit each" ISSUE.md` prints 0; it prints 1,
  because the fix itself quotes the old words (finding F18 of round 3, corrected here).
- Resolution reference: docs(issues): [#2904] say how many commits addressed the review of T4
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628369691>

### F10 - The stored records omit the session's listing

- PR number: 2963
- Source review ID: 5974337471
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628195310>
- Concern: V1 relies on the session listing the handler and on the agent's reply, but the stored
  JSON-result record keeps only `session_id` and the model keys.
- Solution: added a last section to `evidence/m1m2-run-records.md` with the session transcript's
  text blocks and tool call, and the output of `jq -r .result .tmp/m1m2/result.json`, each with
  its command and the time it ran. The result text holds only step 3, so the listing is stored
  from the transcript.
- Current-tree verification: the file's last section holds the step 1 text naming
  `pr-review-handler`, the `Agent` tool call with `subagent_type` `pr-review-handler`, and the
  step 3 text with `READY`; `linter markdown` and `linter cspell` exited 0.
- Resolution reference: docs(issues): [#2904] store the session's listing and the agent's reply of the M1 run
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628370081>

### F11 - The audit record cites two commits of this branch by id

- PR number: 2963
- Source review ID: 5974337471
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628195323>
- Concern: the Findings introduction and the first Processing Log entry named the round-1 head by
  its full id and the pushed head by its abbreviated id, both commits of this branch.
- Solution: both places name each head by the subject of its last commit and describe the ACK
  without quoting its id. Rewriting the log entry in place is recorded in an appended correction
  entry, which also maps the id the round-1 replies cite to its subject. The sweep of the
  branch's other files and of the pull-request description found no other such id.
- Current-tree verification: the PR-branch commit-id check of `open-pull-request`, run against
  `torrust/develop`, prints nothing; `gh-bot pr view 2963 --json body` holds no commit id of this branch.
- Resolution reference: docs(pr-reviews): [#2904] name the heads of PR #2963 round 1 by commit subject
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628370486>

### F12 - The M3 evidence the description promises is not at the reviewed head

- PR number: 2963
- Source review ID: 5974337471
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628195335>
- Concern: at the reviewed head the evidence file had no V3, M3 was `TODO`, AC9 was open and T4
  was `IN_PROGRESS`, although the description said they follow in the commits of the first round.
- Solution: the two commits pushed after the reviewed head add section V3 (with the run that
  stopped at a question before pushing and the push after the contributor's answer) and section
  V6, mark M3 and T4 `DONE`, and add a Progress Log entry. AC9 stays open by design: it covers the
  first three pull requests handled by the agent.
- Current-tree verification: `grep -n "^### V3\|^### V6" manual-verification-evidence.md` finds
  both sections; the M3 row of the scenario table and the T4 row of the task table read `DONE`;
  the AC9 row cites V6.
- Resolution reference: docs(issues): [#2904] record scenario M3 and the first round count of AC9
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628370868>

### F13 - F1's verification counts five sections

- PR number: 2963
- Source review ID: 5974337471
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628195347>
- Concern: F1's current-tree verification said the records file holds five sections; it has six.
- Solution: F1's verification says six sections, each with its command, and that F10 added a
  seventh.
- Current-tree verification: `git show` of the records file at the F1 fixing commit, piped to
  `grep -c '^## '`, prints 6; at the current head it prints 7.
- Resolution reference: docs(pr-reviews): [#2904] count six sections in the F1 verification of PR #2963
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628371374>

### F14 - V3 and V6 rest on run records that are not stored

- PR number: 2963
- Source review ID: 5974503813
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628319852>
- Concern: V3 gives each M3 run's `model` count, the `requestShape` metadata, the wind-down debug
  line, the request-cache tools and the agent's report of run 3 with no stored copy and no
  command, and V6's model of each round depends on them, while V1's records are stored.
- Solution: added `evidence/m3-run-records.md` with the `model` lines of the three M3 runs' session
  and agent transcripts and of rounds 2 and 3, the agents' metadata files and request-cache
  records, the runs' JSON results and start times, how each run was started, the lines that show
  how run 1 ended, and the reports of the three runs. Each record has the command that printed it
  and the time it ran; local path parts and the ids of three branch commits are replaced. V3 and V6
  link the file, and V6 counts rounds 2 and 3.
- Current-tree verification: `grep -n "m3-run-records" manual-verification-evidence.md` finds the
  links in V3 and V6; `grep -c '^## ' evidence/m3-run-records.md` prints 7; `grep -c "/home"` on
  it prints 0; the PR-branch commit-id check of `open-pull-request` prints nothing.
- Resolution reference: docs(issues): [#2904] store the records of the M3 runs and count review rounds 2 and 3
- Follow-up PR URL: N/A
- Reply URL: pending

### F15 - The wait-ceiling variable has no source and was not tested alone

- PR number: 2963
- Source review ID: 5974503813
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628319860>
- Concern: V3 names `CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS=0` without a source, and offers it as an
  alternative to the foreground start, although it was only set together with one.
- Solution: V3 quotes run 1's stderr message, which names the variable, and says it is the only
  source; the limit now says the effect of the variable alone was not observed, since every run
  that set it also asked for a foreground start. The records file stores the message (from the
  working session's transcript, since run 2 overwrote the stderr file; the debug logs do not hold
  it) and, for each start command, whether it set the variable and asked for the foreground.
- Current-tree verification: `grep -n "BG_WAIT" manual-verification-evidence.md` finds the quoted
  message in run 1's result and the limit that says its effect alone was not observed; the
  records file's section "How each run was started" shows `sets_ceiling_variable` true only with
  `prompt_says_foreground` true.
- Resolution reference: docs(issues): [#2904] name the source of the wait-ceiling variable in V3
- Follow-up PR URL: N/A
- Reply URL: pending

### F16 - AC3 stays unchecked while M3 is done

- PR number: 2963
- Source review ID: 5974503813
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628319868>
- Concern: AC3 and its Acceptance Verification row stayed `TODO` while M3, its evidence, was
  `DONE`; AC2 was checked when V2 was recorded.
- Solution: AC3 asks for one real round processed by `pr-review-handler` from fetch to resolved
  threads, and `validate-audit-record.py` exiting `0`; V3 records both, so AC3 is checked and its
  row cites V3 and the records file. The AC9 row gives the three rounds V6 now counts, and a
  Progress Log entry records the round.
- Current-tree verification: `grep -n "AC3" ISSUE.md` shows `- [x] AC3` and the row `| AC3 | DONE`
  citing section V3.
- Resolution reference: docs(issues): [#2904] check AC3 with the evidence of scenario M3
- Follow-up PR URL: N/A
- Reply URL: pending

### F17 - The second-round replies cite rows that the reviewed head does not carry

- PR number: 2963
- Source review ID: 5974634671
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628420714>
- Concern: the round-2 replies (11:59:19 to 11:59:30 UTC) cite rows F8 to F13, but at the reviewed
  head the record had rows only for F1 to F6, no log entry for the round, and the six threads
  were open.
- Solution: the round-2 run committed the six rows with their reply URLs and a log entry after
  the review was posted, and resolved the six threads. In this round the rows were committed and
  pushed before the replies.
- Current-tree verification: the record has rows and detail entries for F8 to F13, each with its
  reply URL, and the 12:01 log entry for round 2, from the commit of 12:01:58 UTC
  (`git log --format=%cI` on the record); a GraphQL query at 12:09 UTC showed the six round-2
  threads resolved by `josecelano-bot`, each with two comments.
- Resolution reference: docs(pr-reviews): [#2904] record PR #2963 review round 2 in the audit
- Follow-up PR URL: N/A
- Reply URL: pending

### F18 - The F9 reply's check does not hold

- PR number: 2963
- Source review ID: 5974634671
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628420725>
- Concern: the F9 reply and verification say `grep -c "one commit each" ISSUE.md` prints 0; it
  prints 1, because the fixed entry quotes the old words where it records their correction.
- Solution: the F9 entry's verification says the only match is that quotation and records the
  correction; the reply cannot be edited. The sweep for the same mistake found that the checks
  of F2 and F3 had also stopped holding after later changes; both now say what held at the fixing
  commit and what holds at this head.
- Current-tree verification: `grep -n "one commit each" ISSUE.md` prints only line 607, the 09:18
  entry, inside the sentence that records the correction; the F2 and F3 checks were rerun at
  12:18 UTC and hold as now written.
- Resolution reference: docs(pr-reviews): [#2904] give the F9 verification of PR #2963 a check that holds
- Follow-up PR URL: N/A
- Reply URL: pending

## Processing Log

- 2026-11-19 11:28 UTC - Started the audit for review `5972182565` (`da2ce7`, APPROVED) and the
  Copilot overview `5972197600`, at the head whose last commit is
  `docs(issues): [#2904] record the task review of T4 and check AC2`. The fixes of
  F1 to F5 were committed at 09:16 to 09:19 UTC by earlier runs of the `pr-review-handler` agent
  on this event, which stopped before pushing; the description edit of F6 was made at 09:24 UTC.
  The maintainer posted an ACK of that head at 09:24:49 UTC; the
  contributor then asked for the fixes to be pushed. The first push attempt failed in the pre-push
  hook (`clippy-allow-reasons` test `it_should_accept_supported_documented_attribute_shapes`:
  `git init` of a fixture reported "File exists"); `cargo test -p clippy-allow-reasons --test cli`
  then passed (10 of 10), and the second push moved the fork branch at 11:27 UTC to the head whose
  last commit is `docs(issues): [#2904] update last-updated-utc of the manual verification evidence`
  (`git ls-remote`). The F6 comment was posted at 11:28 UTC.
- 2026-11-19 11:29 UTC - Replied on the six threads of F1 to F6 (11:28:52 to 11:29:04 UTC, the
  replies' `created_at`), recorded their URLs, and `validate-audit-record.py` exited 0 (6 rows, 0
  failures). Resolved the six threads at 11:29 UTC; each mutation returned `isResolved: true`, and a
  GraphQL refresh counted 0 unresolved threads.
- 2026-11-19 11:53 UTC - Correction for finding F11 of round 2: rewrote in place the Findings introduction
  and the first log entry above. They named the round-1 head by its full commit id, quoted the
  maintainer's ACK with that id, and named the pushed head by its abbreviated id; they now name each
  head by the subject of its last commit. The six round-1 replies, which cannot be edited, name the
  pushed head by that abbreviated id: it is the head whose last commit is
  `docs(issues): [#2904] update last-updated-utc of the manual verification evidence`.
- 2026-11-19 12:01 UTC - Round 2: processed review `5974337471` (`da2ce7`, 11:42:04 UTC, F8 to F13) with the
  `pr-review-handler` agent. F12 was already fixed by the two commits pushed after the reviewed
  head; F8 needs a history rewrite and is `NO_ACTION`. The fixes of F9, F10, F11 and F13 were
  committed at 11:50:35 to 11:54:11 UTC (`git log --format=%cI`) and pushed before the replies
  (`git ls-remote` showed the head whose last commit is
  `docs(pr-reviews): [#2904] count six sections in the F1 verification of PR #2963`). Replied on
  the six threads (11:59:19 to 11:59:30 UTC, the replies' `created_at`), recorded their URLs, and
  `validate-audit-record.py` exited 0 (12 rows, 0 failures). Resolved the six threads; each
  mutation returned `isResolved: true`. A GraphQL refresh at 12:01:10 UTC counted three
  unresolved threads, all from review `5974503813` (`da2ce7`, 11:54:19 UTC, F14 to F16), which
  belongs to the next round and is not processed here.
- 2026-11-19 12:20 UTC - Round 3: processing reviews `5974503813` (`da2ce7`, 11:54:19 UTC, F14 to F16) and
  `5974634671` (`da2ce7`, 12:03:59 UTC, F17 and F18) with the `pr-review-handler` agent. F17 was
  already fixed by the round-2 audit commit; the fixes of F14, F15, F16 and F18 were committed at
  12:16:10 to 12:19:43 UTC (`git log --format=%cI`). The five rows are committed with this entry
  before the replies, with the reply URLs pending and the thread state `OPEN`.
  `validate-audit-record.py` exited 1 on this version with five failures, one missing reply URL
  for each of F14 to F18 and no other; the URLs, the validator's result and the resolutions follow
  in a later entry.

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
