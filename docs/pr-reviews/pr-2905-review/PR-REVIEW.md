---
pr-number: 2905
pr-url: https://github.com/torrust/torrust-tracker/pull/2905
last-updated-utc: "2026-11-17 13:23"
---

<!-- skill-link: process-pr-review -->

# PR #2905 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2905.

<!-- Copied verbatim into each audit record after placeholder replacement. -->

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

<!-- Copied verbatim into each audit record. -->

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

Round 1 is the review `5938631990` of `da2ce7` (CHANGES_REQUESTED, F1 to F7, reviewer-provided IDs), on the first head of
the pull request. Its body holds no finding beyond the seven inline threads.
Round 2 is Copilot's review `5939023583` (COMMENTED, F8 and F9, audit-local IDs), also on the first head of the pull
request; it was submitted after the round 1 fixes were pushed. Copilot labels both findings "Medium severity"; the
severity `Minor` is inferred from that label. Its body says the full agentic review did not run because it did not
start before the timeout.
Round 3 is the review `5939071001` of `da2ce7` (DISMISSED, F10 to F12, audit-local IDs; the reviewer's own F8 and F9
collide with the audit's F8 and F9, and its F10 follows them, so all three are recorded as the original), on the head
that carried the round 1 audit.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2905-f1` | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2905-f2` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2905-f3` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2905-f4` | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2905-f5` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2905-f6` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2905-f7` | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2905-f8` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2905-f9` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2905-f10` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F11 | `review-finding:pr-2905-f11` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | `review-finding:pr-2905-f12` | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The baseline of 9 review rounds for pull request 2851 contradicts that pull request's audit record

- PR number: 2905
- Source review ID: 5938631990
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4599729270
- Concern: The specification said pull request 2851 took 9 review rounds with the review-addressing session on Sonnet 5.5. The audit record of that pull request has 14 rounds, and the round commits name three models, so 9 is neither the total nor the count on the named model. AC9, M6 and References measured against that figure.
- Solution: Background states the 14 rounds with the split by model and says that five rounds already ran on the model D13 pins. AC9, M6 and References use that baseline. The uncounted sentence about where most rounds came from is replaced by a count from the record.
- Current-tree verification: `docs/pr-reviews/pr-2851-review/PR-REVIEW.md` describes rounds 1 to 14. `gh api repos/torrust/torrust-tracker/pulls/2851/commits` lists the round-recording commits with the footers Fable 5.2 (rounds 1 to 3), Sonnet 5.5 (round 4), Sonnet 5 (rounds 5 to 9) and Opus 5.5 (rounds 10 to 14). `grep -c "^- 2026.* - Correction"` on that record prints 10. `grep -n "9 rounds\|9 review"` on the specification prints nothing.
- Resolution reference: `docs(issues): [#2904] state the 14 review rounds of PR #2851 with their split by model`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4599865065

### F2 - D10 adds a line to a list of ADRs to move that the harness-workspace draft does not have

- PR number: 2905
- Source review ID: 5938631990
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4599729286
- Concern: D10, In Scope and T10 said the new ADR is listed among the ADRs to move in the harness-workspace draft. That draft moves crates and lists no ADR to move.
- Solution: D10, In Scope and T10 now say that this issue adds an item to the draft's scope: decide whether the AI-harness ADRs move to the workspace, naming this ADR.
- Current-tree verification: `grep -n -i adr docs/issues/drafts/2003-separate-ai-harness-cargo-workspace/ISSUE.md` shows the container-build ADR and one ADR to create, no ADR to move. `grep -n "ADRs to move\|moves with"` on the specification prints nothing.
- Resolution reference: `docs(issues): [#2904] say that the harness-workspace draft plans no ADR move yet`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4599865539

### F3 - No task produces the AC6 inventory, or moves the planner's leftovers into `create-issue`

- PR number: 2905
- Source review ID: 5938631990
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4599729299
- Concern: AC6 requires an inventory of every rule that only a removed profile held, and P1 sends the planner's leftovers to `create-issue`, but no task of the plan delivered either.
- Solution: The inventory is the file `removed-profile-rules.md` in the specification folder: T4 creates it, T5 to T9 add rows, T10 completes it. T6 moves the rules that only `planner` held into `create-issue`. Commit Points and AC6 name them.
- Current-tree verification: Read the Implementation Plan, the paragraph under it, Commit Points and AC6 in the specification after the change: each names the inventory file; the T6 row names the planner rules.
- Resolution reference: `docs(issues): [#2904] schedule the removed-rules inventory and the planner rules in the plan`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4599866005

### F4 - The plan adds tests (T3) but omits the test-development loop the governing skill requires

- PR number: 2905
- Source review ID: 5938631990
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4599729312
- Concern: The `create-issue` skill requires a progressive test-development loop with a recorded design review when a plan adds tests. Commit Points had dropped the template paragraph that carries it.
- Solution: Commit Points carries the template paragraph again, for T3 and T5, and the T3 rows name the review step. A later commit adds the hook's tests to the T5 row.
- Current-tree verification: Compared the restored paragraph with `docs/templates/ISSUE.md` (the paragraph after the Commit Points table). `linter markdown` and `linter cspell` passed.
- Resolution reference: `docs(issues): [#2904] restore the test-development loop for the tasks that add tests`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4599866406

### F5 - The archive figures do not recompute under the windows stated

- PR number: 2905
- Source review ID: 5938631990
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4599729327
- Concern: The counts 24 of 46 (October) and 2 of 30 (1 to 15 November) of archived specifications with an `agent-review-reports.md` could not be reproduced, and no method was stated. D17 repeated them.
- Solution: Background gives recomputed counts, 25 of 48 and 2 of 32, with the method; D17 uses them. The reviewer counted 24 of 48 for October; the cause of that difference of one was not found and the reply says so. The body of issue 2904 carried the same counts and was edited to match.
- Current-tree verification: On `develop` at the commit "Merge torrust/torrust-tracker#2900": `TZ=UTC git log --diff-filter=AR --name-only --date=format-local:%Y-%m-%d --format='@%cd' -- docs/issues/closed`, keeping for each folder the oldest date, then `git cat-file -e <commit>:docs/issues/closed/<folder>/agent-review-reports.md` per folder. Output: October 25 of 48, 1 to 15 November 2 of 32; EPIC folders two in October (one with a report) and one in November (without).
- Resolution reference: `docs(issues): [#2904] recompute the archived-report counts and state the method`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4599866826

### F6 - The skills move did rewrite two files under `.github/agents/`

- PR number: 2905
- Source review ID: 5938631990
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4599729342
- Concern: D8 said the skills move did not rewrite the Copilot profiles. Its commit changed `README.md` and one profile there.
- Solution: D8 names the two rewritten files and the five profiles that still name stub paths.
- Current-tree verification: `git show --stat` of the commit "refactor(skills): [#2866] move the skills to .claude/skills", limited to `.github/agents`, lists `README.md` and `copilot-suggestions-handler.agent.md`. `grep -ln "\.github/skills/" .github/agents/*` lists `committer`, `implementer`, `github-operator`, `pr-reviewer` and `task-reviewer`.
- Resolution reference: `docs(issues): [#2904] say which profile files the skills move rewrote`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4599867281

### F7 - Numbering is out of order in three lists, and the count of pull requests is loose

- PR number: 2905
- Source review ID: 5938631990
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4599729361
- Concern: The acceptance criteria, the settled questions and the manual scenarios were not in ID order, and Risks said about ten pull requests for twelve tasks.
- Solution: The three lists are ordered by ID, and Risks says twelve pull requests, eleven after the specification's.
- Current-tree verification: `grep -n` on the three lists prints AC1 to AC12, Q1 to Q11 and M1 to M7 in order.
- Resolution reference: `docs(issues): [#2904] order the criteria, questions and scenarios by ID and give the pull request count`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4599867733

### F8 - D17 requires the stop hook to be bounded but gives no number

- PR number: 2905
- Source review ID: 5939023583
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4599997320
- Concern: D17 said the stop hook lets the agent finish after a bounded number of refusals, without the number. T5 tests the bound and AC11 checks it, so they could not be verified against the specification.
- Solution: D17 sets the bound to 3 refusals per run of the agent, the same number as the round limit of D18; the fourth attempt to finish is allowed and the hook reports that the limit was reached. T5, AC11 and scenario M7 name the number.
- Current-tree verification: `grep -n "bounded"` on the specification prints nothing; `grep -n "3 refusals"` lists D17, T5, Commit Points, the progress log, AC11 and M7.
- Resolution reference: `docs(issues): [#2904] set the stop hook's limit to 3 refusals`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4600064214

### F9 - The checkpoints and the completion review still require `agent-review-reports.md`

- PR number: 2905
- Source review ID: 5939023583
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4599997411
- Concern: D18 replaces `agent-review-reports.md` with JSON reply files, but the workflow checkpoints and the Implementation Completion Review of the specification still require the old file, with no rule saying which one applies to this issue.
- Solution: D18 states the transitional rule: the reply files apply to specifications created after T9 changes the issue template; this specification keeps `agent-review-reports.md` for the reviews of its own work until it closes, and the reply files in its folder are the evidence of scenarios M5 and M7 only. The checkpoint and the completion-review bullet point to that rule.
- Current-tree verification: Read D18, the checkpoint line and the last bullet of Implementation Completion Review in the specification after the change; each states or points to the rule. `linter markdown` and `linter cspell` passed.
- Resolution reference: `docs(issues): [#2904] say which report file this issue uses for its own reviews`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4600064725

### F10 - The stated method gives 24 of 48 for October, not 25

- PR number: 2905
- Source review ID: 5939071001
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4600032714
- Concern: The method added for F5 said a folder has a report when the file exists "at that commit". Read as the commit that archived the folder, it gives 24 of 48 for October, not the 25 stated. The folder `2413-2410-si-22-1-document-event-flows-and-draft-adr` received its report in a later commit the same day.
- Solution: The method says the report is checked at the counting revision of `develop`, not at the archive commit, names that folder, and says that checking at the archive commit gives 24 of 48. The counts stay 25 of 48 and 2 of 32. This also answers the difference of one that the F5 entry could not place.
- Current-tree verification: `TZ=UTC git log torrust/develop --diff-filter=AR --date=format-local:'%Y-%m-%d %H:%M' --format='%cd %s' -- docs/issues/closed/2413-2410-si-22-1-document-event-flows-and-draft-adr` prints the archive commit at 2026-10-19 15:56 and the commit that added the report at 2026-10-19 17:22. `git cat-file -e torrust/develop:docs/issues/closed/2413-2410-si-22-1-document-event-flows-and-draft-adr/agent-review-reports.md` exits 0.
- Resolution reference: `docs(issues): [#2904] say at which revision the archived report is checked`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4600155615

### F11 - The inventory accepts an agent as a rule's destination; AC6 accepts only a skill

- PR number: 2905
- Source review ID: 5939071001
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4600032737
- Concern: The inventory paragraph added for F3 let a rule's destination be a named skill or agent, while AC6 required a named skill. A rule moved into an agent file would satisfy the inventory and fail AC6.
- Solution: Both now say the same: a named skill, or the agent file only when the rule is about the role itself (identity, tools, inputs, report format), which P2 keeps in the agent.
- Current-tree verification: Read the inventory paragraph under the Implementation Plan and AC6 in the specification after the change; both carry the same rule. `linter markdown` passed.
- Resolution reference: `docs(issues): [#2904] align the inventory's destinations with AC6`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4600156170

### F12 - Automatic Checks names the reply validator's tests but not the stop hook's

- PR number: 2905
- Source review ID: 5939071001
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4600032753
- Concern: T5 and the test paragraph add tests of the stop hook's limit, but the Automatic Checks list named only the reply validator's tests.
- Solution: Automatic Checks lists the tests of the stop hook's limit of 3 refusals (T5).
- Current-tree verification: Read the Automatic Checks list in the specification after the change. `linter markdown` and `linter cspell` passed.
- Resolution reference: `docs(issues): [#2904] list the stop hook's tests among the automatic checks`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2905#discussion_r4600156740

## Processing Log

- 2026-11-17 12:58 UTC - Started audit, after the seven replies, because it holds their URLs. Review `5938631990` of `da2ce7` was submitted at 12:45:09Z (forge `submitted_at`) with seven findings, F1 to F7. The seven fixes and a plan and log follow-up were committed between 12:48:39Z and 12:55:23Z (`git log --format=%cI`, in UTC), one commit per finding, and pushed; `git ls-remote` showed the fork branch at the local head before the replies. The seven replies were posted between 12:57:58Z and 12:58:14Z (forge `created_at`). The body of issue 2904 was edited for F5 before the replies.
- 2026-11-17 12:59 UTC - `validate-audit-record.py --pr-number 2905 --base torrust/develop`, with its `gh` calls made through the contributor account's `gh-bot`, exited 0 (7 rows, 0 failures). Resolved the seven threads; a GraphQL refresh showed 0 unresolved threads.
- 2026-11-17 13:14 UTC - Round 2: Copilot's review `5939023583` (submitted 13:09:05Z, forge `submitted_at`) on the first head, with two findings, F8 and F9. Both still applied to the current tree. The two fixes were committed at 13:10:24Z and 13:11:26Z (`git log --format=%cI`, in UTC) and pushed; `git ls-remote` showed the fork branch at the local head before the replies. The two replies were posted at 13:14:27Z and 13:14:29Z (forge `created_at`).
- 2026-11-17 13:15 UTC - `validate-audit-record.py --pr-number 2905 --base torrust/develop`, with its `gh` calls made through `gh-bot`, exited 0 (9 rows, 0 failures). Resolved the two threads of round 2; a GraphQL refresh showed 0 unresolved threads.
- 2026-11-17 13:23 UTC - Correction for the entry stamped 13:15: it says the two threads of round 2 were resolved. The call resolved every unresolved thread, five in all: the two of round 2 and the three threads of review `5939071001`, which had been submitted at 13:11:58Z (forge `submitted_at`) and had not been read or answered. The three were reopened by 13:16:45Z (`date -u` after the call). This repeats the mistake recorded in round 8 of the audit of pull request 2851; from here on threads are resolved by their IDs, not by a query for unresolved threads.
- 2026-11-17 13:23 UTC - Round 3: review `5939071001` of `da2ce7` with three nits, recorded as F10 to F12. The three fixes were committed between 13:17:23Z and 13:18:56Z (`git log --format=%cI`, in UTC) and pushed with the round 2 audit commit; `git ls-remote` showed the fork branch at the local head before the replies. The three replies were posted between 13:22:34Z and 13:22:40Z (forge `created_at`).
- 2026-11-17 13:23 UTC - `validate-audit-record.py --pr-number 2905 --base torrust/develop`, with its `gh` calls made through `gh-bot`, exited 0 (12 rows, 0 failures). Before resolving, the only unresolved threads were the three of round 3, each with the contributor's reply as its last comment. Resolved those three by their thread IDs; a GraphQL refresh showed 0 unresolved threads.

<!-- Copied verbatim into each audit record. -->

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
