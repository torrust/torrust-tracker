---
pr-number: 3044
pr-url: https://github.com/torrust/torrust-tracker/pull/3044
last-updated-utc: "2026-11-25 09:04"
---

<!-- skill-link: process-pr-review -->

# PR #3044 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3044.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A (the findings were fixed before the merge).

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

Round 1, on the first head of the pull request (`fa4c27842`): Greptile review 6045345701 (F1, F2)
and CodeRabbit review 6045401167 (F3, F4), both on that head. F1, F2 and F4 are about the
"Review-Service Check Runs" section this PR adds to `watch-pull-request-checks`; F4 repeats F1.
This is the first audit record with the `Reviewer` column the PR introduces.

Round 2, on head `e57378df3`: Greptile review 6045462816 (F5), about the wait loop added for F2.
Round 3, on head `13b96f0cf`: CodeRabbit review 6045569854 with one inline thread (F6) and one
review-body finding (F7) that repeats F5.
Round 4, on head `9d1946125`: CodeRabbit review 6045717795, its answer in the F6 thread (F8).
Round 5, on head `146179e79`: CodeRabbit review 6045878013 with one thread (F9). It came after the
fifth round of fixes; the contributor allowed a sixth round on 2026-11-25.
Round 6, on head `9e743e54f`: CodeRabbit review 6053945210 with one thread (F10) and one
outside-diff finding in its body (F11). On head `2b9de50df`: CodeRabbit review 6054013798, a
follow-up in the F9 thread (F12), and da2ce7's review 6054114265 (`APPROVED`, findings F1 to F7,
recorded as F13 to F19 because their ids collide with this record's). The contributor allowed one
seventh round for all of them on 2026-11-25.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3044-f1 | greptile-apps[bot] | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3044-f2 | greptile-apps[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3044-f3 | coderabbitai[bot] | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3044-f4 | coderabbitai[bot] | Major | correctness | RE_RAISE_OF:F1 | FIXED | RESOLVED |
| F5 | review-finding:pr-3044-f5 | greptile-apps[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3044-f6 | coderabbitai[bot] | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3044-f7 | coderabbitai[bot] | Minor | correctness | RE_RAISE_OF:F5 | FIXED | NON_RESOLVABLE |
| F8 | review-finding:pr-3044-f8 | coderabbitai[bot] | Major | correctness | RE_RAISE_OF:F6 | FIXED | RESOLVED |
| F9 | review-finding:pr-3044-f9 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-3044-f10 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-3044-f11 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | NON_RESOLVABLE |
| F12 | review-finding:pr-3044-f12 | coderabbitai[bot] | Minor | correctness | ORIGINAL | NO_ACTION | RESOLVED |
| F13 | review-finding:pr-3044-f13 | da2ce7 | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-3044-f14 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-3044-f15 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-3044-f16 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-3044-f17 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-3044-f18 | da2ce7 | Nit | link-integrity | ORIGINAL | NO_ACTION | RESOLVED |
| F19 | review-finding:pr-3044-f19 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - A cancelled workflow check looked ready

- PR number: 3044
- Source review ID: 6045345701
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4684924353
- Concern: The rule said the workflow checks pass "when there is no `fail` and no `pending`", so a
  result such as `{"pass":13,"cancel":1}` counted as passing although one job never checked the
  change. Greptile marked it P1; recorded as `Major`.
- Solution: The section now says the workflow checks pass only when every one is `pass` or
  `skipping`; a `cancel` needs attention like a `fail`, and a `pending` needs more waiting.
- Current-tree verification: "Review-Service Check Runs" in
  `.claude/skills/watch-pull-request-checks/SKILL.md` ends with the new rule, and
  `grep -c "no .fail. and no" .claude/skills/watch-pull-request-checks/SKILL.md` prints `0`.
- Resolution reference: docs(watch-pull-request-checks): [#3037] wait for workflow checks alone and treat a cancelled check as unready
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4684999823

### F2 - Waiting for a slow review service

- PR number: 3044
- Source review ID: 6045345701
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4684924402
- Concern: The section left review-service check runs out of the result but gave no way to wait for
  the workflow checks alone: `gh pr checks --watch` keeps waiting for a pending review service, so an
  unattended agent could stay stuck after CI finished. Greptile marked it P2; recorded as `Minor`.
- Solution: The section adds a token-free `until` loop that polls `gh pr checks --json
  bucket,workflow` every 60 seconds and ends when at least one workflow check exists and none is
  pending. "Watching the Checks" step 1 of `wait-for-pr-review` points to it.
- Current-tree verification: the loop's `--jq` condition returned `false` on PR 3044 while its
  workflow runs had not started (only the `CodeRabbit` and `Greptile Review` check runs existed)
  and `true` on PR 3039, whose workflow check had passed.
- Resolution reference: docs(watch-pull-request-checks): [#3037] wait for workflow checks alone and treat a cancelled check as unready
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4685000389

### F3 - A review posted before the current head is never reported

- PR number: 3044
- Source review ID: 6045401167
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4684969852
- Concern: Step 1 of the review loop starts the poller with `--since-head-sha`, so it reports only
  reviews created after that commit was made. A review posted earlier and not processed yet, for
  example when a new session takes over, is never reported, and the new exit condition ("every
  review received so far is processed") had no step that lists the reviews.
- Solution: Step 1 now lists every review on the pull request, from every author, when a session
  starts or takes over, and processes each review the audit record does not cover before it runs
  the poller. The readiness bullet uses the same listing. The workflow-check readiness bullet was
  aligned with F1 in the same commit.
- Current-tree verification: step 1 of "The Review Loop" in
  `.claude/skills/wait-for-pr-review/SKILL.md` has the listing command; run on PR 3044 it printed
  reviews 6045345701 (Greptile), 6045401167 (CodeRabbit) and this branch's own reply reviews.
- Resolution reference: docs(wait-for-pr-review): [#3037] list every review on session start before waiting and before readiness
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4685061857

### F4 - A cancelled workflow check looked ready (re-raise)

- PR number: 3044
- Source review ID: 6045401167
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4684969867
- Concern: The same concern as F1, raised independently on the same head: a `cancel` bucket is
  neither `fail` nor `pending`, so the rule reported readiness.
- Solution: Fixed by the F1 commit. CodeRabbit marked it addressed in `e57378df3` and resolved the
  thread itself; the reply points to the fix and to the aligned readiness bullet of F3.
- Current-tree verification: as F1.
- Resolution reference: docs(watch-pull-request-checks): [#3037] wait for workflow checks alone and treat a cancelled check as unready
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4685062394

### F5 - The wait loop hid `gh` errors and retried without end

- PR number: 3044
- Source review ID: 6045462816
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4685021733
- Concern: The loop added for F2 sent `gh` errors to `/dev/null` and slept again on any failure, so
  an expired login or denied access left the agent waiting without saying why. Greptile marked it
  P2, non-blocking; recorded as `Minor`.
- Solution: The loop keeps `gh`'s error and stops on the first non-zero exit with a message. With
  `--json`, `gh pr checks` exits `0` for passing, failing and pending checks (checked on eleven open
  pull requests), so a non-zero exit is a real error and pending or failed checks stay on the normal
  path.
- Current-tree verification: the loop run against PR 3039 ended at once; against the missing
  PR 999999 it ended with `GraphQL: Could not resolve to a PullRequest with the number of 999999.`
  and `gh pr checks failed; see its error above`.
- Resolution reference: docs(watch-pull-request-checks): [#3037] stop the workflow-check wait on a gh error instead of retrying
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4685124134

### F6 - The wait loop never ended while the runs waited for approval

- PR number: 3044
- Source review ID: 6045569854
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4685108172
- Concern: A fork pull request whose workflow runs are `action_required` has no workflow checks, so
  the loop's condition stayed false and it slept again without end, never reaching "When Checks
  Never Start". Every pull request of a contributor without write access is in this state after each
  push.
- Solution: Each pass of the loop also reads `gh run list --commit <head> --json conclusion` and
  ends with "workflow runs of <head> wait for a maintainer's approval" when a run of the pushed head
  is `action_required`. The text says to ask for the approval and run the loop again.
- Current-tree verification: the loop run against this pull request, whose head `d186506c4` had five
  `action_required` runs, ended on its first pass with that line; against PR 3039 it ended with
  `workflow checks finished`.
- Resolution reference: docs(watch-pull-request-checks): [#3037] end the workflow-check wait when the runs wait for approval
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4685179281

### F7 - The wait loop hid `gh` errors (re-raise)

- PR number: 3044
- Source review ID: 6045569854
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3044#pullrequestreview-6045569854
- Concern: The same concern as F5, in the body of a review of a head that did not have the F5 fix
  yet. It also asked for a bounded number of retries.
- Solution: Fixed by the F5 commit; stopping at the first failed `gh` call is the bound. The answer
  is a pull-request conversation comment, because a review-body finding has no thread.
- Current-tree verification: as F5.
- Resolution reference: docs(watch-pull-request-checks): [#3037] stop the workflow-check wait on a gh error instead of retrying
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3044#issuecomment-7141731519

### F8 - The wait loop stopped when the head had no check at all

- PR number: 3044
- Source review ID: 6045717795
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3044#pullrequestreview-6045717795
- Concern: CodeRabbit's answer in the F6 thread (comment r4685221545): `gh pr checks --json` fails
  with `no checks reported on the '<branch>' branch` when the head has no check at all, so the F6
  loop stopped on that "error" before it looked for runs waiting for approval. It kept F6 open.
- Solution: The loop keeps the `gh pr checks` error in a file. When it is the "no checks reported"
  message, the loop treats the head as having no check yet and goes on to the `action_required`
  test; any other error still stops it. The text explains the message. The reply went in the F6
  thread, whose replies GitHub attaches to its first comment, so the source URL is the review.
- Current-tree verification: the `gh` 2.76.2 source (`pkg/cmd/pr/checks/checks.go`, `collectChecks`)
  returns that error before the JSON exporter runs, and PR 10 of this repository, which has no
  checks, gives it. One pass of the loop on PR 10 gave `ready=false waiting=false` and would sleep;
  on the missing PR 999999 it stopped with the GraphQL error and `gh pr checks failed`.
- Resolution reference: docs(watch-pull-request-checks): [#3037] treat a head with no checks yet as not ready in the workflow-check wait
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4685290079

### F9 - The wait loop kept the head it started with

- PR number: 3044
- Source review ID: 6045878013
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4685346659
- Concern: The loop set `head` once from `git rev-parse HEAD`. If someone else pushed while it
  waited, `gh pr checks` read the new head but `gh run list --commit` still asked about the old one,
  so an approval needed for the new head could go unreported and the loop kept waiting.
- Solution: The loop reads the pull request's head with `gh pr view <PR> --json headRefOid` at the
  start of every pass, so both commands look at the same commit; a failed `gh pr view` stops it with
  its error. The text explains why. Fixed in a sixth round, which the contributor allowed.
- Current-tree verification: the loop text from the skill, run with `gh-bot` and one pass, ended
  with `workflow checks finished` on PR 3044, read head `f5573e389` of PR 10 (no checks) and would
  sleep, and stopped with the GraphQL error on the missing PR 999999.
- Resolution reference: docs(watch-pull-request-checks): [#3037] read the pull request head on every pass of the workflow-check wait
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692027547

### F10 - The wait loop wrote into a `.tmp/` that may not exist

- PR number: 3044
- Source review ID: 6053945210
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692064192
- Concern: In a fresh checkout without `.tmp/`, the redirect of the workflow-check loop fails before its body runs, so the loop never starts. da2ce7 confirmed it independently (his F2).
- Solution: Added `mkdir -p .tmp` before the loop and before the `--watch` example, whose redirect had the same problem.
- Current-tree verification: `grep -c "mkdir -p .tmp" .claude/skills/watch-pull-request-checks/SKILL.md` prints `2`.
- Resolution reference: docs(watch-pull-request-checks): [#3037] create .tmp and make the workflow-check loop the wait after a push
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692560498

### F11 - `--watch` was still the primary wait after a push

- PR number: 3044
- Source review ID: 6053945210
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3044#pullrequestreview-6053945210
- Concern: Outside-diff finding on lines 28-29 of `watch-pull-request-checks`: the `--watch` command keeps waiting for a pending review-service check run, which does not count for readiness, so the workflow-only loop should be the wait after a push.
- Solution: "Wait for the Checks" now sends the reader to the workflow-check loop after a push and keeps `--watch` for when every check run matters; step 1 of "Watching the Checks" in `wait-for-pr-review` says the same. Answered in a conversation comment, because a review-body finding has no thread.
- Current-tree verification: Both sections in the current tree name the loop in "Review-Service Check Runs" first.
- Resolution reference: docs(watch-pull-request-checks): [#3037] create .tmp and make the workflow-check loop the wait after a push
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3044#issuecomment-7157815185

### F12 - A push inside one pass of the wait loop

- PR number: 3044
- Source review ID: 6054013798
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3044#pullrequestreview-6054013798
- Concern: CodeRabbit's follow-up in the F9 thread (comment r4692120426): a push between `gh pr view` and `gh pr checks` in one pass can still make the two commands look at different commits, so the loop can report that the replaced commit needs approval.
- Solution: Not changed, by the contributor's decision on 2026-11-25. The window is the few seconds between two calls; its worst outcome is one approval line about the commit just replaced, after which the poller reports `head_changed` and the next wait reads the new head. Another re-read would make the loop harder to follow. The reply went in the F9 thread, whose replies GitHub attaches to its first comment, so the source URL is the review.
- Current-tree verification: The loop in the current tree reads the head once per pass, as F9 left it.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692560907
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692560907

### F13 - No test pinned the command-line default or the `watch` field

- PR number: 3044
- Source review ID: 6054114265
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692200511
- Concern: Restoring the old two-login default on `--reviewer`, or swapping the two `watch` strings, still passed every test: the CLI fixtures had a pull request without `user`, and the unit tests pass `Reviewers::AllExceptAuthor` directly.
- Solution: Two CLI tests run the binary against a fake pull request opened by `contributor`, with the author's reply review first and a review by `new-reviewer[bot]` after it: without `--reviewer` they expect the bot's review and `watch: all_except_author`; with `--reviewer da2ce7`, a timeout and `watch: listed`. The test-design review records them.
- Current-tree verification: Mutation check, run and reverted: the old default fails the first test; swapped `watch` strings fail both. `cargo test -p wait-for-pr-review` (stable Rust) passes 73 + 3 + 40 tests.
- Resolution reference: test(wait-for-pr-review): [#3037] pin the default reviewer set and the watch field at the command line
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692561312

### F14 - The audit record stopped before review 6053945210

- PR number: 3044
- Source review ID: 6054114265
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692200519
- Concern: Review 6053945210 (its thread and its outside-diff finding) had no row, no reply and no resolution in this record.
- Solution: Rows F10 to F19 record review 6053945210, the follow-up 6054013798 and review 6054114265.
- Current-tree verification: `validate-audit-record.py --pr-number 3044` passes on this record.
- Resolution reference: docs(pr-reviews): [#3037] record review rounds 6 and 7 of PR 3044
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692567012

### F15 - The pull request description no longer matched the branch

- PR number: 3044
- Source review ID: 6054114265
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692200530
- Concern: The description, which the merge tool copies into the merge commit, said nothing about the workflow-check wait, the session-start review listing or the audit record, and pointed the V5 fixes to "the last skill commit".
- Solution: The description now covers them, names the V5 fix by its commit subject and has a Review section. A fix outside the tree, so the resolution reference is the reply that states it.
- Current-tree verification: `gh-bot pr view 3044 --json body` shows the new sections.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692561631
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692561631

### F16 - Merge Readiness stated a stricter checks rule than the other skills

- PR number: 3044
- Source review ID: 6054114265
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692200537
- Concern: "every workflow check of the head commit passed" could hold a pull request for a skipped job, unlike `wait-for-pr-review` and `watch-pull-request-checks`.
- Solution: Merge Readiness now says each workflow check is `pass` or `skipping` and a cancelled one needs attention.
- Current-tree verification: The Checks bullet of Merge Readiness in the current tree has that wording.
- Resolution reference: docs(merge-pull-request): [#3037] state the workflow-check readiness rule as pass or skipping
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692562037

### F17 - The raised round limit was not in the pull request conversation

- PR number: 3044
- Source review ID: 6054114265
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692200543
- Concern: `wait-for-pr-review` asks for a raised round limit to be recorded in the pull request conversation; only this record had it.
- Solution: A conversation comment records the limit raised to 6 and then to 7 rounds. A fix outside the tree, so the resolution reference is the reply that links it.
- Current-tree verification: https://github.com/torrust/torrust-tracker/pull/3044#issuecomment-7157627998 exists.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692562438
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692562438

### F18 - The record cites commit ids

- PR number: 3044
- Source review ID: 6054114265
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692200552
- Concern: The `open-pull-request` commit-id check prints the head ids this record and the evidence cite; a rebase would make them wrong.
- Solution: Not changed. The ids record which head each review saw, the branch merges cleanly and will not be rebased, so they become `develop` commits on merge, and every fix is also cited by its commit subject.
- Current-tree verification: da2ce7 found the merge with `develop` clean; no rebase is planned.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692562895
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692562895

### F19 - The ADR had no Affected Code section and no link from the code

- PR number: 3044
- Source review ID: 6054114265
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692200557
- Concern: `create-adr` asks an ADR that affects specific code for an Affected Code section and a back-link from that code.
- Solution: The ADR lists the poller's `Reviewers` and `--reviewer`/`watch`, the contract check, the skills and the template; the doc comment on `Reviewers` names the ADR.
- Current-tree verification: The ADR has an "Affected Code" section, and `event.rs` names the ADR file above `pub enum Reviewers`.
- Resolution reference: docs(adrs): [#3037] list the affected code of the mandatory-reviewer ADR and link it from Reviewers
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3044#discussion_r4692563271

## Processing Log

- 2026-11-24 18:50 UTC - Round 1: Greptile review 6045345701 (18:50:22) with two threads, after its summary comment (18:50:17).
- 2026-11-24 18:57 UTC - Pushed the fix commit for F1 and F2 (head `e57378df3`), then created this audit record before replying.
- 2026-11-24 18:57 UTC - Replied on both threads (r4684999823, r4685000389) and resolved them; the GraphQL refresh then showed CodeRabbit review 6045401167 (18:54:12), submitted on the same head before the push, with two threads.
- 2026-11-24 19:01 UTC - Pushed the F3 fix (head `13b96f0cf`), replied on both CodeRabbit threads (r4685061857, r4685062394) and resolved the F3 thread; CodeRabbit had resolved the F4 thread itself. A GraphQL refresh shows no unresolved thread.
- 2026-11-24 19:02 UTC - Round 2: Greptile review 6045462816 (18:58:41, on `e57378df3`) with one thread (F5).
- 2026-11-24 19:06 UTC - Pushed the F5 fix (head `d186506c4`), replied (r4685124134) and resolved the thread. Round 3: CodeRabbit review 6045569854 (19:05:28, on `13b96f0cf`) with one thread (F6) and a review-body finding (F7, a re-raise of F5).
- 2026-11-24 19:11 UTC - Pushed the F6 fix (head `9d1946125`), replied on the F6 thread (r4685179281) and resolved it, and answered F7 in a conversation comment. A review listing shows no other review, and a GraphQL refresh shows no unresolved thread. Rounds with a `FIXED` finding: 4 of the limit of 5.
- 2026-11-24 19:15 UTC - Pushed this record (head `911ae3aae`). Round 4: CodeRabbit review 6045717795 (19:14:43, on `9d1946125`), its answer in the F6 thread (F8).
- 2026-11-24 19:20 UTC - Pushed the F8 fix (head `146179e79`) and replied in the F6 thread (r4685290079); the thread stays resolved. A review listing shows no other review, and a GraphQL refresh shows no unresolved thread. Rounds with a `FIXED` finding: 5, the limit; a further round needs the contributor's decision.
- 2026-11-24 19:25 UTC - Round 5: CodeRabbit review 6045878013 (19:25:08, on `146179e79`) with one thread (F9). Its fix needs a sixth round: asked the contributor and pushed nothing. CodeRabbit review 6045921280 (19:28:04, on `69b440e23`) only confirmed F8 in the F6 thread; no finding.
- 2026-11-25 08:01 UTC - The maintainer ACKed `69b440e23`; asked whether to fix or defer F9.
- 2026-11-25 08:08 UTC - The contributor chose to fix F9 now (sixth round). Pushed the fix (head `9e743e54f`), replied (r4692027547) and resolved the thread. A review listing shows no other review, and a GraphQL refresh shows no unresolved thread. Rounds with a `FIXED` finding: 6, the sixth allowed by the contributor.
- 2026-11-25 08:12 UTC - Round 6: CodeRabbit review 6053945210 (08:12:10, on `9e743e54f`) with one thread (F10) and an outside-diff finding (F11); a seventh round was needed, so asked the contributor and pushed nothing.
- 2026-11-25 08:27 UTC - CodeRabbit review 6054013798 (08:18:21, on `2b9de50df`), a follow-up in the F9 thread (F12). da2ce7's review 6054114265 (08:27:00, `APPROVED`, on `2b9de50df`) with seven threads (F13 to F19), and his ACK of `2b9de50df`.
- 2026-11-25 09:04 UTC - The contributor allowed one batched seventh round; recorded in the conversation (issuecomment-7157627998). Pushed the fixes (head `407203788`), updated the pull request description, replied on every thread and on F11 in the conversation, and resolved da2ce7's seven threads; CodeRabbit had resolved the F10 thread. Rounds with a `FIXED` finding: 7, the limit the contributor set.

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
